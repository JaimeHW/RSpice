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
        s.b[447] = (p.p3 == 1.0);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if s.b[447] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[447]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[153] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.v[320] = 0.0;

        s.b[448] = (p.p141 == 0.0);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if s.b[448] {
            s.store_scalar(321, 1e-12);
        }

        if (!s.b[448]) {
            s.store_scalar(321, p.p141);
        }

        s.store_scale(322, 321, p.p1);

        s.store_div_from_scalar(323, 1.0, 322);

        s.v[52] = 0.001;

        s.v[318] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[63] = (1.0 / s.v[62]);

        s.v[265] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.b[449] = ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if s.b[449] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[449]) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[265])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[64] = (1.0 / p.p65);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[89] = (1.0 / s.v[79]);

        s.v[265] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.b[450] = ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if s.b[450] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[450]) {
            s.store_scalar(88, ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) + (0.1 * (((1.0 + (((-s.v[265])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p116;

        s.v[86] = (1.0 / s.v[87]);

        s.v[66] = (1.0 / s.v[75]);

        s.v[324] = (1.0 - (1.0 / p.p82));

        s.v[154] = 0.0;

        s.v[155] = 0.0;

        s.v[172] = 0.0;

        s.v[171] = 1.0;

        s.v[199] = 0.0;

        s.v[201] = 0.0;

        s.v[234] = 0.0;

        s.v[217] = 0.0;

        s.v[42] = 0.0;

        s.v[44] = 0.0;

        s.v[53] = 0.0;

        s.v[54] = 0.0;

        s.v[45] = 0.0;

        s.store_voltage(207, ctx, nodes, Some(3), None);

        s.b[451] = (s.v[207] < 0.0);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if s.b[451] {
            s.store_neg_ad(207, A::ln(A::sub_from_scalar(1.0, s.ad_value(207))));
        }

        s.b[452] = (s.v[207] < p.p124);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if s.b[452] {
            s.copy_ad(11, 207);
        }

        if (!s.b[452]) {
            s.store_offset_ln_ad(11, A::offset(s.ad_value(207), (((-p.p124)) + (1.0))), p.p124);
        }

        s.store_offset(2, 11, s.v[5]);

        s.store_scale(4, 2, 1.0 / (s.v[3]));

        s.store_scale(6, 2, 8.617086918058125e-5);

        s.v[7] = (8.617086918058125e-5 * s.v[3]);

        s.store_div_from_scalar(8, 1.0, 6);

        s.v[9] = (1.0 / s.v[7]);

        s.store_offset(10, 8, (-s.v[9]));

        s.store_offset(12, 2, (-s.v[3]));

        s.store_ln(260, 4);

        s.store_scaled_offset_ad(265, A::sub(s.ad_value(74), A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p114, A::offset(s.ad_value(2), p.p115), 1.0)), (-0.05), 10.0);

        s.b[453] = ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if s.b[453] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(265)), 0.1, 0.05);
        }

        if (!s.b[453]) {
            s.store_ad_value(70, A::add_scaled_inputs3(s.ad_value(74), 1.0, A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p114, A::offset(s.ad_value(2), p.p115), 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.1));
        }

        s.store_scaled_offset_ad(265, A::sub(s.ad_value(88), A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p117, A::offset(s.ad_value(2), p.p118), 1.0)), (-0.05), 10.0);

        s.b[454] = ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if s.b[454] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(265)), 0.1, 0.05);
        }

        if (!s.b[454]) {
            s.store_ad_value(85, A::add_scaled_inputs3(s.ad_value(88), 1.0, A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p117, A::offset(s.ad_value(2), p.p118), 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.1));
        }

        s.store_ad_value(13, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p65, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p104));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.b[455] = (0.05 < s.v[13]);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if s.b[455] {
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(13), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[455]) {
            s.store_offset_mul_ad(14, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(15, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p63, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.b[456] = (0.05 < s.v[15]);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if s.b[456] {
            s.store_ad_value(16, A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[456]) {
            s.store_offset_mul_ad(16, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(21, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p79, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.b[457] = (0.05 < s.v[21]);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if s.b[457] {
            s.store_ad_value(22, A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[457]) {
            s.store_offset_mul_ad(22, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(18, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p70, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.b[458] = (0.05 < s.v[18]);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if s.b[458] {
            s.store_ad_value(17, A::add_scaled_product(s.ad_value(18), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[458]) {
            s.store_offset_mul_ad(17, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(20, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), s.v[75], s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.b[459] = (0.05 < s.v[20]);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if s.b[459] {
            s.store_ad_value(19, A::add_scaled_product(s.ad_value(20), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[459]) {
            s.store_offset_mul_ad(19, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(56, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p26, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p108));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.b[460] = (0.05 < s.v[56]);
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if s.b[460] {
            s.store_ad_value(55, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[460]) {
            s.store_offset_mul_ad(55, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p65), p.p66);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p64);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p70, s.ad_value(17)), p.p71), (1.0 - p.p74), p.p74);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p69);

        s.store_scale(25, 27, p.p74);

        s.store_scaled_exp_scaled_input(28, 260, p.p96, p.p53);

        s.b[461] = (s.v[28] < s.v[322]);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if s.b[461] {
            s.copy_ad(28, 322);
        }

        s.store_scaled_exp_scaled_input(29, 260, (p.p97 - p.p95), p.p55);

        s.store_scaled_exp_scaled_input(30, 260, p.p100, p.p54);

        s.b[462] = (s.v[30] < s.v[322]);
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if s.b[462] {
            s.copy_ad(30, 322);
        }

        s.store_scaled_exp_scaled_input(32, 260, p.p101, p.p56);

        s.store_scaled_exp_scaled_input(33, 260, p.p103, p.p57);

        s.store_scaled_exp_scaled_input(34, 260, p.p103, p.p58);

        s.store_scaled_exp_scaled_input(31, 260, p.p98, p.p59);

        s.b[463] = (p.p121 != 0.0);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        if s.b[463] {
            s.store_offset_scaled(50, 12, ((p.p121) * (p.p9)), p.p9);
            s.store_scaled_offset(265, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[464] = (s.v[50] < 1.0);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if (s.b[463] && s.b[464]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(265)), s.v[52], 1.0);
        }

        if (s.b[463] && (!s.b[464])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), s.v[52]));
        }

        if s.b[463] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[463]) {
            s.store_scalar(48, p.p9);
        }

        s.b[465] = (p.p122 != 0.0);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if s.b[465] {
            s.store_offset_scaled(51, 12, ((p.p122) * (p.p10)), p.p10);
            s.store_scaled_offset(265, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[466] = (s.v[51] < 1.0);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if (s.b[465] && s.b[466]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(265)), s.v[52], 1.0);
        }

        if (s.b[465] && (!s.b[466])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), s.v[52]));
        }

        if s.b[465] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[465]) {
            s.store_scalar(49, p.p10);
        }

        s.store_offset_scaled(317, 12, ((p.p123) * (p.p42)), p.p42);

        s.v[267] = (s.v[318] * s.v[318]);

        s.store_square(268, 317);

        s.b[467] = (s.v[317] < 0.0);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_div_from_scalar_sub_ad(316, (0.5 * s.v[267]), A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(317));
        }

        if (!s.b[467]) {
            s.store_scaled_add_ad_lhs(316, A::sqrt(A::offset(s.ad_value(268), s.v[267])), 317, 0.5);
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_scaled_inputs(s.ad_value(260), (((4.0 - p.p97) - p.p95) + p.p120), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(10), (-p.p104), s.ad_value(48), 1.0)), p.p8);

        s.store_scaled_exp_scaled_input(36, 260, (1.0 - p.p97), p.p11);

        s.store_scaled_exp_scaled_input(37, 260, (1.0 - p.p102), p.p29);

        s.store_scaled_mul_ad(38, A::exp_scaled_input(s.ad_value(260), (6.0 - (2.0 * p.p20))), A::exp_scaled_input(s.ad_value(10), ((-p.p112) * 1.0 / (p.p20))), p.p19);

        s.store_scaled_mul_ad(39, A::exp_scaled_input(s.ad_value(260), (6.0 - (2.0 * p.p31))), A::exp_scaled_input(s.ad_value(10), ((-p.p109) * 1.0 / (p.p31))), p.p30);

        s.store_scaled_mul_ad(42, A::exp_scaled_input(s.ad_value(260), (((4.0 - p.p96) + p.p120) * 1.0 / (p.p16))), A::exp_scaled_input(s.ad_value(10), ((-p.p110) * 1.0 / (p.p16))), p.p15);

        s.store_scaled_mul_ad(44, A::exp_scaled_input(s.ad_value(260), (((4.0 - p.p96) + p.p120) * 1.0 / (p.p18))), A::exp_scaled_input(s.ad_value(10), ((-p.p110) * 1.0 / (p.p18))), p.p17);

        s.b[468] = (p.p23 == 1.0);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if s.b[468] {
            s.store_scaled_exp_scaled_input(53, 10, ((-p.p106) * 1.0 / (p.p16)), p.p24);
            s.store_scaled_exp_scaled_input(54, 10, (-p.p105), p.p27);
            s.store_scaled_exp_scaled_input(45, 10, ((-p.p107) * 1.0 / (p.p18)), p.p25);
        }

        s.store_scaled_mul_ad(43, A::exp_scaled_input(s.ad_value(260), ((4.0 - p.p102) + p.p120)), A::exp_scaled_input(s.ad_value(10), (-p.p111)), p.p28);

        s.store_scaled_mul_ad(46, A::exp_scaled_input(s.ad_value(260), (6.0 - (2.0 * p.p22))), A::exp_scaled_input(s.ad_value(10), ((-p.p112) * 1.0 / (p.p22))), p.p21);

        s.store_scaled_mul_ad(47, A::exp_scaled_input(s.ad_value(260), (4.0 / p.p137)), A::exp_scaled_input(s.ad_value(10), ((-p.p112) * 1.0 / (p.p137))), p.p136);

        s.store_scaled_mul_ad(332, A::sqrt(s.ad_value(4)), A::exp_scaled_input(s.ad_value(12), p.p144), p.p142);

        s.store_powf_ad(261, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(262, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(261), p.p34), s.ad_value(262), (p.p65 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_ad_value(58, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(261), s.ad_value(14), s.ad_value(14), p.p33), s.ad_value(73), A::exp(A::sub_from_scalar(p.p34, s.ad_value(61))), (s.v[64] * s.v[64])));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(263, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(264, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(263), p.p36), s.ad_value(264), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.store_ad_value(84, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(263), s.ad_value(19), s.ad_value(19), p.p35), s.ad_value(90), A::exp(A::sub_from_scalar(p.p36, s.ad_value(83))), (s.v[66] * s.v[66])));

        s.store_exp_scaled_input(261, 260, p.p95);

        s.store_scaled_mul(40, 261, 27, p.p13);

        s.store_scaled_mul(41, 261, 262, p.p12);

        s.store_scaled_mul_ad(94, A::exp_scaled_input(s.ad_value(260), (p.p97 - 2.0)), A::exp_scaled_input(s.ad_value(10), (-p.p119)), p.p85);

        s.store_scaled_exp_scaled_input(95, 260, ((p.p95 + p.p97) - 1.0), p.p86);

        s.store_scaled_exp_scaled_input(96, 260, (p.p98 - 1.0), p.p87);

        s.store_scaled_add(97, 95, 96, (p.p88 * 1.0 / ((p.p86 + p.p87))));

        s.store_scaled_exp_scaled_input(98, 260, (p.p99 - 1.0), p.p89);

        s.store_offset(101, 2, (-300.0));

        s.b[469] = (s.v[2] < 525.0);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if s.b[469] {
            s.store_mul_ad_rhs(99, 1, A::add_scaled_product(A::scale_offset(s.ad_value(101), 0.00072, 1.0), 1.0, s.ad_value(101), s.ad_value(101), (-1.6e-6)));
        }

        if (!s.b[469]) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scaled_exp_scaled_input(100, 260, p.p95, p.p91);

        s.v[103] = (p.p133 * (((s.v[5] / s.v[3])) as f64).powf(p.p135));

        s.b[470] = (p.p56 > 0.0);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if s.b[470] {
            s.store_div_from_scalar(104, 1.0, 32);
        }

        s.b[471] = (s.v[104] > s.v[323]);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if (s.b[470] && s.b[471]) {
            s.copy_ad(104, 323);
        }

        if (!s.b[470]) {
            s.store_scalar(104, 0.0);
        }

        s.b[472] = (p.p57 > 0.0);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[472] {
            s.store_div_from_scalar(105, 1.0, 33);
        }

        s.b[473] = (s.v[105] > s.v[323]);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if (s.b[472] && s.b[473]) {
            s.copy_ad(105, 323);
        }

        if (!s.b[472]) {
            s.store_scalar(105, 0.0);
        }

        s.b[474] = (p.p58 > 0.0);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if s.b[474] {
            s.store_div_from_scalar(106, 1.0, 34);
        }

        s.b[475] = (s.v[106] > s.v[323]);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if (s.b[474] && s.b[475]) {
            s.copy_ad(106, 323);
        }

        if (!s.b[474]) {
            s.store_scalar(106, 0.0);
        }

        s.store_scaled_voltage(236, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(237, ctx, nodes, Some(6), Some(8), p.p3);

        s.store_scaled_voltage(238, ctx, nodes, Some(6), Some(4), p.p3);

        s.store_scaled_voltage(239, ctx, nodes, Some(5), Some(4), p.p3);

        s.store_scaled_voltage(240, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(242, ctx, nodes, Some(7), Some(8), p.p3);

        s.store_scaled_voltage(245, ctx, nodes, Some(2), Some(4), p.p3);

        s.store_scaled_voltage(246, ctx, nodes, Some(1), Some(5), p.p3);

        s.store_scaled_voltage(249, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(250, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(244, ctx, nodes, Some(10), Some(7), p.p3);

        s.store_scaled_voltage(243, ctx, nodes, Some(9), Some(10), p.p3);

        s.store_sub_ad_lhs(241, A::add_scaled_inputs3(s.ad_value(240), 1.0, s.ad_value(237), 1.0, s.ad_value(242), -1.0), 244);

        s.store_sub_ad_lhs(248, A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(250), (-1.0), s.ad_value(241), 1.0), 243);

        s.store_add(247, 250, 248);

        s.b[476] = ((s.v[237] * s.v[8]) < p.p138);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if s.b[476] {
            s.store_exp_mul(251, 237, 8);
        }

        if (!s.b[476]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(251, 281, A::mul(s.ad_value(237), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[477] = (((s.v[238] * s.v[8]) / s.v[48]) < p.p138);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if s.b[477] {
            s.store_exp_ad(252, A::div_scaled_product(s.ad_value(238), s.ad_value(8), 1.0, s.ad_value(48), 1.0));
        }

        if (!s.b[477]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(252, 281, A::div_scaled_product(s.ad_value(238), s.ad_value(8), 1.0, s.ad_value(48), 1.0), (((-p.p138)) + (1.0)));
        }

        s.b[478] = ((s.v[241] * s.v[8]) < p.p138);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if s.b[478] {
            s.store_exp_mul(254, 241, 8);
        }

        if (!s.b[478]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(254, 281, A::mul(s.ad_value(241), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[479] = ((s.v[240] * s.v[8]) < p.p138);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if s.b[479] {
            s.store_exp_mul(253, 240, 8);
        }

        if (!s.b[479]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(253, 281, A::mul(s.ad_value(240), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[480] = ((s.v[247] * s.v[8]) < p.p138);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if s.b[480] {
            s.store_exp_mul(255, 247, 8);
        }

        if (!s.b[480]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(255, 281, A::mul(s.ad_value(247), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[481] = (((s.v[247] - s.v[16]) * s.v[8]) < p.p138);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if s.b[481] {
            s.store_exp_ad(258, A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[481]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(258, 281, A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[482] = (((s.v[241] - s.v[16]) * s.v[8]) < p.p138);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if s.b[482] {
            s.store_exp_ad(256, A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[482]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(256, 281, A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[483] = (((s.v[237] - s.v[16]) * s.v[8]) < p.p138);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if s.b[483] {
            s.store_exp_ad(257, A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[483]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(257, 281, A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[484] = (((s.v[236] - s.v[16]) * s.v[8]) < p.p138);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_exp_ad(259, A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[484]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(259, 281, A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(107, 257, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(108, 259, 4.0, 1.0);

        s.store_ad_value(109, A::div_scaled_inputs(s.ad_value(259), 2.0, A::offset(s.ad_value(108), 1.0), 1.0));

        s.b[485] = (s.v[109] < p.p140);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.store_scalar(109, p.p140);
        }

        s.store_mul_ad_rhs(110, 6, A::add_scaled_inputs3(s.ad_value(107), 1.0, s.ad_value(108), (-1.0), A::ln(A::div(A::offset(s.ad_value(107), 1.0), A::offset(s.ad_value(108), 1.0))), -1.0));

        s.store_div_ad_lhs(111, A::add(s.ad_value(110), s.ad_value(242)), 31);

        s.b[486] = (s.v[111] > 0.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        s.b[487] = (s.v[236] < 100.0);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[487]) {
            s.copy_ad(283, 236);
        }

        if (s.b[486] && (!s.b[487])) {
            s.store_offset_ln_ad(283, A::offset(s.ad_value(236), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[486] {
            s.store_sub_ad_lhs(112, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(6), A::ln(A::offset(A::mul3_scaled_output(s.ad_value(111), s.ad_value(31), s.ad_value(8), 0.5), 1.0)), 2.0), 283);
            s.store_scale(278, 16, 0.2);
            s.store_square(267, 278);
            s.store_square(268, 112);
        }

        s.b[488] = (s.v[112] < 0.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[488]) {
            s.store_ad_value(113, A::div_scaled_inputs(s.ad_value(267), 0.5, A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(112)), 1.0));
        }

        if (s.b[486] && (!s.b[488])) {
            s.store_scaled_add_ad_lhs(113, A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), 112, 0.5);
        }

        if s.b[486] {
            s.store_ad_value(114, A::div_scaled_product(s.ad_value(113), A::offset(s.ad_value(113), (p.p61 * p.p60)), 1.0, A::add_scaled_inputs(s.ad_value(113), p.p60, s.ad_value(31), (p.p61 * p.p60)), 1.0));
            s.store_div(271, 111, 114);
            s.store_scaled_offset(265, 271, (-1.0), 1.0 / (p.p62));
        }

        s.b[489] = (s.v[271] < 1.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[489]) {
            s.store_offset_scaled_ad(269, A::ln_one_plus_exp(s.ad_value(265)), p.p62, 1.0);
        }

        if (s.b[486] && (!s.b[489])) {
            s.store_ad_value(269, A::add_scaled_inputs(s.ad_value(271), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), p.p62));
        }

        if s.b[486] {
            s.store_scale(115, 269, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
            s.store_scale(116, 113, 1.0 / ((p.p61 * p.p60)));
            s.store_div_ad(117, A::offset(A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(115), s.ad_value(116), A::offset(s.ad_value(116), 1.0), 4.0), 1.0)), 1.0), A::mul_scaled_lhs(s.ad_value(115), 2.0, A::offset(s.ad_value(116), 1.0)));
            s.store_div_ad(118, A::add_scaled_sub_value_product(1.0, s.ad_value(117), 1.0, s.ad_value(109), s.ad_value(117), 1.0), A::offset(A::mul(s.ad_value(109), s.ad_value(117)), 1.0));
            s.store_mul_ad_lhs(120, A::mul3_scaled_output(s.ad_value(111), s.ad_value(31), s.ad_value(118), 0.5), 8);
            s.store_ad_value(272, A::add_scaled_product(s.ad_value(120), 2.0, s.ad_value(109), A::offset(A::add(s.ad_value(109), s.ad_value(120)), 1.0), 1.0));
            s.store_scaled_offset(121, 120, (-1.0), 0.5);
            s.store_add_ad_lhs(266, A::square(s.ad_value(121)), 272);
        }

        s.b[490] = (s.v[120] >= 1.0);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[490]) {
            s.store_add_ad_rhs(122, 121, A::sqrt(s.ad_value(266)));
        }

        if (s.b[486] && (!s.b[490])) {
            s.store_div_ad_rhs(122, 272, A::sub(A::sqrt(s.ad_value(266)), s.ad_value(121)));
        }

        s.b[491] = (s.v[122] < p.p139);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[491]) {
            s.store_scalar(122, p.p139);
        }

        if s.b[486] {
            s.store_mul_ad_product_rhs(124, 122, A::offset(s.ad_value(122), 1.0), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
            s.store_scaled_offset(126, 111, (-p.p61), (0.5 * p.p60));
            s.store_scaled_mul(127, 31, 111, (p.p60 * p.p61));
            s.store_add_ad_rhs(128, 126, A::sqrt(A::add(A::square(s.ad_value(126)), s.ad_value(127))));
        }

        s.b[492] = (p.p72 == 0.0);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[492]) {
            s.store_scale(129, 17, 0.1);
        }

        if (s.b[486] && (!s.b[492])) {
            s.store_mul_offset_ad_rhs(129, 17, A::div_scaled_inputs(s.ad_value(111), 2.0, A::add(s.ad_value(111), s.ad_value(114)), 1.0), 0.1);
        }

        if s.b[486] {
            s.store_ad_value(130, A::div_scaled_inputs(s.ad_value(111), p.p61, A::offset(s.ad_value(111), p.p61), 1.0));
            s.store_div_from_scalar_offset_input(202, p.p61, 111, p.p61);
        }

        if (!s.b[486]) {
            s.store_scalar(114, 0.0);
            s.store_ad_value(122, A::div_scaled_inputs(s.ad_value(257), 2.0, A::offset(s.ad_value(107), 1.0), 1.0));
            s.copy_ad(124, 251);
        }

        s.b[493] = ((((s.v[242]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[110]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[107] + s.v[108]))));
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if ((!s.b[486]) && s.b[493]) {
            s.store_scaled_add(131, 122, 109, 0.5);
            s.store_div_ad_rhs(118, 131, A::offset(s.ad_value(131), 1.0));
        }

        if ((!s.b[486]) && (!s.b[493])) {
            s.store_div_ad_rhs(118, 110, A::add_scaled_inputs3(s.ad_value(110), 1.0, s.ad_value(237), 1.0, s.ad_value(236), -1.0));
        }

        if (!s.b[486]) {
            s.copy_ad(128, 242);
            s.store_scale(129, 17, 0.1);
            s.copy_ad(130, 111);
            s.store_sub_from_scalar_ad(202, 1.0, A::scale(s.ad_value(130), 1.0 / (p.p61)));
        }

        s.store_scale(132, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(279, 14, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(238), s.ad_value(132)), 279);

        s.b[494] = (s.v[238] < s.v[132]);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_ad_value(133, A::add_scaled_product(s.ad_value(238), 1.0, s.ad_value(279), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[494]) {
            s.store_ad_value(133, A::add_scaled_product(s.ad_value(132), 1.0, s.ad_value(279), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (1.0 - p.p66));

        s.store_ad_value(134, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(238), 3.0, s.ad_value(133), (-3.0)));

        s.b[495] = (p.p73 == 1.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if s.b[495] {
            s.copy_ad(135, 236);
        }

        s.b[496] = (p.p73 == 2.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if ((!s.b[495]) && s.b[496]) {
            s.store_add(135, 236, 128);
        }

        if ((!s.b[495]) && (!s.b[496])) {
            s.copy_ad(135, 237);
        }

        s.store_div_ad(136, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(137, 17, 1.0, A::powf(s.ad_value(136), ((-1.0) / p.p71)));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(135), s.ad_value(137)), 129);

        s.b[497] = (s.v[135] < s.v[137]);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if s.b[497] {
            s.store_ad_value(138, A::add_scaled_product(s.ad_value(135), 1.0, s.ad_value(129), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[497]) {
            s.store_ad_value(138, A::add_scaled_product(s.ad_value(137), 1.0, s.ad_value(129), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_powf(139, 202, p.p75);

        s.store_add_ad(140, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(139), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(17))), (1.0 - p.p71))), 1.0 / ((1.0 - p.p71))), A::mul3(s.ad_value(139), s.ad_value(136), A::sub(s.ad_value(135), s.ad_value(138))));

        s.store_ad_value(141, A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(140)), 1.0, s.ad_value(25), s.ad_value(236), 1.0));

        s.store_scaled_div(142, 35, 36, 4.0);

        s.store_mul(143, 142, 252);

        s.store_div_ad_rhs(145, 143, A::offset(A::sqrt(A::offset(s.ad_value(143), 1.0)), 1.0));

        s.store_pow_ad(125, s.ad_value(124), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(144, 142, 125);

        s.store_div_ad_rhs(146, 144, A::offset(A::sqrt(A::offset(s.ad_value(144), 1.0)), 1.0));

        s.b[498] = (p.p91 == 0.0);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_add_ad(147, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));
        }

        if (!s.b[498]) {
            s.store_mul_ad_product_lhs(275, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), s.ad_value(100), 8);
            s.store_mul_ad_product_lhs(276, A::div_scaled_inputs(s.ad_value(141), -1.0, s.ad_value(40), 1.0), s.ad_value(100), 8);
            s.store_div_ad(147, A::sub(A::exp(s.ad_value(275)), A::exp(s.ad_value(276))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 147);

        s.b[499] = (s.v[147] < 0.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if s.b[499] {
            s.store_div_from_scalar_sub_ad(148, (0.5 * s.v[267]), A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(147));
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[499]) {
            s.store_scaled_add_ad_lhs(148, A::sqrt(A::offset(s.ad_value(268), s.v[267])), 147, 0.5);
        }

        s.store_mul_offset_ad_rhs(149, 148, A::add_scaled_inputs(s.ad_value(145), 0.5, s.ad_value(146), 0.5), 1.0);

        s.store_scaled_mul(150, 35, 125, p.p14);

        s.store_mul(151, 35, 252);

        s.store_div_ad_lhs(152, A::sub(s.ad_value(151), s.ad_value(150)), 149);

        s.store_scale(265, 238, 10000.0);

        s.b[500] = (s.v[238] < 0.0);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if s.b[500] {
            s.store_scaled_ln_one_plus_exp(282, 265, 0.0001);
        }

        if (!s.b[500]) {
            s.store_ad_value(282, A::add_scaled_inputs(s.ad_value(238), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.0001));
        }

        s.store_scale(284, 282, 1.0 / (p.p143));

        s.b[501] = (s.v[284] < p.p138);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.store_exp(285, 284);
        }

        if (!s.b[501]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_rhs(285, 281, 284, (((-p.p138)) + (1.0)));
        }

        s.store_mul_offset_rhs(333, 332, 285, (-1.0));

        s.store_scaled_offset(265, 238, (-p.p145), 1000.0);

        s.b[502] = (s.v[238] < p.p145);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if s.b[502] {
            s.store_ad_value(286, A::sub_scaled_inputs(s.ad_value(238), 1.0, A::ln_one_plus_exp(s.ad_value(265)), 0.001));
        }

        if (!s.b[502]) {
            s.store_sub_from_scalar_ad(286, p.p145, A::scale(A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.001));
        }

        s.store_mul_scaled_ad_rhs(334, 286, p.p146, A::powf(A::sub_from_scalar(p.p145, s.ad_value(286)), 2.0));

        s.b[503] = (((s.v[238] * s.v[8]) / p.p16) < p.p138);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if s.b[503] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p16)));
        }

        if (!s.b[503]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(238), s.ad_value(8), 1.0 / (p.p16)), (((-p.p138)) + (1.0)));
        }

        s.b[504] = (p.p23 == 1.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        s.b[505] = (((s.v[238] - s.v[55]) * s.v[8]) < p.p138);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if (s.b[504] && s.b[505]) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[504] && (!s.b[505])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(284, 281, A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[506] = (((s.v[152] / s.v[35]) - 1000.0) < 40.0);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (s.b[504] && s.b[506]) {
            s.store_exp_ad(285, A::offset(A::div(s.ad_value(152), s.ad_value(35)), (-1000.0)));
        }

        if (s.b[504] && (!s.b[506])) {
            s.store_scalar(281, ((40.0) as f64).exp());
            s.store_mul_offset_ad_rhs(285, 281, A::div(s.ad_value(152), s.ad_value(35)), (((((-1000.0)) + ((-40.0)))) + (1.0)));
        }

        if s.b[504] {
            let assign3760_ad_e3523: A = A::add(A::add_scaled_products(s.ad_value(42), A::offset(s.ad_value(282), (-1.0)), 1.0, A::div_scaled_product(s.ad_value(53), A::offset(s.ad_value(282), (-1.0)), 2.0, A::offset(A::sqrt(A::scale_offset(s.ad_value(284), 4.0, 1.0)), 1.0), 1.0), A::offset(A::div(s.ad_value(141), s.ad_value(40)), 1.0), 1.0), A::div_scaled_product3(s.ad_value(54), A::offset(s.ad_value(124), (-1.0)), s.ad_value(285), 1.0, A::offset(s.ad_value(285), 1.0), 1.0));
            s.store_ad_value(154, assign3760_ad_e3523);
        }

        s.b[507] = (p.p92 == 0.0);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if ((!s.b[504]) && s.b[507]) {
            s.store_mul_offset_rhs(154, 42, 282, (-1.0));
        }

        if ((!s.b[504]) && (!s.b[507])) {
            s.store_mul_ad_rhs(154, 42, A::add_scaled_product(A::scaled_offset(s.ad_value(282), (-1.0), (1.0 - p.p92)), 1.0, A::offset(A::add(s.ad_value(282), s.ad_value(124)), (-2.0)), A::offset(A::div(s.ad_value(141), s.ad_value(40)), 1.0), p.p92));
        }

        s.b[508] = (((s.v[239] * s.v[8]) / p.p18) < p.p138);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if s.b[508] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p18)));
        }

        if (!s.b[508]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(239), s.ad_value(8), 1.0 / (p.p18)), (((-p.p138)) + (1.0)));
        }

        s.b[509] = (p.p23 == 1.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        s.b[510] = (((s.v[239] - s.v[55]) * s.v[8]) < p.p138);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (s.b[509] && s.b[510]) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[509] && (!s.b[510])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(284, 281, A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        if s.b[509] {
            s.store_ad_value(155, A::add_scaled_product(A::div_scaled_product(s.ad_value(45), A::offset(s.ad_value(282), (-1.0)), 2.0, A::offset(A::sqrt(A::scale_offset(s.ad_value(284), 4.0, 1.0)), 1.0), 1.0), 1.0, s.ad_value(44), A::offset(s.ad_value(282), (-1.0)), 1.0));
        }

        if (!s.b[509]) {
            s.store_mul_offset_rhs(155, 44, 282, (-1.0));
        }

        s.b[511] = (((s.v[238] * s.v[8]) / p.p20) < p.p138);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if s.b[511] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p20)));
        }

        if (!s.b[511]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(238), s.ad_value(8), 1.0 / (p.p20)), (((-p.p138)) + (1.0)));
        }

        s.store_mul_offset_rhs(156, 38, 282, (-1.0));

        s.b[512] = (((s.v[239] * s.v[8]) / p.p22) < p.p138);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if s.b[512] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p22)));
        }

        if (!s.b[512]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(239), s.ad_value(8), 1.0 / (p.p22)), (((-p.p138)) + (1.0)));
        }

        s.store_mul_offset_rhs(158, 46, 282, (-1.0));

        s.b[513] = (((s.v[241] * s.v[8]) / p.p31) < p.p138);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(241), s.ad_value(8)), 1.0 / (p.p31)));
        }

        if (!s.b[513]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(241), s.ad_value(8), 1.0 / (p.p31)), (((-p.p138)) + (1.0)));
        }

        s.store_mul_offset_rhs(157, 39, 282, (-1.0));

        s.b[514] = (((s.v[239] * s.v[8]) / p.p137) < p.p138);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if s.b[514] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p137)));
        }

        if (!s.b[514]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(239), s.ad_value(8), 1.0 / (p.p137)), (((-p.p138)) + (1.0)));
        }

        s.store_mul_offset_rhs(159, 47, 282, (-1.0));

        s.b[515] = (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[238] < 0.0));
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        s.b[516] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p138);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if (s.b[515] && s.b[516]) {
            s.store_exp_ad(68, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))));
        }

        if (s.b[515] && (!s.b[516])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(68, 281, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))), (((-p.p138)) + (1.0)));
        }

        if s.b[515] {
            s.store_mul(261, 238, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(261)), 1e-30)), ((-2.0) - p.p66)), A::sub_scaled_inputs(A::sub_from_scalar((1.0 - (p.p66 * p.p66)), A::scale(s.ad_value(261), (3.0 * (p.p66 - 1.0)))), p.p66, A::mul3_scaled_output(s.ad_value(261), s.ad_value(261), A::offset(s.ad_value(261), (p.p66 - 1.0)), 6.0), 1.0), 0.16666666666666666);
            s.store_ad_value(261, A::div_scaled_product(s.ad_value(238), s.ad_value(61), s.v[62], A::mul(s.ad_value(70), s.ad_value(60)), 1.0));
        }

        s.b[517] = (s.v[261] < (-0.001));
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        s.b[518] = (s.v[261] < p.p138);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if ((s.b[515] && s.b[517]) && s.b[518]) {
            s.store_exp(91, 261);
        }

        if ((s.b[515] && s.b[517]) && (!s.b[518])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_rhs(91, 281, 261, (((-p.p138)) + (1.0)));
        }

        if (s.b[515] && s.b[517]) {
            s.store_mul_scaled_ad_rhs(69, 238, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(261)), 1.0));
        }

        if (s.b[515] && (!s.b[517])) {
            s.store_mul_ad_affine_product_rhs(69, 238, s.ad_value(261), A::offset(A::mul_scaled_lhs(s.ad_value(261), 0.3333333333333333, A::scale_offset(s.ad_value(261), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[515] {
            s.store_mul_ad_affine_product_lhs(57, A::mul3_scaled_output(s.ad_value(58), s.ad_value(69), s.ad_value(59), 2.0), s.ad_value(68), s.v[63], 0.0, 65);
        }

        if (!s.b[515]) {
            s.store_scalar(69, 0.0);
            s.store_scalar(57, 0.0);
        }

        s.b[519] = (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[236] < 0.0));
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(236), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[520] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p138);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (s.b[519] && s.b[520]) {
            s.store_exp_ad(78, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))));
        }

        if (s.b[519] && (!s.b[520])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(78, 281, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))), (((-p.p138)) + (1.0)));
        }

        if s.b[519] {
            s.store_mul(263, 236, 67);
        }

        if s.b[519] {
            let assign4360_ad_e4213: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(263)), 1e-30)), ((-2.0) - s.v[76])), A::sub_scaled_inputs(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(263), (3.0 * (s.v[76] - 1.0)))), s.v[76], A::mul3_scaled_output(s.ad_value(263), s.ad_value(263), A::offset(s.ad_value(263), (s.v[76] - 1.0)), 6.0), 1.0), 0.16666666666666666);
            s.store_ad_value(80, assign4360_ad_e4213);
        }

        if s.b[519] {
            s.store_ad_value(263, A::div_scaled_product(s.ad_value(236), s.ad_value(83), s.v[79], A::mul(s.ad_value(85), s.ad_value(80)), 1.0));
        }

        s.b[521] = (s.v[263] < (-0.001));
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        s.b[522] = (s.v[263] < p.p138);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if ((s.b[519] && s.b[521]) && s.b[522]) {
            s.store_exp(92, 263);
        }

        if ((s.b[519] && s.b[521]) && (!s.b[522])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_rhs(92, 281, 263, (((-p.p138)) + (1.0)));
        }

        if (s.b[519] && s.b[521]) {
            s.store_mul_scaled_ad_rhs(81, 236, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(263)), 1.0));
        }

        if (s.b[519] && (!s.b[521])) {
            s.store_mul_ad_affine_product_rhs(81, 236, s.ad_value(263), A::offset(A::mul_scaled_lhs(s.ad_value(263), 0.3333333333333333, A::scale_offset(s.ad_value(263), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[519] {
            s.store_mul_ad_affine_product_lhs(82, A::mul3_scaled_output(s.ad_value(84), s.ad_value(81), s.ad_value(77), 2.0), s.ad_value(78), s.v[89], 0.0, 67);
        }

        if (!s.b[519]) {
            s.store_scalar(81, 0.0);
            s.store_scalar(82, 0.0);
        }

        s.store_mul(161, 142, 254);

        s.store_scale(162, 256, 4.0);

        s.store_div_ad(164, A::sub(s.ad_value(161), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(161), 1.0)), 1.0));

        s.store_div_ad_rhs(163, 162, A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0));

        s.store_ad_value(160, A::div_scaled_product(s.ad_value(43), A::offset(s.ad_value(254), (-1.0)), 2.0, A::offset(A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(43), 4.0, s.ad_value(37), 1.0), s.ad_value(254)), 1.0)), 1.0), 1.0));

        s.b[523] = ((p.p5 > 0.0) && (p.p32 > 0.0));
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if s.b[523] {
            s.store_scale(160, 160, s.v[153]);
            s.store_ad_value(167, A::div_scaled_product(s.ad_value(43), A::offset(s.ad_value(255), (-1.0)), (p.p32 * 2.0), A::offset(A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(43), 4.0, s.ad_value(37), 1.0), s.ad_value(255)), 1.0)), 1.0), 1.0));
            s.store_scalar(168, 0.0);
        }

        s.b[524] = (p.p5 == 1.0);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (s.b[523] && s.b[524]) {
            s.store_scaled_mul(277, 43, 32, p.p32);
            s.store_mul_sub_from_scalar_ad_rhs(169, 6, 2.0, A::ln(A::mul(s.ad_value(277), s.ad_value(8))));
            s.store_sub(270, 247, 169);
            s.store_scalar(267, (0.11 * 0.11));
            s.store_square(268, 270);
        }

        s.b[525] = (s.v[270] < 0.0);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[525]) {
            s.store_ad_value(170, A::div_scaled_inputs(s.ad_value(267), 0.5, A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(270)), 1.0));
        }

        if ((s.b[523] && s.b[524]) && (!s.b[525])) {
            s.store_scaled_add_ad_lhs(170, A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), 270, 0.5);
        }

        if (s.b[523] && s.b[524]) {
            s.store_div_ad_rhs(171, 170, A::add(A::add_scaled_product(s.ad_value(277), 1.0, A::add(s.ad_value(167), s.ad_value(168)), s.ad_value(32), 1.0), s.ad_value(170)));
        }

        if (s.b[523] && (!s.b[524])) {
            s.store_scalar(169, 0.0);
            s.store_scalar(270, 0.0);
            s.store_scalar(170, 0.0);
            s.store_scalar(171, 1.0);
        }

        if s.b[523] {
            s.store_mul(172, 171, 167);
        }

        s.b[526] = (p.p83 == 1.0);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_add(328, 240, 236);
            s.store_scalar(267, (1e-6 * 1e-6));
            s.store_scaled_mul(268, 328, 328, ((-1.0) * (-1.0)));
        }

        s.b[527] = (((-1.0) * s.v[328]) < 0.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if (s.b[526] && s.b[527]) {
            s.store_ad_value(329, A::div_scaled_inputs(s.ad_value(267), 0.5, A::sub_scaled_inputs(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), 1.0, s.ad_value(328), (-1.0)), 1.0));
        }

        if (s.b[526] && (!s.b[527])) {
            s.store_ad_value(329, A::add_scaled_inputs(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), 0.5, s.ad_value(328), ((-1.0) * 0.5)));
        }

        if s.b[526] {
            s.store_scalar(330, (1.0 / (1.0 - ((s.v[324]) as f64).powf(p.p81))));
            s.store_scalar(325, (s.v[324] * p.p80));
            s.store_scaled_square(327, 330, (((s.v[324]) as f64).powf((p.p81 - 1.0)) * (p.p81 * 1.0 / (p.p80))));
        }

        s.b[528] = (s.v[329] < s.v[325]);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if (s.b[526] && s.b[528]) {
            s.store_div_from_scalar_sub_from_scalar_ad(326, 1.0, 1.0, A::powf(A::scale(s.ad_value(329), 1.0 / (p.p80)), p.p81));
        }

        if (s.b[526] && (!s.b[528])) {
            s.store_ad_value(326, A::add_scaled_product(s.ad_value(330), 1.0, A::sub(s.ad_value(329), s.ad_value(325)), s.ad_value(327), 1.0));
        }

        if (!s.b[526]) {
            s.store_scalar(326, 1.0);
        }

        s.store_mul(82, 82, 326);

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul(160, 160, 326);

        s.store_mul(157, 157, 326);

        s.store_mul(172, 172, 326);

        s.store_add_ad(175, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 175);

        s.b[529] = (s.v[175] < 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_div_from_scalar_sub_ad(176, (0.5 * s.v[267]), A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(175));
        }

        if (!s.b[529]) {
            s.store_scaled_add_ad_lhs(176, A::sqrt(A::offset(s.ad_value(268), s.v[267])), 175, 0.5);
        }

        s.store_mul_offset_ad_rhs(177, 176, A::add_scaled_inputs(s.ad_value(145), 0.5, s.ad_value(146), 0.5), 1.0);

        s.store_div(179, 29, 177);

        s.b[530] = (s.v[179] < s.v[322]);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if s.b[530] {
            s.copy_ad(179, 322);
        }

        s.store_scale(178, 179, 3.0);

        s.store_div_ad_lhs(180, A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(6), A::offset(s.ad_value(253), (-1.0)), 2.0), 178);

        s.b[531] = (s.v[152] > 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        s.b[532] = (p.p38 == 1.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        s.b[533] = (s.v[236] < p.p43);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        s.b[534] = (((-s.v[152]) / p.p41) < p.p138);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (((s.b[531] && s.b[532]) && s.b[533]) && s.b[534]) {
            s.store_exp_scaled_input(314, 152, (-1.0 / (p.p41)));
        }

        if (((s.b[531] && s.b[532]) && s.b[533]) && (!s.b[534])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_ad_rhs(314, 281, A::scale_offset(s.ad_value(152), (-1.0 / (p.p41)), (((-p.p138)) + (1.0))));
        }

        if ((s.b[531] && s.b[532]) && s.b[533]) {
            s.store_mul_sub_from_scalar_lhs(315, p.p43, 236, 314);
        }

        s.b[535] = (((-s.v[316]) * ((s.v[315]) as f64).powf(p.p40)) < p.p138);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((s.b[531] && s.b[532]) && s.b[533]) && s.b[535]) {
            s.store_exp_ad(319, A::mul_scaled_lhs(s.ad_value(316), -1.0, A::powf(s.ad_value(315), p.p40)));
        }

        if (((s.b[531] && s.b[532]) && s.b[533]) && (!s.b[535])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(319, 281, A::mul_scaled_lhs(s.ad_value(316), -1.0, A::powf(s.ad_value(315), p.p40)), (((-p.p138)) + (1.0)));
        }

        if ((s.b[531] && s.b[532]) && s.b[533]) {
            s.store_mul_ad_product_lhs(199, A::div_from_scalar(p.p39, s.ad_value(316)), s.ad_value(315), 319);
        }

        s.b[536] = (p.p38 == 2.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (s.v[236] < s.v[16]);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) {
            s.store_scalar(188, ((2.0 * p.p45) / (p.p44 * p.p44)));
            s.store_div_ad_lhs(266, A::sub(s.ad_value(16), s.ad_value(236)), 202);
            s.store_sqrt_ad(189, A::div_scaled_inputs(s.ad_value(266), 2.0, s.ad_value(188), 1.0));
        }

        s.b[538] = (p.p7 == 0.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && s.b[538]) {
            s.store_scalar(190, p.p44);
        }

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && (!s.b[538])) {
            s.store_sub_from_scalar_ad(119, 1.0, A::scale(s.ad_value(118), 0.5));
            s.store_scaled_mul(190, 119, 119, p.p44);
        }

        if (((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) {
            s.store_ad_value(191, A::div_scaled_product(s.ad_value(189), s.ad_value(190), 1.0, A::sqrt(A::add(A::square(s.ad_value(189)), A::square(s.ad_value(190)))), 1.0));
            s.store_div_ad_lhs(192, A::sub(s.ad_value(16), s.ad_value(236)), 191);
            s.store_add_ad_rhs(193, 192, A::mul3_scaled_output(s.ad_value(191), s.ad_value(188), s.ad_value(202), 0.5));
        }

        s.b[539] = (p.p7 == 0.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && s.b[539]) {
            s.copy_ad(194, 193);
        }

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && (!s.b[539])) {
            s.store_offset_scaled(195, 118, ((2.0) * ((2.0 * p.p46))), (((2.0 * p.p46)) + (1.0)));
            s.store_scalar(196, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
            s.store_sub_ad_rhs(197, 192, A::mul3_scaled_output(s.ad_value(191), s.ad_value(188), A::sub(s.ad_value(196), A::div_scaled_inputs(s.ad_value(152), 1.0, s.ad_value(195), p.p61)), 0.5));
            s.store_ad_value(266, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(192), s.ad_value(192), s.ad_value(130), (0.1 * 1.0 / (p.p61))), 1.0, A::sub(s.ad_value(197), s.ad_value(193)), A::sub(s.ad_value(197), s.ad_value(193)), 1.0));
            s.store_ad_value(194, A::add_scaled_inputs3(s.ad_value(197), 0.5, s.ad_value(193), 0.5, A::sqrt(s.ad_value(266)), 0.5));
        }

        if (((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) {
            s.store_div_ad_lhs(273, A::sub(s.ad_value(194), s.ad_value(192)), 194);
        }

        s.b[540] = (((s.v[273]) as f64).abs() > 1e-7);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && s.b[540]) {
            s.store_scaled_div(198, 191, 273, 0.5);
            s.store_mul_ad(199, A::mul3(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(194), s.ad_value(198)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(194), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(194), 1.0), A::div(s.ad_value(190), s.ad_value(198)), 1.0))));
        }

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && (!s.b[540])) {
            s.store_mul_ad_product_rhs(199, 0, s.ad_value(190), A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(194), 1.0)));
        }

        s.b[541] = (p.p38 == 3.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        s.b[542] = (s.v[236] < p.p43);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) {
            s.store_mul_ad(203, A::powf(A::sub_from_scalar(p.p43, s.ad_value(236)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(152), A::offset(s.ad_value(152), p.p47))), p.p48));
        }

        s.b[543] = (p.p7 == 0.0);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && s.b[543]) {
            s.copy_ad(204, 203);
        }

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) {
            s.store_scaled_offset(205, 152, (-p.p51), 1.0 / (p.p47));
            s.store_scaled_offset(265, 205, (-1.0), 1.0 / (p.p50));
        }

        s.b[544] = (s.v[205] < 1.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if ((((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) && s.b[544]) {
            s.store_offset_scaled_ad(206, A::ln_one_plus_exp(s.ad_value(265)), p.p50, 1.0);
        }

        if ((((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) && (!s.b[544])) {
            s.store_ad_value(206, A::add_scaled_inputs(s.ad_value(205), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), p.p50));
        }

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) {
            s.store_mul_powf_ad_rhs(204, 203, s.ad_value(206), p.p49);
        }

        s.b[545] = (((-s.v[316]) * s.v[204]) < p.p138);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && s.b[545]) {
            s.store_exp_ad(319, A::mul_scaled_lhs(s.ad_value(316), -1.0, s.ad_value(204)));
        }

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[545])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(319, 281, A::mul_scaled_lhs(s.ad_value(316), -1.0, s.ad_value(204)), (((-p.p138)) + (1.0)));
        }

        if ((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) {
            s.store_mul_ad_lhs(199, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p39, s.ad_value(316)), p.p43, s.ad_value(236)), 319);
        }

        s.b[546] = (s.v[199] > 0.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        s.b[547] = (p.p52 == 1.0);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if ((s.b[531] && s.b[546]) && s.b[547]) {
            s.store_add_ad(200, A::add_scaled_product(A::div(s.ad_value(6), A::mul(s.ad_value(152), A::add(s.ad_value(30), s.ad_value(178)))), 1.0, A::div(s.ad_value(149), s.ad_value(35)), s.ad_value(42), 1.0), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(178))));
        }

        s.b[548] = (p.p38 == 3.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (((s.b[531] && s.b[546]) && s.b[547]) && s.b[548]) {
            s.store_scaled_sub(265, 199, 200, 1000000.0);
        }

        s.b[549] = (s.v[199] < s.v[200]);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((((s.b[531] && s.b[546]) && s.b[547]) && s.b[548]) && s.b[549]) {
            s.store_ad_value(199, A::sub_scaled_inputs(s.ad_value(199), 1.0, A::ln_one_plus_exp(s.ad_value(265)), 1e-6));
        }

        if ((((s.b[531] && s.b[546]) && s.b[547]) && s.b[548]) && (!s.b[549])) {
            s.store_ad_value(199, A::sub_scaled_inputs(s.ad_value(200), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), 1e-6));
        }

        if (((s.b[531] && s.b[546]) && s.b[547]) && s.b[548]) {
            s.store_mul(201, 152, 199);
        }

        if (((s.b[531] && s.b[546]) && s.b[547]) && (!s.b[548])) {
            s.store_ad_value(201, A::div_scaled_product3(s.ad_value(152), s.ad_value(199), s.ad_value(200), 1.0, A::add(s.ad_value(199), s.ad_value(200)), 1.0));
        }

        if ((s.b[531] && s.b[546]) && (!s.b[547])) {
            s.store_mul(201, 152, 199);
        }

        s.b[550] = (s.v[124] > 0.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_mul_ln_rhs(123, 6, 124);
        }

        if (!s.b[550]) {
            s.copy_ad(123, 237);
        }

        s.b[551] = (p.p23 == 1.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if s.b[551] {
            s.copy_ad(93, 236);
        }

        if (!s.b[551]) {
            s.copy_ad(93, 237);
        }

        let assign5720_ad_e5777: A = A::add_scaled_inputs_product(A::add_scaled_product(A::add_scaled_products(s.ad_value(152), A::sub(s.ad_value(238), s.ad_value(123)), 1.0, s.ad_value(111), A::sub(s.ad_value(123), s.ad_value(236)), 1.0), 1.0, s.ad_value(201), s.ad_value(123), (-1.0)), 1.0, A::div_scaled_product(s.ad_value(245), s.ad_value(245), 1.0, s.ad_value(28), 1.0), 1.0, A::square(s.ad_value(248)), s.ad_value(104), 1.0);
        let assign5720_ad_e5815: A = A::add_scaled_product(A::add_scaled_inputs_product(A::add_scaled_product(A::add_scaled_product(assign5720_ad_e5777, 1.0, A::square(s.ad_value(243)), s.ad_value(105), 1.0), 1.0, A::square(s.ad_value(244)), s.ad_value(106), 1.0), 1.0, A::div_scaled_product(s.ad_value(246), s.ad_value(246), 1.0, s.ad_value(30), 1.0), 1.0, s.ad_value(180), s.ad_value(240), 1.0), 1.0, A::add(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(154), 1.0, s.ad_value(156), 1.0, s.ad_value(238), s.v[320]), 1.0, s.ad_value(57), (-1.0), s.ad_value(334), 1.0), s.ad_value(333)), s.ad_value(238), 1.0);
        let assign5720_ad_e5841: A = A::add_scaled_product(A::add_scaled_product(A::add_scaled_product(A::add_scaled_product(assign5720_ad_e5815, 1.0, s.ad_value(82), s.ad_value(93), (-1.0)), 1.0, A::add_scaled_inputs3(s.ad_value(155), 1.0, s.ad_value(158), 1.0, s.ad_value(159), 1.0), s.ad_value(239), 1.0), 1.0, A::add_scaled_inputs3(s.ad_value(160), 1.0, s.ad_value(157), 1.0, s.ad_value(241), s.v[320]), s.ad_value(241), 1.0), 1.0, s.ad_value(172), s.ad_value(247), 1.0);
        s.store_ad_value(208, assign5720_ad_e5841);

        s.store_scaled_mul(210, 23, 134, (1.0 - p.p67));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(239), s.ad_value(132)), 279);

        s.b[552] = (s.v[239] < s.v[132]);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if s.b[552] {
            s.store_ad_value(211, A::add_scaled_product(s.ad_value(239), 1.0, s.ad_value(279), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[552]) {
            s.store_ad_value(211, A::add_scaled_product(s.ad_value(132), 1.0, s.ad_value(279), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_mul_scaled_ad_rhs(212, 23, p.p67, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(211), s.ad_value(65))), (1.0 - p.p66)), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(239), 3.0, s.ad_value(211), (-3.0)));

        s.store_scaled_mul(213, 24, 141, p.p76);

        s.store_mul(214, 95, 36);

        s.store_mul3_affine_lhs(218, 214, 145, 0.5, 0.0, 176);

        s.store_mul3_affine_lhs(219, 214, 146, 0.5, 0.0, 176);

        s.store_scale(280, 17, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(241), s.ad_value(137)), 280);

        s.b[553] = (s.v[241] < s.v[137]);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if s.b[553] {
            s.store_ad_value(220, A::add_scaled_product(s.ad_value(241), 1.0, s.ad_value(280), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[553]) {
            s.store_ad_value(220, A::add_scaled_product(s.ad_value(137), 1.0, s.ad_value(280), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_ad_value(221, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(220), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, s.ad_value(136), A::sub(s.ad_value(241), s.ad_value(220)), 1.0));

        s.store_mul_scaled_ad_rhs(222, 24, ((1.0 - p.p76) * (1.0 - p.p32)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(221)), 1.0, s.ad_value(25), s.ad_value(241), 1.0));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(247), s.ad_value(137)), 280);

        s.b[554] = (s.v[247] < s.v[137]);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if s.b[554] {
            s.store_ad_value(223, A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(280), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[554]) {
            s.store_ad_value(223, A::add_scaled_product(s.ad_value(137), 1.0, s.ad_value(280), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_ad_value(224, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(223), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, s.ad_value(136), A::sub(s.ad_value(247), s.ad_value(223)), 1.0));

        s.store_mul_scaled_ad_rhs(225, 24, ((1.0 - p.p76) * p.p32), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(224)), 1.0, s.ad_value(25), s.ad_value(247), 1.0));

        s.store_mul_ad_product_rhs(226, 94, s.ad_value(36), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p84)));

        s.b[555] = ((s.v[238] / (p.p84 * s.v[6])) < p.p138);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_exp_ad(282, A::div_scaled_inputs(s.ad_value(238), 1.0, s.ad_value(6), p.p84));
        }

        if (!s.b[555]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::div_scaled_inputs(s.ad_value(238), 1.0, s.ad_value(6), p.p84), (((-p.p138)) + (1.0)));
        }

        s.store_mul(228, 226, 282);

        s.store_ad_value(229, A::div_scaled_product(s.ad_value(96), s.ad_value(6), 4.0, s.ad_value(31), 1.0));

        s.store_mul_ad_affine_product_rhs(230, 229, s.ad_value(118), A::offset(A::add(s.ad_value(122), s.ad_value(109)), 2.0), 0.5, 0.0);

        s.b[556] = (p.p78 == 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_ad_value(235, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(214), s.ad_value(164), 1.0, s.ad_value(229), s.ad_value(163), 1.0), 0.5, A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[557] = ((((s.v[241] - s.v[22]) / p.p90) * s.v[8]) < p.p138);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if ((!s.b[556]) && s.b[557]) {
            s.store_exp_ad(173, A::mul_scaled_lhs(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90), s.ad_value(8)));
        }

        if ((!s.b[556]) && (!s.b[557])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(173, 281, A::mul_scaled_lhs(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        if (!s.b[556]) {
            s.store_ad_value(235, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(254), 2.0, A::offset(A::sqrt(A::scale_offset(s.ad_value(173), 4.0, 1.0)), 1.0), 1.0));
        }

        s.b[558] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0));
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scale(235, 235, s.v[153]);
        }

        s.b[559] = (p.p78 == 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if (s.b[558] && s.b[559]) {
            s.store_mul(165, 142, 255);
            s.store_div_ad(166, A::sub(s.ad_value(165), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0));
            s.store_scale(231, 258, 4.0);
            s.store_div_ad_rhs(232, 231, A::offset(A::sqrt(A::offset(s.ad_value(231), 1.0)), 1.0));
            s.store_ad_value(233, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(214), s.ad_value(166), 1.0, s.ad_value(229), s.ad_value(232), 1.0), (0.5 * p.p32), A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[560] = (((s.v[247] - s.v[22]) * s.v[8]) < p.p138);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if ((s.b[558] && (!s.b[559])) && s.b[560]) {
            s.store_exp_ad(174, A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)));
        }

        if ((s.b[558] && (!s.b[559])) && (!s.b[560])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(174, 281, A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        if (s.b[558] && (!s.b[559])) {
            s.store_ad_value(233, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(255), (2.0 * p.p32), A::offset(A::sqrt(A::scale_offset(s.ad_value(174), 4.0, 1.0)), 1.0), 1.0));
        }

        if s.b[558] {
            s.store_mul(234, 171, 233);
        }

        s.b[561] = (p.p6 == 1.0);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_offset_powf_ad(182, A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (-p.p66), (-3.0));
            s.store_div_ad_lhs(274, A::sub(s.ad_value(238), s.ad_value(132)), 279);
        }

        s.b[562] = (s.v[274] < 0.0);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if (s.b[561] && s.b[562]) {
            s.store_div_from_scalar_offset_ad(183, 1.0, A::exp(s.ad_value(274)), 1.0);
        }

        if (s.b[561] && (!s.b[562])) {
            s.store_div_ad(183, A::exp_scaled_input(s.ad_value(274), -1.0), A::offset(A::exp_scaled_input(s.ad_value(274), -1.0), 1.0));
        }

        if s.b[561] {
            s.store_offset_mul(181, 182, 183, 3.0);
            s.store_scaled_mul(184, 23, 181, (1.0 - p.p67));
            s.store_mul_ad(187, A::div_scaled_product3(s.ad_value(142), s.ad_value(252), s.ad_value(8), 1.0, s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(143), 1.0))));
            s.store_mul3_affine_lhs(185, 214, 176, 0.5, 0.0, 187);
            s.store_scaled_div(186, 228, 6, (1.0 / (p.p84)));
            s.store_mul_scaled_ad_rhs(217, 240, 0.2, A::add_scaled_inputs3(s.ad_value(184), 1.0, s.ad_value(185), 1.0, s.ad_value(186), 1.0));
        }

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
        if s.b[561] {
            s.store_scale(227, 228, (1.0 - p.p94));
            s.store_add_scaled_inputs(313, 218, 1.0, 228, p.p94);
            s.store_add_scaled_inputs(216, 313, p.p93, 219, 1.0);
            s.store_scale(215, 313, (1.0 - p.p93));
        }

        if (!s.b[561]) {
            s.copy_ad(215, 218);
            s.copy_ad(216, 219);
            s.copy_ad(227, 228);
        }

        s.b[563] = (p.p23 == 1.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        let assign6450_ad_e6586: A = A::ddt(A::scale(A::voltage(ctx, nodes, Some(3), None), p.p134), ddt_scale, eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, A::scale(A::voltage(ctx, nodes, Some(3), None), p.p134).value));
        s.store_scale_ad(209, assign6450_ad_e6586, p.p1);

        s.v[331] = (1.0 - p.p135);

        s.b[564] = (p.p133 > s.v[322]);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        s.b[565] = (p.p132 == 0.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        if (s.b[564] && s.b[565]) {
            s.store_scaled_voltage(102, ctx, nodes, Some(3), None, (1.0 / (s.v[103]) * p.p1));
        }

        s.b[566] = (((s.v[331]) as f64).abs() < 1e-6);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if ((s.b[564] && (!s.b[565])) && s.b[566]) {
            s.store_scaled_ln_ad(102, A::scale_offset(A::voltage(ctx, nodes, Some(3), None), 1.0 / (s.v[5]), 1.0), ((s.v[5] / s.v[103]) * p.p1));
        }

        if ((s.b[564] && (!s.b[565])) && (!s.b[566])) {
            s.store_scaled_offset_ad(102, A::powf(A::scale_offset(A::voltage(ctx, nodes, Some(3), None), 1.0 / (s.v[5]), 1.0), s.v[331]), (-1.0), ((s.v[5] / (s.v[331] * s.v[103])) * p.p1));
        }

        if (!s.b[564]) {
            s.store_div_voltage_by_ad(102, ctx, nodes, Some(3), None, s.ad_value(321));
        }

        s.b[567] = (p.p57 > 0.0);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        s.b[568] = (p.p58 > 0.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        s.store_scale(287, 2, (4.0 * 1.3806226e-23));

        s.store_div(288, 287, 28);

        s.store_div(289, 287, 30);

        s.store_mul(290, 287, 104);

        s.store_mul(291, 287, 105);

        s.store_mul(292, 287, 106);

        s.store_scaled_mul_ad(293, A::div(s.ad_value(287), s.ad_value(178)), A::scale_offset(s.ad_value(253), 4.0, 5.0), 0.3333333333333333);

        s.store_div_ad_lhs(309, A::add(s.ad_value(151), s.ad_value(150)), 149);

        s.store_scaled_abs(294, 309, (2.0 * 1.6021918e-19));

        s.b[569] = (p.p129 > 0.0);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if s.b[569] {
            s.store_abs_ad(310, A::div(s.ad_value(201), s.ad_value(309)));
        }

        if (!s.b[569]) {
            s.store_scalar(310, 0.0);
        }

        s.store_mul_scaled_ad_rhs(306, 201, (2.0 * 1.6021918e-19), A::offset(s.ad_value(310), 1.0));

        s.b[570] = (s.v[309] > 0.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_div_ad_lhs(311, A::add(s.ad_value(215), s.ad_value(216)), 309);
        }

        if (!s.b[570]) {
            s.store_mul3_lhs(311, 95, 176, 149);
        }

        s.b[571] = (p.p130 == 1.0);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if s.b[571] {
            s.store_scale(312, 311, p.p93);
        }

        s.b[572] = (p.p130 == 2.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if ((!s.b[571]) && s.b[572]) {
            s.store_scale(312, 311, p.p131);
        }

        if ((!s.b[571]) && (!s.b[572])) {
            s.store_scalar(312, 0.0);
        }

        s.store_scaled_abs_ad(295, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(154), 1.0, s.ad_value(156), 1.0, s.ad_value(57), -1.0), 1.0, s.ad_value(334), 1.0, s.ad_value(333), 1.0), (2.0 * 1.6021918e-19));

        s.store_add(307, 154, 155);

        s.store_scaled_powf_ad(296, A::abs(s.ad_value(307)), p.p125, p.p127);

        s.b[573] = (s.v[307] < 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if s.b[573] {
            s.store_neg(296, 296);
        }

        s.store_ad_value(308, A::add_scaled_inputs3(s.ad_value(156), 1.0, s.ad_value(158), 1.0, s.ad_value(159), 1.0));

        s.store_scaled_powf_ad(297, A::abs(s.ad_value(308)), p.p126, p.p128);

        s.b[574] = (s.v[308] < 0.0);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if s.b[574] {
            s.store_neg(297, 297);
        }

        s.store_scaled_abs_ad(298, A::add_scaled_inputs3(s.ad_value(155), 1.0, s.ad_value(158), 1.0, s.ad_value(159), 1.0), (2.0 * 1.6021918e-19));

        s.store_scaled_abs(299, 157, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(300, A::abs(s.ad_value(157)), p.p125, p.p127);

        s.b[575] = (s.v[157] < 0.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        if s.b[575] {
            s.store_neg(300, 300);
        }

        s.store_scaled_abs(301, 82, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(302, 160, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(304, A::scale(A::abs(s.ad_value(160)), 1.0 / ((1.0 - (p.p5 * p.p32)))), p.p125, (p.p127 * (1.0 - (p.p5 * p.p32))));

        s.b[576] = (s.v[160] < 0.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if s.b[576] {
            s.store_neg(304, 304);
        }

        s.store_scaled_abs(303, 172, ((2.0 * 1.6021918e-19) * p.p5));

        s.b[577] = (p.p32 == 0.0);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if s.b[577] {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[577]) {
            s.store_scaled_powf_ad(305, A::scale(A::abs(s.ad_value(172)), 1.0 / (p.p32)), p.p125, ((p.p127 * p.p5) * p.p32));
        }

        s.b[578] = (s.v[172] < 0.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if s.b[578] {
            s.store_neg(305, 305);
        }

        s.b[579] = (p.p23 == 1.0);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        s.b[580] = (p.p57 > 0.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        s.b[581] = (p.p58 > 0.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        s.b[582] = (p.p58 > 0.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[447] = (p.p3 == 1.0);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if s.b[447] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[447]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[153] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.b[448] = (p.p141 == 0.0);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if s.b[448] {
            s.store_scalar(321, 1e-12);
        }

        if (!s.b[448]) {
            s.store_scalar(321, p.p141);
        }

        s.store_scale(322, 321, p.p1);

        s.v[52] = 0.001;

        s.v[318] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[265] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.b[449] = ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if s.b[449] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[449]) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[265])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[265] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.b[450] = ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if s.b[450] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[450]) {
            s.store_scalar(88, ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) + (0.1 * (((1.0 + (((-s.v[265])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p116;

        s.v[86] = (1.0 / s.v[87]);

        s.v[171] = 1.0;

        s.v[199] = 0.0;

        s.v[234] = 0.0;

        s.v[217] = 0.0;

        s.v[42] = 0.0;

        s.store_voltage(207, ctx, nodes, Some(3), None);

        s.b[451] = (s.v[207] < 0.0);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if s.b[451] {
            s.store_neg_ad(207, A::ln(A::sub_from_scalar(1.0, s.ad_value(207))));
        }

        s.b[452] = (s.v[207] < p.p124);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if s.b[452] {
            s.copy_ad(11, 207);
        }

        if (!s.b[452]) {
            s.store_offset_ln_ad(11, A::offset(s.ad_value(207), (((-p.p124)) + (1.0))), p.p124);
        }

        s.store_offset(2, 11, s.v[5]);

        s.store_scale(4, 2, 1.0 / (s.v[3]));

        s.store_scale(6, 2, 8.617086918058125e-5);

        s.v[7] = (8.617086918058125e-5 * s.v[3]);

        s.store_div_from_scalar(8, 1.0, 6);

        s.v[9] = (1.0 / s.v[7]);

        s.store_offset(10, 8, (-s.v[9]));

        s.store_offset(12, 2, (-s.v[3]));

        s.store_ln(260, 4);

        s.store_scaled_offset_ad(265, A::sub(s.ad_value(74), A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p114, A::offset(s.ad_value(2), p.p115), 1.0)), (-0.05), 10.0);

        s.b[453] = ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if s.b[453] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(265)), 0.1, 0.05);
        }

        if (!s.b[453]) {
            s.store_ad_value(70, A::add_scaled_inputs3(s.ad_value(74), 1.0, A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p114, A::offset(s.ad_value(2), p.p115), 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.1));
        }

        s.store_scaled_offset_ad(265, A::sub(s.ad_value(88), A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p117, A::offset(s.ad_value(2), p.p118), 1.0)), (-0.05), 10.0);

        s.b[454] = ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if s.b[454] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(265)), 0.1, 0.05);
        }

        if (!s.b[454]) {
            s.store_ad_value(85, A::add_scaled_inputs3(s.ad_value(88), 1.0, A::div_scaled_product(s.ad_value(2), s.ad_value(2), p.p117, A::offset(s.ad_value(2), p.p118), 1.0), (-1.0), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.1));
        }

        s.store_ad_value(13, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p65, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p104));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.b[455] = (0.05 < s.v[13]);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if s.b[455] {
            s.store_ad_value(14, A::add_scaled_product(s.ad_value(13), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[455]) {
            s.store_offset_mul_ad(14, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(15, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p63, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.b[456] = (0.05 < s.v[15]);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if s.b[456] {
            s.store_ad_value(16, A::add_scaled_product(s.ad_value(15), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[456]) {
            s.store_offset_mul_ad(16, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(21, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p79, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.b[457] = (0.05 < s.v[21]);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if s.b[457] {
            s.store_ad_value(22, A::add_scaled_product(s.ad_value(21), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[457]) {
            s.store_offset_mul_ad(22, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(18, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p70, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.b[458] = (0.05 < s.v[18]);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if s.b[458] {
            s.store_ad_value(17, A::add_scaled_product(s.ad_value(18), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[458]) {
            s.store_offset_mul_ad(17, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(20, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), s.v[75], s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.b[459] = (0.05 < s.v[20]);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if s.b[459] {
            s.store_ad_value(19, A::add_scaled_product(s.ad_value(20), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[459]) {
            s.store_offset_mul_ad(19, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_ad_value(56, A::add_scaled_inputs(A::add_scaled_product(s.ad_value(4), p.p26, s.ad_value(6), s.ad_value(260), (-3.0)), 1.0, A::sub_from_scalar(1.0, s.ad_value(4)), p.p108));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.b[460] = (0.05 < s.v[56]);
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if s.b[460] {
            s.store_ad_value(55, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(6), A::ln_one_plus_exp(s.ad_value(265)), 1.0));
        }

        if (!s.b[460]) {
            s.store_offset_mul_ad(55, s.ad_value(6), A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.05);
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p65), p.p66);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p64);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p70, s.ad_value(17)), p.p71), (1.0 - p.p74), p.p74);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p69);

        s.store_scale(25, 27, p.p74);

        s.store_scaled_exp_scaled_input(28, 260, p.p96, p.p53);

        s.b[461] = (s.v[28] < s.v[322]);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if s.b[461] {
            s.copy_ad(28, 322);
        }

        s.store_scaled_exp_scaled_input(29, 260, (p.p97 - p.p95), p.p55);

        s.store_scaled_exp_scaled_input(30, 260, p.p100, p.p54);

        s.b[462] = (s.v[30] < s.v[322]);
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if s.b[462] {
            s.copy_ad(30, 322);
        }

        s.store_scaled_exp_scaled_input(32, 260, p.p101, p.p56);

        s.store_scaled_exp_scaled_input(31, 260, p.p98, p.p59);

        s.b[463] = (p.p121 != 0.0);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        if s.b[463] {
            s.store_offset_scaled(50, 12, ((p.p121) * (p.p9)), p.p9);
            s.store_scaled_offset(265, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[464] = (s.v[50] < 1.0);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if (s.b[463] && s.b[464]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(265)), s.v[52], 1.0);
        }

        if (s.b[463] && (!s.b[464])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), s.v[52]));
        }

        if s.b[463] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[463]) {
            s.store_scalar(48, p.p9);
        }

        s.b[465] = (p.p122 != 0.0);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if s.b[465] {
            s.store_offset_scaled(51, 12, ((p.p122) * (p.p10)), p.p10);
            s.store_scaled_offset(265, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[466] = (s.v[51] < 1.0);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if (s.b[465] && s.b[466]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(265)), s.v[52], 1.0);
        }

        if (s.b[465] && (!s.b[466])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), s.v[52]));
        }

        if s.b[465] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[465]) {
            s.store_scalar(49, p.p10);
        }

        s.store_offset_scaled(317, 12, ((p.p123) * (p.p42)), p.p42);

        s.v[267] = (s.v[318] * s.v[318]);

        s.store_square(268, 317);

        s.b[467] = (s.v[317] < 0.0);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_div_from_scalar_sub_ad(316, (0.5 * s.v[267]), A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(317));
        }

        if (!s.b[467]) {
            s.store_scaled_add_ad_lhs(316, A::sqrt(A::offset(s.ad_value(268), s.v[267])), 317, 0.5);
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_scaled_inputs(s.ad_value(260), (((4.0 - p.p97) - p.p95) + p.p120), s.ad_value(48), 1.0)), A::exp(A::div_scaled_inputs(s.ad_value(10), (-p.p104), s.ad_value(48), 1.0)), p.p8);

        s.store_scaled_exp_scaled_input(36, 260, (1.0 - p.p97), p.p11);

        s.store_scaled_exp_scaled_input(37, 260, (1.0 - p.p102), p.p29);

        s.store_scaled_mul_ad(42, A::exp_scaled_input(s.ad_value(260), (((4.0 - p.p96) + p.p120) * 1.0 / (p.p16))), A::exp_scaled_input(s.ad_value(10), ((-p.p110) * 1.0 / (p.p16))), p.p15);

        s.store_scaled_mul_ad(43, A::exp_scaled_input(s.ad_value(260), ((4.0 - p.p102) + p.p120)), A::exp_scaled_input(s.ad_value(10), (-p.p111)), p.p28);

        s.store_powf_ad(261, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(262, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(261), p.p34), s.ad_value(262), (p.p65 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(263, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(264, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(263), p.p36), s.ad_value(264), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.store_exp_scaled_input(261, 260, p.p95);

        s.store_scaled_mul(40, 261, 27, p.p13);

        s.store_scaled_mul(41, 261, 262, p.p12);

        s.store_scaled_mul_ad(94, A::exp_scaled_input(s.ad_value(260), (p.p97 - 2.0)), A::exp_scaled_input(s.ad_value(10), (-p.p119)), p.p85);

        s.store_scaled_exp_scaled_input(95, 260, ((p.p95 + p.p97) - 1.0), p.p86);

        s.store_scaled_exp_scaled_input(96, 260, (p.p98 - 1.0), p.p87);

        s.store_scaled_add(97, 95, 96, (p.p88 * 1.0 / ((p.p86 + p.p87))));

        s.store_scaled_exp_scaled_input(98, 260, (p.p99 - 1.0), p.p89);

        s.store_offset(101, 2, (-300.0));

        s.b[469] = (s.v[2] < 525.0);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if s.b[469] {
            s.store_mul_ad_rhs(99, 1, A::add_scaled_product(A::scale_offset(s.ad_value(101), 0.00072, 1.0), 1.0, s.ad_value(101), s.ad_value(101), (-1.6e-6)));
        }

        if (!s.b[469]) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scaled_exp_scaled_input(100, 260, p.p95, p.p91);

        s.store_scaled_voltage(236, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(237, ctx, nodes, Some(6), Some(8), p.p3);

        s.store_scaled_voltage(238, ctx, nodes, Some(6), Some(4), p.p3);

        s.store_scaled_voltage(239, ctx, nodes, Some(5), Some(4), p.p3);

        s.store_scaled_voltage(240, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(242, ctx, nodes, Some(7), Some(8), p.p3);

        s.store_scaled_voltage(246, ctx, nodes, Some(1), Some(5), p.p3);

        s.store_scaled_voltage(249, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(250, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(244, ctx, nodes, Some(10), Some(7), p.p3);

        s.store_scaled_voltage(243, ctx, nodes, Some(9), Some(10), p.p3);

        s.store_sub_ad_lhs(241, A::add_scaled_inputs3(s.ad_value(240), 1.0, s.ad_value(237), 1.0, s.ad_value(242), -1.0), 244);

        s.store_sub_ad_lhs(248, A::add_scaled_inputs3(s.ad_value(246), 1.0, s.ad_value(250), (-1.0), s.ad_value(241), 1.0), 243);

        s.store_add(247, 250, 248);

        s.b[476] = ((s.v[237] * s.v[8]) < p.p138);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if s.b[476] {
            s.store_exp_mul(251, 237, 8);
        }

        if (!s.b[476]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(251, 281, A::mul(s.ad_value(237), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[477] = (((s.v[238] * s.v[8]) / s.v[48]) < p.p138);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if s.b[477] {
            s.store_exp_ad(252, A::div_scaled_product(s.ad_value(238), s.ad_value(8), 1.0, s.ad_value(48), 1.0));
        }

        if (!s.b[477]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(252, 281, A::div_scaled_product(s.ad_value(238), s.ad_value(8), 1.0, s.ad_value(48), 1.0), (((-p.p138)) + (1.0)));
        }

        s.b[478] = ((s.v[241] * s.v[8]) < p.p138);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if s.b[478] {
            s.store_exp_mul(254, 241, 8);
        }

        if (!s.b[478]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(254, 281, A::mul(s.ad_value(241), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[479] = ((s.v[240] * s.v[8]) < p.p138);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (!s.b[479]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.b[480] = ((s.v[247] * s.v[8]) < p.p138);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if s.b[480] {
            s.store_exp_mul(255, 247, 8);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[480]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(255, 281, A::mul(s.ad_value(247), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[481] = (((s.v[247] - s.v[16]) * s.v[8]) < p.p138);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if s.b[481] {
            s.store_exp_ad(258, A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[481]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(258, 281, A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[482] = (((s.v[241] - s.v[16]) * s.v[8]) < p.p138);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if s.b[482] {
            s.store_exp_ad(256, A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[482]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(256, 281, A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[483] = (((s.v[237] - s.v[16]) * s.v[8]) < p.p138);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if s.b[483] {
            s.store_exp_ad(257, A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[483]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(257, 281, A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[484] = (((s.v[236] - s.v[16]) * s.v[8]) < p.p138);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_exp_ad(259, A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)));
        }

        if (!s.b[484]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(259, 281, A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(107, 257, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(108, 259, 4.0, 1.0);

        s.store_ad_value(109, A::div_scaled_inputs(s.ad_value(259), 2.0, A::offset(s.ad_value(108), 1.0), 1.0));

        s.b[485] = (s.v[109] < p.p140);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.store_scalar(109, p.p140);
        }

        s.store_mul_ad_rhs(110, 6, A::add_scaled_inputs3(s.ad_value(107), 1.0, s.ad_value(108), (-1.0), A::ln(A::div(A::offset(s.ad_value(107), 1.0), A::offset(s.ad_value(108), 1.0))), -1.0));

        s.store_div_ad_lhs(111, A::add(s.ad_value(110), s.ad_value(242)), 31);

        s.b[486] = (s.v[111] > 0.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        s.b[487] = (s.v[236] < 100.0);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[487]) {
            s.copy_ad(283, 236);
        }

        if (s.b[486] && (!s.b[487])) {
            s.store_offset_ln_ad(283, A::offset(s.ad_value(236), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[486] {
            s.store_sub_ad_lhs(112, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(6), A::ln(A::offset(A::mul3_scaled_output(s.ad_value(111), s.ad_value(31), s.ad_value(8), 0.5), 1.0)), 2.0), 283);
            s.store_scale(278, 16, 0.2);
            s.store_square(267, 278);
            s.store_square(268, 112);
        }

        s.b[488] = (s.v[112] < 0.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[488]) {
            s.store_ad_value(113, A::div_scaled_inputs(s.ad_value(267), 0.5, A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(112)), 1.0));
        }

        if (s.b[486] && (!s.b[488])) {
            s.store_scaled_add_ad_lhs(113, A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), 112, 0.5);
        }

        if s.b[486] {
            s.store_ad_value(114, A::div_scaled_product(s.ad_value(113), A::offset(s.ad_value(113), (p.p61 * p.p60)), 1.0, A::add_scaled_inputs(s.ad_value(113), p.p60, s.ad_value(31), (p.p61 * p.p60)), 1.0));
            s.store_div(271, 111, 114);
            s.store_scaled_offset(265, 271, (-1.0), 1.0 / (p.p62));
        }

        s.b[489] = (s.v[271] < 1.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[489]) {
            s.store_offset_scaled_ad(269, A::ln_one_plus_exp(s.ad_value(265)), p.p62, 1.0);
        }

        if (s.b[486] && (!s.b[489])) {
            s.store_ad_value(269, A::add_scaled_inputs(s.ad_value(271), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), p.p62));
        }

        if s.b[486] {
            s.store_scale(115, 269, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
            s.store_scale(116, 113, 1.0 / ((p.p61 * p.p60)));
            s.store_div_ad(117, A::offset(A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(115), s.ad_value(116), A::offset(s.ad_value(116), 1.0), 4.0), 1.0)), 1.0), A::mul_scaled_lhs(s.ad_value(115), 2.0, A::offset(s.ad_value(116), 1.0)));
            s.store_div_ad(118, A::add_scaled_sub_value_product(1.0, s.ad_value(117), 1.0, s.ad_value(109), s.ad_value(117), 1.0), A::offset(A::mul(s.ad_value(109), s.ad_value(117)), 1.0));
            s.store_mul_ad_lhs(120, A::mul3_scaled_output(s.ad_value(111), s.ad_value(31), s.ad_value(118), 0.5), 8);
            s.store_ad_value(272, A::add_scaled_product(s.ad_value(120), 2.0, s.ad_value(109), A::offset(A::add(s.ad_value(109), s.ad_value(120)), 1.0), 1.0));
            s.store_scaled_offset(121, 120, (-1.0), 0.5);
            s.store_add_ad_lhs(266, A::square(s.ad_value(121)), 272);
        }

        s.b[490] = (s.v[120] >= 1.0);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[490]) {
            s.store_add_ad_rhs(122, 121, A::sqrt(s.ad_value(266)));
        }

        if (s.b[486] && (!s.b[490])) {
            s.store_div_ad_rhs(122, 272, A::sub(A::sqrt(s.ad_value(266)), s.ad_value(121)));
        }

        s.b[491] = (s.v[122] < p.p139);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[491]) {
            s.store_scalar(122, p.p139);
        }

        if s.b[486] {
            s.store_mul_ad_product_rhs(124, 122, A::offset(s.ad_value(122), 1.0), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
            s.store_scaled_offset(126, 111, (-p.p61), (0.5 * p.p60));
            s.store_scaled_mul(127, 31, 111, (p.p60 * p.p61));
            s.store_add_ad_rhs(128, 126, A::sqrt(A::add(A::square(s.ad_value(126)), s.ad_value(127))));
        }

        s.b[492] = (p.p72 == 0.0);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if (s.b[486] && s.b[492]) {
            s.store_scale(129, 17, 0.1);
        }

        if (s.b[486] && (!s.b[492])) {
            s.store_mul_offset_ad_rhs(129, 17, A::div_scaled_inputs(s.ad_value(111), 2.0, A::add(s.ad_value(111), s.ad_value(114)), 1.0), 0.1);
        }

        if s.b[486] {
            s.store_ad_value(130, A::div_scaled_inputs(s.ad_value(111), p.p61, A::offset(s.ad_value(111), p.p61), 1.0));
            s.store_div_from_scalar_offset_input(202, p.p61, 111, p.p61);
        }

        if (!s.b[486]) {
            s.store_scalar(114, 0.0);
            s.store_ad_value(122, A::div_scaled_inputs(s.ad_value(257), 2.0, A::offset(s.ad_value(107), 1.0), 1.0));
            s.copy_ad(124, 251);
        }

        s.b[493] = ((((s.v[242]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[110]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[107] + s.v[108]))));
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if ((!s.b[486]) && s.b[493]) {
            s.store_scaled_add(131, 122, 109, 0.5);
            s.store_div_ad_rhs(118, 131, A::offset(s.ad_value(131), 1.0));
        }

        if ((!s.b[486]) && (!s.b[493])) {
            s.store_div_ad_rhs(118, 110, A::add_scaled_inputs3(s.ad_value(110), 1.0, s.ad_value(237), 1.0, s.ad_value(236), -1.0));
        }

        if (!s.b[486]) {
            s.copy_ad(128, 242);
            s.store_scale(129, 17, 0.1);
            s.copy_ad(130, 111);
            s.store_sub_from_scalar_ad(202, 1.0, A::scale(s.ad_value(130), 1.0 / (p.p61)));
        }

        s.store_scale(132, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(279, 14, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(238), s.ad_value(132)), 279);

        s.b[494] = (s.v[238] < s.v[132]);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_ad_value(133, A::add_scaled_product(s.ad_value(238), 1.0, s.ad_value(279), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[494]) {
            s.store_ad_value(133, A::add_scaled_product(s.ad_value(132), 1.0, s.ad_value(279), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (1.0 - p.p66));

        s.store_ad_value(134, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(238), 3.0, s.ad_value(133), (-3.0)));

        s.b[495] = (p.p73 == 1.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if s.b[495] {
            s.copy_ad(135, 236);
        }

        s.b[496] = (p.p73 == 2.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if ((!s.b[495]) && s.b[496]) {
            s.store_add(135, 236, 128);
        }

        if ((!s.b[495]) && (!s.b[496])) {
            s.copy_ad(135, 237);
        }

        s.store_div_ad(136, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(137, 17, 1.0, A::powf(s.ad_value(136), ((-1.0) / p.p71)));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(135), s.ad_value(137)), 129);

        s.b[497] = (s.v[135] < s.v[137]);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if s.b[497] {
            s.store_ad_value(138, A::add_scaled_product(s.ad_value(135), 1.0, s.ad_value(129), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[497]) {
            s.store_ad_value(138, A::add_scaled_product(s.ad_value(137), 1.0, s.ad_value(129), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_powf(139, 202, p.p75);

        s.store_add_ad(140, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(139), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(17))), (1.0 - p.p71))), 1.0 / ((1.0 - p.p71))), A::mul3(s.ad_value(139), s.ad_value(136), A::sub(s.ad_value(135), s.ad_value(138))));

        s.store_ad_value(141, A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(140)), 1.0, s.ad_value(25), s.ad_value(236), 1.0));

        s.store_scaled_div(142, 35, 36, 4.0);

        s.store_mul(143, 142, 252);

        s.store_div_ad_rhs(145, 143, A::offset(A::sqrt(A::offset(s.ad_value(143), 1.0)), 1.0));

        s.store_pow_ad(125, s.ad_value(124), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(144, 142, 125);

        s.store_div_ad_rhs(146, 144, A::offset(A::sqrt(A::offset(s.ad_value(144), 1.0)), 1.0));

        s.b[498] = (p.p91 == 0.0);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_add_ad(147, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));
        }

        if (!s.b[498]) {
            s.store_mul_ad_product_lhs(275, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), s.ad_value(100), 8);
            s.store_mul_ad_product_lhs(276, A::div_scaled_inputs(s.ad_value(141), -1.0, s.ad_value(40), 1.0), s.ad_value(100), 8);
            s.store_div_ad(147, A::sub(A::exp(s.ad_value(275)), A::exp(s.ad_value(276))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 147);

        s.b[499] = (s.v[147] < 0.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if s.b[499] {
            s.store_div_from_scalar_sub_ad(148, (0.5 * s.v[267]), A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(147));
        }

        if (!s.b[499]) {
            s.store_scaled_add_ad_lhs(148, A::sqrt(A::offset(s.ad_value(268), s.v[267])), 147, 0.5);
        }

        s.store_mul_offset_ad_rhs(149, 148, A::add_scaled_inputs(s.ad_value(145), 0.5, s.ad_value(146), 0.5), 1.0);

        s.store_scaled_mul(150, 35, 125, p.p14);

        s.store_mul(151, 35, 252);

        s.store_div_ad_lhs(152, A::sub(s.ad_value(151), s.ad_value(150)), 149);

        s.store_scale(265, 238, 10000.0);

        s.b[500] = (s.v[238] < 0.0);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if s.b[500] {
            s.store_scaled_ln_one_plus_exp(282, 265, 0.0001);
        }

        if (!s.b[500]) {
            s.store_ad_value(282, A::add_scaled_inputs(s.ad_value(238), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), 0.0001));
        }

        s.store_scale(284, 282, 1.0 / (p.p143));

        s.b[501] = (s.v[284] < p.p138);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if (!s.b[501]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.store_scaled_offset(265, 238, (-p.p145), 1000.0);

        s.b[503] = (((s.v[238] * s.v[8]) / p.p16) < p.p138);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if s.b[503] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p16)));
        }

        if (!s.b[503]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(238), s.ad_value(8), 1.0 / (p.p16)), (((-p.p138)) + (1.0)));
        }

        s.b[504] = (p.p23 == 1.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        s.b[505] = (((s.v[238] - s.v[55]) * s.v[8]) < p.p138);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if (s.b[504] && s.b[505]) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[504] && (!s.b[505])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(284, 281, A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[506] = (((s.v[152] / s.v[35]) - 1000.0) < 40.0);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (s.b[504] && (!s.b[506])) {
            s.store_scalar(281, ((40.0) as f64).exp());
        }

        s.b[508] = (((s.v[239] * s.v[8]) / p.p18) < p.p138);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if s.b[508] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p18)));
        }

        if (!s.b[508]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(239), s.ad_value(8), 1.0 / (p.p18)), (((-p.p138)) + (1.0)));
        }

        s.b[509] = (p.p23 == 1.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        s.b[510] = (((s.v[239] - s.v[55]) * s.v[8]) < p.p138);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (s.b[509] && s.b[510]) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)));
        }

        if (s.b[509] && (!s.b[510])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(284, 281, A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        s.b[511] = (((s.v[238] * s.v[8]) / p.p20) < p.p138);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if s.b[511] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p20)));
        }

        if (!s.b[511]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(238), s.ad_value(8), 1.0 / (p.p20)), (((-p.p138)) + (1.0)));
        }

        s.b[512] = (((s.v[239] * s.v[8]) / p.p22) < p.p138);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if s.b[512] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p22)));
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[512]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(239), s.ad_value(8), 1.0 / (p.p22)), (((-p.p138)) + (1.0)));
        }

        s.b[513] = (((s.v[241] * s.v[8]) / p.p31) < p.p138);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(241), s.ad_value(8)), 1.0 / (p.p31)));
        }

        if (!s.b[513]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(241), s.ad_value(8), 1.0 / (p.p31)), (((-p.p138)) + (1.0)));
        }

        s.b[514] = (((s.v[239] * s.v[8]) / p.p137) < p.p138);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if s.b[514] {
            s.store_ad_value(282, A::exp_scaled_input(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p137)));
        }

        if (!s.b[514]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::mul_scaled_output(s.ad_value(239), s.ad_value(8), 1.0 / (p.p137)), (((-p.p138)) + (1.0)));
        }

        s.b[515] = (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[238] < 0.0));
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        s.b[516] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p138);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if (s.b[515] && (!s.b[516])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if s.b[515] {
            s.store_mul(261, 238, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(261)), 1e-30)), ((-2.0) - p.p66)), A::sub_scaled_inputs(A::sub_from_scalar((1.0 - (p.p66 * p.p66)), A::scale(s.ad_value(261), (3.0 * (p.p66 - 1.0)))), p.p66, A::mul3_scaled_output(s.ad_value(261), s.ad_value(261), A::offset(s.ad_value(261), (p.p66 - 1.0)), 6.0), 1.0), 0.16666666666666666);
            s.store_ad_value(261, A::div_scaled_product(s.ad_value(238), s.ad_value(61), s.v[62], A::mul(s.ad_value(70), s.ad_value(60)), 1.0));
        }

        s.b[517] = (s.v[261] < (-0.001));
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        s.b[518] = (s.v[261] < p.p138);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if ((s.b[515] && s.b[517]) && (!s.b[518])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.b[519] = (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[236] < 0.0));
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(236), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[520] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p138);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (s.b[519] && (!s.b[520])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if s.b[519] {
            s.store_mul(263, 236, 67);
        }

        if s.b[519] {
            let assign4360_ad_e4213: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(263)), 1e-30)), ((-2.0) - s.v[76])), A::sub_scaled_inputs(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(263), (3.0 * (s.v[76] - 1.0)))), s.v[76], A::mul3_scaled_output(s.ad_value(263), s.ad_value(263), A::offset(s.ad_value(263), (s.v[76] - 1.0)), 6.0), 1.0), 0.16666666666666666);
            s.store_ad_value(80, assign4360_ad_e4213);
        }

        if s.b[519] {
            s.store_ad_value(263, A::div_scaled_product(s.ad_value(236), s.ad_value(83), s.v[79], A::mul(s.ad_value(85), s.ad_value(80)), 1.0));
        }

        s.b[521] = (s.v[263] < (-0.001));
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        s.b[522] = (s.v[263] < p.p138);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if ((s.b[519] && s.b[521]) && (!s.b[522])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.store_mul(161, 142, 254);

        s.store_scale(162, 256, 4.0);

        s.store_div_ad(164, A::sub(s.ad_value(161), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(161), 1.0)), 1.0));

        s.store_div_ad_rhs(163, 162, A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0));

        s.b[523] = ((p.p5 > 0.0) && (p.p32 > 0.0));
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if s.b[523] {
            s.store_ad_value(167, A::div_scaled_product(s.ad_value(43), A::offset(s.ad_value(255), (-1.0)), (p.p32 * 2.0), A::offset(A::sqrt(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(43), 4.0, s.ad_value(37), 1.0), s.ad_value(255)), 1.0)), 1.0), 1.0));
            s.store_scalar(168, 0.0);
        }

        s.b[524] = (p.p5 == 1.0);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (s.b[523] && s.b[524]) {
            s.store_scaled_mul(277, 43, 32, p.p32);
            s.store_mul_sub_from_scalar_ad_rhs(169, 6, 2.0, A::ln(A::mul(s.ad_value(277), s.ad_value(8))));
            s.store_sub(270, 247, 169);
            s.store_scalar(267, (0.11 * 0.11));
            s.store_square(268, 270);
        }

        s.b[525] = (s.v[270] < 0.0);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((s.b[523] && s.b[524]) && s.b[525]) {
            s.store_ad_value(170, A::div_scaled_inputs(s.ad_value(267), 0.5, A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(270)), 1.0));
        }

        if ((s.b[523] && s.b[524]) && (!s.b[525])) {
            s.store_scaled_add_ad_lhs(170, A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), 270, 0.5);
        }

        if (s.b[523] && s.b[524]) {
            s.store_div_ad_rhs(171, 170, A::add(A::add_scaled_product(s.ad_value(277), 1.0, A::add(s.ad_value(167), s.ad_value(168)), s.ad_value(32), 1.0), s.ad_value(170)));
        }

        if (s.b[523] && (!s.b[524])) {
            s.store_scalar(169, 0.0);
            s.store_scalar(270, 0.0);
            s.store_scalar(170, 0.0);
            s.store_scalar(171, 1.0);
        }

        s.b[526] = (p.p83 == 1.0);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_add(328, 240, 236);
            s.store_scalar(267, (1e-6 * 1e-6));
            s.store_scaled_mul(268, 328, 328, ((-1.0) * (-1.0)));
        }

        s.store_add_ad(175, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 175);

        s.b[529] = (s.v[175] < 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_div_from_scalar_sub_ad(176, (0.5 * s.v[267]), A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(175));
        }

        if (!s.b[529]) {
            s.store_scaled_add_ad_lhs(176, A::sqrt(A::offset(s.ad_value(268), s.v[267])), 175, 0.5);
        }

        s.store_mul_offset_ad_rhs(177, 176, A::add_scaled_inputs(s.ad_value(145), 0.5, s.ad_value(146), 0.5), 1.0);

        s.store_div(179, 29, 177);

        s.b[530] = (s.v[179] < s.v[322]);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if s.b[530] {
            s.copy_ad(179, 322);
        }

        s.store_scale(178, 179, 3.0);

        s.b[531] = (s.v[152] > 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        s.b[532] = (p.p38 == 1.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        s.b[533] = (s.v[236] < p.p43);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        s.b[534] = (((-s.v[152]) / p.p41) < p.p138);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (((s.b[531] && s.b[532]) && s.b[533]) && s.b[534]) {
            s.store_exp_scaled_input(314, 152, (-1.0 / (p.p41)));
        }

        if (((s.b[531] && s.b[532]) && s.b[533]) && (!s.b[534])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_ad_rhs(314, 281, A::scale_offset(s.ad_value(152), (-1.0 / (p.p41)), (((-p.p138)) + (1.0))));
        }

        if ((s.b[531] && s.b[532]) && s.b[533]) {
            s.store_mul_sub_from_scalar_lhs(315, p.p43, 236, 314);
        }

        s.b[535] = (((-s.v[316]) * ((s.v[315]) as f64).powf(p.p40)) < p.p138);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((s.b[531] && s.b[532]) && s.b[533]) && s.b[535]) {
            s.store_exp_ad(319, A::mul_scaled_lhs(s.ad_value(316), -1.0, A::powf(s.ad_value(315), p.p40)));
        }

        if (((s.b[531] && s.b[532]) && s.b[533]) && (!s.b[535])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(319, 281, A::mul_scaled_lhs(s.ad_value(316), -1.0, A::powf(s.ad_value(315), p.p40)), (((-p.p138)) + (1.0)));
        }

        if ((s.b[531] && s.b[532]) && s.b[533]) {
            s.store_mul_ad_product_lhs(199, A::div_from_scalar(p.p39, s.ad_value(316)), s.ad_value(315), 319);
        }

        s.b[536] = (p.p38 == 2.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (s.v[236] < s.v[16]);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) {
            s.store_scalar(188, ((2.0 * p.p45) / (p.p44 * p.p44)));
            s.store_div_ad_lhs(266, A::sub(s.ad_value(16), s.ad_value(236)), 202);
            s.store_sqrt_ad(189, A::div_scaled_inputs(s.ad_value(266), 2.0, s.ad_value(188), 1.0));
        }

        s.b[538] = (p.p7 == 0.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && s.b[538]) {
            s.store_scalar(190, p.p44);
        }

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && (!s.b[538])) {
            s.store_sub_from_scalar_ad(119, 1.0, A::scale(s.ad_value(118), 0.5));
            s.store_scaled_mul(190, 119, 119, p.p44);
        }

        if (((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) {
            s.store_ad_value(191, A::div_scaled_product(s.ad_value(189), s.ad_value(190), 1.0, A::sqrt(A::add(A::square(s.ad_value(189)), A::square(s.ad_value(190)))), 1.0));
            s.store_div_ad_lhs(192, A::sub(s.ad_value(16), s.ad_value(236)), 191);
            s.store_add_ad_rhs(193, 192, A::mul3_scaled_output(s.ad_value(191), s.ad_value(188), s.ad_value(202), 0.5));
        }

        s.b[539] = (p.p7 == 0.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && s.b[539]) {
            s.copy_ad(194, 193);
        }

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && (!s.b[539])) {
            s.store_offset_scaled(195, 118, ((2.0) * ((2.0 * p.p46))), (((2.0 * p.p46)) + (1.0)));
            s.store_scalar(196, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
            s.store_sub_ad_rhs(197, 192, A::mul3_scaled_output(s.ad_value(191), s.ad_value(188), A::sub(s.ad_value(196), A::div_scaled_inputs(s.ad_value(152), 1.0, s.ad_value(195), p.p61)), 0.5));
            s.store_ad_value(266, A::add_scaled_product(A::mul3_scaled_output(s.ad_value(192), s.ad_value(192), s.ad_value(130), (0.1 * 1.0 / (p.p61))), 1.0, A::sub(s.ad_value(197), s.ad_value(193)), A::sub(s.ad_value(197), s.ad_value(193)), 1.0));
            s.store_ad_value(194, A::add_scaled_inputs3(s.ad_value(197), 0.5, s.ad_value(193), 0.5, A::sqrt(s.ad_value(266)), 0.5));
        }

        if (((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) {
            s.store_div_ad_lhs(273, A::sub(s.ad_value(194), s.ad_value(192)), 194);
        }

        s.b[540] = (((s.v[273]) as f64).abs() > 1e-7);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && s.b[540]) {
            s.store_scaled_div(198, 191, 273, 0.5);
            s.store_mul_ad(199, A::mul3(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(194), s.ad_value(198)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(194), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(194), 1.0), A::div(s.ad_value(190), s.ad_value(198)), 1.0))));
        }

        if ((((s.b[531] && (!s.b[532])) && s.b[536]) && s.b[537]) && (!s.b[540])) {
            s.store_mul_ad_product_rhs(199, 0, s.ad_value(190), A::exp(A::div_scaled_inputs(s.ad_value(99), -1.0, s.ad_value(194), 1.0)));
        }

        s.b[541] = (p.p38 == 3.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        s.b[542] = (s.v[236] < p.p43);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if ((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) {
            s.store_mul_ad(203, A::powf(A::sub_from_scalar(p.p43, s.ad_value(236)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(152), A::offset(s.ad_value(152), p.p47))), p.p48));
        }

        s.b[543] = (p.p7 == 0.0);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && s.b[543]) {
            s.copy_ad(204, 203);
        }

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) {
            s.store_scaled_offset(205, 152, (-p.p51), 1.0 / (p.p47));
            s.store_scaled_offset(265, 205, (-1.0), 1.0 / (p.p50));
        }

        s.b[544] = (s.v[205] < 1.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if ((((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) && s.b[544]) {
            s.store_offset_scaled_ad(206, A::ln_one_plus_exp(s.ad_value(265)), p.p50, 1.0);
        }

        if ((((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) && (!s.b[544])) {
            s.store_ad_value(206, A::add_scaled_inputs(s.ad_value(205), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), p.p50));
        }

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[543])) {
            s.store_mul_powf_ad_rhs(204, 203, s.ad_value(206), p.p49);
        }

        s.b[545] = (((-s.v[316]) * s.v[204]) < p.p138);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && s.b[545]) {
            s.store_exp_ad(319, A::mul_scaled_lhs(s.ad_value(316), -1.0, s.ad_value(204)));
        }

        if (((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) && (!s.b[545])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(319, 281, A::mul_scaled_lhs(s.ad_value(316), -1.0, s.ad_value(204)), (((-p.p138)) + (1.0)));
        }

        if ((((s.b[531] && (!s.b[532])) && (!s.b[536])) && s.b[541]) && s.b[542]) {
            s.store_mul_ad_lhs(199, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p39, s.ad_value(316)), p.p43, s.ad_value(236)), 319);
        }

        s.b[546] = (s.v[199] > 0.0);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        s.b[547] = (p.p52 == 1.0);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if ((s.b[531] && s.b[546]) && s.b[547]) {
            s.store_add_ad(200, A::add_scaled_product(A::div(s.ad_value(6), A::mul(s.ad_value(152), A::add(s.ad_value(30), s.ad_value(178)))), 1.0, A::div(s.ad_value(149), s.ad_value(35)), s.ad_value(42), 1.0), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(178))));
        }

        s.b[548] = (p.p38 == 3.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (((s.b[531] && s.b[546]) && s.b[547]) && s.b[548]) {
            s.store_scaled_sub(265, 199, 200, 1000000.0);
        }

        s.b[549] = (s.v[199] < s.v[200]);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((((s.b[531] && s.b[546]) && s.b[547]) && s.b[548]) && s.b[549]) {
            s.store_ad_value(199, A::sub_scaled_inputs(s.ad_value(199), 1.0, A::ln_one_plus_exp(s.ad_value(265)), 1e-6));
        }

        if ((((s.b[531] && s.b[546]) && s.b[547]) && s.b[548]) && (!s.b[549])) {
            s.store_ad_value(199, A::sub_scaled_inputs(s.ad_value(200), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(265))), 1e-6));
        }

        s.store_scaled_mul(210, 23, 134, (1.0 - p.p67));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(239), s.ad_value(132)), 279);

        s.b[552] = (s.v[239] < s.v[132]);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if s.b[552] {
            s.store_ad_value(211, A::add_scaled_product(s.ad_value(239), 1.0, s.ad_value(279), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[552]) {
            s.store_ad_value(211, A::add_scaled_product(s.ad_value(132), 1.0, s.ad_value(279), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_mul_scaled_ad_rhs(212, 23, p.p67, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(211), s.ad_value(65))), (1.0 - p.p66)), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(239), 3.0, s.ad_value(211), (-3.0)));

        s.store_scaled_mul(213, 24, 141, p.p76);

        s.store_mul(214, 95, 36);

        s.store_mul3_affine_lhs(218, 214, 145, 0.5, 0.0, 176);

        s.store_mul3_affine_lhs(219, 214, 146, 0.5, 0.0, 176);

        s.store_scale(280, 17, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(241), s.ad_value(137)), 280);

        s.b[553] = (s.v[241] < s.v[137]);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if s.b[553] {
            s.store_ad_value(220, A::add_scaled_product(s.ad_value(241), 1.0, s.ad_value(280), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[553]) {
            s.store_ad_value(220, A::add_scaled_product(s.ad_value(137), 1.0, s.ad_value(280), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_ad_value(221, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(220), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, s.ad_value(136), A::sub(s.ad_value(241), s.ad_value(220)), 1.0));

        s.store_mul_scaled_ad_rhs(222, 24, ((1.0 - p.p76) * (1.0 - p.p32)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(221)), 1.0, s.ad_value(25), s.ad_value(241), 1.0));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(247), s.ad_value(137)), 280);

        s.b[554] = (s.v[247] < s.v[137]);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if s.b[554] {
            s.store_ad_value(223, A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(280), A::ln_one_plus_exp(s.ad_value(265)), (-1.0)));
        }

        if (!s.b[554]) {
            s.store_ad_value(223, A::add_scaled_product(s.ad_value(137), 1.0, s.ad_value(280), A::ln_one_plus_exp(A::neg(s.ad_value(265))), (-1.0)));
        }

        s.store_ad_value(224, A::add_scaled_product(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(223), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, s.ad_value(136), A::sub(s.ad_value(247), s.ad_value(223)), 1.0));

        s.store_mul_scaled_ad_rhs(225, 24, ((1.0 - p.p76) * p.p32), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(224)), 1.0, s.ad_value(25), s.ad_value(247), 1.0));

    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        s.store_mul_ad_product_rhs(226, 94, s.ad_value(36), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p84)));

        s.b[555] = ((s.v[238] / (p.p84 * s.v[6])) < p.p138);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_exp_ad(282, A::div_scaled_inputs(s.ad_value(238), 1.0, s.ad_value(6), p.p84));
        }

        if (!s.b[555]) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(282, 281, A::div_scaled_inputs(s.ad_value(238), 1.0, s.ad_value(6), p.p84), (((-p.p138)) + (1.0)));
        }

        s.store_mul(228, 226, 282);

        s.store_ad_value(229, A::div_scaled_product(s.ad_value(96), s.ad_value(6), 4.0, s.ad_value(31), 1.0));

        s.store_mul_ad_affine_product_rhs(230, 229, s.ad_value(118), A::offset(A::add(s.ad_value(122), s.ad_value(109)), 2.0), 0.5, 0.0);

        s.b[556] = (p.p78 == 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_ad_value(235, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(214), s.ad_value(164), 1.0, s.ad_value(229), s.ad_value(163), 1.0), 0.5, A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[557] = ((((s.v[241] - s.v[22]) / p.p90) * s.v[8]) < p.p138);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if ((!s.b[556]) && s.b[557]) {
            s.store_exp_ad(173, A::mul_scaled_lhs(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90), s.ad_value(8)));
        }

        if ((!s.b[556]) && (!s.b[557])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(173, 281, A::mul_scaled_lhs(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        if (!s.b[556]) {
            s.store_ad_value(235, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(254), 2.0, A::offset(A::sqrt(A::scale_offset(s.ad_value(173), 4.0, 1.0)), 1.0), 1.0));
        }

        s.b[558] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0));
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scale(235, 235, s.v[153]);
        }

        s.b[559] = (p.p78 == 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if (s.b[558] && s.b[559]) {
            s.store_mul(165, 142, 255);
            s.store_div_ad(166, A::sub(s.ad_value(165), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0));
            s.store_scale(231, 258, 4.0);
            s.store_div_ad_rhs(232, 231, A::offset(A::sqrt(A::offset(s.ad_value(231), 1.0)), 1.0));
            s.store_ad_value(233, A::div_scaled_product(s.ad_value(97), A::add_scaled_products(s.ad_value(214), s.ad_value(166), 1.0, s.ad_value(229), s.ad_value(232), 1.0), (0.5 * p.p32), A::add(s.ad_value(95), s.ad_value(96)), 1.0));
        }

        s.b[560] = (((s.v[247] - s.v[22]) * s.v[8]) < p.p138);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if ((s.b[558] && (!s.b[559])) && s.b[560]) {
            s.store_exp_ad(174, A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)));
        }

        if ((s.b[558] && (!s.b[559])) && (!s.b[560])) {
            s.store_scalar(281, ((p.p138) as f64).exp());
            s.store_mul_offset_ad_rhs(174, 281, A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)), (((-p.p138)) + (1.0)));
        }

        if (s.b[558] && (!s.b[559])) {
            s.store_ad_value(233, A::div_scaled_product3(s.ad_value(43), s.ad_value(98), s.ad_value(255), (2.0 * p.p32), A::offset(A::sqrt(A::scale_offset(s.ad_value(174), 4.0, 1.0)), 1.0), 1.0));
        }

        if s.b[558] {
            s.store_mul(234, 171, 233);
        }

        s.b[561] = (p.p6 == 1.0);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_offset_powf_ad(182, A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (-p.p66), (-3.0));
            s.store_div_ad_lhs(274, A::sub(s.ad_value(238), s.ad_value(132)), 279);
        }

        s.b[562] = (s.v[274] < 0.0);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if (s.b[561] && s.b[562]) {
            s.store_div_from_scalar_offset_ad(183, 1.0, A::exp(s.ad_value(274)), 1.0);
        }

        if (s.b[561] && (!s.b[562])) {
            s.store_div_ad(183, A::exp_scaled_input(s.ad_value(274), -1.0), A::offset(A::exp_scaled_input(s.ad_value(274), -1.0), 1.0));
        }

        if s.b[561] {
            s.store_offset_mul(181, 182, 183, 3.0);
            s.store_scaled_mul(184, 23, 181, (1.0 - p.p67));
            s.store_mul_ad(187, A::div_scaled_product3(s.ad_value(142), s.ad_value(252), s.ad_value(8), 1.0, s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(143), 1.0))));
            s.store_mul3_affine_lhs(185, 214, 176, 0.5, 0.0, 187);
            s.store_scaled_div(186, 228, 6, (1.0 / (p.p84)));
            s.store_mul_scaled_ad_rhs(217, 240, 0.2, A::add_scaled_inputs3(s.ad_value(184), 1.0, s.ad_value(185), 1.0, s.ad_value(186), 1.0));
            s.store_scale(227, 228, (1.0 - p.p94));
            s.store_add_scaled_inputs(313, 218, 1.0, 228, p.p94);
            s.store_add_scaled_inputs(216, 313, p.p93, 219, 1.0);
            s.store_scale(215, 313, (1.0 - p.p93));
        }

        if (!s.b[561]) {
            s.copy_ad(215, 218);
            s.copy_ad(216, 219);
            s.copy_ad(227, 228);
        }

        let assign6450_e6585: f64 = (p.p134 * (nv3 - 0.0));
        let assign6450_e6586_q: f64 = assign6450_e6585;
        let assign6450_e6588: f64 = (assign6450_e6585 * p.p1);
        let assign6450_e6588_q: f64 = (assign6450_e6586_q * p.p1);
        s.v[209] = assign6450_e6588;
        s.dn[209][3] = (p.p134 * p.p1);
        s.rv[209] = assign6450_e6588_q;
        s.rdn[209][3] = (p.p134 * p.p1);

        s.store_div_ad_lhs(309, A::add(s.ad_value(151), s.ad_value(150)), 149);

        s.b[570] = (s.v[309] > 0.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_div_ad_lhs(311, A::add(s.ad_value(215), s.ad_value(216)), 309);
        }

        if (!s.b[570]) {
            s.store_mul3_lhs(311, 95, 176, 149);
        }

        s.b[571] = (p.p130 == 1.0);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if s.b[571] {
            s.store_scale(312, 311, p.p93);
        }

        s.b[572] = (p.p130 == 2.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if ((!s.b[571]) && s.b[572]) {
            s.store_scale(312, 311, p.p131);
        }

        if ((!s.b[571]) && (!s.b[572])) {
            s.store_scalar(312, 0.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq0_e154: f64 = (p.p3 * s.v[111]);
        let eq0_e154_d_n0: f64 = (p.p3 * s.dn[111][0]);
        let eq0_e154_d_n1: f64 = (p.p3 * s.dn[111][1]);
        let eq0_e154_d_n2: f64 = (p.p3 * s.dn[111][2]);
        let eq0_e154_d_n3: f64 = (p.p3 * s.dn[111][3]);
        let eq0_e154_d_n4: f64 = (p.p3 * s.dn[111][4]);
        let eq0_e154_d_n5: f64 = (p.p3 * s.dn[111][5]);
        let eq0_e154_d_n6: f64 = (p.p3 * s.dn[111][6]);
        let eq0_e154_d_n7: f64 = (p.p3 * s.dn[111][7]);
        let eq0_e154_d_n8: f64 = (p.p3 * s.dn[111][8]);
        let eq0_e154_d_n9: f64 = (p.p3 * s.dn[111][9]);
        let eq0_e154_d_n10: f64 = (p.p3 * s.dn[111][10]);
        let eq0_e154_d_n11: f64 = (p.p3 * s.dn[111][11]);
        let eq0_e154_d_b0: f64 = (p.p3 * s.db[111][0]);
        let eq0_e154_d_b1: f64 = (p.p3 * s.db[111][1]);
        let eq0_e156: f64 = (eq0_e154 * p.p1);
        let eq0_e156_d_n0: f64 = (eq0_e154_d_n0 * p.p1);
        let eq0_e156_d_n1: f64 = (eq0_e154_d_n1 * p.p1);
        let eq0_e156_d_n2: f64 = (eq0_e154_d_n2 * p.p1);
        let eq0_e156_d_n3: f64 = (eq0_e154_d_n3 * p.p1);
        let eq0_e156_d_n4: f64 = (eq0_e154_d_n4 * p.p1);
        let eq0_e156_d_n5: f64 = (eq0_e154_d_n5 * p.p1);
        let eq0_e156_d_n6: f64 = (eq0_e154_d_n6 * p.p1);
        let eq0_e156_d_n7: f64 = (eq0_e154_d_n7 * p.p1);
        let eq0_e156_d_n8: f64 = (eq0_e154_d_n8 * p.p1);
        let eq0_e156_d_n9: f64 = (eq0_e154_d_n9 * p.p1);
        let eq0_e156_d_n10: f64 = (eq0_e154_d_n10 * p.p1);
        let eq0_e156_d_n11: f64 = (eq0_e154_d_n11 * p.p1);
        let eq0_e156_d_b0: f64 = (eq0_e154_d_b0 * p.p1);
        let eq0_e156_d_b1: f64 = (eq0_e154_d_b1 * p.p1);
        let eq0_value: f64 = eq0_e156;
        let eq0_node_derivatives: [f64; 12] = [eq0_e156_d_n0, eq0_e156_d_n1, eq0_e156_d_n2, eq0_e156_d_n3, eq0_e156_d_n4, eq0_e156_d_n5, eq0_e156_d_n6, eq0_e156_d_n7, eq0_e156_d_n8, eq0_e156_d_n9, eq0_e156_d_n10, eq0_e156_d_n11];
        let eq0_branch_derivatives: [f64; 2] = [eq0_e156_d_b0, eq0_e156_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e159: f64 = (p.p3 * s.v[152]);
        let eq1_e159_d_n0: f64 = (p.p3 * s.dn[152][0]);
        let eq1_e159_d_n1: f64 = (p.p3 * s.dn[152][1]);
        let eq1_e159_d_n2: f64 = (p.p3 * s.dn[152][2]);
        let eq1_e159_d_n3: f64 = (p.p3 * s.dn[152][3]);
        let eq1_e159_d_n4: f64 = (p.p3 * s.dn[152][4]);
        let eq1_e159_d_n5: f64 = (p.p3 * s.dn[152][5]);
        let eq1_e159_d_n6: f64 = (p.p3 * s.dn[152][6]);
        let eq1_e159_d_n7: f64 = (p.p3 * s.dn[152][7]);
        let eq1_e159_d_n8: f64 = (p.p3 * s.dn[152][8]);
        let eq1_e159_d_n9: f64 = (p.p3 * s.dn[152][9]);
        let eq1_e159_d_n10: f64 = (p.p3 * s.dn[152][10]);
        let eq1_e159_d_n11: f64 = (p.p3 * s.dn[152][11]);
        let eq1_e159_d_b0: f64 = (p.p3 * s.db[152][0]);
        let eq1_e159_d_b1: f64 = (p.p3 * s.db[152][1]);
        let eq1_e161: f64 = (eq1_e159 * p.p1);
        let eq1_e161_d_n0: f64 = (eq1_e159_d_n0 * p.p1);
        let eq1_e161_d_n1: f64 = (eq1_e159_d_n1 * p.p1);
        let eq1_e161_d_n2: f64 = (eq1_e159_d_n2 * p.p1);
        let eq1_e161_d_n3: f64 = (eq1_e159_d_n3 * p.p1);
        let eq1_e161_d_n4: f64 = (eq1_e159_d_n4 * p.p1);
        let eq1_e161_d_n5: f64 = (eq1_e159_d_n5 * p.p1);
        let eq1_e161_d_n6: f64 = (eq1_e159_d_n6 * p.p1);
        let eq1_e161_d_n7: f64 = (eq1_e159_d_n7 * p.p1);
        let eq1_e161_d_n8: f64 = (eq1_e159_d_n8 * p.p1);
        let eq1_e161_d_n9: f64 = (eq1_e159_d_n9 * p.p1);
        let eq1_e161_d_n10: f64 = (eq1_e159_d_n10 * p.p1);
        let eq1_e161_d_n11: f64 = (eq1_e159_d_n11 * p.p1);
        let eq1_e161_d_b0: f64 = (eq1_e159_d_b0 * p.p1);
        let eq1_e161_d_b1: f64 = (eq1_e159_d_b1 * p.p1);
        let eq1_value: f64 = eq1_e161;
        let eq1_node_derivatives: [f64; 12] = [eq1_e161_d_n0, eq1_e161_d_n1, eq1_e161_d_n2, eq1_e161_d_n3, eq1_e161_d_n4, eq1_e161_d_n5, eq1_e161_d_n6, eq1_e161_d_n7, eq1_e161_d_n8, eq1_e161_d_n9, eq1_e161_d_n10, eq1_e161_d_n11];
        let eq1_branch_derivatives: [f64; 2] = [eq1_e161_d_b0, eq1_e161_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(4),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e165: f64 = (s.v[155] + s.v[158]);
        let eq2_e165_d_n0: f64 = (s.dn[155][0] + s.dn[158][0]);
        let eq2_e165_d_n1: f64 = (s.dn[155][1] + s.dn[158][1]);
        let eq2_e165_d_n2: f64 = (s.dn[155][2] + s.dn[158][2]);
        let eq2_e165_d_n3: f64 = (s.dn[155][3] + s.dn[158][3]);
        let eq2_e165_d_n4: f64 = (s.dn[155][4] + s.dn[158][4]);
        let eq2_e165_d_n5: f64 = (s.dn[155][5] + s.dn[158][5]);
        let eq2_e165_d_n6: f64 = (s.dn[155][6] + s.dn[158][6]);
        let eq2_e165_d_n7: f64 = (s.dn[155][7] + s.dn[158][7]);
        let eq2_e165_d_n8: f64 = (s.dn[155][8] + s.dn[158][8]);
        let eq2_e165_d_n9: f64 = (s.dn[155][9] + s.dn[158][9]);
        let eq2_e165_d_n10: f64 = (s.dn[155][10] + s.dn[158][10]);
        let eq2_e165_d_n11: f64 = (s.dn[155][11] + s.dn[158][11]);
        let eq2_e165_d_b0: f64 = (s.db[155][0] + s.db[158][0]);
        let eq2_e165_d_b1: f64 = (s.db[155][1] + s.db[158][1]);
        let eq2_e167: f64 = (eq2_e165 + s.v[159]);
        let eq2_e167_d_n0: f64 = (eq2_e165_d_n0 + s.dn[159][0]);
        let eq2_e167_d_n1: f64 = (eq2_e165_d_n1 + s.dn[159][1]);
        let eq2_e167_d_n2: f64 = (eq2_e165_d_n2 + s.dn[159][2]);
        let eq2_e167_d_n3: f64 = (eq2_e165_d_n3 + s.dn[159][3]);
        let eq2_e167_d_n4: f64 = (eq2_e165_d_n4 + s.dn[159][4]);
        let eq2_e167_d_n5: f64 = (eq2_e165_d_n5 + s.dn[159][5]);
        let eq2_e167_d_n6: f64 = (eq2_e165_d_n6 + s.dn[159][6]);
        let eq2_e167_d_n7: f64 = (eq2_e165_d_n7 + s.dn[159][7]);
        let eq2_e167_d_n8: f64 = (eq2_e165_d_n8 + s.dn[159][8]);
        let eq2_e167_d_n9: f64 = (eq2_e165_d_n9 + s.dn[159][9]);
        let eq2_e167_d_n10: f64 = (eq2_e165_d_n10 + s.dn[159][10]);
        let eq2_e167_d_n11: f64 = (eq2_e165_d_n11 + s.dn[159][11]);
        let eq2_e167_d_b0: f64 = (eq2_e165_d_b0 + s.db[159][0]);
        let eq2_e167_d_b1: f64 = (eq2_e165_d_b1 + s.db[159][1]);
        let eq2_e168: f64 = (p.p3 * eq2_e167);
        let eq2_e168_d_n0: f64 = (p.p3 * eq2_e167_d_n0);
        let eq2_e168_d_n1: f64 = (p.p3 * eq2_e167_d_n1);
        let eq2_e168_d_n2: f64 = (p.p3 * eq2_e167_d_n2);
        let eq2_e168_d_n3: f64 = (p.p3 * eq2_e167_d_n3);
        let eq2_e168_d_n4: f64 = (p.p3 * eq2_e167_d_n4);
        let eq2_e168_d_n5: f64 = (p.p3 * eq2_e167_d_n5);
        let eq2_e168_d_n6: f64 = (p.p3 * eq2_e167_d_n6);
        let eq2_e168_d_n7: f64 = (p.p3 * eq2_e167_d_n7);
        let eq2_e168_d_n8: f64 = (p.p3 * eq2_e167_d_n8);
        let eq2_e168_d_n9: f64 = (p.p3 * eq2_e167_d_n9);
        let eq2_e168_d_n10: f64 = (p.p3 * eq2_e167_d_n10);
        let eq2_e168_d_n11: f64 = (p.p3 * eq2_e167_d_n11);
        let eq2_e168_d_b0: f64 = (p.p3 * eq2_e167_d_b0);
        let eq2_e168_d_b1: f64 = (p.p3 * eq2_e167_d_b1);
        let eq2_e170: f64 = (eq2_e168 * p.p1);
        let eq2_e170_d_n0: f64 = (eq2_e168_d_n0 * p.p1);
        let eq2_e170_d_n1: f64 = (eq2_e168_d_n1 * p.p1);
        let eq2_e170_d_n2: f64 = (eq2_e168_d_n2 * p.p1);
        let eq2_e170_d_n3: f64 = (eq2_e168_d_n3 * p.p1);
        let eq2_e170_d_n4: f64 = (eq2_e168_d_n4 * p.p1);
        let eq2_e170_d_n5: f64 = (eq2_e168_d_n5 * p.p1);
        let eq2_e170_d_n6: f64 = (eq2_e168_d_n6 * p.p1);
        let eq2_e170_d_n7: f64 = (eq2_e168_d_n7 * p.p1);
        let eq2_e170_d_n8: f64 = (eq2_e168_d_n8 * p.p1);
        let eq2_e170_d_n9: f64 = (eq2_e168_d_n9 * p.p1);
        let eq2_e170_d_n10: f64 = (eq2_e168_d_n10 * p.p1);
        let eq2_e170_d_n11: f64 = (eq2_e168_d_n11 * p.p1);
        let eq2_e170_d_b0: f64 = (eq2_e168_d_b0 * p.p1);
        let eq2_e170_d_b1: f64 = (eq2_e168_d_b1 * p.p1);
        let eq2_value: f64 = eq2_e170;
        let eq2_node_derivatives: [f64; 12] = [eq2_e170_d_n0, eq2_e170_d_n1, eq2_e170_d_n2, eq2_e170_d_n3, eq2_e170_d_n4, eq2_e170_d_n5, eq2_e170_d_n6, eq2_e170_d_n7, eq2_e170_d_n8, eq2_e170_d_n9, eq2_e170_d_n10, eq2_e170_d_n11];
        let eq2_branch_derivatives: [f64; 2] = [eq2_e170_d_b0, eq2_e170_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e174: f64 = (s.v[154] + s.v[156]);
        let eq3_e174_d_n0: f64 = (s.dn[154][0] + s.dn[156][0]);
        let eq3_e174_d_n1: f64 = (s.dn[154][1] + s.dn[156][1]);
        let eq3_e174_d_n2: f64 = (s.dn[154][2] + s.dn[156][2]);
        let eq3_e174_d_n3: f64 = (s.dn[154][3] + s.dn[156][3]);
        let eq3_e174_d_n4: f64 = (s.dn[154][4] + s.dn[156][4]);
        let eq3_e174_d_n5: f64 = (s.dn[154][5] + s.dn[156][5]);
        let eq3_e174_d_n6: f64 = (s.dn[154][6] + s.dn[156][6]);
        let eq3_e174_d_n7: f64 = (s.dn[154][7] + s.dn[156][7]);
        let eq3_e174_d_n8: f64 = (s.dn[154][8] + s.dn[156][8]);
        let eq3_e174_d_n9: f64 = (s.dn[154][9] + s.dn[156][9]);
        let eq3_e174_d_n10: f64 = (s.dn[154][10] + s.dn[156][10]);
        let eq3_e174_d_n11: f64 = (s.dn[154][11] + s.dn[156][11]);
        let eq3_e174_d_b0: f64 = (s.db[154][0] + s.db[156][0]);
        let eq3_e174_d_b1: f64 = (s.db[154][1] + s.db[156][1]);
        let eq3_e177: f64 = (s.v[320] * s.v[238]);
        let eq3_e177_d_n0: f64 = (s.v[320] * s.dn[238][0]);
        let eq3_e177_d_n1: f64 = (s.v[320] * s.dn[238][1]);
        let eq3_e177_d_n2: f64 = (s.v[320] * s.dn[238][2]);
        let eq3_e177_d_n3: f64 = (s.v[320] * s.dn[238][3]);
        let eq3_e177_d_n4: f64 = (s.v[320] * s.dn[238][4]);
        let eq3_e177_d_n5: f64 = (s.v[320] * s.dn[238][5]);
        let eq3_e177_d_n6: f64 = (s.v[320] * s.dn[238][6]);
        let eq3_e177_d_n7: f64 = (s.v[320] * s.dn[238][7]);
        let eq3_e177_d_n8: f64 = (s.v[320] * s.dn[238][8]);
        let eq3_e177_d_n9: f64 = (s.v[320] * s.dn[238][9]);
        let eq3_e177_d_n10: f64 = (s.v[320] * s.dn[238][10]);
        let eq3_e177_d_n11: f64 = (s.v[320] * s.dn[238][11]);
        let eq3_e177_d_b0: f64 = (s.v[320] * s.db[238][0]);
        let eq3_e177_d_b1: f64 = (s.v[320] * s.db[238][1]);
        let eq3_e178: f64 = (eq3_e174 + eq3_e177);
        let eq3_e178_d_n0: f64 = (eq3_e174_d_n0 + eq3_e177_d_n0);
        let eq3_e178_d_n1: f64 = (eq3_e174_d_n1 + eq3_e177_d_n1);
        let eq3_e178_d_n2: f64 = (eq3_e174_d_n2 + eq3_e177_d_n2);
        let eq3_e178_d_n3: f64 = (eq3_e174_d_n3 + eq3_e177_d_n3);
        let eq3_e178_d_n4: f64 = (eq3_e174_d_n4 + eq3_e177_d_n4);
        let eq3_e178_d_n5: f64 = (eq3_e174_d_n5 + eq3_e177_d_n5);
        let eq3_e178_d_n6: f64 = (eq3_e174_d_n6 + eq3_e177_d_n6);
        let eq3_e178_d_n7: f64 = (eq3_e174_d_n7 + eq3_e177_d_n7);
        let eq3_e178_d_n8: f64 = (eq3_e174_d_n8 + eq3_e177_d_n8);
        let eq3_e178_d_n9: f64 = (eq3_e174_d_n9 + eq3_e177_d_n9);
        let eq3_e178_d_n10: f64 = (eq3_e174_d_n10 + eq3_e177_d_n10);
        let eq3_e178_d_n11: f64 = (eq3_e174_d_n11 + eq3_e177_d_n11);
        let eq3_e178_d_b0: f64 = (eq3_e174_d_b0 + eq3_e177_d_b0);
        let eq3_e178_d_b1: f64 = (eq3_e174_d_b1 + eq3_e177_d_b1);
        let eq3_e180: f64 = (eq3_e178 - s.v[57]);
        let eq3_e180_d_n0: f64 = (eq3_e178_d_n0 - s.dn[57][0]);
        let eq3_e180_d_n1: f64 = (eq3_e178_d_n1 - s.dn[57][1]);
        let eq3_e180_d_n2: f64 = (eq3_e178_d_n2 - s.dn[57][2]);
        let eq3_e180_d_n3: f64 = (eq3_e178_d_n3 - s.dn[57][3]);
        let eq3_e180_d_n4: f64 = (eq3_e178_d_n4 - s.dn[57][4]);
        let eq3_e180_d_n5: f64 = (eq3_e178_d_n5 - s.dn[57][5]);
        let eq3_e180_d_n6: f64 = (eq3_e178_d_n6 - s.dn[57][6]);
        let eq3_e180_d_n7: f64 = (eq3_e178_d_n7 - s.dn[57][7]);
        let eq3_e180_d_n8: f64 = (eq3_e178_d_n8 - s.dn[57][8]);
        let eq3_e180_d_n9: f64 = (eq3_e178_d_n9 - s.dn[57][9]);
        let eq3_e180_d_n10: f64 = (eq3_e178_d_n10 - s.dn[57][10]);
        let eq3_e180_d_n11: f64 = (eq3_e178_d_n11 - s.dn[57][11]);
        let eq3_e180_d_b0: f64 = (eq3_e178_d_b0 - s.db[57][0]);
        let eq3_e180_d_b1: f64 = (eq3_e178_d_b1 - s.db[57][1]);
        let eq3_e182: f64 = (eq3_e180 + s.v[334]);
        let eq3_e182_d_n0: f64 = (eq3_e180_d_n0 + s.dn[334][0]);
        let eq3_e182_d_n1: f64 = (eq3_e180_d_n1 + s.dn[334][1]);
        let eq3_e182_d_n2: f64 = (eq3_e180_d_n2 + s.dn[334][2]);
        let eq3_e182_d_n3: f64 = (eq3_e180_d_n3 + s.dn[334][3]);
        let eq3_e182_d_n4: f64 = (eq3_e180_d_n4 + s.dn[334][4]);
        let eq3_e182_d_n5: f64 = (eq3_e180_d_n5 + s.dn[334][5]);
        let eq3_e182_d_n6: f64 = (eq3_e180_d_n6 + s.dn[334][6]);
        let eq3_e182_d_n7: f64 = (eq3_e180_d_n7 + s.dn[334][7]);
        let eq3_e182_d_n8: f64 = (eq3_e180_d_n8 + s.dn[334][8]);
        let eq3_e182_d_n9: f64 = (eq3_e180_d_n9 + s.dn[334][9]);
        let eq3_e182_d_n10: f64 = (eq3_e180_d_n10 + s.dn[334][10]);
        let eq3_e182_d_n11: f64 = (eq3_e180_d_n11 + s.dn[334][11]);
        let eq3_e182_d_b0: f64 = (eq3_e180_d_b0 + s.db[334][0]);
        let eq3_e182_d_b1: f64 = (eq3_e180_d_b1 + s.db[334][1]);
        let eq3_e184: f64 = (eq3_e182 + s.v[333]);
        let eq3_e184_d_n0: f64 = (eq3_e182_d_n0 + s.dn[333][0]);
        let eq3_e184_d_n1: f64 = (eq3_e182_d_n1 + s.dn[333][1]);
        let eq3_e184_d_n2: f64 = (eq3_e182_d_n2 + s.dn[333][2]);
        let eq3_e184_d_n3: f64 = (eq3_e182_d_n3 + s.dn[333][3]);
        let eq3_e184_d_n4: f64 = (eq3_e182_d_n4 + s.dn[333][4]);
        let eq3_e184_d_n5: f64 = (eq3_e182_d_n5 + s.dn[333][5]);
        let eq3_e184_d_n6: f64 = (eq3_e182_d_n6 + s.dn[333][6]);
        let eq3_e184_d_n7: f64 = (eq3_e182_d_n7 + s.dn[333][7]);
        let eq3_e184_d_n8: f64 = (eq3_e182_d_n8 + s.dn[333][8]);
        let eq3_e184_d_n9: f64 = (eq3_e182_d_n9 + s.dn[333][9]);
        let eq3_e184_d_n10: f64 = (eq3_e182_d_n10 + s.dn[333][10]);
        let eq3_e184_d_n11: f64 = (eq3_e182_d_n11 + s.dn[333][11]);
        let eq3_e184_d_b0: f64 = (eq3_e182_d_b0 + s.db[333][0]);
        let eq3_e184_d_b1: f64 = (eq3_e182_d_b1 + s.db[333][1]);
        let eq3_e185: f64 = (p.p3 * eq3_e184);
        let eq3_e185_d_n0: f64 = (p.p3 * eq3_e184_d_n0);
        let eq3_e185_d_n1: f64 = (p.p3 * eq3_e184_d_n1);
        let eq3_e185_d_n2: f64 = (p.p3 * eq3_e184_d_n2);
        let eq3_e185_d_n3: f64 = (p.p3 * eq3_e184_d_n3);
        let eq3_e185_d_n4: f64 = (p.p3 * eq3_e184_d_n4);
        let eq3_e185_d_n5: f64 = (p.p3 * eq3_e184_d_n5);
        let eq3_e185_d_n6: f64 = (p.p3 * eq3_e184_d_n6);
        let eq3_e185_d_n7: f64 = (p.p3 * eq3_e184_d_n7);
        let eq3_e185_d_n8: f64 = (p.p3 * eq3_e184_d_n8);
        let eq3_e185_d_n9: f64 = (p.p3 * eq3_e184_d_n9);
        let eq3_e185_d_n10: f64 = (p.p3 * eq3_e184_d_n10);
        let eq3_e185_d_n11: f64 = (p.p3 * eq3_e184_d_n11);
        let eq3_e185_d_b0: f64 = (p.p3 * eq3_e184_d_b0);
        let eq3_e185_d_b1: f64 = (p.p3 * eq3_e184_d_b1);
        let eq3_e187: f64 = (eq3_e185 * p.p1);
        let eq3_e187_d_n0: f64 = (eq3_e185_d_n0 * p.p1);
        let eq3_e187_d_n1: f64 = (eq3_e185_d_n1 * p.p1);
        let eq3_e187_d_n2: f64 = (eq3_e185_d_n2 * p.p1);
        let eq3_e187_d_n3: f64 = (eq3_e185_d_n3 * p.p1);
        let eq3_e187_d_n4: f64 = (eq3_e185_d_n4 * p.p1);
        let eq3_e187_d_n5: f64 = (eq3_e185_d_n5 * p.p1);
        let eq3_e187_d_n6: f64 = (eq3_e185_d_n6 * p.p1);
        let eq3_e187_d_n7: f64 = (eq3_e185_d_n7 * p.p1);
        let eq3_e187_d_n8: f64 = (eq3_e185_d_n8 * p.p1);
        let eq3_e187_d_n9: f64 = (eq3_e185_d_n9 * p.p1);
        let eq3_e187_d_n10: f64 = (eq3_e185_d_n10 * p.p1);
        let eq3_e187_d_n11: f64 = (eq3_e185_d_n11 * p.p1);
        let eq3_e187_d_b0: f64 = (eq3_e185_d_b0 * p.p1);
        let eq3_e187_d_b1: f64 = (eq3_e185_d_b1 * p.p1);
        let eq3_value: f64 = eq3_e187;
        let eq3_node_derivatives: [f64; 12] = [eq3_e187_d_n0, eq3_e187_d_n1, eq3_e187_d_n2, eq3_e187_d_n3, eq3_e187_d_n4, eq3_e187_d_n5, eq3_e187_d_n6, eq3_e187_d_n7, eq3_e187_d_n8, eq3_e187_d_n9, eq3_e187_d_n10, eq3_e187_d_n11];
        let eq3_branch_derivatives: [f64; 2] = [eq3_e187_d_b0, eq3_e187_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e196, eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n2, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10, eq4_e196_d_n11, eq4_e196_d_b0, eq4_e196_d_b1,) = {
    if s.b[563] {
        let eq4_e191: f64 = (-s.v[82]);
        let eq4_e191_d_n0: f64 = (-s.dn[82][0]);
        let eq4_e191_d_n1: f64 = (-s.dn[82][1]);
        let eq4_e191_d_n2: f64 = (-s.dn[82][2]);
        let eq4_e191_d_n3: f64 = (-s.dn[82][3]);
        let eq4_e191_d_n4: f64 = (-s.dn[82][4]);
        let eq4_e191_d_n5: f64 = (-s.dn[82][5]);
        let eq4_e191_d_n6: f64 = (-s.dn[82][6]);
        let eq4_e191_d_n7: f64 = (-s.dn[82][7]);
        let eq4_e191_d_n8: f64 = (-s.dn[82][8]);
        let eq4_e191_d_n9: f64 = (-s.dn[82][9]);
        let eq4_e191_d_n10: f64 = (-s.dn[82][10]);
        let eq4_e191_d_n11: f64 = (-s.dn[82][11]);
        let eq4_e191_d_b0: f64 = (-s.db[82][0]);
        let eq4_e191_d_b1: f64 = (-s.db[82][1]);
        let eq4_e192: f64 = (p.p3 * eq4_e191);
        let eq4_e192_d_n0: f64 = (p.p3 * eq4_e191_d_n0);
        let eq4_e192_d_n1: f64 = (p.p3 * eq4_e191_d_n1);
        let eq4_e192_d_n2: f64 = (p.p3 * eq4_e191_d_n2);
        let eq4_e192_d_n3: f64 = (p.p3 * eq4_e191_d_n3);
        let eq4_e192_d_n4: f64 = (p.p3 * eq4_e191_d_n4);
        let eq4_e192_d_n5: f64 = (p.p3 * eq4_e191_d_n5);
        let eq4_e192_d_n6: f64 = (p.p3 * eq4_e191_d_n6);
        let eq4_e192_d_n7: f64 = (p.p3 * eq4_e191_d_n7);
        let eq4_e192_d_n8: f64 = (p.p3 * eq4_e191_d_n8);
        let eq4_e192_d_n9: f64 = (p.p3 * eq4_e191_d_n9);
        let eq4_e192_d_n10: f64 = (p.p3 * eq4_e191_d_n10);
        let eq4_e192_d_n11: f64 = (p.p3 * eq4_e191_d_n11);
        let eq4_e192_d_b0: f64 = (p.p3 * eq4_e191_d_b0);
        let eq4_e192_d_b1: f64 = (p.p3 * eq4_e191_d_b1);
        let eq4_e194: f64 = (eq4_e192 * p.p1);
        let eq4_e194_d_n0: f64 = (eq4_e192_d_n0 * p.p1);
        let eq4_e194_d_n1: f64 = (eq4_e192_d_n1 * p.p1);
        let eq4_e194_d_n2: f64 = (eq4_e192_d_n2 * p.p1);
        let eq4_e194_d_n3: f64 = (eq4_e192_d_n3 * p.p1);
        let eq4_e194_d_n4: f64 = (eq4_e192_d_n4 * p.p1);
        let eq4_e194_d_n5: f64 = (eq4_e192_d_n5 * p.p1);
        let eq4_e194_d_n6: f64 = (eq4_e192_d_n6 * p.p1);
        let eq4_e194_d_n7: f64 = (eq4_e192_d_n7 * p.p1);
        let eq4_e194_d_n8: f64 = (eq4_e192_d_n8 * p.p1);
        let eq4_e194_d_n9: f64 = (eq4_e192_d_n9 * p.p1);
        let eq4_e194_d_n10: f64 = (eq4_e192_d_n10 * p.p1);
        let eq4_e194_d_n11: f64 = (eq4_e192_d_n11 * p.p1);
        let eq4_e194_d_b0: f64 = (eq4_e192_d_b0 * p.p1);
        let eq4_e194_d_b1: f64 = (eq4_e192_d_b1 * p.p1);
        (eq4_e194, eq4_e194_d_n0, eq4_e194_d_n1, eq4_e194_d_n2, eq4_e194_d_n3, eq4_e194_d_n4, eq4_e194_d_n5, eq4_e194_d_n6, eq4_e194_d_n7, eq4_e194_d_n8, eq4_e194_d_n9, eq4_e194_d_n10, eq4_e194_d_n11, eq4_e194_d_b0, eq4_e194_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e196;
        let eq4_node_derivatives: [f64; 12] = [eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n2, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10, eq4_e196_d_n11];
        let eq4_branch_derivatives: [f64; 2] = [eq4_e196_d_b0, eq4_e196_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e206, eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n2, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10, eq5_e206_d_n11, eq5_e206_d_b0, eq5_e206_d_b1,) = {
    if (!s.b[563]) {
        let eq5_e201: f64 = (-s.v[82]);
        let eq5_e201_d_n0: f64 = (-s.dn[82][0]);
        let eq5_e201_d_n1: f64 = (-s.dn[82][1]);
        let eq5_e201_d_n2: f64 = (-s.dn[82][2]);
        let eq5_e201_d_n3: f64 = (-s.dn[82][3]);
        let eq5_e201_d_n4: f64 = (-s.dn[82][4]);
        let eq5_e201_d_n5: f64 = (-s.dn[82][5]);
        let eq5_e201_d_n6: f64 = (-s.dn[82][6]);
        let eq5_e201_d_n7: f64 = (-s.dn[82][7]);
        let eq5_e201_d_n8: f64 = (-s.dn[82][8]);
        let eq5_e201_d_n9: f64 = (-s.dn[82][9]);
        let eq5_e201_d_n10: f64 = (-s.dn[82][10]);
        let eq5_e201_d_n11: f64 = (-s.dn[82][11]);
        let eq5_e201_d_b0: f64 = (-s.db[82][0]);
        let eq5_e201_d_b1: f64 = (-s.db[82][1]);
        let eq5_e202: f64 = (p.p3 * eq5_e201);
        let eq5_e202_d_n0: f64 = (p.p3 * eq5_e201_d_n0);
        let eq5_e202_d_n1: f64 = (p.p3 * eq5_e201_d_n1);
        let eq5_e202_d_n2: f64 = (p.p3 * eq5_e201_d_n2);
        let eq5_e202_d_n3: f64 = (p.p3 * eq5_e201_d_n3);
        let eq5_e202_d_n4: f64 = (p.p3 * eq5_e201_d_n4);
        let eq5_e202_d_n5: f64 = (p.p3 * eq5_e201_d_n5);
        let eq5_e202_d_n6: f64 = (p.p3 * eq5_e201_d_n6);
        let eq5_e202_d_n7: f64 = (p.p3 * eq5_e201_d_n7);
        let eq5_e202_d_n8: f64 = (p.p3 * eq5_e201_d_n8);
        let eq5_e202_d_n9: f64 = (p.p3 * eq5_e201_d_n9);
        let eq5_e202_d_n10: f64 = (p.p3 * eq5_e201_d_n10);
        let eq5_e202_d_n11: f64 = (p.p3 * eq5_e201_d_n11);
        let eq5_e202_d_b0: f64 = (p.p3 * eq5_e201_d_b0);
        let eq5_e202_d_b1: f64 = (p.p3 * eq5_e201_d_b1);
        let eq5_e204: f64 = (eq5_e202 * p.p1);
        let eq5_e204_d_n0: f64 = (eq5_e202_d_n0 * p.p1);
        let eq5_e204_d_n1: f64 = (eq5_e202_d_n1 * p.p1);
        let eq5_e204_d_n2: f64 = (eq5_e202_d_n2 * p.p1);
        let eq5_e204_d_n3: f64 = (eq5_e202_d_n3 * p.p1);
        let eq5_e204_d_n4: f64 = (eq5_e202_d_n4 * p.p1);
        let eq5_e204_d_n5: f64 = (eq5_e202_d_n5 * p.p1);
        let eq5_e204_d_n6: f64 = (eq5_e202_d_n6 * p.p1);
        let eq5_e204_d_n7: f64 = (eq5_e202_d_n7 * p.p1);
        let eq5_e204_d_n8: f64 = (eq5_e202_d_n8 * p.p1);
        let eq5_e204_d_n9: f64 = (eq5_e202_d_n9 * p.p1);
        let eq5_e204_d_n10: f64 = (eq5_e202_d_n10 * p.p1);
        let eq5_e204_d_n11: f64 = (eq5_e202_d_n11 * p.p1);
        let eq5_e204_d_b0: f64 = (eq5_e202_d_b0 * p.p1);
        let eq5_e204_d_b1: f64 = (eq5_e202_d_b1 * p.p1);
        (eq5_e204, eq5_e204_d_n0, eq5_e204_d_n1, eq5_e204_d_n2, eq5_e204_d_n3, eq5_e204_d_n4, eq5_e204_d_n5, eq5_e204_d_n6, eq5_e204_d_n7, eq5_e204_d_n8, eq5_e204_d_n9, eq5_e204_d_n10, eq5_e204_d_n11, eq5_e204_d_b0, eq5_e204_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e206;
        let eq5_node_derivatives: [f64; 12] = [eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n2, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10, eq5_e206_d_n11];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e206_d_b0, eq5_e206_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e209: f64 = (p.p3 * s.v[180]);
        let eq6_e209_d_n0: f64 = (p.p3 * s.dn[180][0]);
        let eq6_e209_d_n1: f64 = (p.p3 * s.dn[180][1]);
        let eq6_e209_d_n2: f64 = (p.p3 * s.dn[180][2]);
        let eq6_e209_d_n3: f64 = (p.p3 * s.dn[180][3]);
        let eq6_e209_d_n4: f64 = (p.p3 * s.dn[180][4]);
        let eq6_e209_d_n5: f64 = (p.p3 * s.dn[180][5]);
        let eq6_e209_d_n6: f64 = (p.p3 * s.dn[180][6]);
        let eq6_e209_d_n7: f64 = (p.p3 * s.dn[180][7]);
        let eq6_e209_d_n8: f64 = (p.p3 * s.dn[180][8]);
        let eq6_e209_d_n9: f64 = (p.p3 * s.dn[180][9]);
        let eq6_e209_d_n10: f64 = (p.p3 * s.dn[180][10]);
        let eq6_e209_d_n11: f64 = (p.p3 * s.dn[180][11]);
        let eq6_e209_d_b0: f64 = (p.p3 * s.db[180][0]);
        let eq6_e209_d_b1: f64 = (p.p3 * s.db[180][1]);
        let eq6_e211: f64 = (eq6_e209 * p.p1);
        let eq6_e211_d_n0: f64 = (eq6_e209_d_n0 * p.p1);
        let eq6_e211_d_n1: f64 = (eq6_e209_d_n1 * p.p1);
        let eq6_e211_d_n2: f64 = (eq6_e209_d_n2 * p.p1);
        let eq6_e211_d_n3: f64 = (eq6_e209_d_n3 * p.p1);
        let eq6_e211_d_n4: f64 = (eq6_e209_d_n4 * p.p1);
        let eq6_e211_d_n5: f64 = (eq6_e209_d_n5 * p.p1);
        let eq6_e211_d_n6: f64 = (eq6_e209_d_n6 * p.p1);
        let eq6_e211_d_n7: f64 = (eq6_e209_d_n7 * p.p1);
        let eq6_e211_d_n8: f64 = (eq6_e209_d_n8 * p.p1);
        let eq6_e211_d_n9: f64 = (eq6_e209_d_n9 * p.p1);
        let eq6_e211_d_n10: f64 = (eq6_e209_d_n10 * p.p1);
        let eq6_e211_d_n11: f64 = (eq6_e209_d_n11 * p.p1);
        let eq6_e211_d_b0: f64 = (eq6_e209_d_b0 * p.p1);
        let eq6_e211_d_b1: f64 = (eq6_e209_d_b1 * p.p1);
        let eq6_value: f64 = eq6_e211;
        let eq6_node_derivatives: [f64; 12] = [eq6_e211_d_n0, eq6_e211_d_n1, eq6_e211_d_n2, eq6_e211_d_n3, eq6_e211_d_n4, eq6_e211_d_n5, eq6_e211_d_n6, eq6_e211_d_n7, eq6_e211_d_n8, eq6_e211_d_n9, eq6_e211_d_n10, eq6_e211_d_n11];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e211_d_b0, eq6_e211_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e214: f64 = (-1.0);
        let eq7_e216: f64 = (eq7_e214 * s.v[201]);
        let eq7_e216_d_n0: f64 = (eq7_e214 * s.dn[201][0]);
        let eq7_e216_d_n1: f64 = (eq7_e214 * s.dn[201][1]);
        let eq7_e216_d_n2: f64 = (eq7_e214 * s.dn[201][2]);
        let eq7_e216_d_n3: f64 = (eq7_e214 * s.dn[201][3]);
        let eq7_e216_d_n4: f64 = (eq7_e214 * s.dn[201][4]);
        let eq7_e216_d_n5: f64 = (eq7_e214 * s.dn[201][5]);
        let eq7_e216_d_n6: f64 = (eq7_e214 * s.dn[201][6]);
        let eq7_e216_d_n7: f64 = (eq7_e214 * s.dn[201][7]);
        let eq7_e216_d_n8: f64 = (eq7_e214 * s.dn[201][8]);
        let eq7_e216_d_n9: f64 = (eq7_e214 * s.dn[201][9]);
        let eq7_e216_d_n10: f64 = (eq7_e214 * s.dn[201][10]);
        let eq7_e216_d_n11: f64 = (eq7_e214 * s.dn[201][11]);
        let eq7_e216_d_b0: f64 = (eq7_e214 * s.db[201][0]);
        let eq7_e216_d_b1: f64 = (eq7_e214 * s.db[201][1]);
        let eq7_e217: f64 = (p.p3 * eq7_e216);
        let eq7_e217_d_n0: f64 = (p.p3 * eq7_e216_d_n0);
        let eq7_e217_d_n1: f64 = (p.p3 * eq7_e216_d_n1);
        let eq7_e217_d_n2: f64 = (p.p3 * eq7_e216_d_n2);
        let eq7_e217_d_n3: f64 = (p.p3 * eq7_e216_d_n3);
        let eq7_e217_d_n4: f64 = (p.p3 * eq7_e216_d_n4);
        let eq7_e217_d_n5: f64 = (p.p3 * eq7_e216_d_n5);
        let eq7_e217_d_n6: f64 = (p.p3 * eq7_e216_d_n6);
        let eq7_e217_d_n7: f64 = (p.p3 * eq7_e216_d_n7);
        let eq7_e217_d_n8: f64 = (p.p3 * eq7_e216_d_n8);
        let eq7_e217_d_n9: f64 = (p.p3 * eq7_e216_d_n9);
        let eq7_e217_d_n10: f64 = (p.p3 * eq7_e216_d_n10);
        let eq7_e217_d_n11: f64 = (p.p3 * eq7_e216_d_n11);
        let eq7_e217_d_b0: f64 = (p.p3 * eq7_e216_d_b0);
        let eq7_e217_d_b1: f64 = (p.p3 * eq7_e216_d_b1);
        let eq7_e219: f64 = (eq7_e217 * p.p1);
        let eq7_e219_d_n0: f64 = (eq7_e217_d_n0 * p.p1);
        let eq7_e219_d_n1: f64 = (eq7_e217_d_n1 * p.p1);
        let eq7_e219_d_n2: f64 = (eq7_e217_d_n2 * p.p1);
        let eq7_e219_d_n3: f64 = (eq7_e217_d_n3 * p.p1);
        let eq7_e219_d_n4: f64 = (eq7_e217_d_n4 * p.p1);
        let eq7_e219_d_n5: f64 = (eq7_e217_d_n5 * p.p1);
        let eq7_e219_d_n6: f64 = (eq7_e217_d_n6 * p.p1);
        let eq7_e219_d_n7: f64 = (eq7_e217_d_n7 * p.p1);
        let eq7_e219_d_n8: f64 = (eq7_e217_d_n8 * p.p1);
        let eq7_e219_d_n9: f64 = (eq7_e217_d_n9 * p.p1);
        let eq7_e219_d_n10: f64 = (eq7_e217_d_n10 * p.p1);
        let eq7_e219_d_n11: f64 = (eq7_e217_d_n11 * p.p1);
        let eq7_e219_d_b0: f64 = (eq7_e217_d_b0 * p.p1);
        let eq7_e219_d_b1: f64 = (eq7_e217_d_b1 * p.p1);
        let eq7_value: f64 = eq7_e219;
        let eq7_node_derivatives: [f64; 12] = [eq7_e219_d_n0, eq7_e219_d_n1, eq7_e219_d_n2, eq7_e219_d_n3, eq7_e219_d_n4, eq7_e219_d_n5, eq7_e219_d_n6, eq7_e219_d_n7, eq7_e219_d_n8, eq7_e219_d_n9, eq7_e219_d_n10, eq7_e219_d_n11];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e219_d_b0, eq7_e219_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
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
        let eq8_e222: f64 = (p.p3 * s.v[245]);
        let eq8_e222_d_n0: f64 = (p.p3 * s.dn[245][0]);
        let eq8_e222_d_n1: f64 = (p.p3 * s.dn[245][1]);
        let eq8_e222_d_n2: f64 = (p.p3 * s.dn[245][2]);
        let eq8_e222_d_n3: f64 = (p.p3 * s.dn[245][3]);
        let eq8_e222_d_n4: f64 = (p.p3 * s.dn[245][4]);
        let eq8_e222_d_n5: f64 = (p.p3 * s.dn[245][5]);
        let eq8_e222_d_n6: f64 = (p.p3 * s.dn[245][6]);
        let eq8_e222_d_n7: f64 = (p.p3 * s.dn[245][7]);
        let eq8_e222_d_n8: f64 = (p.p3 * s.dn[245][8]);
        let eq8_e222_d_n9: f64 = (p.p3 * s.dn[245][9]);
        let eq8_e222_d_n10: f64 = (p.p3 * s.dn[245][10]);
        let eq8_e222_d_n11: f64 = (p.p3 * s.dn[245][11]);
        let eq8_e222_d_b0: f64 = (p.p3 * s.db[245][0]);
        let eq8_e222_d_b1: f64 = (p.p3 * s.db[245][1]);
        let eq8_e224: f64 = (eq8_e222 / s.v[28]);
        let eq8_e224_d_n0: f64 = (((eq8_e222_d_n0 * s.v[28]) - (eq8_e222 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n1: f64 = (((eq8_e222_d_n1 * s.v[28]) - (eq8_e222 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n2: f64 = (((eq8_e222_d_n2 * s.v[28]) - (eq8_e222 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n3: f64 = (((eq8_e222_d_n3 * s.v[28]) - (eq8_e222 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n4: f64 = (((eq8_e222_d_n4 * s.v[28]) - (eq8_e222 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n5: f64 = (((eq8_e222_d_n5 * s.v[28]) - (eq8_e222 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n6: f64 = (((eq8_e222_d_n6 * s.v[28]) - (eq8_e222 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n7: f64 = (((eq8_e222_d_n7 * s.v[28]) - (eq8_e222 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n8: f64 = (((eq8_e222_d_n8 * s.v[28]) - (eq8_e222 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n9: f64 = (((eq8_e222_d_n9 * s.v[28]) - (eq8_e222 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n10: f64 = (((eq8_e222_d_n10 * s.v[28]) - (eq8_e222 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_n11: f64 = (((eq8_e222_d_n11 * s.v[28]) - (eq8_e222 * s.dn[28][11])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_b0: f64 = (((eq8_e222_d_b0 * s.v[28]) - (eq8_e222 * s.db[28][0])) / (s.v[28] * s.v[28]));
        let eq8_e224_d_b1: f64 = (((eq8_e222_d_b1 * s.v[28]) - (eq8_e222 * s.db[28][1])) / (s.v[28] * s.v[28]));
        let eq8_e226: f64 = (eq8_e224 * p.p1);
        let eq8_e226_d_n0: f64 = (eq8_e224_d_n0 * p.p1);
        let eq8_e226_d_n1: f64 = (eq8_e224_d_n1 * p.p1);
        let eq8_e226_d_n2: f64 = (eq8_e224_d_n2 * p.p1);
        let eq8_e226_d_n3: f64 = (eq8_e224_d_n3 * p.p1);
        let eq8_e226_d_n4: f64 = (eq8_e224_d_n4 * p.p1);
        let eq8_e226_d_n5: f64 = (eq8_e224_d_n5 * p.p1);
        let eq8_e226_d_n6: f64 = (eq8_e224_d_n6 * p.p1);
        let eq8_e226_d_n7: f64 = (eq8_e224_d_n7 * p.p1);
        let eq8_e226_d_n8: f64 = (eq8_e224_d_n8 * p.p1);
        let eq8_e226_d_n9: f64 = (eq8_e224_d_n9 * p.p1);
        let eq8_e226_d_n10: f64 = (eq8_e224_d_n10 * p.p1);
        let eq8_e226_d_n11: f64 = (eq8_e224_d_n11 * p.p1);
        let eq8_e226_d_b0: f64 = (eq8_e224_d_b0 * p.p1);
        let eq8_e226_d_b1: f64 = (eq8_e224_d_b1 * p.p1);
        let eq8_value: f64 = eq8_e226;
        let eq8_node_derivatives: [f64; 12] = [eq8_e226_d_n0, eq8_e226_d_n1, eq8_e226_d_n2, eq8_e226_d_n3, eq8_e226_d_n4, eq8_e226_d_n5, eq8_e226_d_n6, eq8_e226_d_n7, eq8_e226_d_n8, eq8_e226_d_n9, eq8_e226_d_n10, eq8_e226_d_n11];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e226_d_b0, eq8_e226_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(4),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e229: f64 = (p.p3 * s.v[246]);
        let eq9_e229_d_n0: f64 = (p.p3 * s.dn[246][0]);
        let eq9_e229_d_n1: f64 = (p.p3 * s.dn[246][1]);
        let eq9_e229_d_n2: f64 = (p.p3 * s.dn[246][2]);
        let eq9_e229_d_n3: f64 = (p.p3 * s.dn[246][3]);
        let eq9_e229_d_n4: f64 = (p.p3 * s.dn[246][4]);
        let eq9_e229_d_n5: f64 = (p.p3 * s.dn[246][5]);
        let eq9_e229_d_n6: f64 = (p.p3 * s.dn[246][6]);
        let eq9_e229_d_n7: f64 = (p.p3 * s.dn[246][7]);
        let eq9_e229_d_n8: f64 = (p.p3 * s.dn[246][8]);
        let eq9_e229_d_n9: f64 = (p.p3 * s.dn[246][9]);
        let eq9_e229_d_n10: f64 = (p.p3 * s.dn[246][10]);
        let eq9_e229_d_n11: f64 = (p.p3 * s.dn[246][11]);
        let eq9_e229_d_b0: f64 = (p.p3 * s.db[246][0]);
        let eq9_e229_d_b1: f64 = (p.p3 * s.db[246][1]);
        let eq9_e231: f64 = (eq9_e229 / s.v[30]);
        let eq9_e231_d_n0: f64 = (((eq9_e229_d_n0 * s.v[30]) - (eq9_e229 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n1: f64 = (((eq9_e229_d_n1 * s.v[30]) - (eq9_e229 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n2: f64 = (((eq9_e229_d_n2 * s.v[30]) - (eq9_e229 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n3: f64 = (((eq9_e229_d_n3 * s.v[30]) - (eq9_e229 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n4: f64 = (((eq9_e229_d_n4 * s.v[30]) - (eq9_e229 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n5: f64 = (((eq9_e229_d_n5 * s.v[30]) - (eq9_e229 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n6: f64 = (((eq9_e229_d_n6 * s.v[30]) - (eq9_e229 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n7: f64 = (((eq9_e229_d_n7 * s.v[30]) - (eq9_e229 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n8: f64 = (((eq9_e229_d_n8 * s.v[30]) - (eq9_e229 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n9: f64 = (((eq9_e229_d_n9 * s.v[30]) - (eq9_e229 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n10: f64 = (((eq9_e229_d_n10 * s.v[30]) - (eq9_e229 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_n11: f64 = (((eq9_e229_d_n11 * s.v[30]) - (eq9_e229 * s.dn[30][11])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_b0: f64 = (((eq9_e229_d_b0 * s.v[30]) - (eq9_e229 * s.db[30][0])) / (s.v[30] * s.v[30]));
        let eq9_e231_d_b1: f64 = (((eq9_e229_d_b1 * s.v[30]) - (eq9_e229 * s.db[30][1])) / (s.v[30] * s.v[30]));
        let eq9_e233: f64 = (eq9_e231 * p.p1);
        let eq9_e233_d_n0: f64 = (eq9_e231_d_n0 * p.p1);
        let eq9_e233_d_n1: f64 = (eq9_e231_d_n1 * p.p1);
        let eq9_e233_d_n2: f64 = (eq9_e231_d_n2 * p.p1);
        let eq9_e233_d_n3: f64 = (eq9_e231_d_n3 * p.p1);
        let eq9_e233_d_n4: f64 = (eq9_e231_d_n4 * p.p1);
        let eq9_e233_d_n5: f64 = (eq9_e231_d_n5 * p.p1);
        let eq9_e233_d_n6: f64 = (eq9_e231_d_n6 * p.p1);
        let eq9_e233_d_n7: f64 = (eq9_e231_d_n7 * p.p1);
        let eq9_e233_d_n8: f64 = (eq9_e231_d_n8 * p.p1);
        let eq9_e233_d_n9: f64 = (eq9_e231_d_n9 * p.p1);
        let eq9_e233_d_n10: f64 = (eq9_e231_d_n10 * p.p1);
        let eq9_e233_d_n11: f64 = (eq9_e231_d_n11 * p.p1);
        let eq9_e233_d_b0: f64 = (eq9_e231_d_b0 * p.p1);
        let eq9_e233_d_b1: f64 = (eq9_e231_d_b1 * p.p1);
        let eq9_value: f64 = eq9_e233;
        let eq9_node_derivatives: [f64; 12] = [eq9_e233_d_n0, eq9_e233_d_n1, eq9_e233_d_n2, eq9_e233_d_n3, eq9_e233_d_n4, eq9_e233_d_n5, eq9_e233_d_n6, eq9_e233_d_n7, eq9_e233_d_n8, eq9_e233_d_n9, eq9_e233_d_n10, eq9_e233_d_n11];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e233_d_b0, eq9_e233_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_value: f64 = s.v[102];
        let eq10_node_derivatives: [f64; 12] = [s.dn[102][0], s.dn[102][1], s.dn[102][2], s.dn[102][3], s.dn[102][4], s.dn[102][5], s.dn[102][6], s.dn[102][7], s.dn[102][8], s.dn[102][9], s.dn[102][10], s.dn[102][11]];
        let eq10_branch_derivatives: [f64; 2] = [s.db[102][0], s.db[102][1]];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_value: f64 = s.v[209];
        let eq11_node_derivatives: [f64; 12] = [s.dn[209][0], s.dn[209][1], s.dn[209][2], s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], s.dn[209][7], s.dn[209][8], s.dn[209][9], s.dn[209][10], s.dn[209][11]];
        let eq11_branch_derivatives: [f64; 2] = [s.db[209][0], s.db[209][1]];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e237: f64 = (-1.0);
        let eq12_e239: f64 = (eq12_e237 * s.v[208]);
        let eq12_e239_d_n0: f64 = (eq12_e237 * s.dn[208][0]);
        let eq12_e239_d_n1: f64 = (eq12_e237 * s.dn[208][1]);
        let eq12_e239_d_n2: f64 = (eq12_e237 * s.dn[208][2]);
        let eq12_e239_d_n3: f64 = (eq12_e237 * s.dn[208][3]);
        let eq12_e239_d_n4: f64 = (eq12_e237 * s.dn[208][4]);
        let eq12_e239_d_n5: f64 = (eq12_e237 * s.dn[208][5]);
        let eq12_e239_d_n6: f64 = (eq12_e237 * s.dn[208][6]);
        let eq12_e239_d_n7: f64 = (eq12_e237 * s.dn[208][7]);
        let eq12_e239_d_n8: f64 = (eq12_e237 * s.dn[208][8]);
        let eq12_e239_d_n9: f64 = (eq12_e237 * s.dn[208][9]);
        let eq12_e239_d_n10: f64 = (eq12_e237 * s.dn[208][10]);
        let eq12_e239_d_n11: f64 = (eq12_e237 * s.dn[208][11]);
        let eq12_e239_d_b0: f64 = (eq12_e237 * s.db[208][0]);
        let eq12_e239_d_b1: f64 = (eq12_e237 * s.db[208][1]);
        let eq12_e241: f64 = (eq12_e239 * p.p1);
        let eq12_e241_d_n0: f64 = (eq12_e239_d_n0 * p.p1);
        let eq12_e241_d_n1: f64 = (eq12_e239_d_n1 * p.p1);
        let eq12_e241_d_n2: f64 = (eq12_e239_d_n2 * p.p1);
        let eq12_e241_d_n3: f64 = (eq12_e239_d_n3 * p.p1);
        let eq12_e241_d_n4: f64 = (eq12_e239_d_n4 * p.p1);
        let eq12_e241_d_n5: f64 = (eq12_e239_d_n5 * p.p1);
        let eq12_e241_d_n6: f64 = (eq12_e239_d_n6 * p.p1);
        let eq12_e241_d_n7: f64 = (eq12_e239_d_n7 * p.p1);
        let eq12_e241_d_n8: f64 = (eq12_e239_d_n8 * p.p1);
        let eq12_e241_d_n9: f64 = (eq12_e239_d_n9 * p.p1);
        let eq12_e241_d_n10: f64 = (eq12_e239_d_n10 * p.p1);
        let eq12_e241_d_n11: f64 = (eq12_e239_d_n11 * p.p1);
        let eq12_e241_d_b0: f64 = (eq12_e239_d_b0 * p.p1);
        let eq12_e241_d_b1: f64 = (eq12_e239_d_b1 * p.p1);
        let eq12_value: f64 = eq12_e241;
        let eq12_node_derivatives: [f64; 12] = [eq12_e241_d_n0, eq12_e241_d_n1, eq12_e241_d_n2, eq12_e241_d_n3, eq12_e241_d_n4, eq12_e241_d_n5, eq12_e241_d_n6, eq12_e241_d_n7, eq12_e241_d_n8, eq12_e241_d_n9, eq12_e241_d_n10, eq12_e241_d_n11];
        let eq12_branch_derivatives: [f64; 2] = [eq12_e241_d_b0, eq12_e241_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e245: f64 = (s.v[210] + s.v[215]);
        let eq13_e245_d_n0: f64 = (s.dn[210][0] + s.dn[215][0]);
        let eq13_e245_d_n1: f64 = (s.dn[210][1] + s.dn[215][1]);
        let eq13_e245_d_n2: f64 = (s.dn[210][2] + s.dn[215][2]);
        let eq13_e245_d_n3: f64 = (s.dn[210][3] + s.dn[215][3]);
        let eq13_e245_d_n4: f64 = (s.dn[210][4] + s.dn[215][4]);
        let eq13_e245_d_n5: f64 = (s.dn[210][5] + s.dn[215][5]);
        let eq13_e245_d_n6: f64 = (s.dn[210][6] + s.dn[215][6]);
        let eq13_e245_d_n7: f64 = (s.dn[210][7] + s.dn[215][7]);
        let eq13_e245_d_n8: f64 = (s.dn[210][8] + s.dn[215][8]);
        let eq13_e245_d_n9: f64 = (s.dn[210][9] + s.dn[215][9]);
        let eq13_e245_d_n10: f64 = (s.dn[210][10] + s.dn[215][10]);
        let eq13_e245_d_n11: f64 = (s.dn[210][11] + s.dn[215][11]);
        let eq13_e245_d_b0: f64 = (s.db[210][0] + s.db[215][0]);
        let eq13_e245_d_b1: f64 = (s.db[210][1] + s.db[215][1]);
        let eq13_e247: f64 = (eq13_e245 + s.v[227]);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + s.dn[227][0]);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + s.dn[227][1]);
        let eq13_e247_d_n2: f64 = (eq13_e245_d_n2 + s.dn[227][2]);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + s.dn[227][3]);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + s.dn[227][4]);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + s.dn[227][5]);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + s.dn[227][6]);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + s.dn[227][7]);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + s.dn[227][8]);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + s.dn[227][9]);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + s.dn[227][10]);
        let eq13_e247_d_n11: f64 = (eq13_e245_d_n11 + s.dn[227][11]);
        let eq13_e247_d_b0: f64 = (eq13_e245_d_b0 + s.db[227][0]);
        let eq13_e247_d_b1: f64 = (eq13_e245_d_b1 + s.db[227][1]);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n2: f64 = (p.p3 * eq13_e247_d_n2);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e248_d_n11: f64 = (p.p3 * eq13_e247_d_n11);
        let eq13_e248_d_b0: f64 = (p.p3 * eq13_e247_d_b0);
        let eq13_e248_d_b1: f64 = (p.p3 * eq13_e247_d_b1);
        let eq13_e249: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq13_e248);
        let eq13_e249_d_n0: f64 = (eq13_e248_d_n0 * ddt_scale);
        let eq13_e249_d_n1: f64 = (eq13_e248_d_n1 * ddt_scale);
        let eq13_e249_d_n2: f64 = (eq13_e248_d_n2 * ddt_scale);
        let eq13_e249_d_n3: f64 = (eq13_e248_d_n3 * ddt_scale);
        let eq13_e249_d_n4: f64 = (eq13_e248_d_n4 * ddt_scale);
        let eq13_e249_d_n5: f64 = (eq13_e248_d_n5 * ddt_scale);
        let eq13_e249_d_n6: f64 = (eq13_e248_d_n6 * ddt_scale);
        let eq13_e249_d_n7: f64 = (eq13_e248_d_n7 * ddt_scale);
        let eq13_e249_d_n8: f64 = (eq13_e248_d_n8 * ddt_scale);
        let eq13_e249_d_n9: f64 = (eq13_e248_d_n9 * ddt_scale);
        let eq13_e249_d_n10: f64 = (eq13_e248_d_n10 * ddt_scale);
        let eq13_e249_d_n11: f64 = (eq13_e248_d_n11 * ddt_scale);
        let eq13_e249_d_b0: f64 = (eq13_e248_d_b0 * ddt_scale);
        let eq13_e249_d_b1: f64 = (eq13_e248_d_b1 * ddt_scale);
        let eq13_e251: f64 = (eq13_e249 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e249_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e249_d_n1 * p.p1);
        let eq13_e251_d_n2: f64 = (eq13_e249_d_n2 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e249_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e249_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e249_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e249_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e249_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e249_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e249_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e249_d_n10 * p.p1);
        let eq13_e251_d_n11: f64 = (eq13_e249_d_n11 * p.p1);
        let eq13_e251_d_b0: f64 = (eq13_e249_d_b0 * p.p1);
        let eq13_e251_d_b1: f64 = (eq13_e249_d_b1 * p.p1);
        let eq13_value: f64 = eq13_e251;
        let eq13_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, eq13_e251_d_n2, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, eq13_e251_d_n11];
        let eq13_branch_derivatives: [f64; 2] = [eq13_e251_d_b0, eq13_e251_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * s.v[212]);
        let eq14_e254_d_n0: f64 = (p.p3 * s.dn[212][0]);
        let eq14_e254_d_n1: f64 = (p.p3 * s.dn[212][1]);
        let eq14_e254_d_n2: f64 = (p.p3 * s.dn[212][2]);
        let eq14_e254_d_n3: f64 = (p.p3 * s.dn[212][3]);
        let eq14_e254_d_n4: f64 = (p.p3 * s.dn[212][4]);
        let eq14_e254_d_n5: f64 = (p.p3 * s.dn[212][5]);
        let eq14_e254_d_n6: f64 = (p.p3 * s.dn[212][6]);
        let eq14_e254_d_n7: f64 = (p.p3 * s.dn[212][7]);
        let eq14_e254_d_n8: f64 = (p.p3 * s.dn[212][8]);
        let eq14_e254_d_n9: f64 = (p.p3 * s.dn[212][9]);
        let eq14_e254_d_n10: f64 = (p.p3 * s.dn[212][10]);
        let eq14_e254_d_n11: f64 = (p.p3 * s.dn[212][11]);
        let eq14_e254_d_b0: f64 = (p.p3 * s.db[212][0]);
        let eq14_e254_d_b1: f64 = (p.p3 * s.db[212][1]);
        let eq14_e255: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq14_e254);
        let eq14_e255_d_n0: f64 = (eq14_e254_d_n0 * ddt_scale);
        let eq14_e255_d_n1: f64 = (eq14_e254_d_n1 * ddt_scale);
        let eq14_e255_d_n2: f64 = (eq14_e254_d_n2 * ddt_scale);
        let eq14_e255_d_n3: f64 = (eq14_e254_d_n3 * ddt_scale);
        let eq14_e255_d_n4: f64 = (eq14_e254_d_n4 * ddt_scale);
        let eq14_e255_d_n5: f64 = (eq14_e254_d_n5 * ddt_scale);
        let eq14_e255_d_n6: f64 = (eq14_e254_d_n6 * ddt_scale);
        let eq14_e255_d_n7: f64 = (eq14_e254_d_n7 * ddt_scale);
        let eq14_e255_d_n8: f64 = (eq14_e254_d_n8 * ddt_scale);
        let eq14_e255_d_n9: f64 = (eq14_e254_d_n9 * ddt_scale);
        let eq14_e255_d_n10: f64 = (eq14_e254_d_n10 * ddt_scale);
        let eq14_e255_d_n11: f64 = (eq14_e254_d_n11 * ddt_scale);
        let eq14_e255_d_b0: f64 = (eq14_e254_d_b0 * ddt_scale);
        let eq14_e255_d_b1: f64 = (eq14_e254_d_b1 * ddt_scale);
        let eq14_e257: f64 = (eq14_e255 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e255_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e255_d_n1 * p.p1);
        let eq14_e257_d_n2: f64 = (eq14_e255_d_n2 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e255_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e255_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e255_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e255_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e255_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e255_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e255_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e255_d_n10 * p.p1);
        let eq14_e257_d_n11: f64 = (eq14_e255_d_n11 * p.p1);
        let eq14_e257_d_b0: f64 = (eq14_e255_d_b0 * p.p1);
        let eq14_e257_d_b1: f64 = (eq14_e255_d_b1 * p.p1);
        let eq14_value: f64 = eq14_e257;
        let eq14_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, eq14_e257_d_n2, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, eq14_e257_d_n11];
        let eq14_branch_derivatives: [f64; 2] = [eq14_e257_d_b0, eq14_e257_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (s.v[213] + s.v[216]);
        let eq15_e261_d_n0: f64 = (s.dn[213][0] + s.dn[216][0]);
        let eq15_e261_d_n1: f64 = (s.dn[213][1] + s.dn[216][1]);
        let eq15_e261_d_n2: f64 = (s.dn[213][2] + s.dn[216][2]);
        let eq15_e261_d_n3: f64 = (s.dn[213][3] + s.dn[216][3]);
        let eq15_e261_d_n4: f64 = (s.dn[213][4] + s.dn[216][4]);
        let eq15_e261_d_n5: f64 = (s.dn[213][5] + s.dn[216][5]);
        let eq15_e261_d_n6: f64 = (s.dn[213][6] + s.dn[216][6]);
        let eq15_e261_d_n7: f64 = (s.dn[213][7] + s.dn[216][7]);
        let eq15_e261_d_n8: f64 = (s.dn[213][8] + s.dn[216][8]);
        let eq15_e261_d_n9: f64 = (s.dn[213][9] + s.dn[216][9]);
        let eq15_e261_d_n10: f64 = (s.dn[213][10] + s.dn[216][10]);
        let eq15_e261_d_n11: f64 = (s.dn[213][11] + s.dn[216][11]);
        let eq15_e261_d_b0: f64 = (s.db[213][0] + s.db[216][0]);
        let eq15_e261_d_b1: f64 = (s.db[213][1] + s.db[216][1]);
        let eq15_e263: f64 = (eq15_e261 + s.v[230]);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + s.dn[230][0]);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + s.dn[230][1]);
        let eq15_e263_d_n2: f64 = (eq15_e261_d_n2 + s.dn[230][2]);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + s.dn[230][3]);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + s.dn[230][4]);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + s.dn[230][5]);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + s.dn[230][6]);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + s.dn[230][7]);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + s.dn[230][8]);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + s.dn[230][9]);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + s.dn[230][10]);
        let eq15_e263_d_n11: f64 = (eq15_e261_d_n11 + s.dn[230][11]);
        let eq15_e263_d_b0: f64 = (eq15_e261_d_b0 + s.db[230][0]);
        let eq15_e263_d_b1: f64 = (eq15_e261_d_b1 + s.db[230][1]);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n2: f64 = (p.p3 * eq15_e263_d_n2);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e264_d_n11: f64 = (p.p3 * eq15_e263_d_n11);
        let eq15_e264_d_b0: f64 = (p.p3 * eq15_e263_d_b0);
        let eq15_e264_d_b1: f64 = (p.p3 * eq15_e263_d_b1);
        let eq15_e265: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq15_e264);
        let eq15_e265_d_n0: f64 = (eq15_e264_d_n0 * ddt_scale);
        let eq15_e265_d_n1: f64 = (eq15_e264_d_n1 * ddt_scale);
        let eq15_e265_d_n2: f64 = (eq15_e264_d_n2 * ddt_scale);
        let eq15_e265_d_n3: f64 = (eq15_e264_d_n3 * ddt_scale);
        let eq15_e265_d_n4: f64 = (eq15_e264_d_n4 * ddt_scale);
        let eq15_e265_d_n5: f64 = (eq15_e264_d_n5 * ddt_scale);
        let eq15_e265_d_n6: f64 = (eq15_e264_d_n6 * ddt_scale);
        let eq15_e265_d_n7: f64 = (eq15_e264_d_n7 * ddt_scale);
        let eq15_e265_d_n8: f64 = (eq15_e264_d_n8 * ddt_scale);
        let eq15_e265_d_n9: f64 = (eq15_e264_d_n9 * ddt_scale);
        let eq15_e265_d_n10: f64 = (eq15_e264_d_n10 * ddt_scale);
        let eq15_e265_d_n11: f64 = (eq15_e264_d_n11 * ddt_scale);
        let eq15_e265_d_b0: f64 = (eq15_e264_d_b0 * ddt_scale);
        let eq15_e265_d_b1: f64 = (eq15_e264_d_b1 * ddt_scale);
        let eq15_e267: f64 = (eq15_e265 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e265_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e265_d_n1 * p.p1);
        let eq15_e267_d_n2: f64 = (eq15_e265_d_n2 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e265_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e265_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e265_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e265_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e265_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e265_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e265_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e265_d_n10 * p.p1);
        let eq15_e267_d_n11: f64 = (eq15_e265_d_n11 * p.p1);
        let eq15_e267_d_b0: f64 = (eq15_e265_d_b0 * p.p1);
        let eq15_e267_d_b1: f64 = (eq15_e265_d_b1 * p.p1);
        let eq15_value: f64 = eq15_e267;
        let eq15_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, eq15_e267_d_n2, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, eq15_e267_d_n11];
        let eq15_branch_derivatives: [f64; 2] = [eq15_e267_d_b0, eq15_e267_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * s.v[217]);
        let eq16_e270_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq16_e270_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq16_e270_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq16_e270_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq16_e270_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq16_e270_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq16_e270_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq16_e270_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq16_e270_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq16_e270_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq16_e270_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq16_e270_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq16_e270_d_b0: f64 = (p.p3 * s.db[217][0]);
        let eq16_e270_d_b1: f64 = (p.p3 * s.db[217][1]);
        let eq16_e271: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq16_e270);
        let eq16_e271_d_n0: f64 = (eq16_e270_d_n0 * ddt_scale);
        let eq16_e271_d_n1: f64 = (eq16_e270_d_n1 * ddt_scale);
        let eq16_e271_d_n2: f64 = (eq16_e270_d_n2 * ddt_scale);
        let eq16_e271_d_n3: f64 = (eq16_e270_d_n3 * ddt_scale);
        let eq16_e271_d_n4: f64 = (eq16_e270_d_n4 * ddt_scale);
        let eq16_e271_d_n5: f64 = (eq16_e270_d_n5 * ddt_scale);
        let eq16_e271_d_n6: f64 = (eq16_e270_d_n6 * ddt_scale);
        let eq16_e271_d_n7: f64 = (eq16_e270_d_n7 * ddt_scale);
        let eq16_e271_d_n8: f64 = (eq16_e270_d_n8 * ddt_scale);
        let eq16_e271_d_n9: f64 = (eq16_e270_d_n9 * ddt_scale);
        let eq16_e271_d_n10: f64 = (eq16_e270_d_n10 * ddt_scale);
        let eq16_e271_d_n11: f64 = (eq16_e270_d_n11 * ddt_scale);
        let eq16_e271_d_b0: f64 = (eq16_e270_d_b0 * ddt_scale);
        let eq16_e271_d_b1: f64 = (eq16_e270_d_b1 * ddt_scale);
        let eq16_e273: f64 = (eq16_e271 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e271_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e271_d_n1 * p.p1);
        let eq16_e273_d_n2: f64 = (eq16_e271_d_n2 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e271_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e271_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e271_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e271_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e271_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e271_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e271_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e271_d_n10 * p.p1);
        let eq16_e273_d_n11: f64 = (eq16_e271_d_n11 * p.p1);
        let eq16_e273_d_b0: f64 = (eq16_e271_d_b0 * p.p1);
        let eq16_e273_d_b1: f64 = (eq16_e271_d_b1 * p.p1);
        let eq16_value: f64 = eq16_e273;
        let eq16_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, eq16_e273_d_n2, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, eq16_e273_d_n11];
        let eq16_branch_derivatives: [f64; 2] = [eq16_e273_d_b0, eq16_e273_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
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
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * s.v[249]);
        let eq17_e278_d_n0: f64 = (eq17_e276 * s.dn[249][0]);
        let eq17_e278_d_n1: f64 = (eq17_e276 * s.dn[249][1]);
        let eq17_e278_d_n2: f64 = (eq17_e276 * s.dn[249][2]);
        let eq17_e278_d_n3: f64 = (eq17_e276 * s.dn[249][3]);
        let eq17_e278_d_n4: f64 = (eq17_e276 * s.dn[249][4]);
        let eq17_e278_d_n5: f64 = (eq17_e276 * s.dn[249][5]);
        let eq17_e278_d_n6: f64 = (eq17_e276 * s.dn[249][6]);
        let eq17_e278_d_n7: f64 = (eq17_e276 * s.dn[249][7]);
        let eq17_e278_d_n8: f64 = (eq17_e276 * s.dn[249][8]);
        let eq17_e278_d_n9: f64 = (eq17_e276 * s.dn[249][9]);
        let eq17_e278_d_n10: f64 = (eq17_e276 * s.dn[249][10]);
        let eq17_e278_d_n11: f64 = (eq17_e276 * s.dn[249][11]);
        let eq17_e278_d_b0: f64 = (eq17_e276 * s.db[249][0]);
        let eq17_e278_d_b1: f64 = (eq17_e276 * s.db[249][1]);
        let eq17_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq17_e278);
        let eq17_e279_d_n0: f64 = (eq17_e278_d_n0 * ddt_scale);
        let eq17_e279_d_n1: f64 = (eq17_e278_d_n1 * ddt_scale);
        let eq17_e279_d_n2: f64 = (eq17_e278_d_n2 * ddt_scale);
        let eq17_e279_d_n3: f64 = (eq17_e278_d_n3 * ddt_scale);
        let eq17_e279_d_n4: f64 = (eq17_e278_d_n4 * ddt_scale);
        let eq17_e279_d_n5: f64 = (eq17_e278_d_n5 * ddt_scale);
        let eq17_e279_d_n6: f64 = (eq17_e278_d_n6 * ddt_scale);
        let eq17_e279_d_n7: f64 = (eq17_e278_d_n7 * ddt_scale);
        let eq17_e279_d_n8: f64 = (eq17_e278_d_n8 * ddt_scale);
        let eq17_e279_d_n9: f64 = (eq17_e278_d_n9 * ddt_scale);
        let eq17_e279_d_n10: f64 = (eq17_e278_d_n10 * ddt_scale);
        let eq17_e279_d_n11: f64 = (eq17_e278_d_n11 * ddt_scale);
        let eq17_e279_d_b0: f64 = (eq17_e278_d_b0 * ddt_scale);
        let eq17_e279_d_b1: f64 = (eq17_e278_d_b1 * ddt_scale);
        let eq17_e281: f64 = (eq17_e279 * p.p1);
        let eq17_e281_d_n0: f64 = (eq17_e279_d_n0 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e279_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e279_d_n2 * p.p1);
        let eq17_e281_d_n3: f64 = (eq17_e279_d_n3 * p.p1);
        let eq17_e281_d_n4: f64 = (eq17_e279_d_n4 * p.p1);
        let eq17_e281_d_n5: f64 = (eq17_e279_d_n5 * p.p1);
        let eq17_e281_d_n6: f64 = (eq17_e279_d_n6 * p.p1);
        let eq17_e281_d_n7: f64 = (eq17_e279_d_n7 * p.p1);
        let eq17_e281_d_n8: f64 = (eq17_e279_d_n8 * p.p1);
        let eq17_e281_d_n9: f64 = (eq17_e279_d_n9 * p.p1);
        let eq17_e281_d_n10: f64 = (eq17_e279_d_n10 * p.p1);
        let eq17_e281_d_n11: f64 = (eq17_e279_d_n11 * p.p1);
        let eq17_e281_d_b0: f64 = (eq17_e279_d_b0 * p.p1);
        let eq17_e281_d_b1: f64 = (eq17_e279_d_b1 * p.p1);
        let eq17_value: f64 = eq17_e281;
        let eq17_node_derivatives: [f64; 12] = [eq17_e281_d_n0, eq17_e281_d_n1, eq17_e281_d_n2, eq17_e281_d_n3, eq17_e281_d_n4, eq17_e281_d_n5, eq17_e281_d_n6, eq17_e281_d_n7, eq17_e281_d_n8, eq17_e281_d_n9, eq17_e281_d_n10, eq17_e281_d_n11];
        let eq17_branch_derivatives: [f64; 2] = [eq17_e281_d_b0, eq17_e281_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * s.v[250]);
        let eq18_e286_d_n0: f64 = (eq18_e284 * s.dn[250][0]);
        let eq18_e286_d_n1: f64 = (eq18_e284 * s.dn[250][1]);
        let eq18_e286_d_n2: f64 = (eq18_e284 * s.dn[250][2]);
        let eq18_e286_d_n3: f64 = (eq18_e284 * s.dn[250][3]);
        let eq18_e286_d_n4: f64 = (eq18_e284 * s.dn[250][4]);
        let eq18_e286_d_n5: f64 = (eq18_e284 * s.dn[250][5]);
        let eq18_e286_d_n6: f64 = (eq18_e284 * s.dn[250][6]);
        let eq18_e286_d_n7: f64 = (eq18_e284 * s.dn[250][7]);
        let eq18_e286_d_n8: f64 = (eq18_e284 * s.dn[250][8]);
        let eq18_e286_d_n9: f64 = (eq18_e284 * s.dn[250][9]);
        let eq18_e286_d_n10: f64 = (eq18_e284 * s.dn[250][10]);
        let eq18_e286_d_n11: f64 = (eq18_e284 * s.dn[250][11]);
        let eq18_e286_d_b0: f64 = (eq18_e284 * s.db[250][0]);
        let eq18_e286_d_b1: f64 = (eq18_e284 * s.db[250][1]);
        let eq18_e287: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq18_e286);
        let eq18_e287_d_n0: f64 = (eq18_e286_d_n0 * ddt_scale);
        let eq18_e287_d_n1: f64 = (eq18_e286_d_n1 * ddt_scale);
        let eq18_e287_d_n2: f64 = (eq18_e286_d_n2 * ddt_scale);
        let eq18_e287_d_n3: f64 = (eq18_e286_d_n3 * ddt_scale);
        let eq18_e287_d_n4: f64 = (eq18_e286_d_n4 * ddt_scale);
        let eq18_e287_d_n5: f64 = (eq18_e286_d_n5 * ddt_scale);
        let eq18_e287_d_n6: f64 = (eq18_e286_d_n6 * ddt_scale);
        let eq18_e287_d_n7: f64 = (eq18_e286_d_n7 * ddt_scale);
        let eq18_e287_d_n8: f64 = (eq18_e286_d_n8 * ddt_scale);
        let eq18_e287_d_n9: f64 = (eq18_e286_d_n9 * ddt_scale);
        let eq18_e287_d_n10: f64 = (eq18_e286_d_n10 * ddt_scale);
        let eq18_e287_d_n11: f64 = (eq18_e286_d_n11 * ddt_scale);
        let eq18_e287_d_b0: f64 = (eq18_e286_d_b0 * ddt_scale);
        let eq18_e287_d_b1: f64 = (eq18_e286_d_b1 * ddt_scale);
        let eq18_e289: f64 = (eq18_e287 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e289_d_n2: f64 = (eq18_e287_d_n2 * p.p1);
        let eq18_e289_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e289_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e289_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e289_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e289_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e289_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e289_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e289_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e289_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_e289_d_b0: f64 = (eq18_e287_d_b0 * p.p1);
        let eq18_e289_d_b1: f64 = (eq18_e287_d_b1 * p.p1);
        let eq18_value: f64 = eq18_e289;
        let eq18_node_derivatives: [f64; 12] = [eq18_e289_d_n0, eq18_e289_d_n1, eq18_e289_d_n2, eq18_e289_d_n3, eq18_e289_d_n4, eq18_e289_d_n5, eq18_e289_d_n6, eq18_e289_d_n7, eq18_e289_d_n8, eq18_e289_d_n9, eq18_e289_d_n10, eq18_e289_d_n11];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e289_d_b0, eq18_e289_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e292: f64 = (p.p3 * s.v[172]);
        let eq19_e292_d_n0: f64 = (p.p3 * s.dn[172][0]);
        let eq19_e292_d_n1: f64 = (p.p3 * s.dn[172][1]);
        let eq19_e292_d_n2: f64 = (p.p3 * s.dn[172][2]);
        let eq19_e292_d_n3: f64 = (p.p3 * s.dn[172][3]);
        let eq19_e292_d_n4: f64 = (p.p3 * s.dn[172][4]);
        let eq19_e292_d_n5: f64 = (p.p3 * s.dn[172][5]);
        let eq19_e292_d_n6: f64 = (p.p3 * s.dn[172][6]);
        let eq19_e292_d_n7: f64 = (p.p3 * s.dn[172][7]);
        let eq19_e292_d_n8: f64 = (p.p3 * s.dn[172][8]);
        let eq19_e292_d_n9: f64 = (p.p3 * s.dn[172][9]);
        let eq19_e292_d_n10: f64 = (p.p3 * s.dn[172][10]);
        let eq19_e292_d_n11: f64 = (p.p3 * s.dn[172][11]);
        let eq19_e292_d_b0: f64 = (p.p3 * s.db[172][0]);
        let eq19_e292_d_b1: f64 = (p.p3 * s.db[172][1]);
        let eq19_e294: f64 = (eq19_e292 * p.p1);
        let eq19_e294_d_n0: f64 = (eq19_e292_d_n0 * p.p1);
        let eq19_e294_d_n1: f64 = (eq19_e292_d_n1 * p.p1);
        let eq19_e294_d_n2: f64 = (eq19_e292_d_n2 * p.p1);
        let eq19_e294_d_n3: f64 = (eq19_e292_d_n3 * p.p1);
        let eq19_e294_d_n4: f64 = (eq19_e292_d_n4 * p.p1);
        let eq19_e294_d_n5: f64 = (eq19_e292_d_n5 * p.p1);
        let eq19_e294_d_n6: f64 = (eq19_e292_d_n6 * p.p1);
        let eq19_e294_d_n7: f64 = (eq19_e292_d_n7 * p.p1);
        let eq19_e294_d_n8: f64 = (eq19_e292_d_n8 * p.p1);
        let eq19_e294_d_n9: f64 = (eq19_e292_d_n9 * p.p1);
        let eq19_e294_d_n10: f64 = (eq19_e292_d_n10 * p.p1);
        let eq19_e294_d_n11: f64 = (eq19_e292_d_n11 * p.p1);
        let eq19_e294_d_b0: f64 = (eq19_e292_d_b0 * p.p1);
        let eq19_e294_d_b1: f64 = (eq19_e292_d_b1 * p.p1);
        let eq19_value: f64 = eq19_e294;
        let eq19_node_derivatives: [f64; 12] = [eq19_e294_d_n0, eq19_e294_d_n1, eq19_e294_d_n2, eq19_e294_d_n3, eq19_e294_d_n4, eq19_e294_d_n5, eq19_e294_d_n6, eq19_e294_d_n7, eq19_e294_d_n8, eq19_e294_d_n9, eq19_e294_d_n10, eq19_e294_d_n11];
        let eq19_branch_derivatives: [f64; 2] = [eq19_e294_d_b0, eq19_e294_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e297: f64 = (p.p3 * s.v[248]);
        let eq20_e297_d_n0: f64 = (p.p3 * s.dn[248][0]);
        let eq20_e297_d_n1: f64 = (p.p3 * s.dn[248][1]);
        let eq20_e297_d_n2: f64 = (p.p3 * s.dn[248][2]);
        let eq20_e297_d_n3: f64 = (p.p3 * s.dn[248][3]);
        let eq20_e297_d_n4: f64 = (p.p3 * s.dn[248][4]);
        let eq20_e297_d_n5: f64 = (p.p3 * s.dn[248][5]);
        let eq20_e297_d_n6: f64 = (p.p3 * s.dn[248][6]);
        let eq20_e297_d_n7: f64 = (p.p3 * s.dn[248][7]);
        let eq20_e297_d_n8: f64 = (p.p3 * s.dn[248][8]);
        let eq20_e297_d_n9: f64 = (p.p3 * s.dn[248][9]);
        let eq20_e297_d_n10: f64 = (p.p3 * s.dn[248][10]);
        let eq20_e297_d_n11: f64 = (p.p3 * s.dn[248][11]);
        let eq20_e297_d_b0: f64 = (p.p3 * s.db[248][0]);
        let eq20_e297_d_b1: f64 = (p.p3 * s.db[248][1]);
        let eq20_e299: f64 = (eq20_e297 * s.v[104]);
        let eq20_e299_d_n0: f64 = ((eq20_e297_d_n0 * s.v[104]) + (eq20_e297 * s.dn[104][0]));
        let eq20_e299_d_n1: f64 = ((eq20_e297_d_n1 * s.v[104]) + (eq20_e297 * s.dn[104][1]));
        let eq20_e299_d_n2: f64 = ((eq20_e297_d_n2 * s.v[104]) + (eq20_e297 * s.dn[104][2]));
        let eq20_e299_d_n3: f64 = ((eq20_e297_d_n3 * s.v[104]) + (eq20_e297 * s.dn[104][3]));
        let eq20_e299_d_n4: f64 = ((eq20_e297_d_n4 * s.v[104]) + (eq20_e297 * s.dn[104][4]));
        let eq20_e299_d_n5: f64 = ((eq20_e297_d_n5 * s.v[104]) + (eq20_e297 * s.dn[104][5]));
        let eq20_e299_d_n6: f64 = ((eq20_e297_d_n6 * s.v[104]) + (eq20_e297 * s.dn[104][6]));
        let eq20_e299_d_n7: f64 = ((eq20_e297_d_n7 * s.v[104]) + (eq20_e297 * s.dn[104][7]));
        let eq20_e299_d_n8: f64 = ((eq20_e297_d_n8 * s.v[104]) + (eq20_e297 * s.dn[104][8]));
        let eq20_e299_d_n9: f64 = ((eq20_e297_d_n9 * s.v[104]) + (eq20_e297 * s.dn[104][9]));
        let eq20_e299_d_n10: f64 = ((eq20_e297_d_n10 * s.v[104]) + (eq20_e297 * s.dn[104][10]));
        let eq20_e299_d_n11: f64 = ((eq20_e297_d_n11 * s.v[104]) + (eq20_e297 * s.dn[104][11]));
        let eq20_e299_d_b0: f64 = ((eq20_e297_d_b0 * s.v[104]) + (eq20_e297 * s.db[104][0]));
        let eq20_e299_d_b1: f64 = ((eq20_e297_d_b1 * s.v[104]) + (eq20_e297 * s.db[104][1]));
        let eq20_e301: f64 = (eq20_e299 * p.p1);
        let eq20_e301_d_n0: f64 = (eq20_e299_d_n0 * p.p1);
        let eq20_e301_d_n1: f64 = (eq20_e299_d_n1 * p.p1);
        let eq20_e301_d_n2: f64 = (eq20_e299_d_n2 * p.p1);
        let eq20_e301_d_n3: f64 = (eq20_e299_d_n3 * p.p1);
        let eq20_e301_d_n4: f64 = (eq20_e299_d_n4 * p.p1);
        let eq20_e301_d_n5: f64 = (eq20_e299_d_n5 * p.p1);
        let eq20_e301_d_n6: f64 = (eq20_e299_d_n6 * p.p1);
        let eq20_e301_d_n7: f64 = (eq20_e299_d_n7 * p.p1);
        let eq20_e301_d_n8: f64 = (eq20_e299_d_n8 * p.p1);
        let eq20_e301_d_n9: f64 = (eq20_e299_d_n9 * p.p1);
        let eq20_e301_d_n10: f64 = (eq20_e299_d_n10 * p.p1);
        let eq20_e301_d_n11: f64 = (eq20_e299_d_n11 * p.p1);
        let eq20_e301_d_b0: f64 = (eq20_e299_d_b0 * p.p1);
        let eq20_e301_d_b1: f64 = (eq20_e299_d_b1 * p.p1);
        let eq20_value: f64 = eq20_e301;
        let eq20_node_derivatives: [f64; 12] = [eq20_e301_d_n0, eq20_e301_d_n1, eq20_e301_d_n2, eq20_e301_d_n3, eq20_e301_d_n4, eq20_e301_d_n5, eq20_e301_d_n6, eq20_e301_d_n7, eq20_e301_d_n8, eq20_e301_d_n9, eq20_e301_d_n10, eq20_e301_d_n11];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e301_d_b0, eq20_e301_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e305: f64 = (s.v[225] + s.v[234]);
        let eq21_e305_d_n0: f64 = (s.dn[225][0] + s.dn[234][0]);
        let eq21_e305_d_n1: f64 = (s.dn[225][1] + s.dn[234][1]);
        let eq21_e305_d_n2: f64 = (s.dn[225][2] + s.dn[234][2]);
        let eq21_e305_d_n3: f64 = (s.dn[225][3] + s.dn[234][3]);
        let eq21_e305_d_n4: f64 = (s.dn[225][4] + s.dn[234][4]);
        let eq21_e305_d_n5: f64 = (s.dn[225][5] + s.dn[234][5]);
        let eq21_e305_d_n6: f64 = (s.dn[225][6] + s.dn[234][6]);
        let eq21_e305_d_n7: f64 = (s.dn[225][7] + s.dn[234][7]);
        let eq21_e305_d_n8: f64 = (s.dn[225][8] + s.dn[234][8]);
        let eq21_e305_d_n9: f64 = (s.dn[225][9] + s.dn[234][9]);
        let eq21_e305_d_n10: f64 = (s.dn[225][10] + s.dn[234][10]);
        let eq21_e305_d_n11: f64 = (s.dn[225][11] + s.dn[234][11]);
        let eq21_e305_d_b0: f64 = (s.db[225][0] + s.db[234][0]);
        let eq21_e305_d_b1: f64 = (s.db[225][1] + s.db[234][1]);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n2: f64 = (p.p3 * eq21_e305_d_n2);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e306_d_n11: f64 = (p.p3 * eq21_e305_d_n11);
        let eq21_e306_d_b0: f64 = (p.p3 * eq21_e305_d_b0);
        let eq21_e306_d_b1: f64 = (p.p3 * eq21_e305_d_b1);
        let eq21_e307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq21_e306);
        let eq21_e307_d_n0: f64 = (eq21_e306_d_n0 * ddt_scale);
        let eq21_e307_d_n1: f64 = (eq21_e306_d_n1 * ddt_scale);
        let eq21_e307_d_n2: f64 = (eq21_e306_d_n2 * ddt_scale);
        let eq21_e307_d_n3: f64 = (eq21_e306_d_n3 * ddt_scale);
        let eq21_e307_d_n4: f64 = (eq21_e306_d_n4 * ddt_scale);
        let eq21_e307_d_n5: f64 = (eq21_e306_d_n5 * ddt_scale);
        let eq21_e307_d_n6: f64 = (eq21_e306_d_n6 * ddt_scale);
        let eq21_e307_d_n7: f64 = (eq21_e306_d_n7 * ddt_scale);
        let eq21_e307_d_n8: f64 = (eq21_e306_d_n8 * ddt_scale);
        let eq21_e307_d_n9: f64 = (eq21_e306_d_n9 * ddt_scale);
        let eq21_e307_d_n10: f64 = (eq21_e306_d_n10 * ddt_scale);
        let eq21_e307_d_n11: f64 = (eq21_e306_d_n11 * ddt_scale);
        let eq21_e307_d_b0: f64 = (eq21_e306_d_b0 * ddt_scale);
        let eq21_e307_d_b1: f64 = (eq21_e306_d_b1 * ddt_scale);
        let eq21_e309: f64 = (eq21_e307 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e307_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e307_d_n1 * p.p1);
        let eq21_e309_d_n2: f64 = (eq21_e307_d_n2 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e307_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e307_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e307_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e307_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e307_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e307_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e307_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e307_d_n10 * p.p1);
        let eq21_e309_d_n11: f64 = (eq21_e307_d_n11 * p.p1);
        let eq21_e309_d_b0: f64 = (eq21_e307_d_b0 * p.p1);
        let eq21_e309_d_b1: f64 = (eq21_e307_d_b1 * p.p1);
        let eq21_value: f64 = eq21_e309;
        let eq21_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, eq21_e309_d_n2, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, eq21_e309_d_n11];
        let eq21_branch_derivatives: [f64; 2] = [eq21_e309_d_b0, eq21_e309_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e314: f64 = (s.v[320] * s.v[241]);
        let eq22_e314_d_n0: f64 = (s.v[320] * s.dn[241][0]);
        let eq22_e314_d_n1: f64 = (s.v[320] * s.dn[241][1]);
        let eq22_e314_d_n2: f64 = (s.v[320] * s.dn[241][2]);
        let eq22_e314_d_n3: f64 = (s.v[320] * s.dn[241][3]);
        let eq22_e314_d_n4: f64 = (s.v[320] * s.dn[241][4]);
        let eq22_e314_d_n5: f64 = (s.v[320] * s.dn[241][5]);
        let eq22_e314_d_n6: f64 = (s.v[320] * s.dn[241][6]);
        let eq22_e314_d_n7: f64 = (s.v[320] * s.dn[241][7]);
        let eq22_e314_d_n8: f64 = (s.v[320] * s.dn[241][8]);
        let eq22_e314_d_n9: f64 = (s.v[320] * s.dn[241][9]);
        let eq22_e314_d_n10: f64 = (s.v[320] * s.dn[241][10]);
        let eq22_e314_d_n11: f64 = (s.v[320] * s.dn[241][11]);
        let eq22_e314_d_b0: f64 = (s.v[320] * s.db[241][0]);
        let eq22_e314_d_b1: f64 = (s.v[320] * s.db[241][1]);
        let eq22_e315: f64 = (s.v[157] + eq22_e314);
        let eq22_e315_d_n0: f64 = (s.dn[157][0] + eq22_e314_d_n0);
        let eq22_e315_d_n1: f64 = (s.dn[157][1] + eq22_e314_d_n1);
        let eq22_e315_d_n2: f64 = (s.dn[157][2] + eq22_e314_d_n2);
        let eq22_e315_d_n3: f64 = (s.dn[157][3] + eq22_e314_d_n3);
        let eq22_e315_d_n4: f64 = (s.dn[157][4] + eq22_e314_d_n4);
        let eq22_e315_d_n5: f64 = (s.dn[157][5] + eq22_e314_d_n5);
        let eq22_e315_d_n6: f64 = (s.dn[157][6] + eq22_e314_d_n6);
        let eq22_e315_d_n7: f64 = (s.dn[157][7] + eq22_e314_d_n7);
        let eq22_e315_d_n8: f64 = (s.dn[157][8] + eq22_e314_d_n8);
        let eq22_e315_d_n9: f64 = (s.dn[157][9] + eq22_e314_d_n9);
        let eq22_e315_d_n10: f64 = (s.dn[157][10] + eq22_e314_d_n10);
        let eq22_e315_d_n11: f64 = (s.dn[157][11] + eq22_e314_d_n11);
        let eq22_e315_d_b0: f64 = (s.db[157][0] + eq22_e314_d_b0);
        let eq22_e315_d_b1: f64 = (s.db[157][1] + eq22_e314_d_b1);
        let eq22_e317: f64 = (eq22_e315 + s.v[160]);
        let eq22_e317_d_n0: f64 = (eq22_e315_d_n0 + s.dn[160][0]);
        let eq22_e317_d_n1: f64 = (eq22_e315_d_n1 + s.dn[160][1]);
        let eq22_e317_d_n2: f64 = (eq22_e315_d_n2 + s.dn[160][2]);
        let eq22_e317_d_n3: f64 = (eq22_e315_d_n3 + s.dn[160][3]);
        let eq22_e317_d_n4: f64 = (eq22_e315_d_n4 + s.dn[160][4]);
        let eq22_e317_d_n5: f64 = (eq22_e315_d_n5 + s.dn[160][5]);
        let eq22_e317_d_n6: f64 = (eq22_e315_d_n6 + s.dn[160][6]);
        let eq22_e317_d_n7: f64 = (eq22_e315_d_n7 + s.dn[160][7]);
        let eq22_e317_d_n8: f64 = (eq22_e315_d_n8 + s.dn[160][8]);
        let eq22_e317_d_n9: f64 = (eq22_e315_d_n9 + s.dn[160][9]);
        let eq22_e317_d_n10: f64 = (eq22_e315_d_n10 + s.dn[160][10]);
        let eq22_e317_d_n11: f64 = (eq22_e315_d_n11 + s.dn[160][11]);
        let eq22_e317_d_b0: f64 = (eq22_e315_d_b0 + s.db[160][0]);
        let eq22_e317_d_b1: f64 = (eq22_e315_d_b1 + s.db[160][1]);
        let eq22_e318: f64 = (p.p3 * eq22_e317);
        let eq22_e318_d_n0: f64 = (p.p3 * eq22_e317_d_n0);
        let eq22_e318_d_n1: f64 = (p.p3 * eq22_e317_d_n1);
        let eq22_e318_d_n2: f64 = (p.p3 * eq22_e317_d_n2);
        let eq22_e318_d_n3: f64 = (p.p3 * eq22_e317_d_n3);
        let eq22_e318_d_n4: f64 = (p.p3 * eq22_e317_d_n4);
        let eq22_e318_d_n5: f64 = (p.p3 * eq22_e317_d_n5);
        let eq22_e318_d_n6: f64 = (p.p3 * eq22_e317_d_n6);
        let eq22_e318_d_n7: f64 = (p.p3 * eq22_e317_d_n7);
        let eq22_e318_d_n8: f64 = (p.p3 * eq22_e317_d_n8);
        let eq22_e318_d_n9: f64 = (p.p3 * eq22_e317_d_n9);
        let eq22_e318_d_n10: f64 = (p.p3 * eq22_e317_d_n10);
        let eq22_e318_d_n11: f64 = (p.p3 * eq22_e317_d_n11);
        let eq22_e318_d_b0: f64 = (p.p3 * eq22_e317_d_b0);
        let eq22_e318_d_b1: f64 = (p.p3 * eq22_e317_d_b1);
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
        let eq22_e320_d_b0: f64 = (eq22_e318_d_b0 * p.p1);
        let eq22_e320_d_b1: f64 = (eq22_e318_d_b1 * p.p1);
        let eq22_value: f64 = eq22_e320;
        let eq22_node_derivatives: [f64; 12] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11];
        let eq22_branch_derivatives: [f64; 2] = [eq22_e320_d_b0, eq22_e320_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(10),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e324: f64 = (s.v[222] + s.v[235]);
        let eq23_e324_d_n0: f64 = (s.dn[222][0] + s.dn[235][0]);
        let eq23_e324_d_n1: f64 = (s.dn[222][1] + s.dn[235][1]);
        let eq23_e324_d_n2: f64 = (s.dn[222][2] + s.dn[235][2]);
        let eq23_e324_d_n3: f64 = (s.dn[222][3] + s.dn[235][3]);
        let eq23_e324_d_n4: f64 = (s.dn[222][4] + s.dn[235][4]);
        let eq23_e324_d_n5: f64 = (s.dn[222][5] + s.dn[235][5]);
        let eq23_e324_d_n6: f64 = (s.dn[222][6] + s.dn[235][6]);
        let eq23_e324_d_n7: f64 = (s.dn[222][7] + s.dn[235][7]);
        let eq23_e324_d_n8: f64 = (s.dn[222][8] + s.dn[235][8]);
        let eq23_e324_d_n9: f64 = (s.dn[222][9] + s.dn[235][9]);
        let eq23_e324_d_n10: f64 = (s.dn[222][10] + s.dn[235][10]);
        let eq23_e324_d_n11: f64 = (s.dn[222][11] + s.dn[235][11]);
        let eq23_e324_d_b0: f64 = (s.db[222][0] + s.db[235][0]);
        let eq23_e324_d_b1: f64 = (s.db[222][1] + s.db[235][1]);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n2: f64 = (p.p3 * eq23_e324_d_n2);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e325_d_n11: f64 = (p.p3 * eq23_e324_d_n11);
        let eq23_e325_d_b0: f64 = (p.p3 * eq23_e324_d_b0);
        let eq23_e325_d_b1: f64 = (p.p3 * eq23_e324_d_b1);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq23_e325);
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
        let eq23_e328_d_b0: f64 = (eq23_e326_d_b0 * p.p1);
        let eq23_e328_d_b1: f64 = (eq23_e326_d_b1 * p.p1);
        let eq23_value: f64 = eq23_e328;
        let eq23_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11];
        let eq23_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(10),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e338, eq24_e338_d_n0, eq24_e338_d_n1, eq24_e338_d_n2, eq24_e338_d_n3, eq24_e338_d_n4, eq24_e338_d_n5, eq24_e338_d_n6, eq24_e338_d_n7, eq24_e338_d_n8, eq24_e338_d_n9, eq24_e338_d_n10, eq24_e338_d_n11, eq24_e338_d_b0, eq24_e338_d_b1,) = {
    if s.b[567] {
        let eq24_e332: f64 = (p.p3 * s.v[243]);
        let eq24_e332_d_n0: f64 = (p.p3 * s.dn[243][0]);
        let eq24_e332_d_n1: f64 = (p.p3 * s.dn[243][1]);
        let eq24_e332_d_n2: f64 = (p.p3 * s.dn[243][2]);
        let eq24_e332_d_n3: f64 = (p.p3 * s.dn[243][3]);
        let eq24_e332_d_n4: f64 = (p.p3 * s.dn[243][4]);
        let eq24_e332_d_n5: f64 = (p.p3 * s.dn[243][5]);
        let eq24_e332_d_n6: f64 = (p.p3 * s.dn[243][6]);
        let eq24_e332_d_n7: f64 = (p.p3 * s.dn[243][7]);
        let eq24_e332_d_n8: f64 = (p.p3 * s.dn[243][8]);
        let eq24_e332_d_n9: f64 = (p.p3 * s.dn[243][9]);
        let eq24_e332_d_n10: f64 = (p.p3 * s.dn[243][10]);
        let eq24_e332_d_n11: f64 = (p.p3 * s.dn[243][11]);
        let eq24_e332_d_b0: f64 = (p.p3 * s.db[243][0]);
        let eq24_e332_d_b1: f64 = (p.p3 * s.db[243][1]);
        let eq24_e334: f64 = (eq24_e332 * s.v[105]);
        let eq24_e334_d_n0: f64 = ((eq24_e332_d_n0 * s.v[105]) + (eq24_e332 * s.dn[105][0]));
        let eq24_e334_d_n1: f64 = ((eq24_e332_d_n1 * s.v[105]) + (eq24_e332 * s.dn[105][1]));
        let eq24_e334_d_n2: f64 = ((eq24_e332_d_n2 * s.v[105]) + (eq24_e332 * s.dn[105][2]));
        let eq24_e334_d_n3: f64 = ((eq24_e332_d_n3 * s.v[105]) + (eq24_e332 * s.dn[105][3]));
        let eq24_e334_d_n4: f64 = ((eq24_e332_d_n4 * s.v[105]) + (eq24_e332 * s.dn[105][4]));
        let eq24_e334_d_n5: f64 = ((eq24_e332_d_n5 * s.v[105]) + (eq24_e332 * s.dn[105][5]));
        let eq24_e334_d_n6: f64 = ((eq24_e332_d_n6 * s.v[105]) + (eq24_e332 * s.dn[105][6]));
        let eq24_e334_d_n7: f64 = ((eq24_e332_d_n7 * s.v[105]) + (eq24_e332 * s.dn[105][7]));
        let eq24_e334_d_n8: f64 = ((eq24_e332_d_n8 * s.v[105]) + (eq24_e332 * s.dn[105][8]));
        let eq24_e334_d_n9: f64 = ((eq24_e332_d_n9 * s.v[105]) + (eq24_e332 * s.dn[105][9]));
        let eq24_e334_d_n10: f64 = ((eq24_e332_d_n10 * s.v[105]) + (eq24_e332 * s.dn[105][10]));
        let eq24_e334_d_n11: f64 = ((eq24_e332_d_n11 * s.v[105]) + (eq24_e332 * s.dn[105][11]));
        let eq24_e334_d_b0: f64 = ((eq24_e332_d_b0 * s.v[105]) + (eq24_e332 * s.db[105][0]));
        let eq24_e334_d_b1: f64 = ((eq24_e332_d_b1 * s.v[105]) + (eq24_e332 * s.db[105][1]));
        let eq24_e336: f64 = (eq24_e334 * p.p1);
        let eq24_e336_d_n0: f64 = (eq24_e334_d_n0 * p.p1);
        let eq24_e336_d_n1: f64 = (eq24_e334_d_n1 * p.p1);
        let eq24_e336_d_n2: f64 = (eq24_e334_d_n2 * p.p1);
        let eq24_e336_d_n3: f64 = (eq24_e334_d_n3 * p.p1);
        let eq24_e336_d_n4: f64 = (eq24_e334_d_n4 * p.p1);
        let eq24_e336_d_n5: f64 = (eq24_e334_d_n5 * p.p1);
        let eq24_e336_d_n6: f64 = (eq24_e334_d_n6 * p.p1);
        let eq24_e336_d_n7: f64 = (eq24_e334_d_n7 * p.p1);
        let eq24_e336_d_n8: f64 = (eq24_e334_d_n8 * p.p1);
        let eq24_e336_d_n9: f64 = (eq24_e334_d_n9 * p.p1);
        let eq24_e336_d_n10: f64 = (eq24_e334_d_n10 * p.p1);
        let eq24_e336_d_n11: f64 = (eq24_e334_d_n11 * p.p1);
        let eq24_e336_d_b0: f64 = (eq24_e334_d_b0 * p.p1);
        let eq24_e336_d_b1: f64 = (eq24_e334_d_b1 * p.p1);
        (eq24_e336, eq24_e336_d_n0, eq24_e336_d_n1, eq24_e336_d_n2, eq24_e336_d_n3, eq24_e336_d_n4, eq24_e336_d_n5, eq24_e336_d_n6, eq24_e336_d_n7, eq24_e336_d_n8, eq24_e336_d_n9, eq24_e336_d_n10, eq24_e336_d_n11, eq24_e336_d_b0, eq24_e336_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e338;
        let eq24_node_derivatives: [f64; 12] = [eq24_e338_d_n0, eq24_e338_d_n1, eq24_e338_d_n2, eq24_e338_d_n3, eq24_e338_d_n4, eq24_e338_d_n5, eq24_e338_d_n6, eq24_e338_d_n7, eq24_e338_d_n8, eq24_e338_d_n9, eq24_e338_d_n10, eq24_e338_d_n11];
        let eq24_branch_derivatives: [f64; 2] = [eq24_e338_d_b0, eq24_e338_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq25_e343,) = {
    if (!s.b[567]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e343;
        stamper.stamp_potential_const_local(
            0,
            eq25_value,
        );
        let (eq26_e353, eq26_e353_d_n0, eq26_e353_d_n1, eq26_e353_d_n2, eq26_e353_d_n3, eq26_e353_d_n4, eq26_e353_d_n5, eq26_e353_d_n6, eq26_e353_d_n7, eq26_e353_d_n8, eq26_e353_d_n9, eq26_e353_d_n10, eq26_e353_d_n11, eq26_e353_d_b0, eq26_e353_d_b1,) = {
    if s.b[568] {
        let eq26_e347: f64 = (p.p3 * s.v[244]);
        let eq26_e347_d_n0: f64 = (p.p3 * s.dn[244][0]);
        let eq26_e347_d_n1: f64 = (p.p3 * s.dn[244][1]);
        let eq26_e347_d_n2: f64 = (p.p3 * s.dn[244][2]);
        let eq26_e347_d_n3: f64 = (p.p3 * s.dn[244][3]);
        let eq26_e347_d_n4: f64 = (p.p3 * s.dn[244][4]);
        let eq26_e347_d_n5: f64 = (p.p3 * s.dn[244][5]);
        let eq26_e347_d_n6: f64 = (p.p3 * s.dn[244][6]);
        let eq26_e347_d_n7: f64 = (p.p3 * s.dn[244][7]);
        let eq26_e347_d_n8: f64 = (p.p3 * s.dn[244][8]);
        let eq26_e347_d_n9: f64 = (p.p3 * s.dn[244][9]);
        let eq26_e347_d_n10: f64 = (p.p3 * s.dn[244][10]);
        let eq26_e347_d_n11: f64 = (p.p3 * s.dn[244][11]);
        let eq26_e347_d_b0: f64 = (p.p3 * s.db[244][0]);
        let eq26_e347_d_b1: f64 = (p.p3 * s.db[244][1]);
        let eq26_e349: f64 = (eq26_e347 * s.v[106]);
        let eq26_e349_d_n0: f64 = ((eq26_e347_d_n0 * s.v[106]) + (eq26_e347 * s.dn[106][0]));
        let eq26_e349_d_n1: f64 = ((eq26_e347_d_n1 * s.v[106]) + (eq26_e347 * s.dn[106][1]));
        let eq26_e349_d_n2: f64 = ((eq26_e347_d_n2 * s.v[106]) + (eq26_e347 * s.dn[106][2]));
        let eq26_e349_d_n3: f64 = ((eq26_e347_d_n3 * s.v[106]) + (eq26_e347 * s.dn[106][3]));
        let eq26_e349_d_n4: f64 = ((eq26_e347_d_n4 * s.v[106]) + (eq26_e347 * s.dn[106][4]));
        let eq26_e349_d_n5: f64 = ((eq26_e347_d_n5 * s.v[106]) + (eq26_e347 * s.dn[106][5]));
        let eq26_e349_d_n6: f64 = ((eq26_e347_d_n6 * s.v[106]) + (eq26_e347 * s.dn[106][6]));
        let eq26_e349_d_n7: f64 = ((eq26_e347_d_n7 * s.v[106]) + (eq26_e347 * s.dn[106][7]));
        let eq26_e349_d_n8: f64 = ((eq26_e347_d_n8 * s.v[106]) + (eq26_e347 * s.dn[106][8]));
        let eq26_e349_d_n9: f64 = ((eq26_e347_d_n9 * s.v[106]) + (eq26_e347 * s.dn[106][9]));
        let eq26_e349_d_n10: f64 = ((eq26_e347_d_n10 * s.v[106]) + (eq26_e347 * s.dn[106][10]));
        let eq26_e349_d_n11: f64 = ((eq26_e347_d_n11 * s.v[106]) + (eq26_e347 * s.dn[106][11]));
        let eq26_e349_d_b0: f64 = ((eq26_e347_d_b0 * s.v[106]) + (eq26_e347 * s.db[106][0]));
        let eq26_e349_d_b1: f64 = ((eq26_e347_d_b1 * s.v[106]) + (eq26_e347 * s.db[106][1]));
        let eq26_e351: f64 = (eq26_e349 * p.p1);
        let eq26_e351_d_n0: f64 = (eq26_e349_d_n0 * p.p1);
        let eq26_e351_d_n1: f64 = (eq26_e349_d_n1 * p.p1);
        let eq26_e351_d_n2: f64 = (eq26_e349_d_n2 * p.p1);
        let eq26_e351_d_n3: f64 = (eq26_e349_d_n3 * p.p1);
        let eq26_e351_d_n4: f64 = (eq26_e349_d_n4 * p.p1);
        let eq26_e351_d_n5: f64 = (eq26_e349_d_n5 * p.p1);
        let eq26_e351_d_n6: f64 = (eq26_e349_d_n6 * p.p1);
        let eq26_e351_d_n7: f64 = (eq26_e349_d_n7 * p.p1);
        let eq26_e351_d_n8: f64 = (eq26_e349_d_n8 * p.p1);
        let eq26_e351_d_n9: f64 = (eq26_e349_d_n9 * p.p1);
        let eq26_e351_d_n10: f64 = (eq26_e349_d_n10 * p.p1);
        let eq26_e351_d_n11: f64 = (eq26_e349_d_n11 * p.p1);
        let eq26_e351_d_b0: f64 = (eq26_e349_d_b0 * p.p1);
        let eq26_e351_d_b1: f64 = (eq26_e349_d_b1 * p.p1);
        (eq26_e351, eq26_e351_d_n0, eq26_e351_d_n1, eq26_e351_d_n2, eq26_e351_d_n3, eq26_e351_d_n4, eq26_e351_d_n5, eq26_e351_d_n6, eq26_e351_d_n7, eq26_e351_d_n8, eq26_e351_d_n9, eq26_e351_d_n10, eq26_e351_d_n11, eq26_e351_d_b0, eq26_e351_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e353;
        let eq26_node_derivatives: [f64; 12] = [eq26_e353_d_n0, eq26_e353_d_n1, eq26_e353_d_n2, eq26_e353_d_n3, eq26_e353_d_n4, eq26_e353_d_n5, eq26_e353_d_n6, eq26_e353_d_n7, eq26_e353_d_n8, eq26_e353_d_n9, eq26_e353_d_n10, eq26_e353_d_n11];
        let eq26_branch_derivatives: [f64; 2] = [eq26_e353_d_b0, eq26_e353_d_b1];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e358,) = {
    if (!s.b[568]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e358;
        stamper.stamp_potential_const_local(
            1,
            eq27_value,
        );
        let eq28_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (eq28_value),
        );
        let eq29_value: f64 = (nv11 - 0.0);
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq29_value),
            11,
            multiplicity * (1.0),
        );
        let eq30_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, (nv11 - 0.0));
        let eq30_e368: f64 = (s.v[312] * eq30_e367);
        let eq30_e368_d_n0: f64 = (s.dn[312][0] * eq30_e367);
        let eq30_e368_d_n1: f64 = (s.dn[312][1] * eq30_e367);
        let eq30_e368_d_n2: f64 = (s.dn[312][2] * eq30_e367);
        let eq30_e368_d_n3: f64 = (s.dn[312][3] * eq30_e367);
        let eq30_e368_d_n4: f64 = (s.dn[312][4] * eq30_e367);
        let eq30_e368_d_n5: f64 = (s.dn[312][5] * eq30_e367);
        let eq30_e368_d_n6: f64 = (s.dn[312][6] * eq30_e367);
        let eq30_e368_d_n7: f64 = (s.dn[312][7] * eq30_e367);
        let eq30_e368_d_n8: f64 = (s.dn[312][8] * eq30_e367);
        let eq30_e368_d_n9: f64 = (s.dn[312][9] * eq30_e367);
        let eq30_e368_d_n10: f64 = (s.dn[312][10] * eq30_e367);
        let eq30_e368_d_n11: f64 = ((s.dn[312][11] * eq30_e367) + (s.v[312] * ddt_scale));
        let eq30_e368_d_b0: f64 = (s.db[312][0] * eq30_e367);
        let eq30_e368_d_b1: f64 = (s.db[312][1] * eq30_e367);
        let eq30_value: f64 = eq30_e368;
        let eq30_node_derivatives: [f64; 12] = [eq30_e368_d_n0, eq30_e368_d_n1, eq30_e368_d_n2, eq30_e368_d_n3, eq30_e368_d_n4, eq30_e368_d_n5, eq30_e368_d_n6, eq30_e368_d_n7, eq30_e368_d_n8, eq30_e368_d_n9, eq30_e368_d_n10, eq30_e368_d_n11];
        let eq30_branch_derivatives: [f64; 2] = [eq30_e368_d_b0, eq30_e368_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e371: f64 = (s.v[310] * (nv11 - 0.0));
        let eq31_e371_d_n0: f64 = (s.dn[310][0] * (nv11 - 0.0));
        let eq31_e371_d_n1: f64 = (s.dn[310][1] * (nv11 - 0.0));
        let eq31_e371_d_n2: f64 = (s.dn[310][2] * (nv11 - 0.0));
        let eq31_e371_d_n3: f64 = (s.dn[310][3] * (nv11 - 0.0));
        let eq31_e371_d_n4: f64 = (s.dn[310][4] * (nv11 - 0.0));
        let eq31_e371_d_n5: f64 = (s.dn[310][5] * (nv11 - 0.0));
        let eq31_e371_d_n6: f64 = (s.dn[310][6] * (nv11 - 0.0));
        let eq31_e371_d_n7: f64 = (s.dn[310][7] * (nv11 - 0.0));
        let eq31_e371_d_n8: f64 = (s.dn[310][8] * (nv11 - 0.0));
        let eq31_e371_d_n9: f64 = (s.dn[310][9] * (nv11 - 0.0));
        let eq31_e371_d_n10: f64 = (s.dn[310][10] * (nv11 - 0.0));
        let eq31_e371_d_n11: f64 = ((s.dn[310][11] * (nv11 - 0.0)) + s.v[310]);
        let eq31_e371_d_b0: f64 = (s.db[310][0] * (nv11 - 0.0));
        let eq31_e371_d_b1: f64 = (s.db[310][1] * (nv11 - 0.0));
        let eq31_value: f64 = eq31_e371;
        let eq31_node_derivatives: [f64; 12] = [eq31_e371_d_n0, eq31_e371_d_n1, eq31_e371_d_n2, eq31_e371_d_n3, eq31_e371_d_n4, eq31_e371_d_n5, eq31_e371_d_n6, eq31_e371_d_n7, eq31_e371_d_n8, eq31_e371_d_n9, eq31_e371_d_n10, eq31_e371_d_n11];
        let eq31_branch_derivatives: [f64; 2] = [eq31_e371_d_b0, eq31_e371_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_value: f64 = (nv11 - 0.0);
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (eq32_value),
            11,
            multiplicity * (1.0),
        );
        let eq33_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq33_value),
        );
        let eq34_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (eq34_value),
        );
        let eq35_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(2),
            Some(4),
            multiplicity * (eq35_value),
        );
        let eq36_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (eq36_value),
        );
        let eq37_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq37_value),
        );
        let eq38_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (eq38_value),
        );
        let eq39_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (eq39_value),
        );
        let eq40_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (eq40_value),
        );
        let eq41_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (eq41_value),
        );
        let eq42_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (eq42_value),
        );
        let eq43_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (eq43_value),
        );
        let eq44_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (eq44_value),
        );
        let eq45_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (eq45_value),
        );
        let eq46_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (eq46_value),
        );
        let (eq47_e455,) = {
    if s.b[579] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e455;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq47_value),
        );
        let (eq48_e464,) = {
    if (!s.b[579]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e464;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq48_value),
        );
        let (eq49_e474,) = {
    if (s.b[580] && s.b[581]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e474;
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (eq49_value),
        );
        let (eq50_e484,) = {
    if (s.b[580] && s.b[581]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e484;
        stamper.stamp_current_const_local(
            Some(9),
            Some(10),
            multiplicity * (eq50_value),
        );
        let (eq51_e494,) = {
    if (s.b[580] && s.b[581]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq51_value: f64 = eq51_e494;
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (eq51_value),
        );
        let (eq52_e505,) = {
    if (s.b[580] && (!s.b[581])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e505;
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (eq52_value),
        );
        let (eq53_e516,) = {
    if (s.b[580] && (!s.b[581])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e516;
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (eq53_value),
        );
        let (eq54_e527,) = {
    if ((!s.b[580]) && s.b[582]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e527;
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (eq54_value),
        );
        let (eq55_e538,) = {
    if ((!s.b[580]) && s.b[582]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e538;
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (eq55_value),
        );
        let (eq56_e550,) = {
    if ((!s.b[580]) && (!s.b[582])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e550;
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (eq56_value),
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
        let eq11_e235_q: f64 = s.rv[209];
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (s.rdn[209][3]),
        );
        let eq13_e245: f64 = (s.v[210] + s.v[215]);
        let eq13_e245_d_n0: f64 = (s.dn[210][0] + s.dn[215][0]);
        let eq13_e245_d_n1: f64 = (s.dn[210][1] + s.dn[215][1]);
        let eq13_e245_d_n2: f64 = (s.dn[210][2] + s.dn[215][2]);
        let eq13_e245_d_n3: f64 = (s.dn[210][3] + s.dn[215][3]);
        let eq13_e245_d_n4: f64 = (s.dn[210][4] + s.dn[215][4]);
        let eq13_e245_d_n5: f64 = (s.dn[210][5] + s.dn[215][5]);
        let eq13_e245_d_n6: f64 = (s.dn[210][6] + s.dn[215][6]);
        let eq13_e245_d_n7: f64 = (s.dn[210][7] + s.dn[215][7]);
        let eq13_e245_d_n8: f64 = (s.dn[210][8] + s.dn[215][8]);
        let eq13_e245_d_n9: f64 = (s.dn[210][9] + s.dn[215][9]);
        let eq13_e245_d_n10: f64 = (s.dn[210][10] + s.dn[215][10]);
        let eq13_e245_d_n11: f64 = (s.dn[210][11] + s.dn[215][11]);
        let eq13_e245_d_b0: f64 = (s.db[210][0] + s.db[215][0]);
        let eq13_e245_d_b1: f64 = (s.db[210][1] + s.db[215][1]);
        let eq13_e247: f64 = (eq13_e245 + s.v[227]);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + s.dn[227][0]);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + s.dn[227][1]);
        let eq13_e247_d_n2: f64 = (eq13_e245_d_n2 + s.dn[227][2]);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + s.dn[227][3]);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + s.dn[227][4]);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + s.dn[227][5]);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + s.dn[227][6]);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + s.dn[227][7]);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + s.dn[227][8]);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + s.dn[227][9]);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + s.dn[227][10]);
        let eq13_e247_d_n11: f64 = (eq13_e245_d_n11 + s.dn[227][11]);
        let eq13_e247_d_b0: f64 = (eq13_e245_d_b0 + s.db[227][0]);
        let eq13_e247_d_b1: f64 = (eq13_e245_d_b1 + s.db[227][1]);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n2: f64 = (p.p3 * eq13_e247_d_n2);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e248_d_n11: f64 = (p.p3 * eq13_e247_d_n11);
        let eq13_e248_d_b0: f64 = (p.p3 * eq13_e247_d_b0);
        let eq13_e248_d_b1: f64 = (p.p3 * eq13_e247_d_b1);
        let eq13_e249_q: f64 = eq13_e248;
        let eq13_e251: f64 = (eq13_e248 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_d_n2: f64 = (eq13_e248_d_n2 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_d_n11: f64 = (eq13_e248_d_n11 * p.p1);
        let eq13_e251_d_b0: f64 = (eq13_e248_d_b0 * p.p1);
        let eq13_e251_d_b1: f64 = (eq13_e248_d_b1 * p.p1);
        let eq13_e251_q: f64 = (eq13_e249_q * p.p1);
        let eq13_e251_q_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_q_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_q_d_n2: f64 = (eq13_e248_d_n2 * p.p1);
        let eq13_e251_q_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_q_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_q_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_q_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_q_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_q_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_q_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_q_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_q_d_n11: f64 = (eq13_e248_d_n11 * p.p1);
        let eq13_e251_q_d_b0: f64 = (eq13_e248_d_b0 * p.p1);
        let eq13_e251_q_d_b1: f64 = (eq13_e248_d_b1 * p.p1);
        let eq13_reactive_node_derivatives: [f64; 12] = [eq13_e251_q_d_n0, eq13_e251_q_d_n1, eq13_e251_q_d_n2, eq13_e251_q_d_n3, eq13_e251_q_d_n4, eq13_e251_q_d_n5, eq13_e251_q_d_n6, eq13_e251_q_d_n7, eq13_e251_q_d_n8, eq13_e251_q_d_n9, eq13_e251_q_d_n10, eq13_e251_q_d_n11];
        let eq13_reactive_branch_derivatives: [f64; 2] = [eq13_e251_q_d_b0, eq13_e251_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * s.v[212]);
        let eq14_e254_d_n0: f64 = (p.p3 * s.dn[212][0]);
        let eq14_e254_d_n1: f64 = (p.p3 * s.dn[212][1]);
        let eq14_e254_d_n2: f64 = (p.p3 * s.dn[212][2]);
        let eq14_e254_d_n3: f64 = (p.p3 * s.dn[212][3]);
        let eq14_e254_d_n4: f64 = (p.p3 * s.dn[212][4]);
        let eq14_e254_d_n5: f64 = (p.p3 * s.dn[212][5]);
        let eq14_e254_d_n6: f64 = (p.p3 * s.dn[212][6]);
        let eq14_e254_d_n7: f64 = (p.p3 * s.dn[212][7]);
        let eq14_e254_d_n8: f64 = (p.p3 * s.dn[212][8]);
        let eq14_e254_d_n9: f64 = (p.p3 * s.dn[212][9]);
        let eq14_e254_d_n10: f64 = (p.p3 * s.dn[212][10]);
        let eq14_e254_d_n11: f64 = (p.p3 * s.dn[212][11]);
        let eq14_e254_d_b0: f64 = (p.p3 * s.db[212][0]);
        let eq14_e254_d_b1: f64 = (p.p3 * s.db[212][1]);
        let eq14_e255_q: f64 = eq14_e254;
        let eq14_e257: f64 = (eq14_e254 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_d_n2: f64 = (eq14_e254_d_n2 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_d_n11: f64 = (eq14_e254_d_n11 * p.p1);
        let eq14_e257_d_b0: f64 = (eq14_e254_d_b0 * p.p1);
        let eq14_e257_d_b1: f64 = (eq14_e254_d_b1 * p.p1);
        let eq14_e257_q: f64 = (eq14_e255_q * p.p1);
        let eq14_e257_q_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_q_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_q_d_n2: f64 = (eq14_e254_d_n2 * p.p1);
        let eq14_e257_q_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_q_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_q_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_q_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_q_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_q_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_q_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_q_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_q_d_n11: f64 = (eq14_e254_d_n11 * p.p1);
        let eq14_e257_q_d_b0: f64 = (eq14_e254_d_b0 * p.p1);
        let eq14_e257_q_d_b1: f64 = (eq14_e254_d_b1 * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e257_q_d_n0, eq14_e257_q_d_n1, eq14_e257_q_d_n2, eq14_e257_q_d_n3, eq14_e257_q_d_n4, eq14_e257_q_d_n5, eq14_e257_q_d_n6, eq14_e257_q_d_n7, eq14_e257_q_d_n8, eq14_e257_q_d_n9, eq14_e257_q_d_n10, eq14_e257_q_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e257_q_d_b0, eq14_e257_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (s.v[213] + s.v[216]);
        let eq15_e261_d_n0: f64 = (s.dn[213][0] + s.dn[216][0]);
        let eq15_e261_d_n1: f64 = (s.dn[213][1] + s.dn[216][1]);
        let eq15_e261_d_n2: f64 = (s.dn[213][2] + s.dn[216][2]);
        let eq15_e261_d_n3: f64 = (s.dn[213][3] + s.dn[216][3]);
        let eq15_e261_d_n4: f64 = (s.dn[213][4] + s.dn[216][4]);
        let eq15_e261_d_n5: f64 = (s.dn[213][5] + s.dn[216][5]);
        let eq15_e261_d_n6: f64 = (s.dn[213][6] + s.dn[216][6]);
        let eq15_e261_d_n7: f64 = (s.dn[213][7] + s.dn[216][7]);
        let eq15_e261_d_n8: f64 = (s.dn[213][8] + s.dn[216][8]);
        let eq15_e261_d_n9: f64 = (s.dn[213][9] + s.dn[216][9]);
        let eq15_e261_d_n10: f64 = (s.dn[213][10] + s.dn[216][10]);
        let eq15_e261_d_n11: f64 = (s.dn[213][11] + s.dn[216][11]);
        let eq15_e261_d_b0: f64 = (s.db[213][0] + s.db[216][0]);
        let eq15_e261_d_b1: f64 = (s.db[213][1] + s.db[216][1]);
        let eq15_e263: f64 = (eq15_e261 + s.v[230]);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + s.dn[230][0]);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + s.dn[230][1]);
        let eq15_e263_d_n2: f64 = (eq15_e261_d_n2 + s.dn[230][2]);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + s.dn[230][3]);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + s.dn[230][4]);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + s.dn[230][5]);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + s.dn[230][6]);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + s.dn[230][7]);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + s.dn[230][8]);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + s.dn[230][9]);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + s.dn[230][10]);
        let eq15_e263_d_n11: f64 = (eq15_e261_d_n11 + s.dn[230][11]);
        let eq15_e263_d_b0: f64 = (eq15_e261_d_b0 + s.db[230][0]);
        let eq15_e263_d_b1: f64 = (eq15_e261_d_b1 + s.db[230][1]);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n2: f64 = (p.p3 * eq15_e263_d_n2);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e264_d_n11: f64 = (p.p3 * eq15_e263_d_n11);
        let eq15_e264_d_b0: f64 = (p.p3 * eq15_e263_d_b0);
        let eq15_e264_d_b1: f64 = (p.p3 * eq15_e263_d_b1);
        let eq15_e265_q: f64 = eq15_e264;
        let eq15_e267: f64 = (eq15_e264 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_d_n2: f64 = (eq15_e264_d_n2 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_d_n11: f64 = (eq15_e264_d_n11 * p.p1);
        let eq15_e267_d_b0: f64 = (eq15_e264_d_b0 * p.p1);
        let eq15_e267_d_b1: f64 = (eq15_e264_d_b1 * p.p1);
        let eq15_e267_q: f64 = (eq15_e265_q * p.p1);
        let eq15_e267_q_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_q_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_q_d_n2: f64 = (eq15_e264_d_n2 * p.p1);
        let eq15_e267_q_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_q_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_q_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_q_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_q_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_q_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_q_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_q_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_q_d_n11: f64 = (eq15_e264_d_n11 * p.p1);
        let eq15_e267_q_d_b0: f64 = (eq15_e264_d_b0 * p.p1);
        let eq15_e267_q_d_b1: f64 = (eq15_e264_d_b1 * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e267_q_d_n0, eq15_e267_q_d_n1, eq15_e267_q_d_n2, eq15_e267_q_d_n3, eq15_e267_q_d_n4, eq15_e267_q_d_n5, eq15_e267_q_d_n6, eq15_e267_q_d_n7, eq15_e267_q_d_n8, eq15_e267_q_d_n9, eq15_e267_q_d_n10, eq15_e267_q_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e267_q_d_b0, eq15_e267_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * s.v[217]);
        let eq16_e270_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq16_e270_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq16_e270_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq16_e270_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq16_e270_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq16_e270_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq16_e270_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq16_e270_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq16_e270_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq16_e270_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq16_e270_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq16_e270_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq16_e270_d_b0: f64 = (p.p3 * s.db[217][0]);
        let eq16_e270_d_b1: f64 = (p.p3 * s.db[217][1]);
        let eq16_e271_q: f64 = eq16_e270;
        let eq16_e273: f64 = (eq16_e270 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_d_n2: f64 = (eq16_e270_d_n2 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_d_n11: f64 = (eq16_e270_d_n11 * p.p1);
        let eq16_e273_d_b0: f64 = (eq16_e270_d_b0 * p.p1);
        let eq16_e273_d_b1: f64 = (eq16_e270_d_b1 * p.p1);
        let eq16_e273_q: f64 = (eq16_e271_q * p.p1);
        let eq16_e273_q_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_q_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_q_d_n2: f64 = (eq16_e270_d_n2 * p.p1);
        let eq16_e273_q_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_q_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_q_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_q_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_q_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_q_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_q_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_q_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_q_d_n11: f64 = (eq16_e270_d_n11 * p.p1);
        let eq16_e273_q_d_b0: f64 = (eq16_e270_d_b0 * p.p1);
        let eq16_e273_q_d_b1: f64 = (eq16_e270_d_b1 * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e273_q_d_n0, eq16_e273_q_d_n1, eq16_e273_q_d_n2, eq16_e273_q_d_n3, eq16_e273_q_d_n4, eq16_e273_q_d_n5, eq16_e273_q_d_n6, eq16_e273_q_d_n7, eq16_e273_q_d_n8, eq16_e273_q_d_n9, eq16_e273_q_d_n10, eq16_e273_q_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 2] = [eq16_e273_q_d_b0, eq16_e273_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * s.v[249]);
        let eq17_e278_d_n0: f64 = (eq17_e276 * s.dn[249][0]);
        let eq17_e278_d_n1: f64 = (eq17_e276 * s.dn[249][1]);
        let eq17_e278_d_n2: f64 = (eq17_e276 * s.dn[249][2]);
        let eq17_e278_d_n3: f64 = (eq17_e276 * s.dn[249][3]);
        let eq17_e278_d_n4: f64 = (eq17_e276 * s.dn[249][4]);
        let eq17_e278_d_n5: f64 = (eq17_e276 * s.dn[249][5]);
        let eq17_e278_d_n6: f64 = (eq17_e276 * s.dn[249][6]);
        let eq17_e278_d_n7: f64 = (eq17_e276 * s.dn[249][7]);
        let eq17_e278_d_n8: f64 = (eq17_e276 * s.dn[249][8]);
        let eq17_e278_d_n9: f64 = (eq17_e276 * s.dn[249][9]);
        let eq17_e278_d_n10: f64 = (eq17_e276 * s.dn[249][10]);
        let eq17_e278_d_n11: f64 = (eq17_e276 * s.dn[249][11]);
        let eq17_e278_d_b0: f64 = (eq17_e276 * s.db[249][0]);
        let eq17_e278_d_b1: f64 = (eq17_e276 * s.db[249][1]);
        let eq17_e279_q: f64 = eq17_e278;
        let eq17_e281: f64 = (eq17_e278 * p.p1);
        let eq17_e281_d_n0: f64 = (eq17_e278_d_n0 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_d_n3: f64 = (eq17_e278_d_n3 * p.p1);
        let eq17_e281_d_n4: f64 = (eq17_e278_d_n4 * p.p1);
        let eq17_e281_d_n5: f64 = (eq17_e278_d_n5 * p.p1);
        let eq17_e281_d_n6: f64 = (eq17_e278_d_n6 * p.p1);
        let eq17_e281_d_n7: f64 = (eq17_e278_d_n7 * p.p1);
        let eq17_e281_d_n8: f64 = (eq17_e278_d_n8 * p.p1);
        let eq17_e281_d_n9: f64 = (eq17_e278_d_n9 * p.p1);
        let eq17_e281_d_n10: f64 = (eq17_e278_d_n10 * p.p1);
        let eq17_e281_d_n11: f64 = (eq17_e278_d_n11 * p.p1);
        let eq17_e281_d_b0: f64 = (eq17_e278_d_b0 * p.p1);
        let eq17_e281_d_b1: f64 = (eq17_e278_d_b1 * p.p1);
        let eq17_e281_q: f64 = (eq17_e279_q * p.p1);
        let eq17_e281_q_d_n0: f64 = (eq17_e278_d_n0 * p.p1);
        let eq17_e281_q_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_q_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_q_d_n3: f64 = (eq17_e278_d_n3 * p.p1);
        let eq17_e281_q_d_n4: f64 = (eq17_e278_d_n4 * p.p1);
        let eq17_e281_q_d_n5: f64 = (eq17_e278_d_n5 * p.p1);
        let eq17_e281_q_d_n6: f64 = (eq17_e278_d_n6 * p.p1);
        let eq17_e281_q_d_n7: f64 = (eq17_e278_d_n7 * p.p1);
        let eq17_e281_q_d_n8: f64 = (eq17_e278_d_n8 * p.p1);
        let eq17_e281_q_d_n9: f64 = (eq17_e278_d_n9 * p.p1);
        let eq17_e281_q_d_n10: f64 = (eq17_e278_d_n10 * p.p1);
        let eq17_e281_q_d_n11: f64 = (eq17_e278_d_n11 * p.p1);
        let eq17_e281_q_d_b0: f64 = (eq17_e278_d_b0 * p.p1);
        let eq17_e281_q_d_b1: f64 = (eq17_e278_d_b1 * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e281_q_d_n0, eq17_e281_q_d_n1, eq17_e281_q_d_n2, eq17_e281_q_d_n3, eq17_e281_q_d_n4, eq17_e281_q_d_n5, eq17_e281_q_d_n6, eq17_e281_q_d_n7, eq17_e281_q_d_n8, eq17_e281_q_d_n9, eq17_e281_q_d_n10, eq17_e281_q_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e281_q_d_b0, eq17_e281_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * s.v[250]);
        let eq18_e286_d_n0: f64 = (eq18_e284 * s.dn[250][0]);
        let eq18_e286_d_n1: f64 = (eq18_e284 * s.dn[250][1]);
        let eq18_e286_d_n2: f64 = (eq18_e284 * s.dn[250][2]);
        let eq18_e286_d_n3: f64 = (eq18_e284 * s.dn[250][3]);
        let eq18_e286_d_n4: f64 = (eq18_e284 * s.dn[250][4]);
        let eq18_e286_d_n5: f64 = (eq18_e284 * s.dn[250][5]);
        let eq18_e286_d_n6: f64 = (eq18_e284 * s.dn[250][6]);
        let eq18_e286_d_n7: f64 = (eq18_e284 * s.dn[250][7]);
        let eq18_e286_d_n8: f64 = (eq18_e284 * s.dn[250][8]);
        let eq18_e286_d_n9: f64 = (eq18_e284 * s.dn[250][9]);
        let eq18_e286_d_n10: f64 = (eq18_e284 * s.dn[250][10]);
        let eq18_e286_d_n11: f64 = (eq18_e284 * s.dn[250][11]);
        let eq18_e286_d_b0: f64 = (eq18_e284 * s.db[250][0]);
        let eq18_e286_d_b1: f64 = (eq18_e284 * s.db[250][1]);
        let eq18_e287_q: f64 = eq18_e286;
        let eq18_e289: f64 = (eq18_e286 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_d_n2: f64 = (eq18_e286_d_n2 * p.p1);
        let eq18_e289_d_n3: f64 = (eq18_e286_d_n3 * p.p1);
        let eq18_e289_d_n4: f64 = (eq18_e286_d_n4 * p.p1);
        let eq18_e289_d_n5: f64 = (eq18_e286_d_n5 * p.p1);
        let eq18_e289_d_n6: f64 = (eq18_e286_d_n6 * p.p1);
        let eq18_e289_d_n7: f64 = (eq18_e286_d_n7 * p.p1);
        let eq18_e289_d_n8: f64 = (eq18_e286_d_n8 * p.p1);
        let eq18_e289_d_n9: f64 = (eq18_e286_d_n9 * p.p1);
        let eq18_e289_d_n10: f64 = (eq18_e286_d_n10 * p.p1);
        let eq18_e289_d_n11: f64 = (eq18_e286_d_n11 * p.p1);
        let eq18_e289_d_b0: f64 = (eq18_e286_d_b0 * p.p1);
        let eq18_e289_d_b1: f64 = (eq18_e286_d_b1 * p.p1);
        let eq18_e289_q: f64 = (eq18_e287_q * p.p1);
        let eq18_e289_q_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_q_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_q_d_n2: f64 = (eq18_e286_d_n2 * p.p1);
        let eq18_e289_q_d_n3: f64 = (eq18_e286_d_n3 * p.p1);
        let eq18_e289_q_d_n4: f64 = (eq18_e286_d_n4 * p.p1);
        let eq18_e289_q_d_n5: f64 = (eq18_e286_d_n5 * p.p1);
        let eq18_e289_q_d_n6: f64 = (eq18_e286_d_n6 * p.p1);
        let eq18_e289_q_d_n7: f64 = (eq18_e286_d_n7 * p.p1);
        let eq18_e289_q_d_n8: f64 = (eq18_e286_d_n8 * p.p1);
        let eq18_e289_q_d_n9: f64 = (eq18_e286_d_n9 * p.p1);
        let eq18_e289_q_d_n10: f64 = (eq18_e286_d_n10 * p.p1);
        let eq18_e289_q_d_n11: f64 = (eq18_e286_d_n11 * p.p1);
        let eq18_e289_q_d_b0: f64 = (eq18_e286_d_b0 * p.p1);
        let eq18_e289_q_d_b1: f64 = (eq18_e286_d_b1 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e289_q_d_n0, eq18_e289_q_d_n1, eq18_e289_q_d_n2, eq18_e289_q_d_n3, eq18_e289_q_d_n4, eq18_e289_q_d_n5, eq18_e289_q_d_n6, eq18_e289_q_d_n7, eq18_e289_q_d_n8, eq18_e289_q_d_n9, eq18_e289_q_d_n10, eq18_e289_q_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e289_q_d_b0, eq18_e289_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e305: f64 = (s.v[225] + s.v[234]);
        let eq21_e305_d_n0: f64 = (s.dn[225][0] + s.dn[234][0]);
        let eq21_e305_d_n1: f64 = (s.dn[225][1] + s.dn[234][1]);
        let eq21_e305_d_n2: f64 = (s.dn[225][2] + s.dn[234][2]);
        let eq21_e305_d_n3: f64 = (s.dn[225][3] + s.dn[234][3]);
        let eq21_e305_d_n4: f64 = (s.dn[225][4] + s.dn[234][4]);
        let eq21_e305_d_n5: f64 = (s.dn[225][5] + s.dn[234][5]);
        let eq21_e305_d_n6: f64 = (s.dn[225][6] + s.dn[234][6]);
        let eq21_e305_d_n7: f64 = (s.dn[225][7] + s.dn[234][7]);
        let eq21_e305_d_n8: f64 = (s.dn[225][8] + s.dn[234][8]);
        let eq21_e305_d_n9: f64 = (s.dn[225][9] + s.dn[234][9]);
        let eq21_e305_d_n10: f64 = (s.dn[225][10] + s.dn[234][10]);
        let eq21_e305_d_n11: f64 = (s.dn[225][11] + s.dn[234][11]);
        let eq21_e305_d_b0: f64 = (s.db[225][0] + s.db[234][0]);
        let eq21_e305_d_b1: f64 = (s.db[225][1] + s.db[234][1]);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n2: f64 = (p.p3 * eq21_e305_d_n2);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e306_d_n11: f64 = (p.p3 * eq21_e305_d_n11);
        let eq21_e306_d_b0: f64 = (p.p3 * eq21_e305_d_b0);
        let eq21_e306_d_b1: f64 = (p.p3 * eq21_e305_d_b1);
        let eq21_e307_q: f64 = eq21_e306;
        let eq21_e309: f64 = (eq21_e306 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_d_n2: f64 = (eq21_e306_d_n2 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_d_n11: f64 = (eq21_e306_d_n11 * p.p1);
        let eq21_e309_d_b0: f64 = (eq21_e306_d_b0 * p.p1);
        let eq21_e309_d_b1: f64 = (eq21_e306_d_b1 * p.p1);
        let eq21_e309_q: f64 = (eq21_e307_q * p.p1);
        let eq21_e309_q_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_q_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_q_d_n2: f64 = (eq21_e306_d_n2 * p.p1);
        let eq21_e309_q_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_q_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_q_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_q_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_q_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_q_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_q_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_q_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_q_d_n11: f64 = (eq21_e306_d_n11 * p.p1);
        let eq21_e309_q_d_b0: f64 = (eq21_e306_d_b0 * p.p1);
        let eq21_e309_q_d_b1: f64 = (eq21_e306_d_b1 * p.p1);
        let eq21_reactive_node_derivatives: [f64; 12] = [eq21_e309_q_d_n0, eq21_e309_q_d_n1, eq21_e309_q_d_n2, eq21_e309_q_d_n3, eq21_e309_q_d_n4, eq21_e309_q_d_n5, eq21_e309_q_d_n6, eq21_e309_q_d_n7, eq21_e309_q_d_n8, eq21_e309_q_d_n9, eq21_e309_q_d_n10, eq21_e309_q_d_n11];
        let eq21_reactive_branch_derivatives: [f64; 2] = [eq21_e309_q_d_b0, eq21_e309_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq23_e324: f64 = (s.v[222] + s.v[235]);
        let eq23_e324_d_n0: f64 = (s.dn[222][0] + s.dn[235][0]);
        let eq23_e324_d_n1: f64 = (s.dn[222][1] + s.dn[235][1]);
        let eq23_e324_d_n2: f64 = (s.dn[222][2] + s.dn[235][2]);
        let eq23_e324_d_n3: f64 = (s.dn[222][3] + s.dn[235][3]);
        let eq23_e324_d_n4: f64 = (s.dn[222][4] + s.dn[235][4]);
        let eq23_e324_d_n5: f64 = (s.dn[222][5] + s.dn[235][5]);
        let eq23_e324_d_n6: f64 = (s.dn[222][6] + s.dn[235][6]);
        let eq23_e324_d_n7: f64 = (s.dn[222][7] + s.dn[235][7]);
        let eq23_e324_d_n8: f64 = (s.dn[222][8] + s.dn[235][8]);
        let eq23_e324_d_n9: f64 = (s.dn[222][9] + s.dn[235][9]);
        let eq23_e324_d_n10: f64 = (s.dn[222][10] + s.dn[235][10]);
        let eq23_e324_d_n11: f64 = (s.dn[222][11] + s.dn[235][11]);
        let eq23_e324_d_b0: f64 = (s.db[222][0] + s.db[235][0]);
        let eq23_e324_d_b1: f64 = (s.db[222][1] + s.db[235][1]);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n2: f64 = (p.p3 * eq23_e324_d_n2);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e325_d_n11: f64 = (p.p3 * eq23_e324_d_n11);
        let eq23_e325_d_b0: f64 = (p.p3 * eq23_e324_d_b0);
        let eq23_e325_d_b1: f64 = (p.p3 * eq23_e324_d_b1);
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
        let eq23_e328_q_d_b0: f64 = (eq23_e325_d_b0 * p.p1);
        let eq23_e328_q_d_b1: f64 = (eq23_e325_d_b1 * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e328_q_d_n0, eq23_e328_q_d_n1, eq23_e328_q_d_n2, eq23_e328_q_d_n3, eq23_e328_q_d_n4, eq23_e328_q_d_n5, eq23_e328_q_d_n6, eq23_e328_q_d_n7, eq23_e328_q_d_n8, eq23_e328_q_d_n9, eq23_e328_q_d_n10, eq23_e328_q_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e328_q_d_b0, eq23_e328_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e367_q: f64 = (nv11 - 0.0);
        let eq30_e368: f64 = (s.v[312] * (nv11 - 0.0));
        let eq30_e368_d_n0: f64 = (s.dn[312][0] * (nv11 - 0.0));
        let eq30_e368_d_n1: f64 = (s.dn[312][1] * (nv11 - 0.0));
        let eq30_e368_d_n2: f64 = (s.dn[312][2] * (nv11 - 0.0));
        let eq30_e368_d_n3: f64 = (s.dn[312][3] * (nv11 - 0.0));
        let eq30_e368_d_n4: f64 = (s.dn[312][4] * (nv11 - 0.0));
        let eq30_e368_d_n5: f64 = (s.dn[312][5] * (nv11 - 0.0));
        let eq30_e368_d_n6: f64 = (s.dn[312][6] * (nv11 - 0.0));
        let eq30_e368_d_n7: f64 = (s.dn[312][7] * (nv11 - 0.0));
        let eq30_e368_d_n8: f64 = (s.dn[312][8] * (nv11 - 0.0));
        let eq30_e368_d_n9: f64 = (s.dn[312][9] * (nv11 - 0.0));
        let eq30_e368_d_n10: f64 = (s.dn[312][10] * (nv11 - 0.0));
        let eq30_e368_d_n11: f64 = ((s.dn[312][11] * (nv11 - 0.0)) + s.v[312]);
        let eq30_e368_d_b0: f64 = (s.db[312][0] * (nv11 - 0.0));
        let eq30_e368_d_b1: f64 = (s.db[312][1] * (nv11 - 0.0));
        let eq30_e368_q: f64 = (s.v[312] * eq30_e367_q);
        let eq30_e368_q_d_n0: f64 = (s.dn[312][0] * eq30_e367_q);
        let eq30_e368_q_d_n1: f64 = (s.dn[312][1] * eq30_e367_q);
        let eq30_e368_q_d_n2: f64 = (s.dn[312][2] * eq30_e367_q);
        let eq30_e368_q_d_n3: f64 = (s.dn[312][3] * eq30_e367_q);
        let eq30_e368_q_d_n4: f64 = (s.dn[312][4] * eq30_e367_q);
        let eq30_e368_q_d_n5: f64 = (s.dn[312][5] * eq30_e367_q);
        let eq30_e368_q_d_n6: f64 = (s.dn[312][6] * eq30_e367_q);
        let eq30_e368_q_d_n7: f64 = (s.dn[312][7] * eq30_e367_q);
        let eq30_e368_q_d_n8: f64 = (s.dn[312][8] * eq30_e367_q);
        let eq30_e368_q_d_n9: f64 = (s.dn[312][9] * eq30_e367_q);
        let eq30_e368_q_d_n10: f64 = (s.dn[312][10] * eq30_e367_q);
        let eq30_e368_q_d_n11: f64 = ((s.dn[312][11] * eq30_e367_q) + s.v[312]);
        let eq30_e368_q_d_b0: f64 = (s.db[312][0] * eq30_e367_q);
        let eq30_e368_q_d_b1: f64 = (s.db[312][1] * eq30_e367_q);
        let eq30_reactive_node_derivatives: [f64; 12] = [eq30_e368_q_d_n0, eq30_e368_q_d_n1, eq30_e368_q_d_n2, eq30_e368_q_d_n3, eq30_e368_q_d_n4, eq30_e368_q_d_n5, eq30_e368_q_d_n6, eq30_e368_q_d_n7, eq30_e368_q_d_n8, eq30_e368_q_d_n9, eq30_e368_q_d_n10, eq30_e368_q_d_n11];
        let eq30_reactive_branch_derivatives: [f64; 2] = [eq30_e368_q_d_b0, eq30_e368_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
