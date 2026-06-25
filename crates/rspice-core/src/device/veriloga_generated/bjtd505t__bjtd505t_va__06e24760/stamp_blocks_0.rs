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
        s.v[447] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[447] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[447] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[447] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[447] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[153] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[320] = 0.0;

        s.v[448] = if (p.p141 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[448] != 0.0) {
            s.store_scalar(321, 1e-12);
        }

        if (!(s.v[448] != 0.0)) {
            s.store_scalar(321, p.p141);
        }

        s.store_scale(322, 321, p.p1);

        s.store_div_from_scalar(323, 1.0, 322);

        s.v[52] = 0.001;

        s.v[318] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[63] = (1.0 / s.v[62]);

        s.v[265] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.v[449] = if ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[449] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[449] != 0.0)) {
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

        s.v[450] = if ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[450] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[450] != 0.0)) {
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

        s.store_ad(207, &A::voltage(ctx, &nodes, Some(3), None));

        s.v[451] = if (s.v[207] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[451] != 0.0) {
            s.store_neg_ad(207, A::ln(A::sub_from_scalar(1.0, s.ad_value(207))));
        }

        s.v[452] = if (s.v[207] < p.p124) { 1.0 } else { 0.0 };

        if (s.v[452] != 0.0) {
            s.copy_ad(11, 207);
        }

        if (!(s.v[452] != 0.0)) {
            s.store_offset_ad(11, A::ln(A::offset(A::offset(s.ad_value(207), (-p.p124)), 1.0)), p.p124);
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

        s.store_scale_ad(265, A::offset(A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p114), s.ad_value(2)), A::offset(s.ad_value(2), p.p115))), (-0.05)), 10.0);

        s.v[453] = if ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[453] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[453] != 0.0)) {
            s.store_add_ad(70, A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p114), s.ad_value(2)), A::offset(s.ad_value(2), p.p115))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 0.1));
        }

        s.store_scale_ad(265, A::offset(A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p117), s.ad_value(2)), A::offset(s.ad_value(2), p.p118))), (-0.05)), 10.0);

        s.v[454] = if ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[454] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[454] != 0.0)) {
            s.store_add_ad(85, A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p117), s.ad_value(2)), A::offset(s.ad_value(2), p.p118))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 0.1));
        }

        s.store_add_ad(13, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p65)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p104));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.v[455] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[455] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[455] != 0.0)) {
            s.store_offset_ad(14, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(15, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p63)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.v[456] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[456] != 0.0) {
            s.store_add_ad_rhs(16, 15, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[456] != 0.0)) {
            s.store_offset_ad(16, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(21, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p79)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.v[457] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[457] != 0.0) {
            s.store_add_ad_rhs(22, 21, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[457] != 0.0)) {
            s.store_offset_ad(22, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(18, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p70)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.v[458] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[458] != 0.0) {
            s.store_add_ad_rhs(17, 18, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[458] != 0.0)) {
            s.store_offset_ad(17, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(20, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), s.v[75])), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.v[459] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[459] != 0.0) {
            s.store_add_ad_rhs(19, 20, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[459] != 0.0)) {
            s.store_offset_ad(19, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(56, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p26)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p108));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.v[460] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[460] != 0.0) {
            s.store_add_ad_rhs(55, 56, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[460] != 0.0)) {
            s.store_offset_ad(55, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
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

        s.store_scale_ad(28, A::exp(A::scale(s.ad_value(260), p.p96)), p.p53);

        s.v[461] = if (s.v[28] < s.v[322]) { 1.0 } else { 0.0 };

        if (s.v[461] != 0.0) {
            s.copy_ad(28, 322);
        }

        s.store_scale_ad(29, A::exp(A::scale(s.ad_value(260), (p.p97 - p.p95))), p.p55);

        s.store_scale_ad(30, A::exp(A::scale(s.ad_value(260), p.p100)), p.p54);

        s.v[462] = if (s.v[30] < s.v[322]) { 1.0 } else { 0.0 };

        if (s.v[462] != 0.0) {
            s.copy_ad(30, 322);
        }

        s.store_scale_ad(32, A::exp(A::scale(s.ad_value(260), p.p101)), p.p56);

        s.store_scale_ad(33, A::exp(A::scale(s.ad_value(260), p.p103)), p.p57);

        s.store_scale_ad(34, A::exp(A::scale(s.ad_value(260), p.p103)), p.p58);

        s.store_scale_ad(31, A::exp(A::scale(s.ad_value(260), p.p98)), p.p59);

        s.v[463] = if (p.p121 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[463] != 0.0) {
            s.store_scale_ad(50, A::offset(A::scale(s.ad_value(12), p.p121), 1.0), p.p9);
        }

        if (s.v[463] != 0.0) {
            s.store_scaled_offset(265, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[464] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[463] != 0.0) && (!(s.v[464] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), s.v[52]));
        }

        if (s.v[463] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[463] != 0.0)) {
            s.store_scalar(48, p.p9);
        }

        s.v[465] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[465] != 0.0) {
            s.store_scale_ad(51, A::offset(A::scale(s.ad_value(12), p.p122), 1.0), p.p10);
        }

        if (s.v[465] != 0.0) {
            s.store_scaled_offset(265, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[466] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[465] != 0.0) && (s.v[466] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[465] != 0.0) && (!(s.v[466] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), s.v[52]));
        }

        if (s.v[465] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[465] != 0.0)) {
            s.store_scalar(49, p.p10);
        }

        s.store_scale_ad(317, A::offset(A::scale(s.ad_value(12), p.p123), 1.0), p.p42);

        s.v[267] = (s.v[318] * s.v[318]);

        s.store_square(268, 317);

        s.v[467] = if (s.v[317] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[467] != 0.0) {
            s.store_div_from_scalar_ad(316, (0.5 * s.v[267]), A::sub(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(317)));
        }

        if (!(s.v[467] != 0.0)) {
            s.store_scale_ad(316, A::add(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(317)), 0.5);
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div(A::scale(s.ad_value(260), (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), p.p8), A::exp(A::div(A::scale(s.ad_value(10), (-p.p104)), s.ad_value(48))));

        s.store_scale_ad(36, A::exp(A::scale(s.ad_value(260), (1.0 - p.p97))), p.p11);

        s.store_scale_ad(37, A::exp(A::scale(s.ad_value(260), (1.0 - p.p102))), p.p29);

        s.store_mul_ad(38, A::scale(A::exp(A::scale(s.ad_value(260), (6.0 - (2.0 * p.p20)))), p.p19), A::exp(A::scale(s.ad_value(10), ((-p.p112) * 1.0 / (p.p20)))));

        s.store_mul_ad(39, A::scale(A::exp(A::scale(s.ad_value(260), (6.0 - (2.0 * p.p31)))), p.p30), A::exp(A::scale(s.ad_value(10), ((-p.p109) * 1.0 / (p.p31)))));

        s.store_mul_ad(42, A::scale(A::exp(A::scale(s.ad_value(260), (((4.0 - p.p96) + p.p120) * 1.0 / (p.p16)))), p.p15), A::exp(A::scale(s.ad_value(10), ((-p.p110) * 1.0 / (p.p16)))));

        s.store_mul_ad(44, A::scale(A::exp(A::scale(s.ad_value(260), (((4.0 - p.p96) + p.p120) * 1.0 / (p.p18)))), p.p17), A::exp(A::scale(s.ad_value(10), ((-p.p110) * 1.0 / (p.p18)))));

        s.v[468] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[468] != 0.0) {
            s.store_scale_ad(53, A::exp(A::scale(s.ad_value(10), ((-p.p106) * 1.0 / (p.p16)))), p.p24);
        }

        if (s.v[468] != 0.0) {
            s.store_scale_ad(54, A::exp(A::scale(s.ad_value(10), (-p.p105))), p.p27);
        }

        if (s.v[468] != 0.0) {
            s.store_scale_ad(45, A::exp(A::scale(s.ad_value(10), ((-p.p107) * 1.0 / (p.p18)))), p.p25);
        }

        s.store_mul_ad(43, A::scale(A::exp(A::scale(s.ad_value(260), ((4.0 - p.p102) + p.p120))), p.p28), A::exp(A::scale(s.ad_value(10), (-p.p111))));

        s.store_mul_ad(46, A::scale(A::exp(A::scale(s.ad_value(260), (6.0 - (2.0 * p.p22)))), p.p21), A::exp(A::scale(s.ad_value(10), ((-p.p112) * 1.0 / (p.p22)))));

        s.store_mul_ad(47, A::scale(A::exp(A::scale(s.ad_value(260), (4.0 / p.p137))), p.p136), A::exp(A::scale(s.ad_value(10), ((-p.p112) * 1.0 / (p.p137)))));

        s.store_mul_ad(332, A::scale(A::sqrt(s.ad_value(4)), p.p142), A::exp(A::scale(s.ad_value(12), p.p144)));

        s.store_powf_ad(261, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(262, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p34), s.ad_value(70)), s.ad_value(261)), s.ad_value(262)), p.p65), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_mul_ad(58, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(261), p.p33), s.ad_value(14)), s.ad_value(14)), (s.v[64] * s.v[64])), s.ad_value(73)), A::exp(A::sub_from_scalar(p.p34, s.ad_value(61))));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(263, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(264, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p36), s.ad_value(85)), s.ad_value(263)), s.ad_value(264)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.store_mul_ad(84, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(263), p.p35), s.ad_value(19)), s.ad_value(19)), (s.v[66] * s.v[66])), s.ad_value(90)), A::exp(A::sub_from_scalar(p.p36, s.ad_value(83))));

        s.store_exp_ad(261, A::scale(s.ad_value(260), p.p95));

        s.store_mul_ad_lhs(40, A::scale(s.ad_value(261), p.p13), 27);

        s.store_mul_ad_lhs(41, A::scale(s.ad_value(261), p.p12), 262);

        s.store_mul_ad(94, A::scale(A::exp(A::scale(s.ad_value(260), (p.p97 - 2.0))), p.p85), A::exp(A::scale(s.ad_value(10), (-p.p119))));

        s.store_scale_ad(95, A::exp(A::scale(s.ad_value(260), ((p.p95 + p.p97) - 1.0))), p.p86);

        s.store_scale_ad(96, A::exp(A::scale(s.ad_value(260), (p.p98 - 1.0))), p.p87);

        s.store_scaled_add(97, 95, 96, (p.p88 * 1.0 / ((p.p86 + p.p87))));

        s.store_scale_ad(98, A::exp(A::scale(s.ad_value(260), (p.p99 - 1.0))), p.p89);

        s.store_offset(101, 2, (-300.0));

        s.v[469] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[469] != 0.0) {
            s.store_mul_ad_rhs(99, 1, A::sub(A::offset(A::scale(s.ad_value(101), 0.00072), 1.0), A::mul(A::scale(s.ad_value(101), 1.6e-6), s.ad_value(101))));
        }

        if (!(s.v[469] != 0.0)) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scale_ad(100, A::exp(A::scale(s.ad_value(260), p.p95)), p.p91);

        s.v[103] = (p.p133 * (((s.v[5] / s.v[3])) as f64).powf(p.p135));

        s.v[470] = if (p.p56 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[470] != 0.0) {
            s.store_div_from_scalar(104, 1.0, 32);
        }

        s.v[471] = if (s.v[104] > s.v[323]) { 1.0 } else { 0.0 };

        if ((s.v[470] != 0.0) && (s.v[471] != 0.0)) {
            s.copy_ad(104, 323);
        }

        if (!(s.v[470] != 0.0)) {
            s.store_scalar(104, 0.0);
        }

        s.v[472] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[472] != 0.0) {
            s.store_div_from_scalar(105, 1.0, 33);
        }

        s.v[473] = if (s.v[105] > s.v[323]) { 1.0 } else { 0.0 };

        if ((s.v[472] != 0.0) && (s.v[473] != 0.0)) {
            s.copy_ad(105, 323);
        }

        if (!(s.v[472] != 0.0)) {
            s.store_scalar(105, 0.0);
        }

        s.v[474] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[474] != 0.0) {
            s.store_div_from_scalar(106, 1.0, 34);
        }

        s.v[475] = if (s.v[106] > s.v[323]) { 1.0 } else { 0.0 };

        if ((s.v[474] != 0.0) && (s.v[475] != 0.0)) {
            s.copy_ad(106, 323);
        }

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
        if (!(s.v[474] != 0.0)) {
            s.store_scalar(106, 0.0);
        }

        s.store_ad(236, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(237, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(8)), p.p3));

        s.store_ad(238, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(4)), p.p3));

        s.store_ad(239, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), p.p3));

        s.store_ad(240, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), p.p3));

        s.store_ad(242, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(8)), p.p3));

        s.store_ad(245, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(4)), p.p3));

        s.store_ad(246, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p3));

        s.store_ad(249, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(250, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(244, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(7)), p.p3));

        s.store_ad(243, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(10)), p.p3));

        s.store_sub_ad_lhs(241, A::sub(A::add(s.ad_value(240), s.ad_value(237)), s.ad_value(242)), 244);

        s.store_sub_ad_lhs(248, A::add(A::sub(s.ad_value(246), s.ad_value(250)), s.ad_value(241)), 243);

        s.store_add(247, 250, 248);

        s.v[476] = if ((s.v[237] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[476] != 0.0) {
            s.store_exp_ad(251, A::mul(s.ad_value(237), s.ad_value(8)));
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[476] != 0.0)) {
            s.store_mul_ad_rhs(251, 281, A::offset(A::offset(A::mul(s.ad_value(237), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[477] = if (((s.v[238] * s.v[8]) / s.v[48]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[477] != 0.0) {
            s.store_exp_ad(252, A::div(A::mul(s.ad_value(238), s.ad_value(8)), s.ad_value(48)));
        }

        if (!(s.v[477] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[477] != 0.0)) {
            s.store_mul_ad_rhs(252, 281, A::offset(A::offset(A::div(A::mul(s.ad_value(238), s.ad_value(8)), s.ad_value(48)), (-p.p138)), 1.0));
        }

        s.v[478] = if ((s.v[241] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[478] != 0.0) {
            s.store_exp_ad(254, A::mul(s.ad_value(241), s.ad_value(8)));
        }

        if (!(s.v[478] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[478] != 0.0)) {
            s.store_mul_ad_rhs(254, 281, A::offset(A::offset(A::mul(s.ad_value(241), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[479] = if ((s.v[240] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[479] != 0.0) {
            s.store_exp_ad(253, A::mul(s.ad_value(240), s.ad_value(8)));
        }

        if (!(s.v[479] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[479] != 0.0)) {
            s.store_mul_ad_rhs(253, 281, A::offset(A::offset(A::mul(s.ad_value(240), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[480] = if ((s.v[247] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[480] != 0.0) {
            s.store_exp_ad(255, A::mul(s.ad_value(247), s.ad_value(8)));
        }

        if (!(s.v[480] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[480] != 0.0)) {
            s.store_mul_ad_rhs(255, 281, A::offset(A::offset(A::mul(s.ad_value(247), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[481] = if (((s.v[247] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[481] != 0.0) {
            s.store_exp_ad(258, A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[481] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[481] != 0.0)) {
            s.store_mul_ad_rhs(258, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[482] = if (((s.v[241] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[482] != 0.0) {
            s.store_exp_ad(256, A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[482] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[482] != 0.0)) {
            s.store_mul_ad_rhs(256, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[483] = if (((s.v[237] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[483] != 0.0) {
            s.store_exp_ad(257, A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[483] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[483] != 0.0)) {
            s.store_mul_ad_rhs(257, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[484] = if (((s.v[236] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_exp_ad(259, A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[484] != 0.0)) {
            s.store_mul_ad_rhs(259, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.store_sqrt_ad(107, A::offset(A::scale(s.ad_value(257), 4.0), 1.0));

        s.store_sqrt_ad(108, A::offset(A::scale(s.ad_value(259), 4.0), 1.0));

        s.store_div_ad(109, A::scale(s.ad_value(259), 2.0), A::offset(s.ad_value(108), 1.0));

        s.v[485] = if (s.v[109] < p.p140) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.store_scalar(109, p.p140);
        }

        s.store_mul_ad_rhs(110, 6, A::sub(A::sub(s.ad_value(107), s.ad_value(108)), A::ln(A::div(A::offset(s.ad_value(107), 1.0), A::offset(s.ad_value(108), 1.0)))));

        s.store_div_ad_lhs(111, A::add(s.ad_value(110), s.ad_value(242)), 31);

        s.v[486] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        s.v[487] = if (s.v[236] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[487] != 0.0)) {
            s.copy_ad(283, 236);
        }

        if ((s.v[486] != 0.0) && (!(s.v[487] != 0.0))) {
            s.store_offset_ad(283, A::ln(A::offset(A::offset(s.ad_value(236), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[486] != 0.0) {
            s.store_sub_ad_lhs(112, A::add(s.ad_value(16), A::mul(A::scale(s.ad_value(6), 2.0), A::ln(A::offset(A::mul(A::mul(A::scale(s.ad_value(111), 0.5), s.ad_value(31)), s.ad_value(8)), 1.0)))), 283);
        }

        if (s.v[486] != 0.0) {
            s.store_scale(278, 16, 0.2);
        }

        if (s.v[486] != 0.0) {
            s.store_square(267, 278);
        }

        if (s.v[486] != 0.0) {
            s.store_square(268, 112);
        }

        s.v[488] = if (s.v[112] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[488] != 0.0)) {
            s.store_div_ad(113, A::scale(s.ad_value(267), 0.5), A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(112)));
        }

        if ((s.v[486] != 0.0) && (!(s.v[488] != 0.0))) {
            s.store_scale_ad(113, A::add(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(112)), 0.5);
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(114, A::mul(s.ad_value(113), A::offset(s.ad_value(113), (p.p61 * p.p60))), A::scale(A::add(s.ad_value(113), A::scale(s.ad_value(31), p.p61)), p.p60));
        }

        if (s.v[486] != 0.0) {
            s.store_div(271, 111, 114);
        }

        if (s.v[486] != 0.0) {
            s.store_scaled_offset(265, 271, (-1.0), 1.0 / (p.p62));
        }

        s.v[489] = if (s.v[271] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[489] != 0.0)) {
            s.store_offset_ad(269, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), p.p62), 1.0);
        }

        if ((s.v[486] != 0.0) && (!(s.v[489] != 0.0))) {
            s.store_add_ad_rhs(269, 271, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), p.p62));
        }

        if (s.v[486] != 0.0) {
            s.store_scale(115, 269, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[486] != 0.0) {
            s.store_scale(116, 113, 1.0 / ((p.p61 * p.p60)));
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(117, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(115), 4.0), s.ad_value(116)), A::offset(s.ad_value(116), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(115), 2.0), A::offset(s.ad_value(116), 1.0)));
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(118, A::add(A::sub_from_scalar(1.0, s.ad_value(117)), A::mul(s.ad_value(109), s.ad_value(117))), A::offset(A::mul(s.ad_value(109), s.ad_value(117)), 1.0));
        }

        if (s.v[486] != 0.0) {
            s.store_mul_ad_lhs(120, A::mul(A::mul(A::scale(s.ad_value(111), 0.5), s.ad_value(31)), s.ad_value(118)), 8);
        }

        if (s.v[486] != 0.0) {
            s.store_add_ad(272, A::scale(s.ad_value(120), 2.0), A::mul(s.ad_value(109), A::offset(A::add(s.ad_value(109), s.ad_value(120)), 1.0)));
        }

        if (s.v[486] != 0.0) {
            s.store_scaled_offset(121, 120, (-1.0), 0.5);
        }

        if (s.v[486] != 0.0) {
            s.store_add_ad_lhs(266, A::square(s.ad_value(121)), 272);
        }

        s.v[490] = if (s.v[120] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[490] != 0.0)) {
            s.store_add_ad_rhs(122, 121, A::sqrt(s.ad_value(266)));
        }

        if ((s.v[486] != 0.0) && (!(s.v[490] != 0.0))) {
            s.store_div_ad_rhs(122, 272, A::sub(A::sqrt(s.ad_value(266)), s.ad_value(121)));
        }

        s.v[491] = if (s.v[122] < p.p139) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[491] != 0.0)) {
            s.store_scalar(122, p.p139);
        }

        if (s.v[486] != 0.0) {
            s.store_mul_ad(124, A::mul(s.ad_value(122), A::offset(s.ad_value(122), 1.0)), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
        }

        if (s.v[486] != 0.0) {
            s.store_scaled_offset(126, 111, (-p.p61), (0.5 * p.p60));
        }

        if (s.v[486] != 0.0) {
            s.store_mul_ad_lhs(127, A::scale(s.ad_value(31), (p.p60 * p.p61)), 111);
        }

        if (s.v[486] != 0.0) {
            s.store_add_ad_rhs(128, 126, A::sqrt(A::add(A::square(s.ad_value(126)), s.ad_value(127))));
        }

        s.v[492] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[492] != 0.0)) {
            s.store_scale(129, 17, 0.1);
        }

        if ((s.v[486] != 0.0) && (!(s.v[492] != 0.0))) {
            s.store_mul_ad_rhs(129, 17, A::offset(A::div(A::scale(s.ad_value(111), 2.0), A::add(s.ad_value(111), s.ad_value(114))), 0.1));
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(130, A::scale(s.ad_value(111), p.p61), A::offset(s.ad_value(111), p.p61));
        }

        if (s.v[486] != 0.0) {
            s.store_div_from_scalar_ad(202, p.p61, A::offset(s.ad_value(111), p.p61));
        }

        if (!(s.v[486] != 0.0)) {
            s.store_scalar(114, 0.0);
        }

        if (!(s.v[486] != 0.0)) {
            s.store_div_ad(122, A::scale(s.ad_value(257), 2.0), A::offset(s.ad_value(107), 1.0));
        }

        if (!(s.v[486] != 0.0)) {
            s.copy_ad(124, 251);
        }

        s.v[493] = if ((((s.v[242]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[110]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[107] + s.v[108])))) { 1.0 } else { 0.0 };

        if ((!(s.v[486] != 0.0)) && (s.v[493] != 0.0)) {
            s.store_scaled_add(131, 122, 109, 0.5);
        }

        if ((!(s.v[486] != 0.0)) && (s.v[493] != 0.0)) {
            s.store_div_ad_rhs(118, 131, A::offset(s.ad_value(131), 1.0));
        }

        if ((!(s.v[486] != 0.0)) && (!(s.v[493] != 0.0))) {
            s.store_div_ad_rhs(118, 110, A::sub(A::add(s.ad_value(110), s.ad_value(237)), s.ad_value(236)));
        }

        if (!(s.v[486] != 0.0)) {
            s.copy_ad(128, 242);
        }

        if (!(s.v[486] != 0.0)) {
            s.store_scale(129, 17, 0.1);
        }

        if (!(s.v[486] != 0.0)) {
            s.copy_ad(130, 111);
        }

        if (!(s.v[486] != 0.0)) {
            s.store_sub_from_scalar_ad(202, 1.0, A::scale(s.ad_value(130), 1.0 / (p.p61)));
        }

        s.store_scale(132, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(279, 14, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(238), s.ad_value(132)), 279);

        s.v[494] = if (s.v[238] < s.v[132]) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_sub_ad_rhs(133, 238, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[494] != 0.0)) {
            s.store_sub_ad_rhs(133, 132, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_ad(134, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(238), s.ad_value(133)), 3.0));

        s.v[495] = if (p.p73 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[495] != 0.0) {
            s.copy_ad(135, 236);
        }

        s.v[496] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[495] != 0.0)) && (s.v[496] != 0.0)) {
            s.store_add(135, 236, 128);
        }

        if ((!(s.v[495] != 0.0)) && (!(s.v[496] != 0.0))) {
            s.copy_ad(135, 237);
        }

        s.store_div_ad(136, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(137, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(136), ((-1.0) / p.p71))));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(135), s.ad_value(137)), 129);

        s.v[497] = if (s.v[135] < s.v[137]) { 1.0 } else { 0.0 };

        if (s.v[497] != 0.0) {
            s.store_sub_ad_rhs(138, 135, A::mul(s.ad_value(129), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[497] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(129), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_powf(139, 202, p.p75);

        s.store_add_ad(140, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::mul(s.ad_value(139), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(17))), (1.0 - p.p71))))), A::mul(A::mul(s.ad_value(139), s.ad_value(136)), A::sub(s.ad_value(135), s.ad_value(138))));

        s.store_add_ad(141, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(140)), A::mul(s.ad_value(25), s.ad_value(236)));

        s.store_div_ad_lhs(142, A::scale(s.ad_value(35), 4.0), 36);

        s.store_mul(143, 142, 252);

        s.store_div_ad_rhs(145, 143, A::offset(A::sqrt(A::offset(s.ad_value(143), 1.0)), 1.0));

        s.store_ad(125, &A::pow(s.ad_value(124), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(144, 142, 125);

        s.store_div_ad_rhs(146, 144, A::offset(A::sqrt(A::offset(s.ad_value(144), 1.0)), 1.0));

        s.v[498] = if (p.p91 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_add_ad(147, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(275, A::mul(A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), s.ad_value(100)), 8);
        }

        if (!(s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(276, A::mul(A::div(A::neg(s.ad_value(141)), s.ad_value(40)), s.ad_value(100)), 8);
        }

        if (!(s.v[498] != 0.0)) {
            s.store_div_ad(147, A::sub(A::exp(s.ad_value(275)), A::exp(s.ad_value(276))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 147);

        s.v[499] = if (s.v[147] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[499] != 0.0) {
            s.store_div_from_scalar_ad(148, (0.5 * s.v[267]), A::sub(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(147)));
        }

        if (!(s.v[499] != 0.0)) {
            s.store_scale_ad(148, A::add(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(147)), 0.5);
        }

        s.store_mul_ad_rhs(149, 148, A::offset(A::scale(A::add(s.ad_value(145), s.ad_value(146)), 0.5), 1.0));

        s.store_mul_ad_lhs(150, A::scale(s.ad_value(35), p.p14), 125);

        s.store_mul(151, 35, 252);

        s.store_div_ad_lhs(152, A::sub(s.ad_value(151), s.ad_value(150)), 149);

        s.store_scale(265, 238, 10000.0);

        s.v[500] = if (s.v[238] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[500] != 0.0) {
            s.store_scale_ad(282, A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 0.0001);
        }

        if (!(s.v[500] != 0.0)) {
            s.store_add_ad_rhs(282, 238, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 0.0001));
        }

        s.store_scale(284, 282, 1.0 / (p.p143));

        s.v[501] = if (s.v[284] < p.p138) { 1.0 } else { 0.0 };

        if (s.v[501] != 0.0) {
            s.store_exp(285, 284);
        }

        if (!(s.v[501] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[501] != 0.0)) {
            s.store_mul_ad_rhs(285, 281, A::offset(A::offset(s.ad_value(284), (-p.p138)), 1.0));
        }

        s.store_mul_ad_rhs(333, 332, A::offset(s.ad_value(285), (-1.0)));

        s.store_scaled_offset(265, 238, (-p.p145), 1000.0);

        s.v[502] = if (s.v[238] < p.p145) { 1.0 } else { 0.0 };

        if (s.v[502] != 0.0) {
            s.store_sub_ad_rhs(286, 238, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 0.001));
        }

        if (!(s.v[502] != 0.0)) {
            s.store_sub_from_scalar_ad(286, p.p145, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 0.001));
        }

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
        s.store_mul_ad(334, A::scale(s.ad_value(286), p.p146), A::powf(A::sub_from_scalar(p.p145, s.ad_value(286)), 2.0));

        s.v[503] = if (((s.v[238] * s.v[8]) / p.p16) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[503] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p16)));
        }

        if (!(s.v[503] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[503] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p16)), (-p.p138)), 1.0));
        }

        s.v[504] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[505] = if (((s.v[238] - s.v[55]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[504] != 0.0) && (s.v[505] != 0.0)) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[504] != 0.0) && (!(s.v[505] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((s.v[504] != 0.0) && (!(s.v[505] != 0.0))) {
            s.store_mul_ad_rhs(284, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[506] = if (((s.v[152] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[504] != 0.0) && (s.v[506] != 0.0)) {
            s.store_exp_ad(285, A::offset(A::div(s.ad_value(152), s.ad_value(35)), (-1000.0)));
        }

        if ((s.v[504] != 0.0) && (!(s.v[506] != 0.0))) {
            s.store_scalar(281, ((40.0) as f64).exp());
        }

        if ((s.v[504] != 0.0) && (!(s.v[506] != 0.0))) {
            s.store_mul_ad_rhs(285, 281, A::offset(A::offset(A::offset(A::div(s.ad_value(152), s.ad_value(35)), (-1000.0)), (-40.0)), 1.0));
        }

        if (s.v[504] != 0.0) {
            let assign3760_ad_e3523: A = A::add(A::add(A::mul(s.ad_value(42), A::offset(s.ad_value(282), (-1.0))), A::mul(A::div(A::mul(A::scale(s.ad_value(53), 2.0), A::offset(s.ad_value(282), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(284), 4.0), 1.0)), 1.0)), A::offset(A::div(s.ad_value(141), s.ad_value(40)), 1.0))), A::div(A::mul(A::mul(s.ad_value(54), A::offset(s.ad_value(124), (-1.0))), s.ad_value(285)), A::offset(s.ad_value(285), 1.0)));
            s.store_ad(154, &assign3760_ad_e3523);
        }

        s.v[507] = if (p.p92 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[504] != 0.0)) && (s.v[507] != 0.0)) {
            s.store_mul_ad_rhs(154, 42, A::offset(s.ad_value(282), (-1.0)));
        }

        if ((!(s.v[504] != 0.0)) && (!(s.v[507] != 0.0))) {
            s.store_mul_ad_rhs(154, 42, A::add(A::scale(A::offset(s.ad_value(282), (-1.0)), (1.0 - p.p92)), A::mul(A::scale(A::offset(A::add(s.ad_value(282), s.ad_value(124)), (-2.0)), p.p92), A::offset(A::div(s.ad_value(141), s.ad_value(40)), 1.0))));
        }

        s.v[508] = if (((s.v[239] * s.v[8]) / p.p18) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[508] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p18)));
        }

        if (!(s.v[508] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[508] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p18)), (-p.p138)), 1.0));
        }

        s.v[509] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[510] = if (((s.v[239] - s.v[55]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[509] != 0.0) && (s.v[510] != 0.0)) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[509] != 0.0) && (!(s.v[510] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((s.v[509] != 0.0) && (!(s.v[510] != 0.0))) {
            s.store_mul_ad_rhs(284, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        if (s.v[509] != 0.0) {
            s.store_add_ad(155, A::mul(s.ad_value(44), A::offset(s.ad_value(282), (-1.0))), A::div(A::mul(A::scale(s.ad_value(45), 2.0), A::offset(s.ad_value(282), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(284), 4.0), 1.0)), 1.0)));
        }

        if (!(s.v[509] != 0.0)) {
            s.store_mul_ad_rhs(155, 44, A::offset(s.ad_value(282), (-1.0)));
        }

        s.v[511] = if (((s.v[238] * s.v[8]) / p.p20) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[511] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p20)));
        }

        if (!(s.v[511] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[511] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p20)), (-p.p138)), 1.0));
        }

        s.store_mul_ad_rhs(156, 38, A::offset(s.ad_value(282), (-1.0)));

        s.v[512] = if (((s.v[239] * s.v[8]) / p.p22) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[512] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p22)));
        }

        if (!(s.v[512] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[512] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p22)), (-p.p138)), 1.0));
        }

        s.store_mul_ad_rhs(158, 46, A::offset(s.ad_value(282), (-1.0)));

        s.v[513] = if (((s.v[241] * s.v[8]) / p.p31) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[513] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(241), s.ad_value(8)), 1.0 / (p.p31)));
        }

        if (!(s.v[513] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[513] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(241), s.ad_value(8)), 1.0 / (p.p31)), (-p.p138)), 1.0));
        }

        s.store_mul_ad_rhs(157, 39, A::offset(s.ad_value(282), (-1.0)));

        s.v[514] = if (((s.v[239] * s.v[8]) / p.p137) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[514] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p137)));
        }

        if (!(s.v[514] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[514] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p137)), (-p.p138)), 1.0));
        }

        s.store_mul_ad_rhs(159, 47, A::offset(s.ad_value(282), (-1.0)));

        s.v[515] = if (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[238] < 0.0)) { 1.0 } else { 0.0 };

        s.v[516] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[515] != 0.0) && (s.v[516] != 0.0)) {
            s.store_exp_ad(68, A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))));
        }

        if ((s.v[515] != 0.0) && (!(s.v[516] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((s.v[515] != 0.0) && (!(s.v[516] != 0.0))) {
            s.store_mul_ad_rhs(68, 281, A::offset(A::offset(A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))), (-p.p138)), 1.0));
        }

        if (s.v[515] != 0.0) {
            s.store_mul(261, 238, 65);
        }

        if (s.v[515] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(261)), 1e-30)), ((-2.0) - p.p66)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p66 * p.p66)), A::scale(s.ad_value(261), (3.0 * (p.p66 - 1.0)))), p.p66), A::mul(A::mul(A::scale(s.ad_value(261), 6.0), s.ad_value(261)), A::offset(s.ad_value(261), (p.p66 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[515] != 0.0) {
            s.store_div_ad(261, A::mul(A::scale(s.ad_value(238), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[517] = if (s.v[261] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[518] = if (s.v[261] < p.p138) { 1.0 } else { 0.0 };

        if (((s.v[515] != 0.0) && (s.v[517] != 0.0)) && (s.v[518] != 0.0)) {
            s.store_exp(91, 261);
        }

        if (((s.v[515] != 0.0) && (s.v[517] != 0.0)) && (!(s.v[518] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (((s.v[515] != 0.0) && (s.v[517] != 0.0)) && (!(s.v[518] != 0.0))) {
            s.store_mul_ad_rhs(91, 281, A::offset(A::offset(s.ad_value(261), (-p.p138)), 1.0));
        }

        if ((s.v[515] != 0.0) && (s.v[517] != 0.0)) {
            s.store_mul_ad(69, A::neg(s.ad_value(238)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(261)), 1.0));
        }

        if ((s.v[515] != 0.0) && (!(s.v[517] != 0.0))) {
            s.store_mul_ad(69, A::mul(A::scale(s.ad_value(238), 0.5), s.ad_value(261)), A::offset(A::mul(A::scale(s.ad_value(261), 0.3333333333333333), A::offset(A::scale(s.ad_value(261), 0.25), 1.0)), 1.0));
        }

        if (s.v[515] != 0.0) {
            s.store_scale_ad(57, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(58), 2.0), s.ad_value(69)), s.ad_value(59)), s.ad_value(68)), s.ad_value(65)), s.v[63]);
        }

        if (!(s.v[515] != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        if (!(s.v[515] != 0.0)) {
            s.store_scalar(57, 0.0);
        }

        s.v[519] = if (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[236] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[519] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(236), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[520] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[519] != 0.0) && (s.v[520] != 0.0)) {
            s.store_exp_ad(78, A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))));
        }

        if ((s.v[519] != 0.0) && (!(s.v[520] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((s.v[519] != 0.0) && (!(s.v[520] != 0.0))) {
            s.store_mul_ad_rhs(78, 281, A::offset(A::offset(A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))), (-p.p138)), 1.0));
        }

        if (s.v[519] != 0.0) {
            s.store_mul(263, 236, 67);
        }

        if (s.v[519] != 0.0) {
            let assign4360_ad_e4213: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(263)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(263), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(263), 6.0), s.ad_value(263)), A::offset(s.ad_value(263), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4360_ad_e4213);
        }

        if (s.v[519] != 0.0) {
            s.store_div_ad(263, A::mul(A::scale(s.ad_value(236), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[521] = if (s.v[263] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[522] = if (s.v[263] < p.p138) { 1.0 } else { 0.0 };

        if (((s.v[519] != 0.0) && (s.v[521] != 0.0)) && (s.v[522] != 0.0)) {
            s.store_exp(92, 263);
        }

        if (((s.v[519] != 0.0) && (s.v[521] != 0.0)) && (!(s.v[522] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (((s.v[519] != 0.0) && (s.v[521] != 0.0)) && (!(s.v[522] != 0.0))) {
            s.store_mul_ad_rhs(92, 281, A::offset(A::offset(s.ad_value(263), (-p.p138)), 1.0));
        }

        if ((s.v[519] != 0.0) && (s.v[521] != 0.0)) {
            s.store_mul_ad(81, A::neg(s.ad_value(236)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(263)), 1.0));
        }

        if ((s.v[519] != 0.0) && (!(s.v[521] != 0.0))) {
            s.store_mul_ad(81, A::mul(A::scale(s.ad_value(236), 0.5), s.ad_value(263)), A::offset(A::mul(A::scale(s.ad_value(263), 0.3333333333333333), A::offset(A::scale(s.ad_value(263), 0.25), 1.0)), 1.0));
        }

        if (s.v[519] != 0.0) {
            s.store_scale_ad(82, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(84), 2.0), s.ad_value(81)), s.ad_value(77)), s.ad_value(78)), s.ad_value(67)), s.v[89]);
        }

        if (!(s.v[519] != 0.0)) {
            s.store_scalar(81, 0.0);
        }

        if (!(s.v[519] != 0.0)) {
            s.store_scalar(82, 0.0);
        }

        s.store_mul(161, 142, 254);

        s.store_scale(162, 256, 4.0);

        s.store_div_ad(164, A::sub(s.ad_value(161), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(161), 1.0)), 1.0));

        s.store_div_ad_rhs(163, 162, A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0));

        s.store_div_ad(160, A::mul(A::scale(s.ad_value(43), 2.0), A::offset(s.ad_value(254), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(43), 4.0), s.ad_value(37)), s.ad_value(254)), 1.0)), 1.0));

        s.v[523] = if ((p.p5 > 0.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[523] != 0.0) {
            s.store_scale(160, 160, s.v[153]);
        }

        if (s.v[523] != 0.0) {
            s.store_div_ad(167, A::mul(A::scale(s.ad_value(43), (p.p32 * 2.0)), A::offset(s.ad_value(255), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(43), 4.0), s.ad_value(37)), s.ad_value(255)), 1.0)), 1.0));
        }

        if (s.v[523] != 0.0) {
            s.store_scalar(168, 0.0);
        }

        s.v[524] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad_lhs(277, A::scale(s.ad_value(43), p.p32), 32);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad_rhs(169, 6, A::sub_from_scalar(2.0, A::ln(A::mul(s.ad_value(277), s.ad_value(8)))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub(270, 247, 169);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scalar(267, (0.11 * 0.11));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_square(268, 270);
        }

        s.v[525] = if (s.v[270] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_div_ad(170, A::scale(s.ad_value(267), 0.5), A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(270)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_scale_ad(170, A::add(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(270)), 0.5);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_rhs(171, 170, A::add(A::add(s.ad_value(277), A::mul(A::add(s.ad_value(167), s.ad_value(168)), s.ad_value(32))), s.ad_value(170)));
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(169, 0.0);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(170, 0.0);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(171, 1.0);
        }

        if (s.v[523] != 0.0) {
            s.store_mul(172, 171, 167);
        }

        s.v[526] = if (p.p83 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[526] != 0.0) {
            s.store_add(328, 240, 236);
        }

        if (s.v[526] != 0.0) {
            s.store_scalar(267, (1e-6 * 1e-6));
        }

        if (s.v[526] != 0.0) {
            s.store_mul_ad_lhs(268, A::scale(s.ad_value(328), ((-1.0) * (-1.0))), 328);
        }

        s.v[527] = if (((-1.0) * s.v[328]) < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[526] != 0.0) && (s.v[527] != 0.0)) {
            s.store_div_ad(329, A::scale(s.ad_value(267), 0.5), A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), A::scale(s.ad_value(328), (-1.0))));
        }

        if ((s.v[526] != 0.0) && (!(s.v[527] != 0.0))) {
            s.store_scale_ad(329, A::add(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), A::scale(s.ad_value(328), (-1.0))), 0.5);
        }

        if (s.v[526] != 0.0) {
            s.store_scalar(330, (1.0 / (1.0 - ((s.v[324]) as f64).powf(p.p81))));
        }

        if (s.v[526] != 0.0) {
            s.store_scalar(325, (s.v[324] * p.p80));
        }

        if (s.v[526] != 0.0) {
            s.store_scale_ad(327, A::square(s.ad_value(330)), (((s.v[324]) as f64).powf((p.p81 - 1.0)) * (p.p81 * 1.0 / (p.p80))));
        }

        s.v[528] = if (s.v[329] < s.v[325]) { 1.0 } else { 0.0 };

        if ((s.v[526] != 0.0) && (s.v[528] != 0.0)) {
            s.store_div_from_scalar_ad(326, 1.0, A::sub_from_scalar(1.0, A::powf(A::scale(s.ad_value(329), 1.0 / (p.p80)), p.p81)));
        }

        if ((s.v[526] != 0.0) && (!(s.v[528] != 0.0))) {
            s.store_add_ad_rhs(326, 330, A::mul(A::sub(s.ad_value(329), s.ad_value(325)), s.ad_value(327)));
        }

        if (!(s.v[526] != 0.0)) {
            s.store_scalar(326, 1.0);
        }

        s.store_mul(82, 82, 326);

        s.store_mul(160, 160, 326);

        s.store_mul(157, 157, 326);

        s.store_mul(172, 172, 326);

        s.store_add_ad(175, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 175);

        s.v[529] = if (s.v[175] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_div_from_scalar_ad(176, (0.5 * s.v[267]), A::sub(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(175)));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scale_ad(176, A::add(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(175)), 0.5);
        }

        s.store_mul_ad_rhs(177, 176, A::offset(A::scale(A::add(s.ad_value(145), s.ad_value(146)), 0.5), 1.0));

        s.store_div(179, 29, 177);

        s.v[530] = if (s.v[179] < s.v[322]) { 1.0 } else { 0.0 };

        if (s.v[530] != 0.0) {
            s.copy_ad(179, 322);
        }

        s.store_scale(178, 179, 3.0);

        s.store_div_ad_lhs(180, A::add(A::mul(A::scale(s.ad_value(6), 2.0), A::offset(s.ad_value(253), (-1.0))), s.ad_value(240)), 178);

        s.v[531] = if (s.v[152] > 0.0) { 1.0 } else { 0.0 };

        s.v[532] = if (p.p38 == 1.0) { 1.0 } else { 0.0 };

        s.v[533] = if (s.v[236] < p.p43) { 1.0 } else { 0.0 };

        s.v[534] = if (((-s.v[152]) / p.p41) < p.p138) { 1.0 } else { 0.0 };

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (s.v[534] != 0.0)) {
            s.store_exp_ad(314, A::scale(A::neg(s.ad_value(152)), 1.0 / (p.p41)));
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[534] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[534] != 0.0))) {
            s.store_mul_ad_rhs(314, 281, A::offset(A::offset(A::scale(A::neg(s.ad_value(152)), 1.0 / (p.p41)), (-p.p138)), 1.0));
        }

        if (((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_mul_ad_lhs(315, A::sub_from_scalar(p.p43, s.ad_value(236)), 314);
        }

        s.v[535] = if (((-s.v[316]) * ((s.v[315]) as f64).powf(p.p40)) < p.p138) { 1.0 } else { 0.0 };

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_exp_ad(319, A::mul(A::neg(s.ad_value(316)), A::powf(s.ad_value(315), p.p40)));
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_mul_ad_rhs(319, 281, A::offset(A::offset(A::mul(A::neg(s.ad_value(316)), A::powf(s.ad_value(315), p.p40)), (-p.p138)), 1.0));
        }

        if (((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_mul_ad_lhs(199, A::mul(A::div_from_scalar(p.p39, s.ad_value(316)), s.ad_value(315)), 319);
        }

        s.v[536] = if (p.p38 == 2.0) { 1.0 } else { 0.0 };

        s.v[537] = if (s.v[236] < s.v[16]) { 1.0 } else { 0.0 };

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
        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_scalar(188, ((2.0 * p.p45) / (p.p44 * p.p44)));
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad_lhs(266, A::sub(s.ad_value(16), s.ad_value(236)), 202);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sqrt_ad(189, A::div(A::scale(s.ad_value(266), 2.0), s.ad_value(188)));
        }

        s.v[538] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) {
            s.store_scalar(190, p.p44);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[538] != 0.0))) {
            s.store_sub_from_scalar_ad(119, 1.0, A::scale(s.ad_value(118), 0.5));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[538] != 0.0))) {
            s.store_mul_ad_lhs(190, A::scale(s.ad_value(119), p.p44), 119);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad(191, A::mul(s.ad_value(189), s.ad_value(190)), A::sqrt(A::add(A::square(s.ad_value(189)), A::square(s.ad_value(190)))));
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad_lhs(192, A::sub(s.ad_value(16), s.ad_value(236)), 191);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_add_ad_rhs(193, 192, A::mul(A::mul(A::scale(s.ad_value(191), 0.5), s.ad_value(188)), s.ad_value(202)));
        }

        s.v[539] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[539] != 0.0)) {
            s.copy_ad(194, 193);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_offset_ad(195, A::scale(A::offset(A::scale(s.ad_value(118), 2.0), 1.0), (2.0 * p.p46)), 1.0);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_scalar(196, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_sub_ad_rhs(197, 192, A::mul(A::mul(A::scale(s.ad_value(191), 0.5), s.ad_value(188)), A::sub(s.ad_value(196), A::div(s.ad_value(152), A::scale(s.ad_value(195), p.p61)))));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_add_ad(266, A::mul(A::sub(s.ad_value(197), s.ad_value(193)), A::sub(s.ad_value(197), s.ad_value(193))), A::scale(A::mul(A::mul(A::scale(s.ad_value(192), 0.1), s.ad_value(192)), s.ad_value(130)), 1.0 / (p.p61)));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_scale_ad(194, A::add(A::add(s.ad_value(197), s.ad_value(193)), A::sqrt(s.ad_value(266))), 0.5);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad_lhs(273, A::sub(s.ad_value(194), s.ad_value(192)), 194);
        }

        s.v[540] = if (((s.v[273]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[540] != 0.0)) {
            s.store_div_ad_lhs(198, A::scale(s.ad_value(191), 0.5), 273);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[540] != 0.0)) {
            s.store_mul_ad(199, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(194)), s.ad_value(198)), A::sub(A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(194))), A::exp(A::mul(A::div(A::neg(s.ad_value(99)), s.ad_value(194)), A::offset(A::div(s.ad_value(190), s.ad_value(198)), 1.0)))));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[540] != 0.0))) {
            s.store_mul_ad(199, A::mul(s.ad_value(0), s.ad_value(190)), A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(194))));
        }

        s.v[541] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        s.v[542] = if (s.v[236] < p.p43) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_mul_ad(203, A::powf(A::sub_from_scalar(p.p43, s.ad_value(236)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(152), A::offset(s.ad_value(152), p.p47))), p.p48));
        }

        s.v[543] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (s.v[543] != 0.0)) {
            s.copy_ad(204, 203);
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) {
            s.store_scaled_offset(205, 152, (-p.p51), 1.0 / (p.p47));
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) {
            s.store_scaled_offset(265, 205, (-1.0), 1.0 / (p.p50));
        }

        s.v[544] = if (s.v[205] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) && (s.v[544] != 0.0)) {
            s.store_offset_ad(206, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), p.p50), 1.0);
        }

        if (((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) && (!(s.v[544] != 0.0))) {
            s.store_add_ad_rhs(206, 205, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), p.p50));
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) {
            s.store_mul_ad_rhs(204, 203, A::powf(s.ad_value(206), p.p49));
        }

        s.v[545] = if (((-s.v[316]) * s.v[204]) < p.p138) { 1.0 } else { 0.0 };

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_exp_ad(319, A::mul(A::neg(s.ad_value(316)), s.ad_value(204)));
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[545] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[545] != 0.0))) {
            s.store_mul_ad_rhs(319, 281, A::offset(A::offset(A::mul(A::neg(s.ad_value(316)), s.ad_value(204)), (-p.p138)), 1.0));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_mul_ad_lhs(199, A::mul(A::div_from_scalar(p.p39, s.ad_value(316)), A::sub_from_scalar(p.p43, s.ad_value(236))), 319);
        }

        s.v[546] = if (s.v[199] > 0.0) { 1.0 } else { 0.0 };

        s.v[547] = if (p.p52 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_add_ad(200, A::add(A::div(s.ad_value(6), A::mul(s.ad_value(152), A::add(s.ad_value(30), s.ad_value(178)))), A::mul(A::div(s.ad_value(149), s.ad_value(35)), s.ad_value(42))), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(178))));
        }

        s.v[548] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) {
            s.store_scaled_sub(265, 199, 200, 1000000.0);
        }

        s.v[549] = if (s.v[199] < s.v[200]) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) && (s.v[549] != 0.0)) {
            s.store_sub_ad_rhs(199, 199, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 1e-6));
        }

        if (((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) && (!(s.v[549] != 0.0))) {
            s.store_sub_ad_rhs(199, 200, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 1e-6));
        }

        if ((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) {
            s.store_mul(201, 152, 199);
        }

        if ((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (!(s.v[548] != 0.0))) {
            s.store_div_ad(201, A::mul(A::mul(s.ad_value(152), s.ad_value(199)), s.ad_value(200)), A::add(s.ad_value(199), s.ad_value(200)));
        }

        if (((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (!(s.v[547] != 0.0))) {
            s.store_mul(201, 152, 199);
        }

        s.v[550] = if (s.v[124] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[550] != 0.0) {
            s.store_mul_ad_rhs(123, 6, A::ln(s.ad_value(124)));
        }

        if (!(s.v[550] != 0.0)) {
            s.copy_ad(123, 237);
        }

        s.v[551] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[551] != 0.0) {
            s.copy_ad(93, 236);
        }

        if (!(s.v[551] != 0.0)) {
            s.copy_ad(93, 237);
        }

        let assign5720_ad_e5783: A = A::add(A::add(A::add(A::sub(A::add(A::mul(s.ad_value(152), A::sub(s.ad_value(238), s.ad_value(123))), A::mul(s.ad_value(111), A::sub(s.ad_value(123), s.ad_value(236)))), A::mul(s.ad_value(201), s.ad_value(123))), A::div(A::square(s.ad_value(245)), s.ad_value(28))), A::mul(A::square(s.ad_value(248)), s.ad_value(104))), A::mul(A::square(s.ad_value(243)), s.ad_value(105)));
        let assign5720_ad_e5815: A = A::add(A::add(A::add(A::add(assign5720_ad_e5783, A::mul(A::square(s.ad_value(244)), s.ad_value(106))), A::div(A::square(s.ad_value(246)), s.ad_value(30))), A::mul(s.ad_value(180), s.ad_value(240))), A::mul(A::add(A::add(A::sub(A::add(A::add(s.ad_value(154), s.ad_value(156)), A::scale(s.ad_value(238), s.v[320])), s.ad_value(57)), s.ad_value(334)), s.ad_value(333)), s.ad_value(238)));
        s.store_add_ad(208, A::add(A::add(A::sub(assign5720_ad_e5815, A::mul(s.ad_value(82), s.ad_value(93))), A::mul(A::add(A::add(s.ad_value(155), s.ad_value(158)), s.ad_value(159)), s.ad_value(239))), A::mul(A::add(A::add(s.ad_value(160), s.ad_value(157)), A::scale(s.ad_value(241), s.v[320])), s.ad_value(241))), A::mul(s.ad_value(172), s.ad_value(247)));

        s.store_mul_ad_lhs(210, A::scale(s.ad_value(23), (1.0 - p.p67)), 134);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(239), s.ad_value(132)), 279);

        s.v[552] = if (s.v[239] < s.v[132]) { 1.0 } else { 0.0 };

        if (s.v[552] != 0.0) {
            s.store_sub_ad_rhs(211, 239, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[552] != 0.0)) {
            s.store_sub_ad_rhs(211, 132, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_mul_ad(212, A::scale(s.ad_value(23), p.p67), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(211), s.ad_value(65))), (1.0 - p.p66)))), A::scale(A::sub(s.ad_value(239), s.ad_value(211)), 3.0)));

        s.store_mul_ad_lhs(213, A::scale(s.ad_value(24), p.p76), 141);

        s.store_mul(214, 95, 36);

        s.store_mul_ad_lhs(218, A::mul(A::scale(s.ad_value(214), 0.5), s.ad_value(145)), 176);

        s.store_mul_ad_lhs(219, A::mul(A::scale(s.ad_value(214), 0.5), s.ad_value(146)), 176);

        s.store_scale(280, 17, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(241), s.ad_value(137)), 280);

        s.v[553] = if (s.v[241] < s.v[137]) { 1.0 } else { 0.0 };

        if (s.v[553] != 0.0) {
            s.store_sub_ad_rhs(220, 241, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[553] != 0.0)) {
            s.store_sub_ad_rhs(220, 137, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_add_ad(221, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(220), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(136), A::sub(s.ad_value(241), s.ad_value(220))));

        s.store_scale_ad(222, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(221)), A::mul(s.ad_value(25), s.ad_value(241)))), ((1.0 - p.p76) * (1.0 - p.p32)));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(247), s.ad_value(137)), 280);

        s.v[554] = if (s.v[247] < s.v[137]) { 1.0 } else { 0.0 };

        if (s.v[554] != 0.0) {
            s.store_sub_ad_rhs(223, 247, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[554] != 0.0)) {
            s.store_sub_ad_rhs(223, 137, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_add_ad(224, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(223), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(136), A::sub(s.ad_value(247), s.ad_value(223))));

        s.store_scale_ad(225, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(224)), A::mul(s.ad_value(25), s.ad_value(247)))), ((1.0 - p.p76) * p.p32));

        s.store_mul_ad(226, A::mul(s.ad_value(94), s.ad_value(36)), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p84)));

        s.v[555] = if ((s.v[238] / (p.p84 * s.v[6])) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_exp_ad(282, A::div(s.ad_value(238), A::scale(s.ad_value(6), p.p84)));
        }

        if (!(s.v[555] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[555] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::div(s.ad_value(238), A::scale(s.ad_value(6), p.p84)), (-p.p138)), 1.0));
        }

        s.store_mul(228, 226, 282);

        s.store_div_ad_lhs(229, A::mul(A::scale(s.ad_value(96), 4.0), s.ad_value(6)), 31);

        s.store_mul_ad(230, A::mul(A::scale(s.ad_value(229), 0.5), s.ad_value(118)), A::offset(A::add(s.ad_value(122), s.ad_value(109)), 2.0));

        s.v[556] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_div_ad(235, A::mul(A::scale(s.ad_value(97), 0.5), A::add(A::mul(s.ad_value(214), s.ad_value(164)), A::mul(s.ad_value(229), s.ad_value(163)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[557] = if ((((s.v[241] - s.v[22]) / p.p90) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if ((!(s.v[556] != 0.0)) && (s.v[557] != 0.0)) {
            s.store_exp_ad(173, A::mul(A::scale(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90)), s.ad_value(8)));
        }

        if ((!(s.v[556] != 0.0)) && (!(s.v[557] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((!(s.v[556] != 0.0)) && (!(s.v[557] != 0.0))) {
            s.store_mul_ad_rhs(173, 281, A::offset(A::offset(A::mul(A::scale(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        if (!(s.v[556] != 0.0)) {
            s.store_div_ad(235, A::mul(A::mul(A::scale(s.ad_value(43), 2.0), s.ad_value(98)), s.ad_value(254)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(173), 4.0), 1.0)), 1.0));
        }

        s.v[558] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[558] != 0.0) {
            s.store_scale(235, 235, s.v[153]);
        }

        s.v[559] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_mul(165, 142, 255);
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_div_ad(166, A::sub(s.ad_value(165), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0));
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_scale(231, 258, 4.0);
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_div_ad_rhs(232, 231, A::offset(A::sqrt(A::offset(s.ad_value(231), 1.0)), 1.0));
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_div_ad(233, A::mul(A::scale(s.ad_value(97), (0.5 * p.p32)), A::add(A::mul(s.ad_value(214), s.ad_value(166)), A::mul(s.ad_value(229), s.ad_value(232)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[560] = if (((s.v[247] - s.v[22]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) && (s.v[560] != 0.0)) {
            s.store_exp_ad(174, A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)));
        }

        if (((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) && (!(s.v[560] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) && (!(s.v[560] != 0.0))) {
            s.store_mul_ad_rhs(174, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        if ((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) {
            s.store_div_ad(233, A::mul(A::mul(A::scale(s.ad_value(43), (2.0 * p.p32)), s.ad_value(98)), s.ad_value(255)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(174), 4.0), 1.0)), 1.0));
        }

        if (s.v[558] != 0.0) {
            s.store_mul(234, 171, 233);
        }

        s.v[561] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_offset_ad(182, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (-p.p66)), (-3.0));
        }

        if (s.v[561] != 0.0) {
            s.store_div_ad_lhs(274, A::sub(s.ad_value(238), s.ad_value(132)), 279);
        }

        s.v[562] = if (s.v[274] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[561] != 0.0) && (s.v[562] != 0.0)) {
            s.store_div_from_scalar_ad(183, 1.0, A::offset(A::exp(s.ad_value(274)), 1.0));
        }

        if ((s.v[561] != 0.0) && (!(s.v[562] != 0.0))) {
            s.store_div_ad(183, A::exp(A::neg(s.ad_value(274))), A::offset(A::exp(A::neg(s.ad_value(274))), 1.0));
        }

        if (s.v[561] != 0.0) {
            s.store_offset_ad(181, A::mul(s.ad_value(182), s.ad_value(183)), 3.0);
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad_lhs(184, A::scale(s.ad_value(23), (1.0 - p.p67)), 181);
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad(187, A::div(A::mul(A::mul(s.ad_value(142), s.ad_value(252)), s.ad_value(8)), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(143), 1.0))));
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad_lhs(185, A::mul(A::scale(s.ad_value(214), 0.5), s.ad_value(176)), 187);
        }

        if (s.v[561] != 0.0) {
            s.store_div_ad_rhs(186, 228, A::scale(s.ad_value(6), p.p84));
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad(217, A::scale(s.ad_value(240), 0.2), A::add(A::add(s.ad_value(184), s.ad_value(185)), s.ad_value(186)));
        }

        if (s.v[561] != 0.0) {
            s.store_scale(227, 228, (1.0 - p.p94));
        }

        if (s.v[561] != 0.0) {
            s.store_add_ad_rhs(313, 218, A::scale(s.ad_value(228), p.p94));
        }

        if (s.v[561] != 0.0) {
            s.store_add_ad_lhs(216, A::scale(s.ad_value(313), p.p93), 219);
        }

        if (s.v[561] != 0.0) {
            s.store_scale(215, 313, (1.0 - p.p93));
        }

        if (!(s.v[561] != 0.0)) {
            s.copy_ad(215, 218);
        }

        if (!(s.v[561] != 0.0)) {
            s.copy_ad(216, 219);
        }

        if (!(s.v[561] != 0.0)) {
            s.copy_ad(227, 228);
        }

        s.v[563] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        let assign6450_ad_e6586: A = A::ddt(A::scale(A::voltage(ctx, &nodes, Some(3), None), p.p134), self.ddt_jacobian(1.0), self.eval_ddt(0, A::scale(A::voltage(ctx, &nodes, Some(3), None), p.p134).value));
        s.store_scale_ad(209, assign6450_ad_e6586, p.p1);

        s.v[331] = (1.0 - p.p135);

        s.v[564] = if (p.p133 > s.v[322]) { 1.0 } else { 0.0 };

        s.v[565] = if (p.p132 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[564] != 0.0) && (s.v[565] != 0.0)) {
            s.store_scale_ad(102, A::scale(A::voltage(ctx, &nodes, Some(3), None), 1.0 / (s.v[103])), p.p1);
        }

        s.v[566] = if (((s.v[331]) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((s.v[564] != 0.0) && (!(s.v[565] != 0.0))) && (s.v[566] != 0.0)) {
            s.store_scale_ad(102, A::ln(A::offset(A::scale(A::voltage(ctx, &nodes, Some(3), None), 1.0 / (s.v[5])), 1.0)), ((s.v[5] / s.v[103]) * p.p1));
        }

        if (((s.v[564] != 0.0) && (!(s.v[565] != 0.0))) && (!(s.v[566] != 0.0))) {
            s.store_scale_ad(102, A::offset(A::powf(A::offset(A::scale(A::voltage(ctx, &nodes, Some(3), None), 1.0 / (s.v[5])), 1.0), s.v[331]), (-1.0)), ((s.v[5] / (s.v[331] * s.v[103])) * p.p1));
        }

        if (!(s.v[564] != 0.0)) {
            s.store_div_ad_lhs(102, A::voltage(ctx, &nodes, Some(3), None), 321);
        }

        s.v[567] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        s.v[568] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.store_scale(287, 2, (4.0 * 1.3806226e-23));

        s.store_div(288, 287, 28);

        s.store_div(289, 287, 30);

        s.store_mul(290, 287, 104);

        s.store_mul(291, 287, 105);

        s.store_mul(292, 287, 106);

        s.store_scale_ad(293, A::mul(A::div(s.ad_value(287), s.ad_value(178)), A::offset(A::scale(s.ad_value(253), 4.0), 5.0)), 0.3333333333333333);

        s.store_div_ad_lhs(309, A::add(s.ad_value(151), s.ad_value(150)), 149);

        s.store_scale_ad(294, A::abs(s.ad_value(309)), (2.0 * 1.6021918e-19));

        s.v[569] = if (p.p129 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[569] != 0.0) {
            s.store_abs_ad(310, A::div(s.ad_value(201), s.ad_value(309)));
        }

        if (!(s.v[569] != 0.0)) {
            s.store_scalar(310, 0.0);
        }

        s.store_mul_ad(306, A::scale(s.ad_value(201), (2.0 * 1.6021918e-19)), A::offset(s.ad_value(310), 1.0));

        s.v[570] = if (s.v[309] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[570] != 0.0) {
            s.store_div_ad_lhs(311, A::add(s.ad_value(215), s.ad_value(216)), 309);
        }

        if (!(s.v[570] != 0.0)) {
            s.store_mul_ad_lhs(311, A::mul(s.ad_value(95), s.ad_value(176)), 149);
        }

        s.v[571] = if (p.p130 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[571] != 0.0) {
            s.store_scale(312, 311, p.p93);
        }

        s.v[572] = if (p.p130 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[571] != 0.0)) && (s.v[572] != 0.0)) {
            s.store_scale(312, 311, p.p131);
        }

        if ((!(s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
            s.store_scalar(312, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_scale_ad(295, A::abs(A::add(A::add(A::sub(A::add(s.ad_value(154), s.ad_value(156)), s.ad_value(57)), s.ad_value(334)), s.ad_value(333))), (2.0 * 1.6021918e-19));

        s.store_add(307, 154, 155);

        s.store_scale_ad(296, A::powf(A::abs(s.ad_value(307)), p.p125), p.p127);

        s.v[573] = if (s.v[307] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[573] != 0.0) {
            s.store_neg(296, 296);
        }

        s.store_add_ad_lhs(308, A::add(s.ad_value(156), s.ad_value(158)), 159);

        s.store_scale_ad(297, A::powf(A::abs(s.ad_value(308)), p.p126), p.p128);

        s.v[574] = if (s.v[308] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[574] != 0.0) {
            s.store_neg(297, 297);
        }

        s.store_scale_ad(298, A::abs(A::add(A::add(s.ad_value(155), s.ad_value(158)), s.ad_value(159))), (2.0 * 1.6021918e-19));

        s.store_scale_ad(299, A::abs(s.ad_value(157)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(300, A::powf(A::abs(s.ad_value(157)), p.p125), p.p127);

        s.v[575] = if (s.v[157] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[575] != 0.0) {
            s.store_neg(300, 300);
        }

        s.store_scale_ad(301, A::abs(s.ad_value(82)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(302, A::abs(s.ad_value(160)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(304, A::powf(A::scale(A::abs(s.ad_value(160)), 1.0 / ((1.0 - (p.p5 * p.p32)))), p.p125), (p.p127 * (1.0 - (p.p5 * p.p32))));

        s.v[576] = if (s.v[160] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[576] != 0.0) {
            s.store_neg(304, 304);
        }

        s.store_scale_ad(303, A::abs(s.ad_value(172)), ((2.0 * 1.6021918e-19) * p.p5));

        s.v[577] = if (p.p32 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[577] != 0.0) {
            s.store_scalar(305, 0.0);
        }

        if (!(s.v[577] != 0.0)) {
            s.store_scale_ad(305, A::powf(A::scale(A::abs(s.ad_value(172)), 1.0 / (p.p32)), p.p125), ((p.p127 * p.p5) * p.p32));
        }

        s.v[578] = if (s.v[172] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[578] != 0.0) {
            s.store_neg(305, 305);
        }

        s.v[579] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[580] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        s.v[581] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.v[582] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

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
        s.v[447] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[447] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[447] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[447] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[447] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[153] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[448] = if (p.p141 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[448] != 0.0) {
            s.store_scalar(321, 1e-12);
        }

        if (!(s.v[448] != 0.0)) {
            s.store_scalar(321, p.p141);
        }

        s.store_scale(322, 321, p.p1);

        s.v[52] = 0.001;

        s.v[318] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[265] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.v[449] = if ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[449] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[449] != 0.0)) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[265])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[265] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.v[450] = if ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[450] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[265]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[450] != 0.0)) {
            s.store_scalar(88, ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) + (0.1 * (((1.0 + (((-s.v[265])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p116;

        s.v[86] = (1.0 / s.v[87]);

        s.v[171] = 1.0;

        s.v[199] = 0.0;

        s.v[234] = 0.0;

        s.v[217] = 0.0;

        s.v[42] = 0.0;

        s.store_ad(207, &A::voltage(ctx, &nodes, Some(3), None));

        s.v[451] = if (s.v[207] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[451] != 0.0) {
            s.store_neg_ad(207, A::ln(A::sub_from_scalar(1.0, s.ad_value(207))));
        }

        s.v[452] = if (s.v[207] < p.p124) { 1.0 } else { 0.0 };

        if (s.v[452] != 0.0) {
            s.copy_ad(11, 207);
        }

        if (!(s.v[452] != 0.0)) {
            s.store_offset_ad(11, A::ln(A::offset(A::offset(s.ad_value(207), (-p.p124)), 1.0)), p.p124);
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

        s.store_scale_ad(265, A::offset(A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p114), s.ad_value(2)), A::offset(s.ad_value(2), p.p115))), (-0.05)), 10.0);

        s.v[453] = if ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[453] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[453] != 0.0)) {
            s.store_add_ad(70, A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p114), s.ad_value(2)), A::offset(s.ad_value(2), p.p115))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 0.1));
        }

        s.store_scale_ad(265, A::offset(A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p117), s.ad_value(2)), A::offset(s.ad_value(2), p.p118))), (-0.05)), 10.0);

        s.v[454] = if ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[454] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[454] != 0.0)) {
            s.store_add_ad(85, A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p117), s.ad_value(2)), A::offset(s.ad_value(2), p.p118))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 0.1));
        }

        s.store_add_ad(13, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p65)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p104));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.v[455] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[455] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[455] != 0.0)) {
            s.store_offset_ad(14, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(15, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p63)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.v[456] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[456] != 0.0) {
            s.store_add_ad_rhs(16, 15, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[456] != 0.0)) {
            s.store_offset_ad(16, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(21, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p79)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.v[457] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[457] != 0.0) {
            s.store_add_ad_rhs(22, 21, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[457] != 0.0)) {
            s.store_offset_ad(22, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(18, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p70)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.v[458] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[458] != 0.0) {
            s.store_add_ad_rhs(17, 18, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[458] != 0.0)) {
            s.store_offset_ad(17, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(20, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), s.v[75])), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.v[459] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[459] != 0.0) {
            s.store_add_ad_rhs(19, 20, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[459] != 0.0)) {
            s.store_offset_ad(19, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
        }

        s.store_add_ad(56, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(260)), A::scale(s.ad_value(4), p.p26)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p108));

        s.store_div_ad_lhs(265, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.v[460] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[460] != 0.0) {
            s.store_add_ad_rhs(55, 56, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[460] != 0.0)) {
            s.store_offset_ad(55, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))), 0.05);
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

        s.store_scale_ad(28, A::exp(A::scale(s.ad_value(260), p.p96)), p.p53);

        s.v[461] = if (s.v[28] < s.v[322]) { 1.0 } else { 0.0 };

        if (s.v[461] != 0.0) {
            s.copy_ad(28, 322);
        }

        s.store_scale_ad(29, A::exp(A::scale(s.ad_value(260), (p.p97 - p.p95))), p.p55);

        s.store_scale_ad(30, A::exp(A::scale(s.ad_value(260), p.p100)), p.p54);

        s.v[462] = if (s.v[30] < s.v[322]) { 1.0 } else { 0.0 };

        if (s.v[462] != 0.0) {
            s.copy_ad(30, 322);
        }

        s.store_scale_ad(32, A::exp(A::scale(s.ad_value(260), p.p101)), p.p56);

        s.store_scale_ad(31, A::exp(A::scale(s.ad_value(260), p.p98)), p.p59);

        s.v[463] = if (p.p121 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[463] != 0.0) {
            s.store_scale_ad(50, A::offset(A::scale(s.ad_value(12), p.p121), 1.0), p.p9);
        }

        if (s.v[463] != 0.0) {
            s.store_scaled_offset(265, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[464] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[463] != 0.0) && (s.v[464] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[463] != 0.0) && (!(s.v[464] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), s.v[52]));
        }

        if (s.v[463] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[463] != 0.0)) {
            s.store_scalar(48, p.p9);
        }

        s.v[465] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[465] != 0.0) {
            s.store_scale_ad(51, A::offset(A::scale(s.ad_value(12), p.p122), 1.0), p.p10);
        }

        if (s.v[465] != 0.0) {
            s.store_scaled_offset(265, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[466] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[465] != 0.0) && (s.v[466] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[465] != 0.0) && (!(s.v[466] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), s.v[52]));
        }

        if (s.v[465] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[465] != 0.0)) {
            s.store_scalar(49, p.p10);
        }

        s.store_scale_ad(317, A::offset(A::scale(s.ad_value(12), p.p123), 1.0), p.p42);

        s.v[267] = (s.v[318] * s.v[318]);

        s.store_square(268, 317);

        s.v[467] = if (s.v[317] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[467] != 0.0) {
            s.store_div_from_scalar_ad(316, (0.5 * s.v[267]), A::sub(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(317)));
        }

        if (!(s.v[467] != 0.0)) {
            s.store_scale_ad(316, A::add(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(317)), 0.5);
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div(A::scale(s.ad_value(260), (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), p.p8), A::exp(A::div(A::scale(s.ad_value(10), (-p.p104)), s.ad_value(48))));

        s.store_scale_ad(36, A::exp(A::scale(s.ad_value(260), (1.0 - p.p97))), p.p11);

        s.store_scale_ad(37, A::exp(A::scale(s.ad_value(260), (1.0 - p.p102))), p.p29);

        s.store_mul_ad(42, A::scale(A::exp(A::scale(s.ad_value(260), (((4.0 - p.p96) + p.p120) * 1.0 / (p.p16)))), p.p15), A::exp(A::scale(s.ad_value(10), ((-p.p110) * 1.0 / (p.p16)))));

        s.store_mul_ad(43, A::scale(A::exp(A::scale(s.ad_value(260), ((4.0 - p.p102) + p.p120))), p.p28), A::exp(A::scale(s.ad_value(10), (-p.p111))));

        s.store_powf_ad(261, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(262, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p34), s.ad_value(70)), s.ad_value(261)), s.ad_value(262)), p.p65), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(263, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(264, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p36), s.ad_value(85)), s.ad_value(263)), s.ad_value(264)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.store_exp_ad(261, A::scale(s.ad_value(260), p.p95));

        s.store_mul_ad_lhs(40, A::scale(s.ad_value(261), p.p13), 27);

        s.store_mul_ad_lhs(41, A::scale(s.ad_value(261), p.p12), 262);

        s.store_mul_ad(94, A::scale(A::exp(A::scale(s.ad_value(260), (p.p97 - 2.0))), p.p85), A::exp(A::scale(s.ad_value(10), (-p.p119))));

        s.store_scale_ad(95, A::exp(A::scale(s.ad_value(260), ((p.p95 + p.p97) - 1.0))), p.p86);

        s.store_scale_ad(96, A::exp(A::scale(s.ad_value(260), (p.p98 - 1.0))), p.p87);

        s.store_scaled_add(97, 95, 96, (p.p88 * 1.0 / ((p.p86 + p.p87))));

        s.store_scale_ad(98, A::exp(A::scale(s.ad_value(260), (p.p99 - 1.0))), p.p89);

        s.store_offset(101, 2, (-300.0));

        s.v[469] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[469] != 0.0) {
            s.store_mul_ad_rhs(99, 1, A::sub(A::offset(A::scale(s.ad_value(101), 0.00072), 1.0), A::mul(A::scale(s.ad_value(101), 1.6e-6), s.ad_value(101))));
        }

        if (!(s.v[469] != 0.0)) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scale_ad(100, A::exp(A::scale(s.ad_value(260), p.p95)), p.p91);

        s.store_ad(236, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(237, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(8)), p.p3));

        s.store_ad(238, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(4)), p.p3));

        s.store_ad(239, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), p.p3));

        s.store_ad(240, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), p.p3));

        s.store_ad(242, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(8)), p.p3));

        s.store_ad(246, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p3));

        s.store_ad(249, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(250, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(244, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(7)), p.p3));

        s.store_ad(243, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(10)), p.p3));

        s.store_sub_ad_lhs(241, A::sub(A::add(s.ad_value(240), s.ad_value(237)), s.ad_value(242)), 244);

        s.store_sub_ad_lhs(248, A::add(A::sub(s.ad_value(246), s.ad_value(250)), s.ad_value(241)), 243);

        s.store_add(247, 250, 248);

        s.v[476] = if ((s.v[237] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[476] != 0.0) {
            s.store_exp_ad(251, A::mul(s.ad_value(237), s.ad_value(8)));
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[476] != 0.0)) {
            s.store_mul_ad_rhs(251, 281, A::offset(A::offset(A::mul(s.ad_value(237), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[477] = if (((s.v[238] * s.v[8]) / s.v[48]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[477] != 0.0) {
            s.store_exp_ad(252, A::div(A::mul(s.ad_value(238), s.ad_value(8)), s.ad_value(48)));
        }

        if (!(s.v[477] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[477] != 0.0)) {
            s.store_mul_ad_rhs(252, 281, A::offset(A::offset(A::div(A::mul(s.ad_value(238), s.ad_value(8)), s.ad_value(48)), (-p.p138)), 1.0));
        }

        s.v[478] = if ((s.v[241] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[478] != 0.0) {
            s.store_exp_ad(254, A::mul(s.ad_value(241), s.ad_value(8)));
        }

        if (!(s.v[478] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[478] != 0.0)) {
            s.store_mul_ad_rhs(254, 281, A::offset(A::offset(A::mul(s.ad_value(241), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[479] = if ((s.v[240] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (!(s.v[479] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.v[480] = if ((s.v[247] * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[480] != 0.0) {
            s.store_exp_ad(255, A::mul(s.ad_value(247), s.ad_value(8)));
        }

        if (!(s.v[480] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[480] != 0.0)) {
            s.store_mul_ad_rhs(255, 281, A::offset(A::offset(A::mul(s.ad_value(247), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[481] = if (((s.v[247] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[481] != 0.0) {
            s.store_exp_ad(258, A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[481] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[481] != 0.0)) {
            s.store_mul_ad_rhs(258, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(247), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[482] = if (((s.v[241] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[482] != 0.0) {
            s.store_exp_ad(256, A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)));
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
        if (!(s.v[482] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[482] != 0.0)) {
            s.store_mul_ad_rhs(256, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(241), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[483] = if (((s.v[237] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[483] != 0.0) {
            s.store_exp_ad(257, A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[483] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[483] != 0.0)) {
            s.store_mul_ad_rhs(257, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(237), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[484] = if (((s.v[236] - s.v[16]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_exp_ad(259, A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[484] != 0.0)) {
            s.store_mul_ad_rhs(259, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(236), s.ad_value(16)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.store_sqrt_ad(107, A::offset(A::scale(s.ad_value(257), 4.0), 1.0));

        s.store_sqrt_ad(108, A::offset(A::scale(s.ad_value(259), 4.0), 1.0));

        s.store_div_ad(109, A::scale(s.ad_value(259), 2.0), A::offset(s.ad_value(108), 1.0));

        s.v[485] = if (s.v[109] < p.p140) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.store_scalar(109, p.p140);
        }

        s.store_mul_ad_rhs(110, 6, A::sub(A::sub(s.ad_value(107), s.ad_value(108)), A::ln(A::div(A::offset(s.ad_value(107), 1.0), A::offset(s.ad_value(108), 1.0)))));

        s.store_div_ad_lhs(111, A::add(s.ad_value(110), s.ad_value(242)), 31);

        s.v[486] = if (s.v[111] > 0.0) { 1.0 } else { 0.0 };

        s.v[487] = if (s.v[236] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[487] != 0.0)) {
            s.copy_ad(283, 236);
        }

        if ((s.v[486] != 0.0) && (!(s.v[487] != 0.0))) {
            s.store_offset_ad(283, A::ln(A::offset(A::offset(s.ad_value(236), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[486] != 0.0) {
            s.store_sub_ad_lhs(112, A::add(s.ad_value(16), A::mul(A::scale(s.ad_value(6), 2.0), A::ln(A::offset(A::mul(A::mul(A::scale(s.ad_value(111), 0.5), s.ad_value(31)), s.ad_value(8)), 1.0)))), 283);
        }

        if (s.v[486] != 0.0) {
            s.store_scale(278, 16, 0.2);
        }

        if (s.v[486] != 0.0) {
            s.store_square(267, 278);
        }

        if (s.v[486] != 0.0) {
            s.store_square(268, 112);
        }

        s.v[488] = if (s.v[112] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[488] != 0.0)) {
            s.store_div_ad(113, A::scale(s.ad_value(267), 0.5), A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(112)));
        }

        if ((s.v[486] != 0.0) && (!(s.v[488] != 0.0))) {
            s.store_scale_ad(113, A::add(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(112)), 0.5);
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(114, A::mul(s.ad_value(113), A::offset(s.ad_value(113), (p.p61 * p.p60))), A::scale(A::add(s.ad_value(113), A::scale(s.ad_value(31), p.p61)), p.p60));
        }

        if (s.v[486] != 0.0) {
            s.store_div(271, 111, 114);
        }

        if (s.v[486] != 0.0) {
            s.store_scaled_offset(265, 271, (-1.0), 1.0 / (p.p62));
        }

        s.v[489] = if (s.v[271] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[489] != 0.0)) {
            s.store_offset_ad(269, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), p.p62), 1.0);
        }

        if ((s.v[486] != 0.0) && (!(s.v[489] != 0.0))) {
            s.store_add_ad_rhs(269, 271, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), p.p62));
        }

        if (s.v[486] != 0.0) {
            s.store_scale(115, 269, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[486] != 0.0) {
            s.store_scale(116, 113, 1.0 / ((p.p61 * p.p60)));
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(117, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(115), 4.0), s.ad_value(116)), A::offset(s.ad_value(116), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(115), 2.0), A::offset(s.ad_value(116), 1.0)));
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(118, A::add(A::sub_from_scalar(1.0, s.ad_value(117)), A::mul(s.ad_value(109), s.ad_value(117))), A::offset(A::mul(s.ad_value(109), s.ad_value(117)), 1.0));
        }

        if (s.v[486] != 0.0) {
            s.store_mul_ad_lhs(120, A::mul(A::mul(A::scale(s.ad_value(111), 0.5), s.ad_value(31)), s.ad_value(118)), 8);
        }

        if (s.v[486] != 0.0) {
            s.store_add_ad(272, A::scale(s.ad_value(120), 2.0), A::mul(s.ad_value(109), A::offset(A::add(s.ad_value(109), s.ad_value(120)), 1.0)));
        }

        if (s.v[486] != 0.0) {
            s.store_scaled_offset(121, 120, (-1.0), 0.5);
        }

        if (s.v[486] != 0.0) {
            s.store_add_ad_lhs(266, A::square(s.ad_value(121)), 272);
        }

        s.v[490] = if (s.v[120] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[490] != 0.0)) {
            s.store_add_ad_rhs(122, 121, A::sqrt(s.ad_value(266)));
        }

        if ((s.v[486] != 0.0) && (!(s.v[490] != 0.0))) {
            s.store_div_ad_rhs(122, 272, A::sub(A::sqrt(s.ad_value(266)), s.ad_value(121)));
        }

        s.v[491] = if (s.v[122] < p.p139) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[491] != 0.0)) {
            s.store_scalar(122, p.p139);
        }

        if (s.v[486] != 0.0) {
            s.store_mul_ad(124, A::mul(s.ad_value(122), A::offset(s.ad_value(122), 1.0)), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
        }

        if (s.v[486] != 0.0) {
            s.store_scaled_offset(126, 111, (-p.p61), (0.5 * p.p60));
        }

        if (s.v[486] != 0.0) {
            s.store_mul_ad_lhs(127, A::scale(s.ad_value(31), (p.p60 * p.p61)), 111);
        }

        if (s.v[486] != 0.0) {
            s.store_add_ad_rhs(128, 126, A::sqrt(A::add(A::square(s.ad_value(126)), s.ad_value(127))));
        }

        s.v[492] = if (p.p72 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[486] != 0.0) && (s.v[492] != 0.0)) {
            s.store_scale(129, 17, 0.1);
        }

        if ((s.v[486] != 0.0) && (!(s.v[492] != 0.0))) {
            s.store_mul_ad_rhs(129, 17, A::offset(A::div(A::scale(s.ad_value(111), 2.0), A::add(s.ad_value(111), s.ad_value(114))), 0.1));
        }

        if (s.v[486] != 0.0) {
            s.store_div_ad(130, A::scale(s.ad_value(111), p.p61), A::offset(s.ad_value(111), p.p61));
        }

        if (s.v[486] != 0.0) {
            s.store_div_from_scalar_ad(202, p.p61, A::offset(s.ad_value(111), p.p61));
        }

        if (!(s.v[486] != 0.0)) {
            s.store_scalar(114, 0.0);
        }

        if (!(s.v[486] != 0.0)) {
            s.store_div_ad(122, A::scale(s.ad_value(257), 2.0), A::offset(s.ad_value(107), 1.0));
        }

        if (!(s.v[486] != 0.0)) {
            s.copy_ad(124, 251);
        }

        s.v[493] = if ((((s.v[242]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[110]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[107] + s.v[108])))) { 1.0 } else { 0.0 };

        if ((!(s.v[486] != 0.0)) && (s.v[493] != 0.0)) {
            s.store_scaled_add(131, 122, 109, 0.5);
        }

        if ((!(s.v[486] != 0.0)) && (s.v[493] != 0.0)) {
            s.store_div_ad_rhs(118, 131, A::offset(s.ad_value(131), 1.0));
        }

        if ((!(s.v[486] != 0.0)) && (!(s.v[493] != 0.0))) {
            s.store_div_ad_rhs(118, 110, A::sub(A::add(s.ad_value(110), s.ad_value(237)), s.ad_value(236)));
        }

        if (!(s.v[486] != 0.0)) {
            s.copy_ad(128, 242);
        }

        if (!(s.v[486] != 0.0)) {
            s.store_scale(129, 17, 0.1);
        }

        if (!(s.v[486] != 0.0)) {
            s.copy_ad(130, 111);
        }

        if (!(s.v[486] != 0.0)) {
            s.store_sub_from_scalar_ad(202, 1.0, A::scale(s.ad_value(130), 1.0 / (p.p61)));
        }

        s.store_scale(132, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(279, 14, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(238), s.ad_value(132)), 279);

        s.v[494] = if (s.v[238] < s.v[132]) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_sub_ad_rhs(133, 238, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[494] != 0.0)) {
            s.store_sub_ad_rhs(133, 132, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_ad(134, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(238), s.ad_value(133)), 3.0));

        s.v[495] = if (p.p73 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[495] != 0.0) {
            s.copy_ad(135, 236);
        }

        s.v[496] = if (p.p73 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[495] != 0.0)) && (s.v[496] != 0.0)) {
            s.store_add(135, 236, 128);
        }

        if ((!(s.v[495] != 0.0)) && (!(s.v[496] != 0.0))) {
            s.copy_ad(135, 237);
        }

        s.store_div_ad(136, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(137, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(136), ((-1.0) / p.p71))));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(135), s.ad_value(137)), 129);

        s.v[497] = if (s.v[135] < s.v[137]) { 1.0 } else { 0.0 };

        if (s.v[497] != 0.0) {
            s.store_sub_ad_rhs(138, 135, A::mul(s.ad_value(129), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[497] != 0.0)) {
            s.store_sub_ad_rhs(138, 137, A::mul(s.ad_value(129), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_powf(139, 202, p.p75);

        s.store_add_ad(140, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::mul(s.ad_value(139), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(138), s.ad_value(17))), (1.0 - p.p71))))), A::mul(A::mul(s.ad_value(139), s.ad_value(136)), A::sub(s.ad_value(135), s.ad_value(138))));

        s.store_add_ad(141, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(140)), A::mul(s.ad_value(25), s.ad_value(236)));

        s.store_div_ad_lhs(142, A::scale(s.ad_value(35), 4.0), 36);

        s.store_mul(143, 142, 252);

        s.store_div_ad_rhs(145, 143, A::offset(A::sqrt(A::offset(s.ad_value(143), 1.0)), 1.0));

        s.store_ad(125, &A::pow(s.ad_value(124), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(144, 142, 125);

        s.store_div_ad_rhs(146, 144, A::offset(A::sqrt(A::offset(s.ad_value(144), 1.0)), 1.0));

        s.v[498] = if (p.p91 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_add_ad(147, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(275, A::mul(A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), s.ad_value(100)), 8);
        }

        if (!(s.v[498] != 0.0)) {
            s.store_mul_ad_lhs(276, A::mul(A::div(A::neg(s.ad_value(141)), s.ad_value(40)), s.ad_value(100)), 8);
        }

        if (!(s.v[498] != 0.0)) {
            s.store_div_ad(147, A::sub(A::exp(s.ad_value(275)), A::exp(s.ad_value(276))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 147);

        s.v[499] = if (s.v[147] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[499] != 0.0) {
            s.store_div_from_scalar_ad(148, (0.5 * s.v[267]), A::sub(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(147)));
        }

        if (!(s.v[499] != 0.0)) {
            s.store_scale_ad(148, A::add(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(147)), 0.5);
        }

        s.store_mul_ad_rhs(149, 148, A::offset(A::scale(A::add(s.ad_value(145), s.ad_value(146)), 0.5), 1.0));

        s.store_mul_ad_lhs(150, A::scale(s.ad_value(35), p.p14), 125);

        s.store_mul(151, 35, 252);

        s.store_div_ad_lhs(152, A::sub(s.ad_value(151), s.ad_value(150)), 149);

        s.store_scale(265, 238, 10000.0);

        s.v[500] = if (s.v[238] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[500] != 0.0) {
            s.store_scale_ad(282, A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 0.0001);
        }

        if (!(s.v[500] != 0.0)) {
            s.store_add_ad_rhs(282, 238, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 0.0001));
        }

        s.store_scale(284, 282, 1.0 / (p.p143));

        s.v[501] = if (s.v[284] < p.p138) { 1.0 } else { 0.0 };

        if (!(s.v[501] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.store_scaled_offset(265, 238, (-p.p145), 1000.0);

        s.v[503] = if (((s.v[238] * s.v[8]) / p.p16) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[503] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p16)));
        }

        if (!(s.v[503] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[503] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p16)), (-p.p138)), 1.0));
        }

        s.v[504] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[505] = if (((s.v[238] - s.v[55]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[504] != 0.0) && (s.v[505] != 0.0)) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[504] != 0.0) && (!(s.v[505] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((s.v[504] != 0.0) && (!(s.v[505] != 0.0))) {
            s.store_mul_ad_rhs(284, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(238), s.ad_value(55)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[506] = if (((s.v[152] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[504] != 0.0) && (!(s.v[506] != 0.0))) {
            s.store_scalar(281, ((40.0) as f64).exp());
        }

        s.v[508] = if (((s.v[239] * s.v[8]) / p.p18) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[508] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p18)));
        }

        if (!(s.v[508] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[508] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p18)), (-p.p138)), 1.0));
        }

        s.v[509] = if (p.p23 == 1.0) { 1.0 } else { 0.0 };

        s.v[510] = if (((s.v[239] - s.v[55]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[509] != 0.0) && (s.v[510] != 0.0)) {
            s.store_exp_ad(284, A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[509] != 0.0) && (!(s.v[510] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((s.v[509] != 0.0) && (!(s.v[510] != 0.0))) {
            s.store_mul_ad_rhs(284, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(239), s.ad_value(55)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        s.v[511] = if (((s.v[238] * s.v[8]) / p.p20) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[511] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p20)));
        }

        if (!(s.v[511] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[511] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(238), s.ad_value(8)), 1.0 / (p.p20)), (-p.p138)), 1.0));
        }

        s.v[512] = if (((s.v[239] * s.v[8]) / p.p22) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[512] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p22)));
        }

        if (!(s.v[512] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[512] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p22)), (-p.p138)), 1.0));
        }

        s.v[513] = if (((s.v[241] * s.v[8]) / p.p31) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[513] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(241), s.ad_value(8)), 1.0 / (p.p31)));
        }

        if (!(s.v[513] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[513] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(241), s.ad_value(8)), 1.0 / (p.p31)), (-p.p138)), 1.0));
        }

        s.v[514] = if (((s.v[239] * s.v[8]) / p.p137) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[514] != 0.0) {
            s.store_exp_ad(282, A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p137)));
        }

        if (!(s.v[514] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[514] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::scale(A::mul(s.ad_value(239), s.ad_value(8)), 1.0 / (p.p137)), (-p.p138)), 1.0));
        }

        s.v[515] = if (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[238] < 0.0)) { 1.0 } else { 0.0 };

        s.v[516] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[515] != 0.0) && (!(s.v[516] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (s.v[515] != 0.0) {
            s.store_mul(261, 238, 65);
        }

        if (s.v[515] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(261)), 1e-30)), ((-2.0) - p.p66)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p66 * p.p66)), A::scale(s.ad_value(261), (3.0 * (p.p66 - 1.0)))), p.p66), A::mul(A::mul(A::scale(s.ad_value(261), 6.0), s.ad_value(261)), A::offset(s.ad_value(261), (p.p66 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[515] != 0.0) {
            s.store_div_ad(261, A::mul(A::scale(s.ad_value(238), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[517] = if (s.v[261] < (-0.001)) { 1.0 } else { 0.0 };

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
        s.v[518] = if (s.v[261] < p.p138) { 1.0 } else { 0.0 };

        if (((s.v[515] != 0.0) && (s.v[517] != 0.0)) && (!(s.v[518] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.v[519] = if (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[236] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[519] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(236), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[520] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p138) { 1.0 } else { 0.0 };

        if ((s.v[519] != 0.0) && (!(s.v[520] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (s.v[519] != 0.0) {
            s.store_mul(263, 236, 67);
        }

        if (s.v[519] != 0.0) {
            let assign4360_ad_e4213: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(263)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(263), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(263), 6.0), s.ad_value(263)), A::offset(s.ad_value(263), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4360_ad_e4213);
        }

        if (s.v[519] != 0.0) {
            s.store_div_ad(263, A::mul(A::scale(s.ad_value(236), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[521] = if (s.v[263] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[522] = if (s.v[263] < p.p138) { 1.0 } else { 0.0 };

        if (((s.v[519] != 0.0) && (s.v[521] != 0.0)) && (!(s.v[522] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        s.store_mul(161, 142, 254);

        s.store_scale(162, 256, 4.0);

        s.store_div_ad(164, A::sub(s.ad_value(161), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(161), 1.0)), 1.0));

        s.store_div_ad_rhs(163, 162, A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0));

        s.v[523] = if ((p.p5 > 0.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[523] != 0.0) {
            s.store_div_ad(167, A::mul(A::scale(s.ad_value(43), (p.p32 * 2.0)), A::offset(s.ad_value(255), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(43), 4.0), s.ad_value(37)), s.ad_value(255)), 1.0)), 1.0));
        }

        if (s.v[523] != 0.0) {
            s.store_scalar(168, 0.0);
        }

        s.v[524] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad_lhs(277, A::scale(s.ad_value(43), p.p32), 32);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_mul_ad_rhs(169, 6, A::sub_from_scalar(2.0, A::ln(A::mul(s.ad_value(277), s.ad_value(8)))));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_sub(270, 247, 169);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scalar(267, (0.11 * 0.11));
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_square(268, 270);
        }

        s.v[525] = if (s.v[270] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_div_ad(170, A::scale(s.ad_value(267), 0.5), A::sub(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(270)));
        }

        if (((s.v[523] != 0.0) && (s.v[524] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_scale_ad(170, A::add(A::sqrt(A::add(s.ad_value(268), s.ad_value(267))), s.ad_value(270)), 0.5);
        }

        if ((s.v[523] != 0.0) && (s.v[524] != 0.0)) {
            s.store_div_ad_rhs(171, 170, A::add(A::add(s.ad_value(277), A::mul(A::add(s.ad_value(167), s.ad_value(168)), s.ad_value(32))), s.ad_value(170)));
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(169, 0.0);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(170, 0.0);
        }

        if ((s.v[523] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_scalar(171, 1.0);
        }

        s.v[526] = if (p.p83 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[526] != 0.0) {
            s.store_add(328, 240, 236);
        }

        if (s.v[526] != 0.0) {
            s.store_scalar(267, (1e-6 * 1e-6));
        }

        if (s.v[526] != 0.0) {
            s.store_mul_ad_lhs(268, A::scale(s.ad_value(328), ((-1.0) * (-1.0))), 328);
        }

        s.store_add_ad(175, A::offset(A::div(s.ad_value(134), s.ad_value(41)), 1.0), A::div(s.ad_value(141), s.ad_value(40)));

        s.v[267] = (0.1 * 0.1);

        s.store_square(268, 175);

        s.v[529] = if (s.v[175] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_div_from_scalar_ad(176, (0.5 * s.v[267]), A::sub(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(175)));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_scale_ad(176, A::add(A::sqrt(A::offset(s.ad_value(268), s.v[267])), s.ad_value(175)), 0.5);
        }

        s.store_mul_ad_rhs(177, 176, A::offset(A::scale(A::add(s.ad_value(145), s.ad_value(146)), 0.5), 1.0));

        s.store_div(179, 29, 177);

        s.v[530] = if (s.v[179] < s.v[322]) { 1.0 } else { 0.0 };

        if (s.v[530] != 0.0) {
            s.copy_ad(179, 322);
        }

        s.store_scale(178, 179, 3.0);

        s.v[531] = if (s.v[152] > 0.0) { 1.0 } else { 0.0 };

        s.v[532] = if (p.p38 == 1.0) { 1.0 } else { 0.0 };

        s.v[533] = if (s.v[236] < p.p43) { 1.0 } else { 0.0 };

        s.v[534] = if (((-s.v[152]) / p.p41) < p.p138) { 1.0 } else { 0.0 };

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (s.v[534] != 0.0)) {
            s.store_exp_ad(314, A::scale(A::neg(s.ad_value(152)), 1.0 / (p.p41)));
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[534] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[534] != 0.0))) {
            s.store_mul_ad_rhs(314, 281, A::offset(A::offset(A::scale(A::neg(s.ad_value(152)), 1.0 / (p.p41)), (-p.p138)), 1.0));
        }

        if (((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_mul_ad_lhs(315, A::sub_from_scalar(p.p43, s.ad_value(236)), 314);
        }

        s.v[535] = if (((-s.v[316]) * ((s.v[315]) as f64).powf(p.p40)) < p.p138) { 1.0 } else { 0.0 };

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_exp_ad(319, A::mul(A::neg(s.ad_value(316)), A::powf(s.ad_value(315), p.p40)));
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_mul_ad_rhs(319, 281, A::offset(A::offset(A::mul(A::neg(s.ad_value(316)), A::powf(s.ad_value(315), p.p40)), (-p.p138)), 1.0));
        }

        if (((s.v[531] != 0.0) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_mul_ad_lhs(199, A::mul(A::div_from_scalar(p.p39, s.ad_value(316)), s.ad_value(315)), 319);
        }

        s.v[536] = if (p.p38 == 2.0) { 1.0 } else { 0.0 };

        s.v[537] = if (s.v[236] < s.v[16]) { 1.0 } else { 0.0 };

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_scalar(188, ((2.0 * p.p45) / (p.p44 * p.p44)));
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad_lhs(266, A::sub(s.ad_value(16), s.ad_value(236)), 202);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_sqrt_ad(189, A::div(A::scale(s.ad_value(266), 2.0), s.ad_value(188)));
        }

        s.v[538] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[538] != 0.0)) {
            s.store_scalar(190, p.p44);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[538] != 0.0))) {
            s.store_sub_from_scalar_ad(119, 1.0, A::scale(s.ad_value(118), 0.5));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[538] != 0.0))) {
            s.store_mul_ad_lhs(190, A::scale(s.ad_value(119), p.p44), 119);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad(191, A::mul(s.ad_value(189), s.ad_value(190)), A::sqrt(A::add(A::square(s.ad_value(189)), A::square(s.ad_value(190)))));
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad_lhs(192, A::sub(s.ad_value(16), s.ad_value(236)), 191);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_add_ad_rhs(193, 192, A::mul(A::mul(A::scale(s.ad_value(191), 0.5), s.ad_value(188)), s.ad_value(202)));
        }

        s.v[539] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[539] != 0.0)) {
            s.copy_ad(194, 193);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_offset_ad(195, A::scale(A::offset(A::scale(s.ad_value(118), 2.0), 1.0), (2.0 * p.p46)), 1.0);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_scalar(196, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_sub_ad_rhs(197, 192, A::mul(A::mul(A::scale(s.ad_value(191), 0.5), s.ad_value(188)), A::sub(s.ad_value(196), A::div(s.ad_value(152), A::scale(s.ad_value(195), p.p61)))));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_add_ad(266, A::mul(A::sub(s.ad_value(197), s.ad_value(193)), A::sub(s.ad_value(197), s.ad_value(193))), A::scale(A::mul(A::mul(A::scale(s.ad_value(192), 0.1), s.ad_value(192)), s.ad_value(130)), 1.0 / (p.p61)));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_scale_ad(194, A::add(A::add(s.ad_value(197), s.ad_value(193)), A::sqrt(s.ad_value(266))), 0.5);
        }

        if ((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) {
            s.store_div_ad_lhs(273, A::sub(s.ad_value(194), s.ad_value(192)), 194);
        }

        s.v[540] = if (((s.v[273]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[540] != 0.0)) {
            s.store_div_ad_lhs(198, A::scale(s.ad_value(191), 0.5), 273);
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (s.v[540] != 0.0)) {
            s.store_mul_ad(199, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(194)), s.ad_value(198)), A::sub(A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(194))), A::exp(A::mul(A::div(A::neg(s.ad_value(99)), s.ad_value(194)), A::offset(A::div(s.ad_value(190), s.ad_value(198)), 1.0)))));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (s.v[536] != 0.0)) && (s.v[537] != 0.0)) && (!(s.v[540] != 0.0))) {
            s.store_mul_ad(199, A::mul(s.ad_value(0), s.ad_value(190)), A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(194))));
        }

        s.v[541] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        s.v[542] = if (s.v[236] < p.p43) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_mul_ad(203, A::powf(A::sub_from_scalar(p.p43, s.ad_value(236)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(152), A::offset(s.ad_value(152), p.p47))), p.p48));
        }

        s.v[543] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (s.v[543] != 0.0)) {
            s.copy_ad(204, 203);
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) {
            s.store_scaled_offset(205, 152, (-p.p51), 1.0 / (p.p47));
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) {
            s.store_scaled_offset(265, 205, (-1.0), 1.0 / (p.p50));
        }

        s.v[544] = if (s.v[205] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) && (s.v[544] != 0.0)) {
            s.store_offset_ad(206, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), p.p50), 1.0);
        }

        if (((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) && (!(s.v[544] != 0.0))) {
            s.store_add_ad_rhs(206, 205, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), p.p50));
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[543] != 0.0))) {
            s.store_mul_ad_rhs(204, 203, A::powf(s.ad_value(206), p.p49));
        }

        s.v[545] = if (((-s.v[316]) * s.v[204]) < p.p138) { 1.0 } else { 0.0 };

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_exp_ad(319, A::mul(A::neg(s.ad_value(316)), s.ad_value(204)));
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[545] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) && (!(s.v[545] != 0.0))) {
            s.store_mul_ad_rhs(319, 281, A::offset(A::offset(A::mul(A::neg(s.ad_value(316)), s.ad_value(204)), (-p.p138)), 1.0));
        }

        if (((((s.v[531] != 0.0) && (!(s.v[532] != 0.0))) && (!(s.v[536] != 0.0))) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_mul_ad_lhs(199, A::mul(A::div_from_scalar(p.p39, s.ad_value(316)), A::sub_from_scalar(p.p43, s.ad_value(236))), 319);
        }

        s.v[546] = if (s.v[199] > 0.0) { 1.0 } else { 0.0 };

        s.v[547] = if (p.p52 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_add_ad(200, A::add(A::div(s.ad_value(6), A::mul(s.ad_value(152), A::add(s.ad_value(30), s.ad_value(178)))), A::mul(A::div(s.ad_value(149), s.ad_value(35)), s.ad_value(42))), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(178))));
        }

        s.v[548] = if (p.p38 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) {
            s.store_scaled_sub(265, 199, 200, 1000000.0);
        }

        s.v[549] = if (s.v[199] < s.v[200]) { 1.0 } else { 0.0 };

        if (((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) && (s.v[549] != 0.0)) {
            s.store_sub_ad_rhs(199, 199, A::scale(A::ln(A::offset(A::exp(s.ad_value(265)), 1.0)), 1e-6));
        }

        if (((((s.v[531] != 0.0) && (s.v[546] != 0.0)) && (s.v[547] != 0.0)) && (s.v[548] != 0.0)) && (!(s.v[549] != 0.0))) {
            s.store_sub_ad_rhs(199, 200, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0)), 1e-6));
        }

        s.store_mul_ad_lhs(210, A::scale(s.ad_value(23), (1.0 - p.p67)), 134);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(239), s.ad_value(132)), 279);

        s.v[552] = if (s.v[239] < s.v[132]) { 1.0 } else { 0.0 };

        if (s.v[552] != 0.0) {
            s.store_sub_ad_rhs(211, 239, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[552] != 0.0)) {
            s.store_sub_ad_rhs(211, 132, A::mul(s.ad_value(279), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_mul_ad(212, A::scale(s.ad_value(23), p.p67), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p66))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(211), s.ad_value(65))), (1.0 - p.p66)))), A::scale(A::sub(s.ad_value(239), s.ad_value(211)), 3.0)));

        s.store_mul_ad_lhs(213, A::scale(s.ad_value(24), p.p76), 141);

        s.store_mul(214, 95, 36);

        s.store_mul_ad_lhs(218, A::mul(A::scale(s.ad_value(214), 0.5), s.ad_value(145)), 176);

        s.store_mul_ad_lhs(219, A::mul(A::scale(s.ad_value(214), 0.5), s.ad_value(146)), 176);

        s.store_scale(280, 17, 0.1);

        s.store_div_ad_lhs(265, A::sub(s.ad_value(241), s.ad_value(137)), 280);

        s.v[553] = if (s.v[241] < s.v[137]) { 1.0 } else { 0.0 };

        if (s.v[553] != 0.0) {
            s.store_sub_ad_rhs(220, 241, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[553] != 0.0)) {
            s.store_sub_ad_rhs(220, 137, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_add_ad(221, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(220), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(136), A::sub(s.ad_value(241), s.ad_value(220))));

        s.store_scale_ad(222, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(221)), A::mul(s.ad_value(25), s.ad_value(241)))), ((1.0 - p.p76) * (1.0 - p.p32)));

        s.store_div_ad_lhs(265, A::sub(s.ad_value(247), s.ad_value(137)), 280);

        s.v[554] = if (s.v[247] < s.v[137]) { 1.0 } else { 0.0 };

        if (s.v[554] != 0.0) {
            s.store_sub_ad_rhs(223, 247, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(s.ad_value(265)), 1.0))));
        }

        if (!(s.v[554] != 0.0)) {
            s.store_sub_ad_rhs(223, 137, A::mul(s.ad_value(280), A::ln(A::offset(A::exp(A::neg(s.ad_value(265))), 1.0))));
        }

        s.store_add_ad(224, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p71))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(223), s.ad_value(17))), (1.0 - p.p71)))), A::mul(s.ad_value(136), A::sub(s.ad_value(247), s.ad_value(223))));

        s.store_scale_ad(225, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(224)), A::mul(s.ad_value(25), s.ad_value(247)))), ((1.0 - p.p76) * p.p32));

        s.store_mul_ad(226, A::mul(s.ad_value(94), s.ad_value(36)), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p84)));

        s.v[555] = if ((s.v[238] / (p.p84 * s.v[6])) < p.p138) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_exp_ad(282, A::div(s.ad_value(238), A::scale(s.ad_value(6), p.p84)));
        }

        if (!(s.v[555] != 0.0)) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (!(s.v[555] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::offset(A::offset(A::div(s.ad_value(238), A::scale(s.ad_value(6), p.p84)), (-p.p138)), 1.0));
        }

        s.store_mul(228, 226, 282);

        s.store_div_ad_lhs(229, A::mul(A::scale(s.ad_value(96), 4.0), s.ad_value(6)), 31);

        s.store_mul_ad(230, A::mul(A::scale(s.ad_value(229), 0.5), s.ad_value(118)), A::offset(A::add(s.ad_value(122), s.ad_value(109)), 2.0));

        s.v[556] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_div_ad(235, A::mul(A::scale(s.ad_value(97), 0.5), A::add(A::mul(s.ad_value(214), s.ad_value(164)), A::mul(s.ad_value(229), s.ad_value(163)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[557] = if ((((s.v[241] - s.v[22]) / p.p90) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if ((!(s.v[556] != 0.0)) && (s.v[557] != 0.0)) {
            s.store_exp_ad(173, A::mul(A::scale(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90)), s.ad_value(8)));
        }

        if ((!(s.v[556] != 0.0)) && (!(s.v[557] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if ((!(s.v[556] != 0.0)) && (!(s.v[557] != 0.0))) {
            s.store_mul_ad_rhs(173, 281, A::offset(A::offset(A::mul(A::scale(A::sub(s.ad_value(241), s.ad_value(22)), 1.0 / (p.p90)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        if (!(s.v[556] != 0.0)) {
            s.store_div_ad(235, A::mul(A::mul(A::scale(s.ad_value(43), 2.0), s.ad_value(98)), s.ad_value(254)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(173), 4.0), 1.0)), 1.0));
        }

        s.v[558] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[558] != 0.0) {
            s.store_scale(235, 235, s.v[153]);
        }

        s.v[559] = if (p.p78 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_mul(165, 142, 255);
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_div_ad(166, A::sub(s.ad_value(165), s.ad_value(142)), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0));
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_scale(231, 258, 4.0);
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_div_ad_rhs(232, 231, A::offset(A::sqrt(A::offset(s.ad_value(231), 1.0)), 1.0));
        }

        if ((s.v[558] != 0.0) && (s.v[559] != 0.0)) {
            s.store_div_ad(233, A::mul(A::scale(s.ad_value(97), (0.5 * p.p32)), A::add(A::mul(s.ad_value(214), s.ad_value(166)), A::mul(s.ad_value(229), s.ad_value(232)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[560] = if (((s.v[247] - s.v[22]) * s.v[8]) < p.p138) { 1.0 } else { 0.0 };

        if (((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) && (s.v[560] != 0.0)) {
            s.store_exp_ad(174, A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)));
        }

        if (((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) && (!(s.v[560] != 0.0))) {
            s.store_scalar(281, ((p.p138) as f64).exp());
        }

        if (((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) && (!(s.v[560] != 0.0))) {
            s.store_mul_ad_rhs(174, 281, A::offset(A::offset(A::mul(A::sub(s.ad_value(247), s.ad_value(22)), s.ad_value(8)), (-p.p138)), 1.0));
        }

        if ((s.v[558] != 0.0) && (!(s.v[559] != 0.0))) {
            s.store_div_ad(233, A::mul(A::mul(A::scale(s.ad_value(43), (2.0 * p.p32)), s.ad_value(98)), s.ad_value(255)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(174), 4.0), 1.0)), 1.0));
        }

        if (s.v[558] != 0.0) {
            s.store_mul(234, 171, 233);
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
        let nv3 = ctx.node_voltage(nodes[3]);
        s.v[561] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_offset_ad(182, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(133), s.ad_value(65))), (-p.p66)), (-3.0));
        }

        if (s.v[561] != 0.0) {
            s.store_div_ad_lhs(274, A::sub(s.ad_value(238), s.ad_value(132)), 279);
        }

        s.v[562] = if (s.v[274] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[561] != 0.0) && (s.v[562] != 0.0)) {
            s.store_div_from_scalar_ad(183, 1.0, A::offset(A::exp(s.ad_value(274)), 1.0));
        }

        if ((s.v[561] != 0.0) && (!(s.v[562] != 0.0))) {
            s.store_div_ad(183, A::exp(A::neg(s.ad_value(274))), A::offset(A::exp(A::neg(s.ad_value(274))), 1.0));
        }

        if (s.v[561] != 0.0) {
            s.store_offset_ad(181, A::mul(s.ad_value(182), s.ad_value(183)), 3.0);
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad_lhs(184, A::scale(s.ad_value(23), (1.0 - p.p67)), 181);
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad(187, A::div(A::mul(A::mul(s.ad_value(142), s.ad_value(252)), s.ad_value(8)), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(143), 1.0))));
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad_lhs(185, A::mul(A::scale(s.ad_value(214), 0.5), s.ad_value(176)), 187);
        }

        if (s.v[561] != 0.0) {
            s.store_div_ad_rhs(186, 228, A::scale(s.ad_value(6), p.p84));
        }

        if (s.v[561] != 0.0) {
            s.store_mul_ad(217, A::scale(s.ad_value(240), 0.2), A::add(A::add(s.ad_value(184), s.ad_value(185)), s.ad_value(186)));
        }

        if (s.v[561] != 0.0) {
            s.store_scale(227, 228, (1.0 - p.p94));
        }

        if (s.v[561] != 0.0) {
            s.store_add_ad_rhs(313, 218, A::scale(s.ad_value(228), p.p94));
        }

        if (s.v[561] != 0.0) {
            s.store_add_ad_lhs(216, A::scale(s.ad_value(313), p.p93), 219);
        }

        if (s.v[561] != 0.0) {
            s.store_scale(215, 313, (1.0 - p.p93));
        }

        if (!(s.v[561] != 0.0)) {
            s.copy_ad(215, 218);
        }

        if (!(s.v[561] != 0.0)) {
            s.copy_ad(216, 219);
        }

        if (!(s.v[561] != 0.0)) {
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

        s.v[570] = if (s.v[309] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[570] != 0.0) {
            s.store_div_ad_lhs(311, A::add(s.ad_value(215), s.ad_value(216)), 309);
        }

        if (!(s.v[570] != 0.0)) {
            s.store_mul_ad_lhs(311, A::mul(s.ad_value(95), s.ad_value(176)), 149);
        }

        s.v[571] = if (p.p130 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[571] != 0.0) {
            s.store_scale(312, 311, p.p93);
        }

        s.v[572] = if (p.p130 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[571] != 0.0)) && (s.v[572] != 0.0)) {
            s.store_scale(312, 311, p.p131);
        }

        if ((!(s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
            s.store_scalar(312, 0.0);
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
        let eq0_value: f64 = eq0_e156;
        let eq0_node_derivatives: [f64; 12] = [eq0_e156_d_n0, eq0_e156_d_n1, eq0_e156_d_n2, eq0_e156_d_n3, eq0_e156_d_n4, eq0_e156_d_n5, eq0_e156_d_n6, eq0_e156_d_n7, eq0_e156_d_n8, eq0_e156_d_n9, eq0_e156_d_n10, eq0_e156_d_n11];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let eq1_value: f64 = eq1_e161;
        let eq1_node_derivatives: [f64; 12] = [eq1_e161_d_n0, eq1_e161_d_n1, eq1_e161_d_n2, eq1_e161_d_n3, eq1_e161_d_n4, eq1_e161_d_n5, eq1_e161_d_n6, eq1_e161_d_n7, eq1_e161_d_n8, eq1_e161_d_n9, eq1_e161_d_n10, eq1_e161_d_n11];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[4]),
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
        let eq2_value: f64 = eq2_e170;
        let eq2_node_derivatives: [f64; 12] = [eq2_e170_d_n0, eq2_e170_d_n1, eq2_e170_d_n2, eq2_e170_d_n3, eq2_e170_d_n4, eq2_e170_d_n5, eq2_e170_d_n6, eq2_e170_d_n7, eq2_e170_d_n8, eq2_e170_d_n9, eq2_e170_d_n10, eq2_e170_d_n11];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
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
        let eq3_value: f64 = eq3_e187;
        let eq3_node_derivatives: [f64; 12] = [eq3_e187_d_n0, eq3_e187_d_n1, eq3_e187_d_n2, eq3_e187_d_n3, eq3_e187_d_n4, eq3_e187_d_n5, eq3_e187_d_n6, eq3_e187_d_n7, eq3_e187_d_n8, eq3_e187_d_n9, eq3_e187_d_n10, eq3_e187_d_n11];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[4]),
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
        let (eq4_e196, eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n2, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10, eq4_e196_d_n11,) = {
    if (s.v[563] != 0.0) {
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
        (eq4_e194, eq4_e194_d_n0, eq4_e194_d_n1, eq4_e194_d_n2, eq4_e194_d_n3, eq4_e194_d_n4, eq4_e194_d_n5, eq4_e194_d_n6, eq4_e194_d_n7, eq4_e194_d_n8, eq4_e194_d_n9, eq4_e194_d_n10, eq4_e194_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e196;
        let eq4_node_derivatives: [f64; 12] = [eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n2, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10, eq4_e196_d_n11];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let (eq5_e206, eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n2, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10, eq5_e206_d_n11,) = {
    if (!(s.v[563] != 0.0)) {
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
        (eq5_e204, eq5_e204_d_n0, eq5_e204_d_n1, eq5_e204_d_n2, eq5_e204_d_n3, eq5_e204_d_n4, eq5_e204_d_n5, eq5_e204_d_n6, eq5_e204_d_n7, eq5_e204_d_n8, eq5_e204_d_n9, eq5_e204_d_n10, eq5_e204_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e206;
        let eq5_node_derivatives: [f64; 12] = [eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n2, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10, eq5_e206_d_n11];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
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
        let eq6_value: f64 = eq6_e211;
        let eq6_node_derivatives: [f64; 12] = [eq6_e211_d_n0, eq6_e211_d_n1, eq6_e211_d_n2, eq6_e211_d_n3, eq6_e211_d_n4, eq6_e211_d_n5, eq6_e211_d_n6, eq6_e211_d_n7, eq6_e211_d_n8, eq6_e211_d_n9, eq6_e211_d_n10, eq6_e211_d_n11];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }
}
