#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[439] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[439] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[439] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[439] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[439] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[150] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[314] = 0.0;

        s.v[440] = if (p.p137 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[440] != 0.0) {
            s.store_scalar(315, 1e-12);
        }

        if (!(s.v[440] != 0.0)) {
            s.store_scalar(315, p.p137);
        }

        s.store_scale(316, 315, p.p1);

        s.store_div_from_scalar(317, 1.0, 316);

        s.v[52] = 0.001;

        s.v[312] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[63] = (1.0 / s.v[62]);

        s.v[259] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.v[441] = if ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[441] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[441] != 0.0)) {
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

        s.v[442] = if ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[442] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[442] != 0.0)) {
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

        s.store_scale_ad(259, A::offset(A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), (-0.05)), 10.0);

        s.v[443] = if ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[443] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[443] != 0.0)) {
            s.store_add_ad(70, A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 0.1));
        }

        s.store_scale_ad(259, A::offset(A::offset(s.ad_value(88), (-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))), (-0.05)), 10.0);

        s.v[444] = if ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[444] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[444] != 0.0)) {
            s.store_add_ad(85, A::offset(s.ad_value(88), (-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p65 * s.v[4])) + ((1.0 - s.v[4]) * p.p104));

        s.v[259] = ((0.05 - s.v[13]) / s.v[6]);

        s.v[445] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[445] != 0.0) {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[445] != 0.0)) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p63 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[15]) / s.v[6]);

        s.v[446] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[446] != 0.0) {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[446] != 0.0)) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p79 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[21]) / s.v[6]);

        s.v[447] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[447] != 0.0) {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[447] != 0.0)) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p70 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[18]) / s.v[6]);

        s.v[448] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[448] != 0.0) {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[448] != 0.0)) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[254]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[20]) / s.v[6]);

        s.v[449] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[449] != 0.0) {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[449] != 0.0)) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p26 * s.v[4])) + ((1.0 - s.v[4]) * p.p108));

        s.v[259] = ((0.05 - s.v[56]) / s.v[6]);

        s.v[450] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[450] != 0.0) {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[450] != 0.0)) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p65), p.p66);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p64);

        s.store_offset_ad(26, A::scale(A::powf(A::div_from_scalar(p.p70, s.ad_value(17)), p.p71), (1.0 - p.p74)), p.p74);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p69);

        s.store_scale(25, 27, p.p74);

        s.v[28] = (p.p53 * (((s.v[254] * p.p96)) as f64).exp());

        s.v[451] = if (s.v[28] < s.v[316]) { 1.0 } else { 0.0 };

        if (s.v[451] != 0.0) {
            s.copy_ad(28, 316);
        }

        s.v[29] = (p.p55 * (((s.v[254] * (p.p97 - p.p95))) as f64).exp());

        s.v[30] = (p.p54 * (((s.v[254] * p.p100)) as f64).exp());

        s.v[452] = if (s.v[30] < s.v[316]) { 1.0 } else { 0.0 };

        if (s.v[452] != 0.0) {
            s.copy_ad(30, 316);
        }

        s.v[32] = (p.p56 * (((s.v[254] * p.p101)) as f64).exp());

        s.v[33] = (p.p57 * (((s.v[254] * p.p103)) as f64).exp());

        s.v[34] = (p.p58 * (((s.v[254] * p.p103)) as f64).exp());

        s.v[31] = (p.p59 * (((s.v[254] * p.p98)) as f64).exp());

        s.v[453] = if (p.p121 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[453] != 0.0) {
            s.store_scalar(50, (p.p9 * (1.0 + (s.v[12] * p.p121))));
        }

        if (s.v[453] != 0.0) {
            s.store_scaled_offset(259, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[454] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[453] != 0.0) && (s.v[454] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[453] != 0.0) && (!(s.v[454] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), s.v[52]));
        }

        if (s.v[453] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[453] != 0.0)) {
            s.store_scalar(48, p.p9);
        }

        s.v[455] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[455] != 0.0) {
            s.store_scalar(51, (p.p10 * (1.0 + (s.v[12] * p.p122))));
        }

        if (s.v[455] != 0.0) {
            s.store_scaled_offset(259, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[456] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[455] != 0.0) && (s.v[456] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[455] != 0.0) && (!(s.v[456] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), s.v[52]));
        }

        if (s.v[455] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[455] != 0.0)) {
            s.store_scalar(49, p.p10);
        }

        s.v[311] = (p.p42 * (1.0 + (p.p123 * s.v[12])));

        s.v[261] = (s.v[312] * s.v[312]);

        s.v[262] = (s.v[311] * s.v[311]);

        s.v[457] = if (s.v[311] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[457] != 0.0) {
            s.store_scalar(310, ((0.5 * s.v[261]) / ((((s.v[262] + s.v[261])) as f64).sqrt() - s.v[311])));
        }

        if (!(s.v[457] != 0.0)) {
            s.store_scalar(310, (0.5 * ((((s.v[262] + s.v[261])) as f64).sqrt() + s.v[311])));
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div_from_scalar((s.v[254] * (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), p.p8), A::exp(A::div_from_scalar(((-p.p104) * s.v[10]), s.ad_value(48))));

        s.v[36] = (p.p11 * (((s.v[254] * (1.0 - p.p97))) as f64).exp());

        s.v[37] = (p.p29 * (((s.v[254] * (1.0 - p.p102))) as f64).exp());

        s.v[38] = ((p.p19 * (((s.v[254] * (6.0 - (2.0 * p.p20)))) as f64).exp()) * (((((-p.p112) * s.v[10]) / p.p20)) as f64).exp());

        s.v[39] = ((p.p30 * (((s.v[254] * (6.0 - (2.0 * p.p31)))) as f64).exp()) * (((((-p.p109) * s.v[10]) / p.p31)) as f64).exp());

        s.v[42] = ((p.p15 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p16)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p16)) as f64).exp());

        s.v[44] = ((p.p17 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p18)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p18)) as f64).exp());

        s.v[458] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[458] != 0.0) {
            s.store_scalar(53, (p.p24 * (((((-p.p106) * s.v[10]) / p.p16)) as f64).exp()));
        }

        if (s.v[458] != 0.0) {
            s.store_scalar(54, (p.p27 * ((((-p.p105) * s.v[10])) as f64).exp()));
        }

        if (s.v[458] != 0.0) {
            s.store_scalar(45, (p.p25 * (((((-p.p107) * s.v[10]) / p.p18)) as f64).exp()));
        }

        s.v[43] = ((p.p28 * (((s.v[254] * ((4.0 - p.p102) + p.p120))) as f64).exp()) * ((((-p.p111) * s.v[10])) as f64).exp());

        s.v[46] = ((p.p21 * (((s.v[254] * (6.0 - (2.0 * p.p22)))) as f64).exp()) * (((((-p.p112) * s.v[10]) / p.p22)) as f64).exp());

        s.v[47] = ((p.p132 * (((s.v[254] * (4.0 / p.p133))) as f64).exp()) * (((((-p.p112) * s.v[10]) / p.p133)) as f64).exp());

        s.v[325] = ((p.p138 * ((s.v[4]) as f64).sqrt()) * (((p.p140 * s.v[12])) as f64).exp());

        s.store_powf_ad(255, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(256, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p34), s.ad_value(70)), s.ad_value(255)), s.ad_value(256)), p.p65), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_mul_ad(58, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(255), p.p33), s.ad_value(14)), s.ad_value(14)), (s.v[64] * s.v[64])), s.ad_value(73)), A::exp(A::sub_from_scalar(p.p34, s.ad_value(61))));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(257, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(258, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p36), s.ad_value(85)), s.ad_value(257)), s.ad_value(258)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.store_mul_ad(84, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(257), p.p35), s.ad_value(19)), s.ad_value(19)), (s.v[66] * s.v[66])), s.ad_value(90)), A::exp(A::sub_from_scalar(p.p36, s.ad_value(83))));

        s.v[255] = (((s.v[254] * p.p95)) as f64).exp();

        s.store_scale(40, 27, (p.p13 * s.v[255]));

        s.store_scale(41, 256, (p.p12 * s.v[255]));

        s.v[93] = ((p.p85 * (((s.v[254] * (p.p97 - 2.0))) as f64).exp()) * ((((-p.p119) * s.v[10])) as f64).exp());

        s.v[94] = (p.p86 * (((s.v[254] * ((p.p95 + p.p97) - 1.0))) as f64).exp());

        s.v[95] = (p.p87 * (((s.v[254] * (p.p98 - 1.0))) as f64).exp());

        s.v[96] = ((p.p88 * (s.v[94] + s.v[95])) / (p.p86 + p.p87));

        s.v[97] = (p.p89 * (((s.v[254] * (p.p99 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.v[459] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[459] != 0.0) {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!(s.v[459] != 0.0)) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p91 * (((s.v[254] * p.p95)) as f64).exp());

        s.v[460] = if (p.p56 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[460] != 0.0) {
            s.store_scalar(101, (1.0 / s.v[32]));
        }

        s.v[461] = if (s.v[101] > s.v[317]) { 1.0 } else { 0.0 };

        if ((s.v[460] != 0.0) && (s.v[461] != 0.0)) {
            s.copy_ad(101, 317);
        }

        if (!(s.v[460] != 0.0)) {
            s.store_scalar(101, 0.0);
        }

        s.v[462] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[462] != 0.0) {
            s.store_scalar(102, (1.0 / s.v[33]));
        }

        s.v[463] = if (s.v[102] > s.v[317]) { 1.0 } else { 0.0 };

        if ((s.v[462] != 0.0) && (s.v[463] != 0.0)) {
            s.copy_ad(102, 317);
        }

        if (!(s.v[462] != 0.0)) {
            s.store_scalar(102, 0.0);
        }

        s.v[464] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[464] != 0.0) {
            s.store_scalar(103, (1.0 / s.v[34]));
        }

        s.v[465] = if (s.v[103] > s.v[317]) { 1.0 } else { 0.0 };

        if ((s.v[464] != 0.0) && (s.v[465] != 0.0)) {
            s.copy_ad(103, 317);
        }

        if (!(s.v[464] != 0.0)) {
            s.store_scalar(103, 0.0);
        }

        s.store_ad(230, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), p.p3));

        s.store_ad(231, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(7)), p.p3));

        s.store_ad(232, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(3)), p.p3));

        s.store_ad(233, &A::scale(A::voltage(ctx, &nodes, Some(4), Some(3)), p.p3));

        s.store_ad(234, &A::scale(A::voltage(ctx, &nodes, Some(4), Some(5)), p.p3));

        s.store_ad(236, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(239, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(3)), p.p3));

        s.store_ad(240, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(4)), p.p3));

    }

    pub(super) fn stamp_transient_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_ad(243, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(244, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(238, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(6)), p.p3));

        s.store_ad(237, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(9)), p.p3));

        s.store_sub_ad_lhs(235, A::sub(A::add(s.ad_value(234), s.ad_value(231)), s.ad_value(236)), 238);

        s.store_sub_ad_lhs(242, A::add(A::sub(s.ad_value(240), s.ad_value(244)), s.ad_value(235)), 237);

        s.store_add(241, 244, 242);

        s.v[466] = if ((s.v[231] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[466] != 0.0) {
            s.store_exp_ad(245, A::scale(s.ad_value(231), s.v[8]));
        }

        if (!(s.v[466] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[466] != 0.0)) {
            s.store_mul_ad_rhs(245, 275, A::offset(A::offset(A::scale(s.ad_value(231), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[467] = if (((s.v[232] * s.v[8]) / s.v[48]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[467] != 0.0) {
            s.store_exp_ad(246, A::div(A::scale(s.ad_value(232), s.v[8]), s.ad_value(48)));
        }

        if (!(s.v[467] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(246, 275, A::offset(A::offset(A::div(A::scale(s.ad_value(232), s.v[8]), s.ad_value(48)), (-p.p134)), 1.0));
        }

        s.v[468] = if ((s.v[235] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[468] != 0.0) {
            s.store_exp_ad(248, A::scale(s.ad_value(235), s.v[8]));
        }

        if (!(s.v[468] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[468] != 0.0)) {
            s.store_mul_ad_rhs(248, 275, A::offset(A::offset(A::scale(s.ad_value(235), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[469] = if ((s.v[234] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[469] != 0.0) {
            s.store_exp_ad(247, A::scale(s.ad_value(234), s.v[8]));
        }

        if (!(s.v[469] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[469] != 0.0)) {
            s.store_mul_ad_rhs(247, 275, A::offset(A::offset(A::scale(s.ad_value(234), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[470] = if ((s.v[241] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[470] != 0.0) {
            s.store_exp_ad(249, A::scale(s.ad_value(241), s.v[8]));
        }

        if (!(s.v[470] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[470] != 0.0)) {
            s.store_mul_ad_rhs(249, 275, A::offset(A::offset(A::scale(s.ad_value(241), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[471] = if (((s.v[241] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[471] != 0.0) {
            s.store_exp_ad(252, A::scale(A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[471] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[471] != 0.0)) {
            s.store_mul_ad_rhs(252, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[472] = if (((s.v[235] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[472] != 0.0) {
            s.store_exp_ad(250, A::scale(A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[472] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[472] != 0.0)) {
            s.store_mul_ad_rhs(250, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[473] = if (((s.v[231] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[473] != 0.0) {
            s.store_exp_ad(251, A::scale(A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[473] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[473] != 0.0)) {
            s.store_mul_ad_rhs(251, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[474] = if (((s.v[230] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[474] != 0.0) {
            s.store_exp_ad(253, A::scale(A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[474] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(253, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.store_sqrt_ad(104, A::offset(A::scale(s.ad_value(251), 4.0), 1.0));

        s.store_sqrt_ad(105, A::offset(A::scale(s.ad_value(253), 4.0), 1.0));

        s.store_div_ad(106, A::scale(s.ad_value(253), 2.0), A::offset(s.ad_value(105), 1.0));

        s.v[475] = if (s.v[106] < p.p136) { 1.0 } else { 0.0 };

        if (s.v[475] != 0.0) {
            s.store_scalar(106, p.p136);
        }

        s.store_scale_ad(107, A::sub(A::sub(s.ad_value(104), s.ad_value(105)), A::ln(A::div(A::offset(s.ad_value(104), 1.0), A::offset(s.ad_value(105), 1.0)))), s.v[6]);

        s.store_scaled_add(108, 107, 236, 1.0 / (s.v[31]));

        s.v[476] = if (s.v[108] > 0.0) { 1.0 } else { 0.0 };

        s.v[477] = if (s.v[230] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[477] != 0.0)) {
            s.copy_ad(277, 230);
        }

        if ((s.v[476] != 0.0) && (!(s.v[477] != 0.0))) {
            s.store_offset_ad(277, A::ln(A::offset(A::offset(s.ad_value(230), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[476] != 0.0) {
            s.store_sub_ad_lhs(109, A::add(s.ad_value(16), A::scale(A::ln(A::offset(A::scale(s.ad_value(108), (0.5 * (s.v[31] * s.v[8]))), 1.0)), (2.0 * s.v[6]))), 277);
        }

        if (s.v[476] != 0.0) {
            s.store_scale(272, 16, 0.2);
        }

        if (s.v[476] != 0.0) {
            s.store_square(261, 272);
        }

        if (s.v[476] != 0.0) {
            s.store_square(262, 109);
        }

        s.v[478] = if (s.v[109] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[478] != 0.0)) {
            s.store_div_ad(110, A::scale(s.ad_value(261), 0.5), A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)));
        }

        if ((s.v[476] != 0.0) && (!(s.v[478] != 0.0))) {
            s.store_scale_ad(110, A::add(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)), 0.5);
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(111, A::mul(s.ad_value(110), A::offset(s.ad_value(110), (p.p61 * p.p60))), A::scale(A::offset(s.ad_value(110), (p.p61 * s.v[31])), p.p60));
        }

        if (s.v[476] != 0.0) {
            s.store_div(265, 108, 111);
        }

        if (s.v[476] != 0.0) {
            s.store_scaled_offset(259, 265, (-1.0), 1.0 / (p.p62));
        }

        s.v[479] = if (s.v[265] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_ad(263, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), p.p62), 1.0);
        }

        if ((s.v[476] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_add_ad_rhs(263, 265, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), p.p62));
        }

        if (s.v[476] != 0.0) {
            s.store_scale(112, 263, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[476] != 0.0) {
            s.store_scale(113, 110, 1.0 / ((p.p61 * p.p60)));
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(114, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(112), 4.0), s.ad_value(113)), A::offset(s.ad_value(113), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(112), 2.0), A::offset(s.ad_value(113), 1.0)));
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(115, A::add(A::sub_from_scalar(1.0, s.ad_value(114)), A::mul(s.ad_value(106), s.ad_value(114))), A::offset(A::mul(s.ad_value(106), s.ad_value(114)), 1.0));
        }

        if (s.v[476] != 0.0) {
            s.store_scale_ad(117, A::mul(A::scale(s.ad_value(108), (0.5 * s.v[31])), s.ad_value(115)), s.v[8]);
        }

        if (s.v[476] != 0.0) {
            s.store_add_ad(266, A::scale(s.ad_value(117), 2.0), A::mul(s.ad_value(106), A::offset(A::add(s.ad_value(106), s.ad_value(117)), 1.0)));
        }

        if (s.v[476] != 0.0) {
            s.store_scaled_offset(118, 117, (-1.0), 0.5);
        }

        if (s.v[476] != 0.0) {
            s.store_add_ad_lhs(260, A::square(s.ad_value(118)), 266);
        }

        s.v[480] = if (s.v[117] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[480] != 0.0)) {
            s.store_add_ad_rhs(119, 118, A::sqrt(s.ad_value(260)));
        }

        if ((s.v[476] != 0.0) && (!(s.v[480] != 0.0))) {
            s.store_div_ad_rhs(119, 266, A::sub(A::sqrt(s.ad_value(260)), s.ad_value(118)));
        }

        s.v[481] = if (s.v[119] < p.p135) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[481] != 0.0)) {
            s.store_scalar(119, p.p135);
        }

        if (s.v[476] != 0.0) {
            s.store_mul_ad(121, A::mul(s.ad_value(119), A::offset(s.ad_value(119), 1.0)), A::exp(A::scale(s.ad_value(16), s.v[8])));
        }

        if (s.v[476] != 0.0) {
            s.store_scaled_offset(123, 108, (-p.p61), (0.5 * p.p60));
        }

        if (s.v[476] != 0.0) {
            s.store_scale(124, 108, ((p.p60 * s.v[31]) * p.p61));
        }

        if (s.v[476] != 0.0) {
            s.store_add_ad_rhs(125, 123, A::sqrt(A::add(A::square(s.ad_value(123)), s.ad_value(124))));
        }

        s.v[482] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[482] != 0.0)) {
            s.store_scale(126, 17, 0.1);
        }

        if ((s.v[476] != 0.0) && (!(s.v[482] != 0.0))) {
            s.store_mul_ad_rhs(126, 17, A::offset(A::div(A::scale(s.ad_value(108), 2.0), A::add(s.ad_value(108), s.ad_value(111))), 0.1));
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(127, A::scale(s.ad_value(108), p.p61), A::offset(s.ad_value(108), p.p61));
        }

        if (s.v[476] != 0.0) {
            s.store_div_from_scalar_ad(199, p.p61, A::offset(s.ad_value(108), p.p61));
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_div_ad(119, A::scale(s.ad_value(251), 2.0), A::offset(s.ad_value(104), 1.0));
        }

        if (!(s.v[476] != 0.0)) {
            s.copy_ad(121, 245);
        }

        s.v[483] = if ((((s.v[236]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[107]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[104] + s.v[105])))) { 1.0 } else { 0.0 };

        if ((!(s.v[476] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_scaled_add(128, 119, 106, 0.5);
        }

        if ((!(s.v[476] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_div_ad_rhs(115, 128, A::offset(s.ad_value(128), 1.0));
        }

        if ((!(s.v[476] != 0.0)) && (!(s.v[483] != 0.0))) {
            s.store_div_ad_rhs(115, 107, A::sub(A::add(s.ad_value(107), s.ad_value(231)), s.ad_value(230)));
        }

        if (!(s.v[476] != 0.0)) {
            s.copy_ad(125, 236);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scale(126, 17, 0.1);
        }

        if (!(s.v[476] != 0.0)) {
            s.copy_ad(127, 108);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(199, 1.0, A::scale(s.ad_value(127), 1.0 / (p.p61)));
        }

        s.store_scale(129, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(273, 14, 0.1);

        s.store_div_ad_lhs(259, A::sub(s.ad_value(232), s.ad_value(129)), 273);

        s.v[484] = if (s.v[232] < s.v[129]) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_sub_ad_rhs(130, 232, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[484] != 0.0)) {
            s.store_sub_ad_rhs(130, 129, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_ad(131, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(232), s.ad_value(130)), 3.0));

        s.v[485] = if (p.p73 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.copy_ad(132, 230);
        }

        s.v[486] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_add(132, 230, 125);
        }

        if ((!(s.v[485] != 0.0)) && (!(s.v[486] != 0.0))) {
            s.copy_ad(132, 231);
        }

        s.store_div_ad(133, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(134, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(133), ((-1.0) / p.p71))));

        s.store_div_ad_lhs(259, A::sub(s.ad_value(132), s.ad_value(134)), 126);

        s.v[487] = if (s.v[132] < s.v[134]) { 1.0 } else { 0.0 };

        if (s.v[487] != 0.0) {
            s.store_sub_ad_rhs(135, 132, A::mul(s.ad_value(126), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[487] != 0.0)) {
            s.store_sub_ad_rhs(135, 134, A::mul(s.ad_value(126), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_powf(136, 199, p.p75);

        s.store_add_ad(137, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::mul(s.ad_value(136), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(135), s.ad_value(17))), (1.0 - p.p71))))), A::mul(A::mul(s.ad_value(136), s.ad_value(133)), A::sub(s.ad_value(132), s.ad_value(135))));

        s.store_add_ad(138, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(137)), A::mul(s.ad_value(25), s.ad_value(230)));

        s.store_scale(139, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(140, 139, 246);

        s.store_div_ad_rhs(142, 140, A::offset(A::sqrt(A::offset(s.ad_value(140), 1.0)), 1.0));

        s.store_ad(122, &A::pow(s.ad_value(121), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(141, 139, 122);

        s.store_div_ad_rhs(143, 141, A::offset(A::sqrt(A::offset(s.ad_value(141), 1.0)), 1.0));

        s.v[488] = if (p.p91 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[488] != 0.0) {
            s.store_add_ad(144, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scale_ad(269, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), (s.v[99] * s.v[8]));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scale_ad(270, A::div(A::neg(s.ad_value(138)), s.ad_value(40)), (s.v[99] * s.v[8]));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scale_ad(144, A::sub(A::exp(s.ad_value(269)), A::exp(s.ad_value(270))), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 144);

        s.v[489] = if (s.v[144] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_div_from_scalar_ad(145, (0.5 * s.v[261]), A::sub(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144)));
        }

        if (!(s.v[489] != 0.0)) {
            s.store_scale_ad(145, A::add(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144)), 0.5);
        }

        s.store_mul_ad_rhs(146, 145, A::offset(A::scale(A::add(s.ad_value(142), s.ad_value(143)), 0.5), 1.0));

        s.store_mul_ad_lhs(147, A::scale(s.ad_value(35), p.p14), 122);

        s.store_mul(148, 35, 246);

        s.store_div_ad_lhs(149, A::sub(s.ad_value(148), s.ad_value(147)), 146);

        s.store_scale(259, 232, 10000.0);

        s.v[490] = if (s.v[232] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[490] != 0.0) {
            s.store_scale_ad(276, A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 0.0001);
        }

        if (!(s.v[490] != 0.0)) {
            s.store_add_ad_rhs(276, 232, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 0.0001));
        }

        s.store_scale(278, 276, 1.0 / (p.p139));

        s.v[491] = if (s.v[278] < p.p134) { 1.0 } else { 0.0 };

        if (s.v[491] != 0.0) {
            s.store_exp(279, 278);
        }

        if (!(s.v[491] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[491] != 0.0)) {
            s.store_mul_ad_rhs(279, 275, A::offset(A::offset(s.ad_value(278), (-p.p134)), 1.0));
        }

        s.store_scaled_offset(326, 279, (-1.0), s.v[325]);

        s.store_scaled_offset(259, 232, (-p.p141), 1000.0);

        s.v[492] = if (s.v[232] < p.p141) { 1.0 } else { 0.0 };

        if (s.v[492] != 0.0) {
            s.store_sub_ad_rhs(280, 232, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 0.001));
        }

        if (!(s.v[492] != 0.0)) {
            s.store_sub_from_scalar_ad(280, p.p141, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 0.001));
        }

        s.store_mul_ad(327, A::scale(s.ad_value(280), p.p142), A::powf(A::sub_from_scalar(p.p141, s.ad_value(280)), 2.0));

        s.v[493] = if (((s.v[232] * s.v[8]) / p.p16) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[493] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p16))));
        }

        if (!(s.v[493] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[493] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p16))), (-p.p134)), 1.0));
        }

        s.v[494] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[495] = if (((s.v[232] - s.v[55]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_exp_ad(278, A::scale(A::sub(s.ad_value(232), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_mul_ad_rhs(278, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(232), s.ad_value(55)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[496] = if (((s.v[149] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[494] != 0.0) && (s.v[496] != 0.0)) {
            s.store_exp_ad(279, A::offset(A::div(s.ad_value(149), s.ad_value(35)), (-1000.0)));
        }

        if ((s.v[494] != 0.0) && (!(s.v[496] != 0.0))) {
            s.store_scalar(275, ((40.0) as f64).exp());
        }

        if ((s.v[494] != 0.0) && (!(s.v[496] != 0.0))) {
            s.store_mul_ad_rhs(279, 275, A::offset(A::offset(A::offset(A::div(s.ad_value(149), s.ad_value(35)), (-1000.0)), (-40.0)), 1.0));
        }

        if (s.v[494] != 0.0) {
            let assign3700_ad_e3474: A = A::add(A::add(A::scale(A::offset(s.ad_value(276), (-1.0)), s.v[42]), A::mul(A::div(A::mul(A::scale(s.ad_value(53), 2.0), A::offset(s.ad_value(276), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(278), 4.0), 1.0)), 1.0)), A::offset(A::div(s.ad_value(138), s.ad_value(40)), 1.0))), A::div(A::mul(A::mul(s.ad_value(54), A::offset(s.ad_value(121), (-1.0))), s.ad_value(279)), A::offset(s.ad_value(279), 1.0)));
            s.store_ad(151, &assign3700_ad_e3474);
        }

        s.v[497] = if (p.p92 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[494] != 0.0)) && (s.v[497] != 0.0)) {
            s.store_scaled_offset(151, 276, (-1.0), s.v[42]);
        }

        if ((!(s.v[494] != 0.0)) && (!(s.v[497] != 0.0))) {
            s.store_scale_ad(151, A::add(A::scale(A::offset(s.ad_value(276), (-1.0)), (1.0 - p.p92)), A::mul(A::scale(A::offset(A::add(s.ad_value(276), s.ad_value(121)), (-2.0)), p.p92), A::offset(A::div(s.ad_value(138), s.ad_value(40)), 1.0))), s.v[42]);
        }

        s.v[498] = if (((s.v[233] * s.v[8]) / p.p18) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p18))));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[498] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p18))), (-p.p134)), 1.0));
        }

        s.v[499] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[500] = if (((s.v[233] - s.v[55]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if ((s.v[499] != 0.0) && (s.v[500] != 0.0)) {
            s.store_exp_ad(278, A::scale(A::sub(s.ad_value(233), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[499] != 0.0) && (!(s.v[500] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((s.v[499] != 0.0) && (!(s.v[500] != 0.0))) {
            s.store_mul_ad_rhs(278, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(233), s.ad_value(55)), s.v[8]), (-p.p134)), 1.0));
        }

        if (s.v[499] != 0.0) {
            s.store_add_ad(152, A::scale(A::offset(s.ad_value(276), (-1.0)), s.v[44]), A::div(A::mul(A::scale(s.ad_value(45), 2.0), A::offset(s.ad_value(276), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(278), 4.0), 1.0)), 1.0)));
        }

        if (!(s.v[499] != 0.0)) {
            s.store_scaled_offset(152, 276, (-1.0), s.v[44]);
        }

        s.v[501] = if (((s.v[232] * s.v[8]) / p.p20) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[501] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p20))));
        }

        if (!(s.v[501] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[501] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p20))), (-p.p134)), 1.0));
        }

        s.store_scaled_offset(153, 276, (-1.0), s.v[38]);

        s.v[502] = if (((s.v[233] * s.v[8]) / p.p22) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[502] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p22))));
        }

        if (!(s.v[502] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[502] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p22))), (-p.p134)), 1.0));
        }

        s.store_scaled_offset(155, 276, (-1.0), s.v[46]);

        s.v[503] = if (((s.v[235] * s.v[8]) / p.p31) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[503] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(235), (s.v[8] * 1.0 / (p.p31))));
        }

        if (!(s.v[503] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[503] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(235), (s.v[8] * 1.0 / (p.p31))), (-p.p134)), 1.0));
        }

        s.store_scaled_offset(154, 276, (-1.0), s.v[39]);

        s.v[504] = if (((s.v[233] * s.v[8]) / p.p133) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[504] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p133))));
        }

        if (!(s.v[504] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[504] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p133))), (-p.p134)), 1.0));
        }

        s.store_scaled_offset(156, 276, (-1.0), s.v[47]);

        s.v[505] = if (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[232] < 0.0)) { 1.0 } else { 0.0 };

        s.v[506] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p134) { 1.0 } else { 0.0 };

        if ((s.v[505] != 0.0) && (s.v[506] != 0.0)) {
            s.store_exp_ad(68, A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))));
        }

        if ((s.v[505] != 0.0) && (!(s.v[506] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((s.v[505] != 0.0) && (!(s.v[506] != 0.0))) {
            s.store_mul_ad_rhs(68, 275, A::offset(A::offset(A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))), (-p.p134)), 1.0));
        }

        if (s.v[505] != 0.0) {
            s.store_mul(255, 232, 65);
        }

        if (s.v[505] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(255)), 1e-30)), ((-2.0) - p.p66)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p66 * p.p66)), A::scale(s.ad_value(255), (3.0 * (p.p66 - 1.0)))), p.p66), A::mul(A::mul(A::scale(s.ad_value(255), 6.0), s.ad_value(255)), A::offset(s.ad_value(255), (p.p66 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[505] != 0.0) {
            s.store_div_ad(255, A::mul(A::scale(s.ad_value(232), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[507] = if (s.v[255] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[508] = if (s.v[255] < p.p134) { 1.0 } else { 0.0 };

        if (((s.v[505] != 0.0) && (s.v[507] != 0.0)) && (s.v[508] != 0.0)) {
            s.store_exp(91, 255);
        }

        if (((s.v[505] != 0.0) && (s.v[507] != 0.0)) && (!(s.v[508] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (((s.v[505] != 0.0) && (s.v[507] != 0.0)) && (!(s.v[508] != 0.0))) {
            s.store_mul_ad_rhs(91, 275, A::offset(A::offset(s.ad_value(255), (-p.p134)), 1.0));
        }

        if ((s.v[505] != 0.0) && (s.v[507] != 0.0)) {
            s.store_mul_ad(69, A::neg(s.ad_value(232)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(255)), 1.0));
        }

        if ((s.v[505] != 0.0) && (!(s.v[507] != 0.0))) {
            s.store_mul_ad(69, A::mul(A::scale(s.ad_value(232), 0.5), s.ad_value(255)), A::offset(A::mul(A::scale(s.ad_value(255), 0.3333333333333333), A::offset(A::scale(s.ad_value(255), 0.25), 1.0)), 1.0));
        }

        if (s.v[505] != 0.0) {
            s.store_scale_ad(57, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(58), 2.0), s.ad_value(69)), s.ad_value(59)), s.ad_value(68)), s.ad_value(65)), s.v[63]);
        }

        if (!(s.v[505] != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        if (!(s.v[505] != 0.0)) {
            s.store_scalar(57, 0.0);
        }

        s.v[509] = if (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[230] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[509] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(230), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[510] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p134) { 1.0 } else { 0.0 };

        if ((s.v[509] != 0.0) && (s.v[510] != 0.0)) {
            s.store_exp_ad(78, A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))));
        }

        if ((s.v[509] != 0.0) && (!(s.v[510] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((s.v[509] != 0.0) && (!(s.v[510] != 0.0))) {
            s.store_mul_ad_rhs(78, 275, A::offset(A::offset(A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))), (-p.p134)), 1.0));
        }

        if (s.v[509] != 0.0) {
            s.store_mul(257, 230, 67);
        }

        if (s.v[509] != 0.0) {
            let assign4300_ad_e4164: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(257)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(257), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(257), 6.0), s.ad_value(257)), A::offset(s.ad_value(257), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4300_ad_e4164);
        }

        if (s.v[509] != 0.0) {
            s.store_div_ad(257, A::mul(A::scale(s.ad_value(230), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[511] = if (s.v[257] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[512] = if (s.v[257] < p.p134) { 1.0 } else { 0.0 };

        if (((s.v[509] != 0.0) && (s.v[511] != 0.0)) && (s.v[512] != 0.0)) {
            s.store_exp(92, 257);
        }

        if (((s.v[509] != 0.0) && (s.v[511] != 0.0)) && (!(s.v[512] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (((s.v[509] != 0.0) && (s.v[511] != 0.0)) && (!(s.v[512] != 0.0))) {
            s.store_mul_ad_rhs(92, 275, A::offset(A::offset(s.ad_value(257), (-p.p134)), 1.0));
        }

        if ((s.v[509] != 0.0) && (s.v[511] != 0.0)) {
            s.store_mul_ad(81, A::neg(s.ad_value(230)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(257)), 1.0));
        }

        if ((s.v[509] != 0.0) && (!(s.v[511] != 0.0))) {
            s.store_mul_ad(81, A::mul(A::scale(s.ad_value(230), 0.5), s.ad_value(257)), A::offset(A::mul(A::scale(s.ad_value(257), 0.3333333333333333), A::offset(A::scale(s.ad_value(257), 0.25), 1.0)), 1.0));
        }

        if (s.v[509] != 0.0) {
            s.store_scale_ad(82, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(84), 2.0), s.ad_value(81)), s.ad_value(77)), s.ad_value(78)), s.ad_value(67)), s.v[89]);
        }

        if (!(s.v[509] != 0.0)) {
            s.store_scalar(81, 0.0);
        }

        if (!(s.v[509] != 0.0)) {
            s.store_scalar(82, 0.0);
        }

        s.store_mul(158, 139, 248);

        s.store_scale(159, 250, 4.0);

        s.store_div_ad(161, A::sub(s.ad_value(158), s.ad_value(139)), A::offset(A::sqrt(A::offset(s.ad_value(158), 1.0)), 1.0));

        s.store_div_ad_rhs(160, 159, A::offset(A::sqrt(A::offset(s.ad_value(159), 1.0)), 1.0));

        s.store_div_ad(157, A::scale(A::offset(s.ad_value(248), (-1.0)), (2.0 * s.v[43])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(248), ((4.0 * s.v[43]) / s.v[37])), 1.0)), 1.0));

        s.v[513] = if ((p.p5 > 0.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[513] != 0.0) {
            s.store_scale(157, 157, s.v[150]);
        }

        if (s.v[513] != 0.0) {
            s.store_div_ad(164, A::scale(A::offset(s.ad_value(249), (-1.0)), ((p.p32 * 2.0) * s.v[43])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(249), ((4.0 * s.v[43]) / s.v[37])), 1.0)), 1.0));
        }

        if (s.v[513] != 0.0) {
            s.store_scalar(165, 0.0);
        }

        s.v[514] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_scalar(271, ((p.p32 * s.v[43]) * s.v[32]));
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_scale_ad(166, A::sub_from_scalar(2.0, A::ln(A::scale(s.ad_value(271), s.v[8]))), s.v[6]);
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_sub(264, 241, 166);
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_scalar(261, (0.11 * 0.11));
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_square(262, 264);
        }

        s.v[515] = if (s.v[264] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[513] != 0.0) && (s.v[514] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_div_ad(167, A::scale(s.ad_value(261), 0.5), A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)));
        }

        if (((s.v[513] != 0.0) && (s.v[514] != 0.0)) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(167, A::add(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)), 0.5);
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_div_ad_rhs(168, 167, A::add(A::add(s.ad_value(271), A::scale(A::add(s.ad_value(164), s.ad_value(165)), s.v[32])), s.ad_value(167)));
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(166, 0.0);
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(264, 0.0);
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(167, 0.0);
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(168, 1.0);
        }

        if (s.v[513] != 0.0) {
            s.store_mul(169, 168, 164);
        }

        s.v[516] = if (p.p83 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[516] != 0.0) {
            s.store_add(322, 234, 230);
        }

        if (s.v[516] != 0.0) {
            s.store_scalar(261, (1e-6 * 1e-6));
        }

        if (s.v[516] != 0.0) {
            s.store_mul_ad_lhs(262, A::scale(s.ad_value(322), ((-1.0) * (-1.0))), 322);
        }

        s.v[517] = if (((-1.0) * s.v[322]) < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[516] != 0.0) && (s.v[517] != 0.0)) {
            s.store_div_ad(323, A::scale(s.ad_value(261), 0.5), A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), A::scale(s.ad_value(322), (-1.0))));
        }

        if ((s.v[516] != 0.0) && (!(s.v[517] != 0.0))) {
            s.store_scale_ad(323, A::add(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), A::scale(s.ad_value(322), (-1.0))), 0.5);
        }

        if (s.v[516] != 0.0) {
            s.store_scalar(324, (1.0 / (1.0 - ((s.v[318]) as f64).powf(p.p81))));
        }

        if (s.v[516] != 0.0) {
            s.store_scalar(319, (s.v[318] * p.p80));
        }

        if (s.v[516] != 0.0) {
            s.store_scale_ad(321, A::square(s.ad_value(324)), (((s.v[318]) as f64).powf((p.p81 - 1.0)) * (p.p81 * 1.0 / (p.p80))));
        }

        s.v[518] = if (s.v[323] < s.v[319]) { 1.0 } else { 0.0 };

        if ((s.v[516] != 0.0) && (s.v[518] != 0.0)) {
            s.store_div_from_scalar_ad(320, 1.0, A::sub_from_scalar(1.0, A::powf(A::scale(s.ad_value(323), 1.0 / (p.p80)), p.p81)));
        }

        if ((s.v[516] != 0.0) && (!(s.v[518] != 0.0))) {
            s.store_add_ad_rhs(320, 324, A::mul(A::sub(s.ad_value(323), s.ad_value(319)), s.ad_value(321)));
        }

        if (!(s.v[516] != 0.0)) {
            s.store_scalar(320, 1.0);
        }

        s.store_mul(82, 82, 320);

        s.store_mul(157, 157, 320);

        s.store_mul(154, 154, 320);

        s.store_mul(169, 169, 320);

        s.store_add_ad(172, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 172);

        s.v[519] = if (s.v[172] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[519] != 0.0) {
            s.store_div_from_scalar_ad(173, (0.5 * s.v[261]), A::sub(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172)));
        }

        if (!(s.v[519] != 0.0)) {
            s.store_scale_ad(173, A::add(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172)), 0.5);
        }

        s.store_mul_ad_rhs(174, 173, A::offset(A::scale(A::add(s.ad_value(142), s.ad_value(143)), 0.5), 1.0));

        s.store_div_from_scalar(176, s.v[29], 174);

        s.v[520] = if (s.v[176] < s.v[316]) { 1.0 } else { 0.0 };

        if (s.v[520] != 0.0) {
            s.copy_ad(176, 316);
        }

        s.store_scale(175, 176, 3.0);

        s.store_div_ad_lhs(177, A::add(A::scale(A::offset(s.ad_value(247), (-1.0)), (2.0 * s.v[6])), s.ad_value(234)), 175);

        s.v[521] = if (s.v[149] > 0.0) { 1.0 } else { 0.0 };

        s.v[522] = if (p.p38 == 1.0) { 1.0 } else { 0.0 };

        s.v[523] = if (s.v[230] < p.p43) { 1.0 } else { 0.0 };

        s.v[524] = if (((-s.v[149]) / p.p41) < p.p134) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (s.v[524] != 0.0)) {
            s.store_exp_ad(308, A::scale(A::neg(s.ad_value(149)), 1.0 / (p.p41)));
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[524] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[524] != 0.0))) {
            s.store_mul_ad_rhs(308, 275, A::offset(A::offset(A::scale(A::neg(s.ad_value(149)), 1.0 / (p.p41)), (-p.p134)), 1.0));
        }

        if (((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) {
            s.store_mul_ad_lhs(309, A::sub_from_scalar(p.p43, s.ad_value(230)), 308);
        }

        s.v[525] = if (((-s.v[310]) * ((s.v[309]) as f64).powf(p.p40)) < p.p134) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_exp_ad(313, A::mul(A::neg(s.ad_value(310)), A::powf(s.ad_value(309), p.p40)));
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_mul_ad_rhs(313, 275, A::offset(A::offset(A::mul(A::neg(s.ad_value(310)), A::powf(s.ad_value(309), p.p40)), (-p.p134)), 1.0));
        }

        if (((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) {
            s.store_mul_ad_lhs(196, A::mul(A::div_from_scalar(p.p39, s.ad_value(310)), s.ad_value(309)), 313);
        }

        s.v[526] = if (p.p38 == 2.0) { 1.0 } else { 0.0 };

        s.v[527] = if (s.v[230] < s.v[16]) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_scalar(185, ((2.0 * p.p45) / (p.p44 * p.p44)));
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad_lhs(260, A::sub(s.ad_value(16), s.ad_value(230)), 199);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_sqrt_ad(186, A::div(A::scale(s.ad_value(260), 2.0), s.ad_value(185)));
        }

        s.v[528] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_scalar(187, p.p44);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[528] != 0.0))) {
            s.store_sub_from_scalar_ad(116, 1.0, A::scale(s.ad_value(115), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[528] != 0.0))) {
            s.store_mul_ad_lhs(187, A::scale(s.ad_value(116), p.p44), 116);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad(188, A::mul(s.ad_value(186), s.ad_value(187)), A::sqrt(A::add(A::square(s.ad_value(186)), A::square(s.ad_value(187)))));
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad_lhs(189, A::sub(s.ad_value(16), s.ad_value(230)), 188);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_add_ad_rhs(190, 189, A::mul(A::mul(A::scale(s.ad_value(188), 0.5), s.ad_value(185)), s.ad_value(199)));
        }

        s.v[529] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[529] != 0.0)) {
            s.copy_ad(191, 190);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_offset_ad(192, A::scale(A::offset(A::scale(s.ad_value(115), 2.0), 1.0), (2.0 * p.p46)), 1.0);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_scalar(193, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_sub_ad_rhs(194, 189, A::mul(A::mul(A::scale(s.ad_value(188), 0.5), s.ad_value(185)), A::sub(s.ad_value(193), A::div(s.ad_value(149), A::scale(s.ad_value(192), p.p61)))));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_add_ad(260, A::mul(A::sub(s.ad_value(194), s.ad_value(190)), A::sub(s.ad_value(194), s.ad_value(190))), A::scale(A::mul(A::mul(A::scale(s.ad_value(189), 0.1), s.ad_value(189)), s.ad_value(127)), 1.0 / (p.p61)));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_scale_ad(191, A::add(A::add(s.ad_value(194), s.ad_value(190)), A::sqrt(s.ad_value(260))), 0.5);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad_lhs(267, A::sub(s.ad_value(191), s.ad_value(189)), 191);
        }

        s.v[530] = if (((s.v[267]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_div_ad_lhs(195, A::scale(s.ad_value(188), 0.5), 267);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(196, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(191)), s.ad_value(195)), A::sub(A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(191))), A::exp(A::mul(A::div(A::neg(s.ad_value(98)), s.ad_value(191)), A::offset(A::div(s.ad_value(187), s.ad_value(195)), 1.0)))));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[530] != 0.0))) {
            s.store_mul_ad(196, A::mul(s.ad_value(0), s.ad_value(187)), A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(191))));
        }

        s.v[531] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        s.v[532] = if (s.v[230] < p.p43) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad(200, A::powf(A::sub_from_scalar(p.p43, s.ad_value(230)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(149), A::offset(s.ad_value(149), p.p47))), p.p48));
        }

        s.v[533] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.copy_ad(201, 200);
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_scaled_offset(202, 149, (-p.p51), 1.0 / (p.p47));
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_scaled_offset(259, 202, (-1.0), 1.0 / (p.p50));
        }

        s.v[534] = if (s.v[202] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) && (s.v[534] != 0.0)) {
            s.store_offset_ad(203, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), p.p50), 1.0);
        }

        if (((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) && (!(s.v[534] != 0.0))) {
            s.store_add_ad_rhs(203, 202, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), p.p50));
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_mul_ad_rhs(201, 200, A::powf(s.ad_value(203), p.p49));
        }

        s.v[535] = if (((-s.v[310]) * s.v[201]) < p.p134) { 1.0 } else { 0.0 };

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_exp_ad(313, A::mul(A::neg(s.ad_value(310)), s.ad_value(201)));
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_mul_ad_rhs(313, 275, A::offset(A::offset(A::mul(A::neg(s.ad_value(310)), s.ad_value(201)), (-p.p134)), 1.0));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad_lhs(196, A::mul(A::div_from_scalar(p.p39, s.ad_value(310)), A::sub_from_scalar(p.p43, s.ad_value(230))), 313);
        }

        s.v[536] = if (s.v[196] > 0.0) { 1.0 } else { 0.0 };

        s.v[537] = if (p.p52 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_add_ad(197, A::add(A::div_from_scalar(s.v[6], A::mul(s.ad_value(149), A::add(s.ad_value(30), s.ad_value(175)))), A::scale(A::div(s.ad_value(146), s.ad_value(35)), s.v[42])), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(175))));
        }

        s.v[538] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) {
            s.store_scaled_sub(259, 196, 197, 1000000.0);
        }

        s.v[539] = if (s.v[196] < s.v[197]) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) && (s.v[539] != 0.0)) {
            s.store_sub_ad_rhs(196, 196, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 1e-6));
        }

        if (((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_sub_ad_rhs(196, 197, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 1e-6));
        }

        if ((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) {
            s.store_mul(198, 149, 196);
        }

        if ((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[538] != 0.0))) {
            s.store_div_ad(198, A::mul(A::mul(s.ad_value(149), s.ad_value(196)), s.ad_value(197)), A::add(s.ad_value(196), s.ad_value(197)));
        }

        if (((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (!(s.v[537] != 0.0))) {
            s.store_mul(198, 149, 196);
        }

        s.store_mul_ad_lhs(204, A::scale(s.ad_value(23), (1.0 - p.p67)), 131);

        s.store_div_ad_lhs(259, A::sub(s.ad_value(233), s.ad_value(129)), 273);

        s.v[541] = if (s.v[233] < s.v[129]) { 1.0 } else { 0.0 };

        if (s.v[541] != 0.0) {
            s.store_sub_ad_rhs(205, 233, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[541] != 0.0)) {
            s.store_sub_ad_rhs(205, 129, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_mul_ad(206, A::scale(s.ad_value(23), p.p67), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(205), s.ad_value(65))), (1.0 - p.p66)))), A::scale(A::sub(s.ad_value(233), s.ad_value(205)), 3.0)));

        s.store_mul_ad_lhs(207, A::scale(s.ad_value(24), p.p76), 138);

        s.v[208] = (s.v[94] * s.v[36]);

        s.store_mul_ad_lhs(212, A::scale(s.ad_value(142), (0.5 * s.v[208])), 173);

        s.store_mul_ad_lhs(213, A::scale(s.ad_value(143), (0.5 * s.v[208])), 173);

        s.store_scale(274, 17, 0.1);

        s.store_div_ad_lhs(259, A::sub(s.ad_value(235), s.ad_value(134)), 274);

        s.v[542] = if (s.v[235] < s.v[134]) { 1.0 } else { 0.0 };

        if (s.v[542] != 0.0) {
            s.store_sub_ad_rhs(214, 235, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[542] != 0.0)) {
            s.store_sub_ad_rhs(214, 134, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_add_ad(215, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(214), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(133), A::sub(s.ad_value(235), s.ad_value(214))));

        s.store_scale_ad(216, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(215)), A::mul(s.ad_value(25), s.ad_value(235)))), ((1.0 - p.p76) * (1.0 - p.p32)));

        s.store_div_ad_lhs(259, A::sub(s.ad_value(241), s.ad_value(134)), 274);

        s.v[543] = if (s.v[241] < s.v[134]) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_sub_ad_rhs(217, 241, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[543] != 0.0)) {
            s.store_sub_ad_rhs(217, 134, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_add_ad(218, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(133), A::sub(s.ad_value(241), s.ad_value(217))));

        s.store_scale_ad(219, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(218)), A::mul(s.ad_value(25), s.ad_value(241)))), ((1.0 - p.p76) * p.p32));

        s.store_scale_ad(220, A::powf(A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p84)), (s.v[93] * s.v[36]));

        s.v[544] = if ((s.v[232] / (p.p84 * s.v[6])) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(232), 1.0 / ((p.p84 * s.v[6]))));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[544] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(232), 1.0 / ((p.p84 * s.v[6]))), (-p.p134)), 1.0));
        }

        s.store_mul(222, 220, 276);

        s.v[223] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_ad(224, A::scale(s.ad_value(115), (0.5 * s.v[223])), A::offset(A::add(s.ad_value(119), s.ad_value(106)), 2.0));

        s.v[545] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_scale_ad(229, A::add(A::scale(s.ad_value(161), s.v[208]), A::scale(s.ad_value(160), s.v[223])), ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[546] = if ((((s.v[235] - s.v[22]) / p.p90) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if ((!(s.v[545] != 0.0)) && (s.v[546] != 0.0)) {
            s.store_exp_ad(170, A::scale(A::scale(A::sub(s.ad_value(235), s.ad_value(22)), 1.0 / (p.p90)), s.v[8]));
        }

        if ((!(s.v[545] != 0.0)) && (!(s.v[546] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((!(s.v[545] != 0.0)) && (!(s.v[546] != 0.0))) {
            s.store_mul_ad_rhs(170, 275, A::offset(A::offset(A::scale(A::scale(A::sub(s.ad_value(235), s.ad_value(22)), 1.0 / (p.p90)), s.v[8]), (-p.p134)), 1.0));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_div_ad(229, A::scale(s.ad_value(248), ((2.0 * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(170), 4.0), 1.0)), 1.0));
        }

        s.v[547] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[547] != 0.0) {
            s.store_scale(229, 229, s.v[150]);
        }

        s.v[548] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_mul(162, 139, 249);
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_div_ad(163, A::sub(s.ad_value(162), s.ad_value(139)), A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0));
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_scale(225, 252, 4.0);
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_div_ad_rhs(226, 225, A::offset(A::sqrt(A::offset(s.ad_value(225), 1.0)), 1.0));
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_scale_ad(227, A::add(A::scale(s.ad_value(163), s.v[208]), A::scale(s.ad_value(226), s.v[223])), (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[549] = if (((s.v[241] - s.v[22]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) && (s.v[549] != 0.0)) {
            s.store_exp_ad(171, A::scale(A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]));
        }

        if (((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) && (!(s.v[549] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) && (!(s.v[549] != 0.0))) {
            s.store_mul_ad_rhs(171, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]), (-p.p134)), 1.0));
        }

        if ((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_div_ad(227, A::scale(s.ad_value(249), (((2.0 * p.p32) * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(171), 4.0), 1.0)), 1.0));
        }

        if (s.v[547] != 0.0) {
            s.store_mul(228, 168, 227);
        }

        s.v[550] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[550] != 0.0) {
            s.store_offset_ad(179, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (-p.p66)), (-3.0));
        }

        if (s.v[550] != 0.0) {
            s.store_div_ad_lhs(268, A::sub(s.ad_value(232), s.ad_value(129)), 273);
        }

        s.v[551] = if (s.v[268] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[550] != 0.0) && (s.v[551] != 0.0)) {
            s.store_div_from_scalar_ad(180, 1.0, A::offset(A::exp(s.ad_value(268)), 1.0));
        }

        if ((s.v[550] != 0.0) && (!(s.v[551] != 0.0))) {
            s.store_div_ad(180, A::exp(A::neg(s.ad_value(268))), A::offset(A::exp(A::neg(s.ad_value(268))), 1.0));
        }

        if (s.v[550] != 0.0) {
            s.store_offset_ad(178, A::mul(s.ad_value(179), s.ad_value(180)), 3.0);
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad_lhs(181, A::scale(s.ad_value(23), (1.0 - p.p67)), 178);
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad(184, A::div(A::scale(A::mul(s.ad_value(139), s.ad_value(246)), s.v[8]), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(140), 1.0))));
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad_lhs(182, A::scale(s.ad_value(173), (0.5 * s.v[208])), 184);
        }

        if (s.v[550] != 0.0) {
            s.store_scale(183, 222, 1.0 / ((p.p84 * s.v[6])));
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad(211, A::scale(s.ad_value(234), 0.2), A::add(A::add(s.ad_value(181), s.ad_value(182)), s.ad_value(183)));
        }

        if (s.v[550] != 0.0) {
            s.store_scale(221, 222, (1.0 - p.p94));
        }

        if (s.v[550] != 0.0) {
            s.store_add_ad_rhs(307, 212, A::scale(s.ad_value(222), p.p94));
        }

        if (s.v[550] != 0.0) {
            s.store_add_ad_lhs(210, A::scale(s.ad_value(307), p.p93), 213);
        }

        if (s.v[550] != 0.0) {
            s.store_scale(209, 307, (1.0 - p.p93));
        }

        if (!(s.v[550] != 0.0)) {
            s.copy_ad(209, 212);
        }

        if (!(s.v[550] != 0.0)) {
            s.copy_ad(210, 213);
        }

        if (!(s.v[550] != 0.0)) {
            s.copy_ad(221, 222);
        }

        s.v[552] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[553] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        s.v[554] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.v[281] = ((4.0 * 1.3806226e-23) * s.v[2]);

        s.store_div_from_scalar(282, s.v[281], 28);

        s.store_div_from_scalar(283, s.v[281], 30);

        s.store_scale(284, 101, s.v[281]);

        s.store_scale(285, 102, s.v[281]);

        s.store_scale(286, 103, s.v[281]);

        s.store_scale_ad(287, A::mul(A::div_from_scalar(s.v[281], s.ad_value(175)), A::offset(A::scale(s.ad_value(247), 4.0), 5.0)), 0.3333333333333333);

        s.store_div_ad_lhs(303, A::add(s.ad_value(148), s.ad_value(147)), 146);

        s.store_scale_ad(288, A::abs(s.ad_value(303)), (2.0 * 1.6021918e-19));

        s.v[555] = if (p.p129 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_abs_ad(304, A::div(s.ad_value(198), s.ad_value(303)));
        }

        if (!(s.v[555] != 0.0)) {
            s.store_scalar(304, 0.0);
        }

        s.store_mul_ad(300, A::scale(s.ad_value(198), (2.0 * 1.6021918e-19)), A::offset(s.ad_value(304), 1.0));

        s.v[556] = if (s.v[303] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_div_ad_lhs(305, A::add(s.ad_value(209), s.ad_value(210)), 303);
        }

        if (!(s.v[556] != 0.0)) {
            s.store_mul_ad_lhs(305, A::scale(s.ad_value(173), s.v[94]), 146);
        }

        s.v[557] = if (p.p130 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[557] != 0.0) {
            s.store_scale(306, 305, p.p93);
        }

        s.v[558] = if (p.p130 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[557] != 0.0)) && (s.v[558] != 0.0)) {
            s.store_scale(306, 305, p.p131);
        }

        if ((!(s.v[557] != 0.0)) && (!(s.v[558] != 0.0))) {
            s.store_scalar(306, 0.0);
        }

        s.store_scale_ad(289, A::abs(A::add(A::add(A::sub(A::add(s.ad_value(151), s.ad_value(153)), s.ad_value(57)), s.ad_value(327)), s.ad_value(326))), (2.0 * 1.6021918e-19));

        s.store_add(301, 151, 152);

        s.store_scale_ad(290, A::powf(A::abs(s.ad_value(301)), p.p125), p.p127);

        s.v[559] = if (s.v[301] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[559] != 0.0) {
            s.store_neg(290, 290);
        }

        s.store_add_ad_lhs(302, A::add(s.ad_value(153), s.ad_value(155)), 156);

        s.store_scale_ad(291, A::powf(A::abs(s.ad_value(302)), p.p126), p.p128);

        s.v[560] = if (s.v[302] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[560] != 0.0) {
            s.store_neg(291, 291);
        }

        s.store_scale_ad(292, A::abs(A::add(A::add(s.ad_value(152), s.ad_value(155)), s.ad_value(156))), (2.0 * 1.6021918e-19));

        s.store_scale_ad(293, A::abs(s.ad_value(154)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(294, A::powf(A::abs(s.ad_value(154)), p.p125), p.p127);

        s.v[561] = if (s.v[154] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_neg(294, 294);
        }

        s.store_scale_ad(295, A::abs(s.ad_value(82)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(296, A::abs(s.ad_value(157)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(298, A::powf(A::scale(A::abs(s.ad_value(157)), 1.0 / ((1.0 - (p.p5 * p.p32)))), p.p125), (p.p127 * (1.0 - (p.p5 * p.p32))));

        s.v[562] = if (s.v[157] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[562] != 0.0) {
            s.store_neg(298, 298);
        }

        s.store_scale_ad(297, A::abs(s.ad_value(169)), ((2.0 * 1.6021918e-19) * p.p5));

        s.v[563] = if (p.p32 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[563] != 0.0) {
            s.store_scalar(299, 0.0);
        }

        if (!(s.v[563] != 0.0)) {
            s.store_scale_ad(299, A::powf(A::scale(A::abs(s.ad_value(169)), 1.0 / (p.p32)), p.p125), ((p.p127 * p.p5) * p.p32));
        }

        s.v[564] = if (s.v[169] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[564] != 0.0) {
            s.store_neg(299, 299);
        }

        s.v[565] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[566] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        s.v[567] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.v[568] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[439] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[439] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[439] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[439] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[439] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[150] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[440] = if (p.p137 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[440] != 0.0) {
            s.store_scalar(315, 1e-12);
        }

        if (!(s.v[440] != 0.0)) {
            s.store_scalar(315, p.p137);
        }

        s.store_scale(316, 315, p.p1);

        s.v[52] = 0.001;

        s.v[312] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[259] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.v[441] = if ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[441] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[441] != 0.0)) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[259] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.v[442] = if ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[442] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[442] != 0.0)) {
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

        s.store_scale_ad(259, A::offset(A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), (-0.05)), 10.0);

        s.v[443] = if ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[443] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[443] != 0.0)) {
            s.store_add_ad(70, A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 0.1));
        }

        s.store_scale_ad(259, A::offset(A::offset(s.ad_value(88), (-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))), (-0.05)), 10.0);

        s.v[444] = if ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[444] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[444] != 0.0)) {
            s.store_add_ad(85, A::offset(s.ad_value(88), (-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p65 * s.v[4])) + ((1.0 - s.v[4]) * p.p104));

        s.v[259] = ((0.05 - s.v[13]) / s.v[6]);

        s.v[445] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[445] != 0.0) {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[445] != 0.0)) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p63 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[15]) / s.v[6]);

        s.v[446] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[446] != 0.0) {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[446] != 0.0)) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p79 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[21]) / s.v[6]);

        s.v[447] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[447] != 0.0) {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[447] != 0.0)) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p70 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[18]) / s.v[6]);

        s.v[448] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[448] != 0.0) {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[448] != 0.0)) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[254]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[20]) / s.v[6]);

        s.v[449] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[449] != 0.0) {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[449] != 0.0)) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p26 * s.v[4])) + ((1.0 - s.v[4]) * p.p108));

        s.v[259] = ((0.05 - s.v[56]) / s.v[6]);

        s.v[450] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[450] != 0.0) {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[450] != 0.0)) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p65), p.p66);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p64);

        s.store_offset_ad(26, A::scale(A::powf(A::div_from_scalar(p.p70, s.ad_value(17)), p.p71), (1.0 - p.p74)), p.p74);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p69);

        s.store_scale(25, 27, p.p74);

        s.v[28] = (p.p53 * (((s.v[254] * p.p96)) as f64).exp());

        s.v[451] = if (s.v[28] < s.v[316]) { 1.0 } else { 0.0 };

        if (s.v[451] != 0.0) {
            s.copy_ad(28, 316);
        }

        s.v[29] = (p.p55 * (((s.v[254] * (p.p97 - p.p95))) as f64).exp());

        s.v[30] = (p.p54 * (((s.v[254] * p.p100)) as f64).exp());

        s.v[452] = if (s.v[30] < s.v[316]) { 1.0 } else { 0.0 };

        if (s.v[452] != 0.0) {
            s.copy_ad(30, 316);
        }

        s.v[32] = (p.p56 * (((s.v[254] * p.p101)) as f64).exp());

        s.v[31] = (p.p59 * (((s.v[254] * p.p98)) as f64).exp());

        s.v[453] = if (p.p121 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[453] != 0.0) {
            s.store_scalar(50, (p.p9 * (1.0 + (s.v[12] * p.p121))));
        }

        if (s.v[453] != 0.0) {
            s.store_scaled_offset(259, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[454] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[453] != 0.0) && (s.v[454] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[453] != 0.0) && (!(s.v[454] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), s.v[52]));
        }

        if (s.v[453] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[453] != 0.0)) {
            s.store_scalar(48, p.p9);
        }

        s.v[455] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[455] != 0.0) {
            s.store_scalar(51, (p.p10 * (1.0 + (s.v[12] * p.p122))));
        }

        if (s.v[455] != 0.0) {
            s.store_scaled_offset(259, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[456] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[455] != 0.0) && (s.v[456] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[455] != 0.0) && (!(s.v[456] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), s.v[52]));
        }

        if (s.v[455] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[455] != 0.0)) {
            s.store_scalar(49, p.p10);
        }

        s.v[311] = (p.p42 * (1.0 + (p.p123 * s.v[12])));

        s.v[261] = (s.v[312] * s.v[312]);

        s.v[262] = (s.v[311] * s.v[311]);

        s.v[457] = if (s.v[311] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[457] != 0.0) {
            s.store_scalar(310, ((0.5 * s.v[261]) / ((((s.v[262] + s.v[261])) as f64).sqrt() - s.v[311])));
        }

        if (!(s.v[457] != 0.0)) {
            s.store_scalar(310, (0.5 * ((((s.v[262] + s.v[261])) as f64).sqrt() + s.v[311])));
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div_from_scalar((s.v[254] * (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), p.p8), A::exp(A::div_from_scalar(((-p.p104) * s.v[10]), s.ad_value(48))));

        s.v[36] = (p.p11 * (((s.v[254] * (1.0 - p.p97))) as f64).exp());

        s.v[37] = (p.p29 * (((s.v[254] * (1.0 - p.p102))) as f64).exp());

        s.v[42] = ((p.p15 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p16)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p16)) as f64).exp());

        s.v[43] = ((p.p28 * (((s.v[254] * ((4.0 - p.p102) + p.p120))) as f64).exp()) * ((((-p.p111) * s.v[10])) as f64).exp());

        s.store_powf_ad(255, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(256, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p34), s.ad_value(70)), s.ad_value(255)), s.ad_value(256)), p.p65), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(257, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(258, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p36), s.ad_value(85)), s.ad_value(257)), s.ad_value(258)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.v[255] = (((s.v[254] * p.p95)) as f64).exp();

        s.store_scale(40, 27, (p.p13 * s.v[255]));

        s.store_scale(41, 256, (p.p12 * s.v[255]));

        s.v[93] = ((p.p85 * (((s.v[254] * (p.p97 - 2.0))) as f64).exp()) * ((((-p.p119) * s.v[10])) as f64).exp());

        s.v[94] = (p.p86 * (((s.v[254] * ((p.p95 + p.p97) - 1.0))) as f64).exp());

        s.v[95] = (p.p87 * (((s.v[254] * (p.p98 - 1.0))) as f64).exp());

        s.v[96] = ((p.p88 * (s.v[94] + s.v[95])) / (p.p86 + p.p87));

        s.v[97] = (p.p89 * (((s.v[254] * (p.p99 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.v[459] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[459] != 0.0) {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!(s.v[459] != 0.0)) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p91 * (((s.v[254] * p.p95)) as f64).exp());

        s.store_ad(230, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), p.p3));

        s.store_ad(231, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(7)), p.p3));

        s.store_ad(232, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(3)), p.p3));

        s.store_ad(233, &A::scale(A::voltage(ctx, &nodes, Some(4), Some(3)), p.p3));

        s.store_ad(234, &A::scale(A::voltage(ctx, &nodes, Some(4), Some(5)), p.p3));

        s.store_ad(236, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(240, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(4)), p.p3));

        s.store_ad(243, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(244, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(238, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(6)), p.p3));

        s.store_ad(237, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(9)), p.p3));

        s.store_sub_ad_lhs(235, A::sub(A::add(s.ad_value(234), s.ad_value(231)), s.ad_value(236)), 238);

        s.store_sub_ad_lhs(242, A::add(A::sub(s.ad_value(240), s.ad_value(244)), s.ad_value(235)), 237);

        s.store_add(241, 244, 242);

        s.v[466] = if ((s.v[231] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[466] != 0.0) {
            s.store_exp_ad(245, A::scale(s.ad_value(231), s.v[8]));
        }

        if (!(s.v[466] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[466] != 0.0)) {
            s.store_mul_ad_rhs(245, 275, A::offset(A::offset(A::scale(s.ad_value(231), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[467] = if (((s.v[232] * s.v[8]) / s.v[48]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[467] != 0.0) {
            s.store_exp_ad(246, A::div(A::scale(s.ad_value(232), s.v[8]), s.ad_value(48)));
        }

        if (!(s.v[467] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[467] != 0.0)) {
            s.store_mul_ad_rhs(246, 275, A::offset(A::offset(A::div(A::scale(s.ad_value(232), s.v[8]), s.ad_value(48)), (-p.p134)), 1.0));
        }

        s.v[468] = if ((s.v[235] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[468] != 0.0) {
            s.store_exp_ad(248, A::scale(s.ad_value(235), s.v[8]));
        }

        if (!(s.v[468] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[468] != 0.0)) {
            s.store_mul_ad_rhs(248, 275, A::offset(A::offset(A::scale(s.ad_value(235), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[469] = if ((s.v[234] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (!(s.v[469] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.v[470] = if ((s.v[241] * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[470] != 0.0) {
            s.store_exp_ad(249, A::scale(s.ad_value(241), s.v[8]));
        }

        if (!(s.v[470] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[470] != 0.0)) {
            s.store_mul_ad_rhs(249, 275, A::offset(A::offset(A::scale(s.ad_value(241), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[471] = if (((s.v[241] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[471] != 0.0) {
            s.store_exp_ad(252, A::scale(A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[471] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[471] != 0.0)) {
            s.store_mul_ad_rhs(252, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[472] = if (((s.v[235] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[472] != 0.0) {
            s.store_exp_ad(250, A::scale(A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[472] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[472] != 0.0)) {
            s.store_mul_ad_rhs(250, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[473] = if (((s.v[231] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[473] != 0.0) {
            s.store_exp_ad(251, A::scale(A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[473] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[473] != 0.0)) {
            s.store_mul_ad_rhs(251, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[474] = if (((s.v[230] - s.v[16]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[474] != 0.0) {
            s.store_exp_ad(253, A::scale(A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[474] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[474] != 0.0)) {
            s.store_mul_ad_rhs(253, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]), (-p.p134)), 1.0));
        }

        s.store_sqrt_ad(104, A::offset(A::scale(s.ad_value(251), 4.0), 1.0));

        s.store_sqrt_ad(105, A::offset(A::scale(s.ad_value(253), 4.0), 1.0));

        s.store_div_ad(106, A::scale(s.ad_value(253), 2.0), A::offset(s.ad_value(105), 1.0));

        s.v[475] = if (s.v[106] < p.p136) { 1.0 } else { 0.0 };

        if (s.v[475] != 0.0) {
            s.store_scalar(106, p.p136);
        }

        s.store_scale_ad(107, A::sub(A::sub(s.ad_value(104), s.ad_value(105)), A::ln(A::div(A::offset(s.ad_value(104), 1.0), A::offset(s.ad_value(105), 1.0)))), s.v[6]);

        s.store_scaled_add(108, 107, 236, 1.0 / (s.v[31]));

        s.v[476] = if (s.v[108] > 0.0) { 1.0 } else { 0.0 };

        s.v[477] = if (s.v[230] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[477] != 0.0)) {
            s.copy_ad(277, 230);
        }

        if ((s.v[476] != 0.0) && (!(s.v[477] != 0.0))) {
            s.store_offset_ad(277, A::ln(A::offset(A::offset(s.ad_value(230), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[476] != 0.0) {
            s.store_sub_ad_lhs(109, A::add(s.ad_value(16), A::scale(A::ln(A::offset(A::scale(s.ad_value(108), (0.5 * (s.v[31] * s.v[8]))), 1.0)), (2.0 * s.v[6]))), 277);
        }

        if (s.v[476] != 0.0) {
            s.store_scale(272, 16, 0.2);
        }

        if (s.v[476] != 0.0) {
            s.store_square(261, 272);
        }

        if (s.v[476] != 0.0) {
            s.store_square(262, 109);
        }

        s.v[478] = if (s.v[109] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[478] != 0.0)) {
            s.store_div_ad(110, A::scale(s.ad_value(261), 0.5), A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)));
        }

        if ((s.v[476] != 0.0) && (!(s.v[478] != 0.0))) {
            s.store_scale_ad(110, A::add(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)), 0.5);
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(111, A::mul(s.ad_value(110), A::offset(s.ad_value(110), (p.p61 * p.p60))), A::scale(A::offset(s.ad_value(110), (p.p61 * s.v[31])), p.p60));
        }

        if (s.v[476] != 0.0) {
            s.store_div(265, 108, 111);
        }

        if (s.v[476] != 0.0) {
            s.store_scaled_offset(259, 265, (-1.0), 1.0 / (p.p62));
        }

        s.v[479] = if (s.v[265] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[479] != 0.0)) {
            s.store_offset_ad(263, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), p.p62), 1.0);
        }

        if ((s.v[476] != 0.0) && (!(s.v[479] != 0.0))) {
            s.store_add_ad_rhs(263, 265, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), p.p62));
        }

        if (s.v[476] != 0.0) {
            s.store_scale(112, 263, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[476] != 0.0) {
            s.store_scale(113, 110, 1.0 / ((p.p61 * p.p60)));
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(114, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(112), 4.0), s.ad_value(113)), A::offset(s.ad_value(113), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(112), 2.0), A::offset(s.ad_value(113), 1.0)));
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(115, A::add(A::sub_from_scalar(1.0, s.ad_value(114)), A::mul(s.ad_value(106), s.ad_value(114))), A::offset(A::mul(s.ad_value(106), s.ad_value(114)), 1.0));
        }

        if (s.v[476] != 0.0) {
            s.store_scale_ad(117, A::mul(A::scale(s.ad_value(108), (0.5 * s.v[31])), s.ad_value(115)), s.v[8]);
        }

        if (s.v[476] != 0.0) {
            s.store_add_ad(266, A::scale(s.ad_value(117), 2.0), A::mul(s.ad_value(106), A::offset(A::add(s.ad_value(106), s.ad_value(117)), 1.0)));
        }

        if (s.v[476] != 0.0) {
            s.store_scaled_offset(118, 117, (-1.0), 0.5);
        }

        if (s.v[476] != 0.0) {
            s.store_add_ad_lhs(260, A::square(s.ad_value(118)), 266);
        }

        s.v[480] = if (s.v[117] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[480] != 0.0)) {
            s.store_add_ad_rhs(119, 118, A::sqrt(s.ad_value(260)));
        }

        if ((s.v[476] != 0.0) && (!(s.v[480] != 0.0))) {
            s.store_div_ad_rhs(119, 266, A::sub(A::sqrt(s.ad_value(260)), s.ad_value(118)));
        }

        s.v[481] = if (s.v[119] < p.p135) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[481] != 0.0)) {
            s.store_scalar(119, p.p135);
        }

        if (s.v[476] != 0.0) {
            s.store_mul_ad(121, A::mul(s.ad_value(119), A::offset(s.ad_value(119), 1.0)), A::exp(A::scale(s.ad_value(16), s.v[8])));
        }

        if (s.v[476] != 0.0) {
            s.store_scaled_offset(123, 108, (-p.p61), (0.5 * p.p60));
        }

        if (s.v[476] != 0.0) {
            s.store_scale(124, 108, ((p.p60 * s.v[31]) * p.p61));
        }

        if (s.v[476] != 0.0) {
            s.store_add_ad_rhs(125, 123, A::sqrt(A::add(A::square(s.ad_value(123)), s.ad_value(124))));
        }

        s.v[482] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[476] != 0.0) && (s.v[482] != 0.0)) {
            s.store_scale(126, 17, 0.1);
        }

        if ((s.v[476] != 0.0) && (!(s.v[482] != 0.0))) {
            s.store_mul_ad_rhs(126, 17, A::offset(A::div(A::scale(s.ad_value(108), 2.0), A::add(s.ad_value(108), s.ad_value(111))), 0.1));
        }

        if (s.v[476] != 0.0) {
            s.store_div_ad(127, A::scale(s.ad_value(108), p.p61), A::offset(s.ad_value(108), p.p61));
        }

        if (s.v[476] != 0.0) {
            s.store_div_from_scalar_ad(199, p.p61, A::offset(s.ad_value(108), p.p61));
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_div_ad(119, A::scale(s.ad_value(251), 2.0), A::offset(s.ad_value(104), 1.0));
        }

        if (!(s.v[476] != 0.0)) {
            s.copy_ad(121, 245);
        }

        s.v[483] = if ((((s.v[236]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[107]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[104] + s.v[105])))) { 1.0 } else { 0.0 };

        if ((!(s.v[476] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_scaled_add(128, 119, 106, 0.5);
        }

        if ((!(s.v[476] != 0.0)) && (s.v[483] != 0.0)) {
            s.store_div_ad_rhs(115, 128, A::offset(s.ad_value(128), 1.0));
        }

        if ((!(s.v[476] != 0.0)) && (!(s.v[483] != 0.0))) {
            s.store_div_ad_rhs(115, 107, A::sub(A::add(s.ad_value(107), s.ad_value(231)), s.ad_value(230)));
        }

        if (!(s.v[476] != 0.0)) {
            s.copy_ad(125, 236);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scale(126, 17, 0.1);
        }

        if (!(s.v[476] != 0.0)) {
            s.copy_ad(127, 108);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(199, 1.0, A::scale(s.ad_value(127), 1.0 / (p.p61)));
        }

        s.store_scale(129, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(273, 14, 0.1);

        s.store_div_ad_lhs(259, A::sub(s.ad_value(232), s.ad_value(129)), 273);

        s.v[484] = if (s.v[232] < s.v[129]) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_sub_ad_rhs(130, 232, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[484] != 0.0)) {
            s.store_sub_ad_rhs(130, 129, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_ad(131, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(232), s.ad_value(130)), 3.0));

        s.v[485] = if (p.p73 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.copy_ad(132, 230);
        }

        s.v[486] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[485] != 0.0)) && (s.v[486] != 0.0)) {
            s.store_add(132, 230, 125);
        }

        if ((!(s.v[485] != 0.0)) && (!(s.v[486] != 0.0))) {
            s.copy_ad(132, 231);
        }

        s.store_div_ad(133, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(134, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(133), ((-1.0) / p.p71))));

        s.store_div_ad_lhs(259, A::sub(s.ad_value(132), s.ad_value(134)), 126);

        s.v[487] = if (s.v[132] < s.v[134]) { 1.0 } else { 0.0 };

        if (s.v[487] != 0.0) {
            s.store_sub_ad_rhs(135, 132, A::mul(s.ad_value(126), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[487] != 0.0)) {
            s.store_sub_ad_rhs(135, 134, A::mul(s.ad_value(126), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_powf(136, 199, p.p75);

        s.store_add_ad(137, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::mul(s.ad_value(136), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(135), s.ad_value(17))), (1.0 - p.p71))))), A::mul(A::mul(s.ad_value(136), s.ad_value(133)), A::sub(s.ad_value(132), s.ad_value(135))));

        s.store_add_ad(138, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(137)), A::mul(s.ad_value(25), s.ad_value(230)));

        s.store_scale(139, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(140, 139, 246);

        s.store_div_ad_rhs(142, 140, A::offset(A::sqrt(A::offset(s.ad_value(140), 1.0)), 1.0));

        s.store_ad(122, &A::pow(s.ad_value(121), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(141, 139, 122);

        s.store_div_ad_rhs(143, 141, A::offset(A::sqrt(A::offset(s.ad_value(141), 1.0)), 1.0));

        s.v[488] = if (p.p91 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[488] != 0.0) {
            s.store_add_ad(144, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scale_ad(269, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), (s.v[99] * s.v[8]));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scale_ad(270, A::div(A::neg(s.ad_value(138)), s.ad_value(40)), (s.v[99] * s.v[8]));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scale_ad(144, A::sub(A::exp(s.ad_value(269)), A::exp(s.ad_value(270))), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 144);

        s.v[489] = if (s.v[144] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_div_from_scalar_ad(145, (0.5 * s.v[261]), A::sub(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144)));
        }

        if (!(s.v[489] != 0.0)) {
            s.store_scale_ad(145, A::add(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144)), 0.5);
        }

        s.store_mul_ad_rhs(146, 145, A::offset(A::scale(A::add(s.ad_value(142), s.ad_value(143)), 0.5), 1.0));

        s.store_mul_ad_lhs(147, A::scale(s.ad_value(35), p.p14), 122);

        s.store_mul(148, 35, 246);

        s.store_div_ad_lhs(149, A::sub(s.ad_value(148), s.ad_value(147)), 146);

        s.store_scale(259, 232, 10000.0);

        s.v[490] = if (s.v[232] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[490] != 0.0) {
            s.store_scale_ad(276, A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 0.0001);
        }

        if (!(s.v[490] != 0.0)) {
            s.store_add_ad_rhs(276, 232, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 0.0001));
        }

        s.store_scale(278, 276, 1.0 / (p.p139));

        s.v[491] = if (s.v[278] < p.p134) { 1.0 } else { 0.0 };

        if (!(s.v[491] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_scaled_offset(259, 232, (-p.p141), 1000.0);

        s.v[493] = if (((s.v[232] * s.v[8]) / p.p16) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[493] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p16))));
        }

        if (!(s.v[493] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[493] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p16))), (-p.p134)), 1.0));
        }

        s.v[494] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[495] = if (((s.v[232] - s.v[55]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_exp_ad(278, A::scale(A::sub(s.ad_value(232), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_mul_ad_rhs(278, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(232), s.ad_value(55)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[496] = if (((s.v[149] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[494] != 0.0) && (!(s.v[496] != 0.0))) {
            s.store_scalar(275, ((40.0) as f64).exp());
        }

        s.v[498] = if (((s.v[233] * s.v[8]) / p.p18) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p18))));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[498] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p18))), (-p.p134)), 1.0));
        }

        s.v[499] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[500] = if (((s.v[233] - s.v[55]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if ((s.v[499] != 0.0) && (s.v[500] != 0.0)) {
            s.store_exp_ad(278, A::scale(A::sub(s.ad_value(233), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[499] != 0.0) && (!(s.v[500] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((s.v[499] != 0.0) && (!(s.v[500] != 0.0))) {
            s.store_mul_ad_rhs(278, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(233), s.ad_value(55)), s.v[8]), (-p.p134)), 1.0));
        }

        s.v[501] = if (((s.v[232] * s.v[8]) / p.p20) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[501] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p20))));
        }

        if (!(s.v[501] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[501] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(232), (s.v[8] * 1.0 / (p.p20))), (-p.p134)), 1.0));
        }

        s.v[502] = if (((s.v[233] * s.v[8]) / p.p22) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[502] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p22))));
        }

        if (!(s.v[502] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[502] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p22))), (-p.p134)), 1.0));
        }

        s.v[503] = if (((s.v[235] * s.v[8]) / p.p31) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[503] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(235), (s.v[8] * 1.0 / (p.p31))));
        }

        if (!(s.v[503] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[503] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(235), (s.v[8] * 1.0 / (p.p31))), (-p.p134)), 1.0));
        }

        s.v[504] = if (((s.v[233] * s.v[8]) / p.p133) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[504] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p133))));
        }

        if (!(s.v[504] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[504] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(233), (s.v[8] * 1.0 / (p.p133))), (-p.p134)), 1.0));
        }

        s.v[505] = if (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[232] < 0.0)) { 1.0 } else { 0.0 };

        s.v[506] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p134) { 1.0 } else { 0.0 };

        if ((s.v[505] != 0.0) && (!(s.v[506] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (s.v[505] != 0.0) {
            s.store_mul(255, 232, 65);
        }

        if (s.v[505] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(255)), 1e-30)), ((-2.0) - p.p66)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p66 * p.p66)), A::scale(s.ad_value(255), (3.0 * (p.p66 - 1.0)))), p.p66), A::mul(A::mul(A::scale(s.ad_value(255), 6.0), s.ad_value(255)), A::offset(s.ad_value(255), (p.p66 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[505] != 0.0) {
            s.store_div_ad(255, A::mul(A::scale(s.ad_value(232), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[507] = if (s.v[255] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[508] = if (s.v[255] < p.p134) { 1.0 } else { 0.0 };

        if (((s.v[505] != 0.0) && (s.v[507] != 0.0)) && (!(s.v[508] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.v[509] = if (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[230] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[509] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(230), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[510] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p134) { 1.0 } else { 0.0 };

        if ((s.v[509] != 0.0) && (!(s.v[510] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[509] != 0.0) {
            s.store_mul(257, 230, 67);
        }

        if (s.v[509] != 0.0) {
            let assign4300_ad_e4164: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(257)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(257), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(257), 6.0), s.ad_value(257)), A::offset(s.ad_value(257), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4300_ad_e4164);
        }

        if (s.v[509] != 0.0) {
            s.store_div_ad(257, A::mul(A::scale(s.ad_value(230), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[511] = if (s.v[257] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[512] = if (s.v[257] < p.p134) { 1.0 } else { 0.0 };

        if (((s.v[509] != 0.0) && (s.v[511] != 0.0)) && (!(s.v[512] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_mul(158, 139, 248);

        s.store_scale(159, 250, 4.0);

        s.store_div_ad(161, A::sub(s.ad_value(158), s.ad_value(139)), A::offset(A::sqrt(A::offset(s.ad_value(158), 1.0)), 1.0));

        s.store_div_ad_rhs(160, 159, A::offset(A::sqrt(A::offset(s.ad_value(159), 1.0)), 1.0));

        s.v[513] = if ((p.p5 > 0.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[513] != 0.0) {
            s.store_div_ad(164, A::scale(A::offset(s.ad_value(249), (-1.0)), ((p.p32 * 2.0) * s.v[43])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(249), ((4.0 * s.v[43]) / s.v[37])), 1.0)), 1.0));
        }

        if (s.v[513] != 0.0) {
            s.store_scalar(165, 0.0);
        }

        s.v[514] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_scalar(271, ((p.p32 * s.v[43]) * s.v[32]));
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_scale_ad(166, A::sub_from_scalar(2.0, A::ln(A::scale(s.ad_value(271), s.v[8]))), s.v[6]);
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_sub(264, 241, 166);
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_scalar(261, (0.11 * 0.11));
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_square(262, 264);
        }

        s.v[515] = if (s.v[264] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[513] != 0.0) && (s.v[514] != 0.0)) && (s.v[515] != 0.0)) {
            s.store_div_ad(167, A::scale(s.ad_value(261), 0.5), A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)));
        }

        if (((s.v[513] != 0.0) && (s.v[514] != 0.0)) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(167, A::add(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)), 0.5);
        }

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.store_div_ad_rhs(168, 167, A::add(A::add(s.ad_value(271), A::scale(A::add(s.ad_value(164), s.ad_value(165)), s.v[32])), s.ad_value(167)));
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(166, 0.0);
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(264, 0.0);
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(167, 0.0);
        }

        if ((s.v[513] != 0.0) && (!(s.v[514] != 0.0))) {
            s.store_scalar(168, 1.0);
        }

        s.v[516] = if (p.p83 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[516] != 0.0) {
            s.store_add(322, 234, 230);
        }

        if (s.v[516] != 0.0) {
            s.store_scalar(261, (1e-6 * 1e-6));
        }

        if (s.v[516] != 0.0) {
            s.store_mul_ad_lhs(262, A::scale(s.ad_value(322), ((-1.0) * (-1.0))), 322);
        }

        s.store_add_ad(172, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 172);

        s.v[519] = if (s.v[172] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[519] != 0.0) {
            s.store_div_from_scalar_ad(173, (0.5 * s.v[261]), A::sub(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172)));
        }

        if (!(s.v[519] != 0.0)) {
            s.store_scale_ad(173, A::add(A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172)), 0.5);
        }

        s.store_mul_ad_rhs(174, 173, A::offset(A::scale(A::add(s.ad_value(142), s.ad_value(143)), 0.5), 1.0));

        s.store_div_from_scalar(176, s.v[29], 174);

        s.v[520] = if (s.v[176] < s.v[316]) { 1.0 } else { 0.0 };

        if (s.v[520] != 0.0) {
            s.copy_ad(176, 316);
        }

        s.store_scale(175, 176, 3.0);

        s.v[521] = if (s.v[149] > 0.0) { 1.0 } else { 0.0 };

        s.v[522] = if (p.p38 == 1.0) { 1.0 } else { 0.0 };

        s.v[523] = if (s.v[230] < p.p43) { 1.0 } else { 0.0 };

        s.v[524] = if (((-s.v[149]) / p.p41) < p.p134) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (s.v[524] != 0.0)) {
            s.store_exp_ad(308, A::scale(A::neg(s.ad_value(149)), 1.0 / (p.p41)));
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[524] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[524] != 0.0))) {
            s.store_mul_ad_rhs(308, 275, A::offset(A::offset(A::scale(A::neg(s.ad_value(149)), 1.0 / (p.p41)), (-p.p134)), 1.0));
        }

        if (((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) {
            s.store_mul_ad_lhs(309, A::sub_from_scalar(p.p43, s.ad_value(230)), 308);
        }

        s.v[525] = if (((-s.v[310]) * ((s.v[309]) as f64).powf(p.p40)) < p.p134) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_exp_ad(313, A::mul(A::neg(s.ad_value(310)), A::powf(s.ad_value(309), p.p40)));
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_mul_ad_rhs(313, 275, A::offset(A::offset(A::mul(A::neg(s.ad_value(310)), A::powf(s.ad_value(309), p.p40)), (-p.p134)), 1.0));
        }

        if (((s.v[521] != 0.0) && (s.v[522] != 0.0)) && (s.v[523] != 0.0)) {
            s.store_mul_ad_lhs(196, A::mul(A::div_from_scalar(p.p39, s.ad_value(310)), s.ad_value(309)), 313);
        }

        s.v[526] = if (p.p38 == 2.0) { 1.0 } else { 0.0 };

        s.v[527] = if (s.v[230] < s.v[16]) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_scalar(185, ((2.0 * p.p45) / (p.p44 * p.p44)));
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad_lhs(260, A::sub(s.ad_value(16), s.ad_value(230)), 199);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_sqrt_ad(186, A::div(A::scale(s.ad_value(260), 2.0), s.ad_value(185)));
        }

        s.v[528] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_scalar(187, p.p44);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[528] != 0.0))) {
            s.store_sub_from_scalar_ad(116, 1.0, A::scale(s.ad_value(115), 0.5));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[528] != 0.0))) {
            s.store_mul_ad_lhs(187, A::scale(s.ad_value(116), p.p44), 116);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad(188, A::mul(s.ad_value(186), s.ad_value(187)), A::sqrt(A::add(A::square(s.ad_value(186)), A::square(s.ad_value(187)))));
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad_lhs(189, A::sub(s.ad_value(16), s.ad_value(230)), 188);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_add_ad_rhs(190, 189, A::mul(A::mul(A::scale(s.ad_value(188), 0.5), s.ad_value(185)), s.ad_value(199)));
        }

        s.v[529] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[529] != 0.0)) {
            s.copy_ad(191, 190);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_offset_ad(192, A::scale(A::offset(A::scale(s.ad_value(115), 2.0), 1.0), (2.0 * p.p46)), 1.0);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_scalar(193, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_sub_ad_rhs(194, 189, A::mul(A::mul(A::scale(s.ad_value(188), 0.5), s.ad_value(185)), A::sub(s.ad_value(193), A::div(s.ad_value(149), A::scale(s.ad_value(192), p.p61)))));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_add_ad(260, A::mul(A::sub(s.ad_value(194), s.ad_value(190)), A::sub(s.ad_value(194), s.ad_value(190))), A::scale(A::mul(A::mul(A::scale(s.ad_value(189), 0.1), s.ad_value(189)), s.ad_value(127)), 1.0 / (p.p61)));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[529] != 0.0))) {
            s.store_scale_ad(191, A::add(A::add(s.ad_value(194), s.ad_value(190)), A::sqrt(s.ad_value(260))), 0.5);
        }

        if ((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_div_ad_lhs(267, A::sub(s.ad_value(191), s.ad_value(189)), 191);
        }

        s.v[530] = if (((s.v[267]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_div_ad_lhs(195, A::scale(s.ad_value(188), 0.5), 267);
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_mul_ad(196, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(191)), s.ad_value(195)), A::sub(A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(191))), A::exp(A::mul(A::div(A::neg(s.ad_value(98)), s.ad_value(191)), A::offset(A::div(s.ad_value(187), s.ad_value(195)), 1.0)))));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) && (!(s.v[530] != 0.0))) {
            s.store_mul_ad(196, A::mul(s.ad_value(0), s.ad_value(187)), A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(191))));
        }

        s.v[531] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        s.v[532] = if (s.v[230] < p.p43) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad(200, A::powf(A::sub_from_scalar(p.p43, s.ad_value(230)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(149), A::offset(s.ad_value(149), p.p47))), p.p48));
        }

        s.v[533] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.copy_ad(201, 200);
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_scaled_offset(202, 149, (-p.p51), 1.0 / (p.p47));
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_scaled_offset(259, 202, (-1.0), 1.0 / (p.p50));
        }

        s.v[534] = if (s.v[202] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) && (s.v[534] != 0.0)) {
            s.store_offset_ad(203, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), p.p50), 1.0);
        }

        if (((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) && (!(s.v[534] != 0.0))) {
            s.store_add_ad_rhs(203, 202, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), p.p50));
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_mul_ad_rhs(201, 200, A::powf(s.ad_value(203), p.p49));
        }

        s.v[535] = if (((-s.v[310]) * s.v[201]) < p.p134) { 1.0 } else { 0.0 };

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_exp_ad(313, A::mul(A::neg(s.ad_value(310)), s.ad_value(201)));
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_mul_ad_rhs(313, 275, A::offset(A::offset(A::mul(A::neg(s.ad_value(310)), s.ad_value(201)), (-p.p134)), 1.0));
        }

        if (((((s.v[521] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[526] != 0.0))) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad_lhs(196, A::mul(A::div_from_scalar(p.p39, s.ad_value(310)), A::sub_from_scalar(p.p43, s.ad_value(230))), 313);
        }

        s.v[536] = if (s.v[196] > 0.0) { 1.0 } else { 0.0 };

        s.v[537] = if (p.p52 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_add_ad(197, A::add(A::div_from_scalar(s.v[6], A::mul(s.ad_value(149), A::add(s.ad_value(30), s.ad_value(175)))), A::scale(A::div(s.ad_value(146), s.ad_value(35)), s.v[42])), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(175))));
        }

        s.v[538] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) {
            s.store_scaled_sub(259, 196, 197, 1000000.0);
        }

        s.v[539] = if (s.v[196] < s.v[197]) { 1.0 } else { 0.0 };

        if (((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) && (s.v[539] != 0.0)) {
            s.store_sub_ad_rhs(196, 196, A::scale(A::ln(A::offset(A::exp(s.ad_value(259)), 1.0)), 1e-6));
        }

        if (((((s.v[521] != 0.0) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_sub_ad_rhs(196, 197, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0)), 1e-6));
        }

        s.store_mul_ad_lhs(204, A::scale(s.ad_value(23), (1.0 - p.p67)), 131);

        s.store_div_ad_lhs(259, A::sub(s.ad_value(233), s.ad_value(129)), 273);

        s.v[541] = if (s.v[233] < s.v[129]) { 1.0 } else { 0.0 };

        if (s.v[541] != 0.0) {
            s.store_sub_ad_rhs(205, 233, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[541] != 0.0)) {
            s.store_sub_ad_rhs(205, 129, A::mul(s.ad_value(273), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_mul_ad(206, A::scale(s.ad_value(23), p.p67), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(205), s.ad_value(65))), (1.0 - p.p66)))), A::scale(A::sub(s.ad_value(233), s.ad_value(205)), 3.0)));

        s.store_mul_ad_lhs(207, A::scale(s.ad_value(24), p.p76), 138);

        s.v[208] = (s.v[94] * s.v[36]);

        s.store_mul_ad_lhs(212, A::scale(s.ad_value(142), (0.5 * s.v[208])), 173);

        s.store_mul_ad_lhs(213, A::scale(s.ad_value(143), (0.5 * s.v[208])), 173);

        s.store_scale(274, 17, 0.1);

        s.store_div_ad_lhs(259, A::sub(s.ad_value(235), s.ad_value(134)), 274);

        s.v[542] = if (s.v[235] < s.v[134]) { 1.0 } else { 0.0 };

        if (s.v[542] != 0.0) {
            s.store_sub_ad_rhs(214, 235, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[542] != 0.0)) {
            s.store_sub_ad_rhs(214, 134, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_add_ad(215, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(214), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(133), A::sub(s.ad_value(235), s.ad_value(214))));

        s.store_scale_ad(216, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(215)), A::mul(s.ad_value(25), s.ad_value(235)))), ((1.0 - p.p76) * (1.0 - p.p32)));

        s.store_div_ad_lhs(259, A::sub(s.ad_value(241), s.ad_value(134)), 274);

        s.v[543] = if (s.v[241] < s.v[134]) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_sub_ad_rhs(217, 241, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(s.ad_value(259)), 1.0))));
        }

        if (!(s.v[543] != 0.0)) {
            s.store_sub_ad_rhs(217, 134, A::mul(s.ad_value(274), A::ln(A::offset(A::exp(A::neg(s.ad_value(259))), 1.0))));
        }

        s.store_add_ad(218, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(133), A::sub(s.ad_value(241), s.ad_value(217))));

        s.store_scale_ad(219, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(218)), A::mul(s.ad_value(25), s.ad_value(241)))), ((1.0 - p.p76) * p.p32));

        s.store_scale_ad(220, A::powf(A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p84)), (s.v[93] * s.v[36]));

        s.v[544] = if ((s.v[232] / (p.p84 * s.v[6])) < p.p134) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_exp_ad(276, A::scale(s.ad_value(232), 1.0 / ((p.p84 * s.v[6]))));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (!(s.v[544] != 0.0)) {
            s.store_mul_ad_rhs(276, 275, A::offset(A::offset(A::scale(s.ad_value(232), 1.0 / ((p.p84 * s.v[6]))), (-p.p134)), 1.0));
        }

        s.store_mul(222, 220, 276);

        s.v[223] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_ad(224, A::scale(s.ad_value(115), (0.5 * s.v[223])), A::offset(A::add(s.ad_value(119), s.ad_value(106)), 2.0));

        s.v[545] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_scale_ad(229, A::add(A::scale(s.ad_value(161), s.v[208]), A::scale(s.ad_value(160), s.v[223])), ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[546] = if ((((s.v[235] - s.v[22]) / p.p90) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if ((!(s.v[545] != 0.0)) && (s.v[546] != 0.0)) {
            s.store_exp_ad(170, A::scale(A::scale(A::sub(s.ad_value(235), s.ad_value(22)), 1.0 / (p.p90)), s.v[8]));
        }

        if ((!(s.v[545] != 0.0)) && (!(s.v[546] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if ((!(s.v[545] != 0.0)) && (!(s.v[546] != 0.0))) {
            s.store_mul_ad_rhs(170, 275, A::offset(A::offset(A::scale(A::scale(A::sub(s.ad_value(235), s.ad_value(22)), 1.0 / (p.p90)), s.v[8]), (-p.p134)), 1.0));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_div_ad(229, A::scale(s.ad_value(248), ((2.0 * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(170), 4.0), 1.0)), 1.0));
        }

        s.v[547] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[547] != 0.0) {
            s.store_scale(229, 229, s.v[150]);
        }

        s.v[548] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_mul(162, 139, 249);
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_div_ad(163, A::sub(s.ad_value(162), s.ad_value(139)), A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0));
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_scale(225, 252, 4.0);
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_div_ad_rhs(226, 225, A::offset(A::sqrt(A::offset(s.ad_value(225), 1.0)), 1.0));
        }

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_scale_ad(227, A::add(A::scale(s.ad_value(163), s.v[208]), A::scale(s.ad_value(226), s.v[223])), (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[549] = if (((s.v[241] - s.v[22]) * s.v[8]) < p.p134) { 1.0 } else { 0.0 };

        if (((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) && (s.v[549] != 0.0)) {
            s.store_exp_ad(171, A::scale(A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]));
        }

        if (((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) && (!(s.v[549] != 0.0))) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if (((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) && (!(s.v[549] != 0.0))) {
            s.store_mul_ad_rhs(171, 275, A::offset(A::offset(A::scale(A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]), (-p.p134)), 1.0));
        }

        if ((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_div_ad(227, A::scale(s.ad_value(249), (((2.0 * p.p32) * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(171), 4.0), 1.0)), 1.0));
        }

        if (s.v[547] != 0.0) {
            s.store_mul(228, 168, 227);
        }

        s.v[550] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[550] != 0.0) {
            s.store_offset_ad(179, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (-p.p66)), (-3.0));
        }

        if (s.v[550] != 0.0) {
            s.store_div_ad_lhs(268, A::sub(s.ad_value(232), s.ad_value(129)), 273);
        }

        s.v[551] = if (s.v[268] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[550] != 0.0) && (s.v[551] != 0.0)) {
            s.store_div_from_scalar_ad(180, 1.0, A::offset(A::exp(s.ad_value(268)), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[550] != 0.0) && (!(s.v[551] != 0.0))) {
            s.store_div_ad(180, A::exp(A::neg(s.ad_value(268))), A::offset(A::exp(A::neg(s.ad_value(268))), 1.0));
        }

        if (s.v[550] != 0.0) {
            s.store_offset_ad(178, A::mul(s.ad_value(179), s.ad_value(180)), 3.0);
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad_lhs(181, A::scale(s.ad_value(23), (1.0 - p.p67)), 178);
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad(184, A::div(A::scale(A::mul(s.ad_value(139), s.ad_value(246)), s.v[8]), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(140), 1.0))));
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad_lhs(182, A::scale(s.ad_value(173), (0.5 * s.v[208])), 184);
        }

        if (s.v[550] != 0.0) {
            s.store_scale(183, 222, 1.0 / ((p.p84 * s.v[6])));
        }

        if (s.v[550] != 0.0) {
            s.store_mul_ad(211, A::scale(s.ad_value(234), 0.2), A::add(A::add(s.ad_value(181), s.ad_value(182)), s.ad_value(183)));
        }

        if (s.v[550] != 0.0) {
            s.store_scale(221, 222, (1.0 - p.p94));
        }

        if (s.v[550] != 0.0) {
            s.store_add_ad_rhs(307, 212, A::scale(s.ad_value(222), p.p94));
        }

        if (s.v[550] != 0.0) {
            s.store_add_ad_lhs(210, A::scale(s.ad_value(307), p.p93), 213);
        }

        if (s.v[550] != 0.0) {
            s.store_scale(209, 307, (1.0 - p.p93));
        }

        if (!(s.v[550] != 0.0)) {
            s.copy_ad(209, 212);
        }

        if (!(s.v[550] != 0.0)) {
            s.copy_ad(210, 213);
        }

        if (!(s.v[550] != 0.0)) {
            s.copy_ad(221, 222);
        }

        s.store_div_ad_lhs(303, A::add(s.ad_value(148), s.ad_value(147)), 146);

        s.v[556] = if (s.v[303] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_div_ad_lhs(305, A::add(s.ad_value(209), s.ad_value(210)), 303);
        }

        if (!(s.v[556] != 0.0)) {
            s.store_mul_ad_lhs(305, A::scale(s.ad_value(173), s.v[94]), 146);
        }

        s.v[557] = if (p.p130 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[557] != 0.0) {
            s.store_scale(306, 305, p.p93);
        }

        s.v[558] = if (p.p130 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[557] != 0.0)) && (s.v[558] != 0.0)) {
            s.store_scale(306, 305, p.p131);
        }

        if ((!(s.v[557] != 0.0)) && (!(s.v[558] != 0.0))) {
            s.store_scalar(306, 0.0);
        }

    }

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
        let eq0_value: f64 = eq0_e152;
        let eq0_node_derivatives: [f64; 11] = [eq0_e152_d_n0, eq0_e152_d_n1, eq0_e152_d_n2, eq0_e152_d_n3, eq0_e152_d_n4, eq0_e152_d_n5, eq0_e152_d_n6, eq0_e152_d_n7, eq0_e152_d_n8, eq0_e152_d_n9, eq0_e152_d_n10];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
        let eq1_value: f64 = eq1_e157;
        let eq1_node_derivatives: [f64; 11] = [eq1_e157_d_n0, eq1_e157_d_n1, eq1_e157_d_n2, eq1_e157_d_n3, eq1_e157_d_n4, eq1_e157_d_n5, eq1_e157_d_n6, eq1_e157_d_n7, eq1_e157_d_n8, eq1_e157_d_n9, eq1_e157_d_n10];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            self.multiplicity * (eq1_value),
            &nodes,
            &eq1_node_derivatives,
            &branches,
            &eq1_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
        let eq2_value: f64 = eq2_e166;
        let eq2_node_derivatives: [f64; 11] = [eq2_e166_d_n0, eq2_e166_d_n1, eq2_e166_d_n2, eq2_e166_d_n3, eq2_e166_d_n4, eq2_e166_d_n5, eq2_e166_d_n6, eq2_e166_d_n7, eq2_e166_d_n8, eq2_e166_d_n9, eq2_e166_d_n10];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            self.multiplicity * (eq2_value),
            &nodes,
            &eq2_node_derivatives,
            &branches,
            &eq2_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
        let eq3_value: f64 = eq3_e183;
        let eq3_node_derivatives: [f64; 11] = [eq3_e183_d_n0, eq3_e183_d_n1, eq3_e183_d_n2, eq3_e183_d_n3, eq3_e183_d_n4, eq3_e183_d_n5, eq3_e183_d_n6, eq3_e183_d_n7, eq3_e183_d_n8, eq3_e183_d_n9, eq3_e183_d_n10];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e192, eq4_e192_d_n0, eq4_e192_d_n1, eq4_e192_d_n2, eq4_e192_d_n3, eq4_e192_d_n4, eq4_e192_d_n5, eq4_e192_d_n6, eq4_e192_d_n7, eq4_e192_d_n8, eq4_e192_d_n9, eq4_e192_d_n10,) = {
    if (s.v[552] != 0.0) {
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
        (eq4_e190, eq4_e190_d_n0, eq4_e190_d_n1, eq4_e190_d_n2, eq4_e190_d_n3, eq4_e190_d_n4, eq4_e190_d_n5, eq4_e190_d_n6, eq4_e190_d_n7, eq4_e190_d_n8, eq4_e190_d_n9, eq4_e190_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e192;
        let eq4_node_derivatives: [f64; 11] = [eq4_e192_d_n0, eq4_e192_d_n1, eq4_e192_d_n2, eq4_e192_d_n3, eq4_e192_d_n4, eq4_e192_d_n5, eq4_e192_d_n6, eq4_e192_d_n7, eq4_e192_d_n8, eq4_e192_d_n9, eq4_e192_d_n10];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e202, eq5_e202_d_n0, eq5_e202_d_n1, eq5_e202_d_n2, eq5_e202_d_n3, eq5_e202_d_n4, eq5_e202_d_n5, eq5_e202_d_n6, eq5_e202_d_n7, eq5_e202_d_n8, eq5_e202_d_n9, eq5_e202_d_n10,) = {
    if (!(s.v[552] != 0.0)) {
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
        (eq5_e200, eq5_e200_d_n0, eq5_e200_d_n1, eq5_e200_d_n2, eq5_e200_d_n3, eq5_e200_d_n4, eq5_e200_d_n5, eq5_e200_d_n6, eq5_e200_d_n7, eq5_e200_d_n8, eq5_e200_d_n9, eq5_e200_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e202;
        let eq5_node_derivatives: [f64; 11] = [eq5_e202_d_n0, eq5_e202_d_n1, eq5_e202_d_n2, eq5_e202_d_n3, eq5_e202_d_n4, eq5_e202_d_n5, eq5_e202_d_n6, eq5_e202_d_n7, eq5_e202_d_n8, eq5_e202_d_n9, eq5_e202_d_n10];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
        let eq6_value: f64 = eq6_e207;
        let eq6_node_derivatives: [f64; 11] = [eq6_e207_d_n0, eq6_e207_d_n1, eq6_e207_d_n2, eq6_e207_d_n3, eq6_e207_d_n4, eq6_e207_d_n5, eq6_e207_d_n6, eq6_e207_d_n7, eq6_e207_d_n8, eq6_e207_d_n9, eq6_e207_d_n10];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
        let eq7_value: f64 = eq7_e215;
        let eq7_node_derivatives: [f64; 11] = [eq7_e215_d_n0, eq7_e215_d_n1, eq7_e215_d_n2, eq7_e215_d_n3, eq7_e215_d_n4, eq7_e215_d_n5, eq7_e215_d_n6, eq7_e215_d_n7, eq7_e215_d_n8, eq7_e215_d_n9, eq7_e215_d_n10];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq7_value),
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
            self.multiplicity,
        );
    }
}
