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
        s.v[476] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[476] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[476] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[157] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[338] = 0.0;

        s.v[477] = if (p.p150 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[477] != 0.0) {
            s.store_scalar(339, 1e-12);
        }

        if (!(s.v[477] != 0.0)) {
            s.store_scalar(339, p.p150);
        }

        s.store_scale(340, 339, p.p1);

        s.store_div_from_scalar(341, 1.0, 340);

        s.v[478] = if (p.p134 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[478] != 0.0) {
            s.store_scalar(342, s.v[338]);
        }

        if (!(s.v[478] != 0.0)) {
            s.store_scalar(342, 0.0);
        }

        s.v[52] = 0.001;

        s.v[336] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[63] = (1.0 / s.v[62]);

        s.v[279] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.v[479] = if ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[479] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[479] != 0.0)) {
            s.store_scalar(74, ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) + (0.1 * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p114;

        s.v[72] = (1.0 / s.v[71]);

        s.v[64] = (1.0 / p.p66);

        s.v[75] = p.p71;

        s.v[76] = p.p72;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[89] = (1.0 / s.v[79]);

        s.v[279] = (((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) - 0.05) / 0.1);

        s.v[480] = if ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[480] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[480] != 0.0)) {
            s.store_scalar(88, ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) + (0.1 * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p117;

        s.v[86] = (1.0 / s.v[87]);

        s.v[66] = (1.0 / s.v[75]);

        s.v[343] = (1.0 - (1.0 / p.p83));

        s.v[158] = 0.0;

        s.v[159] = 0.0;

        s.v[176] = 0.0;

        s.v[175] = 1.0;

        s.v[207] = 0.0;

        s.v[209] = 0.0;

        s.v[242] = 0.0;

        s.v[222] = 0.0;

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

        s.v[274] = ((s.v[4]) as f64).ln();

        s.store_scale_ad(279, A::offset(A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), (-0.05)), 10.0);

        s.v[481] = if ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[481] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[481] != 0.0)) {
            s.store_add_ad(70, A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 0.1));
        }

        s.store_scale_ad(279, A::offset(A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), (-0.05)), 10.0);

        s.v[482] = if ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[482] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[482] != 0.0)) {
            s.store_add_ad(85, A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p66 * s.v[4])) + ((1.0 - s.v[4]) * p.p105));

        s.v[279] = ((0.05 - s.v[13]) / s.v[6]);

        s.v[483] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[483] != 0.0) {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[483] != 0.0)) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p64 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[15]) / s.v[6]);

        s.v[484] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p80 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[21]) / s.v[6]);

        s.v[485] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[485] != 0.0)) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p71 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[18]) / s.v[6]);

        s.v[486] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[486] != 0.0) {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[486] != 0.0)) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[274]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[20]) / s.v[6]);

        s.v[487] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[487] != 0.0) {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[487] != 0.0)) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p27 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[279] = ((0.05 - s.v[56]) / s.v[6]);

        s.v[488] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[488] != 0.0) {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[101] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p138 * s.v[4])) + ((1.0 - s.v[4]) * p.p140));

        s.v[279] = ((0.05 - s.v[101]) / s.v[6]);

        s.v[489] = if (0.05 < s.v[101]) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_scalar(102, (s.v[101] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[489] != 0.0)) {
            s.store_scalar(102, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scale_ad(103, A::powf(A::div_from_scalar(p.p138, s.ad_value(102)), p.p139), p.p137);

        s.store_offset_ad(26, A::scale(A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75)), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.v[28] = (p.p54 * (((s.v[274] * p.p97)) as f64).exp());

        s.v[490] = if (s.v[28] < s.v[340]) { 1.0 } else { 0.0 };

        if (s.v[490] != 0.0) {
            s.copy_ad(28, 340);
        }

        s.v[29] = (p.p56 * (((s.v[274] * (p.p98 - p.p96))) as f64).exp());

        s.v[30] = (p.p55 * (((s.v[274] * p.p101)) as f64).exp());

        s.v[491] = if (s.v[30] < s.v[340]) { 1.0 } else { 0.0 };

        if (s.v[491] != 0.0) {
            s.copy_ad(30, 340);
        }

        s.v[32] = (p.p57 * (((s.v[274] * p.p102)) as f64).exp());

        s.v[33] = (p.p58 * (((s.v[274] * p.p104)) as f64).exp());

        s.v[34] = (p.p59 * (((s.v[274] * p.p104)) as f64).exp());

        s.v[31] = (p.p60 * (((s.v[274] * p.p99)) as f64).exp());

        s.v[492] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[492] != 0.0) {
            s.store_scalar(50, (p.p10 * (1.0 + (s.v[12] * p.p122))));
        }

        if (s.v[492] != 0.0) {
            s.store_scaled_offset(279, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[493] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[492] != 0.0) && (s.v[493] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[492] != 0.0) && (!(s.v[493] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), s.v[52]));
        }

        if (s.v[492] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[492] != 0.0)) {
            s.store_scalar(48, p.p10);
        }

        s.v[494] = if (p.p123 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_scalar(51, (p.p11 * (1.0 + (s.v[12] * p.p123))));
        }

        if (s.v[494] != 0.0) {
            s.store_scaled_offset(279, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[495] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), s.v[52]));
        }

        if (s.v[494] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[494] != 0.0)) {
            s.store_scalar(49, p.p11);
        }

        s.v[335] = (p.p43 * (1.0 + (p.p124 * s.v[12])));

        s.v[281] = (s.v[336] * s.v[336]);

        s.v[282] = (s.v[335] * s.v[335]);

        s.v[496] = if (s.v[335] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[496] != 0.0) {
            s.store_scalar(334, ((0.5 * s.v[281]) / ((((s.v[282] + s.v[281])) as f64).sqrt() - s.v[335])));
        }

        if (!(s.v[496] != 0.0)) {
            s.store_scalar(334, (0.5 * ((((s.v[282] + s.v[281])) as f64).sqrt() + s.v[335])));
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div_from_scalar((s.v[274] * (((4.0 - p.p98) - p.p96) + p.p121)), s.ad_value(48))), p.p9), A::exp(A::div_from_scalar(((-p.p105) * s.v[10]), s.ad_value(48))));

        s.v[36] = (p.p12 * (((s.v[274] * (1.0 - p.p98))) as f64).exp());

        s.v[37] = (p.p30 * (((s.v[274] * (1.0 - p.p103))) as f64).exp());

        s.v[38] = ((p.p20 * (((s.v[274] * (6.0 - (2.0 * p.p21)))) as f64).exp()) * (((((-p.p113) * s.v[10]) / p.p21)) as f64).exp());

        s.v[39] = ((p.p31 * (((s.v[274] * (6.0 - (2.0 * p.p32)))) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p32)) as f64).exp());

        s.v[42] = ((p.p16 * ((((s.v[274] * ((4.0 - p.p97) + p.p121)) / p.p17)) as f64).exp()) * (((((-p.p111) * s.v[10]) / p.p17)) as f64).exp());

        s.v[44] = ((p.p18 * ((((s.v[274] * ((4.0 - p.p97) + p.p121)) / p.p19)) as f64).exp()) * (((((-p.p111) * s.v[10]) / p.p19)) as f64).exp());

        s.v[497] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[497] != 0.0) {
            s.store_scalar(53, (p.p25 * (((((-p.p107) * s.v[10]) / p.p17)) as f64).exp()));
        }

        if (s.v[497] != 0.0) {
            s.store_scalar(54, (p.p28 * ((((-p.p106) * s.v[10])) as f64).exp()));
        }

        if (s.v[497] != 0.0) {
            s.store_scalar(45, (p.p26 * (((((-p.p108) * s.v[10]) / p.p19)) as f64).exp()));
        }

        s.v[43] = ((p.p29 * (((s.v[274] * ((4.0 - p.p103) + p.p121))) as f64).exp()) * ((((-p.p112) * s.v[10])) as f64).exp());

        s.v[46] = ((p.p22 * (((s.v[274] * (6.0 - (2.0 * p.p23)))) as f64).exp()) * (((((-p.p113) * s.v[10]) / p.p23)) as f64).exp());

        s.v[47] = ((p.p145 * (((s.v[274] * (4.0 / p.p146))) as f64).exp()) * (((((-p.p113) * s.v[10]) / p.p146)) as f64).exp());

        s.v[350] = ((p.p151 * ((s.v[4]) as f64).sqrt()) * (((p.p153 * s.v[12])) as f64).exp());

        s.store_powf_ad(275, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(276, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p35), s.ad_value(70)), s.ad_value(275)), s.ad_value(276)), p.p66), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_mul_ad(58, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(275), p.p34), s.ad_value(14)), s.ad_value(14)), (s.v[64] * s.v[64])), s.ad_value(73)), A::exp(A::sub_from_scalar(p.p35, s.ad_value(61))));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(277, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(278, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p37), s.ad_value(85)), s.ad_value(277)), s.ad_value(278)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.store_mul_ad(84, A::mul(A::scale(A::mul(A::mul(A::scale(s.ad_value(277), p.p36), s.ad_value(19)), s.ad_value(19)), (s.v[66] * s.v[66])), s.ad_value(90)), A::exp(A::sub_from_scalar(p.p37, s.ad_value(83))));

        s.v[275] = (((s.v[274] * p.p96)) as f64).exp();

        s.store_scale(40, 27, (p.p14 * s.v[275]));

        s.store_scale(41, 276, (p.p13 * s.v[275]));

        s.v[104] = ((p.p133 * (((s.v[274] * (4.0 - p.p141))) as f64).exp()) * ((((-p.p140) * s.v[10])) as f64).exp());

        s.v[105] = ((p.p134 * (((s.v[274] * (3.5 - (0.5 * p.p142)))) as f64).exp()) * ((((-p.p140) * s.v[10])) as f64).exp());

        s.v[106] = (p.p135 * (((s.v[274] * (1.0 - p.p141))) as f64).exp());

        s.v[107] = (p.p136 * (((s.v[274] * (1.0 - p.p142))) as f64).exp());

        s.v[93] = ((p.p86 * (((s.v[274] * (p.p98 - 2.0))) as f64).exp()) * ((((-p.p120) * s.v[10])) as f64).exp());

        s.v[94] = (p.p87 * (((s.v[274] * ((p.p96 + p.p98) - 1.0))) as f64).exp());

        s.v[95] = (p.p88 * (((s.v[274] * (p.p99 - 1.0))) as f64).exp());

        s.v[96] = ((p.p89 * (s.v[94] + s.v[95])) / (p.p87 + p.p88));

        s.v[97] = (p.p90 * (((s.v[274] * (p.p100 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.v[498] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p92 * (((s.v[274] * p.p96)) as f64).exp());

        s.v[499] = if (p.p57 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[499] != 0.0) {
            s.store_scalar(108, (1.0 / s.v[32]));
        }

        s.v[500] = if (s.v[108] > s.v[341]) { 1.0 } else { 0.0 };

        if ((s.v[499] != 0.0) && (s.v[500] != 0.0)) {
            s.copy_ad(108, 341);
        }

        if (!(s.v[499] != 0.0)) {
            s.store_scalar(108, 0.0);
        }

        s.v[501] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[501] != 0.0) {
            s.store_scalar(109, (1.0 / s.v[33]));
        }

        s.v[502] = if (s.v[109] > s.v[341]) { 1.0 } else { 0.0 };

        if ((s.v[501] != 0.0) && (s.v[502] != 0.0)) {
            s.copy_ad(109, 341);
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
        if (!(s.v[501] != 0.0)) {
            s.store_scalar(109, 0.0);
        }

        s.v[503] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[503] != 0.0) {
            s.store_scalar(110, (1.0 / s.v[34]));
        }

        s.v[504] = if (s.v[110] > s.v[341]) { 1.0 } else { 0.0 };

        if ((s.v[503] != 0.0) && (s.v[504] != 0.0)) {
            s.copy_ad(110, 341);
        }

        if (!(s.v[503] != 0.0)) {
            s.store_scalar(110, 0.0);
        }

        s.store_ad(244, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(245, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(8)), p.p3));

        s.store_ad(246, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(4)), p.p3));

        s.store_ad(247, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), p.p3));

        s.store_ad(248, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), p.p3));

        s.store_ad(253, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(7)), p.p3));

        s.store_ad(250, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(8)), p.p3));

        s.store_ad(259, &A::scale(A::voltage(ctx, &nodes, Some(2), Some(4)), p.p3));

        s.store_ad(260, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p3));

        s.store_ad(263, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(264, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(252, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(7)), p.p3));

        s.store_ad(251, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(10)), p.p3));

        s.store_sub_ad_lhs(249, A::sub(A::add(s.ad_value(248), s.ad_value(245)), s.ad_value(250)), 252);

        s.store_sub_ad_lhs(262, A::add(A::sub(s.ad_value(260), s.ad_value(264)), s.ad_value(249)), 251);

        s.store_add(261, 264, 262);

        s.store_sub(255, 253, 252);

        s.store_sub(254, 255, 251);

        s.v[505] = if ((s.v[245] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[505] != 0.0) {
            s.store_exp_ad(265, A::scale(s.ad_value(245), s.v[8]));
        }

        if (!(s.v[505] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[505] != 0.0)) {
            s.store_mul_ad_rhs(265, 295, A::offset(A::offset(A::scale(s.ad_value(245), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[506] = if (((s.v[246] * s.v[8]) / s.v[48]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[506] != 0.0) {
            s.store_exp_ad(266, A::div(A::scale(s.ad_value(246), s.v[8]), s.ad_value(48)));
        }

        if (!(s.v[506] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[506] != 0.0)) {
            s.store_mul_ad_rhs(266, 295, A::offset(A::offset(A::div(A::scale(s.ad_value(246), s.v[8]), s.ad_value(48)), (-p.p147)), 1.0));
        }

        s.v[507] = if ((s.v[249] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[507] != 0.0) {
            s.store_exp_ad(268, A::scale(s.ad_value(249), s.v[8]));
        }

        if (!(s.v[507] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[507] != 0.0)) {
            s.store_mul_ad_rhs(268, 295, A::offset(A::offset(A::scale(s.ad_value(249), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[508] = if ((s.v[248] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[508] != 0.0) {
            s.store_exp_ad(267, A::scale(s.ad_value(248), s.v[8]));
        }

        if (!(s.v[508] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[508] != 0.0)) {
            s.store_mul_ad_rhs(267, 295, A::offset(A::offset(A::scale(s.ad_value(248), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[509] = if ((s.v[261] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[509] != 0.0) {
            s.store_exp_ad(269, A::scale(s.ad_value(261), s.v[8]));
        }

        if (!(s.v[509] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[509] != 0.0)) {
            s.store_mul_ad_rhs(269, 295, A::offset(A::offset(A::scale(s.ad_value(261), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[510] = if ((s.v[253] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[510] != 0.0) {
            s.store_exp_ad(256, A::scale(s.ad_value(253), s.v[8]));
        }

        if (!(s.v[510] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[510] != 0.0)) {
            s.store_mul_ad_rhs(256, 295, A::offset(A::offset(A::scale(s.ad_value(253), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[511] = if ((s.v[254] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[511] != 0.0) {
            s.store_exp_ad(257, A::scale(s.ad_value(254), s.v[8]));
        }

        if (!(s.v[511] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[511] != 0.0)) {
            s.store_mul_ad_rhs(257, 295, A::offset(A::offset(A::scale(s.ad_value(254), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[512] = if ((s.v[255] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[512] != 0.0) {
            s.store_exp_ad(258, A::scale(s.ad_value(255), s.v[8]));
        }

        if (!(s.v[512] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[512] != 0.0)) {
            s.store_mul_ad_rhs(258, 295, A::offset(A::offset(A::scale(s.ad_value(255), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[513] = if (((s.v[261] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[513] != 0.0) {
            s.store_exp_ad(272, A::scale(A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[513] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[513] != 0.0)) {
            s.store_mul_ad_rhs(272, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[514] = if (((s.v[249] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[514] != 0.0) {
            s.store_exp_ad(270, A::scale(A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[514] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[514] != 0.0)) {
            s.store_mul_ad_rhs(270, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[515] = if (((s.v[245] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[515] != 0.0) {
            s.store_exp_ad(271, A::scale(A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[515] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[515] != 0.0)) {
            s.store_mul_ad_rhs(271, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[516] = if (((s.v[244] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[516] != 0.0) {
            s.store_exp_ad(273, A::scale(A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[516] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[516] != 0.0)) {
            s.store_mul_ad_rhs(273, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.store_sqrt_ad(111, A::offset(A::scale(s.ad_value(271), 4.0), 1.0));

        s.store_sqrt_ad(112, A::offset(A::scale(s.ad_value(273), 4.0), 1.0));

        s.store_div_ad(113, A::scale(s.ad_value(273), 2.0), A::offset(s.ad_value(112), 1.0));

        s.v[517] = if (s.v[113] < p.p149) { 1.0 } else { 0.0 };

        if (s.v[517] != 0.0) {
            s.store_scalar(113, p.p149);
        }

        s.store_scale_ad(114, A::sub(A::sub(s.ad_value(111), s.ad_value(112)), A::ln(A::div(A::offset(s.ad_value(111), 1.0), A::offset(s.ad_value(112), 1.0)))), s.v[6]);

        s.store_scaled_add(115, 114, 250, 1.0 / (s.v[31]));

        s.v[518] = if (s.v[115] > 0.0) { 1.0 } else { 0.0 };

        s.v[519] = if (s.v[244] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[519] != 0.0)) {
            s.copy_ad(297, 244);
        }

        if ((s.v[518] != 0.0) && (!(s.v[519] != 0.0))) {
            s.store_offset_ad(297, A::ln(A::offset(A::offset(s.ad_value(244), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[518] != 0.0) {
            s.store_sub_ad_lhs(116, A::add(s.ad_value(16), A::scale(A::ln(A::offset(A::scale(s.ad_value(115), (0.5 * (s.v[31] * s.v[8]))), 1.0)), (2.0 * s.v[6]))), 297);
        }

        if (s.v[518] != 0.0) {
            s.store_scale(292, 16, 0.2);
        }

        if (s.v[518] != 0.0) {
            s.store_square(281, 292);
        }

        if (s.v[518] != 0.0) {
            s.store_square(282, 116);
        }

        s.v[520] = if (s.v[116] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[520] != 0.0)) {
            s.store_div_ad(117, A::scale(s.ad_value(281), 0.5), A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)));
        }

        if ((s.v[518] != 0.0) && (!(s.v[520] != 0.0))) {
            s.store_scale_ad(117, A::add(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)), 0.5);
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(118, A::mul(s.ad_value(117), A::offset(s.ad_value(117), (p.p62 * p.p61))), A::scale(A::offset(s.ad_value(117), (p.p62 * s.v[31])), p.p61));
        }

        if (s.v[518] != 0.0) {
            s.store_div(285, 115, 118);
        }

        if (s.v[518] != 0.0) {
            s.store_scaled_offset(279, 285, (-1.0), 1.0 / (p.p63));
        }

        s.v[521] = if (s.v[285] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[521] != 0.0)) {
            s.store_offset_ad(283, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), p.p63), 1.0);
        }

        if ((s.v[518] != 0.0) && (!(s.v[521] != 0.0))) {
            s.store_add_ad_rhs(283, 285, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), p.p63));
        }

        if (s.v[518] != 0.0) {
            s.store_scale(119, 283, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[518] != 0.0) {
            s.store_scale(120, 117, 1.0 / ((p.p62 * p.p61)));
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(121, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(119), 4.0), s.ad_value(120)), A::offset(s.ad_value(120), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(119), 2.0), A::offset(s.ad_value(120), 1.0)));
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(122, A::add(A::sub_from_scalar(1.0, s.ad_value(121)), A::mul(s.ad_value(113), s.ad_value(121))), A::offset(A::mul(s.ad_value(113), s.ad_value(121)), 1.0));
        }

        if (s.v[518] != 0.0) {
            s.store_scale_ad(124, A::mul(A::scale(s.ad_value(115), (0.5 * s.v[31])), s.ad_value(122)), s.v[8]);
        }

        if (s.v[518] != 0.0) {
            s.store_add_ad(286, A::scale(s.ad_value(124), 2.0), A::mul(s.ad_value(113), A::offset(A::add(s.ad_value(113), s.ad_value(124)), 1.0)));
        }

        if (s.v[518] != 0.0) {
            s.store_scaled_offset(125, 124, (-1.0), 0.5);
        }

        if (s.v[518] != 0.0) {
            s.store_add_ad_lhs(280, A::square(s.ad_value(125)), 286);
        }

        s.v[522] = if (s.v[124] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[522] != 0.0)) {
            s.store_add_ad_rhs(126, 125, A::sqrt(s.ad_value(280)));
        }

        if ((s.v[518] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_div_ad_rhs(126, 286, A::sub(A::sqrt(s.ad_value(280)), s.ad_value(125)));
        }

        s.v[523] = if (s.v[126] < p.p148) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[523] != 0.0)) {
            s.store_scalar(126, p.p148);
        }

        if (s.v[518] != 0.0) {
            s.store_mul_ad(128, A::mul(s.ad_value(126), A::offset(s.ad_value(126), 1.0)), A::exp(A::scale(s.ad_value(16), s.v[8])));
        }

        if (s.v[518] != 0.0) {
            s.store_scaled_offset(130, 115, (-p.p62), (0.5 * p.p61));
        }

        if (s.v[518] != 0.0) {
            s.store_scale(131, 115, ((p.p61 * s.v[31]) * p.p62));
        }

        if (s.v[518] != 0.0) {
            s.store_add_ad_rhs(132, 130, A::sqrt(A::add(A::square(s.ad_value(130)), s.ad_value(131))));
        }

        s.v[524] = if (p.p73 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale(133, 17, 0.1);
        }

        if ((s.v[518] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_mul_ad_rhs(133, 17, A::offset(A::div(A::scale(s.ad_value(115), 2.0), A::add(s.ad_value(115), s.ad_value(118))), 0.1));
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(134, A::scale(s.ad_value(115), p.p62), A::offset(s.ad_value(115), p.p62));
        }

        if (s.v[518] != 0.0) {
            s.store_div_from_scalar_ad(210, p.p62, A::offset(s.ad_value(115), p.p62));
        }

        if (!(s.v[518] != 0.0)) {
            s.store_scalar(118, 0.0);
        }

        if (!(s.v[518] != 0.0)) {
            s.store_div_ad(126, A::scale(s.ad_value(271), 2.0), A::offset(s.ad_value(111), 1.0));
        }

        if (!(s.v[518] != 0.0)) {
            s.copy_ad(128, 265);
        }

        s.v[525] = if ((((s.v[250]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[114]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[111] + s.v[112])))) { 1.0 } else { 0.0 };

        if ((!(s.v[518] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_scaled_add(135, 126, 113, 0.5);
        }

        if ((!(s.v[518] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_div_ad_rhs(122, 135, A::offset(s.ad_value(135), 1.0));
        }

        if ((!(s.v[518] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_div_ad_rhs(122, 114, A::sub(A::add(s.ad_value(114), s.ad_value(245)), s.ad_value(244)));
        }

        if (!(s.v[518] != 0.0)) {
            s.copy_ad(132, 250);
        }

        if (!(s.v[518] != 0.0)) {
            s.store_scale(133, 17, 0.1);
        }

        if (!(s.v[518] != 0.0)) {
            s.copy_ad(134, 115);
        }

        if (!(s.v[518] != 0.0)) {
            s.store_sub_from_scalar_ad(210, 1.0, A::scale(s.ad_value(134), 1.0 / (p.p62)));
        }

        s.store_scale(136, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(293, 14, 0.1);

        s.store_div_ad_lhs(279, A::sub(s.ad_value(246), s.ad_value(136)), 293);

        s.v[526] = if (s.v[246] < s.v[136]) { 1.0 } else { 0.0 };

        if (s.v[526] != 0.0) {
            s.store_sub_ad_rhs(137, 246, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[526] != 0.0)) {
            s.store_sub_ad_rhs(137, 136, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (1.0 - p.p67));

        s.store_add_ad(138, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(246), s.ad_value(137)), 3.0));

        s.v[527] = if (p.p74 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[527] != 0.0) {
            s.copy_ad(139, 244);
        }

        s.v[528] = if (p.p74 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[527] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_add(139, 244, 132);
        }

        if ((!(s.v[527] != 0.0)) && (!(s.v[528] != 0.0))) {
            s.copy_ad(139, 245);
        }

        s.store_div_ad(140, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(141, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(140), ((-1.0) / p.p72))));

        s.store_div_ad_lhs(279, A::sub(s.ad_value(139), s.ad_value(141)), 133);

        s.v[529] = if (s.v[139] < s.v[141]) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_sub_ad_rhs(142, 139, A::mul(s.ad_value(133), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_sub_ad_rhs(142, 141, A::mul(s.ad_value(133), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_powf(143, 210, p.p76);

        s.store_add_ad(144, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::mul(s.ad_value(143), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(142), s.ad_value(17))), (1.0 - p.p72))))), A::mul(A::mul(s.ad_value(143), s.ad_value(140)), A::sub(s.ad_value(139), s.ad_value(142))));

        s.store_add_ad(145, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(144)), A::mul(s.ad_value(25), s.ad_value(244)));

        s.store_scale(146, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(147, 146, 266);

        s.store_div_ad_rhs(149, 147, A::offset(A::sqrt(A::offset(s.ad_value(147), 1.0)), 1.0));

        s.store_ad(129, &A::pow(s.ad_value(128), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(148, 146, 129);

        s.store_div_ad_rhs(150, 148, A::offset(A::sqrt(A::offset(s.ad_value(148), 1.0)), 1.0));

        s.v[530] = if (p.p92 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[530] != 0.0) {
            s.store_add_ad(151, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scale_ad(289, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), (s.v[99] * s.v[8]));
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scale_ad(290, A::div(A::neg(s.ad_value(145)), s.ad_value(40)), (s.v[99] * s.v[8]));
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scale_ad(151, A::sub(A::exp(s.ad_value(289)), A::exp(s.ad_value(290))), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
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
        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 151);

        s.v[531] = if (s.v[151] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[531] != 0.0) {
            s.store_div_from_scalar_ad(152, (0.5 * s.v[281]), A::sub(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(151)));
        }

        if (!(s.v[531] != 0.0)) {
            s.store_scale_ad(152, A::add(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(151)), 0.5);
        }

        s.store_mul_ad_rhs(153, 152, A::offset(A::scale(A::add(s.ad_value(149), s.ad_value(150)), 0.5), 1.0));

        s.store_mul_ad_lhs(154, A::scale(s.ad_value(35), p.p15), 129);

        s.store_mul(155, 35, 266);

        s.store_div_ad_lhs(156, A::sub(s.ad_value(155), s.ad_value(154)), 153);

        s.store_scale(279, 246, 10000.0);

        s.v[532] = if (s.v[246] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[532] != 0.0) {
            s.store_scale_ad(296, A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 0.0001);
        }

        if (!(s.v[532] != 0.0)) {
            s.store_add_ad_rhs(296, 246, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 0.0001));
        }

        s.store_scale(298, 296, 1.0 / (p.p152));

        s.v[533] = if (s.v[298] < p.p147) { 1.0 } else { 0.0 };

        if (s.v[533] != 0.0) {
            s.store_exp(299, 298);
        }

        if (!(s.v[533] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[533] != 0.0)) {
            s.store_mul_ad_rhs(299, 295, A::offset(A::offset(s.ad_value(298), (-p.p147)), 1.0));
        }

        s.store_scaled_offset(351, 299, (-1.0), s.v[350]);

        s.store_scaled_offset(279, 246, (-p.p154), 1000.0);

        s.v[534] = if (s.v[246] < p.p154) { 1.0 } else { 0.0 };

        if (s.v[534] != 0.0) {
            s.store_sub_ad_rhs(300, 246, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 0.001));
        }

        if (!(s.v[534] != 0.0)) {
            s.store_sub_from_scalar_ad(300, p.p154, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 0.001));
        }

        s.store_mul_ad(352, A::scale(s.ad_value(300), p.p155), A::powf(A::sub_from_scalar(p.p154, s.ad_value(300)), 2.0));

        s.v[535] = if (((s.v[246] * s.v[8]) / p.p17) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[535] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p17))));
        }

        if (!(s.v[535] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[535] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p17))), (-p.p147)), 1.0));
        }

        s.v[536] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[537] = if (((s.v[246] - s.v[55]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[536] != 0.0) && (s.v[537] != 0.0)) {
            s.store_exp_ad(298, A::scale(A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[536] != 0.0) && (!(s.v[537] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((s.v[536] != 0.0) && (!(s.v[537] != 0.0))) {
            s.store_mul_ad_rhs(298, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[538] = if (((s.v[156] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[536] != 0.0) && (s.v[538] != 0.0)) {
            s.store_exp_ad(299, A::offset(A::div(s.ad_value(156), s.ad_value(35)), (-1000.0)));
        }

        if ((s.v[536] != 0.0) && (!(s.v[538] != 0.0))) {
            s.store_scalar(295, ((40.0) as f64).exp());
        }

        if ((s.v[536] != 0.0) && (!(s.v[538] != 0.0))) {
            s.store_mul_ad_rhs(299, 295, A::offset(A::offset(A::offset(A::div(s.ad_value(156), s.ad_value(35)), (-1000.0)), (-40.0)), 1.0));
        }

        if (s.v[536] != 0.0) {
            let assign3980_ad_e3745: A = A::add(A::add(A::scale(A::offset(s.ad_value(296), (-1.0)), s.v[42]), A::mul(A::div(A::mul(A::scale(s.ad_value(53), 2.0), A::offset(s.ad_value(296), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(298), 4.0), 1.0)), 1.0)), A::offset(A::div(s.ad_value(145), s.ad_value(40)), 1.0))), A::div(A::mul(A::mul(s.ad_value(54), A::offset(s.ad_value(128), (-1.0))), s.ad_value(299)), A::offset(s.ad_value(299), 1.0)));
            s.store_ad(158, &assign3980_ad_e3745);
        }

        s.v[539] = if (p.p93 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[536] != 0.0)) && (s.v[539] != 0.0)) {
            s.store_scaled_offset(158, 296, (-1.0), s.v[42]);
        }

        if ((!(s.v[536] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_scale_ad(158, A::add(A::scale(A::offset(s.ad_value(296), (-1.0)), (1.0 - p.p93)), A::mul(A::scale(A::offset(A::add(s.ad_value(296), s.ad_value(128)), (-2.0)), p.p93), A::offset(A::div(s.ad_value(145), s.ad_value(40)), 1.0))), s.v[42]);
        }

        s.v[540] = if (((s.v[247] * s.v[8]) / p.p19) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[540] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p19))));
        }

        if (!(s.v[540] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[540] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p19))), (-p.p147)), 1.0));
        }

        s.v[541] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[542] = if (((s.v[247] - s.v[55]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[541] != 0.0) && (s.v[542] != 0.0)) {
            s.store_exp_ad(298, A::scale(A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[541] != 0.0) && (!(s.v[542] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((s.v[541] != 0.0) && (!(s.v[542] != 0.0))) {
            s.store_mul_ad_rhs(298, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]), (-p.p147)), 1.0));
        }

        if (s.v[541] != 0.0) {
            s.store_add_ad(159, A::scale(A::offset(s.ad_value(296), (-1.0)), s.v[44]), A::div(A::mul(A::scale(s.ad_value(45), 2.0), A::offset(s.ad_value(296), (-1.0))), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(298), 4.0), 1.0)), 1.0)));
        }

        if (!(s.v[541] != 0.0)) {
            s.store_scaled_offset(159, 296, (-1.0), s.v[44]);
        }

        s.v[543] = if (((s.v[246] * s.v[8]) / p.p21) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p21))));
        }

        if (!(s.v[543] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[543] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p21))), (-p.p147)), 1.0));
        }

        s.store_scaled_offset(160, 296, (-1.0), s.v[38]);

        s.v[544] = if (((s.v[247] * s.v[8]) / p.p23) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p23))));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[544] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p23))), (-p.p147)), 1.0));
        }

        s.store_scaled_offset(162, 296, (-1.0), s.v[46]);

        s.v[545] = if (((s.v[249] * s.v[8]) / p.p32) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(249), (s.v[8] * 1.0 / (p.p32))));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[545] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(249), (s.v[8] * 1.0 / (p.p32))), (-p.p147)), 1.0));
        }

        s.store_scaled_offset(161, 296, (-1.0), s.v[39]);

        s.v[546] = if (((s.v[247] * s.v[8]) / p.p146) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[546] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p146))));
        }

        if (!(s.v[546] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[546] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p146))), (-p.p147)), 1.0));
        }

        s.store_scaled_offset(163, 296, (-1.0), s.v[47]);

        s.v[547] = if (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[246] < 0.0)) { 1.0 } else { 0.0 };

        s.v[548] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[547] != 0.0) && (s.v[548] != 0.0)) {
            s.store_exp_ad(68, A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))));
        }

        if ((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_mul_ad_rhs(68, 295, A::offset(A::offset(A::mul(s.ad_value(61), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0)))), (-p.p147)), 1.0));
        }

        if (s.v[547] != 0.0) {
            s.store_mul(275, 246, 65);
        }

        if (s.v[547] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(275)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p67 * p.p67)), A::scale(s.ad_value(275), (3.0 * (p.p67 - 1.0)))), p.p67), A::mul(A::mul(A::scale(s.ad_value(275), 6.0), s.ad_value(275)), A::offset(s.ad_value(275), (p.p67 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[547] != 0.0) {
            s.store_div_ad(275, A::mul(A::scale(s.ad_value(246), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[549] = if (s.v[275] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[550] = if (s.v[275] < p.p147) { 1.0 } else { 0.0 };

        if (((s.v[547] != 0.0) && (s.v[549] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_exp(91, 275);
        }

        if (((s.v[547] != 0.0) && (s.v[549] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (((s.v[547] != 0.0) && (s.v[549] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.store_mul_ad_rhs(91, 295, A::offset(A::offset(s.ad_value(275), (-p.p147)), 1.0));
        }

        if ((s.v[547] != 0.0) && (s.v[549] != 0.0)) {
            s.store_mul_ad(69, A::neg(s.ad_value(246)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(275)), 1.0));
        }

        if ((s.v[547] != 0.0) && (!(s.v[549] != 0.0))) {
            s.store_mul_ad(69, A::mul(A::scale(s.ad_value(246), 0.5), s.ad_value(275)), A::offset(A::mul(A::scale(s.ad_value(275), 0.3333333333333333), A::offset(A::scale(s.ad_value(275), 0.25), 1.0)), 1.0));
        }

        if (s.v[547] != 0.0) {
            s.store_scale_ad(57, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(58), 2.0), s.ad_value(69)), s.ad_value(59)), s.ad_value(68)), s.ad_value(65)), s.v[63]);
        }

        if (!(s.v[547] != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        if (!(s.v[547] != 0.0)) {
            s.store_scalar(57, 0.0);
        }

        s.v[551] = if (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[244] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[551] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(244), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[552] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[551] != 0.0) && (s.v[552] != 0.0)) {
            s.store_exp_ad(78, A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))));
        }

        if ((s.v[551] != 0.0) && (!(s.v[552] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((s.v[551] != 0.0) && (!(s.v[552] != 0.0))) {
            s.store_mul_ad_rhs(78, 295, A::offset(A::offset(A::mul(s.ad_value(83), A::sub_from_scalar(1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0)))), (-p.p147)), 1.0));
        }

        if (s.v[551] != 0.0) {
            s.store_mul(277, 244, 67);
        }

        if (s.v[551] != 0.0) {
            let assign4580_ad_e4435: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(277)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(277), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(277), 6.0), s.ad_value(277)), A::offset(s.ad_value(277), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4580_ad_e4435);
        }

        if (s.v[551] != 0.0) {
            s.store_div_ad(277, A::mul(A::scale(s.ad_value(244), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[553] = if (s.v[277] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[554] = if (s.v[277] < p.p147) { 1.0 } else { 0.0 };

        if (((s.v[551] != 0.0) && (s.v[553] != 0.0)) && (s.v[554] != 0.0)) {
            s.store_exp(92, 277);
        }

        if (((s.v[551] != 0.0) && (s.v[553] != 0.0)) && (!(s.v[554] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (((s.v[551] != 0.0) && (s.v[553] != 0.0)) && (!(s.v[554] != 0.0))) {
            s.store_mul_ad_rhs(92, 295, A::offset(A::offset(s.ad_value(277), (-p.p147)), 1.0));
        }

        if ((s.v[551] != 0.0) && (s.v[553] != 0.0)) {
            s.store_mul_ad(81, A::neg(s.ad_value(244)), A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(277)), 1.0));
        }

        if ((s.v[551] != 0.0) && (!(s.v[553] != 0.0))) {
            s.store_mul_ad(81, A::mul(A::scale(s.ad_value(244), 0.5), s.ad_value(277)), A::offset(A::mul(A::scale(s.ad_value(277), 0.3333333333333333), A::offset(A::scale(s.ad_value(277), 0.25), 1.0)), 1.0));
        }

        if (s.v[551] != 0.0) {
            s.store_scale_ad(82, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(84), 2.0), s.ad_value(81)), s.ad_value(77)), s.ad_value(78)), s.ad_value(67)), s.v[89]);
        }

        if (!(s.v[551] != 0.0)) {
            s.store_scalar(81, 0.0);
        }

        if (!(s.v[551] != 0.0)) {
            s.store_scalar(82, 0.0);
        }

        s.store_mul(165, 146, 268);

        s.store_scale(166, 270, 4.0);

        s.store_div_ad(168, A::sub(s.ad_value(165), s.ad_value(146)), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0));

        s.store_div_ad_rhs(167, 166, A::offset(A::sqrt(A::offset(s.ad_value(166), 1.0)), 1.0));

        s.store_div_ad(164, A::scale(A::offset(s.ad_value(268), (-1.0)), (2.0 * s.v[43])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(268), ((4.0 * s.v[43]) / s.v[37])), 1.0)), 1.0));

        s.v[555] = if (p.p8 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[555] != 0.0) {
            s.store_div_ad(182, A::scale(A::sub(s.ad_value(265), s.ad_value(256)), ((p.p143 * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(A::add(s.ad_value(265), A::scale(s.ad_value(256), p.p144)), (4.0 * (s.v[104] / s.v[106]))), 1.0)), 1.0));
        }

        if (s.v[555] != 0.0) {
            s.store_div_ad(179, A::scale(A::sub(s.ad_value(268), s.ad_value(258)), (((1.0 - p.p143) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(A::add(s.ad_value(268), A::scale(s.ad_value(258), p.p144)), (4.0 * (s.v[104] / s.v[106]))), 1.0)), 1.0));
        }

        if (!(s.v[555] != 0.0)) {
            s.store_div_ad(182, A::scale(A::offset(s.ad_value(265), (-1.0)), ((p.p143 * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(265), (4.0 * (s.v[104] / s.v[106]))), 1.0)), 1.0));
        }

        if (!(s.v[555] != 0.0)) {
            s.store_div_ad(179, A::scale(A::offset(s.ad_value(268), (-1.0)), (((1.0 - p.p143) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(268), (4.0 * (s.v[104] / s.v[106]))), 1.0)), 1.0));
        }

        s.store_add_ad(181, A::div(A::scale(A::offset(s.ad_value(256), (-1.0)), (2.0 * s.v[105])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(256), ((p.p144 * 4.0) * (s.v[105] / s.v[107]))), 1.0)), 1.0)), A::mul(s.ad_value(253), s.ad_value(342)));

        s.v[180] = 0.0;

        s.v[556] = if ((p.p5 > 0.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_scale(164, 164, s.v[157]);
        }

        if (s.v[556] != 0.0) {
            s.store_scale(179, 179, s.v[157]);
        }

        if (s.v[556] != 0.0) {
            s.store_div_ad(171, A::scale(A::offset(s.ad_value(269), (-1.0)), ((p.p33 * 2.0) * s.v[43])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(269), ((4.0 * s.v[43]) / s.v[37])), 1.0)), 1.0));
        }

        s.v[557] = if (p.p8 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[556] != 0.0) && (s.v[557] != 0.0)) {
            s.store_div_ad(172, A::scale(A::sub(s.ad_value(269), s.ad_value(257)), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(A::add(s.ad_value(269), A::scale(s.ad_value(257), p.p144)), ((4.0 * s.v[104]) / s.v[106])), 1.0)), 1.0));
        }

        if ((s.v[556] != 0.0) && (!(s.v[557] != 0.0))) {
            s.store_div_ad(172, A::scale(A::offset(s.ad_value(269), (-1.0)), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106])), 1.0)), 1.0));
        }

        s.v[558] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_scalar(291, ((p.p33 * (s.v[43] + s.v[104])) * s.v[32]));
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_scale_ad(173, A::sub_from_scalar(2.0, A::ln(A::scale(s.ad_value(291), s.v[8]))), s.v[6]);
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_sub(284, 261, 173);
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_scalar(281, (0.11 * 0.11));
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_square(282, 284);
        }

        s.v[559] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[556] != 0.0) && (s.v[558] != 0.0)) && (s.v[559] != 0.0)) {
            s.store_div_ad(174, A::scale(s.ad_value(281), 0.5), A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)));
        }

        if (((s.v[556] != 0.0) && (s.v[558] != 0.0)) && (!(s.v[559] != 0.0))) {
            s.store_scale_ad(174, A::add(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)), 0.5);
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_div_ad_rhs(175, 174, A::add(A::add(s.ad_value(291), A::scale(A::add(s.ad_value(171), s.ad_value(172)), s.v[32])), s.ad_value(174)));
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(173, 0.0);
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(174, 0.0);
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(175, 1.0);
        }

        if (s.v[556] != 0.0) {
            s.store_mul(176, 175, 171);
        }

        if (s.v[556] != 0.0) {
            s.store_mul(180, 175, 172);
        }

        s.v[560] = if (p.p84 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[560] != 0.0) {
            s.store_add(347, 248, 244);
        }

        if (s.v[560] != 0.0) {
            s.store_scalar(281, (1e-6 * 1e-6));
        }

        if (s.v[560] != 0.0) {
            s.store_mul_ad_lhs(282, A::scale(s.ad_value(347), ((-1.0) * (-1.0))), 347);
        }

        s.v[561] = if (((-1.0) * s.v[347]) < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[560] != 0.0) && (s.v[561] != 0.0)) {
            s.store_div_ad(348, A::scale(s.ad_value(281), 0.5), A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), A::scale(s.ad_value(347), (-1.0))));
        }

        if ((s.v[560] != 0.0) && (!(s.v[561] != 0.0))) {
            s.store_scale_ad(348, A::add(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), A::scale(s.ad_value(347), (-1.0))), 0.5);
        }

        if (s.v[560] != 0.0) {
            s.store_scalar(349, (1.0 / (1.0 - ((s.v[343]) as f64).powf(p.p82))));
        }

        if (s.v[560] != 0.0) {
            s.store_scalar(344, (s.v[343] * p.p81));
        }

        if (s.v[560] != 0.0) {
            s.store_scale_ad(346, A::square(s.ad_value(349)), (((s.v[343]) as f64).powf((p.p82 - 1.0)) * (p.p82 * 1.0 / (p.p81))));
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
        s.v[562] = if (s.v[348] < s.v[344]) { 1.0 } else { 0.0 };

        if ((s.v[560] != 0.0) && (s.v[562] != 0.0)) {
            s.store_div_from_scalar_ad(345, 1.0, A::sub_from_scalar(1.0, A::powf(A::scale(s.ad_value(348), 1.0 / (p.p81)), p.p82)));
        }

        if ((s.v[560] != 0.0) && (!(s.v[562] != 0.0))) {
            s.store_add_ad_rhs(345, 349, A::mul(A::sub(s.ad_value(348), s.ad_value(344)), s.ad_value(346)));
        }

        if (!(s.v[560] != 0.0)) {
            s.store_scalar(345, 1.0);
        }

        s.store_mul(82, 82, 345);

        s.store_mul(164, 164, 345);

        s.store_mul(161, 161, 345);

        s.store_mul(176, 176, 345);

        s.store_add_ad(183, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 183);

        s.v[563] = if (s.v[183] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[563] != 0.0) {
            s.store_div_from_scalar_ad(184, (0.5 * s.v[281]), A::sub(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(183)));
        }

        if (!(s.v[563] != 0.0)) {
            s.store_scale_ad(184, A::add(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(183)), 0.5);
        }

        s.store_mul_ad_rhs(185, 184, A::offset(A::scale(A::add(s.ad_value(149), s.ad_value(150)), 0.5), 1.0));

        s.store_div_from_scalar(187, s.v[29], 185);

        s.v[564] = if (s.v[187] < s.v[340]) { 1.0 } else { 0.0 };

        if (s.v[564] != 0.0) {
            s.copy_ad(187, 340);
        }

        s.store_scale(186, 187, 3.0);

        s.store_div_ad_lhs(188, A::add(A::scale(A::offset(s.ad_value(267), (-1.0)), (2.0 * s.v[6])), s.ad_value(248)), 186);

        s.v[565] = if (s.v[156] > 0.0) { 1.0 } else { 0.0 };

        s.v[566] = if (p.p39 == 1.0) { 1.0 } else { 0.0 };

        s.v[567] = if (s.v[244] < p.p44) { 1.0 } else { 0.0 };

        s.v[568] = if (((-s.v[156]) / p.p42) < p.p147) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (s.v[568] != 0.0)) {
            s.store_exp_ad(332, A::scale(A::neg(s.ad_value(156)), 1.0 / (p.p42)));
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[568] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[568] != 0.0))) {
            s.store_mul_ad_rhs(332, 295, A::offset(A::offset(A::scale(A::neg(s.ad_value(156)), 1.0 / (p.p42)), (-p.p147)), 1.0));
        }

        if (((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) {
            s.store_mul_ad_lhs(333, A::sub_from_scalar(p.p44, s.ad_value(244)), 332);
        }

        s.v[569] = if (((-s.v[334]) * ((s.v[333]) as f64).powf(p.p41)) < p.p147) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (s.v[569] != 0.0)) {
            s.store_exp_ad(337, A::mul(A::neg(s.ad_value(334)), A::powf(s.ad_value(333), p.p41)));
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[569] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[569] != 0.0))) {
            s.store_mul_ad_rhs(337, 295, A::offset(A::offset(A::mul(A::neg(s.ad_value(334)), A::powf(s.ad_value(333), p.p41)), (-p.p147)), 1.0));
        }

        if (((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) {
            s.store_mul_ad_lhs(207, A::mul(A::div_from_scalar(p.p40, s.ad_value(334)), s.ad_value(333)), 337);
        }

        s.v[570] = if (p.p39 == 2.0) { 1.0 } else { 0.0 };

        s.v[571] = if (s.v[244] < s.v[16]) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_scalar(196, ((2.0 * p.p46) / (p.p45 * p.p45)));
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad_lhs(280, A::sub(s.ad_value(16), s.ad_value(244)), 210);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_sqrt_ad(197, A::div(A::scale(s.ad_value(280), 2.0), s.ad_value(196)));
        }

        s.v[572] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[572] != 0.0)) {
            s.store_scalar(198, p.p45);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
            s.store_sub_from_scalar_ad(123, 1.0, A::scale(s.ad_value(122), 0.5));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
            s.store_mul_ad_lhs(198, A::scale(s.ad_value(123), p.p45), 123);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad(199, A::mul(s.ad_value(197), s.ad_value(198)), A::sqrt(A::add(A::square(s.ad_value(197)), A::square(s.ad_value(198)))));
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad_lhs(200, A::sub(s.ad_value(16), s.ad_value(244)), 199);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_add_ad_rhs(201, 200, A::mul(A::mul(A::scale(s.ad_value(199), 0.5), s.ad_value(196)), s.ad_value(210)));
        }

        s.v[573] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[573] != 0.0)) {
            s.copy_ad(202, 201);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_offset_ad(203, A::scale(A::offset(A::scale(s.ad_value(122), 2.0), 1.0), (2.0 * p.p47)), 1.0);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_scalar(204, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_sub_ad_rhs(205, 200, A::mul(A::mul(A::scale(s.ad_value(199), 0.5), s.ad_value(196)), A::sub(s.ad_value(204), A::div(s.ad_value(156), A::scale(s.ad_value(203), p.p62)))));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_add_ad(280, A::mul(A::sub(s.ad_value(205), s.ad_value(201)), A::sub(s.ad_value(205), s.ad_value(201))), A::scale(A::mul(A::mul(A::scale(s.ad_value(200), 0.1), s.ad_value(200)), s.ad_value(134)), 1.0 / (p.p62)));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_scale_ad(202, A::add(A::add(s.ad_value(205), s.ad_value(201)), A::sqrt(s.ad_value(280))), 0.5);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad_lhs(287, A::sub(s.ad_value(202), s.ad_value(200)), 202);
        }

        s.v[574] = if (((s.v[287]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[574] != 0.0)) {
            s.store_div_ad_lhs(206, A::scale(s.ad_value(199), 0.5), 287);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[574] != 0.0)) {
            s.store_mul_ad(207, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(202)), s.ad_value(206)), A::sub(A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(202))), A::exp(A::mul(A::div(A::neg(s.ad_value(98)), s.ad_value(202)), A::offset(A::div(s.ad_value(198), s.ad_value(206)), 1.0)))));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[574] != 0.0))) {
            s.store_mul_ad(207, A::mul(s.ad_value(0), s.ad_value(198)), A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(202))));
        }

        s.v[575] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        s.v[576] = if (s.v[244] < p.p44) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) {
            s.store_mul_ad(211, A::powf(A::sub_from_scalar(p.p44, s.ad_value(244)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(156), A::offset(s.ad_value(156), p.p48))), p.p49));
        }

        s.v[577] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
            s.copy_ad(212, 211);
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
            s.store_scaled_offset(213, 156, (-p.p52), 1.0 / (p.p48));
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
            s.store_scaled_offset(279, 213, (-1.0), 1.0 / (p.p51));
        }

        s.v[578] = if (s.v[213] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) && (s.v[578] != 0.0)) {
            s.store_offset_ad(214, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), p.p51), 1.0);
        }

        if (((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) && (!(s.v[578] != 0.0))) {
            s.store_add_ad_rhs(214, 213, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), p.p51));
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
            s.store_mul_ad_rhs(212, 211, A::powf(s.ad_value(214), p.p50));
        }

        s.v[579] = if (((-s.v[334]) * s.v[212]) < p.p147) { 1.0 } else { 0.0 };

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (s.v[579] != 0.0)) {
            s.store_exp_ad(337, A::mul(A::neg(s.ad_value(334)), s.ad_value(212)));
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_mul_ad_rhs(337, 295, A::offset(A::offset(A::mul(A::neg(s.ad_value(334)), s.ad_value(212)), (-p.p147)), 1.0));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) {
            s.store_mul_ad_lhs(207, A::mul(A::div_from_scalar(p.p40, s.ad_value(334)), A::sub_from_scalar(p.p44, s.ad_value(244))), 337);
        }

        s.v[580] = if (s.v[207] > 0.0) { 1.0 } else { 0.0 };

        s.v[581] = if (p.p53 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_add_ad(208, A::add(A::div_from_scalar(s.v[6], A::mul(s.ad_value(156), A::add(s.ad_value(30), s.ad_value(186)))), A::scale(A::div(s.ad_value(153), s.ad_value(35)), s.v[42])), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(186))));
        }

        s.v[582] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
            s.store_scaled_sub(279, 207, 208, 1000000.0);
        }

        s.v[583] = if (s.v[207] < s.v[208]) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) && (s.v[583] != 0.0)) {
            s.store_sub_ad_rhs(207, 207, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 1e-6));
        }

        if (((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_sub_ad_rhs(207, 208, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 1e-6));
        }

        if ((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
            s.store_mul(209, 156, 207);
        }

        if ((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
            s.store_div_ad(209, A::mul(A::mul(s.ad_value(156), s.ad_value(207)), s.ad_value(208)), A::add(s.ad_value(207), s.ad_value(208)));
        }

        if (((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (!(s.v[581] != 0.0))) {
            s.store_mul(209, 156, 207);
        }

        s.store_mul_ad_lhs(215, A::scale(s.ad_value(23), (1.0 - p.p68)), 138);

        s.store_div_ad_lhs(279, A::sub(s.ad_value(247), s.ad_value(136)), 293);

        s.v[585] = if (s.v[247] < s.v[136]) { 1.0 } else { 0.0 };

        if (s.v[585] != 0.0) {
            s.store_sub_ad_rhs(216, 247, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[585] != 0.0)) {
            s.store_sub_ad_rhs(216, 136, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_mul_ad(217, A::scale(s.ad_value(23), p.p68), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(65))), (1.0 - p.p67)))), A::scale(A::sub(s.ad_value(247), s.ad_value(216)), 3.0)));

        s.store_mul_ad_lhs(218, A::scale(s.ad_value(24), p.p77), 145);

        s.v[219] = (s.v[94] * s.v[36]);

        s.store_mul_ad_lhs(223, A::scale(s.ad_value(149), (0.5 * s.v[219])), 184);

        s.store_mul_ad_lhs(224, A::scale(s.ad_value(150), (0.5 * s.v[219])), 184);

        s.store_scale(294, 17, 0.1);

        s.store_div_ad_lhs(279, A::sub(s.ad_value(249), s.ad_value(141)), 294);

        s.v[586] = if (s.v[249] < s.v[141]) { 1.0 } else { 0.0 };

        if (s.v[586] != 0.0) {
            s.store_sub_ad_rhs(225, 249, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[586] != 0.0)) {
            s.store_sub_ad_rhs(225, 141, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_add_ad(226, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(225), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(140), A::sub(s.ad_value(249), s.ad_value(225))));

        s.store_scale_ad(227, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(226)), A::mul(s.ad_value(25), s.ad_value(249)))), ((1.0 - p.p77) * (1.0 - p.p33)));

        s.store_div_ad_lhs(279, A::sub(s.ad_value(261), s.ad_value(141)), 294);

        s.v[587] = if (s.v[261] < s.v[141]) { 1.0 } else { 0.0 };

        if (s.v[587] != 0.0) {
            s.store_sub_ad_rhs(228, 261, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[587] != 0.0)) {
            s.store_sub_ad_rhs(228, 141, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_add_ad(229, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(228), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(140), A::sub(s.ad_value(261), s.ad_value(228))));

        s.store_scale_ad(230, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(229)), A::mul(s.ad_value(25), s.ad_value(261)))), ((1.0 - p.p77) * p.p33));

        s.store_scale(301, 102, 0.1);

        s.store_scale(231, 102, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_ad_lhs(279, A::sub(s.ad_value(253), s.ad_value(231)), 301);

        s.v[588] = if (s.v[253] < s.v[231]) { 1.0 } else { 0.0 };

        if (s.v[588] != 0.0) {
            s.store_sub_ad_rhs(232, 253, A::mul(s.ad_value(301), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[588] != 0.0)) {
            s.store_sub_ad_rhs(232, 231, A::mul(s.ad_value(301), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_mul_ad_rhs(233, 103, A::add(A::mul(A::scale(s.ad_value(102), 1.0 / ((1.0 - p.p139))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(102))), (1.0 - p.p139)))), A::scale(A::sub(s.ad_value(253), s.ad_value(232)), 2.0)));

        s.store_scale_ad(234, A::powf(A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p85)), (s.v[93] * s.v[36]));

        s.v[589] = if ((s.v[246] / (p.p85 * s.v[6])) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[589] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(246), 1.0 / ((p.p85 * s.v[6]))));
        }

        if (!(s.v[589] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[589] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(246), 1.0 / ((p.p85 * s.v[6]))), (-p.p147)), 1.0));
        }

        s.store_mul(236, 234, 296);

        s.v[237] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_ad(238, A::scale(s.ad_value(122), (0.5 * s.v[237])), A::offset(A::add(s.ad_value(126), s.ad_value(113)), 2.0));

        s.v[590] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[590] != 0.0) {
            s.store_scale_ad(243, A::add(A::scale(s.ad_value(168), s.v[219]), A::scale(s.ad_value(167), s.v[237])), ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[591] = if ((((s.v[249] - s.v[22]) / p.p91) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if ((!(s.v[590] != 0.0)) && (s.v[591] != 0.0)) {
            s.store_exp_ad(177, A::scale(A::scale(A::sub(s.ad_value(249), s.ad_value(22)), 1.0 / (p.p91)), s.v[8]));
        }

        if ((!(s.v[590] != 0.0)) && (!(s.v[591] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((!(s.v[590] != 0.0)) && (!(s.v[591] != 0.0))) {
            s.store_mul_ad_rhs(177, 295, A::offset(A::offset(A::scale(A::scale(A::sub(s.ad_value(249), s.ad_value(22)), 1.0 / (p.p91)), s.v[8]), (-p.p147)), 1.0));
        }

        if (!(s.v[590] != 0.0)) {
            s.store_div_ad(243, A::scale(s.ad_value(268), ((2.0 * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(177), 4.0), 1.0)), 1.0));
        }

        s.v[592] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[592] != 0.0) {
            s.store_scale(243, 243, s.v[157]);
        }

        s.v[593] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_mul(169, 146, 269);
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_div_ad(170, A::sub(s.ad_value(169), s.ad_value(146)), A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0));
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_scale(239, 272, 4.0);
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_div_ad_rhs(240, 239, A::offset(A::sqrt(A::offset(s.ad_value(239), 1.0)), 1.0));
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_scale_ad(241, A::add(A::scale(s.ad_value(170), s.v[219]), A::scale(s.ad_value(240), s.v[237])), (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[594] = if (((s.v[261] - s.v[22]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) && (s.v[594] != 0.0)) {
            s.store_exp_ad(178, A::scale(A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]));
        }

        if (((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) && (!(s.v[594] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) && (!(s.v[594] != 0.0))) {
            s.store_mul_ad_rhs(178, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]), (-p.p147)), 1.0));
        }

        if ((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) {
            s.store_div_ad(241, A::scale(s.ad_value(269), (((2.0 * p.p33) * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(178), 4.0), 1.0)), 1.0));
        }

        if (s.v[592] != 0.0) {
            s.store_mul(242, 175, 241);
        }

        s.v[595] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[595] != 0.0) {
            s.store_offset_ad(190, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (-p.p67)), (-3.0));
        }

        if (s.v[595] != 0.0) {
            s.store_div_ad_lhs(288, A::sub(s.ad_value(246), s.ad_value(136)), 293);
        }

        s.v[596] = if (s.v[288] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
            s.store_div_from_scalar_ad(191, 1.0, A::offset(A::exp(s.ad_value(288)), 1.0));
        }

        if ((s.v[595] != 0.0) && (!(s.v[596] != 0.0))) {
            s.store_div_ad(191, A::exp(A::neg(s.ad_value(288))), A::offset(A::exp(A::neg(s.ad_value(288))), 1.0));
        }

        if (s.v[595] != 0.0) {
            s.store_offset_ad(189, A::mul(s.ad_value(190), s.ad_value(191)), 3.0);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_lhs(192, A::scale(s.ad_value(23), (1.0 - p.p68)), 189);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad(195, A::div(A::scale(A::mul(s.ad_value(146), s.ad_value(266)), s.v[8]), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(147), 1.0))));
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_lhs(193, A::scale(s.ad_value(184), (0.5 * s.v[219])), 195);
        }

        if (s.v[595] != 0.0) {
            s.store_scale(194, 236, 1.0 / ((p.p85 * s.v[6])));
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad(222, A::scale(s.ad_value(248), 0.2), A::add(A::add(s.ad_value(192), s.ad_value(193)), s.ad_value(194)));
        }

        if (s.v[595] != 0.0) {
            s.store_scale(235, 236, (1.0 - p.p95));
        }

        if (s.v[595] != 0.0) {
            s.store_add_ad_rhs(331, 223, A::scale(s.ad_value(236), p.p95));
        }

        if (s.v[595] != 0.0) {
            s.store_add_ad_lhs(221, A::scale(s.ad_value(331), p.p94), 224);
        }

        if (s.v[595] != 0.0) {
            s.store_scale(220, 331, (1.0 - p.p94));
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(220, 223);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(221, 224);
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
        if (!(s.v[595] != 0.0)) {
            s.copy_ad(235, 236);
        }

        s.v[597] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[598] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.v[599] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

        s.v[302] = ((4.0 * 1.3806226e-23) * s.v[2]);

        s.store_div_from_scalar(303, s.v[302], 28);

        s.store_div_from_scalar(304, s.v[302], 30);

        s.store_scale(305, 108, s.v[302]);

        s.store_scale(306, 109, s.v[302]);

        s.store_scale(307, 110, s.v[302]);

        s.store_scale_ad(308, A::mul(A::div_from_scalar(s.v[302], s.ad_value(186)), A::offset(A::scale(s.ad_value(267), 4.0), 5.0)), 0.3333333333333333);

        s.store_div_ad_lhs(327, A::add(s.ad_value(155), s.ad_value(154)), 153);

        s.store_scale_ad(309, A::abs(s.ad_value(327)), (2.0 * 1.6021918e-19));

        s.v[600] = if (p.p130 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[600] != 0.0) {
            s.store_abs_ad(328, A::div(s.ad_value(209), s.ad_value(327)));
        }

        if (!(s.v[600] != 0.0)) {
            s.store_scalar(328, 0.0);
        }

        s.store_mul_ad(321, A::scale(s.ad_value(209), (2.0 * 1.6021918e-19)), A::offset(s.ad_value(328), 1.0));

        s.v[601] = if (s.v[327] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[601] != 0.0) {
            s.store_div_ad_lhs(329, A::add(s.ad_value(220), s.ad_value(221)), 327);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_mul_ad_lhs(329, A::scale(s.ad_value(184), s.v[94]), 153);
        }

        s.v[602] = if (p.p131 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[602] != 0.0) {
            s.store_scale(330, 329, p.p94);
        }

        s.v[603] = if (p.p131 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[602] != 0.0)) && (s.v[603] != 0.0)) {
            s.store_scale(330, 329, p.p132);
        }

        if ((!(s.v[602] != 0.0)) && (!(s.v[603] != 0.0))) {
            s.store_scalar(330, 0.0);
        }

        s.store_scale_ad(310, A::abs(A::add(A::add(A::sub(A::add(s.ad_value(158), s.ad_value(160)), s.ad_value(57)), s.ad_value(352)), s.ad_value(351))), (2.0 * 1.6021918e-19));

        s.store_add(322, 158, 159);

        s.store_scale_ad(311, A::powf(A::abs(s.ad_value(322)), p.p126), p.p128);

        s.v[604] = if (s.v[322] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[604] != 0.0) {
            s.store_neg(311, 311);
        }

        s.store_add_ad_lhs(323, A::add(s.ad_value(160), s.ad_value(162)), 163);

        s.store_scale_ad(312, A::powf(A::abs(s.ad_value(323)), p.p127), p.p129);

        s.v[605] = if (s.v[323] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[605] != 0.0) {
            s.store_neg(312, 312);
        }

        s.store_scale_ad(313, A::abs(A::add(A::add(s.ad_value(159), s.ad_value(162)), s.ad_value(163))), (2.0 * 1.6021918e-19));

        s.store_scale_ad(314, A::abs(s.ad_value(161)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(315, A::powf(A::abs(s.ad_value(161)), p.p126), p.p128);

        s.v[606] = if (s.v[161] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[606] != 0.0) {
            s.store_neg(315, 315);
        }

        s.store_scale_ad(316, A::abs(s.ad_value(82)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(317, A::abs(s.ad_value(164)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(319, A::powf(A::scale(A::abs(s.ad_value(164)), 1.0 / ((1.0 - (p.p5 * p.p33)))), p.p126), (p.p128 * (1.0 - (p.p5 * p.p33))));

        s.v[607] = if (s.v[164] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[607] != 0.0) {
            s.store_neg(319, 319);
        }

        s.store_scale_ad(318, A::abs(s.ad_value(176)), ((2.0 * 1.6021918e-19) * p.p5));

        s.v[608] = if (p.p33 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[608] != 0.0) {
            s.store_scalar(320, 0.0);
        }

        if (!(s.v[608] != 0.0)) {
            s.store_scale_ad(320, A::powf(A::scale(A::abs(s.ad_value(176)), 1.0 / (p.p33)), p.p126), ((p.p128 * p.p5) * p.p33));
        }

        s.v[609] = if (s.v[176] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[609] != 0.0) {
            s.store_neg(320, 320);
        }

        s.store_scale_ad(324, A::abs(s.ad_value(182)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(325, A::abs(s.ad_value(179)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(326, A::abs(s.ad_value(180)), (2.0 * 1.6021918e-19));

        s.v[610] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[611] = if (p.p58 > 0.0) { 1.0 } else { 0.0 };

        s.v[612] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

        s.v[613] = if (p.p59 > 0.0) { 1.0 } else { 0.0 };

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
        s.v[476] = if (p.p3 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[476] != 0.0) {
            s.store_scalar(0, 70300000.0);
        }

        if (s.v[476] != 0.0) {
            s.store_scalar(1, 123000000.0);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(0, 158000000.0);
        }

        if (!(s.v[476] != 0.0)) {
            s.store_scalar(1, 204000000.0);
        }

        s.v[157] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx.temperature() + p.p0);

        s.v[477] = if (p.p150 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[477] != 0.0) {
            s.store_scalar(339, 1e-12);
        }

        if (!(s.v[477] != 0.0)) {
            s.store_scalar(339, p.p150);
        }

        s.store_scale(340, 339, p.p1);

        s.v[52] = 0.001;

        s.v[336] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[279] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.v[479] = if ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[479] != 0.0) {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[479] != 0.0)) {
            s.store_scalar(74, ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) + (0.1 * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p114;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p71;

        s.v[76] = p.p72;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[279] = (((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) - 0.05) / 0.1);

        s.v[480] = if ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[480] != 0.0) {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[480] != 0.0)) {
            s.store_scalar(88, ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) + (0.1 * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p117;

        s.v[86] = (1.0 / s.v[87]);

        s.v[175] = 1.0;

        s.v[207] = 0.0;

        s.v[242] = 0.0;

        s.v[222] = 0.0;

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

        s.v[274] = ((s.v[4]) as f64).ln();

        s.store_scale_ad(279, A::offset(A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), (-0.05)), 10.0);

        s.v[481] = if ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[481] != 0.0) {
            s.store_offset_ad(70, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[481] != 0.0)) {
            s.store_add_ad(70, A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 0.1));
        }

        s.store_scale_ad(279, A::offset(A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), (-0.05)), 10.0);

        s.v[482] = if ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05) { 1.0 } else { 0.0 };

        if (s.v[482] != 0.0) {
            s.store_offset_ad(85, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 0.1), 0.05);
        }

        if (!(s.v[482] != 0.0)) {
            s.store_add_ad(85, A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p66 * s.v[4])) + ((1.0 - s.v[4]) * p.p105));

        s.v[279] = ((0.05 - s.v[13]) / s.v[6]);

        s.v[483] = if (0.05 < s.v[13]) { 1.0 } else { 0.0 };

        if (s.v[483] != 0.0) {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[483] != 0.0)) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p64 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[15]) / s.v[6]);

        s.v[484] = if (0.05 < s.v[15]) { 1.0 } else { 0.0 };

        if (s.v[484] != 0.0) {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[484] != 0.0)) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p80 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[21]) / s.v[6]);

        s.v[485] = if (0.05 < s.v[21]) { 1.0 } else { 0.0 };

        if (s.v[485] != 0.0) {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[485] != 0.0)) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p71 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[18]) / s.v[6]);

        s.v[486] = if (0.05 < s.v[18]) { 1.0 } else { 0.0 };

        if (s.v[486] != 0.0) {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[486] != 0.0)) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[274]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[20]) / s.v[6]);

        s.v[487] = if (0.05 < s.v[20]) { 1.0 } else { 0.0 };

        if (s.v[487] != 0.0) {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[487] != 0.0)) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p27 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[279] = ((0.05 - s.v[56]) / s.v[6]);

        s.v[488] = if (0.05 < s.v[56]) { 1.0 } else { 0.0 };

        if (s.v[488] != 0.0) {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[488] != 0.0)) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[101] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p138 * s.v[4])) + ((1.0 - s.v[4]) * p.p140));

        s.v[279] = ((0.05 - s.v[101]) / s.v[6]);

        s.v[489] = if (0.05 < s.v[101]) { 1.0 } else { 0.0 };

        if (s.v[489] != 0.0) {
            s.store_scalar(102, (s.v[101] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!(s.v[489] != 0.0)) {
            s.store_scalar(102, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scale_ad(103, A::powf(A::div_from_scalar(p.p138, s.ad_value(102)), p.p139), p.p137);

        s.store_offset_ad(26, A::scale(A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75)), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.v[28] = (p.p54 * (((s.v[274] * p.p97)) as f64).exp());

        s.v[490] = if (s.v[28] < s.v[340]) { 1.0 } else { 0.0 };

        if (s.v[490] != 0.0) {
            s.copy_ad(28, 340);
        }

        s.v[29] = (p.p56 * (((s.v[274] * (p.p98 - p.p96))) as f64).exp());

        s.v[30] = (p.p55 * (((s.v[274] * p.p101)) as f64).exp());

        s.v[491] = if (s.v[30] < s.v[340]) { 1.0 } else { 0.0 };

        if (s.v[491] != 0.0) {
            s.copy_ad(30, 340);
        }

        s.v[32] = (p.p57 * (((s.v[274] * p.p102)) as f64).exp());

        s.v[31] = (p.p60 * (((s.v[274] * p.p99)) as f64).exp());

        s.v[492] = if (p.p122 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[492] != 0.0) {
            s.store_scalar(50, (p.p10 * (1.0 + (s.v[12] * p.p122))));
        }

        if (s.v[492] != 0.0) {
            s.store_scaled_offset(279, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[493] = if (s.v[50] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[492] != 0.0) && (s.v[493] != 0.0)) {
            s.store_offset_ad(50, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[492] != 0.0) && (!(s.v[493] != 0.0))) {
            s.store_add_ad_rhs(50, 50, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), s.v[52]));
        }

        if (s.v[492] != 0.0) {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[492] != 0.0)) {
            s.store_scalar(48, p.p10);
        }

        s.v[494] = if (p.p123 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[494] != 0.0) {
            s.store_scalar(51, (p.p11 * (1.0 + (s.v[12] * p.p123))));
        }

        if (s.v[494] != 0.0) {
            s.store_scaled_offset(279, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.v[495] = if (s.v[51] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[494] != 0.0) && (s.v[495] != 0.0)) {
            s.store_offset_ad(51, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), s.v[52]), 1.0);
        }

        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_add_ad_rhs(51, 51, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), s.v[52]));
        }

        if (s.v[494] != 0.0) {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!(s.v[494] != 0.0)) {
            s.store_scalar(49, p.p11);
        }

        s.v[335] = (p.p43 * (1.0 + (p.p124 * s.v[12])));

        s.v[281] = (s.v[336] * s.v[336]);

        s.v[282] = (s.v[335] * s.v[335]);

        s.v[496] = if (s.v[335] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[496] != 0.0) {
            s.store_scalar(334, ((0.5 * s.v[281]) / ((((s.v[282] + s.v[281])) as f64).sqrt() - s.v[335])));
        }

        if (!(s.v[496] != 0.0)) {
            s.store_scalar(334, (0.5 * ((((s.v[282] + s.v[281])) as f64).sqrt() + s.v[335])));
        }

        s.store_mul_ad(35, A::scale(A::exp(A::div_from_scalar((s.v[274] * (((4.0 - p.p98) - p.p96) + p.p121)), s.ad_value(48))), p.p9), A::exp(A::div_from_scalar(((-p.p105) * s.v[10]), s.ad_value(48))));

        s.v[36] = (p.p12 * (((s.v[274] * (1.0 - p.p98))) as f64).exp());

        s.v[37] = (p.p30 * (((s.v[274] * (1.0 - p.p103))) as f64).exp());

        s.v[42] = ((p.p16 * ((((s.v[274] * ((4.0 - p.p97) + p.p121)) / p.p17)) as f64).exp()) * (((((-p.p111) * s.v[10]) / p.p17)) as f64).exp());

        s.v[43] = ((p.p29 * (((s.v[274] * ((4.0 - p.p103) + p.p121))) as f64).exp()) * ((((-p.p112) * s.v[10])) as f64).exp());

        s.store_powf_ad(275, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(276, 1.0, 73);

        s.store_scale_ad(61, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(70), p.p35), s.ad_value(70)), s.ad_value(275)), s.ad_value(276)), p.p66), s.ad_value(65)), (s.v[72] * s.v[72]));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(277, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(278, 1.0, 90);

        s.store_scale_ad(83, A::mul(A::scale(A::mul(A::mul(A::mul(A::scale(s.ad_value(85), p.p37), s.ad_value(85)), s.ad_value(277)), s.ad_value(278)), s.v[75]), s.ad_value(67)), (s.v[86] * s.v[86]));

        s.v[275] = (((s.v[274] * p.p96)) as f64).exp();

        s.store_scale(40, 27, (p.p14 * s.v[275]));

        s.store_scale(41, 276, (p.p13 * s.v[275]));

        s.v[104] = ((p.p133 * (((s.v[274] * (4.0 - p.p141))) as f64).exp()) * ((((-p.p140) * s.v[10])) as f64).exp());

        s.v[106] = (p.p135 * (((s.v[274] * (1.0 - p.p141))) as f64).exp());

        s.v[93] = ((p.p86 * (((s.v[274] * (p.p98 - 2.0))) as f64).exp()) * ((((-p.p120) * s.v[10])) as f64).exp());

        s.v[94] = (p.p87 * (((s.v[274] * ((p.p96 + p.p98) - 1.0))) as f64).exp());

        s.v[95] = (p.p88 * (((s.v[274] * (p.p99 - 1.0))) as f64).exp());

        s.v[96] = ((p.p89 * (s.v[94] + s.v[95])) / (p.p87 + p.p88));

        s.v[97] = (p.p90 * (((s.v[274] * (p.p100 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.v[498] = if (s.v[2] < 525.0) { 1.0 } else { 0.0 };

        if (s.v[498] != 0.0) {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!(s.v[498] != 0.0)) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p92 * (((s.v[274] * p.p96)) as f64).exp());

        s.store_ad(244, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(7)), p.p3));

        s.store_ad(245, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(8)), p.p3));

        s.store_ad(246, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(4)), p.p3));

        s.store_ad(247, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(4)), p.p3));

        s.store_ad(248, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(6)), p.p3));

        s.store_ad(253, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(7)), p.p3));

        s.store_ad(250, &A::scale(A::voltage(ctx, &nodes, Some(7), Some(8)), p.p3));

        s.store_ad(260, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(5)), p.p3));

        s.store_ad(263, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(2)), p.p3));

        s.store_ad(264, &A::scale(A::voltage(ctx, &nodes, Some(1), Some(0)), p.p3));

        s.store_ad(252, &A::scale(A::voltage(ctx, &nodes, Some(10), Some(7)), p.p3));

        s.store_ad(251, &A::scale(A::voltage(ctx, &nodes, Some(9), Some(10)), p.p3));

        s.store_sub_ad_lhs(249, A::sub(A::add(s.ad_value(248), s.ad_value(245)), s.ad_value(250)), 252);

        s.store_sub_ad_lhs(262, A::add(A::sub(s.ad_value(260), s.ad_value(264)), s.ad_value(249)), 251);

        s.store_add(261, 264, 262);

        s.store_sub(255, 253, 252);

        s.store_sub(254, 255, 251);

        s.v[505] = if ((s.v[245] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[505] != 0.0) {
            s.store_exp_ad(265, A::scale(s.ad_value(245), s.v[8]));
        }

        if (!(s.v[505] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[505] != 0.0)) {
            s.store_mul_ad_rhs(265, 295, A::offset(A::offset(A::scale(s.ad_value(245), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[506] = if (((s.v[246] * s.v[8]) / s.v[48]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[506] != 0.0) {
            s.store_exp_ad(266, A::div(A::scale(s.ad_value(246), s.v[8]), s.ad_value(48)));
        }

        if (!(s.v[506] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[506] != 0.0)) {
            s.store_mul_ad_rhs(266, 295, A::offset(A::offset(A::div(A::scale(s.ad_value(246), s.v[8]), s.ad_value(48)), (-p.p147)), 1.0));
        }

        s.v[507] = if ((s.v[249] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[507] != 0.0) {
            s.store_exp_ad(268, A::scale(s.ad_value(249), s.v[8]));
        }

        if (!(s.v[507] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[507] != 0.0)) {
            s.store_mul_ad_rhs(268, 295, A::offset(A::offset(A::scale(s.ad_value(249), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[508] = if ((s.v[248] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (!(s.v[508] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.v[509] = if ((s.v[261] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[509] != 0.0) {
            s.store_exp_ad(269, A::scale(s.ad_value(261), s.v[8]));
        }

        if (!(s.v[509] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[509] != 0.0)) {
            s.store_mul_ad_rhs(269, 295, A::offset(A::offset(A::scale(s.ad_value(261), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[510] = if ((s.v[253] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (!(s.v[510] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.v[511] = if ((s.v[254] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[511] != 0.0) {
            s.store_exp_ad(257, A::scale(s.ad_value(254), s.v[8]));
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
        if (!(s.v[511] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[511] != 0.0)) {
            s.store_mul_ad_rhs(257, 295, A::offset(A::offset(A::scale(s.ad_value(254), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[512] = if ((s.v[255] * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (!(s.v[512] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.v[513] = if (((s.v[261] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[513] != 0.0) {
            s.store_exp_ad(272, A::scale(A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[513] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[513] != 0.0)) {
            s.store_mul_ad_rhs(272, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[514] = if (((s.v[249] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[514] != 0.0) {
            s.store_exp_ad(270, A::scale(A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[514] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[514] != 0.0)) {
            s.store_mul_ad_rhs(270, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[515] = if (((s.v[245] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[515] != 0.0) {
            s.store_exp_ad(271, A::scale(A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[515] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[515] != 0.0)) {
            s.store_mul_ad_rhs(271, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[516] = if (((s.v[244] - s.v[16]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[516] != 0.0) {
            s.store_exp_ad(273, A::scale(A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]));
        }

        if (!(s.v[516] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[516] != 0.0)) {
            s.store_mul_ad_rhs(273, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]), (-p.p147)), 1.0));
        }

        s.store_sqrt_ad(111, A::offset(A::scale(s.ad_value(271), 4.0), 1.0));

        s.store_sqrt_ad(112, A::offset(A::scale(s.ad_value(273), 4.0), 1.0));

        s.store_div_ad(113, A::scale(s.ad_value(273), 2.0), A::offset(s.ad_value(112), 1.0));

        s.v[517] = if (s.v[113] < p.p149) { 1.0 } else { 0.0 };

        if (s.v[517] != 0.0) {
            s.store_scalar(113, p.p149);
        }

        s.store_scale_ad(114, A::sub(A::sub(s.ad_value(111), s.ad_value(112)), A::ln(A::div(A::offset(s.ad_value(111), 1.0), A::offset(s.ad_value(112), 1.0)))), s.v[6]);

        s.store_scaled_add(115, 114, 250, 1.0 / (s.v[31]));

        s.v[518] = if (s.v[115] > 0.0) { 1.0 } else { 0.0 };

        s.v[519] = if (s.v[244] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[519] != 0.0)) {
            s.copy_ad(297, 244);
        }

        if ((s.v[518] != 0.0) && (!(s.v[519] != 0.0))) {
            s.store_offset_ad(297, A::ln(A::offset(A::offset(s.ad_value(244), (-100.0)), 1.0)), 100.0);
        }

        if (s.v[518] != 0.0) {
            s.store_sub_ad_lhs(116, A::add(s.ad_value(16), A::scale(A::ln(A::offset(A::scale(s.ad_value(115), (0.5 * (s.v[31] * s.v[8]))), 1.0)), (2.0 * s.v[6]))), 297);
        }

        if (s.v[518] != 0.0) {
            s.store_scale(292, 16, 0.2);
        }

        if (s.v[518] != 0.0) {
            s.store_square(281, 292);
        }

        if (s.v[518] != 0.0) {
            s.store_square(282, 116);
        }

        s.v[520] = if (s.v[116] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[520] != 0.0)) {
            s.store_div_ad(117, A::scale(s.ad_value(281), 0.5), A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)));
        }

        if ((s.v[518] != 0.0) && (!(s.v[520] != 0.0))) {
            s.store_scale_ad(117, A::add(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)), 0.5);
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(118, A::mul(s.ad_value(117), A::offset(s.ad_value(117), (p.p62 * p.p61))), A::scale(A::offset(s.ad_value(117), (p.p62 * s.v[31])), p.p61));
        }

        if (s.v[518] != 0.0) {
            s.store_div(285, 115, 118);
        }

        if (s.v[518] != 0.0) {
            s.store_scaled_offset(279, 285, (-1.0), 1.0 / (p.p63));
        }

        s.v[521] = if (s.v[285] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[521] != 0.0)) {
            s.store_offset_ad(283, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), p.p63), 1.0);
        }

        if ((s.v[518] != 0.0) && (!(s.v[521] != 0.0))) {
            s.store_add_ad_rhs(283, 285, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), p.p63));
        }

        if (s.v[518] != 0.0) {
            s.store_scale(119, 283, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
        }

        if (s.v[518] != 0.0) {
            s.store_scale(120, 117, 1.0 / ((p.p62 * p.p61)));
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(121, A::offset(A::sqrt(A::offset(A::mul(A::mul(A::scale(s.ad_value(119), 4.0), s.ad_value(120)), A::offset(s.ad_value(120), 1.0)), 1.0)), 1.0), A::mul(A::scale(s.ad_value(119), 2.0), A::offset(s.ad_value(120), 1.0)));
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(122, A::add(A::sub_from_scalar(1.0, s.ad_value(121)), A::mul(s.ad_value(113), s.ad_value(121))), A::offset(A::mul(s.ad_value(113), s.ad_value(121)), 1.0));
        }

        if (s.v[518] != 0.0) {
            s.store_scale_ad(124, A::mul(A::scale(s.ad_value(115), (0.5 * s.v[31])), s.ad_value(122)), s.v[8]);
        }

        if (s.v[518] != 0.0) {
            s.store_add_ad(286, A::scale(s.ad_value(124), 2.0), A::mul(s.ad_value(113), A::offset(A::add(s.ad_value(113), s.ad_value(124)), 1.0)));
        }

        if (s.v[518] != 0.0) {
            s.store_scaled_offset(125, 124, (-1.0), 0.5);
        }

        if (s.v[518] != 0.0) {
            s.store_add_ad_lhs(280, A::square(s.ad_value(125)), 286);
        }

        s.v[522] = if (s.v[124] >= 1.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[522] != 0.0)) {
            s.store_add_ad_rhs(126, 125, A::sqrt(s.ad_value(280)));
        }

        if ((s.v[518] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_div_ad_rhs(126, 286, A::sub(A::sqrt(s.ad_value(280)), s.ad_value(125)));
        }

        s.v[523] = if (s.v[126] < p.p148) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[523] != 0.0)) {
            s.store_scalar(126, p.p148);
        }

        if (s.v[518] != 0.0) {
            s.store_mul_ad(128, A::mul(s.ad_value(126), A::offset(s.ad_value(126), 1.0)), A::exp(A::scale(s.ad_value(16), s.v[8])));
        }

        if (s.v[518] != 0.0) {
            s.store_scaled_offset(130, 115, (-p.p62), (0.5 * p.p61));
        }

        if (s.v[518] != 0.0) {
            s.store_scale(131, 115, ((p.p61 * s.v[31]) * p.p62));
        }

        if (s.v[518] != 0.0) {
            s.store_add_ad_rhs(132, 130, A::sqrt(A::add(A::square(s.ad_value(130)), s.ad_value(131))));
        }

        s.v[524] = if (p.p73 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[518] != 0.0) && (s.v[524] != 0.0)) {
            s.store_scale(133, 17, 0.1);
        }

        if ((s.v[518] != 0.0) && (!(s.v[524] != 0.0))) {
            s.store_mul_ad_rhs(133, 17, A::offset(A::div(A::scale(s.ad_value(115), 2.0), A::add(s.ad_value(115), s.ad_value(118))), 0.1));
        }

        if (s.v[518] != 0.0) {
            s.store_div_ad(134, A::scale(s.ad_value(115), p.p62), A::offset(s.ad_value(115), p.p62));
        }

        if (s.v[518] != 0.0) {
            s.store_div_from_scalar_ad(210, p.p62, A::offset(s.ad_value(115), p.p62));
        }

        if (!(s.v[518] != 0.0)) {
            s.store_scalar(118, 0.0);
        }

        if (!(s.v[518] != 0.0)) {
            s.store_div_ad(126, A::scale(s.ad_value(271), 2.0), A::offset(s.ad_value(111), 1.0));
        }

        if (!(s.v[518] != 0.0)) {
            s.copy_ad(128, 265);
        }

        s.v[525] = if ((((s.v[250]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[114]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[111] + s.v[112])))) { 1.0 } else { 0.0 };

        if ((!(s.v[518] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_scaled_add(135, 126, 113, 0.5);
        }

        if ((!(s.v[518] != 0.0)) && (s.v[525] != 0.0)) {
            s.store_div_ad_rhs(122, 135, A::offset(s.ad_value(135), 1.0));
        }

        if ((!(s.v[518] != 0.0)) && (!(s.v[525] != 0.0))) {
            s.store_div_ad_rhs(122, 114, A::sub(A::add(s.ad_value(114), s.ad_value(245)), s.ad_value(244)));
        }

        if (!(s.v[518] != 0.0)) {
            s.copy_ad(132, 250);
        }

        if (!(s.v[518] != 0.0)) {
            s.store_scale(133, 17, 0.1);
        }

        if (!(s.v[518] != 0.0)) {
            s.copy_ad(134, 115);
        }

        if (!(s.v[518] != 0.0)) {
            s.store_sub_from_scalar_ad(210, 1.0, A::scale(s.ad_value(134), 1.0 / (p.p62)));
        }

        s.store_scale(136, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(293, 14, 0.1);

        s.store_div_ad_lhs(279, A::sub(s.ad_value(246), s.ad_value(136)), 293);

        s.v[526] = if (s.v[246] < s.v[136]) { 1.0 } else { 0.0 };

        if (s.v[526] != 0.0) {
            s.store_sub_ad_rhs(137, 246, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[526] != 0.0)) {
            s.store_sub_ad_rhs(137, 136, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (1.0 - p.p67));

        s.store_add_ad(138, A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, s.ad_value(59))), A::scale(A::sub(s.ad_value(246), s.ad_value(137)), 3.0));

        s.v[527] = if (p.p74 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[527] != 0.0) {
            s.copy_ad(139, 244);
        }

        s.v[528] = if (p.p74 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[527] != 0.0)) && (s.v[528] != 0.0)) {
            s.store_add(139, 244, 132);
        }

        if ((!(s.v[527] != 0.0)) && (!(s.v[528] != 0.0))) {
            s.copy_ad(139, 245);
        }

        s.store_div_ad(140, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_ad_rhs(141, 17, A::sub_from_scalar(1.0, A::powf(s.ad_value(140), ((-1.0) / p.p72))));

        s.store_div_ad_lhs(279, A::sub(s.ad_value(139), s.ad_value(141)), 133);

        s.v[529] = if (s.v[139] < s.v[141]) { 1.0 } else { 0.0 };

        if (s.v[529] != 0.0) {
            s.store_sub_ad_rhs(142, 139, A::mul(s.ad_value(133), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[529] != 0.0)) {
            s.store_sub_ad_rhs(142, 141, A::mul(s.ad_value(133), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_powf(143, 210, p.p76);

        s.store_add_ad(144, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::mul(s.ad_value(143), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(142), s.ad_value(17))), (1.0 - p.p72))))), A::mul(A::mul(s.ad_value(143), s.ad_value(140)), A::sub(s.ad_value(139), s.ad_value(142))));

        s.store_add_ad(145, A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(144)), A::mul(s.ad_value(25), s.ad_value(244)));

        s.store_scale(146, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(147, 146, 266);

        s.store_div_ad_rhs(149, 147, A::offset(A::sqrt(A::offset(s.ad_value(147), 1.0)), 1.0));

        s.store_ad(129, &A::pow(s.ad_value(128), A::div_from_scalar(1.0, s.ad_value(49))));

        s.store_mul(148, 146, 129);

        s.store_div_ad_rhs(150, 148, A::offset(A::sqrt(A::offset(s.ad_value(148), 1.0)), 1.0));

        s.v[530] = if (p.p92 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[530] != 0.0) {
            s.store_add_ad(151, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scale_ad(289, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), (s.v[99] * s.v[8]));
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scale_ad(290, A::div(A::neg(s.ad_value(145)), s.ad_value(40)), (s.v[99] * s.v[8]));
        }

        if (!(s.v[530] != 0.0)) {
            s.store_scale_ad(151, A::sub(A::exp(s.ad_value(289)), A::exp(s.ad_value(290))), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 151);

        s.v[531] = if (s.v[151] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[531] != 0.0) {
            s.store_div_from_scalar_ad(152, (0.5 * s.v[281]), A::sub(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(151)));
        }

        if (!(s.v[531] != 0.0)) {
            s.store_scale_ad(152, A::add(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(151)), 0.5);
        }

        s.store_mul_ad_rhs(153, 152, A::offset(A::scale(A::add(s.ad_value(149), s.ad_value(150)), 0.5), 1.0));

        s.store_mul_ad_lhs(154, A::scale(s.ad_value(35), p.p15), 129);

        s.store_mul(155, 35, 266);

        s.store_div_ad_lhs(156, A::sub(s.ad_value(155), s.ad_value(154)), 153);

        s.store_scale(279, 246, 10000.0);

        s.v[532] = if (s.v[246] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[532] != 0.0) {
            s.store_scale_ad(296, A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 0.0001);
        }

        if (!(s.v[532] != 0.0)) {
            s.store_add_ad_rhs(296, 246, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 0.0001));
        }

        s.store_scale(298, 296, 1.0 / (p.p152));

        s.v[533] = if (s.v[298] < p.p147) { 1.0 } else { 0.0 };

        if (!(s.v[533] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.store_scaled_offset(279, 246, (-p.p154), 1000.0);

        s.v[535] = if (((s.v[246] * s.v[8]) / p.p17) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[535] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p17))));
        }

        if (!(s.v[535] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[535] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p17))), (-p.p147)), 1.0));
        }

        s.v[536] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[537] = if (((s.v[246] - s.v[55]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[536] != 0.0) && (s.v[537] != 0.0)) {
            s.store_exp_ad(298, A::scale(A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[536] != 0.0) && (!(s.v[537] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((s.v[536] != 0.0) && (!(s.v[537] != 0.0))) {
            s.store_mul_ad_rhs(298, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[538] = if (((s.v[156] / s.v[35]) - 1000.0) < 40.0) { 1.0 } else { 0.0 };

        if ((s.v[536] != 0.0) && (!(s.v[538] != 0.0))) {
            s.store_scalar(295, ((40.0) as f64).exp());
        }

        s.v[540] = if (((s.v[247] * s.v[8]) / p.p19) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[540] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p19))));
        }

        if (!(s.v[540] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[540] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p19))), (-p.p147)), 1.0));
        }

        s.v[541] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[542] = if (((s.v[247] - s.v[55]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[541] != 0.0) && (s.v[542] != 0.0)) {
            s.store_exp_ad(298, A::scale(A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]));
        }

        if ((s.v[541] != 0.0) && (!(s.v[542] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((s.v[541] != 0.0) && (!(s.v[542] != 0.0))) {
            s.store_mul_ad_rhs(298, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]), (-p.p147)), 1.0));
        }

        s.v[543] = if (((s.v[246] * s.v[8]) / p.p21) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[543] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p21))));
        }

        if (!(s.v[543] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[543] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(246), (s.v[8] * 1.0 / (p.p21))), (-p.p147)), 1.0));
        }

        s.v[544] = if (((s.v[247] * s.v[8]) / p.p23) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[544] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p23))));
        }

        if (!(s.v[544] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[544] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p23))), (-p.p147)), 1.0));
        }

        s.v[545] = if (((s.v[249] * s.v[8]) / p.p32) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[545] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(249), (s.v[8] * 1.0 / (p.p32))));
        }

        if (!(s.v[545] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[545] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(249), (s.v[8] * 1.0 / (p.p32))), (-p.p147)), 1.0));
        }

        s.v[546] = if (((s.v[247] * s.v[8]) / p.p146) < p.p147) { 1.0 } else { 0.0 };

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
        if (s.v[546] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p146))));
        }

        if (!(s.v[546] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[546] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(247), (s.v[8] * 1.0 / (p.p146))), (-p.p147)), 1.0));
        }

        s.v[547] = if (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[246] < 0.0)) { 1.0 } else { 0.0 };

        s.v[548] = if ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[547] != 0.0) && (!(s.v[548] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (s.v[547] != 0.0) {
            s.store_mul(275, 246, 65);
        }

        if (s.v[547] != 0.0) {
            s.store_scale_ad(60, A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(275)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale(A::sub_from_scalar((1.0 - (p.p67 * p.p67)), A::scale(s.ad_value(275), (3.0 * (p.p67 - 1.0)))), p.p67), A::mul(A::mul(A::scale(s.ad_value(275), 6.0), s.ad_value(275)), A::offset(s.ad_value(275), (p.p67 - 1.0))))), 0.16666666666666666);
        }

        if (s.v[547] != 0.0) {
            s.store_div_ad(275, A::mul(A::scale(s.ad_value(246), s.v[62]), s.ad_value(61)), A::mul(s.ad_value(70), s.ad_value(60)));
        }

        s.v[549] = if (s.v[275] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[550] = if (s.v[275] < p.p147) { 1.0 } else { 0.0 };

        if (((s.v[547] != 0.0) && (s.v[549] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.v[551] = if (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[244] < 0.0)) { 1.0 } else { 0.0 };

        if (s.v[551] != 0.0) {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(244), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.v[552] = if ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p147) { 1.0 } else { 0.0 };

        if ((s.v[551] != 0.0) && (!(s.v[552] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (s.v[551] != 0.0) {
            s.store_mul(277, 244, 67);
        }

        if (s.v[551] != 0.0) {
            let assign4580_ad_e4435: A = A::scale(A::mul(A::powf(A::sqrt(A::offset(A::square(s.ad_value(277)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale(A::sub_from_scalar((1.0 - (s.v[76] * s.v[76])), A::scale(s.ad_value(277), (3.0 * (s.v[76] - 1.0)))), s.v[76]), A::mul(A::mul(A::scale(s.ad_value(277), 6.0), s.ad_value(277)), A::offset(s.ad_value(277), (s.v[76] - 1.0))))), 0.16666666666666666);
            s.store_ad(80, &assign4580_ad_e4435);
        }

        if (s.v[551] != 0.0) {
            s.store_div_ad(277, A::mul(A::scale(s.ad_value(244), s.v[79]), s.ad_value(83)), A::mul(s.ad_value(85), s.ad_value(80)));
        }

        s.v[553] = if (s.v[277] < (-0.001)) { 1.0 } else { 0.0 };

        s.v[554] = if (s.v[277] < p.p147) { 1.0 } else { 0.0 };

        if (((s.v[551] != 0.0) && (s.v[553] != 0.0)) && (!(s.v[554] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.store_mul(165, 146, 268);

        s.store_scale(166, 270, 4.0);

        s.store_div_ad(168, A::sub(s.ad_value(165), s.ad_value(146)), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0));

        s.store_div_ad_rhs(167, 166, A::offset(A::sqrt(A::offset(s.ad_value(166), 1.0)), 1.0));

        s.v[556] = if ((p.p5 > 0.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[556] != 0.0) {
            s.store_div_ad(171, A::scale(A::offset(s.ad_value(269), (-1.0)), ((p.p33 * 2.0) * s.v[43])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(269), ((4.0 * s.v[43]) / s.v[37])), 1.0)), 1.0));
        }

        s.v[557] = if (p.p8 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[556] != 0.0) && (s.v[557] != 0.0)) {
            s.store_div_ad(172, A::scale(A::sub(s.ad_value(269), s.ad_value(257)), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(A::add(s.ad_value(269), A::scale(s.ad_value(257), p.p144)), ((4.0 * s.v[104]) / s.v[106])), 1.0)), 1.0));
        }

        if ((s.v[556] != 0.0) && (!(s.v[557] != 0.0))) {
            s.store_div_ad(172, A::scale(A::offset(s.ad_value(269), (-1.0)), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106])), 1.0)), 1.0));
        }

        s.v[558] = if (p.p5 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_scalar(291, ((p.p33 * (s.v[43] + s.v[104])) * s.v[32]));
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_scale_ad(173, A::sub_from_scalar(2.0, A::ln(A::scale(s.ad_value(291), s.v[8]))), s.v[6]);
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_sub(284, 261, 173);
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_scalar(281, (0.11 * 0.11));
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_square(282, 284);
        }

        s.v[559] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[556] != 0.0) && (s.v[558] != 0.0)) && (s.v[559] != 0.0)) {
            s.store_div_ad(174, A::scale(s.ad_value(281), 0.5), A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)));
        }

        if (((s.v[556] != 0.0) && (s.v[558] != 0.0)) && (!(s.v[559] != 0.0))) {
            s.store_scale_ad(174, A::add(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)), 0.5);
        }

        if ((s.v[556] != 0.0) && (s.v[558] != 0.0)) {
            s.store_div_ad_rhs(175, 174, A::add(A::add(s.ad_value(291), A::scale(A::add(s.ad_value(171), s.ad_value(172)), s.v[32])), s.ad_value(174)));
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(173, 0.0);
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(284, 0.0);
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(174, 0.0);
        }

        if ((s.v[556] != 0.0) && (!(s.v[558] != 0.0))) {
            s.store_scalar(175, 1.0);
        }

        s.v[560] = if (p.p84 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[560] != 0.0) {
            s.store_add(347, 248, 244);
        }

        if (s.v[560] != 0.0) {
            s.store_scalar(281, (1e-6 * 1e-6));
        }

        if (s.v[560] != 0.0) {
            s.store_mul_ad_lhs(282, A::scale(s.ad_value(347), ((-1.0) * (-1.0))), 347);
        }

        s.store_add_ad(183, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 183);

        s.v[563] = if (s.v[183] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[563] != 0.0) {
            s.store_div_from_scalar_ad(184, (0.5 * s.v[281]), A::sub(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(183)));
        }

        if (!(s.v[563] != 0.0)) {
            s.store_scale_ad(184, A::add(A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(183)), 0.5);
        }

        s.store_mul_ad_rhs(185, 184, A::offset(A::scale(A::add(s.ad_value(149), s.ad_value(150)), 0.5), 1.0));

        s.store_div_from_scalar(187, s.v[29], 185);

        s.v[564] = if (s.v[187] < s.v[340]) { 1.0 } else { 0.0 };

        if (s.v[564] != 0.0) {
            s.copy_ad(187, 340);
        }

        s.store_scale(186, 187, 3.0);

        s.v[565] = if (s.v[156] > 0.0) { 1.0 } else { 0.0 };

        s.v[566] = if (p.p39 == 1.0) { 1.0 } else { 0.0 };

        s.v[567] = if (s.v[244] < p.p44) { 1.0 } else { 0.0 };

        s.v[568] = if (((-s.v[156]) / p.p42) < p.p147) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (s.v[568] != 0.0)) {
            s.store_exp_ad(332, A::scale(A::neg(s.ad_value(156)), 1.0 / (p.p42)));
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[568] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[568] != 0.0))) {
            s.store_mul_ad_rhs(332, 295, A::offset(A::offset(A::scale(A::neg(s.ad_value(156)), 1.0 / (p.p42)), (-p.p147)), 1.0));
        }

        if (((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) {
            s.store_mul_ad_lhs(333, A::sub_from_scalar(p.p44, s.ad_value(244)), 332);
        }

        s.v[569] = if (((-s.v[334]) * ((s.v[333]) as f64).powf(p.p41)) < p.p147) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (s.v[569] != 0.0)) {
            s.store_exp_ad(337, A::mul(A::neg(s.ad_value(334)), A::powf(s.ad_value(333), p.p41)));
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[569] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) && (!(s.v[569] != 0.0))) {
            s.store_mul_ad_rhs(337, 295, A::offset(A::offset(A::mul(A::neg(s.ad_value(334)), A::powf(s.ad_value(333), p.p41)), (-p.p147)), 1.0));
        }

        if (((s.v[565] != 0.0) && (s.v[566] != 0.0)) && (s.v[567] != 0.0)) {
            s.store_mul_ad_lhs(207, A::mul(A::div_from_scalar(p.p40, s.ad_value(334)), s.ad_value(333)), 337);
        }

        s.v[570] = if (p.p39 == 2.0) { 1.0 } else { 0.0 };

        s.v[571] = if (s.v[244] < s.v[16]) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_scalar(196, ((2.0 * p.p46) / (p.p45 * p.p45)));
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad_lhs(280, A::sub(s.ad_value(16), s.ad_value(244)), 210);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_sqrt_ad(197, A::div(A::scale(s.ad_value(280), 2.0), s.ad_value(196)));
        }

        s.v[572] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[572] != 0.0)) {
            s.store_scalar(198, p.p45);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
            s.store_sub_from_scalar_ad(123, 1.0, A::scale(s.ad_value(122), 0.5));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
            s.store_mul_ad_lhs(198, A::scale(s.ad_value(123), p.p45), 123);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad(199, A::mul(s.ad_value(197), s.ad_value(198)), A::sqrt(A::add(A::square(s.ad_value(197)), A::square(s.ad_value(198)))));
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad_lhs(200, A::sub(s.ad_value(16), s.ad_value(244)), 199);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_add_ad_rhs(201, 200, A::mul(A::mul(A::scale(s.ad_value(199), 0.5), s.ad_value(196)), s.ad_value(210)));
        }

        s.v[573] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[573] != 0.0)) {
            s.copy_ad(202, 201);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_offset_ad(203, A::scale(A::offset(A::scale(s.ad_value(122), 2.0), 1.0), (2.0 * p.p47)), 1.0);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_scalar(204, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_sub_ad_rhs(205, 200, A::mul(A::mul(A::scale(s.ad_value(199), 0.5), s.ad_value(196)), A::sub(s.ad_value(204), A::div(s.ad_value(156), A::scale(s.ad_value(203), p.p62)))));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_add_ad(280, A::mul(A::sub(s.ad_value(205), s.ad_value(201)), A::sub(s.ad_value(205), s.ad_value(201))), A::scale(A::mul(A::mul(A::scale(s.ad_value(200), 0.1), s.ad_value(200)), s.ad_value(134)), 1.0 / (p.p62)));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[573] != 0.0))) {
            s.store_scale_ad(202, A::add(A::add(s.ad_value(205), s.ad_value(201)), A::sqrt(s.ad_value(280))), 0.5);
        }

        if ((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) {
            s.store_div_ad_lhs(287, A::sub(s.ad_value(202), s.ad_value(200)), 202);
        }

        s.v[574] = if (((s.v[287]) as f64).abs() > 1e-7) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[574] != 0.0)) {
            s.store_div_ad_lhs(206, A::scale(s.ad_value(199), 0.5), 287);
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (s.v[574] != 0.0)) {
            s.store_mul_ad(207, A::mul(A::mul(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(202)), s.ad_value(206)), A::sub(A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(202))), A::exp(A::mul(A::div(A::neg(s.ad_value(98)), s.ad_value(202)), A::offset(A::div(s.ad_value(198), s.ad_value(206)), 1.0)))));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (s.v[570] != 0.0)) && (s.v[571] != 0.0)) && (!(s.v[574] != 0.0))) {
            s.store_mul_ad(207, A::mul(s.ad_value(0), s.ad_value(198)), A::exp(A::div(A::neg(s.ad_value(98)), s.ad_value(202))));
        }

        s.v[575] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        s.v[576] = if (s.v[244] < p.p44) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) {
            s.store_mul_ad(211, A::powf(A::sub_from_scalar(p.p44, s.ad_value(244)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(156), A::offset(s.ad_value(156), p.p48))), p.p49));
        }

        s.v[577] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
            s.copy_ad(212, 211);
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
            s.store_scaled_offset(213, 156, (-p.p52), 1.0 / (p.p48));
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
            s.store_scaled_offset(279, 213, (-1.0), 1.0 / (p.p51));
        }

        s.v[578] = if (s.v[213] < 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) && (s.v[578] != 0.0)) {
            s.store_offset_ad(214, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), p.p51), 1.0);
        }

        if (((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) && (!(s.v[578] != 0.0))) {
            s.store_add_ad_rhs(214, 213, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), p.p51));
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
            s.store_mul_ad_rhs(212, 211, A::powf(s.ad_value(214), p.p50));
        }

        s.v[579] = if (((-s.v[334]) * s.v[212]) < p.p147) { 1.0 } else { 0.0 };

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (s.v[579] != 0.0)) {
            s.store_exp_ad(337, A::mul(A::neg(s.ad_value(334)), s.ad_value(212)));
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if ((((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) && (!(s.v[579] != 0.0))) {
            s.store_mul_ad_rhs(337, 295, A::offset(A::offset(A::mul(A::neg(s.ad_value(334)), s.ad_value(212)), (-p.p147)), 1.0));
        }

        if (((((s.v[565] != 0.0) && (!(s.v[566] != 0.0))) && (!(s.v[570] != 0.0))) && (s.v[575] != 0.0)) && (s.v[576] != 0.0)) {
            s.store_mul_ad_lhs(207, A::mul(A::div_from_scalar(p.p40, s.ad_value(334)), A::sub_from_scalar(p.p44, s.ad_value(244))), 337);
        }

        s.v[580] = if (s.v[207] > 0.0) { 1.0 } else { 0.0 };

        s.v[581] = if (p.p53 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) {
            s.store_add_ad(208, A::add(A::div_from_scalar(s.v[6], A::mul(s.ad_value(156), A::add(s.ad_value(30), s.ad_value(186)))), A::scale(A::div(s.ad_value(153), s.ad_value(35)), s.v[42])), A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(186))));
        }

        s.v[582] = if (p.p39 == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
            s.store_scaled_sub(279, 207, 208, 1000000.0);
        }

        s.v[583] = if (s.v[207] < s.v[208]) { 1.0 } else { 0.0 };

        if (((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) && (s.v[583] != 0.0)) {
            s.store_sub_ad_rhs(207, 207, A::scale(A::ln(A::offset(A::exp(s.ad_value(279)), 1.0)), 1e-6));
        }

        if (((((s.v[565] != 0.0) && (s.v[580] != 0.0)) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) && (!(s.v[583] != 0.0))) {
            s.store_sub_ad_rhs(207, 208, A::scale(A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0)), 1e-6));
        }

        s.store_mul_ad_lhs(215, A::scale(s.ad_value(23), (1.0 - p.p68)), 138);

        s.store_div_ad_lhs(279, A::sub(s.ad_value(247), s.ad_value(136)), 293);

        s.v[585] = if (s.v[247] < s.v[136]) { 1.0 } else { 0.0 };

        if (s.v[585] != 0.0) {
            s.store_sub_ad_rhs(216, 247, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[585] != 0.0)) {
            s.store_sub_ad_rhs(216, 136, A::mul(s.ad_value(293), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_mul_ad(217, A::scale(s.ad_value(23), p.p68), A::add(A::mul(A::scale(s.ad_value(14), 1.0 / ((1.0 - p.p67))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(65))), (1.0 - p.p67)))), A::scale(A::sub(s.ad_value(247), s.ad_value(216)), 3.0)));

        s.store_mul_ad_lhs(218, A::scale(s.ad_value(24), p.p77), 145);

        s.v[219] = (s.v[94] * s.v[36]);

        s.store_mul_ad_lhs(223, A::scale(s.ad_value(149), (0.5 * s.v[219])), 184);

        s.store_mul_ad_lhs(224, A::scale(s.ad_value(150), (0.5 * s.v[219])), 184);

        s.store_scale(294, 17, 0.1);

        s.store_div_ad_lhs(279, A::sub(s.ad_value(249), s.ad_value(141)), 294);

        s.v[586] = if (s.v[249] < s.v[141]) { 1.0 } else { 0.0 };

        if (s.v[586] != 0.0) {
            s.store_sub_ad_rhs(225, 249, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[586] != 0.0)) {
            s.store_sub_ad_rhs(225, 141, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_add_ad(226, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(225), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(140), A::sub(s.ad_value(249), s.ad_value(225))));

        s.store_scale_ad(227, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(226)), A::mul(s.ad_value(25), s.ad_value(249)))), ((1.0 - p.p77) * (1.0 - p.p33)));

        s.store_div_ad_lhs(279, A::sub(s.ad_value(261), s.ad_value(141)), 294);

        s.v[587] = if (s.v[261] < s.v[141]) { 1.0 } else { 0.0 };

        if (s.v[587] != 0.0) {
            s.store_sub_ad_rhs(228, 261, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[587] != 0.0)) {
            s.store_sub_ad_rhs(228, 141, A::mul(s.ad_value(294), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_add_ad(229, A::mul(A::scale(s.ad_value(17), 1.0 / ((1.0 - p.p72))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(228), s.ad_value(17))), (1.0 - p.p72)))), A::mul(s.ad_value(140), A::sub(s.ad_value(261), s.ad_value(228))));

        s.store_scale_ad(230, A::mul(s.ad_value(24), A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(25)), s.ad_value(229)), A::mul(s.ad_value(25), s.ad_value(261)))), ((1.0 - p.p77) * p.p33));

        s.store_scale(301, 102, 0.1);

        s.store_scale(231, 102, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_ad_lhs(279, A::sub(s.ad_value(253), s.ad_value(231)), 301);

        s.v[588] = if (s.v[253] < s.v[231]) { 1.0 } else { 0.0 };

        if (s.v[588] != 0.0) {
            s.store_sub_ad_rhs(232, 253, A::mul(s.ad_value(301), A::ln(A::offset(A::exp(s.ad_value(279)), 1.0))));
        }

        if (!(s.v[588] != 0.0)) {
            s.store_sub_ad_rhs(232, 231, A::mul(s.ad_value(301), A::ln(A::offset(A::exp(A::neg(s.ad_value(279))), 1.0))));
        }

        s.store_mul_ad_rhs(233, 103, A::add(A::mul(A::scale(s.ad_value(102), 1.0 / ((1.0 - p.p139))), A::sub_from_scalar(1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(102))), (1.0 - p.p139)))), A::scale(A::sub(s.ad_value(253), s.ad_value(232)), 2.0)));

        s.store_scale_ad(234, A::powf(A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p85)), (s.v[93] * s.v[36]));

        s.v[589] = if ((s.v[246] / (p.p85 * s.v[6])) < p.p147) { 1.0 } else { 0.0 };

        if (s.v[589] != 0.0) {
            s.store_exp_ad(296, A::scale(s.ad_value(246), 1.0 / ((p.p85 * s.v[6]))));
        }

        if (!(s.v[589] != 0.0)) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (!(s.v[589] != 0.0)) {
            s.store_mul_ad_rhs(296, 295, A::offset(A::offset(A::scale(s.ad_value(246), 1.0 / ((p.p85 * s.v[6]))), (-p.p147)), 1.0));
        }

        s.store_mul(236, 234, 296);

        s.v[237] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_ad(238, A::scale(s.ad_value(122), (0.5 * s.v[237])), A::offset(A::add(s.ad_value(126), s.ad_value(113)), 2.0));

        s.v[590] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[590] != 0.0) {
            s.store_scale_ad(243, A::add(A::scale(s.ad_value(168), s.v[219]), A::scale(s.ad_value(167), s.v[237])), ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[591] = if ((((s.v[249] - s.v[22]) / p.p91) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if ((!(s.v[590] != 0.0)) && (s.v[591] != 0.0)) {
            s.store_exp_ad(177, A::scale(A::scale(A::sub(s.ad_value(249), s.ad_value(22)), 1.0 / (p.p91)), s.v[8]));
        }

        if ((!(s.v[590] != 0.0)) && (!(s.v[591] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
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
        if ((!(s.v[590] != 0.0)) && (!(s.v[591] != 0.0))) {
            s.store_mul_ad_rhs(177, 295, A::offset(A::offset(A::scale(A::scale(A::sub(s.ad_value(249), s.ad_value(22)), 1.0 / (p.p91)), s.v[8]), (-p.p147)), 1.0));
        }

        if (!(s.v[590] != 0.0)) {
            s.store_div_ad(243, A::scale(s.ad_value(268), ((2.0 * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(177), 4.0), 1.0)), 1.0));
        }

        s.v[592] = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[592] != 0.0) {
            s.store_scale(243, 243, s.v[157]);
        }

        s.v[593] = if (p.p79 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_mul(169, 146, 269);
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_div_ad(170, A::sub(s.ad_value(169), s.ad_value(146)), A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0));
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_scale(239, 272, 4.0);
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_div_ad_rhs(240, 239, A::offset(A::sqrt(A::offset(s.ad_value(239), 1.0)), 1.0));
        }

        if ((s.v[592] != 0.0) && (s.v[593] != 0.0)) {
            s.store_scale_ad(241, A::add(A::scale(s.ad_value(170), s.v[219]), A::scale(s.ad_value(240), s.v[237])), (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95]))));
        }

        s.v[594] = if (((s.v[261] - s.v[22]) * s.v[8]) < p.p147) { 1.0 } else { 0.0 };

        if (((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) && (s.v[594] != 0.0)) {
            s.store_exp_ad(178, A::scale(A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]));
        }

        if (((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) && (!(s.v[594] != 0.0))) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if (((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) && (!(s.v[594] != 0.0))) {
            s.store_mul_ad_rhs(178, 295, A::offset(A::offset(A::scale(A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]), (-p.p147)), 1.0));
        }

        if ((s.v[592] != 0.0) && (!(s.v[593] != 0.0))) {
            s.store_div_ad(241, A::scale(s.ad_value(269), (((2.0 * p.p33) * s.v[43]) * s.v[97])), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(178), 4.0), 1.0)), 1.0));
        }

        if (s.v[592] != 0.0) {
            s.store_mul(242, 175, 241);
        }

        s.v[595] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[595] != 0.0) {
            s.store_offset_ad(190, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (-p.p67)), (-3.0));
        }

        if (s.v[595] != 0.0) {
            s.store_div_ad_lhs(288, A::sub(s.ad_value(246), s.ad_value(136)), 293);
        }

        s.v[596] = if (s.v[288] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
            s.store_div_from_scalar_ad(191, 1.0, A::offset(A::exp(s.ad_value(288)), 1.0));
        }

        if ((s.v[595] != 0.0) && (!(s.v[596] != 0.0))) {
            s.store_div_ad(191, A::exp(A::neg(s.ad_value(288))), A::offset(A::exp(A::neg(s.ad_value(288))), 1.0));
        }

        if (s.v[595] != 0.0) {
            s.store_offset_ad(189, A::mul(s.ad_value(190), s.ad_value(191)), 3.0);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_lhs(192, A::scale(s.ad_value(23), (1.0 - p.p68)), 189);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad(195, A::div(A::scale(A::mul(s.ad_value(146), s.ad_value(266)), s.v[8]), s.ad_value(48)), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(147), 1.0))));
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_lhs(193, A::scale(s.ad_value(184), (0.5 * s.v[219])), 195);
        }

        if (s.v[595] != 0.0) {
            s.store_scale(194, 236, 1.0 / ((p.p85 * s.v[6])));
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad(222, A::scale(s.ad_value(248), 0.2), A::add(A::add(s.ad_value(192), s.ad_value(193)), s.ad_value(194)));
        }

        if (s.v[595] != 0.0) {
            s.store_scale(235, 236, (1.0 - p.p95));
        }

        if (s.v[595] != 0.0) {
            s.store_add_ad_rhs(331, 223, A::scale(s.ad_value(236), p.p95));
        }

        if (s.v[595] != 0.0) {
            s.store_add_ad_lhs(221, A::scale(s.ad_value(331), p.p94), 224);
        }

        if (s.v[595] != 0.0) {
            s.store_scale(220, 331, (1.0 - p.p94));
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(220, 223);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(221, 224);
        }

        if (!(s.v[595] != 0.0)) {
            s.copy_ad(235, 236);
        }

        s.store_div_ad_lhs(327, A::add(s.ad_value(155), s.ad_value(154)), 153);

        s.v[601] = if (s.v[327] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[601] != 0.0) {
            s.store_div_ad_lhs(329, A::add(s.ad_value(220), s.ad_value(221)), 327);
        }

        if (!(s.v[601] != 0.0)) {
            s.store_mul_ad_lhs(329, A::scale(s.ad_value(184), s.v[94]), 153);
        }

        s.v[602] = if (p.p131 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[602] != 0.0) {
            s.store_scale(330, 329, p.p94);
        }

        s.v[603] = if (p.p131 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[602] != 0.0)) && (s.v[603] != 0.0)) {
            s.store_scale(330, 329, p.p132);
        }

        if ((!(s.v[602] != 0.0)) && (!(s.v[603] != 0.0))) {
            s.store_scalar(330, 0.0);
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
        let eq0_e163: f64 = (p.p3 * s.v[115]);
        let eq0_e163_d_n0: f64 = (p.p3 * s.dn[115][0]);
        let eq0_e163_d_n1: f64 = (p.p3 * s.dn[115][1]);
        let eq0_e163_d_n2: f64 = (p.p3 * s.dn[115][2]);
        let eq0_e163_d_n3: f64 = (p.p3 * s.dn[115][3]);
        let eq0_e163_d_n4: f64 = (p.p3 * s.dn[115][4]);
        let eq0_e163_d_n5: f64 = (p.p3 * s.dn[115][5]);
        let eq0_e163_d_n6: f64 = (p.p3 * s.dn[115][6]);
        let eq0_e163_d_n7: f64 = (p.p3 * s.dn[115][7]);
        let eq0_e163_d_n8: f64 = (p.p3 * s.dn[115][8]);
        let eq0_e163_d_n9: f64 = (p.p3 * s.dn[115][9]);
        let eq0_e163_d_n10: f64 = (p.p3 * s.dn[115][10]);
        let eq0_e163_d_n11: f64 = (p.p3 * s.dn[115][11]);
        let eq0_e165: f64 = (eq0_e163 * p.p1);
        let eq0_e165_d_n0: f64 = (eq0_e163_d_n0 * p.p1);
        let eq0_e165_d_n1: f64 = (eq0_e163_d_n1 * p.p1);
        let eq0_e165_d_n2: f64 = (eq0_e163_d_n2 * p.p1);
        let eq0_e165_d_n3: f64 = (eq0_e163_d_n3 * p.p1);
        let eq0_e165_d_n4: f64 = (eq0_e163_d_n4 * p.p1);
        let eq0_e165_d_n5: f64 = (eq0_e163_d_n5 * p.p1);
        let eq0_e165_d_n6: f64 = (eq0_e163_d_n6 * p.p1);
        let eq0_e165_d_n7: f64 = (eq0_e163_d_n7 * p.p1);
        let eq0_e165_d_n8: f64 = (eq0_e163_d_n8 * p.p1);
        let eq0_e165_d_n9: f64 = (eq0_e163_d_n9 * p.p1);
        let eq0_e165_d_n10: f64 = (eq0_e163_d_n10 * p.p1);
        let eq0_e165_d_n11: f64 = (eq0_e163_d_n11 * p.p1);
        let eq0_value: f64 = eq0_e165;
        let eq0_node_derivatives: [f64; 12] = [eq0_e165_d_n0, eq0_e165_d_n1, eq0_e165_d_n2, eq0_e165_d_n3, eq0_e165_d_n4, eq0_e165_d_n5, eq0_e165_d_n6, eq0_e165_d_n7, eq0_e165_d_n8, eq0_e165_d_n9, eq0_e165_d_n10, eq0_e165_d_n11];
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
        let eq1_e168: f64 = (p.p3 * s.v[156]);
        let eq1_e168_d_n0: f64 = (p.p3 * s.dn[156][0]);
        let eq1_e168_d_n1: f64 = (p.p3 * s.dn[156][1]);
        let eq1_e168_d_n2: f64 = (p.p3 * s.dn[156][2]);
        let eq1_e168_d_n3: f64 = (p.p3 * s.dn[156][3]);
        let eq1_e168_d_n4: f64 = (p.p3 * s.dn[156][4]);
        let eq1_e168_d_n5: f64 = (p.p3 * s.dn[156][5]);
        let eq1_e168_d_n6: f64 = (p.p3 * s.dn[156][6]);
        let eq1_e168_d_n7: f64 = (p.p3 * s.dn[156][7]);
        let eq1_e168_d_n8: f64 = (p.p3 * s.dn[156][8]);
        let eq1_e168_d_n9: f64 = (p.p3 * s.dn[156][9]);
        let eq1_e168_d_n10: f64 = (p.p3 * s.dn[156][10]);
        let eq1_e168_d_n11: f64 = (p.p3 * s.dn[156][11]);
        let eq1_e170: f64 = (eq1_e168 * p.p1);
        let eq1_e170_d_n0: f64 = (eq1_e168_d_n0 * p.p1);
        let eq1_e170_d_n1: f64 = (eq1_e168_d_n1 * p.p1);
        let eq1_e170_d_n2: f64 = (eq1_e168_d_n2 * p.p1);
        let eq1_e170_d_n3: f64 = (eq1_e168_d_n3 * p.p1);
        let eq1_e170_d_n4: f64 = (eq1_e168_d_n4 * p.p1);
        let eq1_e170_d_n5: f64 = (eq1_e168_d_n5 * p.p1);
        let eq1_e170_d_n6: f64 = (eq1_e168_d_n6 * p.p1);
        let eq1_e170_d_n7: f64 = (eq1_e168_d_n7 * p.p1);
        let eq1_e170_d_n8: f64 = (eq1_e168_d_n8 * p.p1);
        let eq1_e170_d_n9: f64 = (eq1_e168_d_n9 * p.p1);
        let eq1_e170_d_n10: f64 = (eq1_e168_d_n10 * p.p1);
        let eq1_e170_d_n11: f64 = (eq1_e168_d_n11 * p.p1);
        let eq1_value: f64 = eq1_e170;
        let eq1_node_derivatives: [f64; 12] = [eq1_e170_d_n0, eq1_e170_d_n1, eq1_e170_d_n2, eq1_e170_d_n3, eq1_e170_d_n4, eq1_e170_d_n5, eq1_e170_d_n6, eq1_e170_d_n7, eq1_e170_d_n8, eq1_e170_d_n9, eq1_e170_d_n10, eq1_e170_d_n11];
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
        let eq2_e174: f64 = (s.v[159] + s.v[162]);
        let eq2_e174_d_n0: f64 = (s.dn[159][0] + s.dn[162][0]);
        let eq2_e174_d_n1: f64 = (s.dn[159][1] + s.dn[162][1]);
        let eq2_e174_d_n2: f64 = (s.dn[159][2] + s.dn[162][2]);
        let eq2_e174_d_n3: f64 = (s.dn[159][3] + s.dn[162][3]);
        let eq2_e174_d_n4: f64 = (s.dn[159][4] + s.dn[162][4]);
        let eq2_e174_d_n5: f64 = (s.dn[159][5] + s.dn[162][5]);
        let eq2_e174_d_n6: f64 = (s.dn[159][6] + s.dn[162][6]);
        let eq2_e174_d_n7: f64 = (s.dn[159][7] + s.dn[162][7]);
        let eq2_e174_d_n8: f64 = (s.dn[159][8] + s.dn[162][8]);
        let eq2_e174_d_n9: f64 = (s.dn[159][9] + s.dn[162][9]);
        let eq2_e174_d_n10: f64 = (s.dn[159][10] + s.dn[162][10]);
        let eq2_e174_d_n11: f64 = (s.dn[159][11] + s.dn[162][11]);
        let eq2_e176: f64 = (eq2_e174 + s.v[163]);
        let eq2_e176_d_n0: f64 = (eq2_e174_d_n0 + s.dn[163][0]);
        let eq2_e176_d_n1: f64 = (eq2_e174_d_n1 + s.dn[163][1]);
        let eq2_e176_d_n2: f64 = (eq2_e174_d_n2 + s.dn[163][2]);
        let eq2_e176_d_n3: f64 = (eq2_e174_d_n3 + s.dn[163][3]);
        let eq2_e176_d_n4: f64 = (eq2_e174_d_n4 + s.dn[163][4]);
        let eq2_e176_d_n5: f64 = (eq2_e174_d_n5 + s.dn[163][5]);
        let eq2_e176_d_n6: f64 = (eq2_e174_d_n6 + s.dn[163][6]);
        let eq2_e176_d_n7: f64 = (eq2_e174_d_n7 + s.dn[163][7]);
        let eq2_e176_d_n8: f64 = (eq2_e174_d_n8 + s.dn[163][8]);
        let eq2_e176_d_n9: f64 = (eq2_e174_d_n9 + s.dn[163][9]);
        let eq2_e176_d_n10: f64 = (eq2_e174_d_n10 + s.dn[163][10]);
        let eq2_e176_d_n11: f64 = (eq2_e174_d_n11 + s.dn[163][11]);
        let eq2_e177: f64 = (p.p3 * eq2_e176);
        let eq2_e177_d_n0: f64 = (p.p3 * eq2_e176_d_n0);
        let eq2_e177_d_n1: f64 = (p.p3 * eq2_e176_d_n1);
        let eq2_e177_d_n2: f64 = (p.p3 * eq2_e176_d_n2);
        let eq2_e177_d_n3: f64 = (p.p3 * eq2_e176_d_n3);
        let eq2_e177_d_n4: f64 = (p.p3 * eq2_e176_d_n4);
        let eq2_e177_d_n5: f64 = (p.p3 * eq2_e176_d_n5);
        let eq2_e177_d_n6: f64 = (p.p3 * eq2_e176_d_n6);
        let eq2_e177_d_n7: f64 = (p.p3 * eq2_e176_d_n7);
        let eq2_e177_d_n8: f64 = (p.p3 * eq2_e176_d_n8);
        let eq2_e177_d_n9: f64 = (p.p3 * eq2_e176_d_n9);
        let eq2_e177_d_n10: f64 = (p.p3 * eq2_e176_d_n10);
        let eq2_e177_d_n11: f64 = (p.p3 * eq2_e176_d_n11);
        let eq2_e179: f64 = (eq2_e177 * p.p1);
        let eq2_e179_d_n0: f64 = (eq2_e177_d_n0 * p.p1);
        let eq2_e179_d_n1: f64 = (eq2_e177_d_n1 * p.p1);
        let eq2_e179_d_n2: f64 = (eq2_e177_d_n2 * p.p1);
        let eq2_e179_d_n3: f64 = (eq2_e177_d_n3 * p.p1);
        let eq2_e179_d_n4: f64 = (eq2_e177_d_n4 * p.p1);
        let eq2_e179_d_n5: f64 = (eq2_e177_d_n5 * p.p1);
        let eq2_e179_d_n6: f64 = (eq2_e177_d_n6 * p.p1);
        let eq2_e179_d_n7: f64 = (eq2_e177_d_n7 * p.p1);
        let eq2_e179_d_n8: f64 = (eq2_e177_d_n8 * p.p1);
        let eq2_e179_d_n9: f64 = (eq2_e177_d_n9 * p.p1);
        let eq2_e179_d_n10: f64 = (eq2_e177_d_n10 * p.p1);
        let eq2_e179_d_n11: f64 = (eq2_e177_d_n11 * p.p1);
        let eq2_value: f64 = eq2_e179;
        let eq2_node_derivatives: [f64; 12] = [eq2_e179_d_n0, eq2_e179_d_n1, eq2_e179_d_n2, eq2_e179_d_n3, eq2_e179_d_n4, eq2_e179_d_n5, eq2_e179_d_n6, eq2_e179_d_n7, eq2_e179_d_n8, eq2_e179_d_n9, eq2_e179_d_n10, eq2_e179_d_n11];
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
        let eq3_e183: f64 = (s.v[158] + s.v[160]);
        let eq3_e183_d_n0: f64 = (s.dn[158][0] + s.dn[160][0]);
        let eq3_e183_d_n1: f64 = (s.dn[158][1] + s.dn[160][1]);
        let eq3_e183_d_n2: f64 = (s.dn[158][2] + s.dn[160][2]);
        let eq3_e183_d_n3: f64 = (s.dn[158][3] + s.dn[160][3]);
        let eq3_e183_d_n4: f64 = (s.dn[158][4] + s.dn[160][4]);
        let eq3_e183_d_n5: f64 = (s.dn[158][5] + s.dn[160][5]);
        let eq3_e183_d_n6: f64 = (s.dn[158][6] + s.dn[160][6]);
        let eq3_e183_d_n7: f64 = (s.dn[158][7] + s.dn[160][7]);
        let eq3_e183_d_n8: f64 = (s.dn[158][8] + s.dn[160][8]);
        let eq3_e183_d_n9: f64 = (s.dn[158][9] + s.dn[160][9]);
        let eq3_e183_d_n10: f64 = (s.dn[158][10] + s.dn[160][10]);
        let eq3_e183_d_n11: f64 = (s.dn[158][11] + s.dn[160][11]);
        let eq3_e186: f64 = (s.v[338] * s.v[246]);
        let eq3_e186_d_n0: f64 = (s.v[338] * s.dn[246][0]);
        let eq3_e186_d_n1: f64 = (s.v[338] * s.dn[246][1]);
        let eq3_e186_d_n2: f64 = (s.v[338] * s.dn[246][2]);
        let eq3_e186_d_n3: f64 = (s.v[338] * s.dn[246][3]);
        let eq3_e186_d_n4: f64 = (s.v[338] * s.dn[246][4]);
        let eq3_e186_d_n5: f64 = (s.v[338] * s.dn[246][5]);
        let eq3_e186_d_n6: f64 = (s.v[338] * s.dn[246][6]);
        let eq3_e186_d_n7: f64 = (s.v[338] * s.dn[246][7]);
        let eq3_e186_d_n8: f64 = (s.v[338] * s.dn[246][8]);
        let eq3_e186_d_n9: f64 = (s.v[338] * s.dn[246][9]);
        let eq3_e186_d_n10: f64 = (s.v[338] * s.dn[246][10]);
        let eq3_e186_d_n11: f64 = (s.v[338] * s.dn[246][11]);
        let eq3_e187: f64 = (eq3_e183 + eq3_e186);
        let eq3_e187_d_n0: f64 = (eq3_e183_d_n0 + eq3_e186_d_n0);
        let eq3_e187_d_n1: f64 = (eq3_e183_d_n1 + eq3_e186_d_n1);
        let eq3_e187_d_n2: f64 = (eq3_e183_d_n2 + eq3_e186_d_n2);
        let eq3_e187_d_n3: f64 = (eq3_e183_d_n3 + eq3_e186_d_n3);
        let eq3_e187_d_n4: f64 = (eq3_e183_d_n4 + eq3_e186_d_n4);
        let eq3_e187_d_n5: f64 = (eq3_e183_d_n5 + eq3_e186_d_n5);
        let eq3_e187_d_n6: f64 = (eq3_e183_d_n6 + eq3_e186_d_n6);
        let eq3_e187_d_n7: f64 = (eq3_e183_d_n7 + eq3_e186_d_n7);
        let eq3_e187_d_n8: f64 = (eq3_e183_d_n8 + eq3_e186_d_n8);
        let eq3_e187_d_n9: f64 = (eq3_e183_d_n9 + eq3_e186_d_n9);
        let eq3_e187_d_n10: f64 = (eq3_e183_d_n10 + eq3_e186_d_n10);
        let eq3_e187_d_n11: f64 = (eq3_e183_d_n11 + eq3_e186_d_n11);
        let eq3_e189: f64 = (eq3_e187 - s.v[57]);
        let eq3_e189_d_n0: f64 = (eq3_e187_d_n0 - s.dn[57][0]);
        let eq3_e189_d_n1: f64 = (eq3_e187_d_n1 - s.dn[57][1]);
        let eq3_e189_d_n2: f64 = (eq3_e187_d_n2 - s.dn[57][2]);
        let eq3_e189_d_n3: f64 = (eq3_e187_d_n3 - s.dn[57][3]);
        let eq3_e189_d_n4: f64 = (eq3_e187_d_n4 - s.dn[57][4]);
        let eq3_e189_d_n5: f64 = (eq3_e187_d_n5 - s.dn[57][5]);
        let eq3_e189_d_n6: f64 = (eq3_e187_d_n6 - s.dn[57][6]);
        let eq3_e189_d_n7: f64 = (eq3_e187_d_n7 - s.dn[57][7]);
        let eq3_e189_d_n8: f64 = (eq3_e187_d_n8 - s.dn[57][8]);
        let eq3_e189_d_n9: f64 = (eq3_e187_d_n9 - s.dn[57][9]);
        let eq3_e189_d_n10: f64 = (eq3_e187_d_n10 - s.dn[57][10]);
        let eq3_e189_d_n11: f64 = (eq3_e187_d_n11 - s.dn[57][11]);
        let eq3_e191: f64 = (eq3_e189 + s.v[352]);
        let eq3_e191_d_n0: f64 = (eq3_e189_d_n0 + s.dn[352][0]);
        let eq3_e191_d_n1: f64 = (eq3_e189_d_n1 + s.dn[352][1]);
        let eq3_e191_d_n2: f64 = (eq3_e189_d_n2 + s.dn[352][2]);
        let eq3_e191_d_n3: f64 = (eq3_e189_d_n3 + s.dn[352][3]);
        let eq3_e191_d_n4: f64 = (eq3_e189_d_n4 + s.dn[352][4]);
        let eq3_e191_d_n5: f64 = (eq3_e189_d_n5 + s.dn[352][5]);
        let eq3_e191_d_n6: f64 = (eq3_e189_d_n6 + s.dn[352][6]);
        let eq3_e191_d_n7: f64 = (eq3_e189_d_n7 + s.dn[352][7]);
        let eq3_e191_d_n8: f64 = (eq3_e189_d_n8 + s.dn[352][8]);
        let eq3_e191_d_n9: f64 = (eq3_e189_d_n9 + s.dn[352][9]);
        let eq3_e191_d_n10: f64 = (eq3_e189_d_n10 + s.dn[352][10]);
        let eq3_e191_d_n11: f64 = (eq3_e189_d_n11 + s.dn[352][11]);
        let eq3_e193: f64 = (eq3_e191 + s.v[351]);
        let eq3_e193_d_n0: f64 = (eq3_e191_d_n0 + s.dn[351][0]);
        let eq3_e193_d_n1: f64 = (eq3_e191_d_n1 + s.dn[351][1]);
        let eq3_e193_d_n2: f64 = (eq3_e191_d_n2 + s.dn[351][2]);
        let eq3_e193_d_n3: f64 = (eq3_e191_d_n3 + s.dn[351][3]);
        let eq3_e193_d_n4: f64 = (eq3_e191_d_n4 + s.dn[351][4]);
        let eq3_e193_d_n5: f64 = (eq3_e191_d_n5 + s.dn[351][5]);
        let eq3_e193_d_n6: f64 = (eq3_e191_d_n6 + s.dn[351][6]);
        let eq3_e193_d_n7: f64 = (eq3_e191_d_n7 + s.dn[351][7]);
        let eq3_e193_d_n8: f64 = (eq3_e191_d_n8 + s.dn[351][8]);
        let eq3_e193_d_n9: f64 = (eq3_e191_d_n9 + s.dn[351][9]);
        let eq3_e193_d_n10: f64 = (eq3_e191_d_n10 + s.dn[351][10]);
        let eq3_e193_d_n11: f64 = (eq3_e191_d_n11 + s.dn[351][11]);
        let eq3_e194: f64 = (p.p3 * eq3_e193);
        let eq3_e194_d_n0: f64 = (p.p3 * eq3_e193_d_n0);
        let eq3_e194_d_n1: f64 = (p.p3 * eq3_e193_d_n1);
        let eq3_e194_d_n2: f64 = (p.p3 * eq3_e193_d_n2);
        let eq3_e194_d_n3: f64 = (p.p3 * eq3_e193_d_n3);
        let eq3_e194_d_n4: f64 = (p.p3 * eq3_e193_d_n4);
        let eq3_e194_d_n5: f64 = (p.p3 * eq3_e193_d_n5);
        let eq3_e194_d_n6: f64 = (p.p3 * eq3_e193_d_n6);
        let eq3_e194_d_n7: f64 = (p.p3 * eq3_e193_d_n7);
        let eq3_e194_d_n8: f64 = (p.p3 * eq3_e193_d_n8);
        let eq3_e194_d_n9: f64 = (p.p3 * eq3_e193_d_n9);
        let eq3_e194_d_n10: f64 = (p.p3 * eq3_e193_d_n10);
        let eq3_e194_d_n11: f64 = (p.p3 * eq3_e193_d_n11);
        let eq3_e196: f64 = (eq3_e194 * p.p1);
        let eq3_e196_d_n0: f64 = (eq3_e194_d_n0 * p.p1);
        let eq3_e196_d_n1: f64 = (eq3_e194_d_n1 * p.p1);
        let eq3_e196_d_n2: f64 = (eq3_e194_d_n2 * p.p1);
        let eq3_e196_d_n3: f64 = (eq3_e194_d_n3 * p.p1);
        let eq3_e196_d_n4: f64 = (eq3_e194_d_n4 * p.p1);
        let eq3_e196_d_n5: f64 = (eq3_e194_d_n5 * p.p1);
        let eq3_e196_d_n6: f64 = (eq3_e194_d_n6 * p.p1);
        let eq3_e196_d_n7: f64 = (eq3_e194_d_n7 * p.p1);
        let eq3_e196_d_n8: f64 = (eq3_e194_d_n8 * p.p1);
        let eq3_e196_d_n9: f64 = (eq3_e194_d_n9 * p.p1);
        let eq3_e196_d_n10: f64 = (eq3_e194_d_n10 * p.p1);
        let eq3_e196_d_n11: f64 = (eq3_e194_d_n11 * p.p1);
        let eq3_value: f64 = eq3_e196;
        let eq3_node_derivatives: [f64; 12] = [eq3_e196_d_n0, eq3_e196_d_n1, eq3_e196_d_n2, eq3_e196_d_n3, eq3_e196_d_n4, eq3_e196_d_n5, eq3_e196_d_n6, eq3_e196_d_n7, eq3_e196_d_n8, eq3_e196_d_n9, eq3_e196_d_n10, eq3_e196_d_n11];
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
        let (eq4_e205, eq4_e205_d_n0, eq4_e205_d_n1, eq4_e205_d_n2, eq4_e205_d_n3, eq4_e205_d_n4, eq4_e205_d_n5, eq4_e205_d_n6, eq4_e205_d_n7, eq4_e205_d_n8, eq4_e205_d_n9, eq4_e205_d_n10, eq4_e205_d_n11,) = {
    if (s.v[597] != 0.0) {
        let eq4_e200: f64 = (-s.v[82]);
        let eq4_e200_d_n0: f64 = (-s.dn[82][0]);
        let eq4_e200_d_n1: f64 = (-s.dn[82][1]);
        let eq4_e200_d_n2: f64 = (-s.dn[82][2]);
        let eq4_e200_d_n3: f64 = (-s.dn[82][3]);
        let eq4_e200_d_n4: f64 = (-s.dn[82][4]);
        let eq4_e200_d_n5: f64 = (-s.dn[82][5]);
        let eq4_e200_d_n6: f64 = (-s.dn[82][6]);
        let eq4_e200_d_n7: f64 = (-s.dn[82][7]);
        let eq4_e200_d_n8: f64 = (-s.dn[82][8]);
        let eq4_e200_d_n9: f64 = (-s.dn[82][9]);
        let eq4_e200_d_n10: f64 = (-s.dn[82][10]);
        let eq4_e200_d_n11: f64 = (-s.dn[82][11]);
        let eq4_e201: f64 = (p.p3 * eq4_e200);
        let eq4_e201_d_n0: f64 = (p.p3 * eq4_e200_d_n0);
        let eq4_e201_d_n1: f64 = (p.p3 * eq4_e200_d_n1);
        let eq4_e201_d_n2: f64 = (p.p3 * eq4_e200_d_n2);
        let eq4_e201_d_n3: f64 = (p.p3 * eq4_e200_d_n3);
        let eq4_e201_d_n4: f64 = (p.p3 * eq4_e200_d_n4);
        let eq4_e201_d_n5: f64 = (p.p3 * eq4_e200_d_n5);
        let eq4_e201_d_n6: f64 = (p.p3 * eq4_e200_d_n6);
        let eq4_e201_d_n7: f64 = (p.p3 * eq4_e200_d_n7);
        let eq4_e201_d_n8: f64 = (p.p3 * eq4_e200_d_n8);
        let eq4_e201_d_n9: f64 = (p.p3 * eq4_e200_d_n9);
        let eq4_e201_d_n10: f64 = (p.p3 * eq4_e200_d_n10);
        let eq4_e201_d_n11: f64 = (p.p3 * eq4_e200_d_n11);
        let eq4_e203: f64 = (eq4_e201 * p.p1);
        let eq4_e203_d_n0: f64 = (eq4_e201_d_n0 * p.p1);
        let eq4_e203_d_n1: f64 = (eq4_e201_d_n1 * p.p1);
        let eq4_e203_d_n2: f64 = (eq4_e201_d_n2 * p.p1);
        let eq4_e203_d_n3: f64 = (eq4_e201_d_n3 * p.p1);
        let eq4_e203_d_n4: f64 = (eq4_e201_d_n4 * p.p1);
        let eq4_e203_d_n5: f64 = (eq4_e201_d_n5 * p.p1);
        let eq4_e203_d_n6: f64 = (eq4_e201_d_n6 * p.p1);
        let eq4_e203_d_n7: f64 = (eq4_e201_d_n7 * p.p1);
        let eq4_e203_d_n8: f64 = (eq4_e201_d_n8 * p.p1);
        let eq4_e203_d_n9: f64 = (eq4_e201_d_n9 * p.p1);
        let eq4_e203_d_n10: f64 = (eq4_e201_d_n10 * p.p1);
        let eq4_e203_d_n11: f64 = (eq4_e201_d_n11 * p.p1);
        (eq4_e203, eq4_e203_d_n0, eq4_e203_d_n1, eq4_e203_d_n2, eq4_e203_d_n3, eq4_e203_d_n4, eq4_e203_d_n5, eq4_e203_d_n6, eq4_e203_d_n7, eq4_e203_d_n8, eq4_e203_d_n9, eq4_e203_d_n10, eq4_e203_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e205;
        let eq4_node_derivatives: [f64; 12] = [eq4_e205_d_n0, eq4_e205_d_n1, eq4_e205_d_n2, eq4_e205_d_n3, eq4_e205_d_n4, eq4_e205_d_n5, eq4_e205_d_n6, eq4_e205_d_n7, eq4_e205_d_n8, eq4_e205_d_n9, eq4_e205_d_n10, eq4_e205_d_n11];
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
        let (eq5_e215, eq5_e215_d_n0, eq5_e215_d_n1, eq5_e215_d_n2, eq5_e215_d_n3, eq5_e215_d_n4, eq5_e215_d_n5, eq5_e215_d_n6, eq5_e215_d_n7, eq5_e215_d_n8, eq5_e215_d_n9, eq5_e215_d_n10, eq5_e215_d_n11,) = {
    if (!(s.v[597] != 0.0)) {
        let eq5_e210: f64 = (-s.v[82]);
        let eq5_e210_d_n0: f64 = (-s.dn[82][0]);
        let eq5_e210_d_n1: f64 = (-s.dn[82][1]);
        let eq5_e210_d_n2: f64 = (-s.dn[82][2]);
        let eq5_e210_d_n3: f64 = (-s.dn[82][3]);
        let eq5_e210_d_n4: f64 = (-s.dn[82][4]);
        let eq5_e210_d_n5: f64 = (-s.dn[82][5]);
        let eq5_e210_d_n6: f64 = (-s.dn[82][6]);
        let eq5_e210_d_n7: f64 = (-s.dn[82][7]);
        let eq5_e210_d_n8: f64 = (-s.dn[82][8]);
        let eq5_e210_d_n9: f64 = (-s.dn[82][9]);
        let eq5_e210_d_n10: f64 = (-s.dn[82][10]);
        let eq5_e210_d_n11: f64 = (-s.dn[82][11]);
        let eq5_e211: f64 = (p.p3 * eq5_e210);
        let eq5_e211_d_n0: f64 = (p.p3 * eq5_e210_d_n0);
        let eq5_e211_d_n1: f64 = (p.p3 * eq5_e210_d_n1);
        let eq5_e211_d_n2: f64 = (p.p3 * eq5_e210_d_n2);
        let eq5_e211_d_n3: f64 = (p.p3 * eq5_e210_d_n3);
        let eq5_e211_d_n4: f64 = (p.p3 * eq5_e210_d_n4);
        let eq5_e211_d_n5: f64 = (p.p3 * eq5_e210_d_n5);
        let eq5_e211_d_n6: f64 = (p.p3 * eq5_e210_d_n6);
        let eq5_e211_d_n7: f64 = (p.p3 * eq5_e210_d_n7);
        let eq5_e211_d_n8: f64 = (p.p3 * eq5_e210_d_n8);
        let eq5_e211_d_n9: f64 = (p.p3 * eq5_e210_d_n9);
        let eq5_e211_d_n10: f64 = (p.p3 * eq5_e210_d_n10);
        let eq5_e211_d_n11: f64 = (p.p3 * eq5_e210_d_n11);
        let eq5_e213: f64 = (eq5_e211 * p.p1);
        let eq5_e213_d_n0: f64 = (eq5_e211_d_n0 * p.p1);
        let eq5_e213_d_n1: f64 = (eq5_e211_d_n1 * p.p1);
        let eq5_e213_d_n2: f64 = (eq5_e211_d_n2 * p.p1);
        let eq5_e213_d_n3: f64 = (eq5_e211_d_n3 * p.p1);
        let eq5_e213_d_n4: f64 = (eq5_e211_d_n4 * p.p1);
        let eq5_e213_d_n5: f64 = (eq5_e211_d_n5 * p.p1);
        let eq5_e213_d_n6: f64 = (eq5_e211_d_n6 * p.p1);
        let eq5_e213_d_n7: f64 = (eq5_e211_d_n7 * p.p1);
        let eq5_e213_d_n8: f64 = (eq5_e211_d_n8 * p.p1);
        let eq5_e213_d_n9: f64 = (eq5_e211_d_n9 * p.p1);
        let eq5_e213_d_n10: f64 = (eq5_e211_d_n10 * p.p1);
        let eq5_e213_d_n11: f64 = (eq5_e211_d_n11 * p.p1);
        (eq5_e213, eq5_e213_d_n0, eq5_e213_d_n1, eq5_e213_d_n2, eq5_e213_d_n3, eq5_e213_d_n4, eq5_e213_d_n5, eq5_e213_d_n6, eq5_e213_d_n7, eq5_e213_d_n8, eq5_e213_d_n9, eq5_e213_d_n10, eq5_e213_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e215;
        let eq5_node_derivatives: [f64; 12] = [eq5_e215_d_n0, eq5_e215_d_n1, eq5_e215_d_n2, eq5_e215_d_n3, eq5_e215_d_n4, eq5_e215_d_n5, eq5_e215_d_n6, eq5_e215_d_n7, eq5_e215_d_n8, eq5_e215_d_n9, eq5_e215_d_n10, eq5_e215_d_n11];
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
        let eq6_e218: f64 = (p.p3 * s.v[179]);
        let eq6_e218_d_n0: f64 = (p.p3 * s.dn[179][0]);
        let eq6_e218_d_n1: f64 = (p.p3 * s.dn[179][1]);
        let eq6_e218_d_n2: f64 = (p.p3 * s.dn[179][2]);
        let eq6_e218_d_n3: f64 = (p.p3 * s.dn[179][3]);
        let eq6_e218_d_n4: f64 = (p.p3 * s.dn[179][4]);
        let eq6_e218_d_n5: f64 = (p.p3 * s.dn[179][5]);
        let eq6_e218_d_n6: f64 = (p.p3 * s.dn[179][6]);
        let eq6_e218_d_n7: f64 = (p.p3 * s.dn[179][7]);
        let eq6_e218_d_n8: f64 = (p.p3 * s.dn[179][8]);
        let eq6_e218_d_n9: f64 = (p.p3 * s.dn[179][9]);
        let eq6_e218_d_n10: f64 = (p.p3 * s.dn[179][10]);
        let eq6_e218_d_n11: f64 = (p.p3 * s.dn[179][11]);
        let eq6_e220: f64 = (eq6_e218 * p.p1);
        let eq6_e220_d_n0: f64 = (eq6_e218_d_n0 * p.p1);
        let eq6_e220_d_n1: f64 = (eq6_e218_d_n1 * p.p1);
        let eq6_e220_d_n2: f64 = (eq6_e218_d_n2 * p.p1);
        let eq6_e220_d_n3: f64 = (eq6_e218_d_n3 * p.p1);
        let eq6_e220_d_n4: f64 = (eq6_e218_d_n4 * p.p1);
        let eq6_e220_d_n5: f64 = (eq6_e218_d_n5 * p.p1);
        let eq6_e220_d_n6: f64 = (eq6_e218_d_n6 * p.p1);
        let eq6_e220_d_n7: f64 = (eq6_e218_d_n7 * p.p1);
        let eq6_e220_d_n8: f64 = (eq6_e218_d_n8 * p.p1);
        let eq6_e220_d_n9: f64 = (eq6_e218_d_n9 * p.p1);
        let eq6_e220_d_n10: f64 = (eq6_e218_d_n10 * p.p1);
        let eq6_e220_d_n11: f64 = (eq6_e218_d_n11 * p.p1);
        let eq6_value: f64 = eq6_e220;
        let eq6_node_derivatives: [f64; 12] = [eq6_e220_d_n0, eq6_e220_d_n1, eq6_e220_d_n2, eq6_e220_d_n3, eq6_e220_d_n4, eq6_e220_d_n5, eq6_e220_d_n6, eq6_e220_d_n7, eq6_e220_d_n8, eq6_e220_d_n9, eq6_e220_d_n10, eq6_e220_d_n11];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
