#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        s.b[439] = (p.p3 == 1.0);
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if s.b[439] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[439]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[150] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.v[314] = 0.0;

        s.b[440] = (p.p137 == 0.0);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if s.b[440] {
            s.store_scalar(315, 1e-12);
        }

        if (!s.b[440]) {
            s.store_scalar(315, p.p137);
        }

        s.store_scale(316, 315, p.p1);

        s.store_div_from_scalar(317, 1.0, 316);

        s.v[52] = 0.001;

        s.v[312] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[63] = (1.0 / s.v[62]);

        s.v[259] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.b[441] = ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if s.b[441] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[441]) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[64] = (1.0 / p.p65);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[89] = (1.0 / s.v[79]);

        s.v[259] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.b[442] = ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if s.b[442] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[442]) {
            s.store_scalar(88, ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p116;

        s.v[86] = (1.0 / s.v[87]);

        s.v[66] = (1.0 / s.v[75]);

        s.v[318] = (1.0 - (1.0 / p.p82));

        s.v[151] = 0.0;

        s.v[152] = 0.0;

        s.v[169] = 0.0;

        s.v[168] = 1.0;

        s.v[196] = 0.0;

        s.v[198] = 0.0;

        s.v[228] = 0.0;

        s.v[211] = 0.0;

        s.v[42] = 0.0;

        s.v[44] = 0.0;

        s.v[53] = 0.0;

        s.v[54] = 0.0;

        s.v[45] = 0.0;

        s.v[11] = 0.0;

        s.v[2] = (s.v[5] + s.v[11]);

        s.v[4] = (s.v[2] / s.v[3]);

        s.v[6] = (8.617086918058125e-5 * s.v[2]);

        s.v[7] = (8.617086918058125e-5 * s.v[3]);

        s.v[8] = (1.0 / s.v[6]);

        s.v[9] = (1.0 / s.v[7]);

        s.v[10] = (s.v[8] - s.v[9]);

        s.v[12] = (s.v[2] - s.v[3]);

        s.v[254] = ((s.v[4]) as f64).ln();

        s.store_scaled_offset(259, 74, (((-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))) + ((-0.05))), 10.0);

        s.b[443] = ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05);
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if s.b[443] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(259)), 0.1, 0.05);
        }

        if (!s.b[443]) {
            s.store_ad_value(70, A::add_scaled_inputs(A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.1));
        }

        s.store_scaled_offset(259, 88, (((-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))) + ((-0.05))), 10.0);

        s.b[444] = ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if s.b[444] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(259)), 0.1, 0.05);
        }

        if (!s.b[444]) {
            s.store_ad_value(85, A::add_scaled_inputs(A::offset(s.ad_value(88), (-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p65 * s.v[4])) + ((1.0 - s.v[4]) * p.p104));

        s.v[259] = ((0.05 - s.v[13]) / s.v[6]);

        s.b[445] = (0.05 < s.v[13]);
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if s.b[445] {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[445]) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p63 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[15]) / s.v[6]);

        s.b[446] = (0.05 < s.v[15]);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if s.b[446] {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[446]) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p79 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[21]) / s.v[6]);

        s.b[447] = (0.05 < s.v[21]);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if s.b[447] {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[447]) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p70 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[18]) / s.v[6]);

        s.b[448] = (0.05 < s.v[18]);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if s.b[448] {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[448]) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[254]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[20]) / s.v[6]);

        s.b[449] = (0.05 < s.v[20]);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if s.b[449] {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[449]) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p26 * s.v[4])) + ((1.0 - s.v[4]) * p.p108));

        s.v[259] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[450] = (0.05 < s.v[56]);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if s.b[450] {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[450]) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
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

        s.v[28] = (p.p53 * (((s.v[254] * p.p96)) as f64).exp());

        s.b[451] = (s.v[28] < s.v[316]);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if s.b[451] {
            s.copy_ad(28, 316);
        }

        s.v[29] = (p.p55 * (((s.v[254] * (p.p97 - p.p95))) as f64).exp());

        s.v[30] = (p.p54 * (((s.v[254] * p.p100)) as f64).exp());

        s.b[452] = (s.v[30] < s.v[316]);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if s.b[452] {
            s.copy_ad(30, 316);
        }

        s.v[32] = (p.p56 * (((s.v[254] * p.p101)) as f64).exp());

        s.v[33] = (p.p57 * (((s.v[254] * p.p103)) as f64).exp());

        s.v[34] = (p.p58 * (((s.v[254] * p.p103)) as f64).exp());

        s.v[31] = (p.p59 * (((s.v[254] * p.p98)) as f64).exp());

        s.b[453] = (p.p121 != 0.0);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if s.b[453] {
            s.store_scalar(50, (p.p9 * (1.0 + (s.v[12] * p.p121))));
            s.store_scaled_offset(259, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[454] = (s.v[50] < 1.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if (s.b[453] && s.b[454]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[453] && (!s.b[454])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]));
        }

        if s.b[453] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[453]) {
            s.store_scalar(48, p.p9);
        }

        s.b[455] = (p.p122 != 0.0);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if s.b[455] {
            s.store_scalar(51, (p.p10 * (1.0 + (s.v[12] * p.p122))));
            s.store_scaled_offset(259, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[456] = (s.v[51] < 1.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if (s.b[455] && s.b[456]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[455] && (!s.b[456])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]));
        }

        if s.b[455] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[455]) {
            s.store_scalar(49, p.p10);
        }

        s.v[311] = (p.p42 * (1.0 + (p.p123 * s.v[12])));

        s.v[261] = (s.v[312] * s.v[312]);

        s.v[262] = (s.v[311] * s.v[311]);

        s.b[457] = (s.v[311] < 0.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if s.b[457] {
            s.store_scalar(310, ((0.5 * s.v[261]) / ((((s.v[262] + s.v[261])) as f64).sqrt() - s.v[311])));
        }

        if (!s.b[457]) {
            s.store_scalar(310, (0.5 * ((((s.v[262] + s.v[261])) as f64).sqrt() + s.v[311])));
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_from_scalar((s.v[254] * (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), A::exp(A::div_from_scalar(((-p.p104) * s.v[10]), s.ad_value(48))), p.p8);

        s.v[36] = (p.p11 * (((s.v[254] * (1.0 - p.p97))) as f64).exp());

        s.v[37] = (p.p29 * (((s.v[254] * (1.0 - p.p102))) as f64).exp());

        s.v[38] = ((p.p19 * (((s.v[254] * (6.0 - (2.0 * p.p20)))) as f64).exp()) * (((((-p.p112) * s.v[10]) / p.p20)) as f64).exp());

        s.v[39] = ((p.p30 * (((s.v[254] * (6.0 - (2.0 * p.p31)))) as f64).exp()) * (((((-p.p109) * s.v[10]) / p.p31)) as f64).exp());

        s.v[42] = ((p.p15 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p16)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p16)) as f64).exp());

        s.v[44] = ((p.p17 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p18)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p18)) as f64).exp());

        s.b[458] = (p.p23 == 1.0);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if s.b[458] {
            s.store_scalar(53, (p.p24 * (((((-p.p106) * s.v[10]) / p.p16)) as f64).exp()));
            s.store_scalar(54, (p.p27 * ((((-p.p105) * s.v[10])) as f64).exp()));
            s.store_scalar(45, (p.p25 * (((((-p.p107) * s.v[10]) / p.p18)) as f64).exp()));
        }

        s.v[43] = ((p.p28 * (((s.v[254] * ((4.0 - p.p102) + p.p120))) as f64).exp()) * ((((-p.p111) * s.v[10])) as f64).exp());

        s.v[46] = ((p.p21 * (((s.v[254] * (6.0 - (2.0 * p.p22)))) as f64).exp()) * (((((-p.p112) * s.v[10]) / p.p22)) as f64).exp());

        s.v[47] = ((p.p132 * (((s.v[254] * (4.0 / p.p133))) as f64).exp()) * (((((-p.p112) * s.v[10]) / p.p133)) as f64).exp());

        s.v[325] = ((p.p138 * ((s.v[4]) as f64).sqrt()) * (((p.p140 * s.v[12])) as f64).exp());

        s.store_powf_ad(255, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(256, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(255), p.p34), s.ad_value(256), (p.p65 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_ad_value(58, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(255), s.ad_value(14), s.ad_value(14), p.p33), s.ad_value(73), A::exp(A::sub_from_scalar(p.p34, s.ad_value(61))), (s.v[64] * s.v[64])));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(257, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(258, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(257), p.p36), s.ad_value(258), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.store_ad_value(84, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(257), s.ad_value(19), s.ad_value(19), p.p35), s.ad_value(90), A::exp(A::sub_from_scalar(p.p36, s.ad_value(83))), (s.v[66] * s.v[66])));

        s.v[255] = (((s.v[254] * p.p95)) as f64).exp();

        s.store_scale(40, 27, (p.p13 * s.v[255]));

        s.store_scale(41, 256, (p.p12 * s.v[255]));

        s.v[93] = ((p.p85 * (((s.v[254] * (p.p97 - 2.0))) as f64).exp()) * ((((-p.p119) * s.v[10])) as f64).exp());

        s.v[94] = (p.p86 * (((s.v[254] * ((p.p95 + p.p97) - 1.0))) as f64).exp());

        s.v[95] = (p.p87 * (((s.v[254] * (p.p98 - 1.0))) as f64).exp());

        s.v[96] = ((p.p88 * (s.v[94] + s.v[95])) / (p.p86 + p.p87));

        s.v[97] = (p.p89 * (((s.v[254] * (p.p99 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.b[459] = (s.v[2] < 525.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if s.b[459] {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!s.b[459]) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p91 * (((s.v[254] * p.p95)) as f64).exp());

        s.b[460] = (p.p56 > 0.0);
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if s.b[460] {
            s.store_scalar(101, (1.0 / s.v[32]));
        }

        s.b[461] = (s.v[101] > s.v[317]);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if (s.b[460] && s.b[461]) {
            s.copy_ad(101, 317);
        }

        if (!s.b[460]) {
            s.store_scalar(101, 0.0);
        }

        s.b[462] = (p.p57 > 0.0);
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if s.b[462] {
            s.store_scalar(102, (1.0 / s.v[33]));
        }

        s.b[463] = (s.v[102] > s.v[317]);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        if (s.b[462] && s.b[463]) {
            s.copy_ad(102, 317);
        }

        if (!s.b[462]) {
            s.store_scalar(102, 0.0);
        }

        s.b[464] = (p.p58 > 0.0);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if s.b[464] {
            s.store_scalar(103, (1.0 / s.v[34]));
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[465] = (s.v[103] > s.v[317]);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if (s.b[464] && s.b[465]) {
            s.copy_ad(103, 317);
        }

        if (!s.b[464]) {
            s.store_scalar(103, 0.0);
        }

        s.store_scaled_voltage(230, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(231, ctx, nodes, Some(5), Some(7), p.p3);

        s.store_scaled_voltage(232, ctx, nodes, Some(5), Some(3), p.p3);

        s.store_scaled_voltage(233, ctx, nodes, Some(4), Some(3), p.p3);

        s.store_scaled_voltage(234, ctx, nodes, Some(4), Some(5), p.p3);

        s.store_scaled_voltage(236, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(239, ctx, nodes, Some(2), Some(3), p.p3);

        s.store_scaled_voltage(240, ctx, nodes, Some(1), Some(4), p.p3);

        s.store_scaled_voltage(243, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(244, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(238, ctx, nodes, Some(9), Some(6), p.p3);

        s.store_scaled_voltage(237, ctx, nodes, Some(8), Some(9), p.p3);

        s.store_add_scaled_inputs4(235, s.ad_value(234), 1.0, s.ad_value(231), 1.0, s.ad_value(236), -1.0, s.ad_value(238), -1.0);

        s.store_add_scaled_inputs4(242, s.ad_value(240), 1.0, s.ad_value(244), (-1.0), s.ad_value(235), 1.0, s.ad_value(237), -1.0);

        s.store_add(241, 244, 242);

        s.b[466] = ((s.v[231] * s.v[8]) < p.p134);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if s.b[466] {
            s.store_exp_scaled_input(245, 231, s.v[8]);
        }

        if (!s.b[466]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(245, 275, A::scale_offset(s.ad_value(231), s.v[8], (((-p.p134)) + (1.0))));
        }

        s.b[467] = (((s.v[232] * s.v[8]) / s.v[48]) < p.p134);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_exp_ad(246, A::div_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(48), 1.0));
        }

        if (!s.b[467]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(246, 275, A::div_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(48), 1.0), (((-p.p134)) + (1.0)));
        }

        s.b[468] = ((s.v[235] * s.v[8]) < p.p134);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if s.b[468] {
            s.store_exp_scaled_input(248, 235, s.v[8]);
        }

        if (!s.b[468]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(248, 275, A::scale_offset(s.ad_value(235), s.v[8], (((-p.p134)) + (1.0))));
        }

        s.b[469] = ((s.v[234] * s.v[8]) < p.p134);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if s.b[469] {
            s.store_exp_scaled_input(247, 234, s.v[8]);
        }

        if (!s.b[469]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(247, 275, A::scale_offset(s.ad_value(234), s.v[8], (((-p.p134)) + (1.0))));
        }

        s.b[470] = ((s.v[241] * s.v[8]) < p.p134);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if s.b[470] {
            s.store_exp_scaled_input(249, 241, s.v[8]);
        }

        if (!s.b[470]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(249, 275, A::scale_offset(s.ad_value(241), s.v[8], (((-p.p134)) + (1.0))));
        }

        s.b[471] = (((s.v[241] - s.v[16]) * s.v[8]) < p.p134);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if s.b[471] {
            s.store_ad_value(252, A::exp_scaled_input(A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[471]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(252, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[472] = (((s.v[235] - s.v[16]) * s.v[8]) < p.p134);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if s.b[472] {
            s.store_ad_value(250, A::exp_scaled_input(A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[472]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(250, 275, A::sub_scaled_inputs(s.ad_value(235), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[473] = (((s.v[231] - s.v[16]) * s.v[8]) < p.p134);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if s.b[473] {
            s.store_ad_value(251, A::exp_scaled_input(A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[473]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(251, 275, A::sub_scaled_inputs(s.ad_value(231), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[474] = (((s.v[230] - s.v[16]) * s.v[8]) < p.p134);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if s.b[474] {
            s.store_ad_value(253, A::exp_scaled_input(A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[474]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(253, 275, A::sub_scaled_inputs(s.ad_value(230), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(104, 251, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(105, 253, 4.0, 1.0);

        s.store_div_scaled_value_offset_denominator(106, s.ad_value(253), 2.0, s.ad_value(105), 1.0, 1.0);

        s.b[475] = (s.v[106] < p.p136);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if s.b[475] {
            s.store_scalar(106, p.p136);
        }

        s.store_add_scaled_inputs3(107, s.ad_value(104), s.v[6], s.ad_value(105), ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(104), 1.0, 1.0, A::offset(s.ad_value(105), 1.0), 1.0)), (-s.v[6]));

        s.store_scaled_add(108, 107, 236, 1.0 / (s.v[31]));

        s.b[476] = (s.v[108] > 0.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        s.b[477] = (s.v[230] < 100.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[477]) {
            s.copy_ad(277, 230);
        }

        if (s.b[476] && (!s.b[477])) {
            s.store_offset_ln_ad(277, A::offset(s.ad_value(230), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[476] {
            s.store_add_scaled_inputs3(109, s.ad_value(16), 1.0, A::ln(A::scale_offset(s.ad_value(108), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), s.ad_value(277), -1.0);
            s.store_scale(272, 16, 0.2);
            s.store_square(261, 272);
            s.store_square(262, 109);
        }

        s.b[478] = (s.v[109] < 0.0);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[478]) {
            s.store_div_scaled_inputs(110, s.ad_value(261), 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)), 1.0);
        }

        if (s.b[476] && (!s.b[478])) {
            s.store_scaled_add_ad_lhs(110, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 109, 0.5);
        }

        if s.b[476] {
            s.store_div_scaled_product_offset_rhs(111, s.ad_value(110), s.ad_value(110), (p.p61 * p.p60), 1.0, A::scaled_offset(s.ad_value(110), (p.p61 * s.v[31]), p.p60), 1.0);
            s.store_div(265, 108, 111);
            s.store_scaled_offset(259, 265, (-1.0), 1.0 / (p.p62));
        }

        s.b[479] = (s.v[265] < 1.0);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[479]) {
            s.store_offset_scaled_ad(263, A::ln_one_plus_exp(s.ad_value(259)), p.p62, 1.0);
        }

        if (s.b[476] && (!s.b[479])) {
            s.store_ad_value(263, A::add_scaled_inputs(s.ad_value(265), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p62));
        }

        if s.b[476] {
            s.store_scale(112, 263, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
            s.store_scale(113, 110, 1.0 / ((p.p61 * p.p60)));
            s.store_div_scaled_offset_numerator(114, A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(112), s.ad_value(113), A::offset(s.ad_value(113), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(112), 2.0, A::offset(s.ad_value(113), 1.0)), 1.0);
            s.store_div_ad(115, A::add_scaled_sub_value_product(1.0, s.ad_value(114), 1.0, s.ad_value(106), s.ad_value(114), 1.0), A::offset(A::mul(s.ad_value(106), s.ad_value(114)), 1.0));
            s.store_scaled_mul(117, 108, 115, ((0.5 * s.v[31]) * s.v[8]));
            s.store_ad_value(266, A::add_scaled_offset_product_rhs(s.ad_value(117), 2.0, s.ad_value(106), A::add(s.ad_value(106), s.ad_value(117)), 1.0, 1.0));
            s.store_scaled_offset(118, 117, (-1.0), 0.5);
            s.store_add_ad_lhs(260, A::square(s.ad_value(118)), 266);
        }

        s.b[480] = (s.v[117] >= 1.0);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[480]) {
            s.store_add_ad_rhs(119, 118, A::sqrt(s.ad_value(260)));
        }

        if (s.b[476] && (!s.b[480])) {
            s.store_div_ad_rhs(119, 266, A::sub(A::sqrt(s.ad_value(260)), s.ad_value(118)));
        }

        s.b[481] = (s.v[119] < p.p135);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[481]) {
            s.store_scalar(119, p.p135);
        }

        if s.b[476] {
            s.store_mul_ad_product_rhs(121, 119, A::offset(s.ad_value(119), 1.0), A::exp_scaled_input(s.ad_value(16), s.v[8]));
            s.store_scaled_offset(123, 108, (-p.p61), (0.5 * p.p60));
            s.store_scale(124, 108, ((p.p60 * s.v[31]) * p.p61));
            s.store_add_ad_rhs(125, 123, A::sqrt(A::add(A::square(s.ad_value(123)), s.ad_value(124))));
        }

        s.b[482] = (p.p72 == 0.0);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[482]) {
            s.store_scale(126, 17, 0.1);
        }

        if (s.b[476] && (!s.b[482])) {
            s.store_mul_offset_ad_rhs(126, 17, A::div_scaled_inputs(s.ad_value(108), 2.0, A::add(s.ad_value(108), s.ad_value(111)), 1.0), 0.1);
        }

        if s.b[476] {
            s.store_div_scaled_value_offset_denominator(127, s.ad_value(108), p.p61, s.ad_value(108), p.p61, 1.0);
            s.store_div_from_scalar_offset_input(199, p.p61, 108, p.p61);
        }

        if (!s.b[476]) {
            s.store_scalar(111, 0.0);
            s.store_div_scaled_value_offset_denominator(119, s.ad_value(251), 2.0, s.ad_value(104), 1.0, 1.0);
            s.copy_ad(121, 245);
        }

        s.b[483] = ((((s.v[236]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[107]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[104] + s.v[105]))));
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if ((!s.b[476]) && s.b[483]) {
            s.store_scaled_add(128, 119, 106, 0.5);
            s.store_div_scaled_value_offset_denominator(115, s.ad_value(128), 1.0, s.ad_value(128), 1.0, 1.0);
        }

        if ((!s.b[476]) && (!s.b[483])) {
            s.store_div_ad_rhs(115, 107, A::add_scaled_inputs3(s.ad_value(107), 1.0, s.ad_value(231), 1.0, s.ad_value(230), -1.0));
        }

        if (!s.b[476]) {
            s.copy_ad(125, 236);
            s.store_scale(126, 17, 0.1);
            s.copy_ad(127, 108);
            s.store_sub_from_scalar_ad(199, 1.0, A::scale(s.ad_value(127), 1.0 / (p.p61)));
        }

        s.store_scale(129, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(273, 14, 0.1);

        s.store_div_scaled_inputs2(259, s.ad_value(232), 1.0, s.ad_value(129), (-1.0), s.ad_value(273), 1.0);

        s.b[484] = (s.v[232] < s.v[129]);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_add_scaled_product_right_ad(130, 232, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[484]) {
            s.store_add_scaled_product_right_ad(130, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_scaled_inputs3(131, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(232), 3.0, s.ad_value(130), (-3.0));

        s.b[485] = (p.p73 == 1.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.copy_ad(132, 230);
        }

        s.b[486] = (p.p73 == 2.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if ((!s.b[485]) && s.b[486]) {
            s.store_add(132, 230, 125);
        }

        if ((!s.b[485]) && (!s.b[486])) {
            s.copy_ad(132, 231);
        }

        s.store_div_ad(133, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(134, 17, 1.0, A::powf(s.ad_value(133), ((-1.0) / p.p71)));

        s.store_div_scaled_inputs2(259, s.ad_value(132), 1.0, s.ad_value(134), (-1.0), s.ad_value(126), 1.0);

        s.b[487] = (s.v[132] < s.v[134]);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_add_scaled_product_right_ad(135, 132, 1.0, 126, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[487]) {
            s.store_add_scaled_product_right_ad(135, 134, 1.0, 126, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf(136, 199, p.p75);

        s.store_add_ad(137, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(136), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(135), s.ad_value(17))), (1.0 - p.p71))), 1.0 / ((1.0 - p.p71))), A::mul3(s.ad_value(136), s.ad_value(133), A::sub(s.ad_value(132), s.ad_value(135))));

        s.store_add_scaled_product_value_ad(138, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(137)), 1.0, 25, 230, 1.0);

        s.store_scale(139, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(140, 139, 246);

        s.store_div_scaled_value_offset_denominator(142, s.ad_value(140), 1.0, A::sqrt(A::offset(s.ad_value(140), 1.0)), 1.0, 1.0);

        s.store_pow_ad(122, s.ad_value(121), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(141, 139, 122);

        s.store_div_scaled_value_offset_denominator(143, s.ad_value(141), 1.0, A::sqrt(A::offset(s.ad_value(141), 1.0)), 1.0, 1.0);

        s.b[488] = (p.p91 == 0.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_add_ad(144, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));
        }

        if (!s.b[488]) {
            s.store_offset_scaled_div(269, 131, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_div_scaled_inputs(270, s.ad_value(138), (-(s.v[99] * s.v[8])), s.ad_value(40), 1.0);
            s.store_scaled_sub_ad(144, A::exp(s.ad_value(269)), A::exp(s.ad_value(270)), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 144);

        s.b[489] = (s.v[144] < 0.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_div_from_scalar_sub_ad(145, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144));
        }

        if (!s.b[489]) {
            s.store_scaled_add_ad_lhs(145, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 144, 0.5);
        }

        s.store_mul_offset_ad_rhs(146, 145, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_scaled_mul(147, 35, 122, p.p14);

        s.store_mul(148, 35, 246);

        s.store_div_scaled_inputs2(149, s.ad_value(148), 1.0, s.ad_value(147), (-1.0), s.ad_value(146), 1.0);

        s.store_scale(259, 232, 10000.0);

        s.b[490] = (s.v[232] < 0.0);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.store_scaled_ln_one_plus_exp(276, 259, 0.0001);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[490]) {
            s.store_ad_value(276, A::add_scaled_inputs(s.ad_value(232), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.0001));
        }

        s.store_scale(278, 276, 1.0 / (p.p139));

        s.b[491] = (s.v[278] < p.p134);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if s.b[491] {
            s.store_exp(279, 278);
        }

        if (!s.b[491]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_rhs(279, 275, 278, (((-p.p134)) + (1.0)));
        }

        s.store_scaled_offset(326, 279, (-1.0), s.v[325]);

        s.store_scaled_offset(259, 232, (-p.p141), 1000.0);

        s.b[492] = (s.v[232] < p.p141);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if s.b[492] {
            s.store_ad_value(280, A::sub_scaled_inputs(s.ad_value(232), 1.0, A::ln_one_plus_exp(s.ad_value(259)), 0.001));
        }

        if (!s.b[492]) {
            s.store_sub_from_scalar_ad(280, p.p141, A::scale(A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.001));
        }

        s.store_mul_scaled_ad_rhs(327, 280, p.p142, A::powf(A::sub_from_scalar(p.p141, s.ad_value(280)), 2.0));

        s.b[493] = (((s.v[232] * s.v[8]) / p.p16) < p.p134);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if s.b[493] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p16)));
        }

        if (!s.b[493]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(232), (s.v[8] * 1.0 / (p.p16)), (((-p.p134)) + (1.0))));
        }

        s.b[494] = (p.p23 == 1.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        s.b[495] = (((s.v[232] - s.v[55]) * s.v[8]) < p.p134);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[495]) {
            s.store_ad_value(278, A::exp_scaled_input(A::sub(s.ad_value(232), s.ad_value(55)), s.v[8]));
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 275, A::sub_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(55), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[496] = (((s.v[149] / s.v[35]) - 1000.0) < 40.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[496]) {
            s.store_exp_ad(279, A::offset(A::div(s.ad_value(149), s.ad_value(35)), (-1000.0)));
        }

        if (s.b[494] && (!s.b[496])) {
            s.store_scalar(275, ((40.0) as f64).exp());
            s.store_mul_offset_ad_rhs(279, 275, A::div(s.ad_value(149), s.ad_value(35)), (((((-1000.0)) + ((-40.0)))) + (1.0)));
        }

        if s.b[494] {
            let assign3700_ad_e3474: A = A::add(A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(276), (-1.0), s.v[42]), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(53), A::offset(s.ad_value(276), (-1.0)), 2.0, A::sqrt(A::scale_offset(s.ad_value(278), 4.0, 1.0)), 1.0, 1.0), A::div(s.ad_value(138), s.ad_value(40)), 1.0, 1.0), A::div_scaled_product3(s.ad_value(54), A::offset(s.ad_value(121), (-1.0)), s.ad_value(279), 1.0, A::offset(s.ad_value(279), 1.0), 1.0));
            s.store_ad_value(151, assign3700_ad_e3474);
        }

        s.b[497] = (p.p92 == 0.0);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if ((!s.b[494]) && s.b[497]) {
            s.store_scaled_offset(151, 276, (-1.0), s.v[42]);
        }

        if ((!s.b[494]) && (!s.b[497])) {
            s.store_ad_value(151, A::add_scaled_offset_product_lhs(A::scaled_offset(s.ad_value(276), (-1.0), (1.0 - p.p92)), s.v[42], A::add(s.ad_value(276), s.ad_value(121)), (-2.0), A::offset(A::div(s.ad_value(138), s.ad_value(40)), 1.0), (p.p92 * s.v[42])));
        }

        s.b[498] = (((s.v[233] * s.v[8]) / p.p18) < p.p134);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p18)));
        }

        if (!s.b[498]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(233), (s.v[8] * 1.0 / (p.p18)), (((-p.p134)) + (1.0))));
        }

        s.b[499] = (p.p23 == 1.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        s.b[500] = (((s.v[233] - s.v[55]) * s.v[8]) < p.p134);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if (s.b[499] && s.b[500]) {
            s.store_ad_value(278, A::exp_scaled_input(A::sub(s.ad_value(233), s.ad_value(55)), s.v[8]));
        }

        if (s.b[499] && (!s.b[500])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 275, A::sub_scaled_inputs(s.ad_value(233), s.v[8], s.ad_value(55), s.v[8]), (((-p.p134)) + (1.0)));
        }

        if s.b[499] {
            s.store_add_ad(152, A::scaled_offset(s.ad_value(276), (-1.0), s.v[44]), A::div_scaled_product_offset_denominator(s.ad_value(45), A::offset(s.ad_value(276), (-1.0)), 2.0, A::sqrt(A::scale_offset(s.ad_value(278), 4.0, 1.0)), 1.0, 1.0));
        }

        if (!s.b[499]) {
            s.store_scaled_offset(152, 276, (-1.0), s.v[44]);
        }

        s.b[501] = (((s.v[232] * s.v[8]) / p.p20) < p.p134);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p20)));
        }

        if (!s.b[501]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(232), (s.v[8] * 1.0 / (p.p20)), (((-p.p134)) + (1.0))));
        }

        s.store_scaled_offset(153, 276, (-1.0), s.v[38]);

        s.b[502] = (((s.v[233] * s.v[8]) / p.p22) < p.p134);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if s.b[502] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p22)));
        }

        if (!s.b[502]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(233), (s.v[8] * 1.0 / (p.p22)), (((-p.p134)) + (1.0))));
        }

        s.store_scaled_offset(155, 276, (-1.0), s.v[46]);

        s.b[503] = (((s.v[235] * s.v[8]) / p.p31) < p.p134);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if s.b[503] {
            s.store_exp_scaled_input(276, 235, (s.v[8] * 1.0 / (p.p31)));
        }

        if (!s.b[503]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(235), (s.v[8] * 1.0 / (p.p31)), (((-p.p134)) + (1.0))));
        }

        s.store_scaled_offset(154, 276, (-1.0), s.v[39]);

        s.b[504] = (((s.v[233] * s.v[8]) / p.p133) < p.p134);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if s.b[504] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p133)));
        }

        if (!s.b[504]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(233), (s.v[8] * 1.0 / (p.p133)), (((-p.p134)) + (1.0))));
        }

        s.store_scaled_offset(156, 276, (-1.0), s.v[47]);

        s.b[505] = (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[232] < 0.0));
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        s.b[506] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p134);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (s.b[505] && s.b[506]) {
            s.store_exp_ad(68, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))));
        }

        if (s.b[505] && (!s.b[506])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(68, 275, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))), (((-p.p134)) + (1.0)));
        }

        if s.b[505] {
            s.store_mul(255, 232, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(255)), 1e-30)), ((-2.0) - p.p66)), A::sub(A::scale_offset(A::scale(s.ad_value(255), (3.0 * (p.p66 - 1.0))), (-p.p66), (((1.0 - (p.p66 * p.p66))) * (p.p66))), A::mul3_scaled_output(s.ad_value(255), s.ad_value(255), A::offset(s.ad_value(255), (p.p66 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(255, s.ad_value(232), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0);
        }

        s.b[507] = (s.v[255] < (-0.001));
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        s.b[508] = (s.v[255] < p.p134);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if ((s.b[505] && s.b[507]) && s.b[508]) {
            s.store_exp(91, 255);
        }

        if ((s.b[505] && s.b[507]) && (!s.b[508])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_rhs(91, 275, 255, (((-p.p134)) + (1.0)));
        }

        if (s.b[505] && s.b[507]) {
            s.store_mul_scaled_ad_rhs(69, 232, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(255)), 1.0));
        }

        if (s.b[505] && (!s.b[507])) {
            s.store_mul_ad_affine_product_rhs(69, 232, s.ad_value(255), A::offset(A::mul_scaled_lhs(s.ad_value(255), 0.3333333333333333, A::scale_offset(s.ad_value(255), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[505] {
            s.store_mul_ad_affine_product_lhs(57, A::mul3_scaled_output(s.ad_value(58), s.ad_value(69), s.ad_value(59), 2.0), s.ad_value(68), s.v[63], 0.0, 65);
        }

        if (!s.b[505]) {
            s.store_scalar(69, 0.0);
            s.store_scalar(57, 0.0);
        }

        s.b[509] = (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[230] < 0.0));
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(230), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[510] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p134);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (s.b[509] && s.b[510]) {
            s.store_exp_ad(78, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))));
        }

        if (s.b[509] && (!s.b[510])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(78, 275, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))), (((-p.p134)) + (1.0)));
        }

        if s.b[509] {
            s.store_mul(257, 230, 67);
        }

        if s.b[509] {
            let assign4300_ad_e4164: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(257)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(257), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(257), s.ad_value(257), A::offset(s.ad_value(257), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(80, assign4300_ad_e4164);
        }

        if s.b[509] {
            s.store_div_scaled_product_by_product(257, s.ad_value(230), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0);
        }

        s.b[511] = (s.v[257] < (-0.001));
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        s.b[512] = (s.v[257] < p.p134);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[509] && s.b[511]) && s.b[512]) {
            s.store_exp(92, 257);
        }

        if ((s.b[509] && s.b[511]) && (!s.b[512])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_rhs(92, 275, 257, (((-p.p134)) + (1.0)));
        }

        if (s.b[509] && s.b[511]) {
            s.store_mul_scaled_ad_rhs(81, 230, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(257)), 1.0));
        }

        if (s.b[509] && (!s.b[511])) {
            s.store_mul_ad_affine_product_rhs(81, 230, s.ad_value(257), A::offset(A::mul_scaled_lhs(s.ad_value(257), 0.3333333333333333, A::scale_offset(s.ad_value(257), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[509] {
            s.store_mul_ad_affine_product_lhs(82, A::mul3_scaled_output(s.ad_value(84), s.ad_value(81), s.ad_value(77), 2.0), s.ad_value(78), s.v[89], 0.0, 67);
        }

        if (!s.b[509]) {
            s.store_scalar(81, 0.0);
            s.store_scalar(82, 0.0);
        }

        s.store_mul(158, 139, 248);

        s.store_scale(159, 250, 4.0);

        s.store_div_scaled_inputs2(161, s.ad_value(158), 1.0, s.ad_value(139), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(158), 1.0)), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(160, s.ad_value(159), 1.0, A::sqrt(A::offset(s.ad_value(159), 1.0)), 1.0, 1.0);

        s.store_div_scaled_offset_numerator(157, s.ad_value(248), (2.0 * s.v[43]), ((-1.0) * (2.0 * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(248), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);

        s.b[513] = ((p.p5 > 0.0) && (p.p32 > 0.0));
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_scale(157, 157, s.v[150]);
            s.store_div_scaled_offset_numerator(164, s.ad_value(249), ((p.p32 * 2.0) * s.v[43]), ((-1.0) * ((p.p32 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(249), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);
            s.store_scalar(165, 0.0);
        }

        s.b[514] = (p.p5 == 1.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if (s.b[513] && s.b[514]) {
            s.store_scalar(271, ((p.p32 * s.v[43]) * s.v[32]));
            s.store_offset_scaled_ad(166, A::ln_scaled_input(s.ad_value(271), s.v[8]), (-s.v[6]), ((2.0) * (s.v[6])));
            s.store_sub(264, 241, 166);
            s.store_scalar(261, (0.11 * 0.11));
            s.store_square(262, 264);
        }

        s.b[515] = (s.v[264] < 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[513] && s.b[514]) && s.b[515]) {
            s.store_div_scaled_inputs(167, s.ad_value(261), 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)), 1.0);
        }

        if ((s.b[513] && s.b[514]) && (!s.b[515])) {
            s.store_scaled_add_ad_lhs(167, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 264, 0.5);
        }

        if (s.b[513] && s.b[514]) {
            s.store_div_ad_rhs(168, 167, A::add_scaled_inputs4(s.ad_value(271), 1.0, s.ad_value(164), s.v[32], s.ad_value(165), s.v[32], s.ad_value(167), 1.0));
        }

        if (s.b[513] && (!s.b[514])) {
            s.store_scalar(166, 0.0);
            s.store_scalar(264, 0.0);
            s.store_scalar(167, 0.0);
            s.store_scalar(168, 1.0);
        }

        if s.b[513] {
            s.store_mul(169, 168, 164);
        }

        s.b[516] = (p.p83 == 1.0);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_add(322, 234, 230);
            s.store_scalar(261, (1e-6 * 1e-6));
            s.store_scaled_mul(262, 322, 322, ((-1.0) * (-1.0)));
        }

        s.b[517] = (((-1.0) * s.v[322]) < 0.0);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if (s.b[516] && s.b[517]) {
            s.store_div_scaled_inputs(323, s.ad_value(261), 0.5, A::sub_scaled_inputs(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 1.0, s.ad_value(322), (-1.0)), 1.0);
        }

        if (s.b[516] && (!s.b[517])) {
            s.store_ad_value(323, A::add_scaled_inputs(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 0.5, s.ad_value(322), ((-1.0) * 0.5)));
        }

        if s.b[516] {
            s.store_scalar(324, (1.0 / (1.0 - ((s.v[318]) as f64).powf(p.p81))));
            s.store_scalar(319, (s.v[318] * p.p80));
            s.store_scaled_square(321, 324, (((s.v[318]) as f64).powf((p.p81 - 1.0)) * (p.p81 * 1.0 / (p.p80))));
        }

        s.b[518] = (s.v[323] < s.v[319]);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if (s.b[516] && s.b[518]) {
            s.store_div_from_scalar_sub_from_scalar_ad(320, 1.0, 1.0, A::powf(A::scale(s.ad_value(323), 1.0 / (p.p80)), p.p81));
        }

        if (s.b[516] && (!s.b[518])) {
            s.store_add_scaled_product_left_ad(320, 324, 1.0, A::sub(s.ad_value(323), s.ad_value(319)), 321, 1.0);
        }

        if (!s.b[516]) {
            s.store_scalar(320, 1.0);
        }

        s.store_mul(82, 82, 320);

        s.store_mul(157, 157, 320);

        s.store_mul(154, 154, 320);

        s.store_mul(169, 169, 320);

        s.store_add_ad(172, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 172);

        s.b[519] = (s.v[172] < 0.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_div_from_scalar_sub_ad(173, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172));
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[519]) {
            s.store_scaled_add_ad_lhs(173, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 172, 0.5);
        }

        s.store_mul_offset_ad_rhs(174, 173, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_div_from_scalar(176, s.v[29], 174);

        s.b[520] = (s.v[176] < s.v[316]);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if s.b[520] {
            s.copy_ad(176, 316);
        }

        s.store_scale(175, 176, 3.0);

        s.store_div_scaled_inputs2(177, A::scaled_offset(s.ad_value(247), (-1.0), (2.0 * s.v[6])), 1.0, s.ad_value(234), 1.0, s.ad_value(175), 1.0);

        s.b[521] = (s.v[149] > 0.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        s.b[522] = (p.p38 == 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        s.b[523] = (s.v[230] < p.p43);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        s.b[524] = (((-s.v[149]) / p.p41) < p.p134);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[524]) {
            s.store_exp_scaled_input(308, 149, (-1.0 / (p.p41)));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[524])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(308, 275, A::scale_offset(s.ad_value(149), (-1.0 / (p.p41)), (((-p.p134)) + (1.0))));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_sub_from_scalar_lhs(309, p.p43, 230, 308);
        }

        s.b[525] = (((-s.v[310]) * ((s.v[309]) as f64).powf(p.p40)) < p.p134);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[525]) {
            s.store_exp_ad(313, A::mul_scaled_lhs(s.ad_value(310), -1.0, A::powf(s.ad_value(309), p.p40)));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[525])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, A::powf(s.ad_value(309), p.p40)), (((-p.p134)) + (1.0)));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_ad_product_lhs(196, A::div_from_scalar(p.p39, s.ad_value(310)), s.ad_value(309), 313);
        }

        s.b[526] = (p.p38 == 2.0);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        s.b[527] = (s.v[230] < s.v[16]);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_scalar(185, ((2.0 * p.p45) / (p.p44 * p.p44)));
            s.store_div_scaled_inputs2(260, s.ad_value(16), 1.0, s.ad_value(230), (-1.0), s.ad_value(199), 1.0);
            s.store_sqrt_ad(186, A::div_scaled_inputs(s.ad_value(260), 2.0, s.ad_value(185), 1.0));
        }

        s.b[528] = (p.p7 == 0.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[528]) {
            s.store_scalar(187, p.p44);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[528])) {
            s.store_sub_from_scalar_ad(116, 1.0, A::scale(s.ad_value(115), 0.5));
            s.store_scaled_mul(187, 116, 116, p.p44);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_product_denominator_ad(188, 186, 187, 1.0, A::sqrt(A::add(A::square(s.ad_value(186)), A::square(s.ad_value(187)))), 1.0);
            s.store_div_scaled_inputs2(189, s.ad_value(16), 1.0, s.ad_value(230), (-1.0), s.ad_value(188), 1.0);
            s.store_add_ad_rhs(190, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), s.ad_value(199), 0.5));
        }

        s.b[529] = (p.p7 == 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[529]) {
            s.copy_ad(191, 190);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[529])) {
            s.store_offset_scaled(192, 115, ((2.0) * ((2.0 * p.p46))), (((2.0 * p.p46)) + (1.0)));
            s.store_scalar(193, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
            s.store_sub_ad_rhs(194, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), A::sub(s.ad_value(193), A::div_scaled_inputs(s.ad_value(149), 1.0, s.ad_value(192), p.p61)), 0.5));
            s.store_add_scaled_product(260, A::mul3_scaled_output(s.ad_value(189), s.ad_value(189), s.ad_value(127), (0.1 * 1.0 / (p.p61))), 1.0, A::sub(s.ad_value(194), s.ad_value(190)), A::sub(s.ad_value(194), s.ad_value(190)), 1.0);
            s.store_add_scaled_inputs3(191, s.ad_value(194), 0.5, s.ad_value(190), 0.5, A::sqrt(s.ad_value(260)), 0.5);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_inputs2(267, s.ad_value(191), 1.0, s.ad_value(189), (-1.0), s.ad_value(191), 1.0);
        }

        s.b[530] = (((s.v[267]) as f64).abs() > 1e-7);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[530]) {
            s.store_div_scaled_inputs(195, s.ad_value(188), 0.5, s.ad_value(267), 1.0);
            s.store_mul_ad(196, A::mul3(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(191), s.ad_value(195)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0), A::div(s.ad_value(187), s.ad_value(195)), 1.0))));
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[530])) {
            s.store_mul_ad_product_rhs(196, 0, s.ad_value(187), A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)));
        }

        s.b[531] = (p.p38 == 3.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        s.b[532] = (s.v[230] < p.p43);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad(200, A::powf(A::sub_from_scalar(p.p43, s.ad_value(230)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(149), 1.0, s.ad_value(149), p.p47, 1.0)), p.p48));
        }

        s.b[533] = (p.p7 == 0.0);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[533]) {
            s.copy_ad(201, 200);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_scaled_offset(202, 149, (-p.p51), 1.0 / (p.p47));
            s.store_scaled_offset(259, 202, (-1.0), 1.0 / (p.p50));
        }

        s.b[534] = (s.v[202] < 1.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && s.b[534]) {
            s.store_offset_scaled_ad(203, A::ln_one_plus_exp(s.ad_value(259)), p.p50, 1.0);
        }

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && (!s.b[534])) {
            s.store_ad_value(203, A::add_scaled_inputs(s.ad_value(202), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p50));
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_mul_powf_ad_rhs(201, 200, s.ad_value(203), p.p49);
        }

        s.b[535] = (((-s.v[310]) * s.v[201]) < p.p134);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[535]) {
            s.store_exp_ad(313, A::mul_scaled_lhs(s.ad_value(310), -1.0, s.ad_value(201)));
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[535])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, s.ad_value(201)), (((-p.p134)) + (1.0)));
        }

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad_lhs(196, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p39, s.ad_value(310)), p.p43, s.ad_value(230)), 313);
        }

        s.b[536] = (s.v[196] > 0.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (p.p52 == 1.0);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if ((s.b[521] && s.b[536]) && s.b[537]) {
            s.store_add_scaled_inputs3(197, A::div_from_scalar(s.v[6], A::mul(s.ad_value(149), A::add(s.ad_value(30), s.ad_value(175)))), 1.0, A::div(s.ad_value(146), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(175))), 1.0);
        }

        s.b[538] = (p.p38 == 3.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) {
            s.store_scaled_sub(259, 196, 197, 1000000.0);
        }

        s.b[539] = (s.v[196] < s.v[197]);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && s.b[539]) {
            s.store_ad_value(196, A::sub_scaled_inputs(s.ad_value(196), 1.0, A::ln_one_plus_exp(s.ad_value(259)), 1e-6));
        }

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && (!s.b[539])) {
            s.store_ad_value(196, A::sub_scaled_inputs(s.ad_value(197), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 1e-6));
        }

        if (((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) {
            s.store_mul(198, 149, 196);
        }

        if (((s.b[521] && s.b[536]) && s.b[537]) && (!s.b[538])) {
            s.store_div_scaled_product3_mixed_iiia(198, 149, 196, 197, 1.0, A::add(s.ad_value(196), s.ad_value(197)), 1.0);
        }

        if ((s.b[521] && s.b[536]) && (!s.b[537])) {
            s.store_mul(198, 149, 196);
        }

        s.store_scaled_mul(204, 23, 131, (1.0 - p.p67));

        s.store_div_scaled_inputs2(259, s.ad_value(233), 1.0, s.ad_value(129), (-1.0), s.ad_value(273), 1.0);

        s.b[541] = (s.v[233] < s.v[129]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if s.b[541] {
            s.store_add_scaled_product_right_ad(205, 233, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[541]) {
            s.store_add_scaled_product_right_ad(205, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_mul_scaled_ad_rhs(206, 23, p.p67, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(205), s.ad_value(65))), (1.0 - p.p66)), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(233), 3.0, s.ad_value(205), (-3.0)));

        s.store_scaled_mul(207, 24, 138, p.p76);

        s.v[208] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(212, 142, 173, (0.5 * s.v[208]));

        s.store_scaled_mul(213, 143, 173, (0.5 * s.v[208]));

        s.store_scale(274, 17, 0.1);

        s.store_div_scaled_inputs2(259, s.ad_value(235), 1.0, s.ad_value(134), (-1.0), s.ad_value(274), 1.0);

        s.b[542] = (s.v[235] < s.v[134]);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_add_scaled_product_right_ad(214, 235, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[542]) {
            s.store_add_scaled_product_right_ad(214, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(215, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(214), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(235), s.ad_value(214)), 1.0);

        s.store_mul_scaled_ad_rhs(216, 24, ((1.0 - p.p76) * (1.0 - p.p32)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(215)), 1.0, s.ad_value(25), s.ad_value(235), 1.0));

        s.store_div_scaled_inputs2(259, s.ad_value(241), 1.0, s.ad_value(134), (-1.0), s.ad_value(274), 1.0);

        s.b[543] = (s.v[241] < s.v[134]);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_add_scaled_product_right_ad(217, 241, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[543]) {
            s.store_add_scaled_product_right_ad(217, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(218, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(241), s.ad_value(217)), 1.0);

        s.store_mul_scaled_ad_rhs(219, 24, ((1.0 - p.p76) * p.p32), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(218)), 1.0, s.ad_value(25), s.ad_value(241), 1.0));

        s.store_scaled_powf_ad(220, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p84), (s.v[93] * s.v[36]));

        s.b[544] = ((s.v[232] / (p.p84 * s.v[6])) < p.p134);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(276, 232, 1.0 / ((p.p84 * s.v[6])));
        }

        if (!s.b[544]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(232), 1.0 / ((p.p84 * s.v[6])), (((-p.p134)) + (1.0))));
        }

        s.store_mul(222, 220, 276);

        s.v[223] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_ad_rhs(224, 115, (0.5 * s.v[223]), A::offset(A::add(s.ad_value(119), s.ad_value(106)), 2.0));

        s.b[545] = (p.p78 == 0.0);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_add_scaled_inputs(229, 161, (s.v[208] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 160, (s.v[223] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[546] = ((((s.v[235] - s.v[22]) / p.p90) * s.v[8]) < p.p134);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((!s.b[545]) && s.b[546]) {
            s.store_ad_value(170, A::exp_scaled_input(A::sub(s.ad_value(235), s.ad_value(22)), (1.0 / (p.p90) * s.v[8])));
        }

        if ((!s.b[545]) && (!s.b[546])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(170, 275, A::sub_scaled_inputs(s.ad_value(235), (1.0 / (p.p90) * s.v[8]), s.ad_value(22), (1.0 / (p.p90) * s.v[8])), (((-p.p134)) + (1.0)));
        }

        if (!s.b[545]) {
            s.store_div_scaled_value_offset_denominator(229, s.ad_value(248), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(170), 4.0, 1.0)), 1.0, 1.0);
        }

        s.b[547] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0));
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scale(229, 229, s.v[150]);
        }

        s.b[548] = (p.p78 == 0.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[547] && s.b[548]) {
            s.store_mul(162, 139, 249);
            s.store_div_scaled_inputs2(163, s.ad_value(162), 1.0, s.ad_value(139), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0), 1.0);
            s.store_scale(225, 252, 4.0);
            s.store_div_scaled_value_offset_denominator(226, s.ad_value(225), 1.0, A::sqrt(A::offset(s.ad_value(225), 1.0)), 1.0, 1.0);
            s.store_add_scaled_inputs(227, 163, (s.v[208] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 226, (s.v[223] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[549] = (((s.v[241] - s.v[22]) * s.v[8]) < p.p134);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((s.b[547] && (!s.b[548])) && s.b[549]) {
            s.store_ad_value(171, A::exp_scaled_input(A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]));
        }

        if ((s.b[547] && (!s.b[548])) && (!s.b[549])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(171, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(22), s.v[8]), (((-p.p134)) + (1.0)));
        }

        if (s.b[547] && (!s.b[548])) {
            s.store_div_scaled_value_offset_denominator(227, s.ad_value(249), (((2.0 * p.p32) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(171), 4.0, 1.0)), 1.0, 1.0);
        }

        if s.b[547] {
            s.store_mul(228, 168, 227);
        }

        s.b[550] = (p.p6 == 1.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_offset_powf_ad(179, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (-p.p66), (-3.0));
            s.store_div_scaled_inputs2(268, s.ad_value(232), 1.0, s.ad_value(129), (-1.0), s.ad_value(273), 1.0);
        }

        s.b[551] = (s.v[268] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (s.b[550] && s.b[551]) {
            s.store_div_from_scalar_offset_ad(180, 1.0, A::exp(s.ad_value(268)), 1.0);
        }

        if (s.b[550] && (!s.b[551])) {
            s.store_div_ad(180, A::exp_scaled_input(s.ad_value(268), -1.0), A::offset(A::exp_scaled_input(s.ad_value(268), -1.0), 1.0));
        }

        if s.b[550] {
            s.store_offset_mul(178, 179, 180, 3.0);
            s.store_scaled_mul(181, 23, 178, (1.0 - p.p67));
            s.store_mul_ad(184, A::div_scaled_product(s.ad_value(139), s.ad_value(246), s.v[8], s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(140), 1.0))));
            s.store_scaled_mul(182, 173, 184, (0.5 * s.v[208]));
            s.store_scale(183, 222, 1.0 / ((p.p84 * s.v[6])));
            s.store_mul_scaled_ad_rhs(211, 234, 0.2, A::add_scaled_inputs3(s.ad_value(181), 1.0, s.ad_value(182), 1.0, s.ad_value(183), 1.0));
            s.store_scale(221, 222, (1.0 - p.p94));
            s.store_add_scaled_inputs(307, 212, 1.0, 222, p.p94);
            s.store_add_scaled_inputs(210, 307, p.p93, 213, 1.0);
            s.store_scale(209, 307, (1.0 - p.p93));
        }

        if (!s.b[550]) {
            s.copy_ad(209, 212);
            s.copy_ad(210, 213);
            s.copy_ad(221, 222);
        }

        s.b[552] = (p.p23 == 1.0);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        s.b[553] = (p.p57 > 0.0);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        s.b[554] = (p.p58 > 0.0);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        s.v[281] = ((4.0 * 1.3806226e-23) * s.v[2]);

        s.store_div_from_scalar(282, s.v[281], 28);

        s.store_div_from_scalar(283, s.v[281], 30);

        s.store_scale(284, 101, s.v[281]);

        s.store_scale(285, 102, s.v[281]);

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scale(286, 103, s.v[281]);

        s.store_scaled_mul_ad(287, A::div_from_scalar(s.v[281], s.ad_value(175)), A::scale_offset(s.ad_value(247), 4.0, 5.0), 0.3333333333333333);

        s.store_div_scaled_inputs2(303, s.ad_value(148), 1.0, s.ad_value(147), 1.0, s.ad_value(146), 1.0);

        s.store_scaled_abs(288, 303, (2.0 * 1.6021918e-19));

        s.b[555] = (p.p129 > 0.0);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_abs_ad(304, A::div(s.ad_value(198), s.ad_value(303)));
        }

        if (!s.b[555]) {
            s.store_scalar(304, 0.0);
        }

        s.store_mul_scaled_ad_rhs(300, 198, (2.0 * 1.6021918e-19), A::offset(s.ad_value(304), 1.0));

        s.b[556] = (s.v[303] > 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_div_scaled_inputs2(305, s.ad_value(209), 1.0, s.ad_value(210), 1.0, s.ad_value(303), 1.0);
        }

        if (!s.b[556]) {
            s.store_scaled_mul(305, 173, 146, s.v[94]);
        }

        s.b[557] = (p.p130 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if s.b[557] {
            s.store_scale(306, 305, p.p93);
        }

        s.b[558] = (p.p130 == 2.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if ((!s.b[557]) && s.b[558]) {
            s.store_scale(306, 305, p.p131);
        }

        if ((!s.b[557]) && (!s.b[558])) {
            s.store_scalar(306, 0.0);
        }

        s.store_scaled_abs_ad(289, A::add(A::add_scaled_inputs4(s.ad_value(151), 1.0, s.ad_value(153), 1.0, s.ad_value(57), -1.0, s.ad_value(327), 1.0), s.ad_value(326)), (2.0 * 1.6021918e-19));

        s.store_add(301, 151, 152);

        s.store_scaled_powf_ad(290, A::abs(s.ad_value(301)), p.p125, p.p127);

        s.b[559] = (s.v[301] < 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if s.b[559] {
            s.store_neg(290, 290);
        }

        s.store_add_scaled_inputs3(302, s.ad_value(153), 1.0, s.ad_value(155), 1.0, s.ad_value(156), 1.0);

        s.store_scaled_powf_ad(291, A::abs(s.ad_value(302)), p.p126, p.p128);

        s.b[560] = (s.v[302] < 0.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if s.b[560] {
            s.store_neg(291, 291);
        }

        s.store_scaled_abs_ad(292, A::add_scaled_inputs3(s.ad_value(152), 1.0, s.ad_value(155), 1.0, s.ad_value(156), 1.0), (2.0 * 1.6021918e-19));

        s.store_scaled_abs(293, 154, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(294, A::abs(s.ad_value(154)), p.p125, p.p127);

        s.b[561] = (s.v[154] < 0.0);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_neg(294, 294);
        }

        s.store_scaled_abs(295, 82, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(296, 157, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(298, A::scale(A::abs(s.ad_value(157)), 1.0 / ((1.0 - (p.p5 * p.p32)))), p.p125, (p.p127 * (1.0 - (p.p5 * p.p32))));

        s.b[562] = (s.v[157] < 0.0);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if s.b[562] {
            s.store_neg(298, 298);
        }

        s.store_scaled_abs(297, 169, ((2.0 * 1.6021918e-19) * p.p5));

        s.b[563] = (p.p32 == 0.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if s.b[563] {
            s.store_scalar(299, 0.0);
        }

        if (!s.b[563]) {
            s.store_scaled_powf_ad(299, A::scale(A::abs(s.ad_value(169)), 1.0 / (p.p32)), p.p125, ((p.p127 * p.p5) * p.p32));
        }

        s.b[564] = (s.v[169] < 0.0);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if s.b[564] {
            s.store_neg(299, 299);
        }

        s.b[565] = (p.p23 == 1.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        s.b[566] = (p.p57 > 0.0);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        s.b[567] = (p.p58 > 0.0);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        s.b[568] = (p.p58 > 0.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[439] = (p.p3 == 1.0);
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if s.b[439] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[439]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[150] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.b[440] = (p.p137 == 0.0);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if s.b[440] {
            s.store_scalar(315, 1e-12);
        }

        if (!s.b[440]) {
            s.store_scalar(315, p.p137);
        }

        s.store_scale(316, 315, p.p1);

        s.v[52] = 0.001;

        s.v[312] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[259] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.b[441] = ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if s.b[441] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[441]) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[259] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.b[442] = ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if s.b[442] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[442]) {
            s.store_scalar(88, ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p116;

        s.v[86] = (1.0 / s.v[87]);

        s.v[168] = 1.0;

        s.v[196] = 0.0;

        s.v[228] = 0.0;

        s.v[211] = 0.0;

        s.v[42] = 0.0;

        s.v[11] = 0.0;

        s.v[2] = (s.v[5] + s.v[11]);

        s.v[4] = (s.v[2] / s.v[3]);

        s.v[6] = (8.617086918058125e-5 * s.v[2]);

        s.v[7] = (8.617086918058125e-5 * s.v[3]);

        s.v[8] = (1.0 / s.v[6]);

        s.v[9] = (1.0 / s.v[7]);

        s.v[10] = (s.v[8] - s.v[9]);

        s.v[12] = (s.v[2] - s.v[3]);

        s.v[254] = ((s.v[4]) as f64).ln();

        s.store_scaled_offset(259, 74, (((-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))) + ((-0.05))), 10.0);

        s.b[443] = ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05);
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if s.b[443] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(259)), 0.1, 0.05);
        }

        if (!s.b[443]) {
            s.store_ad_value(70, A::add_scaled_inputs(A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.1));
        }

        s.store_scaled_offset(259, 88, (((-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))) + ((-0.05))), 10.0);

        s.b[444] = ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if s.b[444] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(259)), 0.1, 0.05);
        }

        if (!s.b[444]) {
            s.store_ad_value(85, A::add_scaled_inputs(A::offset(s.ad_value(88), (-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p65 * s.v[4])) + ((1.0 - s.v[4]) * p.p104));

        s.v[259] = ((0.05 - s.v[13]) / s.v[6]);

        s.b[445] = (0.05 < s.v[13]);
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if s.b[445] {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[445]) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p63 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[15]) / s.v[6]);

        s.b[446] = (0.05 < s.v[15]);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if s.b[446] {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[446]) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p79 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[21]) / s.v[6]);

        s.b[447] = (0.05 < s.v[21]);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if s.b[447] {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[447]) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p70 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[18]) / s.v[6]);

        s.b[448] = (0.05 < s.v[18]);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if s.b[448] {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[448]) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[254]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[20]) / s.v[6]);

        s.b[449] = (0.05 < s.v[20]);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if s.b[449] {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[449]) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p26 * s.v[4])) + ((1.0 - s.v[4]) * p.p108));

        s.v[259] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[450] = (0.05 < s.v[56]);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if s.b[450] {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[450]) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
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

        s.v[28] = (p.p53 * (((s.v[254] * p.p96)) as f64).exp());

        s.b[451] = (s.v[28] < s.v[316]);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if s.b[451] {
            s.copy_ad(28, 316);
        }

        s.v[29] = (p.p55 * (((s.v[254] * (p.p97 - p.p95))) as f64).exp());

        s.v[30] = (p.p54 * (((s.v[254] * p.p100)) as f64).exp());

        s.b[452] = (s.v[30] < s.v[316]);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if s.b[452] {
            s.copy_ad(30, 316);
        }

        s.v[32] = (p.p56 * (((s.v[254] * p.p101)) as f64).exp());

        s.v[31] = (p.p59 * (((s.v[254] * p.p98)) as f64).exp());

        s.b[453] = (p.p121 != 0.0);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if s.b[453] {
            s.store_scalar(50, (p.p9 * (1.0 + (s.v[12] * p.p121))));
            s.store_scaled_offset(259, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[454] = (s.v[50] < 1.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if (s.b[453] && s.b[454]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[453] && (!s.b[454])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]));
        }

        if s.b[453] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[453]) {
            s.store_scalar(48, p.p9);
        }

        s.b[455] = (p.p122 != 0.0);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if s.b[455] {
            s.store_scalar(51, (p.p10 * (1.0 + (s.v[12] * p.p122))));
            s.store_scaled_offset(259, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[456] = (s.v[51] < 1.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if (s.b[455] && s.b[456]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[455] && (!s.b[456])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]));
        }

        if s.b[455] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[455]) {
            s.store_scalar(49, p.p10);
        }

        s.v[311] = (p.p42 * (1.0 + (p.p123 * s.v[12])));

        s.v[261] = (s.v[312] * s.v[312]);

        s.v[262] = (s.v[311] * s.v[311]);

        s.b[457] = (s.v[311] < 0.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if s.b[457] {
            s.store_scalar(310, ((0.5 * s.v[261]) / ((((s.v[262] + s.v[261])) as f64).sqrt() - s.v[311])));
        }

        if (!s.b[457]) {
            s.store_scalar(310, (0.5 * ((((s.v[262] + s.v[261])) as f64).sqrt() + s.v[311])));
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_from_scalar((s.v[254] * (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), A::exp(A::div_from_scalar(((-p.p104) * s.v[10]), s.ad_value(48))), p.p8);

        s.v[36] = (p.p11 * (((s.v[254] * (1.0 - p.p97))) as f64).exp());

        s.v[37] = (p.p29 * (((s.v[254] * (1.0 - p.p102))) as f64).exp());

        s.v[42] = ((p.p15 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p16)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p16)) as f64).exp());

        s.v[43] = ((p.p28 * (((s.v[254] * ((4.0 - p.p102) + p.p120))) as f64).exp()) * ((((-p.p111) * s.v[10])) as f64).exp());

        s.store_powf_ad(255, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(256, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(255), p.p34), s.ad_value(256), (p.p65 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(257, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(258, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(257), p.p36), s.ad_value(258), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.v[255] = (((s.v[254] * p.p95)) as f64).exp();

        s.store_scale(40, 27, (p.p13 * s.v[255]));

        s.store_scale(41, 256, (p.p12 * s.v[255]));

        s.v[93] = ((p.p85 * (((s.v[254] * (p.p97 - 2.0))) as f64).exp()) * ((((-p.p119) * s.v[10])) as f64).exp());

        s.v[94] = (p.p86 * (((s.v[254] * ((p.p95 + p.p97) - 1.0))) as f64).exp());

        s.v[95] = (p.p87 * (((s.v[254] * (p.p98 - 1.0))) as f64).exp());

        s.v[96] = ((p.p88 * (s.v[94] + s.v[95])) / (p.p86 + p.p87));

        s.v[97] = (p.p89 * (((s.v[254] * (p.p99 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.b[459] = (s.v[2] < 525.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if s.b[459] {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!s.b[459]) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p91 * (((s.v[254] * p.p95)) as f64).exp());

        s.store_scaled_voltage(230, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(231, ctx, nodes, Some(5), Some(7), p.p3);

        s.store_scaled_voltage(232, ctx, nodes, Some(5), Some(3), p.p3);

        s.store_scaled_voltage(233, ctx, nodes, Some(4), Some(3), p.p3);

        s.store_scaled_voltage(234, ctx, nodes, Some(4), Some(5), p.p3);

        s.store_scaled_voltage(236, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(240, ctx, nodes, Some(1), Some(4), p.p3);

        s.store_scaled_voltage(243, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(244, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(238, ctx, nodes, Some(9), Some(6), p.p3);

        s.store_scaled_voltage(237, ctx, nodes, Some(8), Some(9), p.p3);

        s.store_add_scaled_inputs4(235, s.ad_value(234), 1.0, s.ad_value(231), 1.0, s.ad_value(236), -1.0, s.ad_value(238), -1.0);

        s.store_add_scaled_inputs4(242, s.ad_value(240), 1.0, s.ad_value(244), (-1.0), s.ad_value(235), 1.0, s.ad_value(237), -1.0);

        s.store_add(241, 244, 242);

        s.b[466] = ((s.v[231] * s.v[8]) < p.p134);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if s.b[466] {
            s.store_exp_scaled_input(245, 231, s.v[8]);
        }

        if (!s.b[466]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(245, 275, A::scale_offset(s.ad_value(231), s.v[8], (((-p.p134)) + (1.0))));
        }

        s.b[467] = (((s.v[232] * s.v[8]) / s.v[48]) < p.p134);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_exp_ad(246, A::div_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(48), 1.0));
        }

        if (!s.b[467]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(246, 275, A::div_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(48), 1.0), (((-p.p134)) + (1.0)));
        }

        s.b[468] = ((s.v[235] * s.v[8]) < p.p134);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if s.b[468] {
            s.store_exp_scaled_input(248, 235, s.v[8]);
        }

        if (!s.b[468]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(248, 275, A::scale_offset(s.ad_value(235), s.v[8], (((-p.p134)) + (1.0))));
        }

        s.b[469] = ((s.v[234] * s.v[8]) < p.p134);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if (!s.b[469]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.b[470] = ((s.v[241] * s.v[8]) < p.p134);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if s.b[470] {
            s.store_exp_scaled_input(249, 241, s.v[8]);
        }

        if (!s.b[470]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(249, 275, A::scale_offset(s.ad_value(241), s.v[8], (((-p.p134)) + (1.0))));
        }

        s.b[471] = (((s.v[241] - s.v[16]) * s.v[8]) < p.p134);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if s.b[471] {
            s.store_ad_value(252, A::exp_scaled_input(A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[471]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[471]) {
            s.store_mul_offset_ad_rhs(252, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[472] = (((s.v[235] - s.v[16]) * s.v[8]) < p.p134);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if s.b[472] {
            s.store_ad_value(250, A::exp_scaled_input(A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[472]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(250, 275, A::sub_scaled_inputs(s.ad_value(235), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[473] = (((s.v[231] - s.v[16]) * s.v[8]) < p.p134);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if s.b[473] {
            s.store_ad_value(251, A::exp_scaled_input(A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[473]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(251, 275, A::sub_scaled_inputs(s.ad_value(231), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[474] = (((s.v[230] - s.v[16]) * s.v[8]) < p.p134);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if s.b[474] {
            s.store_ad_value(253, A::exp_scaled_input(A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[474]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(253, 275, A::sub_scaled_inputs(s.ad_value(230), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(104, 251, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(105, 253, 4.0, 1.0);

        s.store_div_scaled_value_offset_denominator(106, s.ad_value(253), 2.0, s.ad_value(105), 1.0, 1.0);

        s.b[475] = (s.v[106] < p.p136);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if s.b[475] {
            s.store_scalar(106, p.p136);
        }

        s.store_add_scaled_inputs3(107, s.ad_value(104), s.v[6], s.ad_value(105), ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(104), 1.0, 1.0, A::offset(s.ad_value(105), 1.0), 1.0)), (-s.v[6]));

        s.store_scaled_add(108, 107, 236, 1.0 / (s.v[31]));

        s.b[476] = (s.v[108] > 0.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        s.b[477] = (s.v[230] < 100.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[477]) {
            s.copy_ad(277, 230);
        }

        if (s.b[476] && (!s.b[477])) {
            s.store_offset_ln_ad(277, A::offset(s.ad_value(230), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[476] {
            s.store_add_scaled_inputs3(109, s.ad_value(16), 1.0, A::ln(A::scale_offset(s.ad_value(108), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), s.ad_value(277), -1.0);
            s.store_scale(272, 16, 0.2);
            s.store_square(261, 272);
            s.store_square(262, 109);
        }

        s.b[478] = (s.v[109] < 0.0);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[478]) {
            s.store_div_scaled_inputs(110, s.ad_value(261), 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)), 1.0);
        }

        if (s.b[476] && (!s.b[478])) {
            s.store_scaled_add_ad_lhs(110, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 109, 0.5);
        }

        if s.b[476] {
            s.store_div_scaled_product_offset_rhs(111, s.ad_value(110), s.ad_value(110), (p.p61 * p.p60), 1.0, A::scaled_offset(s.ad_value(110), (p.p61 * s.v[31]), p.p60), 1.0);
            s.store_div(265, 108, 111);
            s.store_scaled_offset(259, 265, (-1.0), 1.0 / (p.p62));
        }

        s.b[479] = (s.v[265] < 1.0);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[479]) {
            s.store_offset_scaled_ad(263, A::ln_one_plus_exp(s.ad_value(259)), p.p62, 1.0);
        }

        if (s.b[476] && (!s.b[479])) {
            s.store_ad_value(263, A::add_scaled_inputs(s.ad_value(265), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p62));
        }

        if s.b[476] {
            s.store_scale(112, 263, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
            s.store_scale(113, 110, 1.0 / ((p.p61 * p.p60)));
            s.store_div_scaled_offset_numerator(114, A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(112), s.ad_value(113), A::offset(s.ad_value(113), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(112), 2.0, A::offset(s.ad_value(113), 1.0)), 1.0);
            s.store_div_ad(115, A::add_scaled_sub_value_product(1.0, s.ad_value(114), 1.0, s.ad_value(106), s.ad_value(114), 1.0), A::offset(A::mul(s.ad_value(106), s.ad_value(114)), 1.0));
            s.store_scaled_mul(117, 108, 115, ((0.5 * s.v[31]) * s.v[8]));
            s.store_ad_value(266, A::add_scaled_offset_product_rhs(s.ad_value(117), 2.0, s.ad_value(106), A::add(s.ad_value(106), s.ad_value(117)), 1.0, 1.0));
            s.store_scaled_offset(118, 117, (-1.0), 0.5);
            s.store_add_ad_lhs(260, A::square(s.ad_value(118)), 266);
        }

        s.b[480] = (s.v[117] >= 1.0);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[480]) {
            s.store_add_ad_rhs(119, 118, A::sqrt(s.ad_value(260)));
        }

        if (s.b[476] && (!s.b[480])) {
            s.store_div_ad_rhs(119, 266, A::sub(A::sqrt(s.ad_value(260)), s.ad_value(118)));
        }

        s.b[481] = (s.v[119] < p.p135);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[481]) {
            s.store_scalar(119, p.p135);
        }

        if s.b[476] {
            s.store_mul_ad_product_rhs(121, 119, A::offset(s.ad_value(119), 1.0), A::exp_scaled_input(s.ad_value(16), s.v[8]));
            s.store_scaled_offset(123, 108, (-p.p61), (0.5 * p.p60));
            s.store_scale(124, 108, ((p.p60 * s.v[31]) * p.p61));
            s.store_add_ad_rhs(125, 123, A::sqrt(A::add(A::square(s.ad_value(123)), s.ad_value(124))));
        }

        s.b[482] = (p.p72 == 0.0);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[482]) {
            s.store_scale(126, 17, 0.1);
        }

        if (s.b[476] && (!s.b[482])) {
            s.store_mul_offset_ad_rhs(126, 17, A::div_scaled_inputs(s.ad_value(108), 2.0, A::add(s.ad_value(108), s.ad_value(111)), 1.0), 0.1);
        }

        if s.b[476] {
            s.store_div_scaled_value_offset_denominator(127, s.ad_value(108), p.p61, s.ad_value(108), p.p61, 1.0);
            s.store_div_from_scalar_offset_input(199, p.p61, 108, p.p61);
        }

        if (!s.b[476]) {
            s.store_scalar(111, 0.0);
            s.store_div_scaled_value_offset_denominator(119, s.ad_value(251), 2.0, s.ad_value(104), 1.0, 1.0);
            s.copy_ad(121, 245);
        }

        s.b[483] = ((((s.v[236]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[107]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[104] + s.v[105]))));
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if ((!s.b[476]) && s.b[483]) {
            s.store_scaled_add(128, 119, 106, 0.5);
            s.store_div_scaled_value_offset_denominator(115, s.ad_value(128), 1.0, s.ad_value(128), 1.0, 1.0);
        }

        if ((!s.b[476]) && (!s.b[483])) {
            s.store_div_ad_rhs(115, 107, A::add_scaled_inputs3(s.ad_value(107), 1.0, s.ad_value(231), 1.0, s.ad_value(230), -1.0));
        }

        if (!s.b[476]) {
            s.copy_ad(125, 236);
            s.store_scale(126, 17, 0.1);
            s.copy_ad(127, 108);
            s.store_sub_from_scalar_ad(199, 1.0, A::scale(s.ad_value(127), 1.0 / (p.p61)));
        }

        s.store_scale(129, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(273, 14, 0.1);

        s.store_div_scaled_inputs2(259, s.ad_value(232), 1.0, s.ad_value(129), (-1.0), s.ad_value(273), 1.0);

        s.b[484] = (s.v[232] < s.v[129]);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_add_scaled_product_right_ad(130, 232, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[484]) {
            s.store_add_scaled_product_right_ad(130, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_scaled_inputs3(131, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(232), 3.0, s.ad_value(130), (-3.0));

        s.b[485] = (p.p73 == 1.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.copy_ad(132, 230);
        }

        s.b[486] = (p.p73 == 2.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if ((!s.b[485]) && s.b[486]) {
            s.store_add(132, 230, 125);
        }

        if ((!s.b[485]) && (!s.b[486])) {
            s.copy_ad(132, 231);
        }

        s.store_div_ad(133, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(134, 17, 1.0, A::powf(s.ad_value(133), ((-1.0) / p.p71)));

        s.store_div_scaled_inputs2(259, s.ad_value(132), 1.0, s.ad_value(134), (-1.0), s.ad_value(126), 1.0);

        s.b[487] = (s.v[132] < s.v[134]);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_add_scaled_product_right_ad(135, 132, 1.0, 126, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[487]) {
            s.store_add_scaled_product_right_ad(135, 134, 1.0, 126, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf(136, 199, p.p75);

        s.store_add_ad(137, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(136), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(135), s.ad_value(17))), (1.0 - p.p71))), 1.0 / ((1.0 - p.p71))), A::mul3(s.ad_value(136), s.ad_value(133), A::sub(s.ad_value(132), s.ad_value(135))));

        s.store_add_scaled_product_value_ad(138, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(137)), 1.0, 25, 230, 1.0);

        s.store_scale(139, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(140, 139, 246);

        s.store_div_scaled_value_offset_denominator(142, s.ad_value(140), 1.0, A::sqrt(A::offset(s.ad_value(140), 1.0)), 1.0, 1.0);

        s.store_pow_ad(122, s.ad_value(121), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(141, 139, 122);

        s.store_div_scaled_value_offset_denominator(143, s.ad_value(141), 1.0, A::sqrt(A::offset(s.ad_value(141), 1.0)), 1.0, 1.0);

        s.b[488] = (p.p91 == 0.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_add_ad(144, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));
        }

        if (!s.b[488]) {
            s.store_offset_scaled_div(269, 131, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_div_scaled_inputs(270, s.ad_value(138), (-(s.v[99] * s.v[8])), s.ad_value(40), 1.0);
            s.store_scaled_sub_ad(144, A::exp(s.ad_value(269)), A::exp(s.ad_value(270)), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 144);

        s.b[489] = (s.v[144] < 0.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_div_from_scalar_sub_ad(145, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144));
        }

        if (!s.b[489]) {
            s.store_scaled_add_ad_lhs(145, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 144, 0.5);
        }

        s.store_mul_offset_ad_rhs(146, 145, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_scaled_mul(147, 35, 122, p.p14);

        s.store_mul(148, 35, 246);

        s.store_div_scaled_inputs2(149, s.ad_value(148), 1.0, s.ad_value(147), (-1.0), s.ad_value(146), 1.0);

        s.store_scale(259, 232, 10000.0);

        s.b[490] = (s.v[232] < 0.0);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.store_scaled_ln_one_plus_exp(276, 259, 0.0001);
        }

        if (!s.b[490]) {
            s.store_ad_value(276, A::add_scaled_inputs(s.ad_value(232), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.0001));
        }

        s.store_scale(278, 276, 1.0 / (p.p139));

        s.b[491] = (s.v[278] < p.p134);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if (!s.b[491]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_scaled_offset(259, 232, (-p.p141), 1000.0);

        s.b[493] = (((s.v[232] * s.v[8]) / p.p16) < p.p134);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if s.b[493] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p16)));
        }

        if (!s.b[493]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(232), (s.v[8] * 1.0 / (p.p16)), (((-p.p134)) + (1.0))));
        }

        s.b[494] = (p.p23 == 1.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        s.b[495] = (((s.v[232] - s.v[55]) * s.v[8]) < p.p134);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[495]) {
            s.store_ad_value(278, A::exp_scaled_input(A::sub(s.ad_value(232), s.ad_value(55)), s.v[8]));
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 275, A::sub_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(55), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[496] = (((s.v[149] / s.v[35]) - 1000.0) < 40.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if (s.b[494] && (!s.b[496])) {
            s.store_scalar(275, ((40.0) as f64).exp());
        }

        s.b[498] = (((s.v[233] * s.v[8]) / p.p18) < p.p134);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p18)));
        }

        if (!s.b[498]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(233), (s.v[8] * 1.0 / (p.p18)), (((-p.p134)) + (1.0))));
        }

        s.b[499] = (p.p23 == 1.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        s.b[500] = (((s.v[233] - s.v[55]) * s.v[8]) < p.p134);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if (s.b[499] && s.b[500]) {
            s.store_ad_value(278, A::exp_scaled_input(A::sub(s.ad_value(233), s.ad_value(55)), s.v[8]));
        }

        if (s.b[499] && (!s.b[500])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 275, A::sub_scaled_inputs(s.ad_value(233), s.v[8], s.ad_value(55), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[501] = (((s.v[232] * s.v[8]) / p.p20) < p.p134);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p20)));
        }

        if (!s.b[501]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(232), (s.v[8] * 1.0 / (p.p20)), (((-p.p134)) + (1.0))));
        }

        s.b[502] = (((s.v[233] * s.v[8]) / p.p22) < p.p134);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if s.b[502] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p22)));
        }

        if (!s.b[502]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(233), (s.v[8] * 1.0 / (p.p22)), (((-p.p134)) + (1.0))));
        }

        s.b[503] = (((s.v[235] * s.v[8]) / p.p31) < p.p134);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if s.b[503] {
            s.store_exp_scaled_input(276, 235, (s.v[8] * 1.0 / (p.p31)));
        }

        if (!s.b[503]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[503]) {
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(235), (s.v[8] * 1.0 / (p.p31)), (((-p.p134)) + (1.0))));
        }

        s.b[504] = (((s.v[233] * s.v[8]) / p.p133) < p.p134);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if s.b[504] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p133)));
        }

        if (!s.b[504]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(233), (s.v[8] * 1.0 / (p.p133)), (((-p.p134)) + (1.0))));
        }

        s.b[505] = (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[232] < 0.0));
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        s.b[506] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p134);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (s.b[505] && (!s.b[506])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if s.b[505] {
            s.store_mul(255, 232, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(255)), 1e-30)), ((-2.0) - p.p66)), A::sub(A::scale_offset(A::scale(s.ad_value(255), (3.0 * (p.p66 - 1.0))), (-p.p66), (((1.0 - (p.p66 * p.p66))) * (p.p66))), A::mul3_scaled_output(s.ad_value(255), s.ad_value(255), A::offset(s.ad_value(255), (p.p66 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(255, s.ad_value(232), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0);
        }

        s.b[507] = (s.v[255] < (-0.001));
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        s.b[508] = (s.v[255] < p.p134);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if ((s.b[505] && s.b[507]) && (!s.b[508])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.b[509] = (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[230] < 0.0));
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(230), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[510] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p134);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (s.b[509] && (!s.b[510])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if s.b[509] {
            s.store_mul(257, 230, 67);
        }

        if s.b[509] {
            let assign4300_ad_e4164: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(257)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(257), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(257), s.ad_value(257), A::offset(s.ad_value(257), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(80, assign4300_ad_e4164);
        }

        if s.b[509] {
            s.store_div_scaled_product_by_product(257, s.ad_value(230), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0);
        }

        s.b[511] = (s.v[257] < (-0.001));
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        s.b[512] = (s.v[257] < p.p134);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[509] && s.b[511]) && (!s.b[512])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_mul(158, 139, 248);

        s.store_scale(159, 250, 4.0);

        s.store_div_scaled_inputs2(161, s.ad_value(158), 1.0, s.ad_value(139), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(158), 1.0)), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(160, s.ad_value(159), 1.0, A::sqrt(A::offset(s.ad_value(159), 1.0)), 1.0, 1.0);

        s.b[513] = ((p.p5 > 0.0) && (p.p32 > 0.0));
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_div_scaled_offset_numerator(164, s.ad_value(249), ((p.p32 * 2.0) * s.v[43]), ((-1.0) * ((p.p32 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(249), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);
            s.store_scalar(165, 0.0);
        }

        s.b[514] = (p.p5 == 1.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if (s.b[513] && s.b[514]) {
            s.store_scalar(271, ((p.p32 * s.v[43]) * s.v[32]));
            s.store_offset_scaled_ad(166, A::ln_scaled_input(s.ad_value(271), s.v[8]), (-s.v[6]), ((2.0) * (s.v[6])));
            s.store_sub(264, 241, 166);
            s.store_scalar(261, (0.11 * 0.11));
            s.store_square(262, 264);
        }

        s.b[515] = (s.v[264] < 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[513] && s.b[514]) && s.b[515]) {
            s.store_div_scaled_inputs(167, s.ad_value(261), 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)), 1.0);
        }

        if ((s.b[513] && s.b[514]) && (!s.b[515])) {
            s.store_scaled_add_ad_lhs(167, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 264, 0.5);
        }

        if (s.b[513] && s.b[514]) {
            s.store_div_ad_rhs(168, 167, A::add_scaled_inputs4(s.ad_value(271), 1.0, s.ad_value(164), s.v[32], s.ad_value(165), s.v[32], s.ad_value(167), 1.0));
        }

        if (s.b[513] && (!s.b[514])) {
            s.store_scalar(166, 0.0);
            s.store_scalar(264, 0.0);
            s.store_scalar(167, 0.0);
            s.store_scalar(168, 1.0);
        }

        s.b[516] = (p.p83 == 1.0);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_add(322, 234, 230);
            s.store_scalar(261, (1e-6 * 1e-6));
            s.store_scaled_mul(262, 322, 322, ((-1.0) * (-1.0)));
        }

        s.store_add_ad(172, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 172);

        s.b[519] = (s.v[172] < 0.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_div_from_scalar_sub_ad(173, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172));
        }

        if (!s.b[519]) {
            s.store_scaled_add_ad_lhs(173, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 172, 0.5);
        }

        s.store_mul_offset_ad_rhs(174, 173, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_div_from_scalar(176, s.v[29], 174);

        s.b[520] = (s.v[176] < s.v[316]);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if s.b[520] {
            s.copy_ad(176, 316);
        }

        s.store_scale(175, 176, 3.0);

        s.b[521] = (s.v[149] > 0.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        s.b[522] = (p.p38 == 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        s.b[523] = (s.v[230] < p.p43);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        s.b[524] = (((-s.v[149]) / p.p41) < p.p134);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[524]) {
            s.store_exp_scaled_input(308, 149, (-1.0 / (p.p41)));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[524])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(308, 275, A::scale_offset(s.ad_value(149), (-1.0 / (p.p41)), (((-p.p134)) + (1.0))));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_sub_from_scalar_lhs(309, p.p43, 230, 308);
        }

        s.b[525] = (((-s.v[310]) * ((s.v[309]) as f64).powf(p.p40)) < p.p134);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[525]) {
            s.store_exp_ad(313, A::mul_scaled_lhs(s.ad_value(310), -1.0, A::powf(s.ad_value(309), p.p40)));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[525])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, A::powf(s.ad_value(309), p.p40)), (((-p.p134)) + (1.0)));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_ad_product_lhs(196, A::div_from_scalar(p.p39, s.ad_value(310)), s.ad_value(309), 313);
        }

        s.b[526] = (p.p38 == 2.0);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        s.b[527] = (s.v[230] < s.v[16]);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_scalar(185, ((2.0 * p.p45) / (p.p44 * p.p44)));
            s.store_div_scaled_inputs2(260, s.ad_value(16), 1.0, s.ad_value(230), (-1.0), s.ad_value(199), 1.0);
            s.store_sqrt_ad(186, A::div_scaled_inputs(s.ad_value(260), 2.0, s.ad_value(185), 1.0));
        }

        s.b[528] = (p.p7 == 0.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[528]) {
            s.store_scalar(187, p.p44);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[528])) {
            s.store_sub_from_scalar_ad(116, 1.0, A::scale(s.ad_value(115), 0.5));
            s.store_scaled_mul(187, 116, 116, p.p44);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_product_denominator_ad(188, 186, 187, 1.0, A::sqrt(A::add(A::square(s.ad_value(186)), A::square(s.ad_value(187)))), 1.0);
            s.store_div_scaled_inputs2(189, s.ad_value(16), 1.0, s.ad_value(230), (-1.0), s.ad_value(188), 1.0);
            s.store_add_ad_rhs(190, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), s.ad_value(199), 0.5));
        }

        s.b[529] = (p.p7 == 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[529]) {
            s.copy_ad(191, 190);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[529])) {
            s.store_offset_scaled(192, 115, ((2.0) * ((2.0 * p.p46))), (((2.0 * p.p46)) + (1.0)));
            s.store_scalar(193, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
            s.store_sub_ad_rhs(194, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), A::sub(s.ad_value(193), A::div_scaled_inputs(s.ad_value(149), 1.0, s.ad_value(192), p.p61)), 0.5));
            s.store_add_scaled_product(260, A::mul3_scaled_output(s.ad_value(189), s.ad_value(189), s.ad_value(127), (0.1 * 1.0 / (p.p61))), 1.0, A::sub(s.ad_value(194), s.ad_value(190)), A::sub(s.ad_value(194), s.ad_value(190)), 1.0);
            s.store_add_scaled_inputs3(191, s.ad_value(194), 0.5, s.ad_value(190), 0.5, A::sqrt(s.ad_value(260)), 0.5);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_inputs2(267, s.ad_value(191), 1.0, s.ad_value(189), (-1.0), s.ad_value(191), 1.0);
        }

        s.b[530] = (((s.v[267]) as f64).abs() > 1e-7);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[530]) {
            s.store_div_scaled_inputs(195, s.ad_value(188), 0.5, s.ad_value(267), 1.0);
            s.store_mul_ad(196, A::mul3(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(191), s.ad_value(195)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0), A::div(s.ad_value(187), s.ad_value(195)), 1.0))));
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[530])) {
            s.store_mul_ad_product_rhs(196, 0, s.ad_value(187), A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)));
        }

        s.b[531] = (p.p38 == 3.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        s.b[532] = (s.v[230] < p.p43);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad(200, A::powf(A::sub_from_scalar(p.p43, s.ad_value(230)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(149), 1.0, s.ad_value(149), p.p47, 1.0)), p.p48));
        }

        s.b[533] = (p.p7 == 0.0);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[533]) {
            s.copy_ad(201, 200);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_scaled_offset(202, 149, (-p.p51), 1.0 / (p.p47));
            s.store_scaled_offset(259, 202, (-1.0), 1.0 / (p.p50));
        }

        s.b[534] = (s.v[202] < 1.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && s.b[534]) {
            s.store_offset_scaled_ad(203, A::ln_one_plus_exp(s.ad_value(259)), p.p50, 1.0);
        }

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && (!s.b[534])) {
            s.store_ad_value(203, A::add_scaled_inputs(s.ad_value(202), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p50));
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_mul_powf_ad_rhs(201, 200, s.ad_value(203), p.p49);
        }

        s.b[535] = (((-s.v[310]) * s.v[201]) < p.p134);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[535]) {
            s.store_exp_ad(313, A::mul_scaled_lhs(s.ad_value(310), -1.0, s.ad_value(201)));
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[535])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, s.ad_value(201)), (((-p.p134)) + (1.0)));
        }

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad_lhs(196, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p39, s.ad_value(310)), p.p43, s.ad_value(230)), 313);
        }

        s.b[536] = (s.v[196] > 0.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (p.p52 == 1.0);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if ((s.b[521] && s.b[536]) && s.b[537]) {
            s.store_add_scaled_inputs3(197, A::div_from_scalar(s.v[6], A::mul(s.ad_value(149), A::add(s.ad_value(30), s.ad_value(175)))), 1.0, A::div(s.ad_value(146), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(175))), 1.0);
        }

        s.b[538] = (p.p38 == 3.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) {
            s.store_scaled_sub(259, 196, 197, 1000000.0);
        }

        s.b[539] = (s.v[196] < s.v[197]);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && s.b[539]) {
            s.store_ad_value(196, A::sub_scaled_inputs(s.ad_value(196), 1.0, A::ln_one_plus_exp(s.ad_value(259)), 1e-6));
        }

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && (!s.b[539])) {
            s.store_ad_value(196, A::sub_scaled_inputs(s.ad_value(197), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 1e-6));
        }

        s.store_scaled_mul(204, 23, 131, (1.0 - p.p67));

        s.store_div_scaled_inputs2(259, s.ad_value(233), 1.0, s.ad_value(129), (-1.0), s.ad_value(273), 1.0);

        s.b[541] = (s.v[233] < s.v[129]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if s.b[541] {
            s.store_add_scaled_product_right_ad(205, 233, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[541]) {
            s.store_add_scaled_product_right_ad(205, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_mul_scaled_ad_rhs(206, 23, p.p67, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(205), s.ad_value(65))), (1.0 - p.p66)), 1.0 / ((1.0 - p.p66))), 1.0, s.ad_value(233), 3.0, s.ad_value(205), (-3.0)));

        s.store_scaled_mul(207, 24, 138, p.p76);

        s.v[208] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(212, 142, 173, (0.5 * s.v[208]));

        s.store_scaled_mul(213, 143, 173, (0.5 * s.v[208]));

        s.store_scale(274, 17, 0.1);

        s.store_div_scaled_inputs2(259, s.ad_value(235), 1.0, s.ad_value(134), (-1.0), s.ad_value(274), 1.0);

        s.b[542] = (s.v[235] < s.v[134]);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_add_scaled_product_right_ad(214, 235, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[542]) {
            s.store_add_scaled_product_right_ad(214, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(215, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(214), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(235), s.ad_value(214)), 1.0);

        s.store_mul_scaled_ad_rhs(216, 24, ((1.0 - p.p76) * (1.0 - p.p32)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(215)), 1.0, s.ad_value(25), s.ad_value(235), 1.0));

        s.store_div_scaled_inputs2(259, s.ad_value(241), 1.0, s.ad_value(134), (-1.0), s.ad_value(274), 1.0);

        s.b[543] = (s.v[241] < s.v[134]);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_add_scaled_product_right_ad(217, 241, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[543]) {
            s.store_add_scaled_product_right_ad(217, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(218, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(241), s.ad_value(217)), 1.0);

        s.store_mul_scaled_ad_rhs(219, 24, ((1.0 - p.p76) * p.p32), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(218)), 1.0, s.ad_value(25), s.ad_value(241), 1.0));

        s.store_scaled_powf_ad(220, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p84), (s.v[93] * s.v[36]));

        s.b[544] = ((s.v[232] / (p.p84 * s.v[6])) < p.p134);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(276, 232, 1.0 / ((p.p84 * s.v[6])));
        }

        if (!s.b[544]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_ad_rhs(276, 275, A::scale_offset(s.ad_value(232), 1.0 / ((p.p84 * s.v[6])), (((-p.p134)) + (1.0))));
        }

        s.store_mul(222, 220, 276);

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[223] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_ad_rhs(224, 115, (0.5 * s.v[223]), A::offset(A::add(s.ad_value(119), s.ad_value(106)), 2.0));

        s.b[545] = (p.p78 == 0.0);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_add_scaled_inputs(229, 161, (s.v[208] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 160, (s.v[223] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[546] = ((((s.v[235] - s.v[22]) / p.p90) * s.v[8]) < p.p134);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((!s.b[545]) && s.b[546]) {
            s.store_ad_value(170, A::exp_scaled_input(A::sub(s.ad_value(235), s.ad_value(22)), (1.0 / (p.p90) * s.v[8])));
        }

        if ((!s.b[545]) && (!s.b[546])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(170, 275, A::sub_scaled_inputs(s.ad_value(235), (1.0 / (p.p90) * s.v[8]), s.ad_value(22), (1.0 / (p.p90) * s.v[8])), (((-p.p134)) + (1.0)));
        }

        if (!s.b[545]) {
            s.store_div_scaled_value_offset_denominator(229, s.ad_value(248), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(170), 4.0, 1.0)), 1.0, 1.0);
        }

        s.b[547] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0));
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scale(229, 229, s.v[150]);
        }

        s.b[548] = (p.p78 == 0.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[547] && s.b[548]) {
            s.store_mul(162, 139, 249);
            s.store_div_scaled_inputs2(163, s.ad_value(162), 1.0, s.ad_value(139), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0), 1.0);
            s.store_scale(225, 252, 4.0);
            s.store_div_scaled_value_offset_denominator(226, s.ad_value(225), 1.0, A::sqrt(A::offset(s.ad_value(225), 1.0)), 1.0, 1.0);
            s.store_add_scaled_inputs(227, 163, (s.v[208] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 226, (s.v[223] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[549] = (((s.v[241] - s.v[22]) * s.v[8]) < p.p134);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((s.b[547] && (!s.b[548])) && s.b[549]) {
            s.store_ad_value(171, A::exp_scaled_input(A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]));
        }

        if ((s.b[547] && (!s.b[548])) && (!s.b[549])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(171, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(22), s.v[8]), (((-p.p134)) + (1.0)));
        }

        if (s.b[547] && (!s.b[548])) {
            s.store_div_scaled_value_offset_denominator(227, s.ad_value(249), (((2.0 * p.p32) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(171), 4.0, 1.0)), 1.0, 1.0);
        }

        if s.b[547] {
            s.store_mul(228, 168, 227);
        }

        s.b[550] = (p.p6 == 1.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_offset_powf_ad(179, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (-p.p66), (-3.0));
            s.store_div_scaled_inputs2(268, s.ad_value(232), 1.0, s.ad_value(129), (-1.0), s.ad_value(273), 1.0);
        }

        s.b[551] = (s.v[268] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (s.b[550] && s.b[551]) {
            s.store_div_from_scalar_offset_ad(180, 1.0, A::exp(s.ad_value(268)), 1.0);
        }

        if (s.b[550] && (!s.b[551])) {
            s.store_div_ad(180, A::exp_scaled_input(s.ad_value(268), -1.0), A::offset(A::exp_scaled_input(s.ad_value(268), -1.0), 1.0));
        }

        if s.b[550] {
            s.store_offset_mul(178, 179, 180, 3.0);
            s.store_scaled_mul(181, 23, 178, (1.0 - p.p67));
            s.store_mul_ad(184, A::div_scaled_product(s.ad_value(139), s.ad_value(246), s.v[8], s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(140), 1.0))));
            s.store_scaled_mul(182, 173, 184, (0.5 * s.v[208]));
            s.store_scale(183, 222, 1.0 / ((p.p84 * s.v[6])));
            s.store_mul_scaled_ad_rhs(211, 234, 0.2, A::add_scaled_inputs3(s.ad_value(181), 1.0, s.ad_value(182), 1.0, s.ad_value(183), 1.0));
            s.store_scale(221, 222, (1.0 - p.p94));
            s.store_add_scaled_inputs(307, 212, 1.0, 222, p.p94);
            s.store_add_scaled_inputs(210, 307, p.p93, 213, 1.0);
            s.store_scale(209, 307, (1.0 - p.p93));
        }

        if (!s.b[550]) {
            s.copy_ad(209, 212);
            s.copy_ad(210, 213);
            s.copy_ad(221, 222);
        }

        s.store_div_scaled_inputs2(303, s.ad_value(148), 1.0, s.ad_value(147), 1.0, s.ad_value(146), 1.0);

        s.b[556] = (s.v[303] > 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_div_scaled_inputs2(305, s.ad_value(209), 1.0, s.ad_value(210), 1.0, s.ad_value(303), 1.0);
        }

        if (!s.b[556]) {
            s.store_scaled_mul(305, 173, 146, s.v[94]);
        }

        s.b[557] = (p.p130 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if s.b[557] {
            s.store_scale(306, 305, p.p93);
        }

        s.b[558] = (p.p130 == 2.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if ((!s.b[557]) && s.b[558]) {
            s.store_scale(306, 305, p.p131);
        }

        if ((!s.b[557]) && (!s.b[558])) {
            s.store_scalar(306, 0.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq0_e150: f64 = (p.p3 * s.v[108]);
        let eq0_e150_d_n0: f64 = (p.p3 * s.dn[108][0]);
        let eq0_e150_d_n1: f64 = (p.p3 * s.dn[108][1]);
        let eq0_e150_d_n2: f64 = (p.p3 * s.dn[108][2]);
        let eq0_e150_d_n3: f64 = (p.p3 * s.dn[108][3]);
        let eq0_e150_d_n4: f64 = (p.p3 * s.dn[108][4]);
        let eq0_e150_d_n5: f64 = (p.p3 * s.dn[108][5]);
        let eq0_e150_d_n6: f64 = (p.p3 * s.dn[108][6]);
        let eq0_e150_d_n7: f64 = (p.p3 * s.dn[108][7]);
        let eq0_e150_d_n8: f64 = (p.p3 * s.dn[108][8]);
        let eq0_e150_d_n9: f64 = (p.p3 * s.dn[108][9]);
        let eq0_e150_d_n10: f64 = (p.p3 * s.dn[108][10]);
        let eq0_e150_d_b0: f64 = (p.p3 * s.db[108][0]);
        let eq0_e150_d_b1: f64 = (p.p3 * s.db[108][1]);
        let eq0_e152: f64 = (eq0_e150 * p.p1);
        let eq0_e152_d_n0: f64 = (eq0_e150_d_n0 * p.p1);
        let eq0_e152_d_n1: f64 = (eq0_e150_d_n1 * p.p1);
        let eq0_e152_d_n2: f64 = (eq0_e150_d_n2 * p.p1);
        let eq0_e152_d_n3: f64 = (eq0_e150_d_n3 * p.p1);
        let eq0_e152_d_n4: f64 = (eq0_e150_d_n4 * p.p1);
        let eq0_e152_d_n5: f64 = (eq0_e150_d_n5 * p.p1);
        let eq0_e152_d_n6: f64 = (eq0_e150_d_n6 * p.p1);
        let eq0_e152_d_n7: f64 = (eq0_e150_d_n7 * p.p1);
        let eq0_e152_d_n8: f64 = (eq0_e150_d_n8 * p.p1);
        let eq0_e152_d_n9: f64 = (eq0_e150_d_n9 * p.p1);
        let eq0_e152_d_n10: f64 = (eq0_e150_d_n10 * p.p1);
        let eq0_e152_d_b0: f64 = (eq0_e150_d_b0 * p.p1);
        let eq0_e152_d_b1: f64 = (eq0_e150_d_b1 * p.p1);
        let eq0_value: f64 = eq0_e152;
        let eq0_node_derivatives: [f64; 11] = [eq0_e152_d_n0, eq0_e152_d_n1, eq0_e152_d_n2, eq0_e152_d_n3, eq0_e152_d_n4, eq0_e152_d_n5, eq0_e152_d_n6, eq0_e152_d_n7, eq0_e152_d_n8, eq0_e152_d_n9, eq0_e152_d_n10];
        let eq0_branch_derivatives: [f64; 2] = [eq0_e152_d_b0, eq0_e152_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e155: f64 = (p.p3 * s.v[149]);
        let eq1_e155_d_n0: f64 = (p.p3 * s.dn[149][0]);
        let eq1_e155_d_n1: f64 = (p.p3 * s.dn[149][1]);
        let eq1_e155_d_n2: f64 = (p.p3 * s.dn[149][2]);
        let eq1_e155_d_n3: f64 = (p.p3 * s.dn[149][3]);
        let eq1_e155_d_n4: f64 = (p.p3 * s.dn[149][4]);
        let eq1_e155_d_n5: f64 = (p.p3 * s.dn[149][5]);
        let eq1_e155_d_n6: f64 = (p.p3 * s.dn[149][6]);
        let eq1_e155_d_n7: f64 = (p.p3 * s.dn[149][7]);
        let eq1_e155_d_n8: f64 = (p.p3 * s.dn[149][8]);
        let eq1_e155_d_n9: f64 = (p.p3 * s.dn[149][9]);
        let eq1_e155_d_n10: f64 = (p.p3 * s.dn[149][10]);
        let eq1_e155_d_b0: f64 = (p.p3 * s.db[149][0]);
        let eq1_e155_d_b1: f64 = (p.p3 * s.db[149][1]);
        let eq1_e157: f64 = (eq1_e155 * p.p1);
        let eq1_e157_d_n0: f64 = (eq1_e155_d_n0 * p.p1);
        let eq1_e157_d_n1: f64 = (eq1_e155_d_n1 * p.p1);
        let eq1_e157_d_n2: f64 = (eq1_e155_d_n2 * p.p1);
        let eq1_e157_d_n3: f64 = (eq1_e155_d_n3 * p.p1);
        let eq1_e157_d_n4: f64 = (eq1_e155_d_n4 * p.p1);
        let eq1_e157_d_n5: f64 = (eq1_e155_d_n5 * p.p1);
        let eq1_e157_d_n6: f64 = (eq1_e155_d_n6 * p.p1);
        let eq1_e157_d_n7: f64 = (eq1_e155_d_n7 * p.p1);
        let eq1_e157_d_n8: f64 = (eq1_e155_d_n8 * p.p1);
        let eq1_e157_d_n9: f64 = (eq1_e155_d_n9 * p.p1);
        let eq1_e157_d_n10: f64 = (eq1_e155_d_n10 * p.p1);
        let eq1_e157_d_b0: f64 = (eq1_e155_d_b0 * p.p1);
        let eq1_e157_d_b1: f64 = (eq1_e155_d_b1 * p.p1);
        let eq1_value: f64 = eq1_e157;
        let eq1_node_derivatives: [f64; 11] = [eq1_e157_d_n0, eq1_e157_d_n1, eq1_e157_d_n2, eq1_e157_d_n3, eq1_e157_d_n4, eq1_e157_d_n5, eq1_e157_d_n6, eq1_e157_d_n7, eq1_e157_d_n8, eq1_e157_d_n9, eq1_e157_d_n10];
        let eq1_branch_derivatives: [f64; 2] = [eq1_e157_d_b0, eq1_e157_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e161: f64 = (s.v[152] + s.v[155]);
        let eq2_e161_d_n0: f64 = (s.dn[152][0] + s.dn[155][0]);
        let eq2_e161_d_n1: f64 = (s.dn[152][1] + s.dn[155][1]);
        let eq2_e161_d_n2: f64 = (s.dn[152][2] + s.dn[155][2]);
        let eq2_e161_d_n3: f64 = (s.dn[152][3] + s.dn[155][3]);
        let eq2_e161_d_n4: f64 = (s.dn[152][4] + s.dn[155][4]);
        let eq2_e161_d_n5: f64 = (s.dn[152][5] + s.dn[155][5]);
        let eq2_e161_d_n6: f64 = (s.dn[152][6] + s.dn[155][6]);
        let eq2_e161_d_n7: f64 = (s.dn[152][7] + s.dn[155][7]);
        let eq2_e161_d_n8: f64 = (s.dn[152][8] + s.dn[155][8]);
        let eq2_e161_d_n9: f64 = (s.dn[152][9] + s.dn[155][9]);
        let eq2_e161_d_n10: f64 = (s.dn[152][10] + s.dn[155][10]);
        let eq2_e161_d_b0: f64 = (s.db[152][0] + s.db[155][0]);
        let eq2_e161_d_b1: f64 = (s.db[152][1] + s.db[155][1]);
        let eq2_e163: f64 = (eq2_e161 + s.v[156]);
        let eq2_e163_d_n0: f64 = (eq2_e161_d_n0 + s.dn[156][0]);
        let eq2_e163_d_n1: f64 = (eq2_e161_d_n1 + s.dn[156][1]);
        let eq2_e163_d_n2: f64 = (eq2_e161_d_n2 + s.dn[156][2]);
        let eq2_e163_d_n3: f64 = (eq2_e161_d_n3 + s.dn[156][3]);
        let eq2_e163_d_n4: f64 = (eq2_e161_d_n4 + s.dn[156][4]);
        let eq2_e163_d_n5: f64 = (eq2_e161_d_n5 + s.dn[156][5]);
        let eq2_e163_d_n6: f64 = (eq2_e161_d_n6 + s.dn[156][6]);
        let eq2_e163_d_n7: f64 = (eq2_e161_d_n7 + s.dn[156][7]);
        let eq2_e163_d_n8: f64 = (eq2_e161_d_n8 + s.dn[156][8]);
        let eq2_e163_d_n9: f64 = (eq2_e161_d_n9 + s.dn[156][9]);
        let eq2_e163_d_n10: f64 = (eq2_e161_d_n10 + s.dn[156][10]);
        let eq2_e163_d_b0: f64 = (eq2_e161_d_b0 + s.db[156][0]);
        let eq2_e163_d_b1: f64 = (eq2_e161_d_b1 + s.db[156][1]);
        let eq2_e164: f64 = (p.p3 * eq2_e163);
        let eq2_e164_d_n0: f64 = (p.p3 * eq2_e163_d_n0);
        let eq2_e164_d_n1: f64 = (p.p3 * eq2_e163_d_n1);
        let eq2_e164_d_n2: f64 = (p.p3 * eq2_e163_d_n2);
        let eq2_e164_d_n3: f64 = (p.p3 * eq2_e163_d_n3);
        let eq2_e164_d_n4: f64 = (p.p3 * eq2_e163_d_n4);
        let eq2_e164_d_n5: f64 = (p.p3 * eq2_e163_d_n5);
        let eq2_e164_d_n6: f64 = (p.p3 * eq2_e163_d_n6);
        let eq2_e164_d_n7: f64 = (p.p3 * eq2_e163_d_n7);
        let eq2_e164_d_n8: f64 = (p.p3 * eq2_e163_d_n8);
        let eq2_e164_d_n9: f64 = (p.p3 * eq2_e163_d_n9);
        let eq2_e164_d_n10: f64 = (p.p3 * eq2_e163_d_n10);
        let eq2_e164_d_b0: f64 = (p.p3 * eq2_e163_d_b0);
        let eq2_e164_d_b1: f64 = (p.p3 * eq2_e163_d_b1);
        let eq2_e166: f64 = (eq2_e164 * p.p1);
        let eq2_e166_d_n0: f64 = (eq2_e164_d_n0 * p.p1);
        let eq2_e166_d_n1: f64 = (eq2_e164_d_n1 * p.p1);
        let eq2_e166_d_n2: f64 = (eq2_e164_d_n2 * p.p1);
        let eq2_e166_d_n3: f64 = (eq2_e164_d_n3 * p.p1);
        let eq2_e166_d_n4: f64 = (eq2_e164_d_n4 * p.p1);
        let eq2_e166_d_n5: f64 = (eq2_e164_d_n5 * p.p1);
        let eq2_e166_d_n6: f64 = (eq2_e164_d_n6 * p.p1);
        let eq2_e166_d_n7: f64 = (eq2_e164_d_n7 * p.p1);
        let eq2_e166_d_n8: f64 = (eq2_e164_d_n8 * p.p1);
        let eq2_e166_d_n9: f64 = (eq2_e164_d_n9 * p.p1);
        let eq2_e166_d_n10: f64 = (eq2_e164_d_n10 * p.p1);
        let eq2_e166_d_b0: f64 = (eq2_e164_d_b0 * p.p1);
        let eq2_e166_d_b1: f64 = (eq2_e164_d_b1 * p.p1);
        let eq2_value: f64 = eq2_e166;
        let eq2_node_derivatives: [f64; 11] = [eq2_e166_d_n0, eq2_e166_d_n1, eq2_e166_d_n2, eq2_e166_d_n3, eq2_e166_d_n4, eq2_e166_d_n5, eq2_e166_d_n6, eq2_e166_d_n7, eq2_e166_d_n8, eq2_e166_d_n9, eq2_e166_d_n10];
        let eq2_branch_derivatives: [f64; 2] = [eq2_e166_d_b0, eq2_e166_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(3),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e170: f64 = (s.v[151] + s.v[153]);
        let eq3_e170_d_n0: f64 = (s.dn[151][0] + s.dn[153][0]);
        let eq3_e170_d_n1: f64 = (s.dn[151][1] + s.dn[153][1]);
        let eq3_e170_d_n2: f64 = (s.dn[151][2] + s.dn[153][2]);
        let eq3_e170_d_n3: f64 = (s.dn[151][3] + s.dn[153][3]);
        let eq3_e170_d_n4: f64 = (s.dn[151][4] + s.dn[153][4]);
        let eq3_e170_d_n5: f64 = (s.dn[151][5] + s.dn[153][5]);
        let eq3_e170_d_n6: f64 = (s.dn[151][6] + s.dn[153][6]);
        let eq3_e170_d_n7: f64 = (s.dn[151][7] + s.dn[153][7]);
        let eq3_e170_d_n8: f64 = (s.dn[151][8] + s.dn[153][8]);
        let eq3_e170_d_n9: f64 = (s.dn[151][9] + s.dn[153][9]);
        let eq3_e170_d_n10: f64 = (s.dn[151][10] + s.dn[153][10]);
        let eq3_e170_d_b0: f64 = (s.db[151][0] + s.db[153][0]);
        let eq3_e170_d_b1: f64 = (s.db[151][1] + s.db[153][1]);
        let eq3_e173: f64 = (s.v[314] * s.v[232]);
        let eq3_e173_d_n0: f64 = (s.v[314] * s.dn[232][0]);
        let eq3_e173_d_n1: f64 = (s.v[314] * s.dn[232][1]);
        let eq3_e173_d_n2: f64 = (s.v[314] * s.dn[232][2]);
        let eq3_e173_d_n3: f64 = (s.v[314] * s.dn[232][3]);
        let eq3_e173_d_n4: f64 = (s.v[314] * s.dn[232][4]);
        let eq3_e173_d_n5: f64 = (s.v[314] * s.dn[232][5]);
        let eq3_e173_d_n6: f64 = (s.v[314] * s.dn[232][6]);
        let eq3_e173_d_n7: f64 = (s.v[314] * s.dn[232][7]);
        let eq3_e173_d_n8: f64 = (s.v[314] * s.dn[232][8]);
        let eq3_e173_d_n9: f64 = (s.v[314] * s.dn[232][9]);
        let eq3_e173_d_n10: f64 = (s.v[314] * s.dn[232][10]);
        let eq3_e173_d_b0: f64 = (s.v[314] * s.db[232][0]);
        let eq3_e173_d_b1: f64 = (s.v[314] * s.db[232][1]);
        let eq3_e174: f64 = (eq3_e170 + eq3_e173);
        let eq3_e174_d_n0: f64 = (eq3_e170_d_n0 + eq3_e173_d_n0);
        let eq3_e174_d_n1: f64 = (eq3_e170_d_n1 + eq3_e173_d_n1);
        let eq3_e174_d_n2: f64 = (eq3_e170_d_n2 + eq3_e173_d_n2);
        let eq3_e174_d_n3: f64 = (eq3_e170_d_n3 + eq3_e173_d_n3);
        let eq3_e174_d_n4: f64 = (eq3_e170_d_n4 + eq3_e173_d_n4);
        let eq3_e174_d_n5: f64 = (eq3_e170_d_n5 + eq3_e173_d_n5);
        let eq3_e174_d_n6: f64 = (eq3_e170_d_n6 + eq3_e173_d_n6);
        let eq3_e174_d_n7: f64 = (eq3_e170_d_n7 + eq3_e173_d_n7);
        let eq3_e174_d_n8: f64 = (eq3_e170_d_n8 + eq3_e173_d_n8);
        let eq3_e174_d_n9: f64 = (eq3_e170_d_n9 + eq3_e173_d_n9);
        let eq3_e174_d_n10: f64 = (eq3_e170_d_n10 + eq3_e173_d_n10);
        let eq3_e174_d_b0: f64 = (eq3_e170_d_b0 + eq3_e173_d_b0);
        let eq3_e174_d_b1: f64 = (eq3_e170_d_b1 + eq3_e173_d_b1);
        let eq3_e176: f64 = (eq3_e174 - s.v[57]);
        let eq3_e176_d_n0: f64 = (eq3_e174_d_n0 - s.dn[57][0]);
        let eq3_e176_d_n1: f64 = (eq3_e174_d_n1 - s.dn[57][1]);
        let eq3_e176_d_n2: f64 = (eq3_e174_d_n2 - s.dn[57][2]);
        let eq3_e176_d_n3: f64 = (eq3_e174_d_n3 - s.dn[57][3]);
        let eq3_e176_d_n4: f64 = (eq3_e174_d_n4 - s.dn[57][4]);
        let eq3_e176_d_n5: f64 = (eq3_e174_d_n5 - s.dn[57][5]);
        let eq3_e176_d_n6: f64 = (eq3_e174_d_n6 - s.dn[57][6]);
        let eq3_e176_d_n7: f64 = (eq3_e174_d_n7 - s.dn[57][7]);
        let eq3_e176_d_n8: f64 = (eq3_e174_d_n8 - s.dn[57][8]);
        let eq3_e176_d_n9: f64 = (eq3_e174_d_n9 - s.dn[57][9]);
        let eq3_e176_d_n10: f64 = (eq3_e174_d_n10 - s.dn[57][10]);
        let eq3_e176_d_b0: f64 = (eq3_e174_d_b0 - s.db[57][0]);
        let eq3_e176_d_b1: f64 = (eq3_e174_d_b1 - s.db[57][1]);
        let eq3_e178: f64 = (eq3_e176 + s.v[327]);
        let eq3_e178_d_n0: f64 = (eq3_e176_d_n0 + s.dn[327][0]);
        let eq3_e178_d_n1: f64 = (eq3_e176_d_n1 + s.dn[327][1]);
        let eq3_e178_d_n2: f64 = (eq3_e176_d_n2 + s.dn[327][2]);
        let eq3_e178_d_n3: f64 = (eq3_e176_d_n3 + s.dn[327][3]);
        let eq3_e178_d_n4: f64 = (eq3_e176_d_n4 + s.dn[327][4]);
        let eq3_e178_d_n5: f64 = (eq3_e176_d_n5 + s.dn[327][5]);
        let eq3_e178_d_n6: f64 = (eq3_e176_d_n6 + s.dn[327][6]);
        let eq3_e178_d_n7: f64 = (eq3_e176_d_n7 + s.dn[327][7]);
        let eq3_e178_d_n8: f64 = (eq3_e176_d_n8 + s.dn[327][8]);
        let eq3_e178_d_n9: f64 = (eq3_e176_d_n9 + s.dn[327][9]);
        let eq3_e178_d_n10: f64 = (eq3_e176_d_n10 + s.dn[327][10]);
        let eq3_e178_d_b0: f64 = (eq3_e176_d_b0 + s.db[327][0]);
        let eq3_e178_d_b1: f64 = (eq3_e176_d_b1 + s.db[327][1]);
        let eq3_e180: f64 = (eq3_e178 + s.v[326]);
        let eq3_e180_d_n0: f64 = (eq3_e178_d_n0 + s.dn[326][0]);
        let eq3_e180_d_n1: f64 = (eq3_e178_d_n1 + s.dn[326][1]);
        let eq3_e180_d_n2: f64 = (eq3_e178_d_n2 + s.dn[326][2]);
        let eq3_e180_d_n3: f64 = (eq3_e178_d_n3 + s.dn[326][3]);
        let eq3_e180_d_n4: f64 = (eq3_e178_d_n4 + s.dn[326][4]);
        let eq3_e180_d_n5: f64 = (eq3_e178_d_n5 + s.dn[326][5]);
        let eq3_e180_d_n6: f64 = (eq3_e178_d_n6 + s.dn[326][6]);
        let eq3_e180_d_n7: f64 = (eq3_e178_d_n7 + s.dn[326][7]);
        let eq3_e180_d_n8: f64 = (eq3_e178_d_n8 + s.dn[326][8]);
        let eq3_e180_d_n9: f64 = (eq3_e178_d_n9 + s.dn[326][9]);
        let eq3_e180_d_n10: f64 = (eq3_e178_d_n10 + s.dn[326][10]);
        let eq3_e180_d_b0: f64 = (eq3_e178_d_b0 + s.db[326][0]);
        let eq3_e180_d_b1: f64 = (eq3_e178_d_b1 + s.db[326][1]);
        let eq3_e181: f64 = (p.p3 * eq3_e180);
        let eq3_e181_d_n0: f64 = (p.p3 * eq3_e180_d_n0);
        let eq3_e181_d_n1: f64 = (p.p3 * eq3_e180_d_n1);
        let eq3_e181_d_n2: f64 = (p.p3 * eq3_e180_d_n2);
        let eq3_e181_d_n3: f64 = (p.p3 * eq3_e180_d_n3);
        let eq3_e181_d_n4: f64 = (p.p3 * eq3_e180_d_n4);
        let eq3_e181_d_n5: f64 = (p.p3 * eq3_e180_d_n5);
        let eq3_e181_d_n6: f64 = (p.p3 * eq3_e180_d_n6);
        let eq3_e181_d_n7: f64 = (p.p3 * eq3_e180_d_n7);
        let eq3_e181_d_n8: f64 = (p.p3 * eq3_e180_d_n8);
        let eq3_e181_d_n9: f64 = (p.p3 * eq3_e180_d_n9);
        let eq3_e181_d_n10: f64 = (p.p3 * eq3_e180_d_n10);
        let eq3_e181_d_b0: f64 = (p.p3 * eq3_e180_d_b0);
        let eq3_e181_d_b1: f64 = (p.p3 * eq3_e180_d_b1);
        let eq3_e183: f64 = (eq3_e181 * p.p1);
        let eq3_e183_d_n0: f64 = (eq3_e181_d_n0 * p.p1);
        let eq3_e183_d_n1: f64 = (eq3_e181_d_n1 * p.p1);
        let eq3_e183_d_n2: f64 = (eq3_e181_d_n2 * p.p1);
        let eq3_e183_d_n3: f64 = (eq3_e181_d_n3 * p.p1);
        let eq3_e183_d_n4: f64 = (eq3_e181_d_n4 * p.p1);
        let eq3_e183_d_n5: f64 = (eq3_e181_d_n5 * p.p1);
        let eq3_e183_d_n6: f64 = (eq3_e181_d_n6 * p.p1);
        let eq3_e183_d_n7: f64 = (eq3_e181_d_n7 * p.p1);
        let eq3_e183_d_n8: f64 = (eq3_e181_d_n8 * p.p1);
        let eq3_e183_d_n9: f64 = (eq3_e181_d_n9 * p.p1);
        let eq3_e183_d_n10: f64 = (eq3_e181_d_n10 * p.p1);
        let eq3_e183_d_b0: f64 = (eq3_e181_d_b0 * p.p1);
        let eq3_e183_d_b1: f64 = (eq3_e181_d_b1 * p.p1);
        let eq3_value: f64 = eq3_e183;
        let eq3_node_derivatives: [f64; 11] = [eq3_e183_d_n0, eq3_e183_d_n1, eq3_e183_d_n2, eq3_e183_d_n3, eq3_e183_d_n4, eq3_e183_d_n5, eq3_e183_d_n6, eq3_e183_d_n7, eq3_e183_d_n8, eq3_e183_d_n9, eq3_e183_d_n10];
        let eq3_branch_derivatives: [f64; 2] = [eq3_e183_d_b0, eq3_e183_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e192, eq4_e192_d_n0, eq4_e192_d_n1, eq4_e192_d_n2, eq4_e192_d_n3, eq4_e192_d_n4, eq4_e192_d_n5, eq4_e192_d_n6, eq4_e192_d_n7, eq4_e192_d_n8, eq4_e192_d_n9, eq4_e192_d_n10, eq4_e192_d_b0, eq4_e192_d_b1,) = {
    if s.b[552] {
        let eq4_e187: f64 = (-s.v[82]);
        let eq4_e187_d_n0: f64 = (-s.dn[82][0]);
        let eq4_e187_d_n1: f64 = (-s.dn[82][1]);
        let eq4_e187_d_n2: f64 = (-s.dn[82][2]);
        let eq4_e187_d_n3: f64 = (-s.dn[82][3]);
        let eq4_e187_d_n4: f64 = (-s.dn[82][4]);
        let eq4_e187_d_n5: f64 = (-s.dn[82][5]);
        let eq4_e187_d_n6: f64 = (-s.dn[82][6]);
        let eq4_e187_d_n7: f64 = (-s.dn[82][7]);
        let eq4_e187_d_n8: f64 = (-s.dn[82][8]);
        let eq4_e187_d_n9: f64 = (-s.dn[82][9]);
        let eq4_e187_d_n10: f64 = (-s.dn[82][10]);
        let eq4_e187_d_b0: f64 = (-s.db[82][0]);
        let eq4_e187_d_b1: f64 = (-s.db[82][1]);
        let eq4_e188: f64 = (p.p3 * eq4_e187);
        let eq4_e188_d_n0: f64 = (p.p3 * eq4_e187_d_n0);
        let eq4_e188_d_n1: f64 = (p.p3 * eq4_e187_d_n1);
        let eq4_e188_d_n2: f64 = (p.p3 * eq4_e187_d_n2);
        let eq4_e188_d_n3: f64 = (p.p3 * eq4_e187_d_n3);
        let eq4_e188_d_n4: f64 = (p.p3 * eq4_e187_d_n4);
        let eq4_e188_d_n5: f64 = (p.p3 * eq4_e187_d_n5);
        let eq4_e188_d_n6: f64 = (p.p3 * eq4_e187_d_n6);
        let eq4_e188_d_n7: f64 = (p.p3 * eq4_e187_d_n7);
        let eq4_e188_d_n8: f64 = (p.p3 * eq4_e187_d_n8);
        let eq4_e188_d_n9: f64 = (p.p3 * eq4_e187_d_n9);
        let eq4_e188_d_n10: f64 = (p.p3 * eq4_e187_d_n10);
        let eq4_e188_d_b0: f64 = (p.p3 * eq4_e187_d_b0);
        let eq4_e188_d_b1: f64 = (p.p3 * eq4_e187_d_b1);
        let eq4_e190: f64 = (eq4_e188 * p.p1);
        let eq4_e190_d_n0: f64 = (eq4_e188_d_n0 * p.p1);
        let eq4_e190_d_n1: f64 = (eq4_e188_d_n1 * p.p1);
        let eq4_e190_d_n2: f64 = (eq4_e188_d_n2 * p.p1);
        let eq4_e190_d_n3: f64 = (eq4_e188_d_n3 * p.p1);
        let eq4_e190_d_n4: f64 = (eq4_e188_d_n4 * p.p1);
        let eq4_e190_d_n5: f64 = (eq4_e188_d_n5 * p.p1);
        let eq4_e190_d_n6: f64 = (eq4_e188_d_n6 * p.p1);
        let eq4_e190_d_n7: f64 = (eq4_e188_d_n7 * p.p1);
        let eq4_e190_d_n8: f64 = (eq4_e188_d_n8 * p.p1);
        let eq4_e190_d_n9: f64 = (eq4_e188_d_n9 * p.p1);
        let eq4_e190_d_n10: f64 = (eq4_e188_d_n10 * p.p1);
        let eq4_e190_d_b0: f64 = (eq4_e188_d_b0 * p.p1);
        let eq4_e190_d_b1: f64 = (eq4_e188_d_b1 * p.p1);
        (eq4_e190, eq4_e190_d_n0, eq4_e190_d_n1, eq4_e190_d_n2, eq4_e190_d_n3, eq4_e190_d_n4, eq4_e190_d_n5, eq4_e190_d_n6, eq4_e190_d_n7, eq4_e190_d_n8, eq4_e190_d_n9, eq4_e190_d_n10, eq4_e190_d_b0, eq4_e190_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e192;
        let eq4_node_derivatives: [f64; 11] = [eq4_e192_d_n0, eq4_e192_d_n1, eq4_e192_d_n2, eq4_e192_d_n3, eq4_e192_d_n4, eq4_e192_d_n5, eq4_e192_d_n6, eq4_e192_d_n7, eq4_e192_d_n8, eq4_e192_d_n9, eq4_e192_d_n10];
        let eq4_branch_derivatives: [f64; 2] = [eq4_e192_d_b0, eq4_e192_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e202, eq5_e202_d_n0, eq5_e202_d_n1, eq5_e202_d_n2, eq5_e202_d_n3, eq5_e202_d_n4, eq5_e202_d_n5, eq5_e202_d_n6, eq5_e202_d_n7, eq5_e202_d_n8, eq5_e202_d_n9, eq5_e202_d_n10, eq5_e202_d_b0, eq5_e202_d_b1,) = {
    if (!s.b[552]) {
        let eq5_e197: f64 = (-s.v[82]);
        let eq5_e197_d_n0: f64 = (-s.dn[82][0]);
        let eq5_e197_d_n1: f64 = (-s.dn[82][1]);
        let eq5_e197_d_n2: f64 = (-s.dn[82][2]);
        let eq5_e197_d_n3: f64 = (-s.dn[82][3]);
        let eq5_e197_d_n4: f64 = (-s.dn[82][4]);
        let eq5_e197_d_n5: f64 = (-s.dn[82][5]);
        let eq5_e197_d_n6: f64 = (-s.dn[82][6]);
        let eq5_e197_d_n7: f64 = (-s.dn[82][7]);
        let eq5_e197_d_n8: f64 = (-s.dn[82][8]);
        let eq5_e197_d_n9: f64 = (-s.dn[82][9]);
        let eq5_e197_d_n10: f64 = (-s.dn[82][10]);
        let eq5_e197_d_b0: f64 = (-s.db[82][0]);
        let eq5_e197_d_b1: f64 = (-s.db[82][1]);
        let eq5_e198: f64 = (p.p3 * eq5_e197);
        let eq5_e198_d_n0: f64 = (p.p3 * eq5_e197_d_n0);
        let eq5_e198_d_n1: f64 = (p.p3 * eq5_e197_d_n1);
        let eq5_e198_d_n2: f64 = (p.p3 * eq5_e197_d_n2);
        let eq5_e198_d_n3: f64 = (p.p3 * eq5_e197_d_n3);
        let eq5_e198_d_n4: f64 = (p.p3 * eq5_e197_d_n4);
        let eq5_e198_d_n5: f64 = (p.p3 * eq5_e197_d_n5);
        let eq5_e198_d_n6: f64 = (p.p3 * eq5_e197_d_n6);
        let eq5_e198_d_n7: f64 = (p.p3 * eq5_e197_d_n7);
        let eq5_e198_d_n8: f64 = (p.p3 * eq5_e197_d_n8);
        let eq5_e198_d_n9: f64 = (p.p3 * eq5_e197_d_n9);
        let eq5_e198_d_n10: f64 = (p.p3 * eq5_e197_d_n10);
        let eq5_e198_d_b0: f64 = (p.p3 * eq5_e197_d_b0);
        let eq5_e198_d_b1: f64 = (p.p3 * eq5_e197_d_b1);
        let eq5_e200: f64 = (eq5_e198 * p.p1);
        let eq5_e200_d_n0: f64 = (eq5_e198_d_n0 * p.p1);
        let eq5_e200_d_n1: f64 = (eq5_e198_d_n1 * p.p1);
        let eq5_e200_d_n2: f64 = (eq5_e198_d_n2 * p.p1);
        let eq5_e200_d_n3: f64 = (eq5_e198_d_n3 * p.p1);
        let eq5_e200_d_n4: f64 = (eq5_e198_d_n4 * p.p1);
        let eq5_e200_d_n5: f64 = (eq5_e198_d_n5 * p.p1);
        let eq5_e200_d_n6: f64 = (eq5_e198_d_n6 * p.p1);
        let eq5_e200_d_n7: f64 = (eq5_e198_d_n7 * p.p1);
        let eq5_e200_d_n8: f64 = (eq5_e198_d_n8 * p.p1);
        let eq5_e200_d_n9: f64 = (eq5_e198_d_n9 * p.p1);
        let eq5_e200_d_n10: f64 = (eq5_e198_d_n10 * p.p1);
        let eq5_e200_d_b0: f64 = (eq5_e198_d_b0 * p.p1);
        let eq5_e200_d_b1: f64 = (eq5_e198_d_b1 * p.p1);
        (eq5_e200, eq5_e200_d_n0, eq5_e200_d_n1, eq5_e200_d_n2, eq5_e200_d_n3, eq5_e200_d_n4, eq5_e200_d_n5, eq5_e200_d_n6, eq5_e200_d_n7, eq5_e200_d_n8, eq5_e200_d_n9, eq5_e200_d_n10, eq5_e200_d_b0, eq5_e200_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e202;
        let eq5_node_derivatives: [f64; 11] = [eq5_e202_d_n0, eq5_e202_d_n1, eq5_e202_d_n2, eq5_e202_d_n3, eq5_e202_d_n4, eq5_e202_d_n5, eq5_e202_d_n6, eq5_e202_d_n7, eq5_e202_d_n8, eq5_e202_d_n9, eq5_e202_d_n10];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e202_d_b0, eq5_e202_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e205: f64 = (p.p3 * s.v[177]);
        let eq6_e205_d_n0: f64 = (p.p3 * s.dn[177][0]);
        let eq6_e205_d_n1: f64 = (p.p3 * s.dn[177][1]);
        let eq6_e205_d_n2: f64 = (p.p3 * s.dn[177][2]);
        let eq6_e205_d_n3: f64 = (p.p3 * s.dn[177][3]);
        let eq6_e205_d_n4: f64 = (p.p3 * s.dn[177][4]);
        let eq6_e205_d_n5: f64 = (p.p3 * s.dn[177][5]);
        let eq6_e205_d_n6: f64 = (p.p3 * s.dn[177][6]);
        let eq6_e205_d_n7: f64 = (p.p3 * s.dn[177][7]);
        let eq6_e205_d_n8: f64 = (p.p3 * s.dn[177][8]);
        let eq6_e205_d_n9: f64 = (p.p3 * s.dn[177][9]);
        let eq6_e205_d_n10: f64 = (p.p3 * s.dn[177][10]);
        let eq6_e205_d_b0: f64 = (p.p3 * s.db[177][0]);
        let eq6_e205_d_b1: f64 = (p.p3 * s.db[177][1]);
        let eq6_e207: f64 = (eq6_e205 * p.p1);
        let eq6_e207_d_n0: f64 = (eq6_e205_d_n0 * p.p1);
        let eq6_e207_d_n1: f64 = (eq6_e205_d_n1 * p.p1);
        let eq6_e207_d_n2: f64 = (eq6_e205_d_n2 * p.p1);
        let eq6_e207_d_n3: f64 = (eq6_e205_d_n3 * p.p1);
        let eq6_e207_d_n4: f64 = (eq6_e205_d_n4 * p.p1);
        let eq6_e207_d_n5: f64 = (eq6_e205_d_n5 * p.p1);
        let eq6_e207_d_n6: f64 = (eq6_e205_d_n6 * p.p1);
        let eq6_e207_d_n7: f64 = (eq6_e205_d_n7 * p.p1);
        let eq6_e207_d_n8: f64 = (eq6_e205_d_n8 * p.p1);
        let eq6_e207_d_n9: f64 = (eq6_e205_d_n9 * p.p1);
        let eq6_e207_d_n10: f64 = (eq6_e205_d_n10 * p.p1);
        let eq6_e207_d_b0: f64 = (eq6_e205_d_b0 * p.p1);
        let eq6_e207_d_b1: f64 = (eq6_e205_d_b1 * p.p1);
        let eq6_value: f64 = eq6_e207;
        let eq6_node_derivatives: [f64; 11] = [eq6_e207_d_n0, eq6_e207_d_n1, eq6_e207_d_n2, eq6_e207_d_n3, eq6_e207_d_n4, eq6_e207_d_n5, eq6_e207_d_n6, eq6_e207_d_n7, eq6_e207_d_n8, eq6_e207_d_n9, eq6_e207_d_n10];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e207_d_b0, eq6_e207_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(5),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e210: f64 = (-1.0);
        let eq7_e212: f64 = (eq7_e210 * s.v[198]);
        let eq7_e212_d_n0: f64 = (eq7_e210 * s.dn[198][0]);
        let eq7_e212_d_n1: f64 = (eq7_e210 * s.dn[198][1]);
        let eq7_e212_d_n2: f64 = (eq7_e210 * s.dn[198][2]);
        let eq7_e212_d_n3: f64 = (eq7_e210 * s.dn[198][3]);
        let eq7_e212_d_n4: f64 = (eq7_e210 * s.dn[198][4]);
        let eq7_e212_d_n5: f64 = (eq7_e210 * s.dn[198][5]);
        let eq7_e212_d_n6: f64 = (eq7_e210 * s.dn[198][6]);
        let eq7_e212_d_n7: f64 = (eq7_e210 * s.dn[198][7]);
        let eq7_e212_d_n8: f64 = (eq7_e210 * s.dn[198][8]);
        let eq7_e212_d_n9: f64 = (eq7_e210 * s.dn[198][9]);
        let eq7_e212_d_n10: f64 = (eq7_e210 * s.dn[198][10]);
        let eq7_e212_d_b0: f64 = (eq7_e210 * s.db[198][0]);
        let eq7_e212_d_b1: f64 = (eq7_e210 * s.db[198][1]);
        let eq7_e213: f64 = (p.p3 * eq7_e212);
        let eq7_e213_d_n0: f64 = (p.p3 * eq7_e212_d_n0);
        let eq7_e213_d_n1: f64 = (p.p3 * eq7_e212_d_n1);
        let eq7_e213_d_n2: f64 = (p.p3 * eq7_e212_d_n2);
        let eq7_e213_d_n3: f64 = (p.p3 * eq7_e212_d_n3);
        let eq7_e213_d_n4: f64 = (p.p3 * eq7_e212_d_n4);
        let eq7_e213_d_n5: f64 = (p.p3 * eq7_e212_d_n5);
        let eq7_e213_d_n6: f64 = (p.p3 * eq7_e212_d_n6);
        let eq7_e213_d_n7: f64 = (p.p3 * eq7_e212_d_n7);
        let eq7_e213_d_n8: f64 = (p.p3 * eq7_e212_d_n8);
        let eq7_e213_d_n9: f64 = (p.p3 * eq7_e212_d_n9);
        let eq7_e213_d_n10: f64 = (p.p3 * eq7_e212_d_n10);
        let eq7_e213_d_b0: f64 = (p.p3 * eq7_e212_d_b0);
        let eq7_e213_d_b1: f64 = (p.p3 * eq7_e212_d_b1);
        let eq7_e215: f64 = (eq7_e213 * p.p1);
        let eq7_e215_d_n0: f64 = (eq7_e213_d_n0 * p.p1);
        let eq7_e215_d_n1: f64 = (eq7_e213_d_n1 * p.p1);
        let eq7_e215_d_n2: f64 = (eq7_e213_d_n2 * p.p1);
        let eq7_e215_d_n3: f64 = (eq7_e213_d_n3 * p.p1);
        let eq7_e215_d_n4: f64 = (eq7_e213_d_n4 * p.p1);
        let eq7_e215_d_n5: f64 = (eq7_e213_d_n5 * p.p1);
        let eq7_e215_d_n6: f64 = (eq7_e213_d_n6 * p.p1);
        let eq7_e215_d_n7: f64 = (eq7_e213_d_n7 * p.p1);
        let eq7_e215_d_n8: f64 = (eq7_e213_d_n8 * p.p1);
        let eq7_e215_d_n9: f64 = (eq7_e213_d_n9 * p.p1);
        let eq7_e215_d_n10: f64 = (eq7_e213_d_n10 * p.p1);
        let eq7_e215_d_b0: f64 = (eq7_e213_d_b0 * p.p1);
        let eq7_e215_d_b1: f64 = (eq7_e213_d_b1 * p.p1);
        let eq7_value: f64 = eq7_e215;
        let eq7_node_derivatives: [f64; 11] = [eq7_e215_d_n0, eq7_e215_d_n1, eq7_e215_d_n2, eq7_e215_d_n3, eq7_e215_d_n4, eq7_e215_d_n5, eq7_e215_d_n6, eq7_e215_d_n7, eq7_e215_d_n8, eq7_e215_d_n9, eq7_e215_d_n10];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e215_d_b0, eq7_e215_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
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
        let eq8_e218: f64 = (p.p3 * s.v[239]);
        let eq8_e218_d_n0: f64 = (p.p3 * s.dn[239][0]);
        let eq8_e218_d_n1: f64 = (p.p3 * s.dn[239][1]);
        let eq8_e218_d_n2: f64 = (p.p3 * s.dn[239][2]);
        let eq8_e218_d_n3: f64 = (p.p3 * s.dn[239][3]);
        let eq8_e218_d_n4: f64 = (p.p3 * s.dn[239][4]);
        let eq8_e218_d_n5: f64 = (p.p3 * s.dn[239][5]);
        let eq8_e218_d_n6: f64 = (p.p3 * s.dn[239][6]);
        let eq8_e218_d_n7: f64 = (p.p3 * s.dn[239][7]);
        let eq8_e218_d_n8: f64 = (p.p3 * s.dn[239][8]);
        let eq8_e218_d_n9: f64 = (p.p3 * s.dn[239][9]);
        let eq8_e218_d_n10: f64 = (p.p3 * s.dn[239][10]);
        let eq8_e218_d_b0: f64 = (p.p3 * s.db[239][0]);
        let eq8_e218_d_b1: f64 = (p.p3 * s.db[239][1]);
        let eq8_e220: f64 = (eq8_e218 / s.v[28]);
        let eq8_e220_d_n0: f64 = (((eq8_e218_d_n0 * s.v[28]) - (eq8_e218 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n1: f64 = (((eq8_e218_d_n1 * s.v[28]) - (eq8_e218 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n2: f64 = (((eq8_e218_d_n2 * s.v[28]) - (eq8_e218 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n3: f64 = (((eq8_e218_d_n3 * s.v[28]) - (eq8_e218 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n4: f64 = (((eq8_e218_d_n4 * s.v[28]) - (eq8_e218 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n5: f64 = (((eq8_e218_d_n5 * s.v[28]) - (eq8_e218 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n6: f64 = (((eq8_e218_d_n6 * s.v[28]) - (eq8_e218 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n7: f64 = (((eq8_e218_d_n7 * s.v[28]) - (eq8_e218 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n8: f64 = (((eq8_e218_d_n8 * s.v[28]) - (eq8_e218 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n9: f64 = (((eq8_e218_d_n9 * s.v[28]) - (eq8_e218 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_n10: f64 = (((eq8_e218_d_n10 * s.v[28]) - (eq8_e218 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_b0: f64 = (((eq8_e218_d_b0 * s.v[28]) - (eq8_e218 * s.db[28][0])) / (s.v[28] * s.v[28]));
        let eq8_e220_d_b1: f64 = (((eq8_e218_d_b1 * s.v[28]) - (eq8_e218 * s.db[28][1])) / (s.v[28] * s.v[28]));
        let eq8_e222: f64 = (eq8_e220 * p.p1);
        let eq8_e222_d_n0: f64 = (eq8_e220_d_n0 * p.p1);
        let eq8_e222_d_n1: f64 = (eq8_e220_d_n1 * p.p1);
        let eq8_e222_d_n2: f64 = (eq8_e220_d_n2 * p.p1);
        let eq8_e222_d_n3: f64 = (eq8_e220_d_n3 * p.p1);
        let eq8_e222_d_n4: f64 = (eq8_e220_d_n4 * p.p1);
        let eq8_e222_d_n5: f64 = (eq8_e220_d_n5 * p.p1);
        let eq8_e222_d_n6: f64 = (eq8_e220_d_n6 * p.p1);
        let eq8_e222_d_n7: f64 = (eq8_e220_d_n7 * p.p1);
        let eq8_e222_d_n8: f64 = (eq8_e220_d_n8 * p.p1);
        let eq8_e222_d_n9: f64 = (eq8_e220_d_n9 * p.p1);
        let eq8_e222_d_n10: f64 = (eq8_e220_d_n10 * p.p1);
        let eq8_e222_d_b0: f64 = (eq8_e220_d_b0 * p.p1);
        let eq8_e222_d_b1: f64 = (eq8_e220_d_b1 * p.p1);
        let eq8_value: f64 = eq8_e222;
        let eq8_node_derivatives: [f64; 11] = [eq8_e222_d_n0, eq8_e222_d_n1, eq8_e222_d_n2, eq8_e222_d_n3, eq8_e222_d_n4, eq8_e222_d_n5, eq8_e222_d_n6, eq8_e222_d_n7, eq8_e222_d_n8, eq8_e222_d_n9, eq8_e222_d_n10];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e222_d_b0, eq8_e222_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e225: f64 = (p.p3 * s.v[240]);
        let eq9_e225_d_n0: f64 = (p.p3 * s.dn[240][0]);
        let eq9_e225_d_n1: f64 = (p.p3 * s.dn[240][1]);
        let eq9_e225_d_n2: f64 = (p.p3 * s.dn[240][2]);
        let eq9_e225_d_n3: f64 = (p.p3 * s.dn[240][3]);
        let eq9_e225_d_n4: f64 = (p.p3 * s.dn[240][4]);
        let eq9_e225_d_n5: f64 = (p.p3 * s.dn[240][5]);
        let eq9_e225_d_n6: f64 = (p.p3 * s.dn[240][6]);
        let eq9_e225_d_n7: f64 = (p.p3 * s.dn[240][7]);
        let eq9_e225_d_n8: f64 = (p.p3 * s.dn[240][8]);
        let eq9_e225_d_n9: f64 = (p.p3 * s.dn[240][9]);
        let eq9_e225_d_n10: f64 = (p.p3 * s.dn[240][10]);
        let eq9_e225_d_b0: f64 = (p.p3 * s.db[240][0]);
        let eq9_e225_d_b1: f64 = (p.p3 * s.db[240][1]);
        let eq9_e227: f64 = (eq9_e225 / s.v[30]);
        let eq9_e227_d_n0: f64 = (((eq9_e225_d_n0 * s.v[30]) - (eq9_e225 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n1: f64 = (((eq9_e225_d_n1 * s.v[30]) - (eq9_e225 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n2: f64 = (((eq9_e225_d_n2 * s.v[30]) - (eq9_e225 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n3: f64 = (((eq9_e225_d_n3 * s.v[30]) - (eq9_e225 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n4: f64 = (((eq9_e225_d_n4 * s.v[30]) - (eq9_e225 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n5: f64 = (((eq9_e225_d_n5 * s.v[30]) - (eq9_e225 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n6: f64 = (((eq9_e225_d_n6 * s.v[30]) - (eq9_e225 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n7: f64 = (((eq9_e225_d_n7 * s.v[30]) - (eq9_e225 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n8: f64 = (((eq9_e225_d_n8 * s.v[30]) - (eq9_e225 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n9: f64 = (((eq9_e225_d_n9 * s.v[30]) - (eq9_e225 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_n10: f64 = (((eq9_e225_d_n10 * s.v[30]) - (eq9_e225 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_b0: f64 = (((eq9_e225_d_b0 * s.v[30]) - (eq9_e225 * s.db[30][0])) / (s.v[30] * s.v[30]));
        let eq9_e227_d_b1: f64 = (((eq9_e225_d_b1 * s.v[30]) - (eq9_e225 * s.db[30][1])) / (s.v[30] * s.v[30]));
        let eq9_e229: f64 = (eq9_e227 * p.p1);
        let eq9_e229_d_n0: f64 = (eq9_e227_d_n0 * p.p1);
        let eq9_e229_d_n1: f64 = (eq9_e227_d_n1 * p.p1);
        let eq9_e229_d_n2: f64 = (eq9_e227_d_n2 * p.p1);
        let eq9_e229_d_n3: f64 = (eq9_e227_d_n3 * p.p1);
        let eq9_e229_d_n4: f64 = (eq9_e227_d_n4 * p.p1);
        let eq9_e229_d_n5: f64 = (eq9_e227_d_n5 * p.p1);
        let eq9_e229_d_n6: f64 = (eq9_e227_d_n6 * p.p1);
        let eq9_e229_d_n7: f64 = (eq9_e227_d_n7 * p.p1);
        let eq9_e229_d_n8: f64 = (eq9_e227_d_n8 * p.p1);
        let eq9_e229_d_n9: f64 = (eq9_e227_d_n9 * p.p1);
        let eq9_e229_d_n10: f64 = (eq9_e227_d_n10 * p.p1);
        let eq9_e229_d_b0: f64 = (eq9_e227_d_b0 * p.p1);
        let eq9_e229_d_b1: f64 = (eq9_e227_d_b1 * p.p1);
        let eq9_value: f64 = eq9_e229;
        let eq9_node_derivatives: [f64; 11] = [eq9_e229_d_n0, eq9_e229_d_n1, eq9_e229_d_n2, eq9_e229_d_n3, eq9_e229_d_n4, eq9_e229_d_n5, eq9_e229_d_n6, eq9_e229_d_n7, eq9_e229_d_n8, eq9_e229_d_n9, eq9_e229_d_n10];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e229_d_b0, eq9_e229_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e233: f64 = (s.v[204] + s.v[209]);
        let eq10_e233_d_n0: f64 = (s.dn[204][0] + s.dn[209][0]);
        let eq10_e233_d_n1: f64 = (s.dn[204][1] + s.dn[209][1]);
        let eq10_e233_d_n2: f64 = (s.dn[204][2] + s.dn[209][2]);
        let eq10_e233_d_n3: f64 = (s.dn[204][3] + s.dn[209][3]);
        let eq10_e233_d_n4: f64 = (s.dn[204][4] + s.dn[209][4]);
        let eq10_e233_d_n5: f64 = (s.dn[204][5] + s.dn[209][5]);
        let eq10_e233_d_n6: f64 = (s.dn[204][6] + s.dn[209][6]);
        let eq10_e233_d_n7: f64 = (s.dn[204][7] + s.dn[209][7]);
        let eq10_e233_d_n8: f64 = (s.dn[204][8] + s.dn[209][8]);
        let eq10_e233_d_n9: f64 = (s.dn[204][9] + s.dn[209][9]);
        let eq10_e233_d_n10: f64 = (s.dn[204][10] + s.dn[209][10]);
        let eq10_e233_d_b0: f64 = (s.db[204][0] + s.db[209][0]);
        let eq10_e233_d_b1: f64 = (s.db[204][1] + s.db[209][1]);
        let eq10_e235: f64 = (eq10_e233 + s.v[221]);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + s.dn[221][0]);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + s.dn[221][1]);
        let eq10_e235_d_n2: f64 = (eq10_e233_d_n2 + s.dn[221][2]);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + s.dn[221][3]);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + s.dn[221][4]);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + s.dn[221][5]);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + s.dn[221][6]);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + s.dn[221][7]);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + s.dn[221][8]);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + s.dn[221][9]);
        let eq10_e235_d_n10: f64 = (eq10_e233_d_n10 + s.dn[221][10]);
        let eq10_e235_d_b0: f64 = (eq10_e233_d_b0 + s.db[221][0]);
        let eq10_e235_d_b1: f64 = (eq10_e233_d_b1 + s.db[221][1]);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n2: f64 = (p.p3 * eq10_e235_d_n2);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e236_d_n10: f64 = (p.p3 * eq10_e235_d_n10);
        let eq10_e236_d_b0: f64 = (p.p3 * eq10_e235_d_b0);
        let eq10_e236_d_b1: f64 = (p.p3 * eq10_e235_d_b1);
        let eq10_e237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq10_e236);
        let eq10_e237_d_n0: f64 = (eq10_e236_d_n0 * ddt_scale);
        let eq10_e237_d_n1: f64 = (eq10_e236_d_n1 * ddt_scale);
        let eq10_e237_d_n2: f64 = (eq10_e236_d_n2 * ddt_scale);
        let eq10_e237_d_n3: f64 = (eq10_e236_d_n3 * ddt_scale);
        let eq10_e237_d_n4: f64 = (eq10_e236_d_n4 * ddt_scale);
        let eq10_e237_d_n5: f64 = (eq10_e236_d_n5 * ddt_scale);
        let eq10_e237_d_n6: f64 = (eq10_e236_d_n6 * ddt_scale);
        let eq10_e237_d_n7: f64 = (eq10_e236_d_n7 * ddt_scale);
        let eq10_e237_d_n8: f64 = (eq10_e236_d_n8 * ddt_scale);
        let eq10_e237_d_n9: f64 = (eq10_e236_d_n9 * ddt_scale);
        let eq10_e237_d_n10: f64 = (eq10_e236_d_n10 * ddt_scale);
        let eq10_e237_d_b0: f64 = (eq10_e236_d_b0 * ddt_scale);
        let eq10_e237_d_b1: f64 = (eq10_e236_d_b1 * ddt_scale);
        let eq10_e239: f64 = (eq10_e237 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e237_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e237_d_n1 * p.p1);
        let eq10_e239_d_n2: f64 = (eq10_e237_d_n2 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e237_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e237_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e237_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e237_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e237_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e237_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e237_d_n9 * p.p1);
        let eq10_e239_d_n10: f64 = (eq10_e237_d_n10 * p.p1);
        let eq10_e239_d_b0: f64 = (eq10_e237_d_b0 * p.p1);
        let eq10_e239_d_b1: f64 = (eq10_e237_d_b1 * p.p1);
        let eq10_value: f64 = eq10_e239;
        let eq10_node_derivatives: [f64; 11] = [eq10_e239_d_n0, eq10_e239_d_n1, eq10_e239_d_n2, eq10_e239_d_n3, eq10_e239_d_n4, eq10_e239_d_n5, eq10_e239_d_n6, eq10_e239_d_n7, eq10_e239_d_n8, eq10_e239_d_n9, eq10_e239_d_n10];
        let eq10_branch_derivatives: [f64; 2] = [eq10_e239_d_b0, eq10_e239_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e242: f64 = (p.p3 * s.v[206]);
        let eq11_e242_d_n0: f64 = (p.p3 * s.dn[206][0]);
        let eq11_e242_d_n1: f64 = (p.p3 * s.dn[206][1]);
        let eq11_e242_d_n2: f64 = (p.p3 * s.dn[206][2]);
        let eq11_e242_d_n3: f64 = (p.p3 * s.dn[206][3]);
        let eq11_e242_d_n4: f64 = (p.p3 * s.dn[206][4]);
        let eq11_e242_d_n5: f64 = (p.p3 * s.dn[206][5]);
        let eq11_e242_d_n6: f64 = (p.p3 * s.dn[206][6]);
        let eq11_e242_d_n7: f64 = (p.p3 * s.dn[206][7]);
        let eq11_e242_d_n8: f64 = (p.p3 * s.dn[206][8]);
        let eq11_e242_d_n9: f64 = (p.p3 * s.dn[206][9]);
        let eq11_e242_d_n10: f64 = (p.p3 * s.dn[206][10]);
        let eq11_e242_d_b0: f64 = (p.p3 * s.db[206][0]);
        let eq11_e242_d_b1: f64 = (p.p3 * s.db[206][1]);
        let eq11_e243: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq11_e242);
        let eq11_e243_d_n0: f64 = (eq11_e242_d_n0 * ddt_scale);
        let eq11_e243_d_n1: f64 = (eq11_e242_d_n1 * ddt_scale);
        let eq11_e243_d_n2: f64 = (eq11_e242_d_n2 * ddt_scale);
        let eq11_e243_d_n3: f64 = (eq11_e242_d_n3 * ddt_scale);
        let eq11_e243_d_n4: f64 = (eq11_e242_d_n4 * ddt_scale);
        let eq11_e243_d_n5: f64 = (eq11_e242_d_n5 * ddt_scale);
        let eq11_e243_d_n6: f64 = (eq11_e242_d_n6 * ddt_scale);
        let eq11_e243_d_n7: f64 = (eq11_e242_d_n7 * ddt_scale);
        let eq11_e243_d_n8: f64 = (eq11_e242_d_n8 * ddt_scale);
        let eq11_e243_d_n9: f64 = (eq11_e242_d_n9 * ddt_scale);
        let eq11_e243_d_n10: f64 = (eq11_e242_d_n10 * ddt_scale);
        let eq11_e243_d_b0: f64 = (eq11_e242_d_b0 * ddt_scale);
        let eq11_e243_d_b1: f64 = (eq11_e242_d_b1 * ddt_scale);
        let eq11_e245: f64 = (eq11_e243 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e243_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e243_d_n1 * p.p1);
        let eq11_e245_d_n2: f64 = (eq11_e243_d_n2 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e243_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e243_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e243_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e243_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e243_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e243_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e243_d_n9 * p.p1);
        let eq11_e245_d_n10: f64 = (eq11_e243_d_n10 * p.p1);
        let eq11_e245_d_b0: f64 = (eq11_e243_d_b0 * p.p1);
        let eq11_e245_d_b1: f64 = (eq11_e243_d_b1 * p.p1);
        let eq11_value: f64 = eq11_e245;
        let eq11_node_derivatives: [f64; 11] = [eq11_e245_d_n0, eq11_e245_d_n1, eq11_e245_d_n2, eq11_e245_d_n3, eq11_e245_d_n4, eq11_e245_d_n5, eq11_e245_d_n6, eq11_e245_d_n7, eq11_e245_d_n8, eq11_e245_d_n9, eq11_e245_d_n10];
        let eq11_branch_derivatives: [f64; 2] = [eq11_e245_d_b0, eq11_e245_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(3),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e249: f64 = (s.v[207] + s.v[210]);
        let eq12_e249_d_n0: f64 = (s.dn[207][0] + s.dn[210][0]);
        let eq12_e249_d_n1: f64 = (s.dn[207][1] + s.dn[210][1]);
        let eq12_e249_d_n2: f64 = (s.dn[207][2] + s.dn[210][2]);
        let eq12_e249_d_n3: f64 = (s.dn[207][3] + s.dn[210][3]);
        let eq12_e249_d_n4: f64 = (s.dn[207][4] + s.dn[210][4]);
        let eq12_e249_d_n5: f64 = (s.dn[207][5] + s.dn[210][5]);
        let eq12_e249_d_n6: f64 = (s.dn[207][6] + s.dn[210][6]);
        let eq12_e249_d_n7: f64 = (s.dn[207][7] + s.dn[210][7]);
        let eq12_e249_d_n8: f64 = (s.dn[207][8] + s.dn[210][8]);
        let eq12_e249_d_n9: f64 = (s.dn[207][9] + s.dn[210][9]);
        let eq12_e249_d_n10: f64 = (s.dn[207][10] + s.dn[210][10]);
        let eq12_e249_d_b0: f64 = (s.db[207][0] + s.db[210][0]);
        let eq12_e249_d_b1: f64 = (s.db[207][1] + s.db[210][1]);
        let eq12_e251: f64 = (eq12_e249 + s.v[224]);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + s.dn[224][0]);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + s.dn[224][1]);
        let eq12_e251_d_n2: f64 = (eq12_e249_d_n2 + s.dn[224][2]);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + s.dn[224][3]);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + s.dn[224][4]);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + s.dn[224][5]);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + s.dn[224][6]);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + s.dn[224][7]);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + s.dn[224][8]);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + s.dn[224][9]);
        let eq12_e251_d_n10: f64 = (eq12_e249_d_n10 + s.dn[224][10]);
        let eq12_e251_d_b0: f64 = (eq12_e249_d_b0 + s.db[224][0]);
        let eq12_e251_d_b1: f64 = (eq12_e249_d_b1 + s.db[224][1]);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n2: f64 = (p.p3 * eq12_e251_d_n2);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e252_d_n10: f64 = (p.p3 * eq12_e251_d_n10);
        let eq12_e252_d_b0: f64 = (p.p3 * eq12_e251_d_b0);
        let eq12_e252_d_b1: f64 = (p.p3 * eq12_e251_d_b1);
        let eq12_e253: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq12_e252);
        let eq12_e253_d_n0: f64 = (eq12_e252_d_n0 * ddt_scale);
        let eq12_e253_d_n1: f64 = (eq12_e252_d_n1 * ddt_scale);
        let eq12_e253_d_n2: f64 = (eq12_e252_d_n2 * ddt_scale);
        let eq12_e253_d_n3: f64 = (eq12_e252_d_n3 * ddt_scale);
        let eq12_e253_d_n4: f64 = (eq12_e252_d_n4 * ddt_scale);
        let eq12_e253_d_n5: f64 = (eq12_e252_d_n5 * ddt_scale);
        let eq12_e253_d_n6: f64 = (eq12_e252_d_n6 * ddt_scale);
        let eq12_e253_d_n7: f64 = (eq12_e252_d_n7 * ddt_scale);
        let eq12_e253_d_n8: f64 = (eq12_e252_d_n8 * ddt_scale);
        let eq12_e253_d_n9: f64 = (eq12_e252_d_n9 * ddt_scale);
        let eq12_e253_d_n10: f64 = (eq12_e252_d_n10 * ddt_scale);
        let eq12_e253_d_b0: f64 = (eq12_e252_d_b0 * ddt_scale);
        let eq12_e253_d_b1: f64 = (eq12_e252_d_b1 * ddt_scale);
        let eq12_e255: f64 = (eq12_e253 * p.p1);
        let eq12_e255_d_n0: f64 = (eq12_e253_d_n0 * p.p1);
        let eq12_e255_d_n1: f64 = (eq12_e253_d_n1 * p.p1);
        let eq12_e255_d_n2: f64 = (eq12_e253_d_n2 * p.p1);
        let eq12_e255_d_n3: f64 = (eq12_e253_d_n3 * p.p1);
        let eq12_e255_d_n4: f64 = (eq12_e253_d_n4 * p.p1);
        let eq12_e255_d_n5: f64 = (eq12_e253_d_n5 * p.p1);
        let eq12_e255_d_n6: f64 = (eq12_e253_d_n6 * p.p1);
        let eq12_e255_d_n7: f64 = (eq12_e253_d_n7 * p.p1);
        let eq12_e255_d_n8: f64 = (eq12_e253_d_n8 * p.p1);
        let eq12_e255_d_n9: f64 = (eq12_e253_d_n9 * p.p1);
        let eq12_e255_d_n10: f64 = (eq12_e253_d_n10 * p.p1);
        let eq12_e255_d_b0: f64 = (eq12_e253_d_b0 * p.p1);
        let eq12_e255_d_b1: f64 = (eq12_e253_d_b1 * p.p1);
        let eq12_value: f64 = eq12_e255;
        let eq12_node_derivatives: [f64; 11] = [eq12_e255_d_n0, eq12_e255_d_n1, eq12_e255_d_n2, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, eq12_e255_d_n10];
        let eq12_branch_derivatives: [f64; 2] = [eq12_e255_d_b0, eq12_e255_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e258: f64 = (p.p3 * s.v[211]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[211][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[211][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[211][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[211][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[211][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[211][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[211][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[211][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[211][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[211][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[211][10]);
        let eq13_e258_d_b0: f64 = (p.p3 * s.db[211][0]);
        let eq13_e258_d_b1: f64 = (p.p3 * s.db[211][1]);
        let eq13_e259: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq13_e258);
        let eq13_e259_d_n0: f64 = (eq13_e258_d_n0 * ddt_scale);
        let eq13_e259_d_n1: f64 = (eq13_e258_d_n1 * ddt_scale);
        let eq13_e259_d_n2: f64 = (eq13_e258_d_n2 * ddt_scale);
        let eq13_e259_d_n3: f64 = (eq13_e258_d_n3 * ddt_scale);
        let eq13_e259_d_n4: f64 = (eq13_e258_d_n4 * ddt_scale);
        let eq13_e259_d_n5: f64 = (eq13_e258_d_n5 * ddt_scale);
        let eq13_e259_d_n6: f64 = (eq13_e258_d_n6 * ddt_scale);
        let eq13_e259_d_n7: f64 = (eq13_e258_d_n7 * ddt_scale);
        let eq13_e259_d_n8: f64 = (eq13_e258_d_n8 * ddt_scale);
        let eq13_e259_d_n9: f64 = (eq13_e258_d_n9 * ddt_scale);
        let eq13_e259_d_n10: f64 = (eq13_e258_d_n10 * ddt_scale);
        let eq13_e259_d_b0: f64 = (eq13_e258_d_b0 * ddt_scale);
        let eq13_e259_d_b1: f64 = (eq13_e258_d_b1 * ddt_scale);
        let eq13_e261: f64 = (eq13_e259 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e259_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e259_d_n1 * p.p1);
        let eq13_e261_d_n2: f64 = (eq13_e259_d_n2 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e259_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e259_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e259_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e259_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e259_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e259_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e259_d_n9 * p.p1);
        let eq13_e261_d_n10: f64 = (eq13_e259_d_n10 * p.p1);
        let eq13_e261_d_b0: f64 = (eq13_e259_d_b0 * p.p1);
        let eq13_e261_d_b1: f64 = (eq13_e259_d_b1 * p.p1);
        let eq13_value: f64 = eq13_e261;
        let eq13_node_derivatives: [f64; 11] = [eq13_e261_d_n0, eq13_e261_d_n1, eq13_e261_d_n2, eq13_e261_d_n3, eq13_e261_d_n4, eq13_e261_d_n5, eq13_e261_d_n6, eq13_e261_d_n7, eq13_e261_d_n8, eq13_e261_d_n9, eq13_e261_d_n10];
        let eq13_branch_derivatives: [f64; 2] = [eq13_e261_d_b0, eq13_e261_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(5),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * s.v[243]);
        let eq14_e266_d_n0: f64 = (eq14_e264 * s.dn[243][0]);
        let eq14_e266_d_n1: f64 = (eq14_e264 * s.dn[243][1]);
        let eq14_e266_d_n2: f64 = (eq14_e264 * s.dn[243][2]);
        let eq14_e266_d_n3: f64 = (eq14_e264 * s.dn[243][3]);
        let eq14_e266_d_n4: f64 = (eq14_e264 * s.dn[243][4]);
        let eq14_e266_d_n5: f64 = (eq14_e264 * s.dn[243][5]);
        let eq14_e266_d_n6: f64 = (eq14_e264 * s.dn[243][6]);
        let eq14_e266_d_n7: f64 = (eq14_e264 * s.dn[243][7]);
        let eq14_e266_d_n8: f64 = (eq14_e264 * s.dn[243][8]);
        let eq14_e266_d_n9: f64 = (eq14_e264 * s.dn[243][9]);
        let eq14_e266_d_n10: f64 = (eq14_e264 * s.dn[243][10]);
        let eq14_e266_d_b0: f64 = (eq14_e264 * s.db[243][0]);
        let eq14_e266_d_b1: f64 = (eq14_e264 * s.db[243][1]);
        let eq14_e267: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq14_e266);
        let eq14_e267_d_n0: f64 = (eq14_e266_d_n0 * ddt_scale);
        let eq14_e267_d_n1: f64 = (eq14_e266_d_n1 * ddt_scale);
        let eq14_e267_d_n2: f64 = (eq14_e266_d_n2 * ddt_scale);
        let eq14_e267_d_n3: f64 = (eq14_e266_d_n3 * ddt_scale);
        let eq14_e267_d_n4: f64 = (eq14_e266_d_n4 * ddt_scale);
        let eq14_e267_d_n5: f64 = (eq14_e266_d_n5 * ddt_scale);
        let eq14_e267_d_n6: f64 = (eq14_e266_d_n6 * ddt_scale);
        let eq14_e267_d_n7: f64 = (eq14_e266_d_n7 * ddt_scale);
        let eq14_e267_d_n8: f64 = (eq14_e266_d_n8 * ddt_scale);
        let eq14_e267_d_n9: f64 = (eq14_e266_d_n9 * ddt_scale);
        let eq14_e267_d_n10: f64 = (eq14_e266_d_n10 * ddt_scale);
        let eq14_e267_d_b0: f64 = (eq14_e266_d_b0 * ddt_scale);
        let eq14_e267_d_b1: f64 = (eq14_e266_d_b1 * ddt_scale);
        let eq14_e269: f64 = (eq14_e267 * p.p1);
        let eq14_e269_d_n0: f64 = (eq14_e267_d_n0 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e267_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e267_d_n2 * p.p1);
        let eq14_e269_d_n3: f64 = (eq14_e267_d_n3 * p.p1);
        let eq14_e269_d_n4: f64 = (eq14_e267_d_n4 * p.p1);
        let eq14_e269_d_n5: f64 = (eq14_e267_d_n5 * p.p1);
        let eq14_e269_d_n6: f64 = (eq14_e267_d_n6 * p.p1);
        let eq14_e269_d_n7: f64 = (eq14_e267_d_n7 * p.p1);
        let eq14_e269_d_n8: f64 = (eq14_e267_d_n8 * p.p1);
        let eq14_e269_d_n9: f64 = (eq14_e267_d_n9 * p.p1);
        let eq14_e269_d_n10: f64 = (eq14_e267_d_n10 * p.p1);
        let eq14_e269_d_b0: f64 = (eq14_e267_d_b0 * p.p1);
        let eq14_e269_d_b1: f64 = (eq14_e267_d_b1 * p.p1);
        let eq14_value: f64 = eq14_e269;
        let eq14_node_derivatives: [f64; 11] = [eq14_e269_d_n0, eq14_e269_d_n1, eq14_e269_d_n2, eq14_e269_d_n3, eq14_e269_d_n4, eq14_e269_d_n5, eq14_e269_d_n6, eq14_e269_d_n7, eq14_e269_d_n8, eq14_e269_d_n9, eq14_e269_d_n10];
        let eq14_branch_derivatives: [f64; 2] = [eq14_e269_d_b0, eq14_e269_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * s.v[244]);
        let eq15_e274_d_n0: f64 = (eq15_e272 * s.dn[244][0]);
        let eq15_e274_d_n1: f64 = (eq15_e272 * s.dn[244][1]);
        let eq15_e274_d_n2: f64 = (eq15_e272 * s.dn[244][2]);
        let eq15_e274_d_n3: f64 = (eq15_e272 * s.dn[244][3]);
        let eq15_e274_d_n4: f64 = (eq15_e272 * s.dn[244][4]);
        let eq15_e274_d_n5: f64 = (eq15_e272 * s.dn[244][5]);
        let eq15_e274_d_n6: f64 = (eq15_e272 * s.dn[244][6]);
        let eq15_e274_d_n7: f64 = (eq15_e272 * s.dn[244][7]);
        let eq15_e274_d_n8: f64 = (eq15_e272 * s.dn[244][8]);
        let eq15_e274_d_n9: f64 = (eq15_e272 * s.dn[244][9]);
        let eq15_e274_d_n10: f64 = (eq15_e272 * s.dn[244][10]);
        let eq15_e274_d_b0: f64 = (eq15_e272 * s.db[244][0]);
        let eq15_e274_d_b1: f64 = (eq15_e272 * s.db[244][1]);
        let eq15_e275: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq15_e274);
        let eq15_e275_d_n0: f64 = (eq15_e274_d_n0 * ddt_scale);
        let eq15_e275_d_n1: f64 = (eq15_e274_d_n1 * ddt_scale);
        let eq15_e275_d_n2: f64 = (eq15_e274_d_n2 * ddt_scale);
        let eq15_e275_d_n3: f64 = (eq15_e274_d_n3 * ddt_scale);
        let eq15_e275_d_n4: f64 = (eq15_e274_d_n4 * ddt_scale);
        let eq15_e275_d_n5: f64 = (eq15_e274_d_n5 * ddt_scale);
        let eq15_e275_d_n6: f64 = (eq15_e274_d_n6 * ddt_scale);
        let eq15_e275_d_n7: f64 = (eq15_e274_d_n7 * ddt_scale);
        let eq15_e275_d_n8: f64 = (eq15_e274_d_n8 * ddt_scale);
        let eq15_e275_d_n9: f64 = (eq15_e274_d_n9 * ddt_scale);
        let eq15_e275_d_n10: f64 = (eq15_e274_d_n10 * ddt_scale);
        let eq15_e275_d_b0: f64 = (eq15_e274_d_b0 * ddt_scale);
        let eq15_e275_d_b1: f64 = (eq15_e274_d_b1 * ddt_scale);
        let eq15_e277: f64 = (eq15_e275 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e277_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e277_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e277_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e277_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e277_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e277_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e277_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e277_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e277_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e277_d_b0: f64 = (eq15_e275_d_b0 * p.p1);
        let eq15_e277_d_b1: f64 = (eq15_e275_d_b1 * p.p1);
        let eq15_value: f64 = eq15_e277;
        let eq15_node_derivatives: [f64; 11] = [eq15_e277_d_n0, eq15_e277_d_n1, eq15_e277_d_n2, eq15_e277_d_n3, eq15_e277_d_n4, eq15_e277_d_n5, eq15_e277_d_n6, eq15_e277_d_n7, eq15_e277_d_n8, eq15_e277_d_n9, eq15_e277_d_n10];
        let eq15_branch_derivatives: [f64; 2] = [eq15_e277_d_b0, eq15_e277_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq16_e280: f64 = (p.p3 * s.v[169]);
        let eq16_e280_d_n0: f64 = (p.p3 * s.dn[169][0]);
        let eq16_e280_d_n1: f64 = (p.p3 * s.dn[169][1]);
        let eq16_e280_d_n2: f64 = (p.p3 * s.dn[169][2]);
        let eq16_e280_d_n3: f64 = (p.p3 * s.dn[169][3]);
        let eq16_e280_d_n4: f64 = (p.p3 * s.dn[169][4]);
        let eq16_e280_d_n5: f64 = (p.p3 * s.dn[169][5]);
        let eq16_e280_d_n6: f64 = (p.p3 * s.dn[169][6]);
        let eq16_e280_d_n7: f64 = (p.p3 * s.dn[169][7]);
        let eq16_e280_d_n8: f64 = (p.p3 * s.dn[169][8]);
        let eq16_e280_d_n9: f64 = (p.p3 * s.dn[169][9]);
        let eq16_e280_d_n10: f64 = (p.p3 * s.dn[169][10]);
        let eq16_e280_d_b0: f64 = (p.p3 * s.db[169][0]);
        let eq16_e280_d_b1: f64 = (p.p3 * s.db[169][1]);
        let eq16_e282: f64 = (eq16_e280 * p.p1);
        let eq16_e282_d_n0: f64 = (eq16_e280_d_n0 * p.p1);
        let eq16_e282_d_n1: f64 = (eq16_e280_d_n1 * p.p1);
        let eq16_e282_d_n2: f64 = (eq16_e280_d_n2 * p.p1);
        let eq16_e282_d_n3: f64 = (eq16_e280_d_n3 * p.p1);
        let eq16_e282_d_n4: f64 = (eq16_e280_d_n4 * p.p1);
        let eq16_e282_d_n5: f64 = (eq16_e280_d_n5 * p.p1);
        let eq16_e282_d_n6: f64 = (eq16_e280_d_n6 * p.p1);
        let eq16_e282_d_n7: f64 = (eq16_e280_d_n7 * p.p1);
        let eq16_e282_d_n8: f64 = (eq16_e280_d_n8 * p.p1);
        let eq16_e282_d_n9: f64 = (eq16_e280_d_n9 * p.p1);
        let eq16_e282_d_n10: f64 = (eq16_e280_d_n10 * p.p1);
        let eq16_e282_d_b0: f64 = (eq16_e280_d_b0 * p.p1);
        let eq16_e282_d_b1: f64 = (eq16_e280_d_b1 * p.p1);
        let eq16_value: f64 = eq16_e282;
        let eq16_node_derivatives: [f64; 11] = [eq16_e282_d_n0, eq16_e282_d_n1, eq16_e282_d_n2, eq16_e282_d_n3, eq16_e282_d_n4, eq16_e282_d_n5, eq16_e282_d_n6, eq16_e282_d_n7, eq16_e282_d_n8, eq16_e282_d_n9, eq16_e282_d_n10];
        let eq16_branch_derivatives: [f64; 2] = [eq16_e282_d_b0, eq16_e282_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(8),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e285: f64 = (p.p3 * s.v[242]);
        let eq17_e285_d_n0: f64 = (p.p3 * s.dn[242][0]);
        let eq17_e285_d_n1: f64 = (p.p3 * s.dn[242][1]);
        let eq17_e285_d_n2: f64 = (p.p3 * s.dn[242][2]);
        let eq17_e285_d_n3: f64 = (p.p3 * s.dn[242][3]);
        let eq17_e285_d_n4: f64 = (p.p3 * s.dn[242][4]);
        let eq17_e285_d_n5: f64 = (p.p3 * s.dn[242][5]);
        let eq17_e285_d_n6: f64 = (p.p3 * s.dn[242][6]);
        let eq17_e285_d_n7: f64 = (p.p3 * s.dn[242][7]);
        let eq17_e285_d_n8: f64 = (p.p3 * s.dn[242][8]);
        let eq17_e285_d_n9: f64 = (p.p3 * s.dn[242][9]);
        let eq17_e285_d_n10: f64 = (p.p3 * s.dn[242][10]);
        let eq17_e285_d_b0: f64 = (p.p3 * s.db[242][0]);
        let eq17_e285_d_b1: f64 = (p.p3 * s.db[242][1]);
        let eq17_e287: f64 = (eq17_e285 * s.v[101]);
        let eq17_e287_d_n0: f64 = ((eq17_e285_d_n0 * s.v[101]) + (eq17_e285 * s.dn[101][0]));
        let eq17_e287_d_n1: f64 = ((eq17_e285_d_n1 * s.v[101]) + (eq17_e285 * s.dn[101][1]));
        let eq17_e287_d_n2: f64 = ((eq17_e285_d_n2 * s.v[101]) + (eq17_e285 * s.dn[101][2]));
        let eq17_e287_d_n3: f64 = ((eq17_e285_d_n3 * s.v[101]) + (eq17_e285 * s.dn[101][3]));
        let eq17_e287_d_n4: f64 = ((eq17_e285_d_n4 * s.v[101]) + (eq17_e285 * s.dn[101][4]));
        let eq17_e287_d_n5: f64 = ((eq17_e285_d_n5 * s.v[101]) + (eq17_e285 * s.dn[101][5]));
        let eq17_e287_d_n6: f64 = ((eq17_e285_d_n6 * s.v[101]) + (eq17_e285 * s.dn[101][6]));
        let eq17_e287_d_n7: f64 = ((eq17_e285_d_n7 * s.v[101]) + (eq17_e285 * s.dn[101][7]));
        let eq17_e287_d_n8: f64 = ((eq17_e285_d_n8 * s.v[101]) + (eq17_e285 * s.dn[101][8]));
        let eq17_e287_d_n9: f64 = ((eq17_e285_d_n9 * s.v[101]) + (eq17_e285 * s.dn[101][9]));
        let eq17_e287_d_n10: f64 = ((eq17_e285_d_n10 * s.v[101]) + (eq17_e285 * s.dn[101][10]));
        let eq17_e287_d_b0: f64 = ((eq17_e285_d_b0 * s.v[101]) + (eq17_e285 * s.db[101][0]));
        let eq17_e287_d_b1: f64 = ((eq17_e285_d_b1 * s.v[101]) + (eq17_e285 * s.db[101][1]));
        let eq17_e289: f64 = (eq17_e287 * p.p1);
        let eq17_e289_d_n0: f64 = (eq17_e287_d_n0 * p.p1);
        let eq17_e289_d_n1: f64 = (eq17_e287_d_n1 * p.p1);
        let eq17_e289_d_n2: f64 = (eq17_e287_d_n2 * p.p1);
        let eq17_e289_d_n3: f64 = (eq17_e287_d_n3 * p.p1);
        let eq17_e289_d_n4: f64 = (eq17_e287_d_n4 * p.p1);
        let eq17_e289_d_n5: f64 = (eq17_e287_d_n5 * p.p1);
        let eq17_e289_d_n6: f64 = (eq17_e287_d_n6 * p.p1);
        let eq17_e289_d_n7: f64 = (eq17_e287_d_n7 * p.p1);
        let eq17_e289_d_n8: f64 = (eq17_e287_d_n8 * p.p1);
        let eq17_e289_d_n9: f64 = (eq17_e287_d_n9 * p.p1);
        let eq17_e289_d_n10: f64 = (eq17_e287_d_n10 * p.p1);
        let eq17_e289_d_b0: f64 = (eq17_e287_d_b0 * p.p1);
        let eq17_e289_d_b1: f64 = (eq17_e287_d_b1 * p.p1);
        let eq17_value: f64 = eq17_e289;
        let eq17_node_derivatives: [f64; 11] = [eq17_e289_d_n0, eq17_e289_d_n1, eq17_e289_d_n2, eq17_e289_d_n3, eq17_e289_d_n4, eq17_e289_d_n5, eq17_e289_d_n6, eq17_e289_d_n7, eq17_e289_d_n8, eq17_e289_d_n9, eq17_e289_d_n10];
        let eq17_branch_derivatives: [f64; 2] = [eq17_e289_d_b0, eq17_e289_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(8),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e293: f64 = (s.v[219] + s.v[228]);
        let eq18_e293_d_n0: f64 = (s.dn[219][0] + s.dn[228][0]);
        let eq18_e293_d_n1: f64 = (s.dn[219][1] + s.dn[228][1]);
        let eq18_e293_d_n2: f64 = (s.dn[219][2] + s.dn[228][2]);
        let eq18_e293_d_n3: f64 = (s.dn[219][3] + s.dn[228][3]);
        let eq18_e293_d_n4: f64 = (s.dn[219][4] + s.dn[228][4]);
        let eq18_e293_d_n5: f64 = (s.dn[219][5] + s.dn[228][5]);
        let eq18_e293_d_n6: f64 = (s.dn[219][6] + s.dn[228][6]);
        let eq18_e293_d_n7: f64 = (s.dn[219][7] + s.dn[228][7]);
        let eq18_e293_d_n8: f64 = (s.dn[219][8] + s.dn[228][8]);
        let eq18_e293_d_n9: f64 = (s.dn[219][9] + s.dn[228][9]);
        let eq18_e293_d_n10: f64 = (s.dn[219][10] + s.dn[228][10]);
        let eq18_e293_d_b0: f64 = (s.db[219][0] + s.db[228][0]);
        let eq18_e293_d_b1: f64 = (s.db[219][1] + s.db[228][1]);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n2: f64 = (p.p3 * eq18_e293_d_n2);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e294_d_n10: f64 = (p.p3 * eq18_e293_d_n10);
        let eq18_e294_d_b0: f64 = (p.p3 * eq18_e293_d_b0);
        let eq18_e294_d_b1: f64 = (p.p3 * eq18_e293_d_b1);
        let eq18_e295: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq18_e294);
        let eq18_e295_d_n0: f64 = (eq18_e294_d_n0 * ddt_scale);
        let eq18_e295_d_n1: f64 = (eq18_e294_d_n1 * ddt_scale);
        let eq18_e295_d_n2: f64 = (eq18_e294_d_n2 * ddt_scale);
        let eq18_e295_d_n3: f64 = (eq18_e294_d_n3 * ddt_scale);
        let eq18_e295_d_n4: f64 = (eq18_e294_d_n4 * ddt_scale);
        let eq18_e295_d_n5: f64 = (eq18_e294_d_n5 * ddt_scale);
        let eq18_e295_d_n6: f64 = (eq18_e294_d_n6 * ddt_scale);
        let eq18_e295_d_n7: f64 = (eq18_e294_d_n7 * ddt_scale);
        let eq18_e295_d_n8: f64 = (eq18_e294_d_n8 * ddt_scale);
        let eq18_e295_d_n9: f64 = (eq18_e294_d_n9 * ddt_scale);
        let eq18_e295_d_n10: f64 = (eq18_e294_d_n10 * ddt_scale);
        let eq18_e295_d_b0: f64 = (eq18_e294_d_b0 * ddt_scale);
        let eq18_e295_d_b1: f64 = (eq18_e294_d_b1 * ddt_scale);
        let eq18_e297: f64 = (eq18_e295 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e295_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e295_d_n1 * p.p1);
        let eq18_e297_d_n2: f64 = (eq18_e295_d_n2 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e295_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e295_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e295_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e295_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e295_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e295_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e295_d_n9 * p.p1);
        let eq18_e297_d_n10: f64 = (eq18_e295_d_n10 * p.p1);
        let eq18_e297_d_b0: f64 = (eq18_e295_d_b0 * p.p1);
        let eq18_e297_d_b1: f64 = (eq18_e295_d_b1 * p.p1);
        let eq18_value: f64 = eq18_e297;
        let eq18_node_derivatives: [f64; 11] = [eq18_e297_d_n0, eq18_e297_d_n1, eq18_e297_d_n2, eq18_e297_d_n3, eq18_e297_d_n4, eq18_e297_d_n5, eq18_e297_d_n6, eq18_e297_d_n7, eq18_e297_d_n8, eq18_e297_d_n9, eq18_e297_d_n10];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e297_d_b0, eq18_e297_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(8),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e302: f64 = (s.v[314] * s.v[235]);
        let eq19_e302_d_n0: f64 = (s.v[314] * s.dn[235][0]);
        let eq19_e302_d_n1: f64 = (s.v[314] * s.dn[235][1]);
        let eq19_e302_d_n2: f64 = (s.v[314] * s.dn[235][2]);
        let eq19_e302_d_n3: f64 = (s.v[314] * s.dn[235][3]);
        let eq19_e302_d_n4: f64 = (s.v[314] * s.dn[235][4]);
        let eq19_e302_d_n5: f64 = (s.v[314] * s.dn[235][5]);
        let eq19_e302_d_n6: f64 = (s.v[314] * s.dn[235][6]);
        let eq19_e302_d_n7: f64 = (s.v[314] * s.dn[235][7]);
        let eq19_e302_d_n8: f64 = (s.v[314] * s.dn[235][8]);
        let eq19_e302_d_n9: f64 = (s.v[314] * s.dn[235][9]);
        let eq19_e302_d_n10: f64 = (s.v[314] * s.dn[235][10]);
        let eq19_e302_d_b0: f64 = (s.v[314] * s.db[235][0]);
        let eq19_e302_d_b1: f64 = (s.v[314] * s.db[235][1]);
        let eq19_e303: f64 = (s.v[154] + eq19_e302);
        let eq19_e303_d_n0: f64 = (s.dn[154][0] + eq19_e302_d_n0);
        let eq19_e303_d_n1: f64 = (s.dn[154][1] + eq19_e302_d_n1);
        let eq19_e303_d_n2: f64 = (s.dn[154][2] + eq19_e302_d_n2);
        let eq19_e303_d_n3: f64 = (s.dn[154][3] + eq19_e302_d_n3);
        let eq19_e303_d_n4: f64 = (s.dn[154][4] + eq19_e302_d_n4);
        let eq19_e303_d_n5: f64 = (s.dn[154][5] + eq19_e302_d_n5);
        let eq19_e303_d_n6: f64 = (s.dn[154][6] + eq19_e302_d_n6);
        let eq19_e303_d_n7: f64 = (s.dn[154][7] + eq19_e302_d_n7);
        let eq19_e303_d_n8: f64 = (s.dn[154][8] + eq19_e302_d_n8);
        let eq19_e303_d_n9: f64 = (s.dn[154][9] + eq19_e302_d_n9);
        let eq19_e303_d_n10: f64 = (s.dn[154][10] + eq19_e302_d_n10);
        let eq19_e303_d_b0: f64 = (s.db[154][0] + eq19_e302_d_b0);
        let eq19_e303_d_b1: f64 = (s.db[154][1] + eq19_e302_d_b1);
        let eq19_e305: f64 = (eq19_e303 + s.v[157]);
        let eq19_e305_d_n0: f64 = (eq19_e303_d_n0 + s.dn[157][0]);
        let eq19_e305_d_n1: f64 = (eq19_e303_d_n1 + s.dn[157][1]);
        let eq19_e305_d_n2: f64 = (eq19_e303_d_n2 + s.dn[157][2]);
        let eq19_e305_d_n3: f64 = (eq19_e303_d_n3 + s.dn[157][3]);
        let eq19_e305_d_n4: f64 = (eq19_e303_d_n4 + s.dn[157][4]);
        let eq19_e305_d_n5: f64 = (eq19_e303_d_n5 + s.dn[157][5]);
        let eq19_e305_d_n6: f64 = (eq19_e303_d_n6 + s.dn[157][6]);
        let eq19_e305_d_n7: f64 = (eq19_e303_d_n7 + s.dn[157][7]);
        let eq19_e305_d_n8: f64 = (eq19_e303_d_n8 + s.dn[157][8]);
        let eq19_e305_d_n9: f64 = (eq19_e303_d_n9 + s.dn[157][9]);
        let eq19_e305_d_n10: f64 = (eq19_e303_d_n10 + s.dn[157][10]);
        let eq19_e305_d_b0: f64 = (eq19_e303_d_b0 + s.db[157][0]);
        let eq19_e305_d_b1: f64 = (eq19_e303_d_b1 + s.db[157][1]);
        let eq19_e306: f64 = (p.p3 * eq19_e305);
        let eq19_e306_d_n0: f64 = (p.p3 * eq19_e305_d_n0);
        let eq19_e306_d_n1: f64 = (p.p3 * eq19_e305_d_n1);
        let eq19_e306_d_n2: f64 = (p.p3 * eq19_e305_d_n2);
        let eq19_e306_d_n3: f64 = (p.p3 * eq19_e305_d_n3);
        let eq19_e306_d_n4: f64 = (p.p3 * eq19_e305_d_n4);
        let eq19_e306_d_n5: f64 = (p.p3 * eq19_e305_d_n5);
        let eq19_e306_d_n6: f64 = (p.p3 * eq19_e305_d_n6);
        let eq19_e306_d_n7: f64 = (p.p3 * eq19_e305_d_n7);
        let eq19_e306_d_n8: f64 = (p.p3 * eq19_e305_d_n8);
        let eq19_e306_d_n9: f64 = (p.p3 * eq19_e305_d_n9);
        let eq19_e306_d_n10: f64 = (p.p3 * eq19_e305_d_n10);
        let eq19_e306_d_b0: f64 = (p.p3 * eq19_e305_d_b0);
        let eq19_e306_d_b1: f64 = (p.p3 * eq19_e305_d_b1);
        let eq19_e308: f64 = (eq19_e306 * p.p1);
        let eq19_e308_d_n0: f64 = (eq19_e306_d_n0 * p.p1);
        let eq19_e308_d_n1: f64 = (eq19_e306_d_n1 * p.p1);
        let eq19_e308_d_n2: f64 = (eq19_e306_d_n2 * p.p1);
        let eq19_e308_d_n3: f64 = (eq19_e306_d_n3 * p.p1);
        let eq19_e308_d_n4: f64 = (eq19_e306_d_n4 * p.p1);
        let eq19_e308_d_n5: f64 = (eq19_e306_d_n5 * p.p1);
        let eq19_e308_d_n6: f64 = (eq19_e306_d_n6 * p.p1);
        let eq19_e308_d_n7: f64 = (eq19_e306_d_n7 * p.p1);
        let eq19_e308_d_n8: f64 = (eq19_e306_d_n8 * p.p1);
        let eq19_e308_d_n9: f64 = (eq19_e306_d_n9 * p.p1);
        let eq19_e308_d_n10: f64 = (eq19_e306_d_n10 * p.p1);
        let eq19_e308_d_b0: f64 = (eq19_e306_d_b0 * p.p1);
        let eq19_e308_d_b1: f64 = (eq19_e306_d_b1 * p.p1);
        let eq19_value: f64 = eq19_e308;
        let eq19_node_derivatives: [f64; 11] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10];
        let eq19_branch_derivatives: [f64; 2] = [eq19_e308_d_b0, eq19_e308_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(9),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e312: f64 = (s.v[216] + s.v[229]);
        let eq20_e312_d_n0: f64 = (s.dn[216][0] + s.dn[229][0]);
        let eq20_e312_d_n1: f64 = (s.dn[216][1] + s.dn[229][1]);
        let eq20_e312_d_n2: f64 = (s.dn[216][2] + s.dn[229][2]);
        let eq20_e312_d_n3: f64 = (s.dn[216][3] + s.dn[229][3]);
        let eq20_e312_d_n4: f64 = (s.dn[216][4] + s.dn[229][4]);
        let eq20_e312_d_n5: f64 = (s.dn[216][5] + s.dn[229][5]);
        let eq20_e312_d_n6: f64 = (s.dn[216][6] + s.dn[229][6]);
        let eq20_e312_d_n7: f64 = (s.dn[216][7] + s.dn[229][7]);
        let eq20_e312_d_n8: f64 = (s.dn[216][8] + s.dn[229][8]);
        let eq20_e312_d_n9: f64 = (s.dn[216][9] + s.dn[229][9]);
        let eq20_e312_d_n10: f64 = (s.dn[216][10] + s.dn[229][10]);
        let eq20_e312_d_b0: f64 = (s.db[216][0] + s.db[229][0]);
        let eq20_e312_d_b1: f64 = (s.db[216][1] + s.db[229][1]);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n2: f64 = (p.p3 * eq20_e312_d_n2);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e313_d_n10: f64 = (p.p3 * eq20_e312_d_n10);
        let eq20_e313_d_b0: f64 = (p.p3 * eq20_e312_d_b0);
        let eq20_e313_d_b1: f64 = (p.p3 * eq20_e312_d_b1);
        let eq20_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq20_e313);
        let eq20_e314_d_n0: f64 = (eq20_e313_d_n0 * ddt_scale);
        let eq20_e314_d_n1: f64 = (eq20_e313_d_n1 * ddt_scale);
        let eq20_e314_d_n2: f64 = (eq20_e313_d_n2 * ddt_scale);
        let eq20_e314_d_n3: f64 = (eq20_e313_d_n3 * ddt_scale);
        let eq20_e314_d_n4: f64 = (eq20_e313_d_n4 * ddt_scale);
        let eq20_e314_d_n5: f64 = (eq20_e313_d_n5 * ddt_scale);
        let eq20_e314_d_n6: f64 = (eq20_e313_d_n6 * ddt_scale);
        let eq20_e314_d_n7: f64 = (eq20_e313_d_n7 * ddt_scale);
        let eq20_e314_d_n8: f64 = (eq20_e313_d_n8 * ddt_scale);
        let eq20_e314_d_n9: f64 = (eq20_e313_d_n9 * ddt_scale);
        let eq20_e314_d_n10: f64 = (eq20_e313_d_n10 * ddt_scale);
        let eq20_e314_d_b0: f64 = (eq20_e313_d_b0 * ddt_scale);
        let eq20_e314_d_b1: f64 = (eq20_e313_d_b1 * ddt_scale);
        let eq20_e316: f64 = (eq20_e314 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e314_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e314_d_n1 * p.p1);
        let eq20_e316_d_n2: f64 = (eq20_e314_d_n2 * p.p1);
        let eq20_e316_d_n3: f64 = (eq20_e314_d_n3 * p.p1);
        let eq20_e316_d_n4: f64 = (eq20_e314_d_n4 * p.p1);
        let eq20_e316_d_n5: f64 = (eq20_e314_d_n5 * p.p1);
        let eq20_e316_d_n6: f64 = (eq20_e314_d_n6 * p.p1);
        let eq20_e316_d_n7: f64 = (eq20_e314_d_n7 * p.p1);
        let eq20_e316_d_n8: f64 = (eq20_e314_d_n8 * p.p1);
        let eq20_e316_d_n9: f64 = (eq20_e314_d_n9 * p.p1);
        let eq20_e316_d_n10: f64 = (eq20_e314_d_n10 * p.p1);
        let eq20_e316_d_b0: f64 = (eq20_e314_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e314_d_b1 * p.p1);
        let eq20_value: f64 = eq20_e316;
        let eq20_node_derivatives: [f64; 11] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(9),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e326, eq21_e326_d_n0, eq21_e326_d_n1, eq21_e326_d_n2, eq21_e326_d_n3, eq21_e326_d_n4, eq21_e326_d_n5, eq21_e326_d_n6, eq21_e326_d_n7, eq21_e326_d_n8, eq21_e326_d_n9, eq21_e326_d_n10, eq21_e326_d_b0, eq21_e326_d_b1,) = {
    if s.b[553] {
        let eq21_e320: f64 = (p.p3 * s.v[237]);
        let eq21_e320_d_n0: f64 = (p.p3 * s.dn[237][0]);
        let eq21_e320_d_n1: f64 = (p.p3 * s.dn[237][1]);
        let eq21_e320_d_n2: f64 = (p.p3 * s.dn[237][2]);
        let eq21_e320_d_n3: f64 = (p.p3 * s.dn[237][3]);
        let eq21_e320_d_n4: f64 = (p.p3 * s.dn[237][4]);
        let eq21_e320_d_n5: f64 = (p.p3 * s.dn[237][5]);
        let eq21_e320_d_n6: f64 = (p.p3 * s.dn[237][6]);
        let eq21_e320_d_n7: f64 = (p.p3 * s.dn[237][7]);
        let eq21_e320_d_n8: f64 = (p.p3 * s.dn[237][8]);
        let eq21_e320_d_n9: f64 = (p.p3 * s.dn[237][9]);
        let eq21_e320_d_n10: f64 = (p.p3 * s.dn[237][10]);
        let eq21_e320_d_b0: f64 = (p.p3 * s.db[237][0]);
        let eq21_e320_d_b1: f64 = (p.p3 * s.db[237][1]);
        let eq21_e322: f64 = (eq21_e320 * s.v[102]);
        let eq21_e322_d_n0: f64 = ((eq21_e320_d_n0 * s.v[102]) + (eq21_e320 * s.dn[102][0]));
        let eq21_e322_d_n1: f64 = ((eq21_e320_d_n1 * s.v[102]) + (eq21_e320 * s.dn[102][1]));
        let eq21_e322_d_n2: f64 = ((eq21_e320_d_n2 * s.v[102]) + (eq21_e320 * s.dn[102][2]));
        let eq21_e322_d_n3: f64 = ((eq21_e320_d_n3 * s.v[102]) + (eq21_e320 * s.dn[102][3]));
        let eq21_e322_d_n4: f64 = ((eq21_e320_d_n4 * s.v[102]) + (eq21_e320 * s.dn[102][4]));
        let eq21_e322_d_n5: f64 = ((eq21_e320_d_n5 * s.v[102]) + (eq21_e320 * s.dn[102][5]));
        let eq21_e322_d_n6: f64 = ((eq21_e320_d_n6 * s.v[102]) + (eq21_e320 * s.dn[102][6]));
        let eq21_e322_d_n7: f64 = ((eq21_e320_d_n7 * s.v[102]) + (eq21_e320 * s.dn[102][7]));
        let eq21_e322_d_n8: f64 = ((eq21_e320_d_n8 * s.v[102]) + (eq21_e320 * s.dn[102][8]));
        let eq21_e322_d_n9: f64 = ((eq21_e320_d_n9 * s.v[102]) + (eq21_e320 * s.dn[102][9]));
        let eq21_e322_d_n10: f64 = ((eq21_e320_d_n10 * s.v[102]) + (eq21_e320 * s.dn[102][10]));
        let eq21_e322_d_b0: f64 = ((eq21_e320_d_b0 * s.v[102]) + (eq21_e320 * s.db[102][0]));
        let eq21_e322_d_b1: f64 = ((eq21_e320_d_b1 * s.v[102]) + (eq21_e320 * s.db[102][1]));
        let eq21_e324: f64 = (eq21_e322 * p.p1);
        let eq21_e324_d_n0: f64 = (eq21_e322_d_n0 * p.p1);
        let eq21_e324_d_n1: f64 = (eq21_e322_d_n1 * p.p1);
        let eq21_e324_d_n2: f64 = (eq21_e322_d_n2 * p.p1);
        let eq21_e324_d_n3: f64 = (eq21_e322_d_n3 * p.p1);
        let eq21_e324_d_n4: f64 = (eq21_e322_d_n4 * p.p1);
        let eq21_e324_d_n5: f64 = (eq21_e322_d_n5 * p.p1);
        let eq21_e324_d_n6: f64 = (eq21_e322_d_n6 * p.p1);
        let eq21_e324_d_n7: f64 = (eq21_e322_d_n7 * p.p1);
        let eq21_e324_d_n8: f64 = (eq21_e322_d_n8 * p.p1);
        let eq21_e324_d_n9: f64 = (eq21_e322_d_n9 * p.p1);
        let eq21_e324_d_n10: f64 = (eq21_e322_d_n10 * p.p1);
        let eq21_e324_d_b0: f64 = (eq21_e322_d_b0 * p.p1);
        let eq21_e324_d_b1: f64 = (eq21_e322_d_b1 * p.p1);
        (eq21_e324, eq21_e324_d_n0, eq21_e324_d_n1, eq21_e324_d_n2, eq21_e324_d_n3, eq21_e324_d_n4, eq21_e324_d_n5, eq21_e324_d_n6, eq21_e324_d_n7, eq21_e324_d_n8, eq21_e324_d_n9, eq21_e324_d_n10, eq21_e324_d_b0, eq21_e324_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e326;
        let eq21_node_derivatives: [f64; 11] = [eq21_e326_d_n0, eq21_e326_d_n1, eq21_e326_d_n2, eq21_e326_d_n3, eq21_e326_d_n4, eq21_e326_d_n5, eq21_e326_d_n6, eq21_e326_d_n7, eq21_e326_d_n8, eq21_e326_d_n9, eq21_e326_d_n10];
        let eq21_branch_derivatives: [f64; 2] = [eq21_e326_d_b0, eq21_e326_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e331,) = {
    if (!s.b[553]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e331;
        stamper.stamp_potential_const_local(
            0,
            eq22_value,
        );
        let (eq23_e341, eq23_e341_d_n0, eq23_e341_d_n1, eq23_e341_d_n2, eq23_e341_d_n3, eq23_e341_d_n4, eq23_e341_d_n5, eq23_e341_d_n6, eq23_e341_d_n7, eq23_e341_d_n8, eq23_e341_d_n9, eq23_e341_d_n10, eq23_e341_d_b0, eq23_e341_d_b1,) = {
    if s.b[554] {
        let eq23_e335: f64 = (p.p3 * s.v[238]);
        let eq23_e335_d_n0: f64 = (p.p3 * s.dn[238][0]);
        let eq23_e335_d_n1: f64 = (p.p3 * s.dn[238][1]);
        let eq23_e335_d_n2: f64 = (p.p3 * s.dn[238][2]);
        let eq23_e335_d_n3: f64 = (p.p3 * s.dn[238][3]);
        let eq23_e335_d_n4: f64 = (p.p3 * s.dn[238][4]);
        let eq23_e335_d_n5: f64 = (p.p3 * s.dn[238][5]);
        let eq23_e335_d_n6: f64 = (p.p3 * s.dn[238][6]);
        let eq23_e335_d_n7: f64 = (p.p3 * s.dn[238][7]);
        let eq23_e335_d_n8: f64 = (p.p3 * s.dn[238][8]);
        let eq23_e335_d_n9: f64 = (p.p3 * s.dn[238][9]);
        let eq23_e335_d_n10: f64 = (p.p3 * s.dn[238][10]);
        let eq23_e335_d_b0: f64 = (p.p3 * s.db[238][0]);
        let eq23_e335_d_b1: f64 = (p.p3 * s.db[238][1]);
        let eq23_e337: f64 = (eq23_e335 * s.v[103]);
        let eq23_e337_d_n0: f64 = ((eq23_e335_d_n0 * s.v[103]) + (eq23_e335 * s.dn[103][0]));
        let eq23_e337_d_n1: f64 = ((eq23_e335_d_n1 * s.v[103]) + (eq23_e335 * s.dn[103][1]));
        let eq23_e337_d_n2: f64 = ((eq23_e335_d_n2 * s.v[103]) + (eq23_e335 * s.dn[103][2]));
        let eq23_e337_d_n3: f64 = ((eq23_e335_d_n3 * s.v[103]) + (eq23_e335 * s.dn[103][3]));
        let eq23_e337_d_n4: f64 = ((eq23_e335_d_n4 * s.v[103]) + (eq23_e335 * s.dn[103][4]));
        let eq23_e337_d_n5: f64 = ((eq23_e335_d_n5 * s.v[103]) + (eq23_e335 * s.dn[103][5]));
        let eq23_e337_d_n6: f64 = ((eq23_e335_d_n6 * s.v[103]) + (eq23_e335 * s.dn[103][6]));
        let eq23_e337_d_n7: f64 = ((eq23_e335_d_n7 * s.v[103]) + (eq23_e335 * s.dn[103][7]));
        let eq23_e337_d_n8: f64 = ((eq23_e335_d_n8 * s.v[103]) + (eq23_e335 * s.dn[103][8]));
        let eq23_e337_d_n9: f64 = ((eq23_e335_d_n9 * s.v[103]) + (eq23_e335 * s.dn[103][9]));
        let eq23_e337_d_n10: f64 = ((eq23_e335_d_n10 * s.v[103]) + (eq23_e335 * s.dn[103][10]));
        let eq23_e337_d_b0: f64 = ((eq23_e335_d_b0 * s.v[103]) + (eq23_e335 * s.db[103][0]));
        let eq23_e337_d_b1: f64 = ((eq23_e335_d_b1 * s.v[103]) + (eq23_e335 * s.db[103][1]));
        let eq23_e339: f64 = (eq23_e337 * p.p1);
        let eq23_e339_d_n0: f64 = (eq23_e337_d_n0 * p.p1);
        let eq23_e339_d_n1: f64 = (eq23_e337_d_n1 * p.p1);
        let eq23_e339_d_n2: f64 = (eq23_e337_d_n2 * p.p1);
        let eq23_e339_d_n3: f64 = (eq23_e337_d_n3 * p.p1);
        let eq23_e339_d_n4: f64 = (eq23_e337_d_n4 * p.p1);
        let eq23_e339_d_n5: f64 = (eq23_e337_d_n5 * p.p1);
        let eq23_e339_d_n6: f64 = (eq23_e337_d_n6 * p.p1);
        let eq23_e339_d_n7: f64 = (eq23_e337_d_n7 * p.p1);
        let eq23_e339_d_n8: f64 = (eq23_e337_d_n8 * p.p1);
        let eq23_e339_d_n9: f64 = (eq23_e337_d_n9 * p.p1);
        let eq23_e339_d_n10: f64 = (eq23_e337_d_n10 * p.p1);
        let eq23_e339_d_b0: f64 = (eq23_e337_d_b0 * p.p1);
        let eq23_e339_d_b1: f64 = (eq23_e337_d_b1 * p.p1);
        (eq23_e339, eq23_e339_d_n0, eq23_e339_d_n1, eq23_e339_d_n2, eq23_e339_d_n3, eq23_e339_d_n4, eq23_e339_d_n5, eq23_e339_d_n6, eq23_e339_d_n7, eq23_e339_d_n8, eq23_e339_d_n9, eq23_e339_d_n10, eq23_e339_d_b0, eq23_e339_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e341;
        let eq23_node_derivatives: [f64; 11] = [eq23_e341_d_n0, eq23_e341_d_n1, eq23_e341_d_n2, eq23_e341_d_n3, eq23_e341_d_n4, eq23_e341_d_n5, eq23_e341_d_n6, eq23_e341_d_n7, eq23_e341_d_n8, eq23_e341_d_n9, eq23_e341_d_n10];
        let eq23_branch_derivatives: [f64; 2] = [eq23_e341_d_b0, eq23_e341_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e346,) = {
    if (!s.b[554]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e346;
        stamper.stamp_potential_const_local(
            1,
            eq24_value,
        );
        let eq25_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(10),
            None,
            multiplicity * (eq25_value),
        );
        let eq26_value: f64 = (nv10 - 0.0);
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq26_value),
            10,
            multiplicity * (1.0),
        );
        let eq27_e355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, (nv10 - 0.0));
        let eq27_e356: f64 = (s.v[306] * eq27_e355);
        let eq27_e356_d_n0: f64 = (s.dn[306][0] * eq27_e355);
        let eq27_e356_d_n1: f64 = (s.dn[306][1] * eq27_e355);
        let eq27_e356_d_n2: f64 = (s.dn[306][2] * eq27_e355);
        let eq27_e356_d_n3: f64 = (s.dn[306][3] * eq27_e355);
        let eq27_e356_d_n4: f64 = (s.dn[306][4] * eq27_e355);
        let eq27_e356_d_n5: f64 = (s.dn[306][5] * eq27_e355);
        let eq27_e356_d_n6: f64 = (s.dn[306][6] * eq27_e355);
        let eq27_e356_d_n7: f64 = (s.dn[306][7] * eq27_e355);
        let eq27_e356_d_n8: f64 = (s.dn[306][8] * eq27_e355);
        let eq27_e356_d_n9: f64 = (s.dn[306][9] * eq27_e355);
        let eq27_e356_d_n10: f64 = ((s.dn[306][10] * eq27_e355) + (s.v[306] * ddt_scale));
        let eq27_e356_d_b0: f64 = (s.db[306][0] * eq27_e355);
        let eq27_e356_d_b1: f64 = (s.db[306][1] * eq27_e355);
        let eq27_value: f64 = eq27_e356;
        let eq27_node_derivatives: [f64; 11] = [eq27_e356_d_n0, eq27_e356_d_n1, eq27_e356_d_n2, eq27_e356_d_n3, eq27_e356_d_n4, eq27_e356_d_n5, eq27_e356_d_n6, eq27_e356_d_n7, eq27_e356_d_n8, eq27_e356_d_n9, eq27_e356_d_n10];
        let eq27_branch_derivatives: [f64; 2] = [eq27_e356_d_b0, eq27_e356_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq28_e359: f64 = (s.v[304] * (nv10 - 0.0));
        let eq28_e359_d_n0: f64 = (s.dn[304][0] * (nv10 - 0.0));
        let eq28_e359_d_n1: f64 = (s.dn[304][1] * (nv10 - 0.0));
        let eq28_e359_d_n2: f64 = (s.dn[304][2] * (nv10 - 0.0));
        let eq28_e359_d_n3: f64 = (s.dn[304][3] * (nv10 - 0.0));
        let eq28_e359_d_n4: f64 = (s.dn[304][4] * (nv10 - 0.0));
        let eq28_e359_d_n5: f64 = (s.dn[304][5] * (nv10 - 0.0));
        let eq28_e359_d_n6: f64 = (s.dn[304][6] * (nv10 - 0.0));
        let eq28_e359_d_n7: f64 = (s.dn[304][7] * (nv10 - 0.0));
        let eq28_e359_d_n8: f64 = (s.dn[304][8] * (nv10 - 0.0));
        let eq28_e359_d_n9: f64 = (s.dn[304][9] * (nv10 - 0.0));
        let eq28_e359_d_n10: f64 = ((s.dn[304][10] * (nv10 - 0.0)) + s.v[304]);
        let eq28_e359_d_b0: f64 = (s.db[304][0] * (nv10 - 0.0));
        let eq28_e359_d_b1: f64 = (s.db[304][1] * (nv10 - 0.0));
        let eq28_value: f64 = eq28_e359;
        let eq28_node_derivatives: [f64; 11] = [eq28_e359_d_n0, eq28_e359_d_n1, eq28_e359_d_n2, eq28_e359_d_n3, eq28_e359_d_n4, eq28_e359_d_n5, eq28_e359_d_n6, eq28_e359_d_n7, eq28_e359_d_n8, eq28_e359_d_n9, eq28_e359_d_n10];
        let eq28_branch_derivatives: [f64; 2] = [eq28_e359_d_b0, eq28_e359_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq29_value: f64 = (nv10 - 0.0);
        stamper.stamp_current_node1_local(
            Some(7),
            Some(3),
            multiplicity * (eq29_value),
            10,
            multiplicity * (1.0),
        );
        let eq30_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (eq30_value),
        );
        let eq31_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (eq31_value),
        );
        let eq32_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(2),
            Some(3),
            multiplicity * (eq32_value),
        );
        let eq33_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (eq33_value),
        );
        let eq34_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (eq34_value),
        );
        let eq35_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (eq35_value),
        );
        let eq36_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (eq36_value),
        );
        let eq37_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (eq37_value),
        );
        let eq38_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (eq38_value),
        );
        let eq39_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (eq39_value),
        );
        let eq40_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (eq40_value),
        );
        let eq41_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (eq41_value),
        );
        let eq42_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (eq42_value),
        );
        let eq43_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (eq43_value),
        );
        let (eq44_e443,) = {
    if s.b[565] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq44_value: f64 = eq44_e443;
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (eq44_value),
        );
        let (eq45_e452,) = {
    if (!s.b[565]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq45_value: f64 = eq45_e452;
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (eq45_value),
        );
        let (eq46_e462,) = {
    if (s.b[566] && s.b[567]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq46_value: f64 = eq46_e462;
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (eq46_value),
        );
        let (eq47_e472,) = {
    if (s.b[566] && s.b[567]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e472;
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (eq47_value),
        );
        let (eq48_e482,) = {
    if (s.b[566] && s.b[567]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e482;
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (eq48_value),
        );
        let (eq49_e493,) = {
    if (s.b[566] && (!s.b[567])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e493;
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (eq49_value),
        );
        let (eq50_e504,) = {
    if (s.b[566] && (!s.b[567])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e504;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq50_value),
        );
        let (eq51_e515,) = {
    if ((!s.b[566]) && s.b[568]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq51_value: f64 = eq51_e515;
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (eq51_value),
        );
        let (eq52_e526,) = {
    if ((!s.b[566]) && s.b[568]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq52_value: f64 = eq52_e526;
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (eq52_value),
        );
        let (eq53_e538,) = {
    if ((!s.b[566]) && (!s.b[568])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e538;
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
            multiplicity * (eq53_value),
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
        let eq10_e233: f64 = (s.v[204] + s.v[209]);
        let eq10_e233_d_n0: f64 = (s.dn[204][0] + s.dn[209][0]);
        let eq10_e233_d_n1: f64 = (s.dn[204][1] + s.dn[209][1]);
        let eq10_e233_d_n2: f64 = (s.dn[204][2] + s.dn[209][2]);
        let eq10_e233_d_n3: f64 = (s.dn[204][3] + s.dn[209][3]);
        let eq10_e233_d_n4: f64 = (s.dn[204][4] + s.dn[209][4]);
        let eq10_e233_d_n5: f64 = (s.dn[204][5] + s.dn[209][5]);
        let eq10_e233_d_n6: f64 = (s.dn[204][6] + s.dn[209][6]);
        let eq10_e233_d_n7: f64 = (s.dn[204][7] + s.dn[209][7]);
        let eq10_e233_d_n8: f64 = (s.dn[204][8] + s.dn[209][8]);
        let eq10_e233_d_n9: f64 = (s.dn[204][9] + s.dn[209][9]);
        let eq10_e233_d_n10: f64 = (s.dn[204][10] + s.dn[209][10]);
        let eq10_e233_d_b0: f64 = (s.db[204][0] + s.db[209][0]);
        let eq10_e233_d_b1: f64 = (s.db[204][1] + s.db[209][1]);
        let eq10_e235: f64 = (eq10_e233 + s.v[221]);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + s.dn[221][0]);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + s.dn[221][1]);
        let eq10_e235_d_n2: f64 = (eq10_e233_d_n2 + s.dn[221][2]);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + s.dn[221][3]);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + s.dn[221][4]);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + s.dn[221][5]);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + s.dn[221][6]);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + s.dn[221][7]);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + s.dn[221][8]);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + s.dn[221][9]);
        let eq10_e235_d_n10: f64 = (eq10_e233_d_n10 + s.dn[221][10]);
        let eq10_e235_d_b0: f64 = (eq10_e233_d_b0 + s.db[221][0]);
        let eq10_e235_d_b1: f64 = (eq10_e233_d_b1 + s.db[221][1]);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n2: f64 = (p.p3 * eq10_e235_d_n2);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e236_d_n10: f64 = (p.p3 * eq10_e235_d_n10);
        let eq10_e236_d_b0: f64 = (p.p3 * eq10_e235_d_b0);
        let eq10_e236_d_b1: f64 = (p.p3 * eq10_e235_d_b1);
        let eq10_e237_q: f64 = eq10_e236;
        let eq10_e239: f64 = (eq10_e236 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e236_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e236_d_n1 * p.p1);
        let eq10_e239_d_n2: f64 = (eq10_e236_d_n2 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e236_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e236_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e236_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e236_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e236_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e236_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e236_d_n9 * p.p1);
        let eq10_e239_d_n10: f64 = (eq10_e236_d_n10 * p.p1);
        let eq10_e239_d_b0: f64 = (eq10_e236_d_b0 * p.p1);
        let eq10_e239_d_b1: f64 = (eq10_e236_d_b1 * p.p1);
        let eq10_e239_q: f64 = (eq10_e237_q * p.p1);
        let eq10_e239_q_d_n0: f64 = (eq10_e236_d_n0 * p.p1);
        let eq10_e239_q_d_n1: f64 = (eq10_e236_d_n1 * p.p1);
        let eq10_e239_q_d_n2: f64 = (eq10_e236_d_n2 * p.p1);
        let eq10_e239_q_d_n3: f64 = (eq10_e236_d_n3 * p.p1);
        let eq10_e239_q_d_n4: f64 = (eq10_e236_d_n4 * p.p1);
        let eq10_e239_q_d_n5: f64 = (eq10_e236_d_n5 * p.p1);
        let eq10_e239_q_d_n6: f64 = (eq10_e236_d_n6 * p.p1);
        let eq10_e239_q_d_n7: f64 = (eq10_e236_d_n7 * p.p1);
        let eq10_e239_q_d_n8: f64 = (eq10_e236_d_n8 * p.p1);
        let eq10_e239_q_d_n9: f64 = (eq10_e236_d_n9 * p.p1);
        let eq10_e239_q_d_n10: f64 = (eq10_e236_d_n10 * p.p1);
        let eq10_e239_q_d_b0: f64 = (eq10_e236_d_b0 * p.p1);
        let eq10_e239_q_d_b1: f64 = (eq10_e236_d_b1 * p.p1);
        let eq10_reactive_node_derivatives: [f64; 11] = [eq10_e239_q_d_n0, eq10_e239_q_d_n1, eq10_e239_q_d_n2, eq10_e239_q_d_n3, eq10_e239_q_d_n4, eq10_e239_q_d_n5, eq10_e239_q_d_n6, eq10_e239_q_d_n7, eq10_e239_q_d_n8, eq10_e239_q_d_n9, eq10_e239_q_d_n10];
        let eq10_reactive_branch_derivatives: [f64; 2] = [eq10_e239_q_d_b0, eq10_e239_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e242: f64 = (p.p3 * s.v[206]);
        let eq11_e242_d_n0: f64 = (p.p3 * s.dn[206][0]);
        let eq11_e242_d_n1: f64 = (p.p3 * s.dn[206][1]);
        let eq11_e242_d_n2: f64 = (p.p3 * s.dn[206][2]);
        let eq11_e242_d_n3: f64 = (p.p3 * s.dn[206][3]);
        let eq11_e242_d_n4: f64 = (p.p3 * s.dn[206][4]);
        let eq11_e242_d_n5: f64 = (p.p3 * s.dn[206][5]);
        let eq11_e242_d_n6: f64 = (p.p3 * s.dn[206][6]);
        let eq11_e242_d_n7: f64 = (p.p3 * s.dn[206][7]);
        let eq11_e242_d_n8: f64 = (p.p3 * s.dn[206][8]);
        let eq11_e242_d_n9: f64 = (p.p3 * s.dn[206][9]);
        let eq11_e242_d_n10: f64 = (p.p3 * s.dn[206][10]);
        let eq11_e242_d_b0: f64 = (p.p3 * s.db[206][0]);
        let eq11_e242_d_b1: f64 = (p.p3 * s.db[206][1]);
        let eq11_e243_q: f64 = eq11_e242;
        let eq11_e245: f64 = (eq11_e242 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e242_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e242_d_n1 * p.p1);
        let eq11_e245_d_n2: f64 = (eq11_e242_d_n2 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e242_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e242_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e242_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e242_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e242_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e242_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e242_d_n9 * p.p1);
        let eq11_e245_d_n10: f64 = (eq11_e242_d_n10 * p.p1);
        let eq11_e245_d_b0: f64 = (eq11_e242_d_b0 * p.p1);
        let eq11_e245_d_b1: f64 = (eq11_e242_d_b1 * p.p1);
        let eq11_e245_q: f64 = (eq11_e243_q * p.p1);
        let eq11_e245_q_d_n0: f64 = (eq11_e242_d_n0 * p.p1);
        let eq11_e245_q_d_n1: f64 = (eq11_e242_d_n1 * p.p1);
        let eq11_e245_q_d_n2: f64 = (eq11_e242_d_n2 * p.p1);
        let eq11_e245_q_d_n3: f64 = (eq11_e242_d_n3 * p.p1);
        let eq11_e245_q_d_n4: f64 = (eq11_e242_d_n4 * p.p1);
        let eq11_e245_q_d_n5: f64 = (eq11_e242_d_n5 * p.p1);
        let eq11_e245_q_d_n6: f64 = (eq11_e242_d_n6 * p.p1);
        let eq11_e245_q_d_n7: f64 = (eq11_e242_d_n7 * p.p1);
        let eq11_e245_q_d_n8: f64 = (eq11_e242_d_n8 * p.p1);
        let eq11_e245_q_d_n9: f64 = (eq11_e242_d_n9 * p.p1);
        let eq11_e245_q_d_n10: f64 = (eq11_e242_d_n10 * p.p1);
        let eq11_e245_q_d_b0: f64 = (eq11_e242_d_b0 * p.p1);
        let eq11_e245_q_d_b1: f64 = (eq11_e242_d_b1 * p.p1);
        let eq11_reactive_node_derivatives: [f64; 11] = [eq11_e245_q_d_n0, eq11_e245_q_d_n1, eq11_e245_q_d_n2, eq11_e245_q_d_n3, eq11_e245_q_d_n4, eq11_e245_q_d_n5, eq11_e245_q_d_n6, eq11_e245_q_d_n7, eq11_e245_q_d_n8, eq11_e245_q_d_n9, eq11_e245_q_d_n10];
        let eq11_reactive_branch_derivatives: [f64; 2] = [eq11_e245_q_d_b0, eq11_e245_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e249: f64 = (s.v[207] + s.v[210]);
        let eq12_e249_d_n0: f64 = (s.dn[207][0] + s.dn[210][0]);
        let eq12_e249_d_n1: f64 = (s.dn[207][1] + s.dn[210][1]);
        let eq12_e249_d_n2: f64 = (s.dn[207][2] + s.dn[210][2]);
        let eq12_e249_d_n3: f64 = (s.dn[207][3] + s.dn[210][3]);
        let eq12_e249_d_n4: f64 = (s.dn[207][4] + s.dn[210][4]);
        let eq12_e249_d_n5: f64 = (s.dn[207][5] + s.dn[210][5]);
        let eq12_e249_d_n6: f64 = (s.dn[207][6] + s.dn[210][6]);
        let eq12_e249_d_n7: f64 = (s.dn[207][7] + s.dn[210][7]);
        let eq12_e249_d_n8: f64 = (s.dn[207][8] + s.dn[210][8]);
        let eq12_e249_d_n9: f64 = (s.dn[207][9] + s.dn[210][9]);
        let eq12_e249_d_n10: f64 = (s.dn[207][10] + s.dn[210][10]);
        let eq12_e249_d_b0: f64 = (s.db[207][0] + s.db[210][0]);
        let eq12_e249_d_b1: f64 = (s.db[207][1] + s.db[210][1]);
        let eq12_e251: f64 = (eq12_e249 + s.v[224]);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + s.dn[224][0]);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + s.dn[224][1]);
        let eq12_e251_d_n2: f64 = (eq12_e249_d_n2 + s.dn[224][2]);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + s.dn[224][3]);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + s.dn[224][4]);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + s.dn[224][5]);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + s.dn[224][6]);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + s.dn[224][7]);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + s.dn[224][8]);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + s.dn[224][9]);
        let eq12_e251_d_n10: f64 = (eq12_e249_d_n10 + s.dn[224][10]);
        let eq12_e251_d_b0: f64 = (eq12_e249_d_b0 + s.db[224][0]);
        let eq12_e251_d_b1: f64 = (eq12_e249_d_b1 + s.db[224][1]);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n2: f64 = (p.p3 * eq12_e251_d_n2);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e252_d_n10: f64 = (p.p3 * eq12_e251_d_n10);
        let eq12_e252_d_b0: f64 = (p.p3 * eq12_e251_d_b0);
        let eq12_e252_d_b1: f64 = (p.p3 * eq12_e251_d_b1);
        let eq12_e253_q: f64 = eq12_e252;
        let eq12_e255: f64 = (eq12_e252 * p.p1);
        let eq12_e255_d_n0: f64 = (eq12_e252_d_n0 * p.p1);
        let eq12_e255_d_n1: f64 = (eq12_e252_d_n1 * p.p1);
        let eq12_e255_d_n2: f64 = (eq12_e252_d_n2 * p.p1);
        let eq12_e255_d_n3: f64 = (eq12_e252_d_n3 * p.p1);
        let eq12_e255_d_n4: f64 = (eq12_e252_d_n4 * p.p1);
        let eq12_e255_d_n5: f64 = (eq12_e252_d_n5 * p.p1);
        let eq12_e255_d_n6: f64 = (eq12_e252_d_n6 * p.p1);
        let eq12_e255_d_n7: f64 = (eq12_e252_d_n7 * p.p1);
        let eq12_e255_d_n8: f64 = (eq12_e252_d_n8 * p.p1);
        let eq12_e255_d_n9: f64 = (eq12_e252_d_n9 * p.p1);
        let eq12_e255_d_n10: f64 = (eq12_e252_d_n10 * p.p1);
        let eq12_e255_d_b0: f64 = (eq12_e252_d_b0 * p.p1);
        let eq12_e255_d_b1: f64 = (eq12_e252_d_b1 * p.p1);
        let eq12_e255_q: f64 = (eq12_e253_q * p.p1);
        let eq12_e255_q_d_n0: f64 = (eq12_e252_d_n0 * p.p1);
        let eq12_e255_q_d_n1: f64 = (eq12_e252_d_n1 * p.p1);
        let eq12_e255_q_d_n2: f64 = (eq12_e252_d_n2 * p.p1);
        let eq12_e255_q_d_n3: f64 = (eq12_e252_d_n3 * p.p1);
        let eq12_e255_q_d_n4: f64 = (eq12_e252_d_n4 * p.p1);
        let eq12_e255_q_d_n5: f64 = (eq12_e252_d_n5 * p.p1);
        let eq12_e255_q_d_n6: f64 = (eq12_e252_d_n6 * p.p1);
        let eq12_e255_q_d_n7: f64 = (eq12_e252_d_n7 * p.p1);
        let eq12_e255_q_d_n8: f64 = (eq12_e252_d_n8 * p.p1);
        let eq12_e255_q_d_n9: f64 = (eq12_e252_d_n9 * p.p1);
        let eq12_e255_q_d_n10: f64 = (eq12_e252_d_n10 * p.p1);
        let eq12_e255_q_d_b0: f64 = (eq12_e252_d_b0 * p.p1);
        let eq12_e255_q_d_b1: f64 = (eq12_e252_d_b1 * p.p1);
        let eq12_reactive_node_derivatives: [f64; 11] = [eq12_e255_q_d_n0, eq12_e255_q_d_n1, eq12_e255_q_d_n2, eq12_e255_q_d_n3, eq12_e255_q_d_n4, eq12_e255_q_d_n5, eq12_e255_q_d_n6, eq12_e255_q_d_n7, eq12_e255_q_d_n8, eq12_e255_q_d_n9, eq12_e255_q_d_n10];
        let eq12_reactive_branch_derivatives: [f64; 2] = [eq12_e255_q_d_b0, eq12_e255_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e258: f64 = (p.p3 * s.v[211]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[211][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[211][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[211][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[211][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[211][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[211][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[211][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[211][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[211][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[211][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[211][10]);
        let eq13_e258_d_b0: f64 = (p.p3 * s.db[211][0]);
        let eq13_e258_d_b1: f64 = (p.p3 * s.db[211][1]);
        let eq13_e259_q: f64 = eq13_e258;
        let eq13_e261: f64 = (eq13_e258 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e258_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e258_d_n1 * p.p1);
        let eq13_e261_d_n2: f64 = (eq13_e258_d_n2 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e258_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e258_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e258_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e258_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e258_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e258_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e258_d_n9 * p.p1);
        let eq13_e261_d_n10: f64 = (eq13_e258_d_n10 * p.p1);
        let eq13_e261_d_b0: f64 = (eq13_e258_d_b0 * p.p1);
        let eq13_e261_d_b1: f64 = (eq13_e258_d_b1 * p.p1);
        let eq13_e261_q: f64 = (eq13_e259_q * p.p1);
        let eq13_e261_q_d_n0: f64 = (eq13_e258_d_n0 * p.p1);
        let eq13_e261_q_d_n1: f64 = (eq13_e258_d_n1 * p.p1);
        let eq13_e261_q_d_n2: f64 = (eq13_e258_d_n2 * p.p1);
        let eq13_e261_q_d_n3: f64 = (eq13_e258_d_n3 * p.p1);
        let eq13_e261_q_d_n4: f64 = (eq13_e258_d_n4 * p.p1);
        let eq13_e261_q_d_n5: f64 = (eq13_e258_d_n5 * p.p1);
        let eq13_e261_q_d_n6: f64 = (eq13_e258_d_n6 * p.p1);
        let eq13_e261_q_d_n7: f64 = (eq13_e258_d_n7 * p.p1);
        let eq13_e261_q_d_n8: f64 = (eq13_e258_d_n8 * p.p1);
        let eq13_e261_q_d_n9: f64 = (eq13_e258_d_n9 * p.p1);
        let eq13_e261_q_d_n10: f64 = (eq13_e258_d_n10 * p.p1);
        let eq13_e261_q_d_b0: f64 = (eq13_e258_d_b0 * p.p1);
        let eq13_e261_q_d_b1: f64 = (eq13_e258_d_b1 * p.p1);
        let eq13_reactive_node_derivatives: [f64; 11] = [eq13_e261_q_d_n0, eq13_e261_q_d_n1, eq13_e261_q_d_n2, eq13_e261_q_d_n3, eq13_e261_q_d_n4, eq13_e261_q_d_n5, eq13_e261_q_d_n6, eq13_e261_q_d_n7, eq13_e261_q_d_n8, eq13_e261_q_d_n9, eq13_e261_q_d_n10];
        let eq13_reactive_branch_derivatives: [f64; 2] = [eq13_e261_q_d_b0, eq13_e261_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * s.v[243]);
        let eq14_e266_d_n0: f64 = (eq14_e264 * s.dn[243][0]);
        let eq14_e266_d_n1: f64 = (eq14_e264 * s.dn[243][1]);
        let eq14_e266_d_n2: f64 = (eq14_e264 * s.dn[243][2]);
        let eq14_e266_d_n3: f64 = (eq14_e264 * s.dn[243][3]);
        let eq14_e266_d_n4: f64 = (eq14_e264 * s.dn[243][4]);
        let eq14_e266_d_n5: f64 = (eq14_e264 * s.dn[243][5]);
        let eq14_e266_d_n6: f64 = (eq14_e264 * s.dn[243][6]);
        let eq14_e266_d_n7: f64 = (eq14_e264 * s.dn[243][7]);
        let eq14_e266_d_n8: f64 = (eq14_e264 * s.dn[243][8]);
        let eq14_e266_d_n9: f64 = (eq14_e264 * s.dn[243][9]);
        let eq14_e266_d_n10: f64 = (eq14_e264 * s.dn[243][10]);
        let eq14_e266_d_b0: f64 = (eq14_e264 * s.db[243][0]);
        let eq14_e266_d_b1: f64 = (eq14_e264 * s.db[243][1]);
        let eq14_e267_q: f64 = eq14_e266;
        let eq14_e269: f64 = (eq14_e266 * p.p1);
        let eq14_e269_d_n0: f64 = (eq14_e266_d_n0 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e266_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e266_d_n2 * p.p1);
        let eq14_e269_d_n3: f64 = (eq14_e266_d_n3 * p.p1);
        let eq14_e269_d_n4: f64 = (eq14_e266_d_n4 * p.p1);
        let eq14_e269_d_n5: f64 = (eq14_e266_d_n5 * p.p1);
        let eq14_e269_d_n6: f64 = (eq14_e266_d_n6 * p.p1);
        let eq14_e269_d_n7: f64 = (eq14_e266_d_n7 * p.p1);
        let eq14_e269_d_n8: f64 = (eq14_e266_d_n8 * p.p1);
        let eq14_e269_d_n9: f64 = (eq14_e266_d_n9 * p.p1);
        let eq14_e269_d_n10: f64 = (eq14_e266_d_n10 * p.p1);
        let eq14_e269_d_b0: f64 = (eq14_e266_d_b0 * p.p1);
        let eq14_e269_d_b1: f64 = (eq14_e266_d_b1 * p.p1);
        let eq14_e269_q: f64 = (eq14_e267_q * p.p1);
        let eq14_e269_q_d_n0: f64 = (eq14_e266_d_n0 * p.p1);
        let eq14_e269_q_d_n1: f64 = (eq14_e266_d_n1 * p.p1);
        let eq14_e269_q_d_n2: f64 = (eq14_e266_d_n2 * p.p1);
        let eq14_e269_q_d_n3: f64 = (eq14_e266_d_n3 * p.p1);
        let eq14_e269_q_d_n4: f64 = (eq14_e266_d_n4 * p.p1);
        let eq14_e269_q_d_n5: f64 = (eq14_e266_d_n5 * p.p1);
        let eq14_e269_q_d_n6: f64 = (eq14_e266_d_n6 * p.p1);
        let eq14_e269_q_d_n7: f64 = (eq14_e266_d_n7 * p.p1);
        let eq14_e269_q_d_n8: f64 = (eq14_e266_d_n8 * p.p1);
        let eq14_e269_q_d_n9: f64 = (eq14_e266_d_n9 * p.p1);
        let eq14_e269_q_d_n10: f64 = (eq14_e266_d_n10 * p.p1);
        let eq14_e269_q_d_b0: f64 = (eq14_e266_d_b0 * p.p1);
        let eq14_e269_q_d_b1: f64 = (eq14_e266_d_b1 * p.p1);
        let eq14_reactive_node_derivatives: [f64; 11] = [eq14_e269_q_d_n0, eq14_e269_q_d_n1, eq14_e269_q_d_n2, eq14_e269_q_d_n3, eq14_e269_q_d_n4, eq14_e269_q_d_n5, eq14_e269_q_d_n6, eq14_e269_q_d_n7, eq14_e269_q_d_n8, eq14_e269_q_d_n9, eq14_e269_q_d_n10];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e269_q_d_b0, eq14_e269_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * s.v[244]);
        let eq15_e274_d_n0: f64 = (eq15_e272 * s.dn[244][0]);
        let eq15_e274_d_n1: f64 = (eq15_e272 * s.dn[244][1]);
        let eq15_e274_d_n2: f64 = (eq15_e272 * s.dn[244][2]);
        let eq15_e274_d_n3: f64 = (eq15_e272 * s.dn[244][3]);
        let eq15_e274_d_n4: f64 = (eq15_e272 * s.dn[244][4]);
        let eq15_e274_d_n5: f64 = (eq15_e272 * s.dn[244][5]);
        let eq15_e274_d_n6: f64 = (eq15_e272 * s.dn[244][6]);
        let eq15_e274_d_n7: f64 = (eq15_e272 * s.dn[244][7]);
        let eq15_e274_d_n8: f64 = (eq15_e272 * s.dn[244][8]);
        let eq15_e274_d_n9: f64 = (eq15_e272 * s.dn[244][9]);
        let eq15_e274_d_n10: f64 = (eq15_e272 * s.dn[244][10]);
        let eq15_e274_d_b0: f64 = (eq15_e272 * s.db[244][0]);
        let eq15_e274_d_b1: f64 = (eq15_e272 * s.db[244][1]);
        let eq15_e275_q: f64 = eq15_e274;
        let eq15_e277: f64 = (eq15_e274 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e274_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e274_d_n1 * p.p1);
        let eq15_e277_d_n2: f64 = (eq15_e274_d_n2 * p.p1);
        let eq15_e277_d_n3: f64 = (eq15_e274_d_n3 * p.p1);
        let eq15_e277_d_n4: f64 = (eq15_e274_d_n4 * p.p1);
        let eq15_e277_d_n5: f64 = (eq15_e274_d_n5 * p.p1);
        let eq15_e277_d_n6: f64 = (eq15_e274_d_n6 * p.p1);
        let eq15_e277_d_n7: f64 = (eq15_e274_d_n7 * p.p1);
        let eq15_e277_d_n8: f64 = (eq15_e274_d_n8 * p.p1);
        let eq15_e277_d_n9: f64 = (eq15_e274_d_n9 * p.p1);
        let eq15_e277_d_n10: f64 = (eq15_e274_d_n10 * p.p1);
        let eq15_e277_d_b0: f64 = (eq15_e274_d_b0 * p.p1);
        let eq15_e277_d_b1: f64 = (eq15_e274_d_b1 * p.p1);
        let eq15_e277_q: f64 = (eq15_e275_q * p.p1);
        let eq15_e277_q_d_n0: f64 = (eq15_e274_d_n0 * p.p1);
        let eq15_e277_q_d_n1: f64 = (eq15_e274_d_n1 * p.p1);
        let eq15_e277_q_d_n2: f64 = (eq15_e274_d_n2 * p.p1);
        let eq15_e277_q_d_n3: f64 = (eq15_e274_d_n3 * p.p1);
        let eq15_e277_q_d_n4: f64 = (eq15_e274_d_n4 * p.p1);
        let eq15_e277_q_d_n5: f64 = (eq15_e274_d_n5 * p.p1);
        let eq15_e277_q_d_n6: f64 = (eq15_e274_d_n6 * p.p1);
        let eq15_e277_q_d_n7: f64 = (eq15_e274_d_n7 * p.p1);
        let eq15_e277_q_d_n8: f64 = (eq15_e274_d_n8 * p.p1);
        let eq15_e277_q_d_n9: f64 = (eq15_e274_d_n9 * p.p1);
        let eq15_e277_q_d_n10: f64 = (eq15_e274_d_n10 * p.p1);
        let eq15_e277_q_d_b0: f64 = (eq15_e274_d_b0 * p.p1);
        let eq15_e277_q_d_b1: f64 = (eq15_e274_d_b1 * p.p1);
        let eq15_reactive_node_derivatives: [f64; 11] = [eq15_e277_q_d_n0, eq15_e277_q_d_n1, eq15_e277_q_d_n2, eq15_e277_q_d_n3, eq15_e277_q_d_n4, eq15_e277_q_d_n5, eq15_e277_q_d_n6, eq15_e277_q_d_n7, eq15_e277_q_d_n8, eq15_e277_q_d_n9, eq15_e277_q_d_n10];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e277_q_d_b0, eq15_e277_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e293: f64 = (s.v[219] + s.v[228]);
        let eq18_e293_d_n0: f64 = (s.dn[219][0] + s.dn[228][0]);
        let eq18_e293_d_n1: f64 = (s.dn[219][1] + s.dn[228][1]);
        let eq18_e293_d_n2: f64 = (s.dn[219][2] + s.dn[228][2]);
        let eq18_e293_d_n3: f64 = (s.dn[219][3] + s.dn[228][3]);
        let eq18_e293_d_n4: f64 = (s.dn[219][4] + s.dn[228][4]);
        let eq18_e293_d_n5: f64 = (s.dn[219][5] + s.dn[228][5]);
        let eq18_e293_d_n6: f64 = (s.dn[219][6] + s.dn[228][6]);
        let eq18_e293_d_n7: f64 = (s.dn[219][7] + s.dn[228][7]);
        let eq18_e293_d_n8: f64 = (s.dn[219][8] + s.dn[228][8]);
        let eq18_e293_d_n9: f64 = (s.dn[219][9] + s.dn[228][9]);
        let eq18_e293_d_n10: f64 = (s.dn[219][10] + s.dn[228][10]);
        let eq18_e293_d_b0: f64 = (s.db[219][0] + s.db[228][0]);
        let eq18_e293_d_b1: f64 = (s.db[219][1] + s.db[228][1]);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n2: f64 = (p.p3 * eq18_e293_d_n2);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e294_d_n10: f64 = (p.p3 * eq18_e293_d_n10);
        let eq18_e294_d_b0: f64 = (p.p3 * eq18_e293_d_b0);
        let eq18_e294_d_b1: f64 = (p.p3 * eq18_e293_d_b1);
        let eq18_e295_q: f64 = eq18_e294;
        let eq18_e297: f64 = (eq18_e294 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e294_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e294_d_n1 * p.p1);
        let eq18_e297_d_n2: f64 = (eq18_e294_d_n2 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e294_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e294_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e294_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e294_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e294_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e294_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e294_d_n9 * p.p1);
        let eq18_e297_d_n10: f64 = (eq18_e294_d_n10 * p.p1);
        let eq18_e297_d_b0: f64 = (eq18_e294_d_b0 * p.p1);
        let eq18_e297_d_b1: f64 = (eq18_e294_d_b1 * p.p1);
        let eq18_e297_q: f64 = (eq18_e295_q * p.p1);
        let eq18_e297_q_d_n0: f64 = (eq18_e294_d_n0 * p.p1);
        let eq18_e297_q_d_n1: f64 = (eq18_e294_d_n1 * p.p1);
        let eq18_e297_q_d_n2: f64 = (eq18_e294_d_n2 * p.p1);
        let eq18_e297_q_d_n3: f64 = (eq18_e294_d_n3 * p.p1);
        let eq18_e297_q_d_n4: f64 = (eq18_e294_d_n4 * p.p1);
        let eq18_e297_q_d_n5: f64 = (eq18_e294_d_n5 * p.p1);
        let eq18_e297_q_d_n6: f64 = (eq18_e294_d_n6 * p.p1);
        let eq18_e297_q_d_n7: f64 = (eq18_e294_d_n7 * p.p1);
        let eq18_e297_q_d_n8: f64 = (eq18_e294_d_n8 * p.p1);
        let eq18_e297_q_d_n9: f64 = (eq18_e294_d_n9 * p.p1);
        let eq18_e297_q_d_n10: f64 = (eq18_e294_d_n10 * p.p1);
        let eq18_e297_q_d_b0: f64 = (eq18_e294_d_b0 * p.p1);
        let eq18_e297_q_d_b1: f64 = (eq18_e294_d_b1 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 11] = [eq18_e297_q_d_n0, eq18_e297_q_d_n1, eq18_e297_q_d_n2, eq18_e297_q_d_n3, eq18_e297_q_d_n4, eq18_e297_q_d_n5, eq18_e297_q_d_n6, eq18_e297_q_d_n7, eq18_e297_q_d_n8, eq18_e297_q_d_n9, eq18_e297_q_d_n10];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e297_q_d_b0, eq18_e297_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq20_e312: f64 = (s.v[216] + s.v[229]);
        let eq20_e312_d_n0: f64 = (s.dn[216][0] + s.dn[229][0]);
        let eq20_e312_d_n1: f64 = (s.dn[216][1] + s.dn[229][1]);
        let eq20_e312_d_n2: f64 = (s.dn[216][2] + s.dn[229][2]);
        let eq20_e312_d_n3: f64 = (s.dn[216][3] + s.dn[229][3]);
        let eq20_e312_d_n4: f64 = (s.dn[216][4] + s.dn[229][4]);
        let eq20_e312_d_n5: f64 = (s.dn[216][5] + s.dn[229][5]);
        let eq20_e312_d_n6: f64 = (s.dn[216][6] + s.dn[229][6]);
        let eq20_e312_d_n7: f64 = (s.dn[216][7] + s.dn[229][7]);
        let eq20_e312_d_n8: f64 = (s.dn[216][8] + s.dn[229][8]);
        let eq20_e312_d_n9: f64 = (s.dn[216][9] + s.dn[229][9]);
        let eq20_e312_d_n10: f64 = (s.dn[216][10] + s.dn[229][10]);
        let eq20_e312_d_b0: f64 = (s.db[216][0] + s.db[229][0]);
        let eq20_e312_d_b1: f64 = (s.db[216][1] + s.db[229][1]);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n2: f64 = (p.p3 * eq20_e312_d_n2);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e313_d_n10: f64 = (p.p3 * eq20_e312_d_n10);
        let eq20_e313_d_b0: f64 = (p.p3 * eq20_e312_d_b0);
        let eq20_e313_d_b1: f64 = (p.p3 * eq20_e312_d_b1);
        let eq20_e314_q: f64 = eq20_e313;
        let eq20_e316: f64 = (eq20_e313 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_d_n2: f64 = (eq20_e313_d_n2 * p.p1);
        let eq20_e316_d_n3: f64 = (eq20_e313_d_n3 * p.p1);
        let eq20_e316_d_n4: f64 = (eq20_e313_d_n4 * p.p1);
        let eq20_e316_d_n5: f64 = (eq20_e313_d_n5 * p.p1);
        let eq20_e316_d_n6: f64 = (eq20_e313_d_n6 * p.p1);
        let eq20_e316_d_n7: f64 = (eq20_e313_d_n7 * p.p1);
        let eq20_e316_d_n8: f64 = (eq20_e313_d_n8 * p.p1);
        let eq20_e316_d_n9: f64 = (eq20_e313_d_n9 * p.p1);
        let eq20_e316_d_n10: f64 = (eq20_e313_d_n10 * p.p1);
        let eq20_e316_d_b0: f64 = (eq20_e313_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e313_d_b1 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        let eq20_e316_q_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_q_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_q_d_n2: f64 = (eq20_e313_d_n2 * p.p1);
        let eq20_e316_q_d_n3: f64 = (eq20_e313_d_n3 * p.p1);
        let eq20_e316_q_d_n4: f64 = (eq20_e313_d_n4 * p.p1);
        let eq20_e316_q_d_n5: f64 = (eq20_e313_d_n5 * p.p1);
        let eq20_e316_q_d_n6: f64 = (eq20_e313_d_n6 * p.p1);
        let eq20_e316_q_d_n7: f64 = (eq20_e313_d_n7 * p.p1);
        let eq20_e316_q_d_n8: f64 = (eq20_e313_d_n8 * p.p1);
        let eq20_e316_q_d_n9: f64 = (eq20_e313_d_n9 * p.p1);
        let eq20_e316_q_d_n10: f64 = (eq20_e313_d_n10 * p.p1);
        let eq20_e316_q_d_b0: f64 = (eq20_e313_d_b0 * p.p1);
        let eq20_e316_q_d_b1: f64 = (eq20_e313_d_b1 * p.p1);
        let eq20_reactive_node_derivatives: [f64; 11] = [eq20_e316_q_d_n0, eq20_e316_q_d_n1, eq20_e316_q_d_n2, eq20_e316_q_d_n3, eq20_e316_q_d_n4, eq20_e316_q_d_n5, eq20_e316_q_d_n6, eq20_e316_q_d_n7, eq20_e316_q_d_n8, eq20_e316_q_d_n9, eq20_e316_q_d_n10];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e316_q_d_b0, eq20_e316_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e355_q: f64 = (nv10 - 0.0);
        let eq27_e356: f64 = (s.v[306] * (nv10 - 0.0));
        let eq27_e356_d_n0: f64 = (s.dn[306][0] * (nv10 - 0.0));
        let eq27_e356_d_n1: f64 = (s.dn[306][1] * (nv10 - 0.0));
        let eq27_e356_d_n2: f64 = (s.dn[306][2] * (nv10 - 0.0));
        let eq27_e356_d_n3: f64 = (s.dn[306][3] * (nv10 - 0.0));
        let eq27_e356_d_n4: f64 = (s.dn[306][4] * (nv10 - 0.0));
        let eq27_e356_d_n5: f64 = (s.dn[306][5] * (nv10 - 0.0));
        let eq27_e356_d_n6: f64 = (s.dn[306][6] * (nv10 - 0.0));
        let eq27_e356_d_n7: f64 = (s.dn[306][7] * (nv10 - 0.0));
        let eq27_e356_d_n8: f64 = (s.dn[306][8] * (nv10 - 0.0));
        let eq27_e356_d_n9: f64 = (s.dn[306][9] * (nv10 - 0.0));
        let eq27_e356_d_n10: f64 = ((s.dn[306][10] * (nv10 - 0.0)) + s.v[306]);
        let eq27_e356_d_b0: f64 = (s.db[306][0] * (nv10 - 0.0));
        let eq27_e356_d_b1: f64 = (s.db[306][1] * (nv10 - 0.0));
        let eq27_e356_q: f64 = (s.v[306] * eq27_e355_q);
        let eq27_e356_q_d_n0: f64 = (s.dn[306][0] * eq27_e355_q);
        let eq27_e356_q_d_n1: f64 = (s.dn[306][1] * eq27_e355_q);
        let eq27_e356_q_d_n2: f64 = (s.dn[306][2] * eq27_e355_q);
        let eq27_e356_q_d_n3: f64 = (s.dn[306][3] * eq27_e355_q);
        let eq27_e356_q_d_n4: f64 = (s.dn[306][4] * eq27_e355_q);
        let eq27_e356_q_d_n5: f64 = (s.dn[306][5] * eq27_e355_q);
        let eq27_e356_q_d_n6: f64 = (s.dn[306][6] * eq27_e355_q);
        let eq27_e356_q_d_n7: f64 = (s.dn[306][7] * eq27_e355_q);
        let eq27_e356_q_d_n8: f64 = (s.dn[306][8] * eq27_e355_q);
        let eq27_e356_q_d_n9: f64 = (s.dn[306][9] * eq27_e355_q);
        let eq27_e356_q_d_n10: f64 = ((s.dn[306][10] * eq27_e355_q) + s.v[306]);
        let eq27_e356_q_d_b0: f64 = (s.db[306][0] * eq27_e355_q);
        let eq27_e356_q_d_b1: f64 = (s.db[306][1] * eq27_e355_q);
        let eq27_reactive_node_derivatives: [f64; 11] = [eq27_e356_q_d_n0, eq27_e356_q_d_n1, eq27_e356_q_d_n2, eq27_e356_q_d_n3, eq27_e356_q_d_n4, eq27_e356_q_d_n5, eq27_e356_q_d_n6, eq27_e356_q_d_n7, eq27_e356_q_d_n8, eq27_e356_q_d_n9, eq27_e356_q_d_n10];
        let eq27_reactive_branch_derivatives: [f64; 2] = [eq27_e356_q_d_b0, eq27_e356_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
