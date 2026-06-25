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
        s.v[484] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[484] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[160] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[344] = 0.0;

        s.v[485] = if (p.p154 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.store_scalar(345, 1e-12);
        }

        if (!(s.v[485] != 0.0)) {
            s.store_scalar(345, p.p154);
        }

        s.store_scale(346, 345, p.p1);

        s.store_div_from_scalar(347, 1.0, 346);

        s.v[486] = if (p.p134 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[486] != 0.0) {
            s.store_scalar(348, s.v[344]);
        }

        if (!(s.v[486] != 0.0)) {
            s.store_scalar(348, 0.0);
        }

        s.v[52] = 0.001;

        s.v[342] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[63] = (1.0 / s.v[62]);

        s.v[285] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.v[487] = if ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[487] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[487] != 0.0)) {
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

        s.v[488] = if ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[488] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[488] != 0.0)) {
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

        s.store_ad(218, &A::voltage(ctx, &nodes, Some(4), None));

        s.v[489] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_neg_ad(218, A::ln(A::sub_from_scalar(1.0, s.ad_value(218))));
        }

        s.v[490] = if (s.v[218] < p.p125) { 1.0 } else { 0.0 };

        if (s.v[490] != 0.0) {
            s.copy_ad(11, 218);
        }

        if (!(s.v[490] != 0.0)) {
            s.store_offset_ad(11, A::ln(A::offset(A::offset(s.ad_value(218), (-p.p125)), 1.0)), p.p125);
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

        s.store_scale_ad(285, A::offset(A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p115), s.ad_value(2)), A::offset(s.ad_value(2), p.p116))), (-0.05)), 10.0);

        s.v[491] = if ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[491] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[491] != 0.0)) {
            s.store_add_ad(70, A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p115), s.ad_value(2)), A::offset(s.ad_value(2), p.p116))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 0.1));
        }

        s.store_scale_ad(285, A::offset(A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p118), s.ad_value(2)), A::offset(s.ad_value(2), p.p119))), (-0.05)), 10.0);

        s.v[492] = if ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[492] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[492] != 0.0)) {
            s.store_add_ad(85, A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p118), s.ad_value(2)), A::offset(s.ad_value(2), p.p119))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 0.1));
        }

        s.store_add_ad(13, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p66)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p105));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.v[493] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[493] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[493] != 0.0)) {
            s.store_offset_ad(14, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(15, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p64)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.v[494] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_add_ad_rhs(16, 15, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[494] != 0.0)) {
            s.store_offset_ad(16, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(21, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p80)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.v[495] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[495] != 0.0) {
            s.store_add_ad_rhs(22, 21, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[495] != 0.0)) {
            s.store_offset_ad(22, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(18, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p71)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.v[496] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[496] != 0.0) {
            s.store_add_ad_rhs(17, 18, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[496] != 0.0)) {
            s.store_offset_ad(17, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(20, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), s.v[75])), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.v[497] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[497] != 0.0) {
            s.store_add_ad_rhs(19, 20, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[497] != 0.0)) {
            s.store_offset_ad(19, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(56, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p27)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.v[498] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_add_ad_rhs(55, 56, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_offset_ad(55, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(104, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p138)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p140));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(104)), 6);

        s.v[499] = if (0.05 < s.v[104]) { 1.0 } else { 0.0 };

        if (s.v[499] != 0.0) {
            s.store_add_ad_rhs(105, 104, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[499] != 0.0)) {
            s.store_offset_ad(105, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scale_ad(106, A::powf(A::div_from_scalar(p.p138, s.ad_value(105)), p.p139), p.p137);

        s.store_offset_ad(26, A::scale(A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75)), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.store_scale_ad(28, A::exp(A::scale(s.ad_value(280), p.p97)), p.p54);

        s.v[500] = if (s.v[28] < s.v[346]) { 1.0 } else { 0.0 };

        if (s.v[500] != 0.0) {
            s.copy_ad(28, 346);
        }

        s.store_scale_ad(29, A::exp(A::scale(s.ad_value(280), (p.p98 - p.p96))), p.p56);

        s.store_scale_ad(30, A::exp(A::scale(s.ad_value(280), p.p101)), p.p55);

        s.v[501] = if (s.v[30] < s.v[346]) { 1.0 } else { 0.0 };

        if (s.v[501] != 0.0) {
            s.copy_ad(30, 346);
        }

        s.store_scale_ad(32, A::exp(A::scale(s.ad_value(280), p.p102)), p.p57);

        s.store_scale_ad(33, A::exp(A::scale(s.ad_value(280), p.p104)), p.p58);

        s.store_scale_ad(34, A::exp(A::scale(s.ad_value(280), p.p104)), p.p59);

        s.store_scale_ad(31, A::exp(A::scale(s.ad_value(280), p.p99)), p.p60);

        s.v[502] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[502] != 0.0) {
            s.store_scale_ad(50, A::offset(A::scale(s.ad_value(12), p.p122), 1.0), p.p10);
        }

        if (s.v[502] != 0.0) {
            s.store_scaled_offset(285, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[503] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[502] != 0.0) && (s.v[503] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[502] != 0.0) && (!(s.v[503] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), s.v[52]));
        }

        if (s.v[502] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[502] != 0.0)) {
            s.store_scalar(48, p.p10);
        }

        s.v[504] = if (p.p123 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[504] != 0.0) {
            s.store_scale_ad(51, A::offset(A::scale(s.ad_value(12), p.p123), 1.0), p.p11);
        }

        if (s.v[504] != 0.0) {
            s.store_scaled_offset(285, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[505] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[504] != 0.0) && (s.v[505] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[504] != 0.0) && (!(s.v[505] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), s.v[52]));
        }

        if (s.v[504] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[504] != 0.0)) {
            s.store_scalar(49, p.p11);
        }

        s.store_scale_ad(341, A::offset(A::scale(s.ad_value(12), p.p124), 1.0), p.p43);

        s.v[287] = (s.v[342] * s.v[342]);

        s.store_square(288, 341);

        s.v[506] = if (s.v[341] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[506] != 0.0) {
            s.store_div_from_scalar_ad(340, (0.5 * s.v[287]), A::sub(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(341)));
        }

        if (!(s.v[506] != 0.0)) {
            s.store_scale_ad(340, A::add(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(341)), 0.5);
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div(A::scale(s.ad_value(280), (((4.0 - p.p98) - p.p96) + p.p121)), s.ad_value(48))), p.p9), A::exp(A::div(A::scale(s.ad_value(10), (-p.p105)), s.ad_value(48))));

        s.store_scale_ad(36, A::exp(A::scale(s.ad_value(280), (1.0 - p.p98))), p.p12);

        s.store_scale_ad(37, A::exp(A::scale(s.ad_value(280), (1.0 - p.p103))), p.p30);

        s.store_mul_ad(38, A::scale(A::exp(A::scale(s.ad_value(280), (6.0 - (2.0 * p.p21)))), p.p20), A::exp(A::scale(s.ad_value(10), ((-p.p113) * 1.0 / (p.p21)))));

        s.store_mul_ad(39, A::scale(A::exp(A::scale(s.ad_value(280), (6.0 - (2.0 * p.p32)))), p.p31), A::exp(A::scale(s.ad_value(10), ((-p.p110) * 1.0 / (p.p32)))));

        s.store_mul_ad(42, A::scale(A::exp(A::scale(s.ad_value(280), (((4.0 - p.p97) + p.p121) * 1.0 / (p.p17)))), p.p16), A::exp(A::scale(s.ad_value(10), ((-p.p111) * 1.0 / (p.p17)))));

        s.store_mul_ad(44, A::scale(A::exp(A::scale(s.ad_value(280), (((4.0 - p.p97) + p.p121) * 1.0 / (p.p19)))), p.p18), A::exp(A::scale(s.ad_value(10), ((-p.p111) * 1.0 / (p.p19)))));

        s.v[507] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[507] != 0.0) {
            s.store_scale_ad(53, A::exp(A::scale(s.ad_value(10), ((-p.p107) * 1.0 / (p.p17)))), p.p25);
        }

        if (s.v[507] != 0.0) {
            s.store_scale_ad(54, A::exp(A::scale(s.ad_value(10), (-p.p106))), p.p28);
        }

        if (s.v[507] != 0.0) {
            s.store_scale_ad(45, A::exp(A::scale(s.ad_value(10), ((-p.p108) * 1.0 / (p.p19)))), p.p26);
        }

        s.store_mul_ad(43, A::scale(A::exp(A::scale(s.ad_value(280), ((4.0 - p.p103) + p.p121))), p.p29), A::exp(A::scale(s.ad_value(10), (-p.p112))));

        s.store_mul_ad(46, A::scale(A::exp(A::scale(s.ad_value(280), (6.0 - (2.0 * p.p23)))), p.p22), A::exp(A::scale(s.ad_value(10), ((-p.p113) * 1.0 / (p.p23)))));

        s.store_mul_ad(47, A::scale(A::exp(A::scale(s.ad_value(280), (4.0 / p.p150))), p.p149), A::exp(A::scale(s.ad_value(10), ((-p.p113) * 1.0 / (p.p150)))));

        s.store_mul_ad(357, A::scale(A::sqrt(s.ad_value(4)), p.p155), A::exp(A::scale(s.ad_value(12), p.p157)));

        s.store_powf_ad(281, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(282, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p35), s.ad_value(70)), s.ad_value(281)), s.ad_value(282)), p.p66), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_mul_ad(58, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(281), p.p34), s.ad_value(14)), s.ad_value(14)), (s.v[64] * s.v[64])), s.ad_value(73)), A::exp(A::sub_from_scalar(p.p35, s.ad_value(61))));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(283, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(284, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p37), s.ad_value(85)), s.ad_value(283)), s.ad_value(284)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.store_mul_ad(84, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(283), p.p36), s.ad_value(19)), s.ad_value(19)), (s.v[66] * s.v[66])), s.ad_value(90)), A::exp(A::sub_from_scalar(p.p37, s.ad_value(83))));

        s.store_exp_ad(281, A::scale(s.ad_value(280), p.p96));

        s.store_mul_ad_lhs(40, A::scale(s.ad_value(281), p.p14), 27);

        s.store_mul_ad_lhs(41, A::scale(s.ad_value(281), p.p13), 282);

        s.store_mul_ad(107, A::scale(A::exp(A::scale(s.ad_value(280), (4.0 - p.p141))), p.p133), A::exp(A::scale(s.ad_value(10), (-p.p140))));

        s.store_mul_ad(108, A::scale(A::exp(A::scale(s.ad_value(280), (3.5 - (0.5 * p.p142)))), p.p134), A::exp(A::scale(s.ad_value(10), (-p.p140))));

        s.store_scale_ad(109, A::exp(A::scale(s.ad_value(280), (1.0 - p.p141))), p.p135);

        s.store_scale_ad(110, A::exp(A::scale(s.ad_value(280), (1.0 - p.p142))), p.p136);

        s.store_mul_ad(94, A::scale(A::exp(A::scale(s.ad_value(280), (p.p98 - 2.0))), p.p86), A::exp(A::scale(s.ad_value(10), (-p.p120))));

        s.store_scale_ad(95, A::exp(A::scale(s.ad_value(280), ((p.p96 + p.p98) - 1.0))), p.p87);

        s.store_scale_ad(96, A::exp(A::scale(s.ad_value(280), (p.p99 - 1.0))), p.p88);

        s.store_scaled_add(97, 95, 96, (p.p89 * 1.0 / ((p.p87 + p.p88))));

        s.store_scale_ad(98, A::exp(A::scale(s.ad_value(280), (p.p100 - 1.0))), p.p90);

        s.store_offset(101, 2, (-300.0));

        s.v[508] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[508] != 0.0) {
            s.store_mul_ad_rhs(99, 1, A::sub(A::offset(A::scale(s.ad_value(101), 0.00072), 1.0), A::mul(A::scale(s.ad_value(101), 1.6e-6), s.ad_value(101))));
        }

        if (!(s.v[508] != 0.0)) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scale_ad(100, A::exp(A::scale(s.ad_value(280), p.p96)), p.p92);

        s.v[103] = (p.p146 * (((s.v[5] / s.v[3])) as f64).powf(p.p148));

        s.v[509] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[509] != 0.0) {
            s.store_div_from_scalar(111, 1.0, 32);
        }

        s.v[510] = if (s.v[111] > s.v[347]) { 1.0 } else { 0.0 };

        if ((s.v[509] != 0.0) && (s.v[510] != 0.0)) {
            s.copy_ad(111, 347);
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
        if (!(s.v[509] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        s.v[511] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[511] != 0.0) {
            s.store_div_from_scalar(112, 1.0, 33);
        }

        s.v[512] = if (s.v[112] > s.v[347]) { 1.0 } else { 0.0 };

        if ((s.v[511] != 0.0) && (s.v[512] != 0.0)) {
            s.copy_ad(112, 347);
        }

        if (!(s.v[511] != 0.0)) {
            s.store_scalar(112, 0.0);
        }

        s.v[513] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[513] != 0.0) {
            s.store_div_from_scalar(113, 1.0, 34);
        }

        s.v[514] = if (s.v[113] > s.v[347]) { 1.0 } else { 0.0 };

        if ((s.v[513] != 0.0) && (s.v[514] != 0.0)) {
            s.copy_ad(113, 347);
        }

        if (!(s.v[513] != 0.0)) {
            s.store_scalar(113, 0.0);
        }

        s.store_ad(250, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(8)), p.p3));

        s.store_ad(251, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(9)), p.p3));

        s.store_ad(252, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(5)), p.p3));

        s.store_ad(253, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(5)), p.p3));

        s.store_ad(254, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(259, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(8)), p.p3));

        s.store_ad(256, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(9)), p.p3));

        s.store_ad(265, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(5)), p.p3));

        s.store_ad(266, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(6)), p.p3));

        s.store_ad(269, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(270, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(258, &A::scale(A::voltage(ctx, &nodes, Some(11), Some(8)), p.p3));

        s.store_ad(257, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(11)), p.p3));

        s.store_sub_ad_lhs(255, A::sub(A::add(s.ad_value(254), s.ad_value(251)), s.ad_value(256)), 258);

        s.store_sub_ad_lhs(268, A::add(A::sub(s.ad_value(266), s.ad_value(270)), s.ad_value(255)), 257);

        s.store_add(267, 270, 268);

        s.store_sub(261, 259, 258);

        s.store_sub(260, 261, 257);

        s.v[515] = if ((s.v[251] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[515] != 0.0) {
            s.store_exp_ad(271, A::mul(s.ad_value(251), s.ad_value(8)));
        }

        if (!(s.v[515] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[515] != 0.0)) {
            s.store_mul_ad_rhs(271, 301, A::offset(A::offset(A::mul(s.ad_value(251), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[516] = if (((s.v[252] * s.v[8]) / s.v[48]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[516] != 0.0) {
            s.store_exp_ad(272, A::div(A::mul(s.ad_value(252), s.ad_value(8)), s.ad_value(48)));
        }

        if (!(s.v[516] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[516] != 0.0)) {
            s.store_mul_ad_rhs(272, 301, A::offset(A::offset(A::div(A::mul(s.ad_value(252), s.ad_value(8)), s.ad_value(48)), (-p.p151)), 1.0));
        }

        s.v[517] = if ((s.v[255] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[517] != 0.0) {
            s.store_exp_ad(274, A::mul(s.ad_value(255), s.ad_value(8)));
        }

        if (!(s.v[517] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[517] != 0.0)) {
            s.store_mul_ad_rhs(274, 301, A::offset(A::offset(A::mul(s.ad_value(255), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[518] = if ((s.v[254] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[518] != 0.0) {
            s.store_exp_ad(273, A::mul(s.ad_value(254), s.ad_value(8)));
        }

        if (!(s.v[518] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[518] != 0.0)) {
            s.store_mul_ad_rhs(273, 301, A::offset(A::offset(A::mul(s.ad_value(254), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[519] = if ((s.v[267] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[519] != 0.0) {
            s.store_exp_ad(275, A::mul(s.ad_value(267), s.ad_value(8)));
        }

        if (!(s.v[519] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[519] != 0.0)) {
            s.store_mul_ad_rhs(275, 301, A::offset(A::offset(A::mul(s.ad_value(267), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[520] = if ((s.v[259] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[520] != 0.0) {
            s.store_exp_ad(262, A::mul(s.ad_value(259), s.ad_value(8)));
        }

        if (!(s.v[520] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[520] != 0.0)) {
            s.store_mul_ad_rhs(262, 301, A::offset(A::offset(A::mul(s.ad_value(259), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[521] = if ((s.v[260] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[521] != 0.0) {
            s.store_exp_ad(263, A::mul(s.ad_value(260), s.ad_value(8)));
        }

        if (!(s.v[521] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[521] != 0.0)) {
            s.store_mul_ad_rhs(263, 301, A::offset(A::offset(A::mul(s.ad_value(260), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[522] = if ((s.v[261] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[522] != 0.0) {
            s.store_exp_ad(264, A::mul(s.ad_value(261), s.ad_value(8)));
        }

        if (!(s.v[522] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[522] != 0.0)) {
            s.store_mul_ad_rhs(264, 301, A::offset(A::offset(A::mul(s.ad_value(261), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[523] = if (((s.v[267] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[523] != 0.0) {
            s.store_exp_ad(278, A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[523] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[523] != 0.0)) {
            s.store_mul_ad_rhs(278, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[524] = if (((s.v[255] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[524] != 0.0) {
            s.store_exp_ad(276, A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[524] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[524] != 0.0)) {
            s.store_mul_ad_rhs(276, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[525] = if (((s.v[251] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[525] != 0.0) {
            s.store_exp_ad(277, A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[525] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[525] != 0.0)) {
            s.store_mul_ad_rhs(277, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[526] = if (((s.v[250] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[526] != 0.0) {
            s.store_exp_ad(279, A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[526] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[526] != 0.0)) {
            s.store_mul_ad_rhs(279, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.store_sqrt_ad(114, A::offset(A::scale(s.ad_value(277), 4.0), 1.0));

        s.store_sqrt_ad(115, A::offset(A::scale(s.ad_value(279), 4.0), 1.0));

        s.store_div_ad(116, A::scale(s.ad_value(279), 2.0), A::offset(s.ad_value(115), 1.0));

        s.v[527] = if (s.v[116] < p.p153) { 1.0 } else { 0.0 };

        if (s.v[527] != 0.0) {
            s.store_scalar(116, p.p153);
        }

        s.store_mul_ad_rhs(117, 6, A::sub(A::sub(s.ad_value(114), s.ad_value(115)), A::ln(A::div(A::offset(s.ad_value(114), 1.0), A::offset(s.ad_value(115), 1.0)))));

        s.store_div_ad_lhs(118, A::add(s.ad_value(117), s.ad_value(256)), 31);

        s.v[528] = if (s.v[118] > 0.0) { 1.0 } else { 0.0 };

        s.v[529] = if (s.v[250] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[529] != 0.0)) {
            s.copy_ad(303, 250);
        }

        if ((s.v[528] != 0.0) && (!(s.v[529] != 0.0))) {
            s.store_offset_ad(303, A::ln(A::offset(A::offset(s.ad_value(250), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[528] != 0.0) {
            s.store_sub_ad_lhs(119, A::add(s.ad_value(16), A::mul(A::scale(s.ad_value(6), 2.0), A::ln(A::offset(A::mul(A::mul(A::scale(s.ad_value(118), 0.5), s.ad_value(31)), s.ad_value(8)), 1.0)))), 303);
        }

        if (s.v[528] != 0.0) {
            s.store_scale(298, 16, 0.2);
        }

        if (s.v[528] != 0.0) {
            s.store_square(287, 298);
        }

        if (s.v[528] != 0.0) {
            s.store_square(288, 119);
        }

        s.v[530] = if (s.v[119] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[530] != 0.0)) {
            s.store_div_ad(120, A::scale(s.ad_value(287), 0.5), A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(119)));
        }

        if ((s.v[528] != 0.0) && (!(s.v[530] != 0.0))) {
            s.store_scale_ad(120, A::add(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(119)), 0.5);
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(121, A::mul(s.ad_value(120), A::offset(s.ad_value(120), (p.p62 * p.p61))), A::scale(A::add(s.ad_value(120), A::scale(s.ad_value(31), p.p62)), p.p61));
        }

        if (s.v[528] != 0.0) {
            s.store_div(291, 118, 121);
        }

        if (s.v[528] != 0.0) {
            s.store_scaled_offset(285, 291, (-1.0), 1.0 / (p.p63));
        }

        s.v[531] = if (s.v[291] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[531] != 0.0)) {
            s.store_offset_ad(289, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), p.p63), 1.0);
        }

        if ((s.v[528] != 0.0) && (!(s.v[531] != 0.0))) {
            s.store_add_ad_rhs(289, 291, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), p.p63));
        }

        if (s.v[528] != 0.0) {
            s.store_scale(122, 289, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[528] != 0.0) {
            s.store_scale(123, 120, 1.0 / ((p.p62 * p.p61)));
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(124, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(122), 4.0), s.ad_value(123)), A::offset(s.ad_value(123), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(122), 2.0), A::offset(s.ad_value(123), 1.0)));
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(125, A::add(A::sub_from_scalar(1.0, s.ad_value(124)), A::mul(s.ad_value(116), s.ad_value(124))), A::offset(A::mul(s.ad_value(116), s.ad_value(124)), 1.0));
        }

        if (s.v[528] != 0.0) {
            s.store_mul_ad_lhs(127, A::mul(A::mul(A::scale(s.ad_value(118), 0.5), s.ad_value(31)), s.ad_value(125)), 8);
        }

        if (s.v[528] != 0.0) {
            s.store_add_ad(292, A::scale(s.ad_value(127), 2.0), A::mul(s.ad_value(116), A::offset(A::add(s.ad_value(116), s.ad_value(127)), 1.0)));
        }

        if (s.v[528] != 0.0) {
            s.store_scaled_offset(128, 127, (-1.0), 0.5);
        }

        if (s.v[528] != 0.0) {
            s.store_add_ad_lhs(286, A::square(s.ad_value(128)), 292);
        }

        s.v[532] = if (s.v[127] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[532] != 0.0)) {
            s.store_add_ad_rhs(129, 128, A::sqrt(s.ad_value(286)));
        }

        if ((s.v[528] != 0.0) && (!(s.v[532] != 0.0))) {
            s.store_div_ad_rhs(129, 292, A::sub(A::sqrt(s.ad_value(286)), s.ad_value(128)));
        }

        s.v[533] = if (s.v[129] < p.p152) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[533] != 0.0)) {
            s.store_scalar(129, p.p152);
        }

        if (s.v[528] != 0.0) {
            s.store_mul_ad(131, A::mul(s.ad_value(129), A::offset(s.ad_value(129), 1.0)), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
        }

        if (s.v[528] != 0.0) {
            s.store_scaled_offset(133, 118, (-p.p62), (0.5 * p.p61));
        }

        if (s.v[528] != 0.0) {
            s.store_mul_ad_lhs(134, A::scale(s.ad_value(31), (p.p61 * p.p62)), 118);
        }

        if (s.v[528] != 0.0) {
            s.store_add_ad_rhs(135, 133, A::sqrt(A::add(A::square(s.ad_value(133)), s.ad_value(134))));
        }

        s.v[534] = if (p.p73 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[534] != 0.0)) {
            s.store_scale(136, 17, 0.1);
        }

        if ((s.v[528] != 0.0) && (!(s.v[534] != 0.0))) {
            s.store_mul_ad_rhs(136, 17, A::offset(A::div(A::scale(s.ad_value(118), 2.0), A::add(s.ad_value(118), s.ad_value(121))), 0.1));
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(137, A::scale(s.ad_value(118), p.p62), A::offset(s.ad_value(118), p.p62));
        }

        if (s.v[528] != 0.0) {
            s.store_div_from_scalar_ad(213, p.p62, A::offset(s.ad_value(118), p.p62));
        }

        if (!(s.v[528] != 0.0)) {
            s.store_scalar(121, 0.0);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_div_ad(129, A::scale(s.ad_value(277), 2.0), A::offset(s.ad_value(114), 1.0));
        }

        if (!(s.v[528] != 0.0)) {
            s.copy_ad(131, 271);
        }

        s.v[535] = if ((((s.v[256]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[117]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[114] + s.v[115])))) { 1.0 } else { 0.0 };

        if ((!(s.v[528] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_scaled_add(138, 129, 116, 0.5);
        }

        if ((!(s.v[528] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_div_ad_rhs(125, 138, A::offset(s.ad_value(138), 1.0));
        }

        if ((!(s.v[528] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_div_ad_rhs(125, 117, A::sub(A::add(s.ad_value(117), s.ad_value(251)), s.ad_value(250)));
        }

        if (!(s.v[528] != 0.0)) {
            s.copy_ad(135, 256);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_scale(136, 17, 0.1);
        }

        if (!(s.v[528] != 0.0)) {
            s.copy_ad(137, 118);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_sub_from_scalar_ad(213, 1.0, A::scale(s.ad_value(137), 1.0 / (p.p62)));
        }

        s.store_scale(139, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(299, 14, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(252), s.ad_value(139)), 299);

        s.v[536] = if (s.v[252] < s.v[139]) { 1.0 } else { 0.0 };

        if (s.v[536] != 0.0) {
            s.store_sub_ad_rhs(140, 252, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[536] != 0.0)) {
            s.store_sub_ad_rhs(140, 139, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (1.0 - p.p67));

        s.store_add_ad(141, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(252), s.ad_value(140)), 3.0));

        s.v[537] = if (p.p74 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[537] != 0.0) {
            s.copy_ad(142, 250);
        }

        s.v[538] = if (p.p74 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[537] != 0.0)) && (s.v[538] != 0.0)) {
            s.store_add(142, 250, 135);
        }

        if ((!(s.v[537] != 0.0)) && (!(s.v[538] != 0.0))) {
            s.copy_ad(142, 251);
        }

        s.store_div_ad(143, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(144, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(143), ((-1.0) / p.p72))));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(142), s.ad_value(144)), 136);

        s.v[539] = if (s.v[142] < s.v[144]) { 1.0 } else { 0.0 };

        if (s.v[539] != 0.0) {
            s.store_sub_ad_rhs(145, 142, A::mul(s.ad_value(136), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[539] != 0.0)) {
            s.store_sub_ad_rhs(145, 144, A::mul(s.ad_value(136), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_powf(146, 213, p.p76);

        s.store_add_ad(147, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::mul(s.ad_value(146), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(145), s.ad_value(17))), (1.0 - p.p72))))), A::mul(A::mul(s.ad_value(146), s.ad_value(143)), A::sub(s.ad_value(142), s.ad_value(145))));

        s.store_add_ad(148, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(147)), A::mul(s.ad_value(25), s.ad_value(250)));

        s.store_div_ad_lhs(149, A::scale(s.ad_value(35), 4.0), 36);

        s.store_mul(150, 149, 272);

        s.store_div_ad_rhs(152, 150, A::offset(A::sqrt(A::offset(s.ad_value(150), 1.0)), 1.0));

        s.store_ad(132, &A::pow(s.ad_value(131), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(151, 149, 132);

        s.store_div_ad_rhs(153, 151, A::offset(A::sqrt(A::offset(s.ad_value(151), 1.0)), 1.0));

        s.v[540] = if (p.p92 == 0.0) { 1.0 } else { 0.0 };

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
        if (s.v[540] != 0.0) {
            s.store_add_ad(154, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));
        }

        if (!(s.v[540] != 0.0)) {
            s.store_mul_ad_lhs(295, A::mul(A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), s.ad_value(100)), 8);
        }

        if (!(s.v[540] != 0.0)) {
            s.store_mul_ad_lhs(296, A::mul(A::div(A::neg(s.ad_value(148)), s.ad_value(40)), s.ad_value(100)), 8);
        }

        if (!(s.v[540] != 0.0)) {
            s.store_div_ad(154, A::sub(A::exp(s.ad_value(295)), A::exp(s.ad_value(296))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 154);

        s.v[541] = if (s.v[154] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[541] != 0.0) {
            s.store_div_from_scalar_ad(155, (0.5 * s.v[287]), A::sub(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(154)));
        }

        if (!(s.v[541] != 0.0)) {
            s.store_scale_ad(155, A::add(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(154)), 0.5);
        }

        s.store_mul_ad_rhs(156, 155, A::offset(A::scale(A::add(s.ad_value(152), s.ad_value(153)), 0.5), 1.0));

        s.store_mul_ad_lhs(157, A::scale(s.ad_value(35), p.p15), 132);

        s.store_mul(158, 35, 272);

        s.store_div_ad_lhs(159, A::sub(s.ad_value(158), s.ad_value(157)), 156);

        s.store_scale(285, 252, 10000.0);

        s.v[542] = if (s.v[252] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[542] != 0.0) {
            s.store_scale_ad(302, A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 0.0001);
        }

        if (!(s.v[542] != 0.0)) {
            s.store_add_ad_rhs(302, 252, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 0.0001));
        }

        s.store_scale(304, 302, 1.0 / (p.p156));

        s.v[543] = if (s.v[304] < p.p151) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_exp(305, 304);
        }

        if (!(s.v[543] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[543] != 0.0)) {
            s.store_mul_ad_rhs(305, 301, A::offset(A::offset(s.ad_value(304), (-p.p151)), 1.0));
        }

        s.store_mul_ad_rhs(358, 357, A::offset(s.ad_value(305), (-1.0)));

        s.store_scaled_offset(285, 252, (-p.p158), 1000.0);

        s.v[544] = if (s.v[252] < p.p158) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_sub_ad_rhs(306, 252, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 0.001));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_sub_from_scalar_ad(306, p.p158, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 0.001));
        }

        s.store_mul_ad(359, A::scale(s.ad_value(306), p.p159), A::powf(A::sub_from_scalar(p.p158, s.ad_value(306)), 2.0));

        s.v[545] = if (((s.v[252] * s.v[8]) / p.p17) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p17)));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[545] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p17)), (-p.p151)), 1.0));
        }

        s.v[546] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[547] = if (((s.v[252] - s.v[55]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[546] != 0.0) && (s.v[547] != 0.0)) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[546] != 0.0) && (!(s.v[547] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((s.v[546] != 0.0) && (!(s.v[547] != 0.0))) {
            s.store_mul_ad_rhs(304, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[548] = if (((s.v[159] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[546] != 0.0) && (s.v[548] != 0.0)) {
            s.store_exp_ad(305, A::offset(A::div(s.ad_value(159), s.ad_value(35)), (-1000.0)));
        }

        if ((s.v[546] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_scalar(301, ((40.0) as f64).exp());
        }

        if ((s.v[546] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_mul_ad_rhs(305, 301, A::offset(A::offset(A::offset(A::div(s.ad_value(159), s.ad_value(35)), (-1000.0)), (-40.0)), 1.0));
        }

        if (s.v[546] != 0.0) {
            let assign4040_ad_e3794: A = A::add(A::add(A::mul(s.ad_value(42), A::offset(s.ad_value(302), (-1.0))), A::mul(A::div(A::mul(A::scale(s.ad_value(53), 2.0), A::offset(s.ad_value(302), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(304), 4.0), 1.0)), 1.0)), A::offset(A::div(s.ad_value(148), s.ad_value(40)), 1.0))), A::div(A::mul(A::mul(s.ad_value(54), A::offset(s.ad_value(131), (-1.0))), s.ad_value(305)), A::offset(s.ad_value(305), 1.0)));
            s.store_ad(161, &assign4040_ad_e3794);
        }

        s.v[549] = if (p.p93 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[546] != 0.0)) && (s.v[549] != 0.0)) {
            s.store_mul_ad_rhs(161, 42, A::offset(s.ad_value(302), (-1.0)));
        }

        if ((!(s.v[546] != 0.0)) && (!(s.v[549] != 0.0))) {
            s.store_mul_ad_rhs(161, 42, A::add(A::scale(A::offset(s.ad_value(302), (-1.0)), (1.0 - p.p93)), A::mul(A::scale(A::offset(A::add(s.ad_value(302), s.ad_value(131)), (-2.0)), p.p93), A::offset(A::div(s.ad_value(148), s.ad_value(40)), 1.0))));
        }

        s.v[550] = if (((s.v[253] * s.v[8]) / p.p19) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[550] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p19)));
        }

        if (!(s.v[550] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[550] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p19)), (-p.p151)), 1.0));
        }

        s.v[551] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[552] = if (((s.v[253] - s.v[55]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[551] != 0.0) && (s.v[552] != 0.0)) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[551] != 0.0) && (!(s.v[552] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((s.v[551] != 0.0) && (!(s.v[552] != 0.0))) {
            s.store_mul_ad_rhs(304, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        if (s.v[551] != 0.0) {
            s.store_add_ad(162, A::mul(s.ad_value(44), A::offset(s.ad_value(302), (-1.0))), A::div(A::mul(A::scale(s.ad_value(45), 2.0), A::offset(s.ad_value(302), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(304), 4.0), 1.0)), 1.0)));
        }

        if (!(s.v[551] != 0.0)) {
            s.store_mul_ad_rhs(162, 44, A::offset(s.ad_value(302), (-1.0)));
        }

        s.v[553] = if (((s.v[252] * s.v[8]) / p.p21) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[553] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p21)));
        }

        if (!(s.v[553] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[553] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p21)), (-p.p151)), 1.0));
        }

        s.store_mul_ad_rhs(163, 38, A::offset(s.ad_value(302), (-1.0)));

        s.v[554] = if (((s.v[253] * s.v[8]) / p.p23) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[554] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p23)));
        }

        if (!(s.v[554] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[554] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p23)), (-p.p151)), 1.0));
        }

        s.store_mul_ad_rhs(165, 46, A::offset(s.ad_value(302), (-1.0)));

        s.v[555] = if (((s.v[255] * s.v[8]) / p.p32) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(255), s.ad_value(8)), 1.0 / (p.p32)));
        }

        if (!(s.v[555] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[555] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(255), s.ad_value(8)), 1.0 / (p.p32)), (-p.p151)), 1.0));
        }

        s.store_mul_ad_rhs(164, 39, A::offset(s.ad_value(302), (-1.0)));

        s.v[556] = if (((s.v[253] * s.v[8]) / p.p150) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p150)));
        }

        if (!(s.v[556] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[556] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p150)), (-p.p151)), 1.0));
        }

        s.store_mul_ad_rhs(166, 47, A::offset(s.ad_value(302), (-1.0)));

        s.v[557] = if (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[252] < 0.0)) { 1.0 } else { 0.0 };

        s.v[558] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[557] != 0.0) && (s.v[558] != 0.0)) {
            s.store_exp_ad(68, A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))));
        }

        if ((s.v[557] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((s.v[557] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_mul_ad_rhs(68, 301, A::offset(A::offset(A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))), (-p.p151)), 1.0));
        }

        if (s.v[557] != 0.0) {
            s.store_mul(281, 252, 65);
        }

        if (s.v[557] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(281)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p67 * p.p67)), A::scale(s.ad_value(281), (3.0 * (p.p67 - 1.0)))), p.p67), A::mul(A::mul(A::scale(s.ad_value(281), 6.0), s.ad_value(281)), A::offset(s.ad_value(281), (p.p67 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[557] != 0.0) {
            s.store_div_ad(281, A::mul(A::scale(s.ad_value(252), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[559] = if (s.v[281] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[560] = if (s.v[281] < p.p151) { 1.0 } else { 0.0 };

        if (((s.v[557] != 0.0) && (s.v[559] != 0.0)) && (s.v[560] != 0.0)) {
            s.store_exp(91, 281);
        }

        if (((s.v[557] != 0.0) && (s.v[559] != 0.0)) && (!(s.v[560] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (((s.v[557] != 0.0) && (s.v[559] != 0.0)) && (!(s.v[560] != 0.0))) {
            s.store_mul_ad_rhs(91, 301, A::offset(A::offset(s.ad_value(281), (-p.p151)), 1.0));
        }

        if ((s.v[557] != 0.0) && (s.v[559] != 0.0)) {
            s.store_mul_ad(69, A::neg(s.ad_value(252)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(281)), 1.0));
        }

        if ((s.v[557] != 0.0) && (!(s.v[559] != 0.0))) {
            s.store_mul_ad(69, A::mul(A::scale(s.ad_value(252), 0.5), s.ad_value(281)), A::offset(A::mul(A::scale(s.ad_value(281), 0.3333333333333333), A::offset(A::scale(s.ad_value(281), 0.25), 1.0)), 1.0));
        }

        if (s.v[557] != 0.0) {
            s.store_scale_ad(57, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(58), 2.0), s.ad_value(69)), s.ad_value(59)), s.ad_value(68)), s.ad_value(65)), s.v[63]);
        }

        if (!(s.v[557] != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        if (!(s.v[557] != 0.0)) {
            s.store_scalar(57, 0.0);
        }

        s.v[561] = if (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[250] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(250), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[562] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[561] != 0.0) && (s.v[562] != 0.0)) {
            s.store_exp_ad(78, A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))));
        }

        if ((s.v[561] != 0.0) && (!(s.v[562] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((s.v[561] != 0.0) && (!(s.v[562] != 0.0))) {
            s.store_mul_ad_rhs(78, 301, A::offset(A::offset(A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))), (-p.p151)), 1.0));
        }

        if (s.v[561] != 0.0) {
            s.store_mul(283, 250, 67);
        }

        if (s.v[561] != 0.0) {
            let assign4640_ad_e4484: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(283)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(283), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(283), 6.0), s.ad_value(283)), A::offset(s.ad_value(283), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4640_ad_e4484);
        }

        if (s.v[561] != 0.0) {
            s.store_div_ad(283, A::mul(A::scale(s.ad_value(250), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[563] = if (s.v[283] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[564] = if (s.v[283] < p.p151) { 1.0 } else { 0.0 };

        if (((s.v[561] != 0.0) && (s.v[563] != 0.0)) && (s.v[564] != 0.0)) {
            s.store_exp(92, 283);
        }

        if (((s.v[561] != 0.0) && (s.v[563] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (((s.v[561] != 0.0) && (s.v[563] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_mul_ad_rhs(92, 301, A::offset(A::offset(s.ad_value(283), (-p.p151)), 1.0));
        }

        if ((s.v[561] != 0.0) && (s.v[563] != 0.0)) {
            s.store_mul_ad(81, A::neg(s.ad_value(250)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(283)), 1.0));
        }

        if ((s.v[561] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_mul_ad(81, A::mul(A::scale(s.ad_value(250), 0.5), s.ad_value(283)), A::offset(A::mul(A::scale(s.ad_value(283), 0.3333333333333333), A::offset(A::scale(s.ad_value(283), 0.25), 1.0)), 1.0));
        }

        if (s.v[561] != 0.0) {
            s.store_scale_ad(82, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(84), 2.0), s.ad_value(81)), s.ad_value(77)), s.ad_value(78)), s.ad_value(67)), s.v[89]);
        }

        if (!(s.v[561] != 0.0)) {
            s.store_scalar(81, 0.0);
        }

        if (!(s.v[561] != 0.0)) {
            s.store_scalar(82, 0.0);
        }

        s.store_mul(168, 149, 274);

        s.store_scale(169, 276, 4.0);

        s.store_div_ad(171, A::sub(s.ad_value(168), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(168), 1.0)), 1.0));

        s.store_div_ad_rhs(170, 169, A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0));

        s.store_div_ad(167, A::mul(A::scale(s.ad_value(43), 2.0), A::offset(s.ad_value(274), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(43), 4.0), s.ad_value(37)), s.ad_value(274)), 1.0)), 1.0));

        s.v[565] = if (p.p8 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[565] != 0.0) {
            s.store_div_ad(185, A::mul(A::scale(s.ad_value(107), (p.p143 * 2.0)), A::sub(s.ad_value(271), s.ad_value(262))), A::offset(A::sqrt(A::offset(A::mul(A::scale(A::div(s.ad_value(107), s.ad_value(109)), 4.0), A::add(s.ad_value(271), A::scale(s.ad_value(262), p.p144))), 1.0)), 1.0));
        }

        if (s.v[565] != 0.0) {
            s.store_div_ad(182, A::mul(A::scale(s.ad_value(107), ((1.0 - p.p143) * 2.0)), A::sub(s.ad_value(274), s.ad_value(264))), A::offset(A::sqrt(A::offset(A::mul(A::scale(A::div(s.ad_value(107), s.ad_value(109)), 4.0), A::add(s.ad_value(274), A::scale(s.ad_value(264), p.p144))), 1.0)), 1.0));
        }

        if (!(s.v[565] != 0.0)) {
            s.store_div_ad(185, A::mul(A::scale(s.ad_value(107), (p.p143 * 2.0)), A::offset(s.ad_value(271), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::scale(A::div(s.ad_value(107), s.ad_value(109)), 4.0), s.ad_value(271)), 1.0)), 1.0));
        }

        if (!(s.v[565] != 0.0)) {
            s.store_div_ad(182, A::mul(A::scale(s.ad_value(107), ((1.0 - p.p143) * 2.0)), A::offset(s.ad_value(274), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::scale(A::div(s.ad_value(107), s.ad_value(109)), 4.0), s.ad_value(274)), 1.0)), 1.0));
        }

        s.store_add_ad(184, A::div(A::mul(A::scale(s.ad_value(108), 2.0), A::offset(s.ad_value(262), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::scale(A::div(s.ad_value(108), s.ad_value(110)), (p.p144 * 4.0)), s.ad_value(262)), 1.0)), 1.0)), A::mul(s.ad_value(259), s.ad_value(348)));

        s.v[183] = 0.0;

        s.v[566] = if ((p.p5 > 0.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[566] != 0.0) {
            s.store_scale(167, 167, s.v[160]);
        }

        if (s.v[566] != 0.0) {
            s.store_scale(182, 182, s.v[160]);
        }

        if (s.v[566] != 0.0) {
            s.store_div_ad(174, A::mul(A::scale(s.ad_value(43), (p.p33 * 2.0)), A::offset(s.ad_value(275), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(43), 4.0), s.ad_value(37)), s.ad_value(275)), 1.0)), 1.0));
        }

        s.v[567] = if (p.p8 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[566] != 0.0) && (s.v[567] != 0.0)) {
            s.store_div_ad(175, A::mul(A::scale(s.ad_value(107), (((1.0 - p.p143) * p.p33) * 2.0)), A::sub(s.ad_value(275), s.ad_value(263))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(107), 4.0), s.ad_value(109)), A::add(s.ad_value(275), A::scale(s.ad_value(263), p.p144))), 1.0)), 1.0));
        }

        if ((s.v[566] != 0.0) && (!(s.v[567] != 0.0))) {
            s.store_div_ad(175, A::mul(A::scale(s.ad_value(107), (((1.0 - p.p143) * p.p33) * 2.0)), A::offset(s.ad_value(275), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(107), 4.0), s.ad_value(109)), s.ad_value(275)), 1.0)), 1.0));
        }

        s.v[568] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_mul_ad_lhs(297, A::scale(A::add(s.ad_value(43), s.ad_value(107)), p.p33), 32);
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_mul_ad_rhs(176, 6, A::sub_from_scalar(2.0, A::ln(A::mul(s.ad_value(297), s.ad_value(8)))));
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_sub(290, 267, 176);
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_scalar(287, (0.11 * 0.11));
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_square(288, 290);
        }

        s.v[569] = if (s.v[290] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[566] != 0.0) && (s.v[568] != 0.0)) && (s.v[569] != 0.0)) {
            s.store_div_ad(177, A::scale(s.ad_value(287), 0.5), A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(290)));
        }

        if (((s.v[566] != 0.0) && (s.v[568] != 0.0)) && (!(s.v[569] != 0.0))) {
            s.store_scale_ad(177, A::add(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(290)), 0.5);
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_div_ad_rhs(178, 177, A::add(A::add(s.ad_value(297), A::mul(A::add(s.ad_value(174), s.ad_value(175)), s.ad_value(32))), s.ad_value(177)));
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(176, 0.0);
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(290, 0.0);
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(177, 0.0);
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(178, 1.0);
        }

        if (s.v[566] != 0.0) {
            s.store_mul(179, 178, 174);
        }

        if (s.v[566] != 0.0) {
            s.store_mul(183, 178, 175);
        }

        s.v[570] = if (p.p84 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[570] != 0.0) {
            s.store_add(353, 254, 250);
        }

        if (s.v[570] != 0.0) {
            s.store_scalar(287, (1e-6 * 1e-6));
        }

        if (s.v[570] != 0.0) {
            s.store_mul_ad_lhs(288, A::scale(s.ad_value(353), ((-1.0) * (-1.0))), 353);
        }

        s.v[571] = if (((-1.0) * s.v[353]) < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[570] != 0.0) && (s.v[571] != 0.0)) {
            s.store_div_ad(354, A::scale(s.ad_value(287), 0.5), A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), A::scale(s.ad_value(353), (-1.0))));
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
        if ((s.v[570] != 0.0) && (!(s.v[571] != 0.0))) {
            s.store_scale_ad(354, A::add(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), A::scale(s.ad_value(353), (-1.0))), 0.5);
        }

        if (s.v[570] != 0.0) {
            s.store_scalar(355, (1.0 / (1.0 - ((s.v[349]) as f64).powf(p.p82))));
        }

        if (s.v[570] != 0.0) {
            s.store_scalar(350, (s.v[349] * p.p81));
        }

        if (s.v[570] != 0.0) {
            s.store_scale_ad(352, A::square(s.ad_value(355)), (((s.v[349]) as f64).powf((p.p82 - 1.0)) * (p.p82 * 1.0 / (p.p81))));
        }

        s.v[572] = if (s.v[354] < s.v[350]) { 1.0 } else { 0.0 };

        if ((s.v[570] != 0.0) && (s.v[572] != 0.0)) {
            s.store_div_from_scalar_ad(351, 1.0, A::sub_from_scalar(1.0, A::powf(A::scale(s.ad_value(354), 1.0 / (p.p81)), p.p82)));
        }

        if ((s.v[570] != 0.0) && (!(s.v[572] != 0.0))) {
            s.store_add_ad_rhs(351, 355, A::mul(A::sub(s.ad_value(354), s.ad_value(350)), s.ad_value(352)));
        }

        if (!(s.v[570] != 0.0)) {
            s.store_scalar(351, 1.0);
        }

        s.store_mul(82, 82, 351);

        s.store_mul(167, 167, 351);

        s.store_mul(164, 164, 351);

        s.store_mul(179, 179, 351);

        s.store_add_ad(186, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 186);

        s.v[573] = if (s.v[186] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[573] != 0.0) {
            s.store_div_from_scalar_ad(187, (0.5 * s.v[287]), A::sub(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(186)));
        }

        if (!(s.v[573] != 0.0)) {
            s.store_scale_ad(187, A::add(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(186)), 0.5);
        }

        s.store_mul_ad_rhs(188, 187, A::offset(A::scale(A::add(s.ad_value(152), s.ad_value(153)), 0.5), 1.0));

        s.store_div(190, 29, 188);

        s.v[574] = if (s.v[190] < s.v[346]) { 1.0 } else { 0.0 };

        if (s.v[574] != 0.0) {
            s.copy_ad(190, 346);
        }

        s.store_scale(189, 190, 3.0);

        s.store_div_ad_lhs(191, A::add(A::mul(A::scale(s.ad_value(6), 2.0), A::offset(s.ad_value(273), (-1.0))), s.ad_value(254)), 189);

        s.v[575] = if (s.v[159] > 0.0) { 1.0 } else { 0.0 };

        s.v[576] = if (p.p39 == 1.0) { 1.0 } else { 0.0 };

        s.v[577] = if (s.v[250] < p.p44) { 1.0 } else { 0.0 };

        s.v[578] = if (((-s.v[159]) / p.p42) < p.p151) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (s.v[578] != 0.0)) {
            s.store_exp_ad(338, A::scale(A::neg(s.ad_value(159)), 1.0 / (p.p42)));
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[578] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[578] != 0.0))) {
            s.store_mul_ad_rhs(338, 301, A::offset(A::offset(A::scale(A::neg(s.ad_value(159)), 1.0 / (p.p42)), (-p.p151)), 1.0));
        }

        if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
            s.store_mul_ad_lhs(339, A::sub_from_scalar(p.p44, s.ad_value(250)), 338);
        }

        s.v[579] = if (((-s.v[340]) * ((s.v[339]) as f64).powf(p.p41)) < p.p151) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (s.v[579] != 0.0)) {
            s.store_exp_ad(343, A::mul(A::neg(s.ad_value(340)), A::powf(s.ad_value(339), p.p41)));
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_mul_ad_rhs(343, 301, A::offset(A::offset(A::mul(A::neg(s.ad_value(340)), A::powf(s.ad_value(339), p.p41)), (-p.p151)), 1.0));
        }

        if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(A::div_from_scalar(p.p40, s.ad_value(340)), s.ad_value(339)), 343);
        }

        s.v[580] = if (p.p39 == 2.0) { 1.0 } else { 0.0 };

        s.v[581] = if (s.v[250] < s.v[16]) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_scalar(199, ((2.0 * p.p46) / (p.p45 * p.p45)));
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad_lhs(286, A::sub(s.ad_value(16), s.ad_value(250)), 213);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_sqrt_ad(200, A::div(A::scale(s.ad_value(286), 2.0), s.ad_value(199)));
        }

        s.v[582] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
            s.store_scalar(201, p.p45);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
            s.store_sub_from_scalar_ad(126, 1.0, A::scale(s.ad_value(125), 0.5));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
            s.store_mul_ad_lhs(201, A::scale(s.ad_value(126), p.p45), 126);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad(202, A::mul(s.ad_value(200), s.ad_value(201)), A::sqrt(A::add(A::square(s.ad_value(200)), A::square(s.ad_value(201)))));
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad_lhs(203, A::sub(s.ad_value(16), s.ad_value(250)), 202);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_add_ad_rhs(204, 203, A::mul(A::mul(A::scale(s.ad_value(202), 0.5), s.ad_value(199)), s.ad_value(213)));
        }

        s.v[583] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[583] != 0.0)) {
            s.copy_ad(205, 204);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_offset_ad(206, A::scale(A::offset(A::scale(s.ad_value(125), 2.0), 1.0), (2.0 * p.p47)), 1.0);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_scalar(207, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_sub_ad_rhs(208, 203, A::mul(A::mul(A::scale(s.ad_value(202), 0.5), s.ad_value(199)), A::sub(s.ad_value(207), A::div(s.ad_value(159), A::scale(s.ad_value(206), p.p62)))));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_add_ad(286, A::mul(A::sub(s.ad_value(208), s.ad_value(204)), A::sub(s.ad_value(208), s.ad_value(204))), A::scale(A::mul(A::mul(A::scale(s.ad_value(203), 0.1), s.ad_value(203)), s.ad_value(137)), 1.0 / (p.p62)));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_scale_ad(205, A::add(A::add(s.ad_value(208), s.ad_value(204)), A::sqrt(s.ad_value(286))), 0.5);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad_lhs(293, A::sub(s.ad_value(205), s.ad_value(203)), 205);
        }

        s.v[584] = if (((s.v[293]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[584] != 0.0)) {
            s.store_div_ad_lhs(209, A::scale(s.ad_value(202), 0.5), 293);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[584] != 0.0)) {
            s.store_mul_ad(210, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(205)), s.ad_value(209)), A::sub(A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(205))), A::exp(A::mul(A::div(A::neg(s.ad_value(99)), s.ad_value(205)), A::offset(A::div(s.ad_value(201), s.ad_value(209)), 1.0)))));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[584] != 0.0))) {
            s.store_mul_ad(210, A::mul(s.ad_value(0), s.ad_value(201)), A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(205))));
        }

        s.v[585] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        s.v[586] = if (s.v[250] < p.p44) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) {
            s.store_mul_ad(214, A::powf(A::sub_from_scalar(p.p44, s.ad_value(250)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(159), A::offset(s.ad_value(159), p.p48))), p.p49));
        }

        s.v[587] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (s.v[587] != 0.0)) {
            s.copy_ad(215, 214);
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
            s.store_scaled_offset(216, 159, (-p.p52), 1.0 / (p.p48));
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
            s.store_scaled_offset(285, 216, (-1.0), 1.0 / (p.p51));
        }

        s.v[588] = if (s.v[216] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) && (s.v[588] != 0.0)) {
            s.store_offset_ad(217, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), p.p51), 1.0);
        }

        if (((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) && (!(s.v[588] != 0.0))) {
            s.store_add_ad_rhs(217, 216, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), p.p51));
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
            s.store_mul_ad_rhs(215, 214, A::powf(s.ad_value(217), p.p50));
        }

        s.v[589] = if (((-s.v[340]) * s.v[215]) < p.p151) { 1.0 } else { 0.0 };

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (s.v[589] != 0.0)) {
            s.store_exp_ad(343, A::mul(A::neg(s.ad_value(340)), s.ad_value(215)));
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[589] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[589] != 0.0))) {
            s.store_mul_ad_rhs(343, 301, A::offset(A::offset(A::mul(A::neg(s.ad_value(340)), s.ad_value(215)), (-p.p151)), 1.0));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(A::div_from_scalar(p.p40, s.ad_value(340)), A::sub_from_scalar(p.p44, s.ad_value(250))), 343);
        }

        s.v[590] = if (s.v[210] > 0.0) { 1.0 } else { 0.0 };

        s.v[591] = if (p.p53 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) {
            s.store_add_ad(211, A::add(A::div(s.ad_value(6), A::mul(s.ad_value(159), A::add(s.ad_value(30), s.ad_value(189)))), A::mul(A::div(s.ad_value(156), s.ad_value(35)), s.ad_value(42))), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(189))));
        }

        s.v[592] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) {
            s.store_scaled_sub(285, 210, 211, 1000000.0);
        }

        s.v[593] = if (s.v[210] < s.v[211]) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) && (s.v[593] != 0.0)) {
            s.store_sub_ad_rhs(210, 210, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 1e-6));
        }

        if (((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) && (!(s.v[593] != 0.0))) {
            s.store_sub_ad_rhs(210, 211, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 1e-6));
        }

        if ((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) {
            s.store_mul(212, 159, 210);
        }

        if ((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (!(s.v[592] != 0.0))) {
            s.store_div_ad(212, A::mul(A::mul(s.ad_value(159), s.ad_value(210)), s.ad_value(211)), A::add(s.ad_value(210), s.ad_value(211)));
        }

        if (((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (!(s.v[591] != 0.0))) {
            s.store_mul(212, 159, 210);
        }

        s.v[594] = if (s.v[131] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[594] != 0.0) {
            s.store_mul_ad_rhs(130, 6, A::ln(s.ad_value(131)));
        }

        if (!(s.v[594] != 0.0)) {
            s.copy_ad(130, 251);
        }

        s.v[595] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[595] != 0.0) {
            s.copy_ad(93, 250);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(93, 251);
        }

        let assign6110_ad_e6278: A = A::add(A::add(A::add(A::sub(A::add(A::mul(s.ad_value(159), A::sub(s.ad_value(252), s.ad_value(130))), A::mul(s.ad_value(118), A::sub(s.ad_value(130), s.ad_value(250)))), A::mul(s.ad_value(212), s.ad_value(130))), A::div(A::square(s.ad_value(265)), s.ad_value(28))), A::mul(A::square(s.ad_value(268)), s.ad_value(111))), A::mul(A::square(s.ad_value(257)), s.ad_value(112)));
        let assign6110_ad_e6310: A = A::add(A::add(A::add(A::add(assign6110_ad_e6278, A::mul(A::square(s.ad_value(258)), s.ad_value(113))), A::div(A::square(s.ad_value(266)), s.ad_value(30))), A::mul(s.ad_value(191), s.ad_value(254))), A::mul(A::add(A::add(A::sub(A::add(A::add(s.ad_value(161), s.ad_value(163)), A::scale(s.ad_value(252), s.v[344])), s.ad_value(57)), s.ad_value(359)), s.ad_value(358)), s.ad_value(252)));
        let assign6110_ad_e6342: A = A::add(A::add(A::add(A::add(A::sub(assign6110_ad_e6310, A::mul(s.ad_value(82), s.ad_value(93))), A::mul(A::add(A::add(s.ad_value(162), s.ad_value(165)), s.ad_value(166)), s.ad_value(253))), A::mul(A::add(A::add(s.ad_value(167), s.ad_value(164)), A::scale(s.ad_value(255), s.v[344])), s.ad_value(255))), A::mul(s.ad_value(179), s.ad_value(267))), A::mul(s.ad_value(182), A::sub(s.ad_value(255), s.ad_value(261))));
        s.store_add_ad(219, A::add(A::add(assign6110_ad_e6342, A::mul(s.ad_value(185), A::sub(s.ad_value(250), s.ad_value(259)))), A::mul(s.ad_value(183), A::sub(s.ad_value(267), s.ad_value(260)))), A::mul(s.ad_value(184), s.ad_value(259)));

        s.store_mul_ad_lhs(221, A::scale(s.ad_value(23), (1.0 - p.p68)), 141);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(253), s.ad_value(139)), 299);

        s.v[596] = if (s.v[253] < s.v[139]) { 1.0 } else { 0.0 };

        if (s.v[596] != 0.0) {
            s.store_sub_ad_rhs(222, 253, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[596] != 0.0)) {
            s.store_sub_ad_rhs(222, 139, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_mul_ad(223, A::scale(s.ad_value(23), p.p68), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(65))), (1.0 - p.p67)))), A::scale(A::sub(s.ad_value(253), s.ad_value(222)), 3.0)));

        s.store_mul_ad_lhs(224, A::scale(s.ad_value(24), p.p77), 148);

        s.store_mul(225, 95, 36);

        s.store_mul_ad_lhs(229, A::mul(A::scale(s.ad_value(225), 0.5), s.ad_value(152)), 187);

        s.store_mul_ad_lhs(230, A::mul(A::scale(s.ad_value(225), 0.5), s.ad_value(153)), 187);

        s.store_scale(300, 17, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(255), s.ad_value(144)), 300);

        s.v[597] = if (s.v[255] < s.v[144]) { 1.0 } else { 0.0 };

        if (s.v[597] != 0.0) {
            s.store_sub_ad_rhs(231, 255, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[597] != 0.0)) {
            s.store_sub_ad_rhs(231, 144, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_add_ad(232, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(231), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(143), A::sub(s.ad_value(255), s.ad_value(231))));

        s.store_scale_ad(233, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(232)), A::mul(s.ad_value(25), s.ad_value(255)))), ((1.0 - p.p77) * (1.0 - p.p33)));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(267), s.ad_value(144)), 300);

        s.v[598] = if (s.v[267] < s.v[144]) { 1.0 } else { 0.0 };

        if (s.v[598] != 0.0) {
            s.store_sub_ad_rhs(234, 267, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[598] != 0.0)) {
            s.store_sub_ad_rhs(234, 144, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_add_ad(235, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(234), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(143), A::sub(s.ad_value(267), s.ad_value(234))));

        s.store_scale_ad(236, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(235)), A::mul(s.ad_value(25), s.ad_value(267)))), ((1.0 - p.p77) * p.p33));

        s.store_scale(307, 105, 0.1);

        s.store_scale(237, 105, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(259), s.ad_value(237)), 307);

        s.v[599] = if (s.v[259] < s.v[237]) { 1.0 } else { 0.0 };

        if (s.v[599] != 0.0) {
            s.store_sub_ad_rhs(238, 259, A::mul(s.ad_value(307), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[599] != 0.0)) {
            s.store_sub_ad_rhs(238, 237, A::mul(s.ad_value(307), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_mul_ad_rhs(239, 106, A::add(A::mul(A::scale(s.ad_value(105), 1.0 / ((1.0 - p.p139))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(238), s.ad_value(105))), (1.0 - p.p139)))), A::scale(A::sub(s.ad_value(259), s.ad_value(238)), 2.0)));

        s.store_mul_ad(240, A::mul(s.ad_value(94), s.ad_value(36)), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p85)));

        s.v[600] = if ((s.v[252] / (p.p85 * s.v[6])) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[600] != 0.0) {
            s.store_exp_ad(302, A::div(s.ad_value(252), A::scale(s.ad_value(6), p.p85)));
        }

        if (!(s.v[600] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[600] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::div(s.ad_value(252), A::scale(s.ad_value(6), p.p85)), (-p.p151)), 1.0));
        }

        s.store_mul(242, 240, 302);

        s.store_div_ad_lhs(243, A::mul(A::scale(s.ad_value(96), 4.0), s.ad_value(6)), 31);

        s.store_mul_ad(244, A::mul(A::scale(s.ad_value(243), 0.5), s.ad_value(125)), A::offset(A::add(s.ad_value(129), s.ad_value(116)), 2.0));

        s.v[601] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[601] != 0.0) {
            s.store_div_ad(249, A::mul(A::scale(s.ad_value(97), 0.5), A::add(A::mul(s.ad_value(225), s.ad_value(171)), A::mul(s.ad_value(243), s.ad_value(170)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[602] = if ((((s.v[255] - s.v[22]) / p.p91) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if ((!(s.v[601] != 0.0)) && (s.v[602] != 0.0)) {
            s.store_exp_ad(180, A::mul(A::scale(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91)), s.ad_value(8)));
        }

        if ((!(s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((!(s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
            s.store_mul_ad_rhs(180, 301, A::offset(A::offset(A::mul(A::scale(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        if (!(s.v[601] != 0.0)) {
            s.store_div_ad(249, A::mul(A::mul(A::scale(s.ad_value(43), 2.0), s.ad_value(98)), s.ad_value(274)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(180), 4.0), 1.0)), 1.0));
        }

        s.v[603] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[603] != 0.0) {
            s.store_scale(249, 249, s.v[160]);
        }

        s.v[604] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_mul(172, 149, 275);
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_div_ad(173, A::sub(s.ad_value(172), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(172), 1.0)), 1.0));
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_scale(245, 278, 4.0);
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_div_ad_rhs(246, 245, A::offset(A::sqrt(A::offset(s.ad_value(245), 1.0)), 1.0));
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_div_ad(247, A::mul(A::scale(s.ad_value(97), (0.5 * p.p33)), A::add(A::mul(s.ad_value(225), s.ad_value(173)), A::mul(s.ad_value(243), s.ad_value(246)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[605] = if (((s.v[267] - s.v[22]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) && (s.v[605] != 0.0)) {
            s.store_exp_ad(181, A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)));
        }

        if (((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) && (!(s.v[605] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) && (!(s.v[605] != 0.0))) {
            s.store_mul_ad_rhs(181, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        if ((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) {
            s.store_div_ad(247, A::mul(A::mul(A::scale(s.ad_value(43), (2.0 * p.p33)), s.ad_value(98)), s.ad_value(275)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(181), 4.0), 1.0)), 1.0));
        }

        if (s.v[603] != 0.0) {
            s.store_mul(248, 178, 247);
        }

        s.v[606] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[606] != 0.0) {
            s.store_offset_ad(193, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (-p.p67)), (-3.0));
        }

        if (s.v[606] != 0.0) {
            s.store_div_ad_lhs(294, A::sub(s.ad_value(252), s.ad_value(139)), 299);
        }

        s.v[607] = if (s.v[294] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[606] != 0.0) && (s.v[607] != 0.0)) {
            s.store_div_from_scalar_ad(194, 1.0, A::offset(A::exp(s.ad_value(294)), 1.0));
        }

        if ((s.v[606] != 0.0) && (!(s.v[607] != 0.0))) {
            s.store_div_ad(194, A::exp(A::neg(s.ad_value(294))), A::offset(A::exp(A::neg(s.ad_value(294))), 1.0));
        }

        if (s.v[606] != 0.0) {
            s.store_offset_ad(192, A::mul(s.ad_value(193), s.ad_value(194)), 3.0);
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad_lhs(195, A::scale(s.ad_value(23), (1.0 - p.p68)), 192);
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
        if (s.v[606] != 0.0) {
            s.store_mul_ad(198, A::div(A::mul(A::mul(s.ad_value(149), s.ad_value(272)), s.ad_value(8)), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(150), 1.0))));
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad_lhs(196, A::mul(A::scale(s.ad_value(225), 0.5), s.ad_value(187)), 198);
        }

        if (s.v[606] != 0.0) {
            s.store_div_ad_rhs(197, 242, A::scale(s.ad_value(6), p.p85));
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad(228, A::scale(s.ad_value(254), 0.2), A::add(A::add(s.ad_value(195), s.ad_value(196)), s.ad_value(197)));
        }

        if (s.v[606] != 0.0) {
            s.store_scale(241, 242, (1.0 - p.p95));
        }

        if (s.v[606] != 0.0) {
            s.store_add_ad_rhs(337, 229, A::scale(s.ad_value(242), p.p95));
        }

        if (s.v[606] != 0.0) {
            s.store_add_ad_lhs(227, A::scale(s.ad_value(337), p.p94), 230);
        }

        if (s.v[606] != 0.0) {
            s.store_scale(226, 337, (1.0 - p.p94));
        }

        if (!(s.v[606] != 0.0)) {
            s.copy_ad(226, 229);
        }

        if (!(s.v[606] != 0.0)) {
            s.copy_ad(227, 230);
        }

        if (!(s.v[606] != 0.0)) {
            s.copy_ad(241, 242);
        }

        s.v[608] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        let assign6910_ad_e7175: A = A::ddt(A::scale(A::voltage(ctx, &nodes, Some(4), None), p.p147), self.ddt_jacobian(1.0), self.eval_ddt(0, A::scale(A::voltage(ctx, &nodes, Some(4), None), p.p147).value));
        s.store_scale_ad(220, assign6910_ad_e7175, p.p1);

        s.v[356] = (1.0 - p.p148);

        s.v[609] = if (p.p146 > s.v[346]) { 1.0 } else { 0.0 };

        s.v[610] = if (p.p145 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[609] != 0.0) && (s.v[610] != 0.0)) {
            s.store_scale_ad(102, A::scale(A::voltage(ctx, &nodes, Some(4), None), 1.0 / (s.v[103])), p.p1);
        }

        s.v[611] = if (((s.v[356]) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((s.v[609] != 0.0) && (!(s.v[610] != 0.0))) && (s.v[611] != 0.0)) {
            s.store_scale_ad(102, A::ln(A::offset(A::scale(A::voltage(ctx, &nodes, Some(4), None), 1.0 / (s.v[5])), 1.0)), ((s.v[5] / s.v[103]) * p.p1));
        }

        if (((s.v[609] != 0.0) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_scale_ad(102, A::offset(A::powf(A::offset(A::scale(A::voltage(ctx, &nodes, Some(4), None), 1.0 / (s.v[5])), 1.0), s.v[356]), (-1.0)), ((s.v[5] / (s.v[356] * s.v[103])) * p.p1));
        }

        if (!(s.v[609] != 0.0)) {
            s.store_div_ad_lhs(102, A::voltage(ctx, &nodes, Some(4), None), 345);
        }

        s.v[612] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.v[613] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

        s.store_scale(308, 2, (4.0 * 1.3806226e-23));

        s.store_div(309, 308, 28);

        s.store_div(310, 308, 30);

        s.store_mul(311, 308, 111);

        s.store_mul(312, 308, 112);

        s.store_mul(313, 308, 113);

        s.store_scale_ad(314, A::mul(A::div(s.ad_value(308), s.ad_value(189)), A::offset(A::scale(s.ad_value(273), 4.0), 5.0)), 0.3333333333333333);

        s.store_div_ad_lhs(333, A::add(s.ad_value(158), s.ad_value(157)), 156);

        s.store_scale_ad(315, A::abs(s.ad_value(333)), (2.0 * 1.6021918e-19));

        s.v[614] = if (p.p130 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[614] != 0.0) {
            s.store_abs_ad(334, A::div(s.ad_value(212), s.ad_value(333)));
        }

        if (!(s.v[614] != 0.0)) {
            s.store_scalar(334, 0.0);
        }

        s.store_mul_ad(327, A::scale(s.ad_value(212), (2.0 * 1.6021918e-19)), A::offset(s.ad_value(334), 1.0));

        s.v[615] = if (s.v[333] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[615] != 0.0) {
            s.store_div_ad_lhs(335, A::add(s.ad_value(226), s.ad_value(227)), 333);
        }

        if (!(s.v[615] != 0.0)) {
            s.store_mul_ad_lhs(335, A::mul(s.ad_value(95), s.ad_value(187)), 156);
        }

        s.v[616] = if (p.p131 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[616] != 0.0) {
            s.store_scale(336, 335, p.p94);
        }

        s.v[617] = if (p.p131 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[616] != 0.0)) && (s.v[617] != 0.0)) {
            s.store_scale(336, 335, p.p132);
        }

        if ((!(s.v[616] != 0.0)) && (!(s.v[617] != 0.0))) {
            s.store_scalar(336, 0.0);
        }

        s.store_scale_ad(316, A::abs(A::add(A::add(A::sub(A::add(s.ad_value(161), s.ad_value(163)), s.ad_value(57)), s.ad_value(359)), s.ad_value(358))), (2.0 * 1.6021918e-19));

        s.store_add(328, 161, 162);

        s.store_scale_ad(317, A::powf(A::abs(s.ad_value(328)), p.p126), p.p128);

        s.v[618] = if (s.v[328] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[618] != 0.0) {
            s.store_neg(317, 317);
        }

        s.store_add_ad_lhs(329, A::add(s.ad_value(163), s.ad_value(165)), 166);

        s.store_scale_ad(318, A::powf(A::abs(s.ad_value(329)), p.p127), p.p129);

        s.v[619] = if (s.v[329] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[619] != 0.0) {
            s.store_neg(318, 318);
        }

        s.store_scale_ad(319, A::abs(A::add(A::add(s.ad_value(162), s.ad_value(165)), s.ad_value(166))), (2.0 * 1.6021918e-19));

        s.store_scale_ad(320, A::abs(s.ad_value(164)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(321, A::powf(A::abs(s.ad_value(164)), p.p126), p.p128);

        s.v[620] = if (s.v[164] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[620] != 0.0) {
            s.store_neg(321, 321);
        }

        s.store_scale_ad(322, A::abs(s.ad_value(82)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(323, A::abs(s.ad_value(167)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(325, A::powf(A::scale(A::abs(s.ad_value(167)), 1.0 / ((1.0 - (p.p5 * p.p33)))), p.p126), (p.p128 * (1.0 - (p.p5 * p.p33))));

        s.v[621] = if (s.v[167] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[621] != 0.0) {
            s.store_neg(325, 325);
        }

        s.store_scale_ad(324, A::abs(s.ad_value(179)), ((2.0 * 1.6021918e-19) * p.p5));

        s.v[622] = if (p.p33 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[622] != 0.0) {
            s.store_scalar(326, 0.0);
        }

        if (!(s.v[622] != 0.0)) {
            s.store_scale_ad(326, A::powf(A::scale(A::abs(s.ad_value(179)), 1.0 / (p.p33)), p.p126), ((p.p128 * p.p5) * p.p33));
        }

        s.v[623] = if (s.v[179] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[623] != 0.0) {
            s.store_neg(326, 326);
        }

        s.store_scale_ad(330, A::abs(s.ad_value(185)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(331, A::abs(s.ad_value(182)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(332, A::abs(s.ad_value(183)), (2.0 * 1.6021918e-19));

        s.v[624] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[625] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.v[626] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

        s.v[627] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

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
        s.v[484] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[484] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[160] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[485] = if (p.p154 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.store_scalar(345, 1e-12);
        }

        if (!(s.v[485] != 0.0)) {
            s.store_scalar(345, p.p154);
        }

        s.store_scale(346, 345, p.p1);

        s.v[52] = 0.001;

        s.v[342] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[285] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.v[487] = if ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[487] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[487] != 0.0)) {
            s.store_scalar(74, ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) + (0.1 * (((1.0 + (((-s.v[285])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p114;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p71;

        s.v[76] = p.p72;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[285] = (((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) - 0.05) / 0.1);

        s.v[488] = if ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[488] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[285]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scalar(88, ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) + (0.1 * (((1.0 + (((-s.v[285])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p117;

        s.v[86] = (1.0 / s.v[87]);

        s.v[178] = 1.0;

        s.v[210] = 0.0;

        s.v[248] = 0.0;

        s.v[228] = 0.0;

        s.v[42] = 0.0;

        s.store_ad(218, &A::voltage(ctx, &nodes, Some(4), None));

        s.v[489] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_neg_ad(218, A::ln(A::sub_from_scalar(1.0, s.ad_value(218))));
        }

        s.v[490] = if (s.v[218] < p.p125) { 1.0 } else { 0.0 };

        if (s.v[490] != 0.0) {
            s.copy_ad(11, 218);
        }

        if (!(s.v[490] != 0.0)) {
            s.store_offset_ad(11, A::ln(A::offset(A::offset(s.ad_value(218), (-p.p125)), 1.0)), p.p125);
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

        s.store_scale_ad(285, A::offset(A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p115), s.ad_value(2)), A::offset(s.ad_value(2), p.p116))), (-0.05)), 10.0);

        s.v[491] = if ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[491] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[491] != 0.0)) {
            s.store_add_ad(70, A::sub(s.ad_value(74), A::div(A::mul(A::scale(s.ad_value(2), p.p115), s.ad_value(2)), A::offset(s.ad_value(2), p.p116))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 0.1));
        }

        s.store_scale_ad(285, A::offset(A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p118), s.ad_value(2)), A::offset(s.ad_value(2), p.p119))), (-0.05)), 10.0);

        s.v[492] = if ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[492] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[492] != 0.0)) {
            s.store_add_ad(85, A::sub(s.ad_value(88), A::div(A::mul(A::scale(s.ad_value(2), p.p118), s.ad_value(2)), A::offset(s.ad_value(2), p.p119))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 0.1));
        }

        s.store_add_ad(13, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p66)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p105));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(13)), 6);

        s.v[493] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[493] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[493] != 0.0)) {
            s.store_offset_ad(14, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(15, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p64)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(15)), 6);

        s.v[494] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_add_ad_rhs(16, 15, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[494] != 0.0)) {
            s.store_offset_ad(16, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(21, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p80)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(21)), 6);

        s.v[495] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[495] != 0.0) {
            s.store_add_ad_rhs(22, 21, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[495] != 0.0)) {
            s.store_offset_ad(22, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(18, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p71)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(18)), 6);

        s.v[496] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[496] != 0.0) {
            s.store_add_ad_rhs(17, 18, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[496] != 0.0)) {
            s.store_offset_ad(17, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(20, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), s.v[75])), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p110));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(20)), 6);

        s.v[497] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[497] != 0.0) {
            s.store_add_ad_rhs(19, 20, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[497] != 0.0)) {
            s.store_offset_ad(19, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(56, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p27)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p109));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(56)), 6);

        s.v[498] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_add_ad_rhs(55, 56, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_offset_ad(55, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_add_ad(104, A::add(A::mul(A::scale(s.ad_value(6), (-3.0)), s.ad_value(280)), A::scale(s.ad_value(4), p.p138)), A::scale(A::sub_from_scalar(1.0, s.ad_value(4)), p.p140));

        s.store_div_ad_lhs(285, A::sub_from_scalar(0.05, s.ad_value(104)), 6);

        s.v[499] = if (0.05 < s.v[104]) { 1.0 } else { 0.0 };

        if (s.v[499] != 0.0) {
            s.store_add_ad_rhs(105, 104, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[499] != 0.0)) {
            s.store_offset_ad(105, A::mul(s.ad_value(6), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))), 0.05);
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scale_ad(106, A::powf(A::div_from_scalar(p.p138, s.ad_value(105)), p.p139), p.p137);

        s.store_offset_ad(26, A::scale(A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75)), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.store_scale_ad(28, A::exp(A::scale(s.ad_value(280), p.p97)), p.p54);

        s.v[500] = if (s.v[28] < s.v[346]) { 1.0 } else { 0.0 };

        if (s.v[500] != 0.0) {
            s.copy_ad(28, 346);
        }

        s.store_scale_ad(29, A::exp(A::scale(s.ad_value(280), (p.p98 - p.p96))), p.p56);

        s.store_scale_ad(30, A::exp(A::scale(s.ad_value(280), p.p101)), p.p55);

        s.v[501] = if (s.v[30] < s.v[346]) { 1.0 } else { 0.0 };

        if (s.v[501] != 0.0) {
            s.copy_ad(30, 346);
        }

        s.store_scale_ad(32, A::exp(A::scale(s.ad_value(280), p.p102)), p.p57);

        s.store_scale_ad(31, A::exp(A::scale(s.ad_value(280), p.p99)), p.p60);

        s.v[502] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[502] != 0.0) {
            s.store_scale_ad(50, A::offset(A::scale(s.ad_value(12), p.p122), 1.0), p.p10);
        }

        if (s.v[502] != 0.0) {
            s.store_scaled_offset(285, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[503] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[502] != 0.0) && (s.v[503] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[502] != 0.0) && (!(s.v[503] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), s.v[52]));
        }

        if (s.v[502] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[502] != 0.0)) {
            s.store_scalar(48, p.p10);
        }

        s.v[504] = if (p.p123 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[504] != 0.0) {
            s.store_scale_ad(51, A::offset(A::scale(s.ad_value(12), p.p123), 1.0), p.p11);
        }

        if (s.v[504] != 0.0) {
            s.store_scaled_offset(285, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[505] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[504] != 0.0) && (s.v[505] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[504] != 0.0) && (!(s.v[505] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), s.v[52]));
        }

        if (s.v[504] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[504] != 0.0)) {
            s.store_scalar(49, p.p11);
        }

        s.store_scale_ad(341, A::offset(A::scale(s.ad_value(12), p.p124), 1.0), p.p43);

        s.v[287] = (s.v[342] * s.v[342]);

        s.store_square(288, 341);

        s.v[506] = if (s.v[341] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[506] != 0.0) {
            s.store_div_from_scalar_ad(340, (0.5 * s.v[287]), A::sub(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(341)));
        }

        if (!(s.v[506] != 0.0)) {
            s.store_scale_ad(340, A::add(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(341)), 0.5);
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div(A::scale(s.ad_value(280), (((4.0 - p.p98) - p.p96) + p.p121)), s.ad_value(48))), p.p9), A::exp(A::div(A::scale(s.ad_value(10), (-p.p105)), s.ad_value(48))));

        s.store_scale_ad(36, A::exp(A::scale(s.ad_value(280), (1.0 - p.p98))), p.p12);

        s.store_scale_ad(37, A::exp(A::scale(s.ad_value(280), (1.0 - p.p103))), p.p30);

        s.store_mul_ad(42, A::scale(A::exp(A::scale(s.ad_value(280), (((4.0 - p.p97) + p.p121) * 1.0 / (p.p17)))), p.p16), A::exp(A::scale(s.ad_value(10), ((-p.p111) * 1.0 / (p.p17)))));

        s.store_mul_ad(43, A::scale(A::exp(A::scale(s.ad_value(280), ((4.0 - p.p103) + p.p121))), p.p29), A::exp(A::scale(s.ad_value(10), (-p.p112))));

        s.store_powf_ad(281, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(282, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p35), s.ad_value(70)), s.ad_value(281)), s.ad_value(282)), p.p66), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(283, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(284, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p37), s.ad_value(85)), s.ad_value(283)), s.ad_value(284)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.store_exp_ad(281, A::scale(s.ad_value(280), p.p96));

        s.store_mul_ad_lhs(40, A::scale(s.ad_value(281), p.p14), 27);

        s.store_mul_ad_lhs(41, A::scale(s.ad_value(281), p.p13), 282);

        s.store_mul_ad(107, A::scale(A::exp(A::scale(s.ad_value(280), (4.0 - p.p141))), p.p133), A::exp(A::scale(s.ad_value(10), (-p.p140))));

        s.store_scale_ad(109, A::exp(A::scale(s.ad_value(280), (1.0 - p.p141))), p.p135);

        s.store_mul_ad(94, A::scale(A::exp(A::scale(s.ad_value(280), (p.p98 - 2.0))), p.p86), A::exp(A::scale(s.ad_value(10), (-p.p120))));

        s.store_scale_ad(95, A::exp(A::scale(s.ad_value(280), ((p.p96 + p.p98) - 1.0))), p.p87);

        s.store_scale_ad(96, A::exp(A::scale(s.ad_value(280), (p.p99 - 1.0))), p.p88);

        s.store_scaled_add(97, 95, 96, (p.p89 * 1.0 / ((p.p87 + p.p88))));

        s.store_scale_ad(98, A::exp(A::scale(s.ad_value(280), (p.p100 - 1.0))), p.p90);

        s.store_offset(101, 2, (-300.0));

        s.v[508] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[508] != 0.0) {
            s.store_mul_ad_rhs(99, 1, A::sub(A::offset(A::scale(s.ad_value(101), 0.00072), 1.0), A::mul(A::scale(s.ad_value(101), 1.6e-6), s.ad_value(101))));
        }

        if (!(s.v[508] != 0.0)) {
            s.store_scale(99, 1, 1.081);
        }

        s.store_scale_ad(100, A::exp(A::scale(s.ad_value(280), p.p96)), p.p92);

        s.store_ad(250, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(8)), p.p3));

        s.store_ad(251, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(9)), p.p3));

        s.store_ad(252, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(5)), p.p3));

        s.store_ad(253, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(5)), p.p3));

        s.store_ad(254, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(259, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(8)), p.p3));

        s.store_ad(256, &A::scale(A::voltage(ctx, &nodes, Some(8), Some(9)), p.p3));

        s.store_ad(266, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(6)), p.p3));

        s.store_ad(269, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(270, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(258, &A::scale(A::voltage(ctx, &nodes, Some(11), Some(8)), p.p3));

        s.store_ad(257, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(11)), p.p3));

        s.store_sub_ad_lhs(255, A::sub(A::add(s.ad_value(254), s.ad_value(251)), s.ad_value(256)), 258);

        s.store_sub_ad_lhs(268, A::add(A::sub(s.ad_value(266), s.ad_value(270)), s.ad_value(255)), 257);

        s.store_add(267, 270, 268);

        s.store_sub(261, 259, 258);

        s.store_sub(260, 261, 257);

        s.v[515] = if ((s.v[251] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[515] != 0.0) {
            s.store_exp_ad(271, A::mul(s.ad_value(251), s.ad_value(8)));
        }

        if (!(s.v[515] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[515] != 0.0)) {
            s.store_mul_ad_rhs(271, 301, A::offset(A::offset(A::mul(s.ad_value(251), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[516] = if (((s.v[252] * s.v[8]) / s.v[48]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[516] != 0.0) {
            s.store_exp_ad(272, A::div(A::mul(s.ad_value(252), s.ad_value(8)), s.ad_value(48)));
        }

        if (!(s.v[516] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[516] != 0.0)) {
            s.store_mul_ad_rhs(272, 301, A::offset(A::offset(A::div(A::mul(s.ad_value(252), s.ad_value(8)), s.ad_value(48)), (-p.p151)), 1.0));
        }

        s.v[517] = if ((s.v[255] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[517] != 0.0) {
            s.store_exp_ad(274, A::mul(s.ad_value(255), s.ad_value(8)));
        }

        if (!(s.v[517] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[517] != 0.0)) {
            s.store_mul_ad_rhs(274, 301, A::offset(A::offset(A::mul(s.ad_value(255), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[518] = if ((s.v[254] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (!(s.v[518] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.v[519] = if ((s.v[267] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[519] != 0.0) {
            s.store_exp_ad(275, A::mul(s.ad_value(267), s.ad_value(8)));
        }

        if (!(s.v[519] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
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
        if (!(s.v[519] != 0.0)) {
            s.store_mul_ad_rhs(275, 301, A::offset(A::offset(A::mul(s.ad_value(267), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[520] = if ((s.v[259] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (!(s.v[520] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.v[521] = if ((s.v[260] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[521] != 0.0) {
            s.store_exp_ad(263, A::mul(s.ad_value(260), s.ad_value(8)));
        }

        if (!(s.v[521] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[521] != 0.0)) {
            s.store_mul_ad_rhs(263, 301, A::offset(A::offset(A::mul(s.ad_value(260), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[522] = if ((s.v[261] * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (!(s.v[522] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.v[523] = if (((s.v[267] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[523] != 0.0) {
            s.store_exp_ad(278, A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[523] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[523] != 0.0)) {
            s.store_mul_ad_rhs(278, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(267), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[524] = if (((s.v[255] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[524] != 0.0) {
            s.store_exp_ad(276, A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[524] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[524] != 0.0)) {
            s.store_mul_ad_rhs(276, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(255), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[525] = if (((s.v[251] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[525] != 0.0) {
            s.store_exp_ad(277, A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[525] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[525] != 0.0)) {
            s.store_mul_ad_rhs(277, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(251), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[526] = if (((s.v[250] - s.v[16]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[526] != 0.0) {
            s.store_exp_ad(279, A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)));
        }

        if (!(s.v[526] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[526] != 0.0)) {
            s.store_mul_ad_rhs(279, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(250), s.ad_value(16)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.store_sqrt_ad(114, A::offset(A::scale(s.ad_value(277), 4.0), 1.0));

        s.store_sqrt_ad(115, A::offset(A::scale(s.ad_value(279), 4.0), 1.0));

        s.store_div_ad(116, A::scale(s.ad_value(279), 2.0), A::offset(s.ad_value(115), 1.0));

        s.v[527] = if (s.v[116] < p.p153) { 1.0 } else { 0.0 };

        if (s.v[527] != 0.0) {
            s.store_scalar(116, p.p153);
        }

        s.store_mul_ad_rhs(117, 6, A::sub(A::sub(s.ad_value(114), s.ad_value(115)), A::ln(A::div(A::offset(s.ad_value(114), 1.0), A::offset(s.ad_value(115), 1.0)))));

        s.store_div_ad_lhs(118, A::add(s.ad_value(117), s.ad_value(256)), 31);

        s.v[528] = if (s.v[118] > 0.0) { 1.0 } else { 0.0 };

        s.v[529] = if (s.v[250] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[529] != 0.0)) {
            s.copy_ad(303, 250);
        }

        if ((s.v[528] != 0.0) && (!(s.v[529] != 0.0))) {
            s.store_offset_ad(303, A::ln(A::offset(A::offset(s.ad_value(250), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[528] != 0.0) {
            s.store_sub_ad_lhs(119, A::add(s.ad_value(16), A::mul(A::scale(s.ad_value(6), 2.0), A::ln(A::offset(A::mul(A::mul(A::scale(s.ad_value(118), 0.5), s.ad_value(31)), s.ad_value(8)), 1.0)))), 303);
        }

        if (s.v[528] != 0.0) {
            s.store_scale(298, 16, 0.2);
        }

        if (s.v[528] != 0.0) {
            s.store_square(287, 298);
        }

        if (s.v[528] != 0.0) {
            s.store_square(288, 119);
        }

        s.v[530] = if (s.v[119] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[530] != 0.0)) {
            s.store_div_ad(120, A::scale(s.ad_value(287), 0.5), A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(119)));
        }

        if ((s.v[528] != 0.0) && (!(s.v[530] != 0.0))) {
            s.store_scale_ad(120, A::add(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(119)), 0.5);
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(121, A::mul(s.ad_value(120), A::offset(s.ad_value(120), (p.p62 * p.p61))), A::scale(A::add(s.ad_value(120), A::scale(s.ad_value(31), p.p62)), p.p61));
        }

        if (s.v[528] != 0.0) {
            s.store_div(291, 118, 121);
        }

        if (s.v[528] != 0.0) {
            s.store_scaled_offset(285, 291, (-1.0), 1.0 / (p.p63));
        }

        s.v[531] = if (s.v[291] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[531] != 0.0)) {
            s.store_offset_ad(289, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), p.p63), 1.0);
        }

        if ((s.v[528] != 0.0) && (!(s.v[531] != 0.0))) {
            s.store_add_ad_rhs(289, 291, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), p.p63));
        }

        if (s.v[528] != 0.0) {
            s.store_scale(122, 289, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[528] != 0.0) {
            s.store_scale(123, 120, 1.0 / ((p.p62 * p.p61)));
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(124, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(122), 4.0), s.ad_value(123)), A::offset(s.ad_value(123), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(122), 2.0), A::offset(s.ad_value(123), 1.0)));
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(125, A::add(A::sub_from_scalar(1.0, s.ad_value(124)), A::mul(s.ad_value(116), s.ad_value(124))), A::offset(A::mul(s.ad_value(116), s.ad_value(124)), 1.0));
        }

        if (s.v[528] != 0.0) {
            s.store_mul_ad_lhs(127, A::mul(A::mul(A::scale(s.ad_value(118), 0.5), s.ad_value(31)), s.ad_value(125)), 8);
        }

        if (s.v[528] != 0.0) {
            s.store_add_ad(292, A::scale(s.ad_value(127), 2.0), A::mul(s.ad_value(116), A::offset(A::add(s.ad_value(116), s.ad_value(127)), 1.0)));
        }

        if (s.v[528] != 0.0) {
            s.store_scaled_offset(128, 127, (-1.0), 0.5);
        }

        if (s.v[528] != 0.0) {
            s.store_add_ad_lhs(286, A::square(s.ad_value(128)), 292);
        }

        s.v[532] = if (s.v[127] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[532] != 0.0)) {
            s.store_add_ad_rhs(129, 128, A::sqrt(s.ad_value(286)));
        }

        if ((s.v[528] != 0.0) && (!(s.v[532] != 0.0))) {
            s.store_div_ad_rhs(129, 292, A::sub(A::sqrt(s.ad_value(286)), s.ad_value(128)));
        }

        s.v[533] = if (s.v[129] < p.p152) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[533] != 0.0)) {
            s.store_scalar(129, p.p152);
        }

        if (s.v[528] != 0.0) {
            s.store_mul_ad(131, A::mul(s.ad_value(129), A::offset(s.ad_value(129), 1.0)), A::exp(A::mul(s.ad_value(16), s.ad_value(8))));
        }

        if (s.v[528] != 0.0) {
            s.store_scaled_offset(133, 118, (-p.p62), (0.5 * p.p61));
        }

        if (s.v[528] != 0.0) {
            s.store_mul_ad_lhs(134, A::scale(s.ad_value(31), (p.p61 * p.p62)), 118);
        }

        if (s.v[528] != 0.0) {
            s.store_add_ad_rhs(135, 133, A::sqrt(A::add(A::square(s.ad_value(133)), s.ad_value(134))));
        }

        s.v[534] = if (p.p73 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[528] != 0.0) && (s.v[534] != 0.0)) {
            s.store_scale(136, 17, 0.1);
        }

        if ((s.v[528] != 0.0) && (!(s.v[534] != 0.0))) {
            s.store_mul_ad_rhs(136, 17, A::offset(A::div(A::scale(s.ad_value(118), 2.0), A::add(s.ad_value(118), s.ad_value(121))), 0.1));
        }

        if (s.v[528] != 0.0) {
            s.store_div_ad(137, A::scale(s.ad_value(118), p.p62), A::offset(s.ad_value(118), p.p62));
        }

        if (s.v[528] != 0.0) {
            s.store_div_from_scalar_ad(213, p.p62, A::offset(s.ad_value(118), p.p62));
        }

        if (!(s.v[528] != 0.0)) {
            s.store_scalar(121, 0.0);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_div_ad(129, A::scale(s.ad_value(277), 2.0), A::offset(s.ad_value(114), 1.0));
        }

        if (!(s.v[528] != 0.0)) {
            s.copy_ad(131, 271);
        }

        s.v[535] = if ((((s.v[256]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[117]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[114] + s.v[115])))) { 1.0 } else { 0.0 };

        if ((!(s.v[528] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_scaled_add(138, 129, 116, 0.5);
        }

        if ((!(s.v[528] != 0.0)) && (s.v[535] != 0.0)) {
            s.store_div_ad_rhs(125, 138, A::offset(s.ad_value(138), 1.0));
        }

        if ((!(s.v[528] != 0.0)) && (!(s.v[535] != 0.0))) {
            s.store_div_ad_rhs(125, 117, A::sub(A::add(s.ad_value(117), s.ad_value(251)), s.ad_value(250)));
        }

        if (!(s.v[528] != 0.0)) {
            s.copy_ad(135, 256);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_scale(136, 17, 0.1);
        }

        if (!(s.v[528] != 0.0)) {
            s.copy_ad(137, 118);
        }

        if (!(s.v[528] != 0.0)) {
            s.store_sub_from_scalar_ad(213, 1.0, A::scale(s.ad_value(137), 1.0 / (p.p62)));
        }

        s.store_scale(139, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(299, 14, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(252), s.ad_value(139)), 299);

        s.v[536] = if (s.v[252] < s.v[139]) { 1.0 } else { 0.0 };

        if (s.v[536] != 0.0) {
            s.store_sub_ad_rhs(140, 252, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[536] != 0.0)) {
            s.store_sub_ad_rhs(140, 139, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (1.0 - p.p67));

        s.store_add_ad(141, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(252), s.ad_value(140)), 3.0));

        s.v[537] = if (p.p74 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[537] != 0.0) {
            s.copy_ad(142, 250);
        }

        s.v[538] = if (p.p74 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[537] != 0.0)) && (s.v[538] != 0.0)) {
            s.store_add(142, 250, 135);
        }

        if ((!(s.v[537] != 0.0)) && (!(s.v[538] != 0.0))) {
            s.copy_ad(142, 251);
        }

        s.store_div_ad(143, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(144, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(143), ((-1.0) / p.p72))));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(142), s.ad_value(144)), 136);

        s.v[539] = if (s.v[142] < s.v[144]) { 1.0 } else { 0.0 };

        if (s.v[539] != 0.0) {
            s.store_sub_ad_rhs(145, 142, A::mul(s.ad_value(136), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[539] != 0.0)) {
            s.store_sub_ad_rhs(145, 144, A::mul(s.ad_value(136), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_powf(146, 213, p.p76);

        s.store_add_ad(147, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::mul(s.ad_value(146), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(145), s.ad_value(17))), (1.0 - p.p72))))), A::mul(A::mul(s.ad_value(146), s.ad_value(143)), A::sub(s.ad_value(142), s.ad_value(145))));

        s.store_add_ad(148, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(147)), A::mul(s.ad_value(25), s.ad_value(250)));

        s.store_div_ad_lhs(149, A::scale(s.ad_value(35), 4.0), 36);

        s.store_mul(150, 149, 272);

        s.store_div_ad_rhs(152, 150, A::offset(A::sqrt(A::offset(s.ad_value(150), 1.0)), 1.0));

        s.store_ad(132, &A::pow(s.ad_value(131), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(151, 149, 132);

        s.store_div_ad_rhs(153, 151, A::offset(A::sqrt(A::offset(s.ad_value(151), 1.0)), 1.0));

        s.v[540] = if (p.p92 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[540] != 0.0) {
            s.store_add_ad(154, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));
        }

        if (!(s.v[540] != 0.0)) {
            s.store_mul_ad_lhs(295, A::mul(A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), s.ad_value(100)), 8);
        }

        if (!(s.v[540] != 0.0)) {
            s.store_mul_ad_lhs(296, A::mul(A::div(A::neg(s.ad_value(148)), s.ad_value(40)), s.ad_value(100)), 8);
        }

        if (!(s.v[540] != 0.0)) {
            s.store_div_ad(154, A::sub(A::exp(s.ad_value(295)), A::exp(s.ad_value(296))), A::offset(A::exp(A::mul(s.ad_value(100), s.ad_value(8))), (-1.0)));
        }

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 154);

        s.v[541] = if (s.v[154] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[541] != 0.0) {
            s.store_div_from_scalar_ad(155, (0.5 * s.v[287]), A::sub(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(154)));
        }

        if (!(s.v[541] != 0.0)) {
            s.store_scale_ad(155, A::add(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(154)), 0.5);
        }

        s.store_mul_ad_rhs(156, 155, A::offset(A::scale(A::add(s.ad_value(152), s.ad_value(153)), 0.5), 1.0));

        s.store_mul_ad_lhs(157, A::scale(s.ad_value(35), p.p15), 132);

        s.store_mul(158, 35, 272);

        s.store_div_ad_lhs(159, A::sub(s.ad_value(158), s.ad_value(157)), 156);

        s.store_scale(285, 252, 10000.0);

        s.v[542] = if (s.v[252] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[542] != 0.0) {
            s.store_scale_ad(302, A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 0.0001);
        }

        if (!(s.v[542] != 0.0)) {
            s.store_add_ad_rhs(302, 252, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 0.0001));
        }

        s.store_scale(304, 302, 1.0 / (p.p156));

        s.v[543] = if (s.v[304] < p.p151) { 1.0 } else { 0.0 };

        if (!(s.v[543] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.store_scaled_offset(285, 252, (-p.p158), 1000.0);

        s.v[545] = if (((s.v[252] * s.v[8]) / p.p17) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p17)));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[545] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p17)), (-p.p151)), 1.0));
        }

        s.v[546] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[547] = if (((s.v[252] - s.v[55]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[546] != 0.0) && (s.v[547] != 0.0)) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[546] != 0.0) && (!(s.v[547] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((s.v[546] != 0.0) && (!(s.v[547] != 0.0))) {
            s.store_mul_ad_rhs(304, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(252), s.ad_value(55)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[548] = if (((s.v[159] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[546] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_scalar(301, ((40.0) as f64).exp());
        }

        s.v[550] = if (((s.v[253] * s.v[8]) / p.p19) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[550] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p19)));
        }

        if (!(s.v[550] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[550] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p19)), (-p.p151)), 1.0));
        }

        s.v[551] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[552] = if (((s.v[253] - s.v[55]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[551] != 0.0) && (s.v[552] != 0.0)) {
            s.store_exp_ad(304, A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)));
        }

        if ((s.v[551] != 0.0) && (!(s.v[552] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((s.v[551] != 0.0) && (!(s.v[552] != 0.0))) {
            s.store_mul_ad_rhs(304, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(253), s.ad_value(55)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        s.v[553] = if (((s.v[252] * s.v[8]) / p.p21) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[553] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p21)));
        }

        if (!(s.v[553] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[553] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(252), s.ad_value(8)), 1.0 / (p.p21)), (-p.p151)), 1.0));
        }

        s.v[554] = if (((s.v[253] * s.v[8]) / p.p23) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[554] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p23)));
        }

        if (!(s.v[554] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[554] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p23)), (-p.p151)), 1.0));
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
        s.v[555] = if (((s.v[255] * s.v[8]) / p.p32) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(255), s.ad_value(8)), 1.0 / (p.p32)));
        }

        if (!(s.v[555] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[555] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(255), s.ad_value(8)), 1.0 / (p.p32)), (-p.p151)), 1.0));
        }

        s.v[556] = if (((s.v[253] * s.v[8]) / p.p150) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_exp_ad(302, A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p150)));
        }

        if (!(s.v[556] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[556] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::scale(A::mul(s.ad_value(253), s.ad_value(8)), 1.0 / (p.p150)), (-p.p151)), 1.0));
        }

        s.v[557] = if (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[252] < 0.0)) { 1.0 } else { 0.0 };

        s.v[558] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[557] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (s.v[557] != 0.0) {
            s.store_mul(281, 252, 65);
        }

        if (s.v[557] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(281)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p67 * p.p67)), A::scale(s.ad_value(281), (3.0 * (p.p67 - 1.0)))), p.p67), A::mul(A::mul(A::scale(s.ad_value(281), 6.0), s.ad_value(281)), A::offset(s.ad_value(281), (p.p67 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[557] != 0.0) {
            s.store_div_ad(281, A::mul(A::scale(s.ad_value(252), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[559] = if (s.v[281] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[560] = if (s.v[281] < p.p151) { 1.0 } else { 0.0 };

        if (((s.v[557] != 0.0) && (s.v[559] != 0.0)) && (!(s.v[560] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.v[561] = if (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[250] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[561] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(250), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[562] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p151) { 1.0 } else { 0.0 };

        if ((s.v[561] != 0.0) && (!(s.v[562] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (s.v[561] != 0.0) {
            s.store_mul(283, 250, 67);
        }

        if (s.v[561] != 0.0) {
            let assign4640_ad_e4484: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(283)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(283), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(283), 6.0), s.ad_value(283)), A::offset(s.ad_value(283), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4640_ad_e4484);
        }

        if (s.v[561] != 0.0) {
            s.store_div_ad(283, A::mul(A::scale(s.ad_value(250), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[563] = if (s.v[283] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[564] = if (s.v[283] < p.p151) { 1.0 } else { 0.0 };

        if (((s.v[561] != 0.0) && (s.v[563] != 0.0)) && (!(s.v[564] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        s.store_mul(168, 149, 274);

        s.store_scale(169, 276, 4.0);

        s.store_div_ad(171, A::sub(s.ad_value(168), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(168), 1.0)), 1.0));

        s.store_div_ad_rhs(170, 169, A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0));

        s.v[566] = if ((p.p5 > 0.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[566] != 0.0) {
            s.store_div_ad(174, A::mul(A::scale(s.ad_value(43), (p.p33 * 2.0)), A::offset(s.ad_value(275), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(43), 4.0), s.ad_value(37)), s.ad_value(275)), 1.0)), 1.0));
        }

        s.v[567] = if (p.p8 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[566] != 0.0) && (s.v[567] != 0.0)) {
            s.store_div_ad(175, A::mul(A::scale(s.ad_value(107), (((1.0 - p.p143) * p.p33) * 2.0)), A::sub(s.ad_value(275), s.ad_value(263))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(107), 4.0), s.ad_value(109)), A::add(s.ad_value(275), A::scale(s.ad_value(263), p.p144))), 1.0)), 1.0));
        }

        if ((s.v[566] != 0.0) && (!(s.v[567] != 0.0))) {
            s.store_div_ad(175, A::mul(A::scale(s.ad_value(107), (((1.0 - p.p143) * p.p33) * 2.0)), A::offset(s.ad_value(275), (-1.0))), A::offset(A::sqrt(A::offset(A::mul(A::div(A::scale(s.ad_value(107), 4.0), s.ad_value(109)), s.ad_value(275)), 1.0)), 1.0));
        }

        s.v[568] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_mul_ad_lhs(297, A::scale(A::add(s.ad_value(43), s.ad_value(107)), p.p33), 32);
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_mul_ad_rhs(176, 6, A::sub_from_scalar(2.0, A::ln(A::mul(s.ad_value(297), s.ad_value(8)))));
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_sub(290, 267, 176);
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_scalar(287, (0.11 * 0.11));
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_square(288, 290);
        }

        s.v[569] = if (s.v[290] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[566] != 0.0) && (s.v[568] != 0.0)) && (s.v[569] != 0.0)) {
            s.store_div_ad(177, A::scale(s.ad_value(287), 0.5), A::sub(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(290)));
        }

        if (((s.v[566] != 0.0) && (s.v[568] != 0.0)) && (!(s.v[569] != 0.0))) {
            s.store_scale_ad(177, A::add(A::sqrt(A::add(s.ad_value(288), s.ad_value(287))), s.ad_value(290)), 0.5);
        }

        if ((s.v[566] != 0.0) && (s.v[568] != 0.0)) {
            s.store_div_ad_rhs(178, 177, A::add(A::add(s.ad_value(297), A::mul(A::add(s.ad_value(174), s.ad_value(175)), s.ad_value(32))), s.ad_value(177)));
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(176, 0.0);
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(290, 0.0);
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(177, 0.0);
        }

        if ((s.v[566] != 0.0) && (!(s.v[568] != 0.0))) {
            s.store_scalar(178, 1.0);
        }

        s.v[570] = if (p.p84 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[570] != 0.0) {
            s.store_add(353, 254, 250);
        }

        if (s.v[570] != 0.0) {
            s.store_scalar(287, (1e-6 * 1e-6));
        }

        if (s.v[570] != 0.0) {
            s.store_mul_ad_lhs(288, A::scale(s.ad_value(353), ((-1.0) * (-1.0))), 353);
        }

        s.store_add_ad(186, A::offset(A::div(s.ad_value(141), s.ad_value(41)), 1.0), A::div(s.ad_value(148), s.ad_value(40)));

        s.v[287] = (0.1 * 0.1);

        s.store_square(288, 186);

        s.v[573] = if (s.v[186] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[573] != 0.0) {
            s.store_div_from_scalar_ad(187, (0.5 * s.v[287]), A::sub(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(186)));
        }

        if (!(s.v[573] != 0.0)) {
            s.store_scale_ad(187, A::add(A::sqrt(A::offset(s.ad_value(288), s.v[287])), s.ad_value(186)), 0.5);
        }

        s.store_mul_ad_rhs(188, 187, A::offset(A::scale(A::add(s.ad_value(152), s.ad_value(153)), 0.5), 1.0));

        s.store_div(190, 29, 188);

        s.v[574] = if (s.v[190] < s.v[346]) { 1.0 } else { 0.0 };

        if (s.v[574] != 0.0) {
            s.copy_ad(190, 346);
        }

        s.store_scale(189, 190, 3.0);

        s.v[575] = if (s.v[159] > 0.0) { 1.0 } else { 0.0 };

        s.v[576] = if (p.p39 == 1.0) { 1.0 } else { 0.0 };

        s.v[577] = if (s.v[250] < p.p44) { 1.0 } else { 0.0 };

        s.v[578] = if (((-s.v[159]) / p.p42) < p.p151) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (s.v[578] != 0.0)) {
            s.store_exp_ad(338, A::scale(A::neg(s.ad_value(159)), 1.0 / (p.p42)));
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[578] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[578] != 0.0))) {
            s.store_mul_ad_rhs(338, 301, A::offset(A::offset(A::scale(A::neg(s.ad_value(159)), 1.0 / (p.p42)), (-p.p151)), 1.0));
        }

        if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
            s.store_mul_ad_lhs(339, A::sub_from_scalar(p.p44, s.ad_value(250)), 338);
        }

        s.v[579] = if (((-s.v[340]) * ((s.v[339]) as f64).powf(p.p41)) < p.p151) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (s.v[579] != 0.0)) {
            s.store_exp_ad(343, A::mul(A::neg(s.ad_value(340)), A::powf(s.ad_value(339), p.p41)));
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_mul_ad_rhs(343, 301, A::offset(A::offset(A::mul(A::neg(s.ad_value(340)), A::powf(s.ad_value(339), p.p41)), (-p.p151)), 1.0));
        }

        if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(A::div_from_scalar(p.p40, s.ad_value(340)), s.ad_value(339)), 343);
        }

        s.v[580] = if (p.p39 == 2.0) { 1.0 } else { 0.0 };

        s.v[581] = if (s.v[250] < s.v[16]) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_scalar(199, ((2.0 * p.p46) / (p.p45 * p.p45)));
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad_lhs(286, A::sub(s.ad_value(16), s.ad_value(250)), 213);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_sqrt_ad(200, A::div(A::scale(s.ad_value(286), 2.0), s.ad_value(199)));
        }

        s.v[582] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
            s.store_scalar(201, p.p45);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
            s.store_sub_from_scalar_ad(126, 1.0, A::scale(s.ad_value(125), 0.5));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
            s.store_mul_ad_lhs(201, A::scale(s.ad_value(126), p.p45), 126);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad(202, A::mul(s.ad_value(200), s.ad_value(201)), A::sqrt(A::add(A::square(s.ad_value(200)), A::square(s.ad_value(201)))));
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad_lhs(203, A::sub(s.ad_value(16), s.ad_value(250)), 202);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_add_ad_rhs(204, 203, A::mul(A::mul(A::scale(s.ad_value(202), 0.5), s.ad_value(199)), s.ad_value(213)));
        }

        s.v[583] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[583] != 0.0)) {
            s.copy_ad(205, 204);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_offset_ad(206, A::scale(A::offset(A::scale(s.ad_value(125), 2.0), 1.0), (2.0 * p.p47)), 1.0);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_scalar(207, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_sub_ad_rhs(208, 203, A::mul(A::mul(A::scale(s.ad_value(202), 0.5), s.ad_value(199)), A::sub(s.ad_value(207), A::div(s.ad_value(159), A::scale(s.ad_value(206), p.p62)))));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_add_ad(286, A::mul(A::sub(s.ad_value(208), s.ad_value(204)), A::sub(s.ad_value(208), s.ad_value(204))), A::scale(A::mul(A::mul(A::scale(s.ad_value(203), 0.1), s.ad_value(203)), s.ad_value(137)), 1.0 / (p.p62)));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_scale_ad(205, A::add(A::add(s.ad_value(208), s.ad_value(204)), A::sqrt(s.ad_value(286))), 0.5);
        }

        if ((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_div_ad_lhs(293, A::sub(s.ad_value(205), s.ad_value(203)), 205);
        }

        s.v[584] = if (((s.v[293]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[584] != 0.0)) {
            s.store_div_ad_lhs(209, A::scale(s.ad_value(202), 0.5), 293);
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[584] != 0.0)) {
            s.store_mul_ad(210, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(99)), s.ad_value(205)), s.ad_value(209)), A::sub(A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(205))), A::exp(A::mul(A::div(A::neg(s.ad_value(99)), s.ad_value(205)), A::offset(A::div(s.ad_value(201), s.ad_value(209)), 1.0)))));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[584] != 0.0))) {
            s.store_mul_ad(210, A::mul(s.ad_value(0), s.ad_value(201)), A::exp(A::div(A::neg(s.ad_value(99)), s.ad_value(205))));
        }

        s.v[585] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        s.v[586] = if (s.v[250] < p.p44) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) {
            s.store_mul_ad(214, A::powf(A::sub_from_scalar(p.p44, s.ad_value(250)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(159), A::offset(s.ad_value(159), p.p48))), p.p49));
        }

        s.v[587] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (s.v[587] != 0.0)) {
            s.copy_ad(215, 214);
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
            s.store_scaled_offset(216, 159, (-p.p52), 1.0 / (p.p48));
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
            s.store_scaled_offset(285, 216, (-1.0), 1.0 / (p.p51));
        }

        s.v[588] = if (s.v[216] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) && (s.v[588] != 0.0)) {
            s.store_offset_ad(217, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), p.p51), 1.0);
        }

        if (((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) && (!(s.v[588] != 0.0))) {
            s.store_add_ad_rhs(217, 216, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), p.p51));
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[587] != 0.0))) {
            s.store_mul_ad_rhs(215, 214, A::powf(s.ad_value(217), p.p50));
        }

        s.v[589] = if (((-s.v[340]) * s.v[215]) < p.p151) { 1.0 } else { 0.0 };

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (s.v[589] != 0.0)) {
            s.store_exp_ad(343, A::mul(A::neg(s.ad_value(340)), s.ad_value(215)));
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[589] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) && (!(s.v[589] != 0.0))) {
            s.store_mul_ad_rhs(343, 301, A::offset(A::offset(A::mul(A::neg(s.ad_value(340)), s.ad_value(215)), (-p.p151)), 1.0));
        }

        if (((((s.v[575] != 0.0) && (!(s.v[576] != 0.0))) && (!(s.v[580] != 0.0))) && (s.v[585] != 0.0)) && (s.v[586] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(A::div_from_scalar(p.p40, s.ad_value(340)), A::sub_from_scalar(p.p44, s.ad_value(250))), 343);
        }

        s.v[590] = if (s.v[210] > 0.0) { 1.0 } else { 0.0 };

        s.v[591] = if (p.p53 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) {
            s.store_add_ad(211, A::add(A::div(s.ad_value(6), A::mul(s.ad_value(159), A::add(s.ad_value(30), s.ad_value(189)))), A::mul(A::div(s.ad_value(156), s.ad_value(35)), s.ad_value(42))), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(189))));
        }

        s.v[592] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) {
            s.store_scaled_sub(285, 210, 211, 1000000.0);
        }

        s.v[593] = if (s.v[210] < s.v[211]) { 1.0 } else { 0.0 };

        if (((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) && (s.v[593] != 0.0)) {
            s.store_sub_ad_rhs(210, 210, A::scale(A::ln(A::offset(A::exp(s.ad_value(285)), 1.0)), 1e-6));
        }

        if (((((s.v[575] != 0.0) && (s.v[590] != 0.0)) && (s.v[591] != 0.0)) && (s.v[592] != 0.0)) && (!(s.v[593] != 0.0))) {
            s.store_sub_ad_rhs(210, 211, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0)), 1e-6));
        }

        s.store_mul_ad_lhs(221, A::scale(s.ad_value(23), (1.0 - p.p68)), 141);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(253), s.ad_value(139)), 299);

        s.v[596] = if (s.v[253] < s.v[139]) { 1.0 } else { 0.0 };

        if (s.v[596] != 0.0) {
            s.store_sub_ad_rhs(222, 253, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[596] != 0.0)) {
            s.store_sub_ad_rhs(222, 139, A::mul(s.ad_value(299), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_mul_ad(223, A::scale(s.ad_value(23), p.p68), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(65))), (1.0 - p.p67)))), A::scale(A::sub(s.ad_value(253), s.ad_value(222)), 3.0)));

        s.store_mul_ad_lhs(224, A::scale(s.ad_value(24), p.p77), 148);

        s.store_mul(225, 95, 36);

        s.store_mul_ad_lhs(229, A::mul(A::scale(s.ad_value(225), 0.5), s.ad_value(152)), 187);

        s.store_mul_ad_lhs(230, A::mul(A::scale(s.ad_value(225), 0.5), s.ad_value(153)), 187);

        s.store_scale(300, 17, 0.1);

        s.store_div_ad_lhs(285, A::sub(s.ad_value(255), s.ad_value(144)), 300);

        s.v[597] = if (s.v[255] < s.v[144]) { 1.0 } else { 0.0 };

        if (s.v[597] != 0.0) {
            s.store_sub_ad_rhs(231, 255, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[597] != 0.0)) {
            s.store_sub_ad_rhs(231, 144, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_add_ad(232, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(231), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(143), A::sub(s.ad_value(255), s.ad_value(231))));

        s.store_scale_ad(233, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(232)), A::mul(s.ad_value(25), s.ad_value(255)))), ((1.0 - p.p77) * (1.0 - p.p33)));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(267), s.ad_value(144)), 300);

        s.v[598] = if (s.v[267] < s.v[144]) { 1.0 } else { 0.0 };

        if (s.v[598] != 0.0) {
            s.store_sub_ad_rhs(234, 267, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[598] != 0.0)) {
            s.store_sub_ad_rhs(234, 144, A::mul(s.ad_value(300), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_add_ad(235, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(234), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(143), A::sub(s.ad_value(267), s.ad_value(234))));

        s.store_scale_ad(236, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(235)), A::mul(s.ad_value(25), s.ad_value(267)))), ((1.0 - p.p77) * p.p33));

        s.store_scale(307, 105, 0.1);

        s.store_scale(237, 105, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_ad_lhs(285, A::sub(s.ad_value(259), s.ad_value(237)), 307);

        s.v[599] = if (s.v[259] < s.v[237]) { 1.0 } else { 0.0 };

        if (s.v[599] != 0.0) {
            s.store_sub_ad_rhs(238, 259, A::mul(s.ad_value(307), A::ln(A::offset(A::exp(s.ad_value(285)), 1.0))));
        }

        if (!(s.v[599] != 0.0)) {
            s.store_sub_ad_rhs(238, 237, A::mul(s.ad_value(307), A::ln(A::offset(A::exp(A::neg(s.ad_value(285))), 1.0))));
        }

        s.store_mul_ad_rhs(239, 106, A::add(A::mul(A::scale(s.ad_value(105), 1.0 / ((1.0 - p.p139))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(238), s.ad_value(105))), (1.0 - p.p139)))), A::scale(A::sub(s.ad_value(259), s.ad_value(238)), 2.0)));

        s.store_mul_ad(240, A::mul(s.ad_value(94), s.ad_value(36)), A::powf(A::div(s.ad_value(35), s.ad_value(36)), (1.0 / p.p85)));

        s.v[600] = if ((s.v[252] / (p.p85 * s.v[6])) < p.p151) { 1.0 } else { 0.0 };

        if (s.v[600] != 0.0) {
            s.store_exp_ad(302, A::div(s.ad_value(252), A::scale(s.ad_value(6), p.p85)));
        }

        if (!(s.v[600] != 0.0)) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (!(s.v[600] != 0.0)) {
            s.store_mul_ad_rhs(302, 301, A::offset(A::offset(A::div(s.ad_value(252), A::scale(s.ad_value(6), p.p85)), (-p.p151)), 1.0));
        }

        s.store_mul(242, 240, 302);

        s.store_div_ad_lhs(243, A::mul(A::scale(s.ad_value(96), 4.0), s.ad_value(6)), 31);

        s.store_mul_ad(244, A::mul(A::scale(s.ad_value(243), 0.5), s.ad_value(125)), A::offset(A::add(s.ad_value(129), s.ad_value(116)), 2.0));

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
        let nv4 = ctx.node_voltage(nodes[4]);
        s.v[601] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[601] != 0.0) {
            s.store_div_ad(249, A::mul(A::scale(s.ad_value(97), 0.5), A::add(A::mul(s.ad_value(225), s.ad_value(171)), A::mul(s.ad_value(243), s.ad_value(170)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[602] = if ((((s.v[255] - s.v[22]) / p.p91) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if ((!(s.v[601] != 0.0)) && (s.v[602] != 0.0)) {
            s.store_exp_ad(180, A::mul(A::scale(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91)), s.ad_value(8)));
        }

        if ((!(s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if ((!(s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
            s.store_mul_ad_rhs(180, 301, A::offset(A::offset(A::mul(A::scale(A::sub(s.ad_value(255), s.ad_value(22)), 1.0 / (p.p91)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        if (!(s.v[601] != 0.0)) {
            s.store_div_ad(249, A::mul(A::mul(A::scale(s.ad_value(43), 2.0), s.ad_value(98)), s.ad_value(274)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(180), 4.0), 1.0)), 1.0));
        }

        s.v[603] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[603] != 0.0) {
            s.store_scale(249, 249, s.v[160]);
        }

        s.v[604] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_mul(172, 149, 275);
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_div_ad(173, A::sub(s.ad_value(172), s.ad_value(149)), A::offset(A::sqrt(A::offset(s.ad_value(172), 1.0)), 1.0));
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_scale(245, 278, 4.0);
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_div_ad_rhs(246, 245, A::offset(A::sqrt(A::offset(s.ad_value(245), 1.0)), 1.0));
        }

        if ((s.v[603] != 0.0) && (s.v[604] != 0.0)) {
            s.store_div_ad(247, A::mul(A::scale(s.ad_value(97), (0.5 * p.p33)), A::add(A::mul(s.ad_value(225), s.ad_value(173)), A::mul(s.ad_value(243), s.ad_value(246)))), A::add(s.ad_value(95), s.ad_value(96)));
        }

        s.v[605] = if (((s.v[267] - s.v[22]) * s.v[8]) < p.p151) { 1.0 } else { 0.0 };

        if (((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) && (s.v[605] != 0.0)) {
            s.store_exp_ad(181, A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)));
        }

        if (((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) && (!(s.v[605] != 0.0))) {
            s.store_scalar(301, ((p.p151) as f64).exp());
        }

        if (((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) && (!(s.v[605] != 0.0))) {
            s.store_mul_ad_rhs(181, 301, A::offset(A::offset(A::mul(A::sub(s.ad_value(267), s.ad_value(22)), s.ad_value(8)), (-p.p151)), 1.0));
        }

        if ((s.v[603] != 0.0) && (!(s.v[604] != 0.0))) {
            s.store_div_ad(247, A::mul(A::mul(A::scale(s.ad_value(43), (2.0 * p.p33)), s.ad_value(98)), s.ad_value(275)), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(181), 4.0), 1.0)), 1.0));
        }

        if (s.v[603] != 0.0) {
            s.store_mul(248, 178, 247);
        }

        s.v[606] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[606] != 0.0) {
            s.store_offset_ad(193, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(140), s.ad_value(65))), (-p.p67)), (-3.0));
        }

        if (s.v[606] != 0.0) {
            s.store_div_ad_lhs(294, A::sub(s.ad_value(252), s.ad_value(139)), 299);
        }

        s.v[607] = if (s.v[294] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[606] != 0.0) && (s.v[607] != 0.0)) {
            s.store_div_from_scalar_ad(194, 1.0, A::offset(A::exp(s.ad_value(294)), 1.0));
        }

        if ((s.v[606] != 0.0) && (!(s.v[607] != 0.0))) {
            s.store_div_ad(194, A::exp(A::neg(s.ad_value(294))), A::offset(A::exp(A::neg(s.ad_value(294))), 1.0));
        }

        if (s.v[606] != 0.0) {
            s.store_offset_ad(192, A::mul(s.ad_value(193), s.ad_value(194)), 3.0);
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad_lhs(195, A::scale(s.ad_value(23), (1.0 - p.p68)), 192);
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad(198, A::div(A::mul(A::mul(s.ad_value(149), s.ad_value(272)), s.ad_value(8)), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(150), 1.0))));
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad_lhs(196, A::mul(A::scale(s.ad_value(225), 0.5), s.ad_value(187)), 198);
        }

        if (s.v[606] != 0.0) {
            s.store_div_ad_rhs(197, 242, A::scale(s.ad_value(6), p.p85));
        }

        if (s.v[606] != 0.0) {
            s.store_mul_ad(228, A::scale(s.ad_value(254), 0.2), A::add(A::add(s.ad_value(195), s.ad_value(196)), s.ad_value(197)));
        }

        if (s.v[606] != 0.0) {
            s.store_scale(241, 242, (1.0 - p.p95));
        }

        if (s.v[606] != 0.0) {
            s.store_add_ad_rhs(337, 229, A::scale(s.ad_value(242), p.p95));
        }

        if (s.v[606] != 0.0) {
            s.store_add_ad_lhs(227, A::scale(s.ad_value(337), p.p94), 230);
        }

        if (s.v[606] != 0.0) {
            s.store_scale(226, 337, (1.0 - p.p94));
        }

        if (!(s.v[606] != 0.0)) {
            s.copy_ad(226, 229);
        }

        if (!(s.v[606] != 0.0)) {
            s.copy_ad(227, 230);
        }

        if (!(s.v[606] != 0.0)) {
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

        s.v[615] = if (s.v[333] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[615] != 0.0) {
            s.store_div_ad_lhs(335, A::add(s.ad_value(226), s.ad_value(227)), 333);
        }

        if (!(s.v[615] != 0.0)) {
            s.store_mul_ad_lhs(335, A::mul(s.ad_value(95), s.ad_value(187)), 156);
        }

        s.v[616] = if (p.p131 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[616] != 0.0) {
            s.store_scale(336, 335, p.p94);
        }

        s.v[617] = if (p.p131 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[616] != 0.0)) && (s.v[617] != 0.0)) {
            s.store_scale(336, 335, p.p132);
        }

        if ((!(s.v[616] != 0.0)) && (!(s.v[617] != 0.0))) {
            s.store_scalar(336, 0.0);
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
        let eq0_value: f64 = eq0_e169;
        let eq0_node_derivatives: [f64; 13] = [eq0_e169_d_n0, eq0_e169_d_n1, eq0_e169_d_n2, eq0_e169_d_n3, eq0_e169_d_n4, eq0_e169_d_n5, eq0_e169_d_n6, eq0_e169_d_n7, eq0_e169_d_n8, eq0_e169_d_n9, eq0_e169_d_n10, eq0_e169_d_n11, eq0_e169_d_n12];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
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
        let eq1_value: f64 = eq1_e174;
        let eq1_node_derivatives: [f64; 13] = [eq1_e174_d_n0, eq1_e174_d_n1, eq1_e174_d_n2, eq1_e174_d_n3, eq1_e174_d_n4, eq1_e174_d_n5, eq1_e174_d_n6, eq1_e174_d_n7, eq1_e174_d_n8, eq1_e174_d_n9, eq1_e174_d_n10, eq1_e174_d_n11, eq1_e174_d_n12];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
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
        let eq2_value: f64 = eq2_e183;
        let eq2_node_derivatives: [f64; 13] = [eq2_e183_d_n0, eq2_e183_d_n1, eq2_e183_d_n2, eq2_e183_d_n3, eq2_e183_d_n4, eq2_e183_d_n5, eq2_e183_d_n6, eq2_e183_d_n7, eq2_e183_d_n8, eq2_e183_d_n9, eq2_e183_d_n10, eq2_e183_d_n11, eq2_e183_d_n12];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let eq3_value: f64 = eq3_e200;
        let eq3_node_derivatives: [f64; 13] = [eq3_e200_d_n0, eq3_e200_d_n1, eq3_e200_d_n2, eq3_e200_d_n3, eq3_e200_d_n4, eq3_e200_d_n5, eq3_e200_d_n6, eq3_e200_d_n7, eq3_e200_d_n8, eq3_e200_d_n9, eq3_e200_d_n10, eq3_e200_d_n11, eq3_e200_d_n12];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
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
        let (eq4_e209, eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n2, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11, eq4_e209_d_n12,) = {
    if (s.v[608] != 0.0) {
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
        (eq4_e207, eq4_e207_d_n0, eq4_e207_d_n1, eq4_e207_d_n2, eq4_e207_d_n3, eq4_e207_d_n4, eq4_e207_d_n5, eq4_e207_d_n6, eq4_e207_d_n7, eq4_e207_d_n8, eq4_e207_d_n9, eq4_e207_d_n10, eq4_e207_d_n11, eq4_e207_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e209;
        let eq4_node_derivatives: [f64; 13] = [eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n2, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11, eq4_e209_d_n12];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let (eq5_e219, eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n2, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11, eq5_e219_d_n12,) = {
    if (!(s.v[608] != 0.0)) {
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
        (eq5_e217, eq5_e217_d_n0, eq5_e217_d_n1, eq5_e217_d_n2, eq5_e217_d_n3, eq5_e217_d_n4, eq5_e217_d_n5, eq5_e217_d_n6, eq5_e217_d_n7, eq5_e217_d_n8, eq5_e217_d_n9, eq5_e217_d_n10, eq5_e217_d_n11, eq5_e217_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e219;
        let eq5_node_derivatives: [f64; 13] = [eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n2, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11, eq5_e219_d_n12];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
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
        let eq6_value: f64 = eq6_e224;
        let eq6_node_derivatives: [f64; 13] = [eq6_e224_d_n0, eq6_e224_d_n1, eq6_e224_d_n2, eq6_e224_d_n3, eq6_e224_d_n4, eq6_e224_d_n5, eq6_e224_d_n6, eq6_e224_d_n7, eq6_e224_d_n8, eq6_e224_d_n9, eq6_e224_d_n10, eq6_e224_d_n11, eq6_e224_d_n12];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }
}
