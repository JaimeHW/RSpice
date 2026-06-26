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
        s.b[476] = (p.p3 == 1.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if s.b[476] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[476]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[157] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.v[338] = 0.0;

        s.b[477] = (p.p150 == 0.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if s.b[477] {
            s.store_scalar(339, 1e-12);
        }

        if (!s.b[477]) {
            s.store_scalar(339, p.p150);
        }

        s.store_scale(340, 339, p.p1);

        s.store_div_from_scalar(341, 1.0, 340);

        s.b[478] = (p.p134 > 0.0);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if s.b[478] {
            s.store_scalar(342, s.v[338]);
        }

        if (!s.b[478]) {
            s.store_scalar(342, 0.0);
        }

        s.v[52] = 0.001;

        s.v[336] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[63] = (1.0 / s.v[62]);

        s.v[279] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.b[479] = ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if s.b[479] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[479]) {
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

        s.b[480] = ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if s.b[480] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[480]) {
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

        s.store_scaled_offset(279, 74, (((-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))) + ((-0.05))), 10.0);

        s.b[481] = ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if s.b[481] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(279)), 0.1, 0.05);
        }

        if (!s.b[481]) {
            s.store_ad_value(70, A::add_scaled_inputs(A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1));
        }

        s.store_scaled_offset(279, 88, (((-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))) + ((-0.05))), 10.0);

        s.b[482] = ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if s.b[482] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(279)), 0.1, 0.05);
        }

        if (!s.b[482]) {
            s.store_ad_value(85, A::add_scaled_inputs(A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p66 * s.v[4])) + ((1.0 - s.v[4]) * p.p105));

        s.v[279] = ((0.05 - s.v[13]) / s.v[6]);

        s.b[483] = (0.05 < s.v[13]);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if s.b[483] {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[483]) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p64 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[15]) / s.v[6]);

        s.b[484] = (0.05 < s.v[15]);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[484]) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p80 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[21]) / s.v[6]);

        s.b[485] = (0.05 < s.v[21]);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[485]) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p71 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[18]) / s.v[6]);

        s.b[486] = (0.05 < s.v[18]);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if s.b[486] {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[486]) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[274]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[20]) / s.v[6]);

        s.b[487] = (0.05 < s.v[20]);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[487]) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p27 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[279] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[488] = (0.05 < s.v[56]);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[488]) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[101] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p138 * s.v[4])) + ((1.0 - s.v[4]) * p.p140));

        s.v[279] = ((0.05 - s.v[101]) / s.v[6]);

        s.b[489] = (0.05 < s.v[101]);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_scalar(102, (s.v[101] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[489]) {
            s.store_scalar(102, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scaled_powf_ad(103, A::div_from_scalar(p.p138, s.ad_value(102)), p.p139, p.p137);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.v[28] = (p.p54 * (((s.v[274] * p.p97)) as f64).exp());

        s.b[490] = (s.v[28] < s.v[340]);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.copy_ad(28, 340);
        }

        s.v[29] = (p.p56 * (((s.v[274] * (p.p98 - p.p96))) as f64).exp());

        s.v[30] = (p.p55 * (((s.v[274] * p.p101)) as f64).exp());

        s.b[491] = (s.v[30] < s.v[340]);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if s.b[491] {
            s.copy_ad(30, 340);
        }

        s.v[32] = (p.p57 * (((s.v[274] * p.p102)) as f64).exp());

        s.v[33] = (p.p58 * (((s.v[274] * p.p104)) as f64).exp());

        s.v[34] = (p.p59 * (((s.v[274] * p.p104)) as f64).exp());

        s.v[31] = (p.p60 * (((s.v[274] * p.p99)) as f64).exp());

        s.b[492] = (p.p122 != 0.0);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if s.b[492] {
            s.store_scalar(50, (p.p10 * (1.0 + (s.v[12] * p.p122))));
            s.store_scaled_offset(279, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[493] = (s.v[50] < 1.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if (s.b[492] && s.b[493]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(279)), s.v[52], 1.0);
        }

        if (s.b[492] && (!s.b[493])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]));
        }

        if s.b[492] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[492]) {
            s.store_scalar(48, p.p10);
        }

        s.b[494] = (p.p123 != 0.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_scalar(51, (p.p11 * (1.0 + (s.v[12] * p.p123))));
            s.store_scaled_offset(279, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[495] = (s.v[51] < 1.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[495]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(279)), s.v[52], 1.0);
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]));
        }

        if s.b[494] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[494]) {
            s.store_scalar(49, p.p11);
        }

        s.v[335] = (p.p43 * (1.0 + (p.p124 * s.v[12])));

        s.v[281] = (s.v[336] * s.v[336]);

        s.v[282] = (s.v[335] * s.v[335]);

        s.b[496] = (s.v[335] < 0.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if s.b[496] {
            s.store_scalar(334, ((0.5 * s.v[281]) / ((((s.v[282] + s.v[281])) as f64).sqrt() - s.v[335])));
        }

        if (!s.b[496]) {
            s.store_scalar(334, (0.5 * ((((s.v[282] + s.v[281])) as f64).sqrt() + s.v[335])));
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_from_scalar((s.v[274] * (((4.0 - p.p98) - p.p96) + p.p121)), s.ad_value(48))), A::exp(A::div_from_scalar(((-p.p105) * s.v[10]), s.ad_value(48))), p.p9);

        s.v[36] = (p.p12 * (((s.v[274] * (1.0 - p.p98))) as f64).exp());

        s.v[37] = (p.p30 * (((s.v[274] * (1.0 - p.p103))) as f64).exp());

        s.v[38] = ((p.p20 * (((s.v[274] * (6.0 - (2.0 * p.p21)))) as f64).exp()) * (((((-p.p113) * s.v[10]) / p.p21)) as f64).exp());

        s.v[39] = ((p.p31 * (((s.v[274] * (6.0 - (2.0 * p.p32)))) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p32)) as f64).exp());

        s.v[42] = ((p.p16 * ((((s.v[274] * ((4.0 - p.p97) + p.p121)) / p.p17)) as f64).exp()) * (((((-p.p111) * s.v[10]) / p.p17)) as f64).exp());

        s.v[44] = ((p.p18 * ((((s.v[274] * ((4.0 - p.p97) + p.p121)) / p.p19)) as f64).exp()) * (((((-p.p111) * s.v[10]) / p.p19)) as f64).exp());

        s.b[497] = (p.p24 == 1.0);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if s.b[497] {
            s.store_scalar(53, (p.p25 * (((((-p.p107) * s.v[10]) / p.p17)) as f64).exp()));
            s.store_scalar(54, (p.p28 * ((((-p.p106) * s.v[10])) as f64).exp()));
            s.store_scalar(45, (p.p26 * (((((-p.p108) * s.v[10]) / p.p19)) as f64).exp()));
        }

        s.v[43] = ((p.p29 * (((s.v[274] * ((4.0 - p.p103) + p.p121))) as f64).exp()) * ((((-p.p112) * s.v[10])) as f64).exp());

        s.v[46] = ((p.p22 * (((s.v[274] * (6.0 - (2.0 * p.p23)))) as f64).exp()) * (((((-p.p113) * s.v[10]) / p.p23)) as f64).exp());

        s.v[47] = ((p.p145 * (((s.v[274] * (4.0 / p.p146))) as f64).exp()) * (((((-p.p113) * s.v[10]) / p.p146)) as f64).exp());

        s.v[350] = ((p.p151 * ((s.v[4]) as f64).sqrt()) * (((p.p153 * s.v[12])) as f64).exp());

        s.store_powf_ad(275, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(276, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(275), p.p35), s.ad_value(276), (p.p66 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_ad_value(58, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(275), s.ad_value(14), s.ad_value(14), p.p34), s.ad_value(73), A::exp(A::sub_from_scalar(p.p35, s.ad_value(61))), (s.v[64] * s.v[64])));

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(277, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(278, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(277), p.p37), s.ad_value(278), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.store_ad_value(84, A::mul3_scaled_output(A::mul3_scaled_output(s.ad_value(277), s.ad_value(19), s.ad_value(19), p.p36), s.ad_value(90), A::exp(A::sub_from_scalar(p.p37, s.ad_value(83))), (s.v[66] * s.v[66])));

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

        s.b[498] = (s.v[2] < 525.0);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!s.b[498]) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p92 * (((s.v[274] * p.p96)) as f64).exp());

        s.b[499] = (p.p57 > 0.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if s.b[499] {
            s.store_scalar(108, (1.0 / s.v[32]));
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[500] = (s.v[108] > s.v[341]);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if (s.b[499] && s.b[500]) {
            s.copy_ad(108, 341);
        }

        if (!s.b[499]) {
            s.store_scalar(108, 0.0);
        }

        s.b[501] = (p.p58 > 0.0);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.store_scalar(109, (1.0 / s.v[33]));
        }

        s.b[502] = (s.v[109] > s.v[341]);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if (s.b[501] && s.b[502]) {
            s.copy_ad(109, 341);
        }

        if (!s.b[501]) {
            s.store_scalar(109, 0.0);
        }

        s.b[503] = (p.p59 > 0.0);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if s.b[503] {
            s.store_scalar(110, (1.0 / s.v[34]));
        }

        s.b[504] = (s.v[110] > s.v[341]);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if (s.b[503] && s.b[504]) {
            s.copy_ad(110, 341);
        }

        if (!s.b[503]) {
            s.store_scalar(110, 0.0);
        }

        s.store_scaled_voltage(244, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(245, ctx, nodes, Some(6), Some(8), p.p3);

        s.store_scaled_voltage(246, ctx, nodes, Some(6), Some(4), p.p3);

        s.store_scaled_voltage(247, ctx, nodes, Some(5), Some(4), p.p3);

        s.store_scaled_voltage(248, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(253, ctx, nodes, Some(3), Some(7), p.p3);

        s.store_scaled_voltage(250, ctx, nodes, Some(7), Some(8), p.p3);

        s.store_scaled_voltage(259, ctx, nodes, Some(2), Some(4), p.p3);

        s.store_scaled_voltage(260, ctx, nodes, Some(1), Some(5), p.p3);

        s.store_scaled_voltage(263, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(264, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(252, ctx, nodes, Some(10), Some(7), p.p3);

        s.store_scaled_voltage(251, ctx, nodes, Some(9), Some(10), p.p3);

        s.store_ad_value(249, A::add_scaled_inputs4(s.ad_value(248), 1.0, s.ad_value(245), 1.0, s.ad_value(250), -1.0, s.ad_value(252), -1.0));

        s.store_ad_value(262, A::add_scaled_inputs4(s.ad_value(260), 1.0, s.ad_value(264), (-1.0), s.ad_value(249), 1.0, s.ad_value(251), -1.0));

        s.store_add(261, 264, 262);

        s.store_sub(255, 253, 252);

        s.store_sub(254, 255, 251);

        s.b[505] = ((s.v[245] * s.v[8]) < p.p147);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if s.b[505] {
            s.store_exp_scaled_input(265, 245, s.v[8]);
        }

        if (!s.b[505]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(265, 295, A::scale_offset(s.ad_value(245), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[506] = (((s.v[246] * s.v[8]) / s.v[48]) < p.p147);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if s.b[506] {
            s.store_exp_ad(266, A::div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0));
        }

        if (!s.b[506]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(266, 295, A::div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0), (((-p.p147)) + (1.0)));
        }

        s.b[507] = ((s.v[249] * s.v[8]) < p.p147);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if s.b[507] {
            s.store_exp_scaled_input(268, 249, s.v[8]);
        }

        if (!s.b[507]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(268, 295, A::scale_offset(s.ad_value(249), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[508] = ((s.v[248] * s.v[8]) < p.p147);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if s.b[508] {
            s.store_exp_scaled_input(267, 248, s.v[8]);
        }

        if (!s.b[508]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(267, 295, A::scale_offset(s.ad_value(248), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[509] = ((s.v[261] * s.v[8]) < p.p147);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_exp_scaled_input(269, 261, s.v[8]);
        }

        if (!s.b[509]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(269, 295, A::scale_offset(s.ad_value(261), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[510] = ((s.v[253] * s.v[8]) < p.p147);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if s.b[510] {
            s.store_exp_scaled_input(256, 253, s.v[8]);
        }

        if (!s.b[510]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(256, 295, A::scale_offset(s.ad_value(253), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[511] = ((s.v[254] * s.v[8]) < p.p147);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if s.b[511] {
            s.store_exp_scaled_input(257, 254, s.v[8]);
        }

        if (!s.b[511]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(257, 295, A::scale_offset(s.ad_value(254), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[512] = ((s.v[255] * s.v[8]) < p.p147);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if s.b[512] {
            s.store_exp_scaled_input(258, 255, s.v[8]);
        }

        if (!s.b[512]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(258, 295, A::scale_offset(s.ad_value(255), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[513] = (((s.v[261] - s.v[16]) * s.v[8]) < p.p147);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_ad_value(272, A::exp_scaled_input(A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[513]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(272, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[514] = (((s.v[249] - s.v[16]) * s.v[8]) < p.p147);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if s.b[514] {
            s.store_ad_value(270, A::exp_scaled_input(A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[514]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(270, 295, A::sub_scaled_inputs(s.ad_value(249), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[515] = (((s.v[245] - s.v[16]) * s.v[8]) < p.p147);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if s.b[515] {
            s.store_ad_value(271, A::exp_scaled_input(A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[515]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(271, 295, A::sub_scaled_inputs(s.ad_value(245), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[516] = (((s.v[244] - s.v[16]) * s.v[8]) < p.p147);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_ad_value(273, A::exp_scaled_input(A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[516]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(273, 295, A::sub_scaled_inputs(s.ad_value(244), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(111, 271, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(112, 273, 4.0, 1.0);

        s.store_ad_value(113, A::div_scaled_value_offset_denominator(s.ad_value(273), 2.0, s.ad_value(112), 1.0, 1.0));

        s.b[517] = (s.v[113] < p.p149);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if s.b[517] {
            s.store_scalar(113, p.p149);
        }

        s.store_ad_value(114, A::add_scaled_inputs3(s.ad_value(111), s.v[6], s.ad_value(112), ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(111), 1.0, 1.0, A::offset(s.ad_value(112), 1.0), 1.0)), (-s.v[6])));

        s.store_scaled_add(115, 114, 250, 1.0 / (s.v[31]));

        s.b[518] = (s.v[115] > 0.0);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        s.b[519] = (s.v[244] < 100.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[519]) {
            s.copy_ad(297, 244);
        }

        if (s.b[518] && (!s.b[519])) {
            s.store_offset_ln_ad(297, A::offset(s.ad_value(244), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[518] {
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(16), 1.0, A::ln(A::scale_offset(s.ad_value(115), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), s.ad_value(297), -1.0));
            s.store_scale(292, 16, 0.2);
            s.store_square(281, 292);
            s.store_square(282, 116);
        }

        s.b[520] = (s.v[116] < 0.0);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[520]) {
            s.store_ad_value(117, A::div_scaled_inputs(s.ad_value(281), 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)), 1.0));
        }

        if (s.b[518] && (!s.b[520])) {
            s.store_scaled_add_ad_lhs(117, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 116, 0.5);
        }

        if s.b[518] {
            s.store_ad_value(118, A::div_scaled_product_offset_rhs(s.ad_value(117), s.ad_value(117), (p.p62 * p.p61), 1.0, A::scaled_offset(s.ad_value(117), (p.p62 * s.v[31]), p.p61), 1.0));
            s.store_div(285, 115, 118);
            s.store_scaled_offset(279, 285, (-1.0), 1.0 / (p.p63));
        }

        s.b[521] = (s.v[285] < 1.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[521]) {
            s.store_offset_scaled_ad(283, A::ln_one_plus_exp(s.ad_value(279)), p.p63, 1.0);
        }

        if (s.b[518] && (!s.b[521])) {
            s.store_ad_value(283, A::add_scaled_inputs(s.ad_value(285), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p63));
        }

        if s.b[518] {
            s.store_scale(119, 283, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
            s.store_scale(120, 117, 1.0 / ((p.p62 * p.p61)));
            s.store_ad_value(121, A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(119), s.ad_value(120), A::offset(s.ad_value(120), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(119), 2.0, A::offset(s.ad_value(120), 1.0)), 1.0));
            s.store_div_ad(122, A::add_scaled_sub_value_product(1.0, s.ad_value(121), 1.0, s.ad_value(113), s.ad_value(121), 1.0), A::offset(A::mul(s.ad_value(113), s.ad_value(121)), 1.0));
            s.store_scaled_mul(124, 115, 122, ((0.5 * s.v[31]) * s.v[8]));
            s.store_ad_value(286, A::add_scaled_offset_product_rhs(s.ad_value(124), 2.0, s.ad_value(113), A::add(s.ad_value(113), s.ad_value(124)), 1.0, 1.0));
            s.store_scaled_offset(125, 124, (-1.0), 0.5);
            s.store_add_ad_lhs(280, A::square(s.ad_value(125)), 286);
        }

        s.b[522] = (s.v[124] >= 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[522]) {
            s.store_add_ad_rhs(126, 125, A::sqrt(s.ad_value(280)));
        }

        if (s.b[518] && (!s.b[522])) {
            s.store_div_ad_rhs(126, 286, A::sub(A::sqrt(s.ad_value(280)), s.ad_value(125)));
        }

        s.b[523] = (s.v[126] < p.p148);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[523]) {
            s.store_scalar(126, p.p148);
        }

        if s.b[518] {
            s.store_mul_ad_product_rhs(128, 126, A::offset(s.ad_value(126), 1.0), A::exp_scaled_input(s.ad_value(16), s.v[8]));
            s.store_scaled_offset(130, 115, (-p.p62), (0.5 * p.p61));
            s.store_scale(131, 115, ((p.p61 * s.v[31]) * p.p62));
            s.store_add_ad_rhs(132, 130, A::sqrt(A::add(A::square(s.ad_value(130)), s.ad_value(131))));
        }

        s.b[524] = (p.p73 == 0.0);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[524]) {
            s.store_scale(133, 17, 0.1);
        }

        if (s.b[518] && (!s.b[524])) {
            s.store_mul_offset_ad_rhs(133, 17, A::div_scaled_inputs(s.ad_value(115), 2.0, A::add(s.ad_value(115), s.ad_value(118)), 1.0), 0.1);
        }

        if s.b[518] {
            s.store_ad_value(134, A::div_scaled_value_offset_denominator(s.ad_value(115), p.p62, s.ad_value(115), p.p62, 1.0));
            s.store_div_from_scalar_offset_input(210, p.p62, 115, p.p62);
        }

        if (!s.b[518]) {
            s.store_scalar(118, 0.0);
            s.store_ad_value(126, A::div_scaled_value_offset_denominator(s.ad_value(271), 2.0, s.ad_value(111), 1.0, 1.0));
            s.copy_ad(128, 265);
        }

        s.b[525] = ((((s.v[250]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[114]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[111] + s.v[112]))));
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((!s.b[518]) && s.b[525]) {
            s.store_scaled_add(135, 126, 113, 0.5);
            s.store_ad_value(122, A::div_scaled_value_offset_denominator(s.ad_value(135), 1.0, s.ad_value(135), 1.0, 1.0));
        }

        if ((!s.b[518]) && (!s.b[525])) {
            s.store_div_ad_rhs(122, 114, A::add_scaled_inputs3(s.ad_value(114), 1.0, s.ad_value(245), 1.0, s.ad_value(244), -1.0));
        }

        if (!s.b[518]) {
            s.copy_ad(132, 250);
            s.store_scale(133, 17, 0.1);
            s.copy_ad(134, 115);
            s.store_sub_from_scalar_ad(210, 1.0, A::scale(s.ad_value(134), 1.0 / (p.p62)));
        }

        s.store_scale(136, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(293, 14, 0.1);

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(246), 1.0, s.ad_value(136), (-1.0), s.ad_value(293), 1.0));

        s.b[526] = (s.v[246] < s.v[136]);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_add_scaled_product(137, s.ad_value(246), 1.0, s.ad_value(293), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[526]) {
            s.store_add_scaled_product(137, s.ad_value(136), 1.0, s.ad_value(293), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (1.0 - p.p67));

        s.store_ad_value(138, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(246), 3.0, s.ad_value(137), (-3.0)));

        s.b[527] = (p.p74 == 1.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if s.b[527] {
            s.copy_ad(139, 244);
        }

        s.b[528] = (p.p74 == 2.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((!s.b[527]) && s.b[528]) {
            s.store_add(139, 244, 132);
        }

        if ((!s.b[527]) && (!s.b[528])) {
            s.copy_ad(139, 245);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_div_ad(140, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(141, 17, 1.0, A::powf(s.ad_value(140), ((-1.0) / p.p72)));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(139), 1.0, s.ad_value(141), (-1.0), s.ad_value(133), 1.0));

        s.b[529] = (s.v[139] < s.v[141]);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_add_scaled_product(142, s.ad_value(139), 1.0, s.ad_value(133), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[529]) {
            s.store_add_scaled_product(142, s.ad_value(141), 1.0, s.ad_value(133), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf(143, 210, p.p76);

        s.store_add_ad(144, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(143), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(142), s.ad_value(17))), (1.0 - p.p72))), 1.0 / ((1.0 - p.p72))), A::mul3(s.ad_value(143), s.ad_value(140), A::sub(s.ad_value(139), s.ad_value(142))));

        s.store_add_scaled_product(145, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(144)), 1.0, s.ad_value(25), s.ad_value(244), 1.0);

        s.store_scale(146, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(147, 146, 266);

        s.store_ad_value(149, A::div_scaled_value_offset_denominator(s.ad_value(147), 1.0, A::sqrt(A::offset(s.ad_value(147), 1.0)), 1.0, 1.0));

        s.store_pow_ad(129, s.ad_value(128), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(148, 146, 129);

        s.store_ad_value(150, A::div_scaled_value_offset_denominator(s.ad_value(148), 1.0, A::sqrt(A::offset(s.ad_value(148), 1.0)), 1.0, 1.0));

        s.b[530] = (p.p92 == 0.0);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if s.b[530] {
            s.store_add_ad(151, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));
        }

        if (!s.b[530]) {
            s.store_offset_scaled_div(289, 138, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_scaled_div(290, 145, 40, (-(s.v[99] * s.v[8])));
            s.store_scaled_sub_ad(151, A::exp(s.ad_value(289)), A::exp(s.ad_value(290)), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 151);

        s.b[531] = (s.v[151] < 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if s.b[531] {
            s.store_div_from_scalar_sub_ad(152, (0.5 * s.v[281]), A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(151));
        }

        if (!s.b[531]) {
            s.store_scaled_add_ad_lhs(152, A::sqrt(A::offset(s.ad_value(282), s.v[281])), 151, 0.5);
        }

        s.store_mul_offset_ad_rhs(153, 152, A::add_scaled_inputs(s.ad_value(149), 0.5, s.ad_value(150), 0.5), 1.0);

        s.store_scaled_mul(154, 35, 129, p.p15);

        s.store_mul(155, 35, 266);

        s.store_ad_value(156, A::div_scaled_inputs2(s.ad_value(155), 1.0, s.ad_value(154), (-1.0), s.ad_value(153), 1.0));

        s.store_scale(279, 246, 10000.0);

        s.b[532] = (s.v[246] < 0.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if s.b[532] {
            s.store_scaled_ln_one_plus_exp(296, 279, 0.0001);
        }

        if (!s.b[532]) {
            s.store_ad_value(296, A::add_scaled_inputs(s.ad_value(246), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.0001));
        }

        s.store_scale(298, 296, 1.0 / (p.p152));

        s.b[533] = (s.v[298] < p.p147);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if s.b[533] {
            s.store_exp(299, 298);
        }

        if (!s.b[533]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_rhs(299, 295, 298, (((-p.p147)) + (1.0)));
        }

        s.store_scaled_offset(351, 299, (-1.0), s.v[350]);

        s.store_scaled_offset(279, 246, (-p.p154), 1000.0);

        s.b[534] = (s.v[246] < p.p154);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if s.b[534] {
            s.store_ad_value(300, A::sub_scaled_inputs(s.ad_value(246), 1.0, A::ln_one_plus_exp(s.ad_value(279)), 0.001));
        }

        if (!s.b[534]) {
            s.store_sub_from_scalar_ad(300, p.p154, A::scale(A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.001));
        }

        s.store_mul_scaled_ad_rhs(352, 300, p.p155, A::powf(A::sub_from_scalar(p.p154, s.ad_value(300)), 2.0));

        s.b[535] = (((s.v[246] * s.v[8]) / p.p17) < p.p147);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if s.b[535] {
            s.store_exp_scaled_input(296, 246, (s.v[8] * 1.0 / (p.p17)));
        }

        if (!s.b[535]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(246), (s.v[8] * 1.0 / (p.p17)), (((-p.p147)) + (1.0))));
        }

        s.b[536] = (p.p24 == 1.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (((s.v[246] - s.v[55]) * s.v[8]) < p.p147);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (s.b[536] && s.b[537]) {
            s.store_ad_value(298, A::exp_scaled_input(A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]));
        }

        if (s.b[536] && (!s.b[537])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(298, 295, A::sub_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(55), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[538] = (((s.v[156] / s.v[35]) - 1000.0) < 40.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if (s.b[536] && s.b[538]) {
            s.store_exp_ad(299, A::offset(A::div(s.ad_value(156), s.ad_value(35)), (-1000.0)));
        }

        if (s.b[536] && (!s.b[538])) {
            s.store_scalar(295, ((40.0) as f64).exp());
            s.store_mul_offset_ad_rhs(299, 295, A::div(s.ad_value(156), s.ad_value(35)), (((((-1000.0)) + ((-40.0)))) + (1.0)));
        }

        if s.b[536] {
            let assign3980_ad_e3745: A = A::add(A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(296), (-1.0), s.v[42]), 1.0, A::div_scaled_product_offset_denominator(s.ad_value(53), A::offset(s.ad_value(296), (-1.0)), 2.0, A::sqrt(A::scale_offset(s.ad_value(298), 4.0, 1.0)), 1.0, 1.0), A::div(s.ad_value(145), s.ad_value(40)), 1.0, 1.0), A::div_scaled_product3(s.ad_value(54), A::offset(s.ad_value(128), (-1.0)), s.ad_value(299), 1.0, A::offset(s.ad_value(299), 1.0), 1.0));
            s.store_ad_value(158, assign3980_ad_e3745);
        }

        s.b[539] = (p.p93 == 0.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((!s.b[536]) && s.b[539]) {
            s.store_scaled_offset(158, 296, (-1.0), s.v[42]);
        }

        if ((!s.b[536]) && (!s.b[539])) {
            s.store_ad_value(158, A::add_scaled_offset_product_lhs(A::scaled_offset(s.ad_value(296), (-1.0), (1.0 - p.p93)), s.v[42], A::add(s.ad_value(296), s.ad_value(128)), (-2.0), A::offset(A::div(s.ad_value(145), s.ad_value(40)), 1.0), (p.p93 * s.v[42])));
        }

        s.b[540] = (((s.v[247] * s.v[8]) / p.p19) < p.p147);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if s.b[540] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p19)));
        }

        if (!s.b[540]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(247), (s.v[8] * 1.0 / (p.p19)), (((-p.p147)) + (1.0))));
        }

        s.b[541] = (p.p24 == 1.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        s.b[542] = (((s.v[247] - s.v[55]) * s.v[8]) < p.p147);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if (s.b[541] && s.b[542]) {
            s.store_ad_value(298, A::exp_scaled_input(A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]));
        }

        if (s.b[541] && (!s.b[542])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(298, 295, A::sub_scaled_inputs(s.ad_value(247), s.v[8], s.ad_value(55), s.v[8]), (((-p.p147)) + (1.0)));
        }

        if s.b[541] {
            s.store_add_ad(159, A::scaled_offset(s.ad_value(296), (-1.0), s.v[44]), A::div_scaled_product_offset_denominator(s.ad_value(45), A::offset(s.ad_value(296), (-1.0)), 2.0, A::sqrt(A::scale_offset(s.ad_value(298), 4.0, 1.0)), 1.0, 1.0));
        }

        if (!s.b[541]) {
            s.store_scaled_offset(159, 296, (-1.0), s.v[44]);
        }

        s.b[543] = (((s.v[246] * s.v[8]) / p.p21) < p.p147);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_exp_scaled_input(296, 246, (s.v[8] * 1.0 / (p.p21)));
        }

        if (!s.b[543]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(246), (s.v[8] * 1.0 / (p.p21)), (((-p.p147)) + (1.0))));
        }

        s.store_scaled_offset(160, 296, (-1.0), s.v[38]);

        s.b[544] = (((s.v[247] * s.v[8]) / p.p23) < p.p147);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p23)));
        }

        if (!s.b[544]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(247), (s.v[8] * 1.0 / (p.p23)), (((-p.p147)) + (1.0))));
        }

        s.store_scaled_offset(162, 296, (-1.0), s.v[46]);

        s.b[545] = (((s.v[249] * s.v[8]) / p.p32) < p.p147);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_exp_scaled_input(296, 249, (s.v[8] * 1.0 / (p.p32)));
        }

        if (!s.b[545]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(249), (s.v[8] * 1.0 / (p.p32)), (((-p.p147)) + (1.0))));
        }

        s.store_scaled_offset(161, 296, (-1.0), s.v[39]);

        s.b[546] = (((s.v[247] * s.v[8]) / p.p146) < p.p147);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if s.b[546] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p146)));
        }

        if (!s.b[546]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(247), (s.v[8] * 1.0 / (p.p146)), (((-p.p147)) + (1.0))));
        }

        s.store_scaled_offset(163, 296, (-1.0), s.v[47]);

        s.b[547] = (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[246] < 0.0));
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        s.b[548] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p147);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[547] && s.b[548]) {
            s.store_exp_ad(68, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))));
        }

        if (s.b[547] && (!s.b[548])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(68, 295, A::mul_sub_from_scalar_rhs(s.ad_value(61), 1.0, A::div_from_scalar(s.v[62], A::scale(s.ad_value(59), 2.0))), (((-p.p147)) + (1.0)));
        }

        if s.b[547] {
            s.store_mul(275, 246, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(275)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale_offset(A::scale(s.ad_value(275), (3.0 * (p.p67 - 1.0))), (-p.p67), (((1.0 - (p.p67 * p.p67))) * (p.p67))), A::mul3_scaled_output(s.ad_value(275), s.ad_value(275), A::offset(s.ad_value(275), (p.p67 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(275, A::div_scaled_product_by_product(s.ad_value(246), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0));
        }

        s.b[549] = (s.v[275] < (-0.001));
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        s.b[550] = (s.v[275] < p.p147);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if ((s.b[547] && s.b[549]) && s.b[550]) {
            s.store_exp(91, 275);
        }

        if ((s.b[547] && s.b[549]) && (!s.b[550])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_rhs(91, 295, 275, (((-p.p147)) + (1.0)));
        }

        if (s.b[547] && s.b[549]) {
            s.store_mul_scaled_ad_rhs(69, 246, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(91)), s.ad_value(275)), 1.0));
        }

        if (s.b[547] && (!s.b[549])) {
            s.store_mul_ad_affine_product_rhs(69, 246, s.ad_value(275), A::offset(A::mul_scaled_lhs(s.ad_value(275), 0.3333333333333333, A::scale_offset(s.ad_value(275), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[547] {
            s.store_mul_ad_affine_product_lhs(57, A::mul3_scaled_output(s.ad_value(58), s.ad_value(69), s.ad_value(59), 2.0), s.ad_value(68), s.v[63], 0.0, 65);
        }

        if (!s.b[547]) {
            s.store_scalar(69, 0.0);
            s.store_scalar(57, 0.0);
        }

        s.b[551] = (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[244] < 0.0));
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if s.b[551] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(244), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[552] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p147);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[551] && s.b[552]) {
            s.store_exp_ad(78, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))));
        }

        if (s.b[551] && (!s.b[552])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(78, 295, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::div_from_scalar(s.v[79], A::scale(s.ad_value(77), 2.0))), (((-p.p147)) + (1.0)));
        }

        if s.b[551] {
            s.store_mul(277, 244, 67);
        }

        if s.b[551] {
            let assign4580_ad_e4435: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(277)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(277), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(277), s.ad_value(277), A::offset(s.ad_value(277), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(80, assign4580_ad_e4435);
        }

        if s.b[551] {
            s.store_ad_value(277, A::div_scaled_product_by_product(s.ad_value(244), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0));
        }

        s.b[553] = (s.v[277] < (-0.001));
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        s.b[554] = (s.v[277] < p.p147);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if ((s.b[551] && s.b[553]) && s.b[554]) {
            s.store_exp(92, 277);
        }

        if ((s.b[551] && s.b[553]) && (!s.b[554])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_rhs(92, 295, 277, (((-p.p147)) + (1.0)));
        }

        if (s.b[551] && s.b[553]) {
            s.store_mul_scaled_ad_rhs(81, 244, -1.0, A::offset(A::div(A::sub_from_scalar(1.0, s.ad_value(92)), s.ad_value(277)), 1.0));
        }

        if (s.b[551] && (!s.b[553])) {
            s.store_mul_ad_affine_product_rhs(81, 244, s.ad_value(277), A::offset(A::mul_scaled_lhs(s.ad_value(277), 0.3333333333333333, A::scale_offset(s.ad_value(277), 0.25, 1.0)), 1.0), 0.5, 0.0);
        }

        if s.b[551] {
            s.store_mul_ad_affine_product_lhs(82, A::mul3_scaled_output(s.ad_value(84), s.ad_value(81), s.ad_value(77), 2.0), s.ad_value(78), s.v[89], 0.0, 67);
        }

        if (!s.b[551]) {
            s.store_scalar(81, 0.0);
            s.store_scalar(82, 0.0);
        }

        s.store_mul(165, 146, 268);

        s.store_scale(166, 270, 4.0);

        s.store_ad_value(168, A::div_scaled_inputs2(s.ad_value(165), 1.0, s.ad_value(146), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0), 1.0));

        s.store_ad_value(167, A::div_scaled_value_offset_denominator(s.ad_value(166), 1.0, A::sqrt(A::offset(s.ad_value(166), 1.0)), 1.0, 1.0));

        s.store_ad_value(164, A::div_scaled_offset_numerator(s.ad_value(268), (2.0 * s.v[43]), ((-1.0) * (2.0 * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(268), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0));

        s.b[555] = (p.p8 == 1.0);
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if s.b[555] {
            s.store_ad_value(182, A::div_scaled_inputs2(s.ad_value(265), ((p.p143 * 2.0) * s.v[104]), s.ad_value(256), (-((p.p143 * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::add_scaled_inputs(s.ad_value(265), (4.0 * (s.v[104] / s.v[106])), s.ad_value(256), (p.p144 * (4.0 * (s.v[104] / s.v[106])))), 1.0)), 1.0), 1.0));
            s.store_ad_value(179, A::div_scaled_inputs2(s.ad_value(268), (((1.0 - p.p143) * 2.0) * s.v[104]), s.ad_value(258), (-(((1.0 - p.p143) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::add_scaled_inputs(s.ad_value(268), (4.0 * (s.v[104] / s.v[106])), s.ad_value(258), (p.p144 * (4.0 * (s.v[104] / s.v[106])))), 1.0)), 1.0), 1.0));
        }

        if (!s.b[555]) {
            s.store_ad_value(182, A::div_scaled_offset_numerator(s.ad_value(265), ((p.p143 * 2.0) * s.v[104]), ((-1.0) * ((p.p143 * 2.0) * s.v[104])), A::offset(A::sqrt(A::scale_offset(s.ad_value(265), (4.0 * (s.v[104] / s.v[106])), 1.0)), 1.0), 1.0));
            s.store_ad_value(179, A::div_scaled_offset_numerator(s.ad_value(268), (((1.0 - p.p143) * 2.0) * s.v[104]), ((-1.0) * (((1.0 - p.p143) * 2.0) * s.v[104])), A::offset(A::sqrt(A::scale_offset(s.ad_value(268), (4.0 * (s.v[104] / s.v[106])), 1.0)), 1.0), 1.0));
        }

        s.store_add_scaled_product(181, A::div_scaled_offset_numerator(s.ad_value(256), (2.0 * s.v[105]), ((-1.0) * (2.0 * s.v[105])), A::offset(A::sqrt(A::scale_offset(s.ad_value(256), ((p.p144 * 4.0) * (s.v[105] / s.v[107])), 1.0)), 1.0), 1.0), 1.0, s.ad_value(253), s.ad_value(342), 1.0);

        s.v[180] = 0.0;

        s.b[556] = ((p.p5 > 0.0) && (p.p33 > 0.0));
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_scale(164, 164, s.v[157]);
            s.store_scale(179, 179, s.v[157]);
            s.store_ad_value(171, A::div_scaled_offset_numerator(s.ad_value(269), ((p.p33 * 2.0) * s.v[43]), ((-1.0) * ((p.p33 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0));
        }

        s.b[557] = (p.p8 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[557]) {
            s.store_ad_value(172, A::div_scaled_inputs2(s.ad_value(269), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), s.ad_value(257), (-((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::add_scaled_inputs(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), s.ad_value(257), (p.p144 * ((4.0 * s.v[104]) / s.v[106]))), 1.0)), 1.0), 1.0));
        }

        if (s.b[556] && (!s.b[557])) {
            s.store_ad_value(172, A::div_scaled_offset_numerator(s.ad_value(269), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), ((-1.0) * ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), 1.0)), 1.0), 1.0));
        }

        s.b[558] = (p.p5 == 1.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[558]) {
            s.store_scalar(291, ((p.p33 * (s.v[43] + s.v[104])) * s.v[32]));
            s.store_offset_scaled_ad(173, A::ln_scaled_input(s.ad_value(291), s.v[8]), (-s.v[6]), ((2.0) * (s.v[6])));
            s.store_sub(284, 261, 173);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[556] && s.b[558]) {
            s.store_scalar(281, (0.11 * 0.11));
            s.store_square(282, 284);
        }

        s.b[559] = (s.v[284] < 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if ((s.b[556] && s.b[558]) && s.b[559]) {
            s.store_ad_value(174, A::div_scaled_inputs(s.ad_value(281), 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)), 1.0));
        }

        if ((s.b[556] && s.b[558]) && (!s.b[559])) {
            s.store_scaled_add_ad_lhs(174, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 284, 0.5);
        }

        if (s.b[556] && s.b[558]) {
            s.store_div_ad_rhs(175, 174, A::add_scaled_inputs4(s.ad_value(291), 1.0, s.ad_value(171), s.v[32], s.ad_value(172), s.v[32], s.ad_value(174), 1.0));
        }

        if (s.b[556] && (!s.b[558])) {
            s.store_scalar(173, 0.0);
            s.store_scalar(284, 0.0);
            s.store_scalar(174, 0.0);
            s.store_scalar(175, 1.0);
        }

        if s.b[556] {
            s.store_mul(176, 175, 171);
            s.store_mul(180, 175, 172);
        }

        s.b[560] = (p.p84 == 1.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if s.b[560] {
            s.store_add(347, 248, 244);
            s.store_scalar(281, (1e-6 * 1e-6));
            s.store_scaled_mul(282, 347, 347, ((-1.0) * (-1.0)));
        }

        s.b[561] = (((-1.0) * s.v[347]) < 0.0);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if (s.b[560] && s.b[561]) {
            s.store_ad_value(348, A::div_scaled_inputs(s.ad_value(281), 0.5, A::sub_scaled_inputs(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 1.0, s.ad_value(347), (-1.0)), 1.0));
        }

        if (s.b[560] && (!s.b[561])) {
            s.store_ad_value(348, A::add_scaled_inputs(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 0.5, s.ad_value(347), ((-1.0) * 0.5)));
        }

        if s.b[560] {
            s.store_scalar(349, (1.0 / (1.0 - ((s.v[343]) as f64).powf(p.p82))));
            s.store_scalar(344, (s.v[343] * p.p81));
            s.store_scaled_square(346, 349, (((s.v[343]) as f64).powf((p.p82 - 1.0)) * (p.p82 * 1.0 / (p.p81))));
        }

        s.b[562] = (s.v[348] < s.v[344]);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if (s.b[560] && s.b[562]) {
            s.store_div_from_scalar_sub_from_scalar_ad(345, 1.0, 1.0, A::powf(A::scale(s.ad_value(348), 1.0 / (p.p81)), p.p82));
        }

        if (s.b[560] && (!s.b[562])) {
            s.store_add_scaled_product(345, s.ad_value(349), 1.0, A::sub(s.ad_value(348), s.ad_value(344)), s.ad_value(346), 1.0);
        }

        if (!s.b[560]) {
            s.store_scalar(345, 1.0);
        }

        s.store_mul(82, 82, 345);

        s.store_mul(164, 164, 345);

        s.store_mul(161, 161, 345);

        s.store_mul(176, 176, 345);

        s.store_add_ad(183, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 183);

        s.b[563] = (s.v[183] < 0.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if s.b[563] {
            s.store_div_from_scalar_sub_ad(184, (0.5 * s.v[281]), A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(183));
        }

        if (!s.b[563]) {
            s.store_scaled_add_ad_lhs(184, A::sqrt(A::offset(s.ad_value(282), s.v[281])), 183, 0.5);
        }

        s.store_mul_offset_ad_rhs(185, 184, A::add_scaled_inputs(s.ad_value(149), 0.5, s.ad_value(150), 0.5), 1.0);

        s.store_div_from_scalar(187, s.v[29], 185);

        s.b[564] = (s.v[187] < s.v[340]);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if s.b[564] {
            s.copy_ad(187, 340);
        }

        s.store_scale(186, 187, 3.0);

        s.store_ad_value(188, A::div_scaled_inputs2(A::scaled_offset(s.ad_value(267), (-1.0), (2.0 * s.v[6])), 1.0, s.ad_value(248), 1.0, s.ad_value(186), 1.0));

        s.b[565] = (s.v[156] > 0.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        s.b[566] = (p.p39 == 1.0);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        s.b[567] = (s.v[244] < p.p44);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        s.b[568] = (((-s.v[156]) / p.p42) < p.p147);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[568]) {
            s.store_exp_scaled_input(332, 156, (-1.0 / (p.p42)));
        }

        if (((s.b[565] && s.b[566]) && s.b[567]) && (!s.b[568])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(332, 295, A::scale_offset(s.ad_value(156), (-1.0 / (p.p42)), (((-p.p147)) + (1.0))));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_sub_from_scalar_lhs(333, p.p44, 244, 332);
        }

        s.b[569] = (((-s.v[334]) * ((s.v[333]) as f64).powf(p.p41)) < p.p147);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[569]) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(334), -1.0, A::powf(s.ad_value(333), p.p41)));
        }

        if (((s.b[565] && s.b[566]) && s.b[567]) && (!s.b[569])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(337, 295, A::mul_scaled_lhs(s.ad_value(334), -1.0, A::powf(s.ad_value(333), p.p41)), (((-p.p147)) + (1.0)));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_ad_product_lhs(207, A::div_from_scalar(p.p40, s.ad_value(334)), s.ad_value(333), 337);
        }

        s.b[570] = (p.p39 == 2.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        s.b[571] = (s.v[244] < s.v[16]);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_scalar(196, ((2.0 * p.p46) / (p.p45 * p.p45)));
            s.store_ad_value(280, A::div_scaled_inputs2(s.ad_value(16), 1.0, s.ad_value(244), (-1.0), s.ad_value(210), 1.0));
            s.store_sqrt_ad(197, A::div_scaled_inputs(s.ad_value(280), 2.0, s.ad_value(196), 1.0));
        }

        s.b[572] = (p.p7 == 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[572]) {
            s.store_scalar(198, p.p45);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[572])) {
            s.store_sub_from_scalar_ad(123, 1.0, A::scale(s.ad_value(122), 0.5));
            s.store_scaled_mul(198, 123, 123, p.p45);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_product(199, s.ad_value(197), s.ad_value(198), 1.0, A::sqrt(A::add(A::square(s.ad_value(197)), A::square(s.ad_value(198)))), 1.0);
            s.store_ad_value(200, A::div_scaled_inputs2(s.ad_value(16), 1.0, s.ad_value(244), (-1.0), s.ad_value(199), 1.0));
            s.store_add_ad_rhs(201, 200, A::mul3_scaled_output(s.ad_value(199), s.ad_value(196), s.ad_value(210), 0.5));
        }

        s.b[573] = (p.p7 == 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[573]) {
            s.copy_ad(202, 201);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[573])) {
            s.store_offset_scaled(203, 122, ((2.0) * ((2.0 * p.p47))), (((2.0 * p.p47)) + (1.0)));
            s.store_scalar(204, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
            s.store_sub_ad_rhs(205, 200, A::mul3_scaled_output(s.ad_value(199), s.ad_value(196), A::sub(s.ad_value(204), A::div_scaled_inputs(s.ad_value(156), 1.0, s.ad_value(203), p.p62)), 0.5));
            s.store_add_scaled_product(280, A::mul3_scaled_output(s.ad_value(200), s.ad_value(200), s.ad_value(134), (0.1 * 1.0 / (p.p62))), 1.0, A::sub(s.ad_value(205), s.ad_value(201)), A::sub(s.ad_value(205), s.ad_value(201)), 1.0);
            s.store_ad_value(202, A::add_scaled_inputs3(s.ad_value(205), 0.5, s.ad_value(201), 0.5, A::sqrt(s.ad_value(280)), 0.5));
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_ad_value(287, A::div_scaled_inputs2(s.ad_value(202), 1.0, s.ad_value(200), (-1.0), s.ad_value(202), 1.0));
        }

        s.b[574] = (((s.v[287]) as f64).abs() > 1e-7);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[574]) {
            s.store_scaled_div(206, 199, 287, 0.5);
            s.store_mul_ad(207, A::mul3(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(202), s.ad_value(206)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::div(s.ad_value(198), s.ad_value(206)), 1.0))));
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[574])) {
            s.store_mul_ad_product_rhs(207, 0, s.ad_value(198), A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0)));
        }

        s.b[575] = (p.p39 == 3.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (s.v[244] < p.p44);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_ad(211, A::powf(A::sub_from_scalar(p.p44, s.ad_value(244)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(156), 1.0, s.ad_value(156), p.p48, 1.0)), p.p49));
        }

        s.b[577] = (p.p7 == 0.0);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[577]) {
            s.copy_ad(212, 211);
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_scaled_offset(213, 156, (-p.p52), 1.0 / (p.p48));
            s.store_scaled_offset(279, 213, (-1.0), 1.0 / (p.p51));
        }

        s.b[578] = (s.v[213] < 1.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if ((((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) && s.b[578]) {
            s.store_offset_scaled_ad(214, A::ln_one_plus_exp(s.ad_value(279)), p.p51, 1.0);
        }

        if ((((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) && (!s.b[578])) {
            s.store_ad_value(214, A::add_scaled_inputs(s.ad_value(213), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p51));
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_mul_powf_ad_rhs(212, 211, s.ad_value(214), p.p50);
        }

        s.b[579] = (((-s.v[334]) * s.v[212]) < p.p147);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[579]) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(334), -1.0, s.ad_value(212)));
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[579])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(337, 295, A::mul_scaled_lhs(s.ad_value(334), -1.0, s.ad_value(212)), (((-p.p147)) + (1.0)));
        }

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_ad_lhs(207, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p40, s.ad_value(334)), p.p44, s.ad_value(244)), 337);
        }

        s.b[580] = (s.v[207] > 0.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        s.b[581] = (p.p53 == 1.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if ((s.b[565] && s.b[580]) && s.b[581]) {
            s.store_ad_value(208, A::add_scaled_inputs3(A::div_from_scalar(s.v[6], A::mul(s.ad_value(156), A::add(s.ad_value(30), s.ad_value(186)))), 1.0, A::div(s.ad_value(153), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(186))), 1.0));
        }

        s.b[582] = (p.p39 == 3.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_scaled_sub(279, 207, 208, 1000000.0);
        }

        s.b[583] = (s.v[207] < s.v[208]);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && s.b[583]) {
            s.store_ad_value(207, A::sub_scaled_inputs(s.ad_value(207), 1.0, A::ln_one_plus_exp(s.ad_value(279)), 1e-6));
        }

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && (!s.b[583])) {
            s.store_ad_value(207, A::sub_scaled_inputs(s.ad_value(208), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 1e-6));
        }

        if (((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_mul(209, 156, 207);
        }

        if (((s.b[565] && s.b[580]) && s.b[581]) && (!s.b[582])) {
            s.store_ad_value(209, A::div_scaled_product3(s.ad_value(156), s.ad_value(207), s.ad_value(208), 1.0, A::add(s.ad_value(207), s.ad_value(208)), 1.0));
        }

        if ((s.b[565] && s.b[580]) && (!s.b[581])) {
            s.store_mul(209, 156, 207);
        }

        s.store_scaled_mul(215, 23, 138, (1.0 - p.p68));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(247), 1.0, s.ad_value(136), (-1.0), s.ad_value(293), 1.0));

        s.b[585] = (s.v[247] < s.v[136]);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if s.b[585] {
            s.store_add_scaled_product(216, s.ad_value(247), 1.0, s.ad_value(293), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[585]) {
            s.store_add_scaled_product(216, s.ad_value(136), 1.0, s.ad_value(293), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_scaled_ad_rhs(217, 23, p.p68, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(65))), (1.0 - p.p67)), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(247), 3.0, s.ad_value(216), (-3.0)));

        s.store_scaled_mul(218, 24, 145, p.p77);

        s.v[219] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(223, 149, 184, (0.5 * s.v[219]));

        s.store_scaled_mul(224, 150, 184, (0.5 * s.v[219]));

        s.store_scale(294, 17, 0.1);

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(249), 1.0, s.ad_value(141), (-1.0), s.ad_value(294), 1.0));

        s.b[586] = (s.v[249] < s.v[141]);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if s.b[586] {
            s.store_add_scaled_product(225, s.ad_value(249), 1.0, s.ad_value(294), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[586]) {
            s.store_add_scaled_product(225, s.ad_value(141), 1.0, s.ad_value(294), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product(226, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(225), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(140), A::sub(s.ad_value(249), s.ad_value(225)), 1.0);

        s.store_mul_scaled_ad_rhs(227, 24, ((1.0 - p.p77) * (1.0 - p.p33)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(226)), 1.0, s.ad_value(25), s.ad_value(249), 1.0));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(261), 1.0, s.ad_value(141), (-1.0), s.ad_value(294), 1.0));

        s.b[587] = (s.v[261] < s.v[141]);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if s.b[587] {
            s.store_add_scaled_product(228, s.ad_value(261), 1.0, s.ad_value(294), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[587]) {
            s.store_add_scaled_product(228, s.ad_value(141), 1.0, s.ad_value(294), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product(229, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(228), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(140), A::sub(s.ad_value(261), s.ad_value(228)), 1.0);

        s.store_mul_scaled_ad_rhs(230, 24, ((1.0 - p.p77) * p.p33), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(229)), 1.0, s.ad_value(25), s.ad_value(261), 1.0));

        s.store_scale(301, 102, 0.1);

        s.store_scale(231, 102, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(253), 1.0, s.ad_value(231), (-1.0), s.ad_value(301), 1.0));

        s.b[588] = (s.v[253] < s.v[231]);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if s.b[588] {
            s.store_add_scaled_product(232, s.ad_value(253), 1.0, s.ad_value(301), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[588]) {
            s.store_add_scaled_product(232, s.ad_value(231), 1.0, s.ad_value(301), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_ad_rhs(233, 103, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(102), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(102))), (1.0 - p.p139)), 1.0 / ((1.0 - p.p139))), 1.0, s.ad_value(253), 2.0, s.ad_value(232), (-2.0)));

        s.store_scaled_powf_ad(234, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p85), (s.v[93] * s.v[36]));

        s.b[589] = ((s.v[246] / (p.p85 * s.v[6])) < p.p147);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if s.b[589] {
            s.store_exp_scaled_input(296, 246, 1.0 / ((p.p85 * s.v[6])));
        }

        if (!s.b[589]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(246), 1.0 / ((p.p85 * s.v[6])), (((-p.p147)) + (1.0))));
        }

        s.store_mul(236, 234, 296);

        s.v[237] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_ad_rhs(238, 122, (0.5 * s.v[237]), A::offset(A::add(s.ad_value(126), s.ad_value(113)), 2.0));

        s.b[590] = (p.p79 == 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_add_scaled_inputs(243, 168, (s.v[219] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 167, (s.v[237] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[591] = ((((s.v[249] - s.v[22]) / p.p91) * s.v[8]) < p.p147);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((!s.b[590]) && s.b[591]) {
            s.store_ad_value(177, A::exp_scaled_input(A::sub(s.ad_value(249), s.ad_value(22)), (1.0 / (p.p91) * s.v[8])));
        }

        if ((!s.b[590]) && (!s.b[591])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(177, 295, A::sub_scaled_inputs(s.ad_value(249), (1.0 / (p.p91) * s.v[8]), s.ad_value(22), (1.0 / (p.p91) * s.v[8])), (((-p.p147)) + (1.0)));
        }

        if (!s.b[590]) {
            s.store_ad_value(243, A::div_scaled_value_offset_denominator(s.ad_value(268), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(177), 4.0, 1.0)), 1.0, 1.0));
        }

        s.b[592] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0));
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[592] {
            s.store_scale(243, 243, s.v[157]);
        }

        s.b[593] = (p.p79 == 0.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (s.b[592] && s.b[593]) {
            s.store_mul(169, 146, 269);
            s.store_ad_value(170, A::div_scaled_inputs2(s.ad_value(169), 1.0, s.ad_value(146), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0), 1.0));
            s.store_scale(239, 272, 4.0);
            s.store_ad_value(240, A::div_scaled_value_offset_denominator(s.ad_value(239), 1.0, A::sqrt(A::offset(s.ad_value(239), 1.0)), 1.0, 1.0));
            s.store_add_scaled_inputs(241, 170, (s.v[219] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 240, (s.v[237] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[594] = (((s.v[261] - s.v[22]) * s.v[8]) < p.p147);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((s.b[592] && (!s.b[593])) && s.b[594]) {
            s.store_ad_value(178, A::exp_scaled_input(A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]));
        }

        if ((s.b[592] && (!s.b[593])) && (!s.b[594])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(178, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(22), s.v[8]), (((-p.p147)) + (1.0)));
        }

        if (s.b[592] && (!s.b[593])) {
            s.store_ad_value(241, A::div_scaled_value_offset_denominator(s.ad_value(269), (((2.0 * p.p33) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(178), 4.0, 1.0)), 1.0, 1.0));
        }

        if s.b[592] {
            s.store_mul(242, 175, 241);
        }

        s.b[595] = (p.p6 == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_offset_powf_ad(190, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (-p.p67), (-3.0));
            s.store_ad_value(288, A::div_scaled_inputs2(s.ad_value(246), 1.0, s.ad_value(136), (-1.0), s.ad_value(293), 1.0));
        }

        s.b[596] = (s.v[288] < 0.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_div_from_scalar_offset_ad(191, 1.0, A::exp(s.ad_value(288)), 1.0);
        }

        if (s.b[595] && (!s.b[596])) {
            s.store_div_ad(191, A::exp_scaled_input(s.ad_value(288), -1.0), A::offset(A::exp_scaled_input(s.ad_value(288), -1.0), 1.0));
        }

        if s.b[595] {
            s.store_offset_mul(189, 190, 191, 3.0);
            s.store_scaled_mul(192, 23, 189, (1.0 - p.p68));
            s.store_mul_ad(195, A::div_scaled_product(s.ad_value(146), s.ad_value(266), s.v[8], s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(147), 1.0))));
            s.store_scaled_mul(193, 184, 195, (0.5 * s.v[219]));
            s.store_scale(194, 236, 1.0 / ((p.p85 * s.v[6])));
            s.store_mul_scaled_ad_rhs(222, 248, 0.2, A::add_scaled_inputs3(s.ad_value(192), 1.0, s.ad_value(193), 1.0, s.ad_value(194), 1.0));
            s.store_scale(235, 236, (1.0 - p.p95));
            s.store_add_scaled_inputs(331, 223, 1.0, 236, p.p95);
            s.store_add_scaled_inputs(221, 331, p.p94, 224, 1.0);
            s.store_scale(220, 331, (1.0 - p.p94));
        }

        if (!s.b[595]) {
            s.copy_ad(220, 223);
            s.copy_ad(221, 224);
            s.copy_ad(235, 236);
        }

        s.b[597] = (p.p24 == 1.0);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        s.b[598] = (p.p58 > 0.0);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        s.b[599] = (p.p59 > 0.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        s.v[302] = ((4.0 * 1.3806226e-23) * s.v[2]);

        s.store_div_from_scalar(303, s.v[302], 28);

        s.store_div_from_scalar(304, s.v[302], 30);

        s.store_scale(305, 108, s.v[302]);

        s.store_scale(306, 109, s.v[302]);

        s.store_scale(307, 110, s.v[302]);

        s.store_scaled_mul_ad(308, A::div_from_scalar(s.v[302], s.ad_value(186)), A::scale_offset(s.ad_value(267), 4.0, 5.0), 0.3333333333333333);

        s.store_ad_value(327, A::div_scaled_inputs2(s.ad_value(155), 1.0, s.ad_value(154), 1.0, s.ad_value(153), 1.0));

        s.store_scaled_abs(309, 327, (2.0 * 1.6021918e-19));

        s.b[600] = (p.p130 > 0.0);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if s.b[600] {
            s.store_abs_ad(328, A::div(s.ad_value(209), s.ad_value(327)));
        }

        if (!s.b[600]) {
            s.store_scalar(328, 0.0);
        }

        s.store_mul_scaled_ad_rhs(321, 209, (2.0 * 1.6021918e-19), A::offset(s.ad_value(328), 1.0));

        s.b[601] = (s.v[327] > 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_ad_value(329, A::div_scaled_inputs2(s.ad_value(220), 1.0, s.ad_value(221), 1.0, s.ad_value(327), 1.0));
        }

        if (!s.b[601]) {
            s.store_scaled_mul(329, 184, 153, s.v[94]);
        }

        s.b[602] = (p.p131 == 1.0);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if s.b[602] {
            s.store_scale(330, 329, p.p94);
        }

        s.b[603] = (p.p131 == 2.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if ((!s.b[602]) && s.b[603]) {
            s.store_scale(330, 329, p.p132);
        }

        if ((!s.b[602]) && (!s.b[603])) {
            s.store_scalar(330, 0.0);
        }

        s.store_scaled_abs_ad(310, A::add(A::add_scaled_inputs4(s.ad_value(158), 1.0, s.ad_value(160), 1.0, s.ad_value(57), -1.0, s.ad_value(352), 1.0), s.ad_value(351)), (2.0 * 1.6021918e-19));

        s.store_add(322, 158, 159);

        s.store_scaled_powf_ad(311, A::abs(s.ad_value(322)), p.p126, p.p128);

        s.b[604] = (s.v[322] < 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if s.b[604] {
            s.store_neg(311, 311);
        }

        s.store_ad_value(323, A::add_scaled_inputs3(s.ad_value(160), 1.0, s.ad_value(162), 1.0, s.ad_value(163), 1.0));

        s.store_scaled_powf_ad(312, A::abs(s.ad_value(323)), p.p127, p.p129);

        s.b[605] = (s.v[323] < 0.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if s.b[605] {
            s.store_neg(312, 312);
        }

        s.store_scaled_abs_ad(313, A::add_scaled_inputs3(s.ad_value(159), 1.0, s.ad_value(162), 1.0, s.ad_value(163), 1.0), (2.0 * 1.6021918e-19));

        s.store_scaled_abs(314, 161, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(315, A::abs(s.ad_value(161)), p.p126, p.p128);

        s.b[606] = (s.v[161] < 0.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if s.b[606] {
            s.store_neg(315, 315);
        }

        s.store_scaled_abs(316, 82, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(317, 164, (2.0 * 1.6021918e-19));

        s.store_scaled_powf_ad(319, A::scale(A::abs(s.ad_value(164)), 1.0 / ((1.0 - (p.p5 * p.p33)))), p.p126, (p.p128 * (1.0 - (p.p5 * p.p33))));

        s.b[607] = (s.v[164] < 0.0);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if s.b[607] {
            s.store_neg(319, 319);
        }

        s.store_scaled_abs(318, 176, ((2.0 * 1.6021918e-19) * p.p5));

        s.b[608] = (p.p33 == 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if s.b[608] {
            s.store_scalar(320, 0.0);
        }

        if (!s.b[608]) {
            s.store_scaled_powf_ad(320, A::scale(A::abs(s.ad_value(176)), 1.0 / (p.p33)), p.p126, ((p.p128 * p.p5) * p.p33));
        }

        s.b[609] = (s.v[176] < 0.0);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if s.b[609] {
            s.store_neg(320, 320);
        }

        s.store_scaled_abs(324, 182, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(325, 179, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(326, 180, (2.0 * 1.6021918e-19));

        s.b[610] = (p.p24 == 1.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        s.b[611] = (p.p58 > 0.0);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        s.b[612] = (p.p59 > 0.0);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        s.b[613] = (p.p59 > 0.0);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[476] = (p.p3 == 1.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if s.b[476] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[476]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[157] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.b[477] = (p.p150 == 0.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if s.b[477] {
            s.store_scalar(339, 1e-12);
        }

        if (!s.b[477]) {
            s.store_scalar(339, p.p150);
        }

        s.store_scale(340, 339, p.p1);

        s.v[52] = 0.001;

        s.v[336] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[279] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.b[479] = ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if s.b[479] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[479]) {
            s.store_scalar(74, ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) + (0.1 * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p114;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p71;

        s.v[76] = p.p72;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[279] = (((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) - 0.05) / 0.1);

        s.b[480] = ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if s.b[480] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[480]) {
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

        s.store_scaled_offset(279, 74, (((-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))) + ((-0.05))), 10.0);

        s.b[481] = ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if s.b[481] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(279)), 0.1, 0.05);
        }

        if (!s.b[481]) {
            s.store_ad_value(70, A::add_scaled_inputs(A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1));
        }

        s.store_scaled_offset(279, 88, (((-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))) + ((-0.05))), 10.0);

        s.b[482] = ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if s.b[482] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(279)), 0.1, 0.05);
        }

        if (!s.b[482]) {
            s.store_ad_value(85, A::add_scaled_inputs(A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1));
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p66 * s.v[4])) + ((1.0 - s.v[4]) * p.p105));

        s.v[279] = ((0.05 - s.v[13]) / s.v[6]);

        s.b[483] = (0.05 < s.v[13]);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if s.b[483] {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[483]) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p64 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[15]) / s.v[6]);

        s.b[484] = (0.05 < s.v[15]);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[484]) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p80 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[21]) / s.v[6]);

        s.b[485] = (0.05 < s.v[21]);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[485]) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p71 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[18]) / s.v[6]);

        s.b[486] = (0.05 < s.v[18]);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if s.b[486] {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[486]) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[274]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[20]) / s.v[6]);

        s.b[487] = (0.05 < s.v[20]);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[487]) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p27 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[279] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[488] = (0.05 < s.v[56]);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[488]) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[101] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p138 * s.v[4])) + ((1.0 - s.v[4]) * p.p140));

        s.v[279] = ((0.05 - s.v[101]) / s.v[6]);

        s.b[489] = (0.05 < s.v[101]);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_scalar(102, (s.v[101] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[489]) {
            s.store_scalar(102, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(73, A::scale(s.ad_value(65), p.p66), p.p67);

        s.store_powf_ad(90, A::scale(s.ad_value(67), s.v[75]), s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scaled_powf_ad(103, A::div_from_scalar(p.p138, s.ad_value(102)), p.p139, p.p137);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.v[28] = (p.p54 * (((s.v[274] * p.p97)) as f64).exp());

        s.b[490] = (s.v[28] < s.v[340]);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.copy_ad(28, 340);
        }

        s.v[29] = (p.p56 * (((s.v[274] * (p.p98 - p.p96))) as f64).exp());

        s.v[30] = (p.p55 * (((s.v[274] * p.p101)) as f64).exp());

        s.b[491] = (s.v[30] < s.v[340]);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if s.b[491] {
            s.copy_ad(30, 340);
        }

        s.v[32] = (p.p57 * (((s.v[274] * p.p102)) as f64).exp());

        s.v[31] = (p.p60 * (((s.v[274] * p.p99)) as f64).exp());

        s.b[492] = (p.p122 != 0.0);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if s.b[492] {
            s.store_scalar(50, (p.p10 * (1.0 + (s.v[12] * p.p122))));
            s.store_scaled_offset(279, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[493] = (s.v[50] < 1.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if (s.b[492] && s.b[493]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(279)), s.v[52], 1.0);
        }

        if (s.b[492] && (!s.b[493])) {
            s.store_ad_value(50, A::add_scaled_inputs(s.ad_value(50), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]));
        }

        if s.b[492] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[492]) {
            s.store_scalar(48, p.p10);
        }

        s.b[494] = (p.p123 != 0.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_scalar(51, (p.p11 * (1.0 + (s.v[12] * p.p123))));
            s.store_scaled_offset(279, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[495] = (s.v[51] < 1.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[495]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(279)), s.v[52], 1.0);
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_ad_value(51, A::add_scaled_inputs(s.ad_value(51), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]));
        }

        if s.b[494] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[494]) {
            s.store_scalar(49, p.p11);
        }

        s.v[335] = (p.p43 * (1.0 + (p.p124 * s.v[12])));

        s.v[281] = (s.v[336] * s.v[336]);

        s.v[282] = (s.v[335] * s.v[335]);

        s.b[496] = (s.v[335] < 0.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if s.b[496] {
            s.store_scalar(334, ((0.5 * s.v[281]) / ((((s.v[282] + s.v[281])) as f64).sqrt() - s.v[335])));
        }

        if (!s.b[496]) {
            s.store_scalar(334, (0.5 * ((((s.v[282] + s.v[281])) as f64).sqrt() + s.v[335])));
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_from_scalar((s.v[274] * (((4.0 - p.p98) - p.p96) + p.p121)), s.ad_value(48))), A::exp(A::div_from_scalar(((-p.p105) * s.v[10]), s.ad_value(48))), p.p9);

        s.v[36] = (p.p12 * (((s.v[274] * (1.0 - p.p98))) as f64).exp());

        s.v[37] = (p.p30 * (((s.v[274] * (1.0 - p.p103))) as f64).exp());

        s.v[42] = ((p.p16 * ((((s.v[274] * ((4.0 - p.p97) + p.p121)) / p.p17)) as f64).exp()) * (((((-p.p111) * s.v[10]) / p.p17)) as f64).exp());

        s.v[43] = ((p.p29 * (((s.v[274] * ((4.0 - p.p103) + p.p121))) as f64).exp()) * ((((-p.p112) * s.v[10])) as f64).exp());

        s.store_powf_ad(275, A::scale(s.ad_value(70), s.v[72]), (-0.5));

        s.store_div_from_scalar(276, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(275), p.p35), s.ad_value(276), (p.p66 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_ad(277, A::scale(s.ad_value(85), s.v[86]), (-0.5));

        s.store_div_from_scalar(278, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(277), p.p37), s.ad_value(278), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

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

        s.b[498] = (s.v[2] < 525.0);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!s.b[498]) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p92 * (((s.v[274] * p.p96)) as f64).exp());

        s.store_scaled_voltage(244, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(245, ctx, nodes, Some(6), Some(8), p.p3);

        s.store_scaled_voltage(246, ctx, nodes, Some(6), Some(4), p.p3);

        s.store_scaled_voltage(247, ctx, nodes, Some(5), Some(4), p.p3);

        s.store_scaled_voltage(248, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(253, ctx, nodes, Some(3), Some(7), p.p3);

        s.store_scaled_voltage(250, ctx, nodes, Some(7), Some(8), p.p3);

        s.store_scaled_voltage(260, ctx, nodes, Some(1), Some(5), p.p3);

        s.store_scaled_voltage(263, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(264, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(252, ctx, nodes, Some(10), Some(7), p.p3);

        s.store_scaled_voltage(251, ctx, nodes, Some(9), Some(10), p.p3);

        s.store_ad_value(249, A::add_scaled_inputs4(s.ad_value(248), 1.0, s.ad_value(245), 1.0, s.ad_value(250), -1.0, s.ad_value(252), -1.0));

        s.store_ad_value(262, A::add_scaled_inputs4(s.ad_value(260), 1.0, s.ad_value(264), (-1.0), s.ad_value(249), 1.0, s.ad_value(251), -1.0));

        s.store_add(261, 264, 262);

        s.store_sub(255, 253, 252);

        s.store_sub(254, 255, 251);

        s.b[505] = ((s.v[245] * s.v[8]) < p.p147);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if s.b[505] {
            s.store_exp_scaled_input(265, 245, s.v[8]);
        }

        if (!s.b[505]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(265, 295, A::scale_offset(s.ad_value(245), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[506] = (((s.v[246] * s.v[8]) / s.v[48]) < p.p147);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if s.b[506] {
            s.store_exp_ad(266, A::div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0));
        }

        if (!s.b[506]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(266, 295, A::div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0), (((-p.p147)) + (1.0)));
        }

        s.b[507] = ((s.v[249] * s.v[8]) < p.p147);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if s.b[507] {
            s.store_exp_scaled_input(268, 249, s.v[8]);
        }

        if (!s.b[507]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(268, 295, A::scale_offset(s.ad_value(249), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[508] = ((s.v[248] * s.v[8]) < p.p147);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if (!s.b[508]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[509] = ((s.v[261] * s.v[8]) < p.p147);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_exp_scaled_input(269, 261, s.v[8]);
        }

        if (!s.b[509]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(269, 295, A::scale_offset(s.ad_value(261), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[510] = ((s.v[253] * s.v[8]) < p.p147);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (!s.b[510]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[511] = ((s.v[254] * s.v[8]) < p.p147);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if s.b[511] {
            s.store_exp_scaled_input(257, 254, s.v[8]);
        }

        if (!s.b[511]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(257, 295, A::scale_offset(s.ad_value(254), s.v[8], (((-p.p147)) + (1.0))));
        }

        s.b[512] = ((s.v[255] * s.v[8]) < p.p147);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if (!s.b[512]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[513] = (((s.v[261] - s.v[16]) * s.v[8]) < p.p147);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_ad_value(272, A::exp_scaled_input(A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[513]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(272, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[514] = (((s.v[249] - s.v[16]) * s.v[8]) < p.p147);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if s.b[514] {
            s.store_ad_value(270, A::exp_scaled_input(A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[514]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(270, 295, A::sub_scaled_inputs(s.ad_value(249), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[515] = (((s.v[245] - s.v[16]) * s.v[8]) < p.p147);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if s.b[515] {
            s.store_ad_value(271, A::exp_scaled_input(A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[515]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(271, 295, A::sub_scaled_inputs(s.ad_value(245), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[516] = (((s.v[244] - s.v[16]) * s.v[8]) < p.p147);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_ad_value(273, A::exp_scaled_input(A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]));
        }

        if (!s.b[516]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(273, 295, A::sub_scaled_inputs(s.ad_value(244), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(111, 271, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(112, 273, 4.0, 1.0);

        s.store_ad_value(113, A::div_scaled_value_offset_denominator(s.ad_value(273), 2.0, s.ad_value(112), 1.0, 1.0));

        s.b[517] = (s.v[113] < p.p149);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if s.b[517] {
            s.store_scalar(113, p.p149);
        }

        s.store_ad_value(114, A::add_scaled_inputs3(s.ad_value(111), s.v[6], s.ad_value(112), ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(111), 1.0, 1.0, A::offset(s.ad_value(112), 1.0), 1.0)), (-s.v[6])));

        s.store_scaled_add(115, 114, 250, 1.0 / (s.v[31]));

        s.b[518] = (s.v[115] > 0.0);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        s.b[519] = (s.v[244] < 100.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[519]) {
            s.copy_ad(297, 244);
        }

        if (s.b[518] && (!s.b[519])) {
            s.store_offset_ln_ad(297, A::offset(s.ad_value(244), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[518] {
            s.store_ad_value(116, A::add_scaled_inputs3(s.ad_value(16), 1.0, A::ln(A::scale_offset(s.ad_value(115), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), s.ad_value(297), -1.0));
            s.store_scale(292, 16, 0.2);
            s.store_square(281, 292);
            s.store_square(282, 116);
        }

        s.b[520] = (s.v[116] < 0.0);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[520]) {
            s.store_ad_value(117, A::div_scaled_inputs(s.ad_value(281), 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)), 1.0));
        }

        if (s.b[518] && (!s.b[520])) {
            s.store_scaled_add_ad_lhs(117, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 116, 0.5);
        }

        if s.b[518] {
            s.store_ad_value(118, A::div_scaled_product_offset_rhs(s.ad_value(117), s.ad_value(117), (p.p62 * p.p61), 1.0, A::scaled_offset(s.ad_value(117), (p.p62 * s.v[31]), p.p61), 1.0));
            s.store_div(285, 115, 118);
            s.store_scaled_offset(279, 285, (-1.0), 1.0 / (p.p63));
        }

        s.b[521] = (s.v[285] < 1.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[521]) {
            s.store_offset_scaled_ad(283, A::ln_one_plus_exp(s.ad_value(279)), p.p63, 1.0);
        }

        if (s.b[518] && (!s.b[521])) {
            s.store_ad_value(283, A::add_scaled_inputs(s.ad_value(285), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p63));
        }

        if s.b[518] {
            s.store_scale(119, 283, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
            s.store_scale(120, 117, 1.0 / ((p.p62 * p.p61)));
            s.store_ad_value(121, A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(119), s.ad_value(120), A::offset(s.ad_value(120), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(119), 2.0, A::offset(s.ad_value(120), 1.0)), 1.0));
            s.store_div_ad(122, A::add_scaled_sub_value_product(1.0, s.ad_value(121), 1.0, s.ad_value(113), s.ad_value(121), 1.0), A::offset(A::mul(s.ad_value(113), s.ad_value(121)), 1.0));
            s.store_scaled_mul(124, 115, 122, ((0.5 * s.v[31]) * s.v[8]));
            s.store_ad_value(286, A::add_scaled_offset_product_rhs(s.ad_value(124), 2.0, s.ad_value(113), A::add(s.ad_value(113), s.ad_value(124)), 1.0, 1.0));
            s.store_scaled_offset(125, 124, (-1.0), 0.5);
            s.store_add_ad_lhs(280, A::square(s.ad_value(125)), 286);
        }

        s.b[522] = (s.v[124] >= 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[522]) {
            s.store_add_ad_rhs(126, 125, A::sqrt(s.ad_value(280)));
        }

        if (s.b[518] && (!s.b[522])) {
            s.store_div_ad_rhs(126, 286, A::sub(A::sqrt(s.ad_value(280)), s.ad_value(125)));
        }

        s.b[523] = (s.v[126] < p.p148);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[523]) {
            s.store_scalar(126, p.p148);
        }

        if s.b[518] {
            s.store_mul_ad_product_rhs(128, 126, A::offset(s.ad_value(126), 1.0), A::exp_scaled_input(s.ad_value(16), s.v[8]));
            s.store_scaled_offset(130, 115, (-p.p62), (0.5 * p.p61));
            s.store_scale(131, 115, ((p.p61 * s.v[31]) * p.p62));
            s.store_add_ad_rhs(132, 130, A::sqrt(A::add(A::square(s.ad_value(130)), s.ad_value(131))));
        }

        s.b[524] = (p.p73 == 0.0);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[524]) {
            s.store_scale(133, 17, 0.1);
        }

        if (s.b[518] && (!s.b[524])) {
            s.store_mul_offset_ad_rhs(133, 17, A::div_scaled_inputs(s.ad_value(115), 2.0, A::add(s.ad_value(115), s.ad_value(118)), 1.0), 0.1);
        }

        if s.b[518] {
            s.store_ad_value(134, A::div_scaled_value_offset_denominator(s.ad_value(115), p.p62, s.ad_value(115), p.p62, 1.0));
            s.store_div_from_scalar_offset_input(210, p.p62, 115, p.p62);
        }

        if (!s.b[518]) {
            s.store_scalar(118, 0.0);
            s.store_ad_value(126, A::div_scaled_value_offset_denominator(s.ad_value(271), 2.0, s.ad_value(111), 1.0, 1.0));
            s.copy_ad(128, 265);
        }

        s.b[525] = ((((s.v[250]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[114]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[111] + s.v[112]))));
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((!s.b[518]) && s.b[525]) {
            s.store_scaled_add(135, 126, 113, 0.5);
            s.store_ad_value(122, A::div_scaled_value_offset_denominator(s.ad_value(135), 1.0, s.ad_value(135), 1.0, 1.0));
        }

        if ((!s.b[518]) && (!s.b[525])) {
            s.store_div_ad_rhs(122, 114, A::add_scaled_inputs3(s.ad_value(114), 1.0, s.ad_value(245), 1.0, s.ad_value(244), -1.0));
        }

        if (!s.b[518]) {
            s.copy_ad(132, 250);
            s.store_scale(133, 17, 0.1);
            s.copy_ad(134, 115);
            s.store_sub_from_scalar_ad(210, 1.0, A::scale(s.ad_value(134), 1.0 / (p.p62)));
        }

        s.store_scale(136, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(293, 14, 0.1);

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(246), 1.0, s.ad_value(136), (-1.0), s.ad_value(293), 1.0));

        s.b[526] = (s.v[246] < s.v[136]);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_add_scaled_product(137, s.ad_value(246), 1.0, s.ad_value(293), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[526]) {
            s.store_add_scaled_product(137, s.ad_value(136), 1.0, s.ad_value(293), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (1.0 - p.p67));

        s.store_ad_value(138, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(246), 3.0, s.ad_value(137), (-3.0)));

        s.b[527] = (p.p74 == 1.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if s.b[527] {
            s.copy_ad(139, 244);
        }

        s.b[528] = (p.p74 == 2.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((!s.b[527]) && s.b[528]) {
            s.store_add(139, 244, 132);
        }

        if ((!s.b[527]) && (!s.b[528])) {
            s.copy_ad(139, 245);
        }

        s.store_div_ad(140, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(141, 17, 1.0, A::powf(s.ad_value(140), ((-1.0) / p.p72)));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(139), 1.0, s.ad_value(141), (-1.0), s.ad_value(133), 1.0));

        s.b[529] = (s.v[139] < s.v[141]);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_add_scaled_product(142, s.ad_value(139), 1.0, s.ad_value(133), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[529]) {
            s.store_add_scaled_product(142, s.ad_value(141), 1.0, s.ad_value(133), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf(143, 210, p.p76);

        s.store_add_ad(144, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(143), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(142), s.ad_value(17))), (1.0 - p.p72))), 1.0 / ((1.0 - p.p72))), A::mul3(s.ad_value(143), s.ad_value(140), A::sub(s.ad_value(139), s.ad_value(142))));

        s.store_add_scaled_product(145, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(144)), 1.0, s.ad_value(25), s.ad_value(244), 1.0);

        s.store_scale(146, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(147, 146, 266);

        s.store_ad_value(149, A::div_scaled_value_offset_denominator(s.ad_value(147), 1.0, A::sqrt(A::offset(s.ad_value(147), 1.0)), 1.0, 1.0));

        s.store_pow_ad(129, s.ad_value(128), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(148, 146, 129);

        s.store_ad_value(150, A::div_scaled_value_offset_denominator(s.ad_value(148), 1.0, A::sqrt(A::offset(s.ad_value(148), 1.0)), 1.0, 1.0));

        s.b[530] = (p.p92 == 0.0);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if s.b[530] {
            s.store_add_ad(151, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));
        }

        if (!s.b[530]) {
            s.store_offset_scaled_div(289, 138, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_scaled_div(290, 145, 40, (-(s.v[99] * s.v[8])));
            s.store_scaled_sub_ad(151, A::exp(s.ad_value(289)), A::exp(s.ad_value(290)), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 151);

        s.b[531] = (s.v[151] < 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if s.b[531] {
            s.store_div_from_scalar_sub_ad(152, (0.5 * s.v[281]), A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(151));
        }

        if (!s.b[531]) {
            s.store_scaled_add_ad_lhs(152, A::sqrt(A::offset(s.ad_value(282), s.v[281])), 151, 0.5);
        }

        s.store_mul_offset_ad_rhs(153, 152, A::add_scaled_inputs(s.ad_value(149), 0.5, s.ad_value(150), 0.5), 1.0);

        s.store_scaled_mul(154, 35, 129, p.p15);

        s.store_mul(155, 35, 266);

        s.store_ad_value(156, A::div_scaled_inputs2(s.ad_value(155), 1.0, s.ad_value(154), (-1.0), s.ad_value(153), 1.0));

        s.store_scale(279, 246, 10000.0);

        s.b[532] = (s.v[246] < 0.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if s.b[532] {
            s.store_scaled_ln_one_plus_exp(296, 279, 0.0001);
        }

        if (!s.b[532]) {
            s.store_ad_value(296, A::add_scaled_inputs(s.ad_value(246), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.0001));
        }

        s.store_scale(298, 296, 1.0 / (p.p152));

        s.b[533] = (s.v[298] < p.p147);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (!s.b[533]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.store_scaled_offset(279, 246, (-p.p154), 1000.0);

        s.b[535] = (((s.v[246] * s.v[8]) / p.p17) < p.p147);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if s.b[535] {
            s.store_exp_scaled_input(296, 246, (s.v[8] * 1.0 / (p.p17)));
        }

        if (!s.b[535]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(246), (s.v[8] * 1.0 / (p.p17)), (((-p.p147)) + (1.0))));
        }

        s.b[536] = (p.p24 == 1.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (((s.v[246] - s.v[55]) * s.v[8]) < p.p147);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (s.b[536] && s.b[537]) {
            s.store_ad_value(298, A::exp_scaled_input(A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]));
        }

        if (s.b[536] && (!s.b[537])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(298, 295, A::sub_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(55), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[538] = (((s.v[156] / s.v[35]) - 1000.0) < 40.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if (s.b[536] && (!s.b[538])) {
            s.store_scalar(295, ((40.0) as f64).exp());
        }

        s.b[540] = (((s.v[247] * s.v[8]) / p.p19) < p.p147);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if s.b[540] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p19)));
        }

        if (!s.b[540]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(247), (s.v[8] * 1.0 / (p.p19)), (((-p.p147)) + (1.0))));
        }

        s.b[541] = (p.p24 == 1.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[542] = (((s.v[247] - s.v[55]) * s.v[8]) < p.p147);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if (s.b[541] && s.b[542]) {
            s.store_ad_value(298, A::exp_scaled_input(A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]));
        }

        if (s.b[541] && (!s.b[542])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(298, 295, A::sub_scaled_inputs(s.ad_value(247), s.v[8], s.ad_value(55), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[543] = (((s.v[246] * s.v[8]) / p.p21) < p.p147);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_exp_scaled_input(296, 246, (s.v[8] * 1.0 / (p.p21)));
        }

        if (!s.b[543]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(246), (s.v[8] * 1.0 / (p.p21)), (((-p.p147)) + (1.0))));
        }

        s.b[544] = (((s.v[247] * s.v[8]) / p.p23) < p.p147);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p23)));
        }

        if (!s.b[544]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(247), (s.v[8] * 1.0 / (p.p23)), (((-p.p147)) + (1.0))));
        }

        s.b[545] = (((s.v[249] * s.v[8]) / p.p32) < p.p147);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_exp_scaled_input(296, 249, (s.v[8] * 1.0 / (p.p32)));
        }

        if (!s.b[545]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(249), (s.v[8] * 1.0 / (p.p32)), (((-p.p147)) + (1.0))));
        }

        s.b[546] = (((s.v[247] * s.v[8]) / p.p146) < p.p147);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if s.b[546] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p146)));
        }

        if (!s.b[546]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(247), (s.v[8] * 1.0 / (p.p146)), (((-p.p147)) + (1.0))));
        }

        s.b[547] = (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[246] < 0.0));
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        s.b[548] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p147);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[547] && (!s.b[548])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if s.b[547] {
            s.store_mul(275, 246, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt(A::offset(A::square(s.ad_value(275)), 1e-30)), ((-2.0) - p.p67)), A::sub(A::scale_offset(A::scale(s.ad_value(275), (3.0 * (p.p67 - 1.0))), (-p.p67), (((1.0 - (p.p67 * p.p67))) * (p.p67))), A::mul3_scaled_output(s.ad_value(275), s.ad_value(275), A::offset(s.ad_value(275), (p.p67 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(275, A::div_scaled_product_by_product(s.ad_value(246), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0));
        }

        s.b[549] = (s.v[275] < (-0.001));
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        s.b[550] = (s.v[275] < p.p147);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if ((s.b[547] && s.b[549]) && (!s.b[550])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[551] = (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[244] < 0.0));
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if s.b[551] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(244), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[552] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p147);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[551] && (!s.b[552])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if s.b[551] {
            s.store_mul(277, 244, 67);
        }

        if s.b[551] {
            let assign4580_ad_e4435: A = A::mul_scaled_output(A::powf(A::sqrt(A::offset(A::square(s.ad_value(277)), 1e-30)), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(277), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(277), s.ad_value(277), A::offset(s.ad_value(277), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_ad_value(80, assign4580_ad_e4435);
        }

        if s.b[551] {
            s.store_ad_value(277, A::div_scaled_product_by_product(s.ad_value(244), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0));
        }

        s.b[553] = (s.v[277] < (-0.001));
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        s.b[554] = (s.v[277] < p.p147);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if ((s.b[551] && s.b[553]) && (!s.b[554])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.store_mul(165, 146, 268);

        s.store_scale(166, 270, 4.0);

        s.store_ad_value(168, A::div_scaled_inputs2(s.ad_value(165), 1.0, s.ad_value(146), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0), 1.0));

        s.store_ad_value(167, A::div_scaled_value_offset_denominator(s.ad_value(166), 1.0, A::sqrt(A::offset(s.ad_value(166), 1.0)), 1.0, 1.0));

        s.b[556] = ((p.p5 > 0.0) && (p.p33 > 0.0));
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_ad_value(171, A::div_scaled_offset_numerator(s.ad_value(269), ((p.p33 * 2.0) * s.v[43]), ((-1.0) * ((p.p33 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0));
        }

        s.b[557] = (p.p8 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[557]) {
            s.store_ad_value(172, A::div_scaled_inputs2(s.ad_value(269), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), s.ad_value(257), (-((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::add_scaled_inputs(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), s.ad_value(257), (p.p144 * ((4.0 * s.v[104]) / s.v[106]))), 1.0)), 1.0), 1.0));
        }

        if (s.b[556] && (!s.b[557])) {
            s.store_ad_value(172, A::div_scaled_offset_numerator(s.ad_value(269), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), ((-1.0) * ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), 1.0)), 1.0), 1.0));
        }

        s.b[558] = (p.p5 == 1.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[558]) {
            s.store_scalar(291, ((p.p33 * (s.v[43] + s.v[104])) * s.v[32]));
            s.store_offset_scaled_ad(173, A::ln_scaled_input(s.ad_value(291), s.v[8]), (-s.v[6]), ((2.0) * (s.v[6])));
            s.store_sub(284, 261, 173);
            s.store_scalar(281, (0.11 * 0.11));
            s.store_square(282, 284);
        }

        s.b[559] = (s.v[284] < 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if ((s.b[556] && s.b[558]) && s.b[559]) {
            s.store_ad_value(174, A::div_scaled_inputs(s.ad_value(281), 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)), 1.0));
        }

        if ((s.b[556] && s.b[558]) && (!s.b[559])) {
            s.store_scaled_add_ad_lhs(174, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 284, 0.5);
        }

        if (s.b[556] && s.b[558]) {
            s.store_div_ad_rhs(175, 174, A::add_scaled_inputs4(s.ad_value(291), 1.0, s.ad_value(171), s.v[32], s.ad_value(172), s.v[32], s.ad_value(174), 1.0));
        }

        if (s.b[556] && (!s.b[558])) {
            s.store_scalar(173, 0.0);
            s.store_scalar(284, 0.0);
            s.store_scalar(174, 0.0);
            s.store_scalar(175, 1.0);
        }

        s.b[560] = (p.p84 == 1.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if s.b[560] {
            s.store_add(347, 248, 244);
            s.store_scalar(281, (1e-6 * 1e-6));
            s.store_scaled_mul(282, 347, 347, ((-1.0) * (-1.0)));
        }

        s.store_add_ad(183, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 183);

        s.b[563] = (s.v[183] < 0.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if s.b[563] {
            s.store_div_from_scalar_sub_ad(184, (0.5 * s.v[281]), A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(183));
        }

        if (!s.b[563]) {
            s.store_scaled_add_ad_lhs(184, A::sqrt(A::offset(s.ad_value(282), s.v[281])), 183, 0.5);
        }

        s.store_mul_offset_ad_rhs(185, 184, A::add_scaled_inputs(s.ad_value(149), 0.5, s.ad_value(150), 0.5), 1.0);

        s.store_div_from_scalar(187, s.v[29], 185);

        s.b[564] = (s.v[187] < s.v[340]);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if s.b[564] {
            s.copy_ad(187, 340);
        }

        s.store_scale(186, 187, 3.0);

        s.b[565] = (s.v[156] > 0.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        s.b[566] = (p.p39 == 1.0);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        s.b[567] = (s.v[244] < p.p44);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        s.b[568] = (((-s.v[156]) / p.p42) < p.p147);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[568]) {
            s.store_exp_scaled_input(332, 156, (-1.0 / (p.p42)));
        }

        if (((s.b[565] && s.b[566]) && s.b[567]) && (!s.b[568])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(332, 295, A::scale_offset(s.ad_value(156), (-1.0 / (p.p42)), (((-p.p147)) + (1.0))));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_sub_from_scalar_lhs(333, p.p44, 244, 332);
        }

        s.b[569] = (((-s.v[334]) * ((s.v[333]) as f64).powf(p.p41)) < p.p147);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[569]) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(334), -1.0, A::powf(s.ad_value(333), p.p41)));
        }

        if (((s.b[565] && s.b[566]) && s.b[567]) && (!s.b[569])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(337, 295, A::mul_scaled_lhs(s.ad_value(334), -1.0, A::powf(s.ad_value(333), p.p41)), (((-p.p147)) + (1.0)));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_ad_product_lhs(207, A::div_from_scalar(p.p40, s.ad_value(334)), s.ad_value(333), 337);
        }

        s.b[570] = (p.p39 == 2.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        s.b[571] = (s.v[244] < s.v[16]);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_scalar(196, ((2.0 * p.p46) / (p.p45 * p.p45)));
            s.store_ad_value(280, A::div_scaled_inputs2(s.ad_value(16), 1.0, s.ad_value(244), (-1.0), s.ad_value(210), 1.0));
            s.store_sqrt_ad(197, A::div_scaled_inputs(s.ad_value(280), 2.0, s.ad_value(196), 1.0));
        }

        s.b[572] = (p.p7 == 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[572]) {
            s.store_scalar(198, p.p45);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[572])) {
            s.store_sub_from_scalar_ad(123, 1.0, A::scale(s.ad_value(122), 0.5));
            s.store_scaled_mul(198, 123, 123, p.p45);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_product(199, s.ad_value(197), s.ad_value(198), 1.0, A::sqrt(A::add(A::square(s.ad_value(197)), A::square(s.ad_value(198)))), 1.0);
            s.store_ad_value(200, A::div_scaled_inputs2(s.ad_value(16), 1.0, s.ad_value(244), (-1.0), s.ad_value(199), 1.0));
            s.store_add_ad_rhs(201, 200, A::mul3_scaled_output(s.ad_value(199), s.ad_value(196), s.ad_value(210), 0.5));
        }

        s.b[573] = (p.p7 == 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[573]) {
            s.copy_ad(202, 201);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[573])) {
            s.store_offset_scaled(203, 122, ((2.0) * ((2.0 * p.p47))), (((2.0 * p.p47)) + (1.0)));
            s.store_scalar(204, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
            s.store_sub_ad_rhs(205, 200, A::mul3_scaled_output(s.ad_value(199), s.ad_value(196), A::sub(s.ad_value(204), A::div_scaled_inputs(s.ad_value(156), 1.0, s.ad_value(203), p.p62)), 0.5));
            s.store_add_scaled_product(280, A::mul3_scaled_output(s.ad_value(200), s.ad_value(200), s.ad_value(134), (0.1 * 1.0 / (p.p62))), 1.0, A::sub(s.ad_value(205), s.ad_value(201)), A::sub(s.ad_value(205), s.ad_value(201)), 1.0);
            s.store_ad_value(202, A::add_scaled_inputs3(s.ad_value(205), 0.5, s.ad_value(201), 0.5, A::sqrt(s.ad_value(280)), 0.5));
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_ad_value(287, A::div_scaled_inputs2(s.ad_value(202), 1.0, s.ad_value(200), (-1.0), s.ad_value(202), 1.0));
        }

        s.b[574] = (((s.v[287]) as f64).abs() > 1e-7);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[574]) {
            s.store_scaled_div(206, 199, 287, 0.5);
            s.store_mul_ad(207, A::mul3(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(202), s.ad_value(206)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::div(s.ad_value(198), s.ad_value(206)), 1.0))));
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[574])) {
            s.store_mul_ad_product_rhs(207, 0, s.ad_value(198), A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0)));
        }

        s.b[575] = (p.p39 == 3.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (s.v[244] < p.p44);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_ad(211, A::powf(A::sub_from_scalar(p.p44, s.ad_value(244)), p.p41), A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(156), 1.0, s.ad_value(156), p.p48, 1.0)), p.p49));
        }

        s.b[577] = (p.p7 == 0.0);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[577]) {
            s.copy_ad(212, 211);
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_scaled_offset(213, 156, (-p.p52), 1.0 / (p.p48));
            s.store_scaled_offset(279, 213, (-1.0), 1.0 / (p.p51));
        }

        s.b[578] = (s.v[213] < 1.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if ((((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) && s.b[578]) {
            s.store_offset_scaled_ad(214, A::ln_one_plus_exp(s.ad_value(279)), p.p51, 1.0);
        }

        if ((((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) && (!s.b[578])) {
            s.store_ad_value(214, A::add_scaled_inputs(s.ad_value(213), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p51));
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_mul_powf_ad_rhs(212, 211, s.ad_value(214), p.p50);
        }

        s.b[579] = (((-s.v[334]) * s.v[212]) < p.p147);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[579]) {
            s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(334), -1.0, s.ad_value(212)));
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[579])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(337, 295, A::mul_scaled_lhs(s.ad_value(334), -1.0, s.ad_value(212)), (((-p.p147)) + (1.0)));
        }

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_ad_lhs(207, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p40, s.ad_value(334)), p.p44, s.ad_value(244)), 337);
        }

        s.b[580] = (s.v[207] > 0.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        s.b[581] = (p.p53 == 1.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if ((s.b[565] && s.b[580]) && s.b[581]) {
            s.store_ad_value(208, A::add_scaled_inputs3(A::div_from_scalar(s.v[6], A::mul(s.ad_value(156), A::add(s.ad_value(30), s.ad_value(186)))), 1.0, A::div(s.ad_value(153), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(186))), 1.0));
        }

        s.b[582] = (p.p39 == 3.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_scaled_sub(279, 207, 208, 1000000.0);
        }

        s.b[583] = (s.v[207] < s.v[208]);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && s.b[583]) {
            s.store_ad_value(207, A::sub_scaled_inputs(s.ad_value(207), 1.0, A::ln_one_plus_exp(s.ad_value(279)), 1e-6));
        }

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && (!s.b[583])) {
            s.store_ad_value(207, A::sub_scaled_inputs(s.ad_value(208), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 1e-6));
        }

        s.store_scaled_mul(215, 23, 138, (1.0 - p.p68));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(247), 1.0, s.ad_value(136), (-1.0), s.ad_value(293), 1.0));

        s.b[585] = (s.v[247] < s.v[136]);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if s.b[585] {
            s.store_add_scaled_product(216, s.ad_value(247), 1.0, s.ad_value(293), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[585]) {
            s.store_add_scaled_product(216, s.ad_value(136), 1.0, s.ad_value(293), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_scaled_ad_rhs(217, 23, p.p68, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(65))), (1.0 - p.p67)), 1.0 / ((1.0 - p.p67))), 1.0, s.ad_value(247), 3.0, s.ad_value(216), (-3.0)));

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scaled_mul(218, 24, 145, p.p77);

        s.v[219] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(223, 149, 184, (0.5 * s.v[219]));

        s.store_scaled_mul(224, 150, 184, (0.5 * s.v[219]));

        s.store_scale(294, 17, 0.1);

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(249), 1.0, s.ad_value(141), (-1.0), s.ad_value(294), 1.0));

        s.b[586] = (s.v[249] < s.v[141]);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if s.b[586] {
            s.store_add_scaled_product(225, s.ad_value(249), 1.0, s.ad_value(294), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[586]) {
            s.store_add_scaled_product(225, s.ad_value(141), 1.0, s.ad_value(294), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product(226, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(225), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(140), A::sub(s.ad_value(249), s.ad_value(225)), 1.0);

        s.store_mul_scaled_ad_rhs(227, 24, ((1.0 - p.p77) * (1.0 - p.p33)), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(226)), 1.0, s.ad_value(25), s.ad_value(249), 1.0));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(261), 1.0, s.ad_value(141), (-1.0), s.ad_value(294), 1.0));

        s.b[587] = (s.v[261] < s.v[141]);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if s.b[587] {
            s.store_add_scaled_product(228, s.ad_value(261), 1.0, s.ad_value(294), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[587]) {
            s.store_add_scaled_product(228, s.ad_value(141), 1.0, s.ad_value(294), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product(229, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(228), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, s.ad_value(140), A::sub(s.ad_value(261), s.ad_value(228)), 1.0);

        s.store_mul_scaled_ad_rhs(230, 24, ((1.0 - p.p77) * p.p33), A::add_scaled_product(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(229)), 1.0, s.ad_value(25), s.ad_value(261), 1.0));

        s.store_scale(301, 102, 0.1);

        s.store_scale(231, 102, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_ad_value(279, A::div_scaled_inputs2(s.ad_value(253), 1.0, s.ad_value(231), (-1.0), s.ad_value(301), 1.0));

        s.b[588] = (s.v[253] < s.v[231]);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if s.b[588] {
            s.store_add_scaled_product(232, s.ad_value(253), 1.0, s.ad_value(301), A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[588]) {
            s.store_add_scaled_product(232, s.ad_value(231), 1.0, s.ad_value(301), A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_ad_rhs(233, 103, A::add_scaled_inputs3(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(102), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(102))), (1.0 - p.p139)), 1.0 / ((1.0 - p.p139))), 1.0, s.ad_value(253), 2.0, s.ad_value(232), (-2.0)));

        s.store_scaled_powf_ad(234, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p85), (s.v[93] * s.v[36]));

        s.b[589] = ((s.v[246] / (p.p85 * s.v[6])) < p.p147);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if s.b[589] {
            s.store_exp_scaled_input(296, 246, 1.0 / ((p.p85 * s.v[6])));
        }

        if (!s.b[589]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_ad_rhs(296, 295, A::scale_offset(s.ad_value(246), 1.0 / ((p.p85 * s.v[6])), (((-p.p147)) + (1.0))));
        }

        s.store_mul(236, 234, 296);

        s.v[237] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_ad_rhs(238, 122, (0.5 * s.v[237]), A::offset(A::add(s.ad_value(126), s.ad_value(113)), 2.0));

        s.b[590] = (p.p79 == 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_add_scaled_inputs(243, 168, (s.v[219] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 167, (s.v[237] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[591] = ((((s.v[249] - s.v[22]) / p.p91) * s.v[8]) < p.p147);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((!s.b[590]) && s.b[591]) {
            s.store_ad_value(177, A::exp_scaled_input(A::sub(s.ad_value(249), s.ad_value(22)), (1.0 / (p.p91) * s.v[8])));
        }

        if ((!s.b[590]) && (!s.b[591])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(177, 295, A::sub_scaled_inputs(s.ad_value(249), (1.0 / (p.p91) * s.v[8]), s.ad_value(22), (1.0 / (p.p91) * s.v[8])), (((-p.p147)) + (1.0)));
        }

        if (!s.b[590]) {
            s.store_ad_value(243, A::div_scaled_value_offset_denominator(s.ad_value(268), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(177), 4.0, 1.0)), 1.0, 1.0));
        }

        s.b[592] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0));
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if s.b[592] {
            s.store_scale(243, 243, s.v[157]);
        }

        s.b[593] = (p.p79 == 0.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (s.b[592] && s.b[593]) {
            s.store_mul(169, 146, 269);
            s.store_ad_value(170, A::div_scaled_inputs2(s.ad_value(169), 1.0, s.ad_value(146), (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0), 1.0));
            s.store_scale(239, 272, 4.0);
            s.store_ad_value(240, A::div_scaled_value_offset_denominator(s.ad_value(239), 1.0, A::sqrt(A::offset(s.ad_value(239), 1.0)), 1.0, 1.0));
            s.store_add_scaled_inputs(241, 170, (s.v[219] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 240, (s.v[237] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[594] = (((s.v[261] - s.v[22]) * s.v[8]) < p.p147);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((s.b[592] && (!s.b[593])) && s.b[594]) {
            s.store_ad_value(178, A::exp_scaled_input(A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]));
        }

        if ((s.b[592] && (!s.b[593])) && (!s.b[594])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(178, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(22), s.v[8]), (((-p.p147)) + (1.0)));
        }

        if (s.b[592] && (!s.b[593])) {
            s.store_ad_value(241, A::div_scaled_value_offset_denominator(s.ad_value(269), (((2.0 * p.p33) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(178), 4.0, 1.0)), 1.0, 1.0));
        }

        if s.b[592] {
            s.store_mul(242, 175, 241);
        }

        s.b[595] = (p.p6 == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_offset_powf_ad(190, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (-p.p67), (-3.0));
            s.store_ad_value(288, A::div_scaled_inputs2(s.ad_value(246), 1.0, s.ad_value(136), (-1.0), s.ad_value(293), 1.0));
        }

        s.b[596] = (s.v[288] < 0.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_div_from_scalar_offset_ad(191, 1.0, A::exp(s.ad_value(288)), 1.0);
        }

        if (s.b[595] && (!s.b[596])) {
            s.store_div_ad(191, A::exp_scaled_input(s.ad_value(288), -1.0), A::offset(A::exp_scaled_input(s.ad_value(288), -1.0), 1.0));
        }

        if s.b[595] {
            s.store_offset_mul(189, 190, 191, 3.0);
            s.store_scaled_mul(192, 23, 189, (1.0 - p.p68));
            s.store_mul_ad(195, A::div_scaled_product(s.ad_value(146), s.ad_value(266), s.v[8], s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(147), 1.0))));
            s.store_scaled_mul(193, 184, 195, (0.5 * s.v[219]));
            s.store_scale(194, 236, 1.0 / ((p.p85 * s.v[6])));
            s.store_mul_scaled_ad_rhs(222, 248, 0.2, A::add_scaled_inputs3(s.ad_value(192), 1.0, s.ad_value(193), 1.0, s.ad_value(194), 1.0));
            s.store_scale(235, 236, (1.0 - p.p95));
            s.store_add_scaled_inputs(331, 223, 1.0, 236, p.p95);
            s.store_add_scaled_inputs(221, 331, p.p94, 224, 1.0);
            s.store_scale(220, 331, (1.0 - p.p94));
        }

        if (!s.b[595]) {
            s.copy_ad(220, 223);
            s.copy_ad(221, 224);
            s.copy_ad(235, 236);
        }

        s.store_ad_value(327, A::div_scaled_inputs2(s.ad_value(155), 1.0, s.ad_value(154), 1.0, s.ad_value(153), 1.0));

        s.b[601] = (s.v[327] > 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_ad_value(329, A::div_scaled_inputs2(s.ad_value(220), 1.0, s.ad_value(221), 1.0, s.ad_value(327), 1.0));
        }

        if (!s.b[601]) {
            s.store_scaled_mul(329, 184, 153, s.v[94]);
        }

        s.b[602] = (p.p131 == 1.0);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if s.b[602] {
            s.store_scale(330, 329, p.p94);
        }

        s.b[603] = (p.p131 == 2.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if ((!s.b[602]) && s.b[603]) {
            s.store_scale(330, 329, p.p132);
        }

        if ((!s.b[602]) && (!s.b[603])) {
            s.store_scalar(330, 0.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
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
        let eq0_e163_d_b0: f64 = (p.p3 * s.db[115][0]);
        let eq0_e163_d_b1: f64 = (p.p3 * s.db[115][1]);
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
        let eq0_e165_d_b0: f64 = (eq0_e163_d_b0 * p.p1);
        let eq0_e165_d_b1: f64 = (eq0_e163_d_b1 * p.p1);
        let eq0_value: f64 = eq0_e165;
        let eq0_node_derivatives: [f64; 12] = [eq0_e165_d_n0, eq0_e165_d_n1, eq0_e165_d_n2, eq0_e165_d_n3, eq0_e165_d_n4, eq0_e165_d_n5, eq0_e165_d_n6, eq0_e165_d_n7, eq0_e165_d_n8, eq0_e165_d_n9, eq0_e165_d_n10, eq0_e165_d_n11];
        let eq0_branch_derivatives: [f64; 2] = [eq0_e165_d_b0, eq0_e165_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
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
        let eq1_e168_d_b0: f64 = (p.p3 * s.db[156][0]);
        let eq1_e168_d_b1: f64 = (p.p3 * s.db[156][1]);
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
        let eq1_e170_d_b0: f64 = (eq1_e168_d_b0 * p.p1);
        let eq1_e170_d_b1: f64 = (eq1_e168_d_b1 * p.p1);
        let eq1_value: f64 = eq1_e170;
        let eq1_node_derivatives: [f64; 12] = [eq1_e170_d_n0, eq1_e170_d_n1, eq1_e170_d_n2, eq1_e170_d_n3, eq1_e170_d_n4, eq1_e170_d_n5, eq1_e170_d_n6, eq1_e170_d_n7, eq1_e170_d_n8, eq1_e170_d_n9, eq1_e170_d_n10, eq1_e170_d_n11];
        let eq1_branch_derivatives: [f64; 2] = [eq1_e170_d_b0, eq1_e170_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(4),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
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
        let eq2_e174_d_b0: f64 = (s.db[159][0] + s.db[162][0]);
        let eq2_e174_d_b1: f64 = (s.db[159][1] + s.db[162][1]);
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
        let eq2_e176_d_b0: f64 = (eq2_e174_d_b0 + s.db[163][0]);
        let eq2_e176_d_b1: f64 = (eq2_e174_d_b1 + s.db[163][1]);
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
        let eq2_e177_d_b0: f64 = (p.p3 * eq2_e176_d_b0);
        let eq2_e177_d_b1: f64 = (p.p3 * eq2_e176_d_b1);
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
        let eq2_e179_d_b0: f64 = (eq2_e177_d_b0 * p.p1);
        let eq2_e179_d_b1: f64 = (eq2_e177_d_b1 * p.p1);
        let eq2_value: f64 = eq2_e179;
        let eq2_node_derivatives: [f64; 12] = [eq2_e179_d_n0, eq2_e179_d_n1, eq2_e179_d_n2, eq2_e179_d_n3, eq2_e179_d_n4, eq2_e179_d_n5, eq2_e179_d_n6, eq2_e179_d_n7, eq2_e179_d_n8, eq2_e179_d_n9, eq2_e179_d_n10, eq2_e179_d_n11];
        let eq2_branch_derivatives: [f64; 2] = [eq2_e179_d_b0, eq2_e179_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
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
        let eq3_e183_d_b0: f64 = (s.db[158][0] + s.db[160][0]);
        let eq3_e183_d_b1: f64 = (s.db[158][1] + s.db[160][1]);
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
        let eq3_e186_d_b0: f64 = (s.v[338] * s.db[246][0]);
        let eq3_e186_d_b1: f64 = (s.v[338] * s.db[246][1]);
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
        let eq3_e187_d_b0: f64 = (eq3_e183_d_b0 + eq3_e186_d_b0);
        let eq3_e187_d_b1: f64 = (eq3_e183_d_b1 + eq3_e186_d_b1);
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
        let eq3_e189_d_b0: f64 = (eq3_e187_d_b0 - s.db[57][0]);
        let eq3_e189_d_b1: f64 = (eq3_e187_d_b1 - s.db[57][1]);
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
        let eq3_e191_d_b0: f64 = (eq3_e189_d_b0 + s.db[352][0]);
        let eq3_e191_d_b1: f64 = (eq3_e189_d_b1 + s.db[352][1]);
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
        let eq3_e193_d_b0: f64 = (eq3_e191_d_b0 + s.db[351][0]);
        let eq3_e193_d_b1: f64 = (eq3_e191_d_b1 + s.db[351][1]);
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
        let eq3_e194_d_b0: f64 = (p.p3 * eq3_e193_d_b0);
        let eq3_e194_d_b1: f64 = (p.p3 * eq3_e193_d_b1);
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
        let eq3_e196_d_b0: f64 = (eq3_e194_d_b0 * p.p1);
        let eq3_e196_d_b1: f64 = (eq3_e194_d_b1 * p.p1);
        let eq3_value: f64 = eq3_e196;
        let eq3_node_derivatives: [f64; 12] = [eq3_e196_d_n0, eq3_e196_d_n1, eq3_e196_d_n2, eq3_e196_d_n3, eq3_e196_d_n4, eq3_e196_d_n5, eq3_e196_d_n6, eq3_e196_d_n7, eq3_e196_d_n8, eq3_e196_d_n9, eq3_e196_d_n10, eq3_e196_d_n11];
        let eq3_branch_derivatives: [f64; 2] = [eq3_e196_d_b0, eq3_e196_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e205, eq4_e205_d_n0, eq4_e205_d_n1, eq4_e205_d_n2, eq4_e205_d_n3, eq4_e205_d_n4, eq4_e205_d_n5, eq4_e205_d_n6, eq4_e205_d_n7, eq4_e205_d_n8, eq4_e205_d_n9, eq4_e205_d_n10, eq4_e205_d_n11, eq4_e205_d_b0, eq4_e205_d_b1,) = {
    if s.b[597] {
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
        let eq4_e200_d_b0: f64 = (-s.db[82][0]);
        let eq4_e200_d_b1: f64 = (-s.db[82][1]);
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
        let eq4_e201_d_b0: f64 = (p.p3 * eq4_e200_d_b0);
        let eq4_e201_d_b1: f64 = (p.p3 * eq4_e200_d_b1);
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
        let eq4_e203_d_b0: f64 = (eq4_e201_d_b0 * p.p1);
        let eq4_e203_d_b1: f64 = (eq4_e201_d_b1 * p.p1);
        (eq4_e203, eq4_e203_d_n0, eq4_e203_d_n1, eq4_e203_d_n2, eq4_e203_d_n3, eq4_e203_d_n4, eq4_e203_d_n5, eq4_e203_d_n6, eq4_e203_d_n7, eq4_e203_d_n8, eq4_e203_d_n9, eq4_e203_d_n10, eq4_e203_d_n11, eq4_e203_d_b0, eq4_e203_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e205;
        let eq4_node_derivatives: [f64; 12] = [eq4_e205_d_n0, eq4_e205_d_n1, eq4_e205_d_n2, eq4_e205_d_n3, eq4_e205_d_n4, eq4_e205_d_n5, eq4_e205_d_n6, eq4_e205_d_n7, eq4_e205_d_n8, eq4_e205_d_n9, eq4_e205_d_n10, eq4_e205_d_n11];
        let eq4_branch_derivatives: [f64; 2] = [eq4_e205_d_b0, eq4_e205_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e215, eq5_e215_d_n0, eq5_e215_d_n1, eq5_e215_d_n2, eq5_e215_d_n3, eq5_e215_d_n4, eq5_e215_d_n5, eq5_e215_d_n6, eq5_e215_d_n7, eq5_e215_d_n8, eq5_e215_d_n9, eq5_e215_d_n10, eq5_e215_d_n11, eq5_e215_d_b0, eq5_e215_d_b1,) = {
    if (!s.b[597]) {
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
        let eq5_e210_d_b0: f64 = (-s.db[82][0]);
        let eq5_e210_d_b1: f64 = (-s.db[82][1]);
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
        let eq5_e211_d_b0: f64 = (p.p3 * eq5_e210_d_b0);
        let eq5_e211_d_b1: f64 = (p.p3 * eq5_e210_d_b1);
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
        let eq5_e213_d_b0: f64 = (eq5_e211_d_b0 * p.p1);
        let eq5_e213_d_b1: f64 = (eq5_e211_d_b1 * p.p1);
        (eq5_e213, eq5_e213_d_n0, eq5_e213_d_n1, eq5_e213_d_n2, eq5_e213_d_n3, eq5_e213_d_n4, eq5_e213_d_n5, eq5_e213_d_n6, eq5_e213_d_n7, eq5_e213_d_n8, eq5_e213_d_n9, eq5_e213_d_n10, eq5_e213_d_n11, eq5_e213_d_b0, eq5_e213_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e215;
        let eq5_node_derivatives: [f64; 12] = [eq5_e215_d_n0, eq5_e215_d_n1, eq5_e215_d_n2, eq5_e215_d_n3, eq5_e215_d_n4, eq5_e215_d_n5, eq5_e215_d_n6, eq5_e215_d_n7, eq5_e215_d_n8, eq5_e215_d_n9, eq5_e215_d_n10, eq5_e215_d_n11];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e215_d_b0, eq5_e215_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
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
        let eq6_e218_d_b0: f64 = (p.p3 * s.db[179][0]);
        let eq6_e218_d_b1: f64 = (p.p3 * s.db[179][1]);
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
        let eq6_e220_d_b0: f64 = (eq6_e218_d_b0 * p.p1);
        let eq6_e220_d_b1: f64 = (eq6_e218_d_b1 * p.p1);
        let eq6_value: f64 = eq6_e220;
        let eq6_node_derivatives: [f64; 12] = [eq6_e220_d_n0, eq6_e220_d_n1, eq6_e220_d_n2, eq6_e220_d_n3, eq6_e220_d_n4, eq6_e220_d_n5, eq6_e220_d_n6, eq6_e220_d_n7, eq6_e220_d_n8, eq6_e220_d_n9, eq6_e220_d_n10, eq6_e220_d_n11];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e220_d_b0, eq6_e220_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e223: f64 = (p.p3 * s.v[182]);
        let eq7_e223_d_n0: f64 = (p.p3 * s.dn[182][0]);
        let eq7_e223_d_n1: f64 = (p.p3 * s.dn[182][1]);
        let eq7_e223_d_n2: f64 = (p.p3 * s.dn[182][2]);
        let eq7_e223_d_n3: f64 = (p.p3 * s.dn[182][3]);
        let eq7_e223_d_n4: f64 = (p.p3 * s.dn[182][4]);
        let eq7_e223_d_n5: f64 = (p.p3 * s.dn[182][5]);
        let eq7_e223_d_n6: f64 = (p.p3 * s.dn[182][6]);
        let eq7_e223_d_n7: f64 = (p.p3 * s.dn[182][7]);
        let eq7_e223_d_n8: f64 = (p.p3 * s.dn[182][8]);
        let eq7_e223_d_n9: f64 = (p.p3 * s.dn[182][9]);
        let eq7_e223_d_n10: f64 = (p.p3 * s.dn[182][10]);
        let eq7_e223_d_n11: f64 = (p.p3 * s.dn[182][11]);
        let eq7_e223_d_b0: f64 = (p.p3 * s.db[182][0]);
        let eq7_e223_d_b1: f64 = (p.p3 * s.db[182][1]);
        let eq7_e225: f64 = (eq7_e223 * p.p1);
        let eq7_e225_d_n0: f64 = (eq7_e223_d_n0 * p.p1);
        let eq7_e225_d_n1: f64 = (eq7_e223_d_n1 * p.p1);
        let eq7_e225_d_n2: f64 = (eq7_e223_d_n2 * p.p1);
        let eq7_e225_d_n3: f64 = (eq7_e223_d_n3 * p.p1);
        let eq7_e225_d_n4: f64 = (eq7_e223_d_n4 * p.p1);
        let eq7_e225_d_n5: f64 = (eq7_e223_d_n5 * p.p1);
        let eq7_e225_d_n6: f64 = (eq7_e223_d_n6 * p.p1);
        let eq7_e225_d_n7: f64 = (eq7_e223_d_n7 * p.p1);
        let eq7_e225_d_n8: f64 = (eq7_e223_d_n8 * p.p1);
        let eq7_e225_d_n9: f64 = (eq7_e223_d_n9 * p.p1);
        let eq7_e225_d_n10: f64 = (eq7_e223_d_n10 * p.p1);
        let eq7_e225_d_n11: f64 = (eq7_e223_d_n11 * p.p1);
        let eq7_e225_d_b0: f64 = (eq7_e223_d_b0 * p.p1);
        let eq7_e225_d_b1: f64 = (eq7_e223_d_b1 * p.p1);
        let eq7_value: f64 = eq7_e225;
        let eq7_node_derivatives: [f64; 12] = [eq7_e225_d_n0, eq7_e225_d_n1, eq7_e225_d_n2, eq7_e225_d_n3, eq7_e225_d_n4, eq7_e225_d_n5, eq7_e225_d_n6, eq7_e225_d_n7, eq7_e225_d_n8, eq7_e225_d_n9, eq7_e225_d_n10, eq7_e225_d_n11];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e225_d_b0, eq7_e225_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(3),
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
        let eq8_e228: f64 = (p.p3 * s.v[180]);
        let eq8_e228_d_n0: f64 = (p.p3 * s.dn[180][0]);
        let eq8_e228_d_n1: f64 = (p.p3 * s.dn[180][1]);
        let eq8_e228_d_n2: f64 = (p.p3 * s.dn[180][2]);
        let eq8_e228_d_n3: f64 = (p.p3 * s.dn[180][3]);
        let eq8_e228_d_n4: f64 = (p.p3 * s.dn[180][4]);
        let eq8_e228_d_n5: f64 = (p.p3 * s.dn[180][5]);
        let eq8_e228_d_n6: f64 = (p.p3 * s.dn[180][6]);
        let eq8_e228_d_n7: f64 = (p.p3 * s.dn[180][7]);
        let eq8_e228_d_n8: f64 = (p.p3 * s.dn[180][8]);
        let eq8_e228_d_n9: f64 = (p.p3 * s.dn[180][9]);
        let eq8_e228_d_n10: f64 = (p.p3 * s.dn[180][10]);
        let eq8_e228_d_n11: f64 = (p.p3 * s.dn[180][11]);
        let eq8_e228_d_b0: f64 = (p.p3 * s.db[180][0]);
        let eq8_e228_d_b1: f64 = (p.p3 * s.db[180][1]);
        let eq8_e230: f64 = (eq8_e228 * p.p1);
        let eq8_e230_d_n0: f64 = (eq8_e228_d_n0 * p.p1);
        let eq8_e230_d_n1: f64 = (eq8_e228_d_n1 * p.p1);
        let eq8_e230_d_n2: f64 = (eq8_e228_d_n2 * p.p1);
        let eq8_e230_d_n3: f64 = (eq8_e228_d_n3 * p.p1);
        let eq8_e230_d_n4: f64 = (eq8_e228_d_n4 * p.p1);
        let eq8_e230_d_n5: f64 = (eq8_e228_d_n5 * p.p1);
        let eq8_e230_d_n6: f64 = (eq8_e228_d_n6 * p.p1);
        let eq8_e230_d_n7: f64 = (eq8_e228_d_n7 * p.p1);
        let eq8_e230_d_n8: f64 = (eq8_e228_d_n8 * p.p1);
        let eq8_e230_d_n9: f64 = (eq8_e228_d_n9 * p.p1);
        let eq8_e230_d_n10: f64 = (eq8_e228_d_n10 * p.p1);
        let eq8_e230_d_n11: f64 = (eq8_e228_d_n11 * p.p1);
        let eq8_e230_d_b0: f64 = (eq8_e228_d_b0 * p.p1);
        let eq8_e230_d_b1: f64 = (eq8_e228_d_b1 * p.p1);
        let eq8_value: f64 = eq8_e230;
        let eq8_node_derivatives: [f64; 12] = [eq8_e230_d_n0, eq8_e230_d_n1, eq8_e230_d_n2, eq8_e230_d_n3, eq8_e230_d_n4, eq8_e230_d_n5, eq8_e230_d_n6, eq8_e230_d_n7, eq8_e230_d_n8, eq8_e230_d_n9, eq8_e230_d_n10, eq8_e230_d_n11];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e230_d_b0, eq8_e230_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(3),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e233: f64 = (p.p3 * s.v[181]);
        let eq9_e233_d_n0: f64 = (p.p3 * s.dn[181][0]);
        let eq9_e233_d_n1: f64 = (p.p3 * s.dn[181][1]);
        let eq9_e233_d_n2: f64 = (p.p3 * s.dn[181][2]);
        let eq9_e233_d_n3: f64 = (p.p3 * s.dn[181][3]);
        let eq9_e233_d_n4: f64 = (p.p3 * s.dn[181][4]);
        let eq9_e233_d_n5: f64 = (p.p3 * s.dn[181][5]);
        let eq9_e233_d_n6: f64 = (p.p3 * s.dn[181][6]);
        let eq9_e233_d_n7: f64 = (p.p3 * s.dn[181][7]);
        let eq9_e233_d_n8: f64 = (p.p3 * s.dn[181][8]);
        let eq9_e233_d_n9: f64 = (p.p3 * s.dn[181][9]);
        let eq9_e233_d_n10: f64 = (p.p3 * s.dn[181][10]);
        let eq9_e233_d_n11: f64 = (p.p3 * s.dn[181][11]);
        let eq9_e233_d_b0: f64 = (p.p3 * s.db[181][0]);
        let eq9_e233_d_b1: f64 = (p.p3 * s.db[181][1]);
        let eq9_e235: f64 = (eq9_e233 * p.p1);
        let eq9_e235_d_n0: f64 = (eq9_e233_d_n0 * p.p1);
        let eq9_e235_d_n1: f64 = (eq9_e233_d_n1 * p.p1);
        let eq9_e235_d_n2: f64 = (eq9_e233_d_n2 * p.p1);
        let eq9_e235_d_n3: f64 = (eq9_e233_d_n3 * p.p1);
        let eq9_e235_d_n4: f64 = (eq9_e233_d_n4 * p.p1);
        let eq9_e235_d_n5: f64 = (eq9_e233_d_n5 * p.p1);
        let eq9_e235_d_n6: f64 = (eq9_e233_d_n6 * p.p1);
        let eq9_e235_d_n7: f64 = (eq9_e233_d_n7 * p.p1);
        let eq9_e235_d_n8: f64 = (eq9_e233_d_n8 * p.p1);
        let eq9_e235_d_n9: f64 = (eq9_e233_d_n9 * p.p1);
        let eq9_e235_d_n10: f64 = (eq9_e233_d_n10 * p.p1);
        let eq9_e235_d_n11: f64 = (eq9_e233_d_n11 * p.p1);
        let eq9_e235_d_b0: f64 = (eq9_e233_d_b0 * p.p1);
        let eq9_e235_d_b1: f64 = (eq9_e233_d_b1 * p.p1);
        let eq9_value: f64 = eq9_e235;
        let eq9_node_derivatives: [f64; 12] = [eq9_e235_d_n0, eq9_e235_d_n1, eq9_e235_d_n2, eq9_e235_d_n3, eq9_e235_d_n4, eq9_e235_d_n5, eq9_e235_d_n6, eq9_e235_d_n7, eq9_e235_d_n8, eq9_e235_d_n9, eq9_e235_d_n10, eq9_e235_d_n11];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e235_d_b0, eq9_e235_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e238: f64 = (p.p3 * s.v[188]);
        let eq10_e238_d_n0: f64 = (p.p3 * s.dn[188][0]);
        let eq10_e238_d_n1: f64 = (p.p3 * s.dn[188][1]);
        let eq10_e238_d_n2: f64 = (p.p3 * s.dn[188][2]);
        let eq10_e238_d_n3: f64 = (p.p3 * s.dn[188][3]);
        let eq10_e238_d_n4: f64 = (p.p3 * s.dn[188][4]);
        let eq10_e238_d_n5: f64 = (p.p3 * s.dn[188][5]);
        let eq10_e238_d_n6: f64 = (p.p3 * s.dn[188][6]);
        let eq10_e238_d_n7: f64 = (p.p3 * s.dn[188][7]);
        let eq10_e238_d_n8: f64 = (p.p3 * s.dn[188][8]);
        let eq10_e238_d_n9: f64 = (p.p3 * s.dn[188][9]);
        let eq10_e238_d_n10: f64 = (p.p3 * s.dn[188][10]);
        let eq10_e238_d_n11: f64 = (p.p3 * s.dn[188][11]);
        let eq10_e238_d_b0: f64 = (p.p3 * s.db[188][0]);
        let eq10_e238_d_b1: f64 = (p.p3 * s.db[188][1]);
        let eq10_e240: f64 = (eq10_e238 * p.p1);
        let eq10_e240_d_n0: f64 = (eq10_e238_d_n0 * p.p1);
        let eq10_e240_d_n1: f64 = (eq10_e238_d_n1 * p.p1);
        let eq10_e240_d_n2: f64 = (eq10_e238_d_n2 * p.p1);
        let eq10_e240_d_n3: f64 = (eq10_e238_d_n3 * p.p1);
        let eq10_e240_d_n4: f64 = (eq10_e238_d_n4 * p.p1);
        let eq10_e240_d_n5: f64 = (eq10_e238_d_n5 * p.p1);
        let eq10_e240_d_n6: f64 = (eq10_e238_d_n6 * p.p1);
        let eq10_e240_d_n7: f64 = (eq10_e238_d_n7 * p.p1);
        let eq10_e240_d_n8: f64 = (eq10_e238_d_n8 * p.p1);
        let eq10_e240_d_n9: f64 = (eq10_e238_d_n9 * p.p1);
        let eq10_e240_d_n10: f64 = (eq10_e238_d_n10 * p.p1);
        let eq10_e240_d_n11: f64 = (eq10_e238_d_n11 * p.p1);
        let eq10_e240_d_b0: f64 = (eq10_e238_d_b0 * p.p1);
        let eq10_e240_d_b1: f64 = (eq10_e238_d_b1 * p.p1);
        let eq10_value: f64 = eq10_e240;
        let eq10_node_derivatives: [f64; 12] = [eq10_e240_d_n0, eq10_e240_d_n1, eq10_e240_d_n2, eq10_e240_d_n3, eq10_e240_d_n4, eq10_e240_d_n5, eq10_e240_d_n6, eq10_e240_d_n7, eq10_e240_d_n8, eq10_e240_d_n9, eq10_e240_d_n10, eq10_e240_d_n11];
        let eq10_branch_derivatives: [f64; 2] = [eq10_e240_d_b0, eq10_e240_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e243: f64 = (-1.0);
        let eq11_e245: f64 = (eq11_e243 * s.v[209]);
        let eq11_e245_d_n0: f64 = (eq11_e243 * s.dn[209][0]);
        let eq11_e245_d_n1: f64 = (eq11_e243 * s.dn[209][1]);
        let eq11_e245_d_n2: f64 = (eq11_e243 * s.dn[209][2]);
        let eq11_e245_d_n3: f64 = (eq11_e243 * s.dn[209][3]);
        let eq11_e245_d_n4: f64 = (eq11_e243 * s.dn[209][4]);
        let eq11_e245_d_n5: f64 = (eq11_e243 * s.dn[209][5]);
        let eq11_e245_d_n6: f64 = (eq11_e243 * s.dn[209][6]);
        let eq11_e245_d_n7: f64 = (eq11_e243 * s.dn[209][7]);
        let eq11_e245_d_n8: f64 = (eq11_e243 * s.dn[209][8]);
        let eq11_e245_d_n9: f64 = (eq11_e243 * s.dn[209][9]);
        let eq11_e245_d_n10: f64 = (eq11_e243 * s.dn[209][10]);
        let eq11_e245_d_n11: f64 = (eq11_e243 * s.dn[209][11]);
        let eq11_e245_d_b0: f64 = (eq11_e243 * s.db[209][0]);
        let eq11_e245_d_b1: f64 = (eq11_e243 * s.db[209][1]);
        let eq11_e246: f64 = (p.p3 * eq11_e245);
        let eq11_e246_d_n0: f64 = (p.p3 * eq11_e245_d_n0);
        let eq11_e246_d_n1: f64 = (p.p3 * eq11_e245_d_n1);
        let eq11_e246_d_n2: f64 = (p.p3 * eq11_e245_d_n2);
        let eq11_e246_d_n3: f64 = (p.p3 * eq11_e245_d_n3);
        let eq11_e246_d_n4: f64 = (p.p3 * eq11_e245_d_n4);
        let eq11_e246_d_n5: f64 = (p.p3 * eq11_e245_d_n5);
        let eq11_e246_d_n6: f64 = (p.p3 * eq11_e245_d_n6);
        let eq11_e246_d_n7: f64 = (p.p3 * eq11_e245_d_n7);
        let eq11_e246_d_n8: f64 = (p.p3 * eq11_e245_d_n8);
        let eq11_e246_d_n9: f64 = (p.p3 * eq11_e245_d_n9);
        let eq11_e246_d_n10: f64 = (p.p3 * eq11_e245_d_n10);
        let eq11_e246_d_n11: f64 = (p.p3 * eq11_e245_d_n11);
        let eq11_e246_d_b0: f64 = (p.p3 * eq11_e245_d_b0);
        let eq11_e246_d_b1: f64 = (p.p3 * eq11_e245_d_b1);
        let eq11_e248: f64 = (eq11_e246 * p.p1);
        let eq11_e248_d_n0: f64 = (eq11_e246_d_n0 * p.p1);
        let eq11_e248_d_n1: f64 = (eq11_e246_d_n1 * p.p1);
        let eq11_e248_d_n2: f64 = (eq11_e246_d_n2 * p.p1);
        let eq11_e248_d_n3: f64 = (eq11_e246_d_n3 * p.p1);
        let eq11_e248_d_n4: f64 = (eq11_e246_d_n4 * p.p1);
        let eq11_e248_d_n5: f64 = (eq11_e246_d_n5 * p.p1);
        let eq11_e248_d_n6: f64 = (eq11_e246_d_n6 * p.p1);
        let eq11_e248_d_n7: f64 = (eq11_e246_d_n7 * p.p1);
        let eq11_e248_d_n8: f64 = (eq11_e246_d_n8 * p.p1);
        let eq11_e248_d_n9: f64 = (eq11_e246_d_n9 * p.p1);
        let eq11_e248_d_n10: f64 = (eq11_e246_d_n10 * p.p1);
        let eq11_e248_d_n11: f64 = (eq11_e246_d_n11 * p.p1);
        let eq11_e248_d_b0: f64 = (eq11_e246_d_b0 * p.p1);
        let eq11_e248_d_b1: f64 = (eq11_e246_d_b1 * p.p1);
        let eq11_value: f64 = eq11_e248;
        let eq11_node_derivatives: [f64; 12] = [eq11_e248_d_n0, eq11_e248_d_n1, eq11_e248_d_n2, eq11_e248_d_n3, eq11_e248_d_n4, eq11_e248_d_n5, eq11_e248_d_n6, eq11_e248_d_n7, eq11_e248_d_n8, eq11_e248_d_n9, eq11_e248_d_n10, eq11_e248_d_n11];
        let eq11_branch_derivatives: [f64; 2] = [eq11_e248_d_b0, eq11_e248_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e251: f64 = (p.p3 * s.v[259]);
        let eq12_e251_d_n0: f64 = (p.p3 * s.dn[259][0]);
        let eq12_e251_d_n1: f64 = (p.p3 * s.dn[259][1]);
        let eq12_e251_d_n2: f64 = (p.p3 * s.dn[259][2]);
        let eq12_e251_d_n3: f64 = (p.p3 * s.dn[259][3]);
        let eq12_e251_d_n4: f64 = (p.p3 * s.dn[259][4]);
        let eq12_e251_d_n5: f64 = (p.p3 * s.dn[259][5]);
        let eq12_e251_d_n6: f64 = (p.p3 * s.dn[259][6]);
        let eq12_e251_d_n7: f64 = (p.p3 * s.dn[259][7]);
        let eq12_e251_d_n8: f64 = (p.p3 * s.dn[259][8]);
        let eq12_e251_d_n9: f64 = (p.p3 * s.dn[259][9]);
        let eq12_e251_d_n10: f64 = (p.p3 * s.dn[259][10]);
        let eq12_e251_d_n11: f64 = (p.p3 * s.dn[259][11]);
        let eq12_e251_d_b0: f64 = (p.p3 * s.db[259][0]);
        let eq12_e251_d_b1: f64 = (p.p3 * s.db[259][1]);
        let eq12_e253: f64 = (eq12_e251 / s.v[28]);
        let eq12_e253_d_n0: f64 = (((eq12_e251_d_n0 * s.v[28]) - (eq12_e251 * s.dn[28][0])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n1: f64 = (((eq12_e251_d_n1 * s.v[28]) - (eq12_e251 * s.dn[28][1])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n2: f64 = (((eq12_e251_d_n2 * s.v[28]) - (eq12_e251 * s.dn[28][2])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n3: f64 = (((eq12_e251_d_n3 * s.v[28]) - (eq12_e251 * s.dn[28][3])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n4: f64 = (((eq12_e251_d_n4 * s.v[28]) - (eq12_e251 * s.dn[28][4])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n5: f64 = (((eq12_e251_d_n5 * s.v[28]) - (eq12_e251 * s.dn[28][5])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n6: f64 = (((eq12_e251_d_n6 * s.v[28]) - (eq12_e251 * s.dn[28][6])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n7: f64 = (((eq12_e251_d_n7 * s.v[28]) - (eq12_e251 * s.dn[28][7])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n8: f64 = (((eq12_e251_d_n8 * s.v[28]) - (eq12_e251 * s.dn[28][8])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n9: f64 = (((eq12_e251_d_n9 * s.v[28]) - (eq12_e251 * s.dn[28][9])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n10: f64 = (((eq12_e251_d_n10 * s.v[28]) - (eq12_e251 * s.dn[28][10])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_n11: f64 = (((eq12_e251_d_n11 * s.v[28]) - (eq12_e251 * s.dn[28][11])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_b0: f64 = (((eq12_e251_d_b0 * s.v[28]) - (eq12_e251 * s.db[28][0])) / (s.v[28] * s.v[28]));
        let eq12_e253_d_b1: f64 = (((eq12_e251_d_b1 * s.v[28]) - (eq12_e251 * s.db[28][1])) / (s.v[28] * s.v[28]));
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
        let eq12_e255_d_n11: f64 = (eq12_e253_d_n11 * p.p1);
        let eq12_e255_d_b0: f64 = (eq12_e253_d_b0 * p.p1);
        let eq12_e255_d_b1: f64 = (eq12_e253_d_b1 * p.p1);
        let eq12_value: f64 = eq12_e255;
        let eq12_node_derivatives: [f64; 12] = [eq12_e255_d_n0, eq12_e255_d_n1, eq12_e255_d_n2, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, eq12_e255_d_n10, eq12_e255_d_n11];
        let eq12_branch_derivatives: [f64; 2] = [eq12_e255_d_b0, eq12_e255_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(4),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e258: f64 = (p.p3 * s.v[260]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[260][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[260][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[260][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[260][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[260][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[260][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[260][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[260][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[260][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[260][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[260][10]);
        let eq13_e258_d_n11: f64 = (p.p3 * s.dn[260][11]);
        let eq13_e258_d_b0: f64 = (p.p3 * s.db[260][0]);
        let eq13_e258_d_b1: f64 = (p.p3 * s.db[260][1]);
        let eq13_e260: f64 = (eq13_e258 / s.v[30]);
        let eq13_e260_d_n0: f64 = (((eq13_e258_d_n0 * s.v[30]) - (eq13_e258 * s.dn[30][0])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n1: f64 = (((eq13_e258_d_n1 * s.v[30]) - (eq13_e258 * s.dn[30][1])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n2: f64 = (((eq13_e258_d_n2 * s.v[30]) - (eq13_e258 * s.dn[30][2])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n3: f64 = (((eq13_e258_d_n3 * s.v[30]) - (eq13_e258 * s.dn[30][3])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n4: f64 = (((eq13_e258_d_n4 * s.v[30]) - (eq13_e258 * s.dn[30][4])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n5: f64 = (((eq13_e258_d_n5 * s.v[30]) - (eq13_e258 * s.dn[30][5])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n6: f64 = (((eq13_e258_d_n6 * s.v[30]) - (eq13_e258 * s.dn[30][6])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n7: f64 = (((eq13_e258_d_n7 * s.v[30]) - (eq13_e258 * s.dn[30][7])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n8: f64 = (((eq13_e258_d_n8 * s.v[30]) - (eq13_e258 * s.dn[30][8])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n9: f64 = (((eq13_e258_d_n9 * s.v[30]) - (eq13_e258 * s.dn[30][9])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n10: f64 = (((eq13_e258_d_n10 * s.v[30]) - (eq13_e258 * s.dn[30][10])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_n11: f64 = (((eq13_e258_d_n11 * s.v[30]) - (eq13_e258 * s.dn[30][11])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_b0: f64 = (((eq13_e258_d_b0 * s.v[30]) - (eq13_e258 * s.db[30][0])) / (s.v[30] * s.v[30]));
        let eq13_e260_d_b1: f64 = (((eq13_e258_d_b1 * s.v[30]) - (eq13_e258 * s.db[30][1])) / (s.v[30] * s.v[30]));
        let eq13_e262: f64 = (eq13_e260 * p.p1);
        let eq13_e262_d_n0: f64 = (eq13_e260_d_n0 * p.p1);
        let eq13_e262_d_n1: f64 = (eq13_e260_d_n1 * p.p1);
        let eq13_e262_d_n2: f64 = (eq13_e260_d_n2 * p.p1);
        let eq13_e262_d_n3: f64 = (eq13_e260_d_n3 * p.p1);
        let eq13_e262_d_n4: f64 = (eq13_e260_d_n4 * p.p1);
        let eq13_e262_d_n5: f64 = (eq13_e260_d_n5 * p.p1);
        let eq13_e262_d_n6: f64 = (eq13_e260_d_n6 * p.p1);
        let eq13_e262_d_n7: f64 = (eq13_e260_d_n7 * p.p1);
        let eq13_e262_d_n8: f64 = (eq13_e260_d_n8 * p.p1);
        let eq13_e262_d_n9: f64 = (eq13_e260_d_n9 * p.p1);
        let eq13_e262_d_n10: f64 = (eq13_e260_d_n10 * p.p1);
        let eq13_e262_d_n11: f64 = (eq13_e260_d_n11 * p.p1);
        let eq13_e262_d_b0: f64 = (eq13_e260_d_b0 * p.p1);
        let eq13_e262_d_b1: f64 = (eq13_e260_d_b1 * p.p1);
        let eq13_value: f64 = eq13_e262;
        let eq13_node_derivatives: [f64; 12] = [eq13_e262_d_n0, eq13_e262_d_n1, eq13_e262_d_n2, eq13_e262_d_n3, eq13_e262_d_n4, eq13_e262_d_n5, eq13_e262_d_n6, eq13_e262_d_n7, eq13_e262_d_n8, eq13_e262_d_n9, eq13_e262_d_n10, eq13_e262_d_n11];
        let eq13_branch_derivatives: [f64; 2] = [eq13_e262_d_b0, eq13_e262_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e266: f64 = (s.v[215] + s.v[220]);
        let eq14_e266_d_n0: f64 = (s.dn[215][0] + s.dn[220][0]);
        let eq14_e266_d_n1: f64 = (s.dn[215][1] + s.dn[220][1]);
        let eq14_e266_d_n2: f64 = (s.dn[215][2] + s.dn[220][2]);
        let eq14_e266_d_n3: f64 = (s.dn[215][3] + s.dn[220][3]);
        let eq14_e266_d_n4: f64 = (s.dn[215][4] + s.dn[220][4]);
        let eq14_e266_d_n5: f64 = (s.dn[215][5] + s.dn[220][5]);
        let eq14_e266_d_n6: f64 = (s.dn[215][6] + s.dn[220][6]);
        let eq14_e266_d_n7: f64 = (s.dn[215][7] + s.dn[220][7]);
        let eq14_e266_d_n8: f64 = (s.dn[215][8] + s.dn[220][8]);
        let eq14_e266_d_n9: f64 = (s.dn[215][9] + s.dn[220][9]);
        let eq14_e266_d_n10: f64 = (s.dn[215][10] + s.dn[220][10]);
        let eq14_e266_d_n11: f64 = (s.dn[215][11] + s.dn[220][11]);
        let eq14_e266_d_b0: f64 = (s.db[215][0] + s.db[220][0]);
        let eq14_e266_d_b1: f64 = (s.db[215][1] + s.db[220][1]);
        let eq14_e268: f64 = (eq14_e266 + s.v[235]);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + s.dn[235][0]);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + s.dn[235][1]);
        let eq14_e268_d_n2: f64 = (eq14_e266_d_n2 + s.dn[235][2]);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + s.dn[235][3]);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + s.dn[235][4]);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + s.dn[235][5]);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + s.dn[235][6]);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + s.dn[235][7]);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + s.dn[235][8]);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + s.dn[235][9]);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + s.dn[235][10]);
        let eq14_e268_d_n11: f64 = (eq14_e266_d_n11 + s.dn[235][11]);
        let eq14_e268_d_b0: f64 = (eq14_e266_d_b0 + s.db[235][0]);
        let eq14_e268_d_b1: f64 = (eq14_e266_d_b1 + s.db[235][1]);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n2: f64 = (p.p3 * eq14_e268_d_n2);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e269_d_n11: f64 = (p.p3 * eq14_e268_d_n11);
        let eq14_e269_d_b0: f64 = (p.p3 * eq14_e268_d_b0);
        let eq14_e269_d_b1: f64 = (p.p3 * eq14_e268_d_b1);
        let eq14_e270: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq14_e269);
        let eq14_e270_d_n0: f64 = (eq14_e269_d_n0 * ddt_scale);
        let eq14_e270_d_n1: f64 = (eq14_e269_d_n1 * ddt_scale);
        let eq14_e270_d_n2: f64 = (eq14_e269_d_n2 * ddt_scale);
        let eq14_e270_d_n3: f64 = (eq14_e269_d_n3 * ddt_scale);
        let eq14_e270_d_n4: f64 = (eq14_e269_d_n4 * ddt_scale);
        let eq14_e270_d_n5: f64 = (eq14_e269_d_n5 * ddt_scale);
        let eq14_e270_d_n6: f64 = (eq14_e269_d_n6 * ddt_scale);
        let eq14_e270_d_n7: f64 = (eq14_e269_d_n7 * ddt_scale);
        let eq14_e270_d_n8: f64 = (eq14_e269_d_n8 * ddt_scale);
        let eq14_e270_d_n9: f64 = (eq14_e269_d_n9 * ddt_scale);
        let eq14_e270_d_n10: f64 = (eq14_e269_d_n10 * ddt_scale);
        let eq14_e270_d_n11: f64 = (eq14_e269_d_n11 * ddt_scale);
        let eq14_e270_d_b0: f64 = (eq14_e269_d_b0 * ddt_scale);
        let eq14_e270_d_b1: f64 = (eq14_e269_d_b1 * ddt_scale);
        let eq14_e272: f64 = (eq14_e270 * p.p1);
        let eq14_e272_d_n0: f64 = (eq14_e270_d_n0 * p.p1);
        let eq14_e272_d_n1: f64 = (eq14_e270_d_n1 * p.p1);
        let eq14_e272_d_n2: f64 = (eq14_e270_d_n2 * p.p1);
        let eq14_e272_d_n3: f64 = (eq14_e270_d_n3 * p.p1);
        let eq14_e272_d_n4: f64 = (eq14_e270_d_n4 * p.p1);
        let eq14_e272_d_n5: f64 = (eq14_e270_d_n5 * p.p1);
        let eq14_e272_d_n6: f64 = (eq14_e270_d_n6 * p.p1);
        let eq14_e272_d_n7: f64 = (eq14_e270_d_n7 * p.p1);
        let eq14_e272_d_n8: f64 = (eq14_e270_d_n8 * p.p1);
        let eq14_e272_d_n9: f64 = (eq14_e270_d_n9 * p.p1);
        let eq14_e272_d_n10: f64 = (eq14_e270_d_n10 * p.p1);
        let eq14_e272_d_n11: f64 = (eq14_e270_d_n11 * p.p1);
        let eq14_e272_d_b0: f64 = (eq14_e270_d_b0 * p.p1);
        let eq14_e272_d_b1: f64 = (eq14_e270_d_b1 * p.p1);
        let eq14_value: f64 = eq14_e272;
        let eq14_node_derivatives: [f64; 12] = [eq14_e272_d_n0, eq14_e272_d_n1, eq14_e272_d_n2, eq14_e272_d_n3, eq14_e272_d_n4, eq14_e272_d_n5, eq14_e272_d_n6, eq14_e272_d_n7, eq14_e272_d_n8, eq14_e272_d_n9, eq14_e272_d_n10, eq14_e272_d_n11];
        let eq14_branch_derivatives: [f64; 2] = [eq14_e272_d_b0, eq14_e272_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e275: f64 = (p.p3 * s.v[217]);
        let eq15_e275_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq15_e275_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq15_e275_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq15_e275_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq15_e275_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq15_e275_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq15_e275_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq15_e275_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq15_e275_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq15_e275_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq15_e275_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq15_e275_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq15_e275_d_b0: f64 = (p.p3 * s.db[217][0]);
        let eq15_e275_d_b1: f64 = (p.p3 * s.db[217][1]);
        let eq15_e276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq15_e275);
        let eq15_e276_d_n0: f64 = (eq15_e275_d_n0 * ddt_scale);
        let eq15_e276_d_n1: f64 = (eq15_e275_d_n1 * ddt_scale);
        let eq15_e276_d_n2: f64 = (eq15_e275_d_n2 * ddt_scale);
        let eq15_e276_d_n3: f64 = (eq15_e275_d_n3 * ddt_scale);
        let eq15_e276_d_n4: f64 = (eq15_e275_d_n4 * ddt_scale);
        let eq15_e276_d_n5: f64 = (eq15_e275_d_n5 * ddt_scale);
        let eq15_e276_d_n6: f64 = (eq15_e275_d_n6 * ddt_scale);
        let eq15_e276_d_n7: f64 = (eq15_e275_d_n7 * ddt_scale);
        let eq15_e276_d_n8: f64 = (eq15_e275_d_n8 * ddt_scale);
        let eq15_e276_d_n9: f64 = (eq15_e275_d_n9 * ddt_scale);
        let eq15_e276_d_n10: f64 = (eq15_e275_d_n10 * ddt_scale);
        let eq15_e276_d_n11: f64 = (eq15_e275_d_n11 * ddt_scale);
        let eq15_e276_d_b0: f64 = (eq15_e275_d_b0 * ddt_scale);
        let eq15_e276_d_b1: f64 = (eq15_e275_d_b1 * ddt_scale);
        let eq15_e278: f64 = (eq15_e276 * p.p1);
        let eq15_e278_d_n0: f64 = (eq15_e276_d_n0 * p.p1);
        let eq15_e278_d_n1: f64 = (eq15_e276_d_n1 * p.p1);
        let eq15_e278_d_n2: f64 = (eq15_e276_d_n2 * p.p1);
        let eq15_e278_d_n3: f64 = (eq15_e276_d_n3 * p.p1);
        let eq15_e278_d_n4: f64 = (eq15_e276_d_n4 * p.p1);
        let eq15_e278_d_n5: f64 = (eq15_e276_d_n5 * p.p1);
        let eq15_e278_d_n6: f64 = (eq15_e276_d_n6 * p.p1);
        let eq15_e278_d_n7: f64 = (eq15_e276_d_n7 * p.p1);
        let eq15_e278_d_n8: f64 = (eq15_e276_d_n8 * p.p1);
        let eq15_e278_d_n9: f64 = (eq15_e276_d_n9 * p.p1);
        let eq15_e278_d_n10: f64 = (eq15_e276_d_n10 * p.p1);
        let eq15_e278_d_n11: f64 = (eq15_e276_d_n11 * p.p1);
        let eq15_e278_d_b0: f64 = (eq15_e276_d_b0 * p.p1);
        let eq15_e278_d_b1: f64 = (eq15_e276_d_b1 * p.p1);
        let eq15_value: f64 = eq15_e278;
        let eq15_node_derivatives: [f64; 12] = [eq15_e278_d_n0, eq15_e278_d_n1, eq15_e278_d_n2, eq15_e278_d_n3, eq15_e278_d_n4, eq15_e278_d_n5, eq15_e278_d_n6, eq15_e278_d_n7, eq15_e278_d_n8, eq15_e278_d_n9, eq15_e278_d_n10, eq15_e278_d_n11];
        let eq15_branch_derivatives: [f64; 2] = [eq15_e278_d_b0, eq15_e278_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
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
        let eq16_e282: f64 = (s.v[218] + s.v[221]);
        let eq16_e282_d_n0: f64 = (s.dn[218][0] + s.dn[221][0]);
        let eq16_e282_d_n1: f64 = (s.dn[218][1] + s.dn[221][1]);
        let eq16_e282_d_n2: f64 = (s.dn[218][2] + s.dn[221][2]);
        let eq16_e282_d_n3: f64 = (s.dn[218][3] + s.dn[221][3]);
        let eq16_e282_d_n4: f64 = (s.dn[218][4] + s.dn[221][4]);
        let eq16_e282_d_n5: f64 = (s.dn[218][5] + s.dn[221][5]);
        let eq16_e282_d_n6: f64 = (s.dn[218][6] + s.dn[221][6]);
        let eq16_e282_d_n7: f64 = (s.dn[218][7] + s.dn[221][7]);
        let eq16_e282_d_n8: f64 = (s.dn[218][8] + s.dn[221][8]);
        let eq16_e282_d_n9: f64 = (s.dn[218][9] + s.dn[221][9]);
        let eq16_e282_d_n10: f64 = (s.dn[218][10] + s.dn[221][10]);
        let eq16_e282_d_n11: f64 = (s.dn[218][11] + s.dn[221][11]);
        let eq16_e282_d_b0: f64 = (s.db[218][0] + s.db[221][0]);
        let eq16_e282_d_b1: f64 = (s.db[218][1] + s.db[221][1]);
        let eq16_e284: f64 = (eq16_e282 + s.v[238]);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + s.dn[238][0]);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + s.dn[238][1]);
        let eq16_e284_d_n2: f64 = (eq16_e282_d_n2 + s.dn[238][2]);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + s.dn[238][3]);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + s.dn[238][4]);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + s.dn[238][5]);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + s.dn[238][6]);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + s.dn[238][7]);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + s.dn[238][8]);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + s.dn[238][9]);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + s.dn[238][10]);
        let eq16_e284_d_n11: f64 = (eq16_e282_d_n11 + s.dn[238][11]);
        let eq16_e284_d_b0: f64 = (eq16_e282_d_b0 + s.db[238][0]);
        let eq16_e284_d_b1: f64 = (eq16_e282_d_b1 + s.db[238][1]);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n2: f64 = (p.p3 * eq16_e284_d_n2);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e285_d_n11: f64 = (p.p3 * eq16_e284_d_n11);
        let eq16_e285_d_b0: f64 = (p.p3 * eq16_e284_d_b0);
        let eq16_e285_d_b1: f64 = (p.p3 * eq16_e284_d_b1);
        let eq16_e286: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq16_e285);
        let eq16_e286_d_n0: f64 = (eq16_e285_d_n0 * ddt_scale);
        let eq16_e286_d_n1: f64 = (eq16_e285_d_n1 * ddt_scale);
        let eq16_e286_d_n2: f64 = (eq16_e285_d_n2 * ddt_scale);
        let eq16_e286_d_n3: f64 = (eq16_e285_d_n3 * ddt_scale);
        let eq16_e286_d_n4: f64 = (eq16_e285_d_n4 * ddt_scale);
        let eq16_e286_d_n5: f64 = (eq16_e285_d_n5 * ddt_scale);
        let eq16_e286_d_n6: f64 = (eq16_e285_d_n6 * ddt_scale);
        let eq16_e286_d_n7: f64 = (eq16_e285_d_n7 * ddt_scale);
        let eq16_e286_d_n8: f64 = (eq16_e285_d_n8 * ddt_scale);
        let eq16_e286_d_n9: f64 = (eq16_e285_d_n9 * ddt_scale);
        let eq16_e286_d_n10: f64 = (eq16_e285_d_n10 * ddt_scale);
        let eq16_e286_d_n11: f64 = (eq16_e285_d_n11 * ddt_scale);
        let eq16_e286_d_b0: f64 = (eq16_e285_d_b0 * ddt_scale);
        let eq16_e286_d_b1: f64 = (eq16_e285_d_b1 * ddt_scale);
        let eq16_e288: f64 = (eq16_e286 * p.p1);
        let eq16_e288_d_n0: f64 = (eq16_e286_d_n0 * p.p1);
        let eq16_e288_d_n1: f64 = (eq16_e286_d_n1 * p.p1);
        let eq16_e288_d_n2: f64 = (eq16_e286_d_n2 * p.p1);
        let eq16_e288_d_n3: f64 = (eq16_e286_d_n3 * p.p1);
        let eq16_e288_d_n4: f64 = (eq16_e286_d_n4 * p.p1);
        let eq16_e288_d_n5: f64 = (eq16_e286_d_n5 * p.p1);
        let eq16_e288_d_n6: f64 = (eq16_e286_d_n6 * p.p1);
        let eq16_e288_d_n7: f64 = (eq16_e286_d_n7 * p.p1);
        let eq16_e288_d_n8: f64 = (eq16_e286_d_n8 * p.p1);
        let eq16_e288_d_n9: f64 = (eq16_e286_d_n9 * p.p1);
        let eq16_e288_d_n10: f64 = (eq16_e286_d_n10 * p.p1);
        let eq16_e288_d_n11: f64 = (eq16_e286_d_n11 * p.p1);
        let eq16_e288_d_b0: f64 = (eq16_e286_d_b0 * p.p1);
        let eq16_e288_d_b1: f64 = (eq16_e286_d_b1 * p.p1);
        let eq16_value: f64 = eq16_e288;
        let eq16_node_derivatives: [f64; 12] = [eq16_e288_d_n0, eq16_e288_d_n1, eq16_e288_d_n2, eq16_e288_d_n3, eq16_e288_d_n4, eq16_e288_d_n5, eq16_e288_d_n6, eq16_e288_d_n7, eq16_e288_d_n8, eq16_e288_d_n9, eq16_e288_d_n10, eq16_e288_d_n11];
        let eq16_branch_derivatives: [f64; 2] = [eq16_e288_d_b0, eq16_e288_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e291: f64 = (p.p3 * s.v[233]);
        let eq17_e291_d_n0: f64 = (p.p3 * s.dn[233][0]);
        let eq17_e291_d_n1: f64 = (p.p3 * s.dn[233][1]);
        let eq17_e291_d_n2: f64 = (p.p3 * s.dn[233][2]);
        let eq17_e291_d_n3: f64 = (p.p3 * s.dn[233][3]);
        let eq17_e291_d_n4: f64 = (p.p3 * s.dn[233][4]);
        let eq17_e291_d_n5: f64 = (p.p3 * s.dn[233][5]);
        let eq17_e291_d_n6: f64 = (p.p3 * s.dn[233][6]);
        let eq17_e291_d_n7: f64 = (p.p3 * s.dn[233][7]);
        let eq17_e291_d_n8: f64 = (p.p3 * s.dn[233][8]);
        let eq17_e291_d_n9: f64 = (p.p3 * s.dn[233][9]);
        let eq17_e291_d_n10: f64 = (p.p3 * s.dn[233][10]);
        let eq17_e291_d_n11: f64 = (p.p3 * s.dn[233][11]);
        let eq17_e291_d_b0: f64 = (p.p3 * s.db[233][0]);
        let eq17_e291_d_b1: f64 = (p.p3 * s.db[233][1]);
        let eq17_e292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq17_e291);
        let eq17_e292_d_n0: f64 = (eq17_e291_d_n0 * ddt_scale);
        let eq17_e292_d_n1: f64 = (eq17_e291_d_n1 * ddt_scale);
        let eq17_e292_d_n2: f64 = (eq17_e291_d_n2 * ddt_scale);
        let eq17_e292_d_n3: f64 = (eq17_e291_d_n3 * ddt_scale);
        let eq17_e292_d_n4: f64 = (eq17_e291_d_n4 * ddt_scale);
        let eq17_e292_d_n5: f64 = (eq17_e291_d_n5 * ddt_scale);
        let eq17_e292_d_n6: f64 = (eq17_e291_d_n6 * ddt_scale);
        let eq17_e292_d_n7: f64 = (eq17_e291_d_n7 * ddt_scale);
        let eq17_e292_d_n8: f64 = (eq17_e291_d_n8 * ddt_scale);
        let eq17_e292_d_n9: f64 = (eq17_e291_d_n9 * ddt_scale);
        let eq17_e292_d_n10: f64 = (eq17_e291_d_n10 * ddt_scale);
        let eq17_e292_d_n11: f64 = (eq17_e291_d_n11 * ddt_scale);
        let eq17_e292_d_b0: f64 = (eq17_e291_d_b0 * ddt_scale);
        let eq17_e292_d_b1: f64 = (eq17_e291_d_b1 * ddt_scale);
        let eq17_e294: f64 = (eq17_e292 * p.p1);
        let eq17_e294_d_n0: f64 = (eq17_e292_d_n0 * p.p1);
        let eq17_e294_d_n1: f64 = (eq17_e292_d_n1 * p.p1);
        let eq17_e294_d_n2: f64 = (eq17_e292_d_n2 * p.p1);
        let eq17_e294_d_n3: f64 = (eq17_e292_d_n3 * p.p1);
        let eq17_e294_d_n4: f64 = (eq17_e292_d_n4 * p.p1);
        let eq17_e294_d_n5: f64 = (eq17_e292_d_n5 * p.p1);
        let eq17_e294_d_n6: f64 = (eq17_e292_d_n6 * p.p1);
        let eq17_e294_d_n7: f64 = (eq17_e292_d_n7 * p.p1);
        let eq17_e294_d_n8: f64 = (eq17_e292_d_n8 * p.p1);
        let eq17_e294_d_n9: f64 = (eq17_e292_d_n9 * p.p1);
        let eq17_e294_d_n10: f64 = (eq17_e292_d_n10 * p.p1);
        let eq17_e294_d_n11: f64 = (eq17_e292_d_n11 * p.p1);
        let eq17_e294_d_b0: f64 = (eq17_e292_d_b0 * p.p1);
        let eq17_e294_d_b1: f64 = (eq17_e292_d_b1 * p.p1);
        let eq17_value: f64 = eq17_e294;
        let eq17_node_derivatives: [f64; 12] = [eq17_e294_d_n0, eq17_e294_d_n1, eq17_e294_d_n2, eq17_e294_d_n3, eq17_e294_d_n4, eq17_e294_d_n5, eq17_e294_d_n6, eq17_e294_d_n7, eq17_e294_d_n8, eq17_e294_d_n9, eq17_e294_d_n10, eq17_e294_d_n11];
        let eq17_branch_derivatives: [f64; 2] = [eq17_e294_d_b0, eq17_e294_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e297: f64 = (p.p3 * s.v[222]);
        let eq18_e297_d_n0: f64 = (p.p3 * s.dn[222][0]);
        let eq18_e297_d_n1: f64 = (p.p3 * s.dn[222][1]);
        let eq18_e297_d_n2: f64 = (p.p3 * s.dn[222][2]);
        let eq18_e297_d_n3: f64 = (p.p3 * s.dn[222][3]);
        let eq18_e297_d_n4: f64 = (p.p3 * s.dn[222][4]);
        let eq18_e297_d_n5: f64 = (p.p3 * s.dn[222][5]);
        let eq18_e297_d_n6: f64 = (p.p3 * s.dn[222][6]);
        let eq18_e297_d_n7: f64 = (p.p3 * s.dn[222][7]);
        let eq18_e297_d_n8: f64 = (p.p3 * s.dn[222][8]);
        let eq18_e297_d_n9: f64 = (p.p3 * s.dn[222][9]);
        let eq18_e297_d_n10: f64 = (p.p3 * s.dn[222][10]);
        let eq18_e297_d_n11: f64 = (p.p3 * s.dn[222][11]);
        let eq18_e297_d_b0: f64 = (p.p3 * s.db[222][0]);
        let eq18_e297_d_b1: f64 = (p.p3 * s.db[222][1]);
        let eq18_e298: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq18_e297);
        let eq18_e298_d_n0: f64 = (eq18_e297_d_n0 * ddt_scale);
        let eq18_e298_d_n1: f64 = (eq18_e297_d_n1 * ddt_scale);
        let eq18_e298_d_n2: f64 = (eq18_e297_d_n2 * ddt_scale);
        let eq18_e298_d_n3: f64 = (eq18_e297_d_n3 * ddt_scale);
        let eq18_e298_d_n4: f64 = (eq18_e297_d_n4 * ddt_scale);
        let eq18_e298_d_n5: f64 = (eq18_e297_d_n5 * ddt_scale);
        let eq18_e298_d_n6: f64 = (eq18_e297_d_n6 * ddt_scale);
        let eq18_e298_d_n7: f64 = (eq18_e297_d_n7 * ddt_scale);
        let eq18_e298_d_n8: f64 = (eq18_e297_d_n8 * ddt_scale);
        let eq18_e298_d_n9: f64 = (eq18_e297_d_n9 * ddt_scale);
        let eq18_e298_d_n10: f64 = (eq18_e297_d_n10 * ddt_scale);
        let eq18_e298_d_n11: f64 = (eq18_e297_d_n11 * ddt_scale);
        let eq18_e298_d_b0: f64 = (eq18_e297_d_b0 * ddt_scale);
        let eq18_e298_d_b1: f64 = (eq18_e297_d_b1 * ddt_scale);
        let eq18_e300: f64 = (eq18_e298 * p.p1);
        let eq18_e300_d_n0: f64 = (eq18_e298_d_n0 * p.p1);
        let eq18_e300_d_n1: f64 = (eq18_e298_d_n1 * p.p1);
        let eq18_e300_d_n2: f64 = (eq18_e298_d_n2 * p.p1);
        let eq18_e300_d_n3: f64 = (eq18_e298_d_n3 * p.p1);
        let eq18_e300_d_n4: f64 = (eq18_e298_d_n4 * p.p1);
        let eq18_e300_d_n5: f64 = (eq18_e298_d_n5 * p.p1);
        let eq18_e300_d_n6: f64 = (eq18_e298_d_n6 * p.p1);
        let eq18_e300_d_n7: f64 = (eq18_e298_d_n7 * p.p1);
        let eq18_e300_d_n8: f64 = (eq18_e298_d_n8 * p.p1);
        let eq18_e300_d_n9: f64 = (eq18_e298_d_n9 * p.p1);
        let eq18_e300_d_n10: f64 = (eq18_e298_d_n10 * p.p1);
        let eq18_e300_d_n11: f64 = (eq18_e298_d_n11 * p.p1);
        let eq18_e300_d_b0: f64 = (eq18_e298_d_b0 * p.p1);
        let eq18_e300_d_b1: f64 = (eq18_e298_d_b1 * p.p1);
        let eq18_value: f64 = eq18_e300;
        let eq18_node_derivatives: [f64; 12] = [eq18_e300_d_n0, eq18_e300_d_n1, eq18_e300_d_n2, eq18_e300_d_n3, eq18_e300_d_n4, eq18_e300_d_n5, eq18_e300_d_n6, eq18_e300_d_n7, eq18_e300_d_n8, eq18_e300_d_n9, eq18_e300_d_n10, eq18_e300_d_n11];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e300_d_b0, eq18_e300_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * s.v[263]);
        let eq19_e305_d_n0: f64 = (eq19_e303 * s.dn[263][0]);
        let eq19_e305_d_n1: f64 = (eq19_e303 * s.dn[263][1]);
        let eq19_e305_d_n2: f64 = (eq19_e303 * s.dn[263][2]);
        let eq19_e305_d_n3: f64 = (eq19_e303 * s.dn[263][3]);
        let eq19_e305_d_n4: f64 = (eq19_e303 * s.dn[263][4]);
        let eq19_e305_d_n5: f64 = (eq19_e303 * s.dn[263][5]);
        let eq19_e305_d_n6: f64 = (eq19_e303 * s.dn[263][6]);
        let eq19_e305_d_n7: f64 = (eq19_e303 * s.dn[263][7]);
        let eq19_e305_d_n8: f64 = (eq19_e303 * s.dn[263][8]);
        let eq19_e305_d_n9: f64 = (eq19_e303 * s.dn[263][9]);
        let eq19_e305_d_n10: f64 = (eq19_e303 * s.dn[263][10]);
        let eq19_e305_d_n11: f64 = (eq19_e303 * s.dn[263][11]);
        let eq19_e305_d_b0: f64 = (eq19_e303 * s.db[263][0]);
        let eq19_e305_d_b1: f64 = (eq19_e303 * s.db[263][1]);
        let eq19_e306: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq19_e305);
        let eq19_e306_d_n0: f64 = (eq19_e305_d_n0 * ddt_scale);
        let eq19_e306_d_n1: f64 = (eq19_e305_d_n1 * ddt_scale);
        let eq19_e306_d_n2: f64 = (eq19_e305_d_n2 * ddt_scale);
        let eq19_e306_d_n3: f64 = (eq19_e305_d_n3 * ddt_scale);
        let eq19_e306_d_n4: f64 = (eq19_e305_d_n4 * ddt_scale);
        let eq19_e306_d_n5: f64 = (eq19_e305_d_n5 * ddt_scale);
        let eq19_e306_d_n6: f64 = (eq19_e305_d_n6 * ddt_scale);
        let eq19_e306_d_n7: f64 = (eq19_e305_d_n7 * ddt_scale);
        let eq19_e306_d_n8: f64 = (eq19_e305_d_n8 * ddt_scale);
        let eq19_e306_d_n9: f64 = (eq19_e305_d_n9 * ddt_scale);
        let eq19_e306_d_n10: f64 = (eq19_e305_d_n10 * ddt_scale);
        let eq19_e306_d_n11: f64 = (eq19_e305_d_n11 * ddt_scale);
        let eq19_e306_d_b0: f64 = (eq19_e305_d_b0 * ddt_scale);
        let eq19_e306_d_b1: f64 = (eq19_e305_d_b1 * ddt_scale);
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
        let eq19_e308_d_n11: f64 = (eq19_e306_d_n11 * p.p1);
        let eq19_e308_d_b0: f64 = (eq19_e306_d_b0 * p.p1);
        let eq19_e308_d_b1: f64 = (eq19_e306_d_b1 * p.p1);
        let eq19_value: f64 = eq19_e308;
        let eq19_node_derivatives: [f64; 12] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10, eq19_e308_d_n11];
        let eq19_branch_derivatives: [f64; 2] = [eq19_e308_d_b0, eq19_e308_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * s.v[264]);
        let eq20_e313_d_n0: f64 = (eq20_e311 * s.dn[264][0]);
        let eq20_e313_d_n1: f64 = (eq20_e311 * s.dn[264][1]);
        let eq20_e313_d_n2: f64 = (eq20_e311 * s.dn[264][2]);
        let eq20_e313_d_n3: f64 = (eq20_e311 * s.dn[264][3]);
        let eq20_e313_d_n4: f64 = (eq20_e311 * s.dn[264][4]);
        let eq20_e313_d_n5: f64 = (eq20_e311 * s.dn[264][5]);
        let eq20_e313_d_n6: f64 = (eq20_e311 * s.dn[264][6]);
        let eq20_e313_d_n7: f64 = (eq20_e311 * s.dn[264][7]);
        let eq20_e313_d_n8: f64 = (eq20_e311 * s.dn[264][8]);
        let eq20_e313_d_n9: f64 = (eq20_e311 * s.dn[264][9]);
        let eq20_e313_d_n10: f64 = (eq20_e311 * s.dn[264][10]);
        let eq20_e313_d_n11: f64 = (eq20_e311 * s.dn[264][11]);
        let eq20_e313_d_b0: f64 = (eq20_e311 * s.db[264][0]);
        let eq20_e313_d_b1: f64 = (eq20_e311 * s.db[264][1]);
        let eq20_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq20_e313);
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
        let eq20_e314_d_n11: f64 = (eq20_e313_d_n11 * ddt_scale);
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
        let eq20_e316_d_n11: f64 = (eq20_e314_d_n11 * p.p1);
        let eq20_e316_d_b0: f64 = (eq20_e314_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e314_d_b1 * p.p1);
        let eq20_value: f64 = eq20_e316;
        let eq20_node_derivatives: [f64; 12] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10, eq20_e316_d_n11];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e319: f64 = (p.p3 * s.v[176]);
        let eq21_e319_d_n0: f64 = (p.p3 * s.dn[176][0]);
        let eq21_e319_d_n1: f64 = (p.p3 * s.dn[176][1]);
        let eq21_e319_d_n2: f64 = (p.p3 * s.dn[176][2]);
        let eq21_e319_d_n3: f64 = (p.p3 * s.dn[176][3]);
        let eq21_e319_d_n4: f64 = (p.p3 * s.dn[176][4]);
        let eq21_e319_d_n5: f64 = (p.p3 * s.dn[176][5]);
        let eq21_e319_d_n6: f64 = (p.p3 * s.dn[176][6]);
        let eq21_e319_d_n7: f64 = (p.p3 * s.dn[176][7]);
        let eq21_e319_d_n8: f64 = (p.p3 * s.dn[176][8]);
        let eq21_e319_d_n9: f64 = (p.p3 * s.dn[176][9]);
        let eq21_e319_d_n10: f64 = (p.p3 * s.dn[176][10]);
        let eq21_e319_d_n11: f64 = (p.p3 * s.dn[176][11]);
        let eq21_e319_d_b0: f64 = (p.p3 * s.db[176][0]);
        let eq21_e319_d_b1: f64 = (p.p3 * s.db[176][1]);
        let eq21_e321: f64 = (eq21_e319 * p.p1);
        let eq21_e321_d_n0: f64 = (eq21_e319_d_n0 * p.p1);
        let eq21_e321_d_n1: f64 = (eq21_e319_d_n1 * p.p1);
        let eq21_e321_d_n2: f64 = (eq21_e319_d_n2 * p.p1);
        let eq21_e321_d_n3: f64 = (eq21_e319_d_n3 * p.p1);
        let eq21_e321_d_n4: f64 = (eq21_e319_d_n4 * p.p1);
        let eq21_e321_d_n5: f64 = (eq21_e319_d_n5 * p.p1);
        let eq21_e321_d_n6: f64 = (eq21_e319_d_n6 * p.p1);
        let eq21_e321_d_n7: f64 = (eq21_e319_d_n7 * p.p1);
        let eq21_e321_d_n8: f64 = (eq21_e319_d_n8 * p.p1);
        let eq21_e321_d_n9: f64 = (eq21_e319_d_n9 * p.p1);
        let eq21_e321_d_n10: f64 = (eq21_e319_d_n10 * p.p1);
        let eq21_e321_d_n11: f64 = (eq21_e319_d_n11 * p.p1);
        let eq21_e321_d_b0: f64 = (eq21_e319_d_b0 * p.p1);
        let eq21_e321_d_b1: f64 = (eq21_e319_d_b1 * p.p1);
        let eq21_value: f64 = eq21_e321;
        let eq21_node_derivatives: [f64; 12] = [eq21_e321_d_n0, eq21_e321_d_n1, eq21_e321_d_n2, eq21_e321_d_n3, eq21_e321_d_n4, eq21_e321_d_n5, eq21_e321_d_n6, eq21_e321_d_n7, eq21_e321_d_n8, eq21_e321_d_n9, eq21_e321_d_n10, eq21_e321_d_n11];
        let eq21_branch_derivatives: [f64; 2] = [eq21_e321_d_b0, eq21_e321_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e324: f64 = (p.p3 * s.v[262]);
        let eq22_e324_d_n0: f64 = (p.p3 * s.dn[262][0]);
        let eq22_e324_d_n1: f64 = (p.p3 * s.dn[262][1]);
        let eq22_e324_d_n2: f64 = (p.p3 * s.dn[262][2]);
        let eq22_e324_d_n3: f64 = (p.p3 * s.dn[262][3]);
        let eq22_e324_d_n4: f64 = (p.p3 * s.dn[262][4]);
        let eq22_e324_d_n5: f64 = (p.p3 * s.dn[262][5]);
        let eq22_e324_d_n6: f64 = (p.p3 * s.dn[262][6]);
        let eq22_e324_d_n7: f64 = (p.p3 * s.dn[262][7]);
        let eq22_e324_d_n8: f64 = (p.p3 * s.dn[262][8]);
        let eq22_e324_d_n9: f64 = (p.p3 * s.dn[262][9]);
        let eq22_e324_d_n10: f64 = (p.p3 * s.dn[262][10]);
        let eq22_e324_d_n11: f64 = (p.p3 * s.dn[262][11]);
        let eq22_e324_d_b0: f64 = (p.p3 * s.db[262][0]);
        let eq22_e324_d_b1: f64 = (p.p3 * s.db[262][1]);
        let eq22_e326: f64 = (eq22_e324 * s.v[108]);
        let eq22_e326_d_n0: f64 = ((eq22_e324_d_n0 * s.v[108]) + (eq22_e324 * s.dn[108][0]));
        let eq22_e326_d_n1: f64 = ((eq22_e324_d_n1 * s.v[108]) + (eq22_e324 * s.dn[108][1]));
        let eq22_e326_d_n2: f64 = ((eq22_e324_d_n2 * s.v[108]) + (eq22_e324 * s.dn[108][2]));
        let eq22_e326_d_n3: f64 = ((eq22_e324_d_n3 * s.v[108]) + (eq22_e324 * s.dn[108][3]));
        let eq22_e326_d_n4: f64 = ((eq22_e324_d_n4 * s.v[108]) + (eq22_e324 * s.dn[108][4]));
        let eq22_e326_d_n5: f64 = ((eq22_e324_d_n5 * s.v[108]) + (eq22_e324 * s.dn[108][5]));
        let eq22_e326_d_n6: f64 = ((eq22_e324_d_n6 * s.v[108]) + (eq22_e324 * s.dn[108][6]));
        let eq22_e326_d_n7: f64 = ((eq22_e324_d_n7 * s.v[108]) + (eq22_e324 * s.dn[108][7]));
        let eq22_e326_d_n8: f64 = ((eq22_e324_d_n8 * s.v[108]) + (eq22_e324 * s.dn[108][8]));
        let eq22_e326_d_n9: f64 = ((eq22_e324_d_n9 * s.v[108]) + (eq22_e324 * s.dn[108][9]));
        let eq22_e326_d_n10: f64 = ((eq22_e324_d_n10 * s.v[108]) + (eq22_e324 * s.dn[108][10]));
        let eq22_e326_d_n11: f64 = ((eq22_e324_d_n11 * s.v[108]) + (eq22_e324 * s.dn[108][11]));
        let eq22_e326_d_b0: f64 = ((eq22_e324_d_b0 * s.v[108]) + (eq22_e324 * s.db[108][0]));
        let eq22_e326_d_b1: f64 = ((eq22_e324_d_b1 * s.v[108]) + (eq22_e324 * s.db[108][1]));
        let eq22_e328: f64 = (eq22_e326 * p.p1);
        let eq22_e328_d_n0: f64 = (eq22_e326_d_n0 * p.p1);
        let eq22_e328_d_n1: f64 = (eq22_e326_d_n1 * p.p1);
        let eq22_e328_d_n2: f64 = (eq22_e326_d_n2 * p.p1);
        let eq22_e328_d_n3: f64 = (eq22_e326_d_n3 * p.p1);
        let eq22_e328_d_n4: f64 = (eq22_e326_d_n4 * p.p1);
        let eq22_e328_d_n5: f64 = (eq22_e326_d_n5 * p.p1);
        let eq22_e328_d_n6: f64 = (eq22_e326_d_n6 * p.p1);
        let eq22_e328_d_n7: f64 = (eq22_e326_d_n7 * p.p1);
        let eq22_e328_d_n8: f64 = (eq22_e326_d_n8 * p.p1);
        let eq22_e328_d_n9: f64 = (eq22_e326_d_n9 * p.p1);
        let eq22_e328_d_n10: f64 = (eq22_e326_d_n10 * p.p1);
        let eq22_e328_d_n11: f64 = (eq22_e326_d_n11 * p.p1);
        let eq22_e328_d_b0: f64 = (eq22_e326_d_b0 * p.p1);
        let eq22_e328_d_b1: f64 = (eq22_e326_d_b1 * p.p1);
        let eq22_value: f64 = eq22_e328;
        let eq22_node_derivatives: [f64; 12] = [eq22_e328_d_n0, eq22_e328_d_n1, eq22_e328_d_n2, eq22_e328_d_n3, eq22_e328_d_n4, eq22_e328_d_n5, eq22_e328_d_n6, eq22_e328_d_n7, eq22_e328_d_n8, eq22_e328_d_n9, eq22_e328_d_n10, eq22_e328_d_n11];
        let eq22_branch_derivatives: [f64; 2] = [eq22_e328_d_b0, eq22_e328_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e332: f64 = (s.v[230] + s.v[242]);
        let eq23_e332_d_n0: f64 = (s.dn[230][0] + s.dn[242][0]);
        let eq23_e332_d_n1: f64 = (s.dn[230][1] + s.dn[242][1]);
        let eq23_e332_d_n2: f64 = (s.dn[230][2] + s.dn[242][2]);
        let eq23_e332_d_n3: f64 = (s.dn[230][3] + s.dn[242][3]);
        let eq23_e332_d_n4: f64 = (s.dn[230][4] + s.dn[242][4]);
        let eq23_e332_d_n5: f64 = (s.dn[230][5] + s.dn[242][5]);
        let eq23_e332_d_n6: f64 = (s.dn[230][6] + s.dn[242][6]);
        let eq23_e332_d_n7: f64 = (s.dn[230][7] + s.dn[242][7]);
        let eq23_e332_d_n8: f64 = (s.dn[230][8] + s.dn[242][8]);
        let eq23_e332_d_n9: f64 = (s.dn[230][9] + s.dn[242][9]);
        let eq23_e332_d_n10: f64 = (s.dn[230][10] + s.dn[242][10]);
        let eq23_e332_d_n11: f64 = (s.dn[230][11] + s.dn[242][11]);
        let eq23_e332_d_b0: f64 = (s.db[230][0] + s.db[242][0]);
        let eq23_e332_d_b1: f64 = (s.db[230][1] + s.db[242][1]);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n2: f64 = (p.p3 * eq23_e332_d_n2);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e333_d_n11: f64 = (p.p3 * eq23_e332_d_n11);
        let eq23_e333_d_b0: f64 = (p.p3 * eq23_e332_d_b0);
        let eq23_e333_d_b1: f64 = (p.p3 * eq23_e332_d_b1);
        let eq23_e334: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq23_e333);
        let eq23_e334_d_n0: f64 = (eq23_e333_d_n0 * ddt_scale);
        let eq23_e334_d_n1: f64 = (eq23_e333_d_n1 * ddt_scale);
        let eq23_e334_d_n2: f64 = (eq23_e333_d_n2 * ddt_scale);
        let eq23_e334_d_n3: f64 = (eq23_e333_d_n3 * ddt_scale);
        let eq23_e334_d_n4: f64 = (eq23_e333_d_n4 * ddt_scale);
        let eq23_e334_d_n5: f64 = (eq23_e333_d_n5 * ddt_scale);
        let eq23_e334_d_n6: f64 = (eq23_e333_d_n6 * ddt_scale);
        let eq23_e334_d_n7: f64 = (eq23_e333_d_n7 * ddt_scale);
        let eq23_e334_d_n8: f64 = (eq23_e333_d_n8 * ddt_scale);
        let eq23_e334_d_n9: f64 = (eq23_e333_d_n9 * ddt_scale);
        let eq23_e334_d_n10: f64 = (eq23_e333_d_n10 * ddt_scale);
        let eq23_e334_d_n11: f64 = (eq23_e333_d_n11 * ddt_scale);
        let eq23_e334_d_b0: f64 = (eq23_e333_d_b0 * ddt_scale);
        let eq23_e334_d_b1: f64 = (eq23_e333_d_b1 * ddt_scale);
        let eq23_e336: f64 = (eq23_e334 * p.p1);
        let eq23_e336_d_n0: f64 = (eq23_e334_d_n0 * p.p1);
        let eq23_e336_d_n1: f64 = (eq23_e334_d_n1 * p.p1);
        let eq23_e336_d_n2: f64 = (eq23_e334_d_n2 * p.p1);
        let eq23_e336_d_n3: f64 = (eq23_e334_d_n3 * p.p1);
        let eq23_e336_d_n4: f64 = (eq23_e334_d_n4 * p.p1);
        let eq23_e336_d_n5: f64 = (eq23_e334_d_n5 * p.p1);
        let eq23_e336_d_n6: f64 = (eq23_e334_d_n6 * p.p1);
        let eq23_e336_d_n7: f64 = (eq23_e334_d_n7 * p.p1);
        let eq23_e336_d_n8: f64 = (eq23_e334_d_n8 * p.p1);
        let eq23_e336_d_n9: f64 = (eq23_e334_d_n9 * p.p1);
        let eq23_e336_d_n10: f64 = (eq23_e334_d_n10 * p.p1);
        let eq23_e336_d_n11: f64 = (eq23_e334_d_n11 * p.p1);
        let eq23_e336_d_b0: f64 = (eq23_e334_d_b0 * p.p1);
        let eq23_e336_d_b1: f64 = (eq23_e334_d_b1 * p.p1);
        let eq23_value: f64 = eq23_e336;
        let eq23_node_derivatives: [f64; 12] = [eq23_e336_d_n0, eq23_e336_d_n1, eq23_e336_d_n2, eq23_e336_d_n3, eq23_e336_d_n4, eq23_e336_d_n5, eq23_e336_d_n6, eq23_e336_d_n7, eq23_e336_d_n8, eq23_e336_d_n9, eq23_e336_d_n10, eq23_e336_d_n11];
        let eq23_branch_derivatives: [f64; 2] = [eq23_e336_d_b0, eq23_e336_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
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
        let eq24_e341: f64 = (s.v[338] * s.v[249]);
        let eq24_e341_d_n0: f64 = (s.v[338] * s.dn[249][0]);
        let eq24_e341_d_n1: f64 = (s.v[338] * s.dn[249][1]);
        let eq24_e341_d_n2: f64 = (s.v[338] * s.dn[249][2]);
        let eq24_e341_d_n3: f64 = (s.v[338] * s.dn[249][3]);
        let eq24_e341_d_n4: f64 = (s.v[338] * s.dn[249][4]);
        let eq24_e341_d_n5: f64 = (s.v[338] * s.dn[249][5]);
        let eq24_e341_d_n6: f64 = (s.v[338] * s.dn[249][6]);
        let eq24_e341_d_n7: f64 = (s.v[338] * s.dn[249][7]);
        let eq24_e341_d_n8: f64 = (s.v[338] * s.dn[249][8]);
        let eq24_e341_d_n9: f64 = (s.v[338] * s.dn[249][9]);
        let eq24_e341_d_n10: f64 = (s.v[338] * s.dn[249][10]);
        let eq24_e341_d_n11: f64 = (s.v[338] * s.dn[249][11]);
        let eq24_e341_d_b0: f64 = (s.v[338] * s.db[249][0]);
        let eq24_e341_d_b1: f64 = (s.v[338] * s.db[249][1]);
        let eq24_e342: f64 = (s.v[161] + eq24_e341);
        let eq24_e342_d_n0: f64 = (s.dn[161][0] + eq24_e341_d_n0);
        let eq24_e342_d_n1: f64 = (s.dn[161][1] + eq24_e341_d_n1);
        let eq24_e342_d_n2: f64 = (s.dn[161][2] + eq24_e341_d_n2);
        let eq24_e342_d_n3: f64 = (s.dn[161][3] + eq24_e341_d_n3);
        let eq24_e342_d_n4: f64 = (s.dn[161][4] + eq24_e341_d_n4);
        let eq24_e342_d_n5: f64 = (s.dn[161][5] + eq24_e341_d_n5);
        let eq24_e342_d_n6: f64 = (s.dn[161][6] + eq24_e341_d_n6);
        let eq24_e342_d_n7: f64 = (s.dn[161][7] + eq24_e341_d_n7);
        let eq24_e342_d_n8: f64 = (s.dn[161][8] + eq24_e341_d_n8);
        let eq24_e342_d_n9: f64 = (s.dn[161][9] + eq24_e341_d_n9);
        let eq24_e342_d_n10: f64 = (s.dn[161][10] + eq24_e341_d_n10);
        let eq24_e342_d_n11: f64 = (s.dn[161][11] + eq24_e341_d_n11);
        let eq24_e342_d_b0: f64 = (s.db[161][0] + eq24_e341_d_b0);
        let eq24_e342_d_b1: f64 = (s.db[161][1] + eq24_e341_d_b1);
        let eq24_e344: f64 = (eq24_e342 + s.v[164]);
        let eq24_e344_d_n0: f64 = (eq24_e342_d_n0 + s.dn[164][0]);
        let eq24_e344_d_n1: f64 = (eq24_e342_d_n1 + s.dn[164][1]);
        let eq24_e344_d_n2: f64 = (eq24_e342_d_n2 + s.dn[164][2]);
        let eq24_e344_d_n3: f64 = (eq24_e342_d_n3 + s.dn[164][3]);
        let eq24_e344_d_n4: f64 = (eq24_e342_d_n4 + s.dn[164][4]);
        let eq24_e344_d_n5: f64 = (eq24_e342_d_n5 + s.dn[164][5]);
        let eq24_e344_d_n6: f64 = (eq24_e342_d_n6 + s.dn[164][6]);
        let eq24_e344_d_n7: f64 = (eq24_e342_d_n7 + s.dn[164][7]);
        let eq24_e344_d_n8: f64 = (eq24_e342_d_n8 + s.dn[164][8]);
        let eq24_e344_d_n9: f64 = (eq24_e342_d_n9 + s.dn[164][9]);
        let eq24_e344_d_n10: f64 = (eq24_e342_d_n10 + s.dn[164][10]);
        let eq24_e344_d_n11: f64 = (eq24_e342_d_n11 + s.dn[164][11]);
        let eq24_e344_d_b0: f64 = (eq24_e342_d_b0 + s.db[164][0]);
        let eq24_e344_d_b1: f64 = (eq24_e342_d_b1 + s.db[164][1]);
        let eq24_e345: f64 = (p.p3 * eq24_e344);
        let eq24_e345_d_n0: f64 = (p.p3 * eq24_e344_d_n0);
        let eq24_e345_d_n1: f64 = (p.p3 * eq24_e344_d_n1);
        let eq24_e345_d_n2: f64 = (p.p3 * eq24_e344_d_n2);
        let eq24_e345_d_n3: f64 = (p.p3 * eq24_e344_d_n3);
        let eq24_e345_d_n4: f64 = (p.p3 * eq24_e344_d_n4);
        let eq24_e345_d_n5: f64 = (p.p3 * eq24_e344_d_n5);
        let eq24_e345_d_n6: f64 = (p.p3 * eq24_e344_d_n6);
        let eq24_e345_d_n7: f64 = (p.p3 * eq24_e344_d_n7);
        let eq24_e345_d_n8: f64 = (p.p3 * eq24_e344_d_n8);
        let eq24_e345_d_n9: f64 = (p.p3 * eq24_e344_d_n9);
        let eq24_e345_d_n10: f64 = (p.p3 * eq24_e344_d_n10);
        let eq24_e345_d_n11: f64 = (p.p3 * eq24_e344_d_n11);
        let eq24_e345_d_b0: f64 = (p.p3 * eq24_e344_d_b0);
        let eq24_e345_d_b1: f64 = (p.p3 * eq24_e344_d_b1);
        let eq24_e347: f64 = (eq24_e345 * p.p1);
        let eq24_e347_d_n0: f64 = (eq24_e345_d_n0 * p.p1);
        let eq24_e347_d_n1: f64 = (eq24_e345_d_n1 * p.p1);
        let eq24_e347_d_n2: f64 = (eq24_e345_d_n2 * p.p1);
        let eq24_e347_d_n3: f64 = (eq24_e345_d_n3 * p.p1);
        let eq24_e347_d_n4: f64 = (eq24_e345_d_n4 * p.p1);
        let eq24_e347_d_n5: f64 = (eq24_e345_d_n5 * p.p1);
        let eq24_e347_d_n6: f64 = (eq24_e345_d_n6 * p.p1);
        let eq24_e347_d_n7: f64 = (eq24_e345_d_n7 * p.p1);
        let eq24_e347_d_n8: f64 = (eq24_e345_d_n8 * p.p1);
        let eq24_e347_d_n9: f64 = (eq24_e345_d_n9 * p.p1);
        let eq24_e347_d_n10: f64 = (eq24_e345_d_n10 * p.p1);
        let eq24_e347_d_n11: f64 = (eq24_e345_d_n11 * p.p1);
        let eq24_e347_d_b0: f64 = (eq24_e345_d_b0 * p.p1);
        let eq24_e347_d_b1: f64 = (eq24_e345_d_b1 * p.p1);
        let eq24_value: f64 = eq24_e347;
        let eq24_node_derivatives: [f64; 12] = [eq24_e347_d_n0, eq24_e347_d_n1, eq24_e347_d_n2, eq24_e347_d_n3, eq24_e347_d_n4, eq24_e347_d_n5, eq24_e347_d_n6, eq24_e347_d_n7, eq24_e347_d_n8, eq24_e347_d_n9, eq24_e347_d_n10, eq24_e347_d_n11];
        let eq24_branch_derivatives: [f64; 2] = [eq24_e347_d_b0, eq24_e347_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e351: f64 = (s.v[227] + s.v[243]);
        let eq25_e351_d_n0: f64 = (s.dn[227][0] + s.dn[243][0]);
        let eq25_e351_d_n1: f64 = (s.dn[227][1] + s.dn[243][1]);
        let eq25_e351_d_n2: f64 = (s.dn[227][2] + s.dn[243][2]);
        let eq25_e351_d_n3: f64 = (s.dn[227][3] + s.dn[243][3]);
        let eq25_e351_d_n4: f64 = (s.dn[227][4] + s.dn[243][4]);
        let eq25_e351_d_n5: f64 = (s.dn[227][5] + s.dn[243][5]);
        let eq25_e351_d_n6: f64 = (s.dn[227][6] + s.dn[243][6]);
        let eq25_e351_d_n7: f64 = (s.dn[227][7] + s.dn[243][7]);
        let eq25_e351_d_n8: f64 = (s.dn[227][8] + s.dn[243][8]);
        let eq25_e351_d_n9: f64 = (s.dn[227][9] + s.dn[243][9]);
        let eq25_e351_d_n10: f64 = (s.dn[227][10] + s.dn[243][10]);
        let eq25_e351_d_n11: f64 = (s.dn[227][11] + s.dn[243][11]);
        let eq25_e351_d_b0: f64 = (s.db[227][0] + s.db[243][0]);
        let eq25_e351_d_b1: f64 = (s.db[227][1] + s.db[243][1]);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n2: f64 = (p.p3 * eq25_e351_d_n2);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e352_d_n11: f64 = (p.p3 * eq25_e351_d_n11);
        let eq25_e352_d_b0: f64 = (p.p3 * eq25_e351_d_b0);
        let eq25_e352_d_b1: f64 = (p.p3 * eq25_e351_d_b1);
        let eq25_e353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq25_e352);
        let eq25_e353_d_n0: f64 = (eq25_e352_d_n0 * ddt_scale);
        let eq25_e353_d_n1: f64 = (eq25_e352_d_n1 * ddt_scale);
        let eq25_e353_d_n2: f64 = (eq25_e352_d_n2 * ddt_scale);
        let eq25_e353_d_n3: f64 = (eq25_e352_d_n3 * ddt_scale);
        let eq25_e353_d_n4: f64 = (eq25_e352_d_n4 * ddt_scale);
        let eq25_e353_d_n5: f64 = (eq25_e352_d_n5 * ddt_scale);
        let eq25_e353_d_n6: f64 = (eq25_e352_d_n6 * ddt_scale);
        let eq25_e353_d_n7: f64 = (eq25_e352_d_n7 * ddt_scale);
        let eq25_e353_d_n8: f64 = (eq25_e352_d_n8 * ddt_scale);
        let eq25_e353_d_n9: f64 = (eq25_e352_d_n9 * ddt_scale);
        let eq25_e353_d_n10: f64 = (eq25_e352_d_n10 * ddt_scale);
        let eq25_e353_d_n11: f64 = (eq25_e352_d_n11 * ddt_scale);
        let eq25_e353_d_b0: f64 = (eq25_e352_d_b0 * ddt_scale);
        let eq25_e353_d_b1: f64 = (eq25_e352_d_b1 * ddt_scale);
        let eq25_e355: f64 = (eq25_e353 * p.p1);
        let eq25_e355_d_n0: f64 = (eq25_e353_d_n0 * p.p1);
        let eq25_e355_d_n1: f64 = (eq25_e353_d_n1 * p.p1);
        let eq25_e355_d_n2: f64 = (eq25_e353_d_n2 * p.p1);
        let eq25_e355_d_n3: f64 = (eq25_e353_d_n3 * p.p1);
        let eq25_e355_d_n4: f64 = (eq25_e353_d_n4 * p.p1);
        let eq25_e355_d_n5: f64 = (eq25_e353_d_n5 * p.p1);
        let eq25_e355_d_n6: f64 = (eq25_e353_d_n6 * p.p1);
        let eq25_e355_d_n7: f64 = (eq25_e353_d_n7 * p.p1);
        let eq25_e355_d_n8: f64 = (eq25_e353_d_n8 * p.p1);
        let eq25_e355_d_n9: f64 = (eq25_e353_d_n9 * p.p1);
        let eq25_e355_d_n10: f64 = (eq25_e353_d_n10 * p.p1);
        let eq25_e355_d_n11: f64 = (eq25_e353_d_n11 * p.p1);
        let eq25_e355_d_b0: f64 = (eq25_e353_d_b0 * p.p1);
        let eq25_e355_d_b1: f64 = (eq25_e353_d_b1 * p.p1);
        let eq25_value: f64 = eq25_e355;
        let eq25_node_derivatives: [f64; 12] = [eq25_e355_d_n0, eq25_e355_d_n1, eq25_e355_d_n2, eq25_e355_d_n3, eq25_e355_d_n4, eq25_e355_d_n5, eq25_e355_d_n6, eq25_e355_d_n7, eq25_e355_d_n8, eq25_e355_d_n9, eq25_e355_d_n10, eq25_e355_d_n11];
        let eq25_branch_derivatives: [f64; 2] = [eq25_e355_d_b0, eq25_e355_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(10),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e365, eq26_e365_d_n0, eq26_e365_d_n1, eq26_e365_d_n2, eq26_e365_d_n3, eq26_e365_d_n4, eq26_e365_d_n5, eq26_e365_d_n6, eq26_e365_d_n7, eq26_e365_d_n8, eq26_e365_d_n9, eq26_e365_d_n10, eq26_e365_d_n11, eq26_e365_d_b0, eq26_e365_d_b1,) = {
    if s.b[598] {
        let eq26_e359: f64 = (p.p3 * s.v[251]);
        let eq26_e359_d_n0: f64 = (p.p3 * s.dn[251][0]);
        let eq26_e359_d_n1: f64 = (p.p3 * s.dn[251][1]);
        let eq26_e359_d_n2: f64 = (p.p3 * s.dn[251][2]);
        let eq26_e359_d_n3: f64 = (p.p3 * s.dn[251][3]);
        let eq26_e359_d_n4: f64 = (p.p3 * s.dn[251][4]);
        let eq26_e359_d_n5: f64 = (p.p3 * s.dn[251][5]);
        let eq26_e359_d_n6: f64 = (p.p3 * s.dn[251][6]);
        let eq26_e359_d_n7: f64 = (p.p3 * s.dn[251][7]);
        let eq26_e359_d_n8: f64 = (p.p3 * s.dn[251][8]);
        let eq26_e359_d_n9: f64 = (p.p3 * s.dn[251][9]);
        let eq26_e359_d_n10: f64 = (p.p3 * s.dn[251][10]);
        let eq26_e359_d_n11: f64 = (p.p3 * s.dn[251][11]);
        let eq26_e359_d_b0: f64 = (p.p3 * s.db[251][0]);
        let eq26_e359_d_b1: f64 = (p.p3 * s.db[251][1]);
        let eq26_e361: f64 = (eq26_e359 * s.v[109]);
        let eq26_e361_d_n0: f64 = ((eq26_e359_d_n0 * s.v[109]) + (eq26_e359 * s.dn[109][0]));
        let eq26_e361_d_n1: f64 = ((eq26_e359_d_n1 * s.v[109]) + (eq26_e359 * s.dn[109][1]));
        let eq26_e361_d_n2: f64 = ((eq26_e359_d_n2 * s.v[109]) + (eq26_e359 * s.dn[109][2]));
        let eq26_e361_d_n3: f64 = ((eq26_e359_d_n3 * s.v[109]) + (eq26_e359 * s.dn[109][3]));
        let eq26_e361_d_n4: f64 = ((eq26_e359_d_n4 * s.v[109]) + (eq26_e359 * s.dn[109][4]));
        let eq26_e361_d_n5: f64 = ((eq26_e359_d_n5 * s.v[109]) + (eq26_e359 * s.dn[109][5]));
        let eq26_e361_d_n6: f64 = ((eq26_e359_d_n6 * s.v[109]) + (eq26_e359 * s.dn[109][6]));
        let eq26_e361_d_n7: f64 = ((eq26_e359_d_n7 * s.v[109]) + (eq26_e359 * s.dn[109][7]));
        let eq26_e361_d_n8: f64 = ((eq26_e359_d_n8 * s.v[109]) + (eq26_e359 * s.dn[109][8]));
        let eq26_e361_d_n9: f64 = ((eq26_e359_d_n9 * s.v[109]) + (eq26_e359 * s.dn[109][9]));
        let eq26_e361_d_n10: f64 = ((eq26_e359_d_n10 * s.v[109]) + (eq26_e359 * s.dn[109][10]));
        let eq26_e361_d_n11: f64 = ((eq26_e359_d_n11 * s.v[109]) + (eq26_e359 * s.dn[109][11]));
        let eq26_e361_d_b0: f64 = ((eq26_e359_d_b0 * s.v[109]) + (eq26_e359 * s.db[109][0]));
        let eq26_e361_d_b1: f64 = ((eq26_e359_d_b1 * s.v[109]) + (eq26_e359 * s.db[109][1]));
        let eq26_e363: f64 = (eq26_e361 * p.p1);
        let eq26_e363_d_n0: f64 = (eq26_e361_d_n0 * p.p1);
        let eq26_e363_d_n1: f64 = (eq26_e361_d_n1 * p.p1);
        let eq26_e363_d_n2: f64 = (eq26_e361_d_n2 * p.p1);
        let eq26_e363_d_n3: f64 = (eq26_e361_d_n3 * p.p1);
        let eq26_e363_d_n4: f64 = (eq26_e361_d_n4 * p.p1);
        let eq26_e363_d_n5: f64 = (eq26_e361_d_n5 * p.p1);
        let eq26_e363_d_n6: f64 = (eq26_e361_d_n6 * p.p1);
        let eq26_e363_d_n7: f64 = (eq26_e361_d_n7 * p.p1);
        let eq26_e363_d_n8: f64 = (eq26_e361_d_n8 * p.p1);
        let eq26_e363_d_n9: f64 = (eq26_e361_d_n9 * p.p1);
        let eq26_e363_d_n10: f64 = (eq26_e361_d_n10 * p.p1);
        let eq26_e363_d_n11: f64 = (eq26_e361_d_n11 * p.p1);
        let eq26_e363_d_b0: f64 = (eq26_e361_d_b0 * p.p1);
        let eq26_e363_d_b1: f64 = (eq26_e361_d_b1 * p.p1);
        (eq26_e363, eq26_e363_d_n0, eq26_e363_d_n1, eq26_e363_d_n2, eq26_e363_d_n3, eq26_e363_d_n4, eq26_e363_d_n5, eq26_e363_d_n6, eq26_e363_d_n7, eq26_e363_d_n8, eq26_e363_d_n9, eq26_e363_d_n10, eq26_e363_d_n11, eq26_e363_d_b0, eq26_e363_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e365;
        let eq26_node_derivatives: [f64; 12] = [eq26_e365_d_n0, eq26_e365_d_n1, eq26_e365_d_n2, eq26_e365_d_n3, eq26_e365_d_n4, eq26_e365_d_n5, eq26_e365_d_n6, eq26_e365_d_n7, eq26_e365_d_n8, eq26_e365_d_n9, eq26_e365_d_n10, eq26_e365_d_n11];
        let eq26_branch_derivatives: [f64; 2] = [eq26_e365_d_b0, eq26_e365_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e370,) = {
    if (!s.b[598]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e370;
        stamper.stamp_potential_const_local(
            0,
            eq27_value,
        );
        let (eq28_e380, eq28_e380_d_n0, eq28_e380_d_n1, eq28_e380_d_n2, eq28_e380_d_n3, eq28_e380_d_n4, eq28_e380_d_n5, eq28_e380_d_n6, eq28_e380_d_n7, eq28_e380_d_n8, eq28_e380_d_n9, eq28_e380_d_n10, eq28_e380_d_n11, eq28_e380_d_b0, eq28_e380_d_b1,) = {
    if s.b[599] {
        let eq28_e374: f64 = (p.p3 * s.v[252]);
        let eq28_e374_d_n0: f64 = (p.p3 * s.dn[252][0]);
        let eq28_e374_d_n1: f64 = (p.p3 * s.dn[252][1]);
        let eq28_e374_d_n2: f64 = (p.p3 * s.dn[252][2]);
        let eq28_e374_d_n3: f64 = (p.p3 * s.dn[252][3]);
        let eq28_e374_d_n4: f64 = (p.p3 * s.dn[252][4]);
        let eq28_e374_d_n5: f64 = (p.p3 * s.dn[252][5]);
        let eq28_e374_d_n6: f64 = (p.p3 * s.dn[252][6]);
        let eq28_e374_d_n7: f64 = (p.p3 * s.dn[252][7]);
        let eq28_e374_d_n8: f64 = (p.p3 * s.dn[252][8]);
        let eq28_e374_d_n9: f64 = (p.p3 * s.dn[252][9]);
        let eq28_e374_d_n10: f64 = (p.p3 * s.dn[252][10]);
        let eq28_e374_d_n11: f64 = (p.p3 * s.dn[252][11]);
        let eq28_e374_d_b0: f64 = (p.p3 * s.db[252][0]);
        let eq28_e374_d_b1: f64 = (p.p3 * s.db[252][1]);
        let eq28_e376: f64 = (eq28_e374 * s.v[110]);
        let eq28_e376_d_n0: f64 = ((eq28_e374_d_n0 * s.v[110]) + (eq28_e374 * s.dn[110][0]));
        let eq28_e376_d_n1: f64 = ((eq28_e374_d_n1 * s.v[110]) + (eq28_e374 * s.dn[110][1]));
        let eq28_e376_d_n2: f64 = ((eq28_e374_d_n2 * s.v[110]) + (eq28_e374 * s.dn[110][2]));
        let eq28_e376_d_n3: f64 = ((eq28_e374_d_n3 * s.v[110]) + (eq28_e374 * s.dn[110][3]));
        let eq28_e376_d_n4: f64 = ((eq28_e374_d_n4 * s.v[110]) + (eq28_e374 * s.dn[110][4]));
        let eq28_e376_d_n5: f64 = ((eq28_e374_d_n5 * s.v[110]) + (eq28_e374 * s.dn[110][5]));
        let eq28_e376_d_n6: f64 = ((eq28_e374_d_n6 * s.v[110]) + (eq28_e374 * s.dn[110][6]));
        let eq28_e376_d_n7: f64 = ((eq28_e374_d_n7 * s.v[110]) + (eq28_e374 * s.dn[110][7]));
        let eq28_e376_d_n8: f64 = ((eq28_e374_d_n8 * s.v[110]) + (eq28_e374 * s.dn[110][8]));
        let eq28_e376_d_n9: f64 = ((eq28_e374_d_n9 * s.v[110]) + (eq28_e374 * s.dn[110][9]));
        let eq28_e376_d_n10: f64 = ((eq28_e374_d_n10 * s.v[110]) + (eq28_e374 * s.dn[110][10]));
        let eq28_e376_d_n11: f64 = ((eq28_e374_d_n11 * s.v[110]) + (eq28_e374 * s.dn[110][11]));
        let eq28_e376_d_b0: f64 = ((eq28_e374_d_b0 * s.v[110]) + (eq28_e374 * s.db[110][0]));
        let eq28_e376_d_b1: f64 = ((eq28_e374_d_b1 * s.v[110]) + (eq28_e374 * s.db[110][1]));
        let eq28_e378: f64 = (eq28_e376 * p.p1);
        let eq28_e378_d_n0: f64 = (eq28_e376_d_n0 * p.p1);
        let eq28_e378_d_n1: f64 = (eq28_e376_d_n1 * p.p1);
        let eq28_e378_d_n2: f64 = (eq28_e376_d_n2 * p.p1);
        let eq28_e378_d_n3: f64 = (eq28_e376_d_n3 * p.p1);
        let eq28_e378_d_n4: f64 = (eq28_e376_d_n4 * p.p1);
        let eq28_e378_d_n5: f64 = (eq28_e376_d_n5 * p.p1);
        let eq28_e378_d_n6: f64 = (eq28_e376_d_n6 * p.p1);
        let eq28_e378_d_n7: f64 = (eq28_e376_d_n7 * p.p1);
        let eq28_e378_d_n8: f64 = (eq28_e376_d_n8 * p.p1);
        let eq28_e378_d_n9: f64 = (eq28_e376_d_n9 * p.p1);
        let eq28_e378_d_n10: f64 = (eq28_e376_d_n10 * p.p1);
        let eq28_e378_d_n11: f64 = (eq28_e376_d_n11 * p.p1);
        let eq28_e378_d_b0: f64 = (eq28_e376_d_b0 * p.p1);
        let eq28_e378_d_b1: f64 = (eq28_e376_d_b1 * p.p1);
        (eq28_e378, eq28_e378_d_n0, eq28_e378_d_n1, eq28_e378_d_n2, eq28_e378_d_n3, eq28_e378_d_n4, eq28_e378_d_n5, eq28_e378_d_n6, eq28_e378_d_n7, eq28_e378_d_n8, eq28_e378_d_n9, eq28_e378_d_n10, eq28_e378_d_n11, eq28_e378_d_b0, eq28_e378_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e380;
        let eq28_node_derivatives: [f64; 12] = [eq28_e380_d_n0, eq28_e380_d_n1, eq28_e380_d_n2, eq28_e380_d_n3, eq28_e380_d_n4, eq28_e380_d_n5, eq28_e380_d_n6, eq28_e380_d_n7, eq28_e380_d_n8, eq28_e380_d_n9, eq28_e380_d_n10, eq28_e380_d_n11];
        let eq28_branch_derivatives: [f64; 2] = [eq28_e380_d_b0, eq28_e380_d_b1];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e385,) = {
    if (!s.b[599]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e385;
        stamper.stamp_potential_const_local(
            1,
            eq29_value,
        );
        let eq30_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (eq30_value),
        );
        let eq31_value: f64 = (nv11 - 0.0);
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq31_value),
            11,
            multiplicity * (1.0),
        );
        let eq32_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, (nv11 - 0.0));
        let eq32_e395: f64 = (s.v[330] * eq32_e394);
        let eq32_e395_d_n0: f64 = (s.dn[330][0] * eq32_e394);
        let eq32_e395_d_n1: f64 = (s.dn[330][1] * eq32_e394);
        let eq32_e395_d_n2: f64 = (s.dn[330][2] * eq32_e394);
        let eq32_e395_d_n3: f64 = (s.dn[330][3] * eq32_e394);
        let eq32_e395_d_n4: f64 = (s.dn[330][4] * eq32_e394);
        let eq32_e395_d_n5: f64 = (s.dn[330][5] * eq32_e394);
        let eq32_e395_d_n6: f64 = (s.dn[330][6] * eq32_e394);
        let eq32_e395_d_n7: f64 = (s.dn[330][7] * eq32_e394);
        let eq32_e395_d_n8: f64 = (s.dn[330][8] * eq32_e394);
        let eq32_e395_d_n9: f64 = (s.dn[330][9] * eq32_e394);
        let eq32_e395_d_n10: f64 = (s.dn[330][10] * eq32_e394);
        let eq32_e395_d_n11: f64 = ((s.dn[330][11] * eq32_e394) + (s.v[330] * ddt_scale));
        let eq32_e395_d_b0: f64 = (s.db[330][0] * eq32_e394);
        let eq32_e395_d_b1: f64 = (s.db[330][1] * eq32_e394);
        let eq32_value: f64 = eq32_e395;
        let eq32_node_derivatives: [f64; 12] = [eq32_e395_d_n0, eq32_e395_d_n1, eq32_e395_d_n2, eq32_e395_d_n3, eq32_e395_d_n4, eq32_e395_d_n5, eq32_e395_d_n6, eq32_e395_d_n7, eq32_e395_d_n8, eq32_e395_d_n9, eq32_e395_d_n10, eq32_e395_d_n11];
        let eq32_branch_derivatives: [f64; 2] = [eq32_e395_d_b0, eq32_e395_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e398: f64 = (s.v[328] * (nv11 - 0.0));
        let eq33_e398_d_n0: f64 = (s.dn[328][0] * (nv11 - 0.0));
        let eq33_e398_d_n1: f64 = (s.dn[328][1] * (nv11 - 0.0));
        let eq33_e398_d_n2: f64 = (s.dn[328][2] * (nv11 - 0.0));
        let eq33_e398_d_n3: f64 = (s.dn[328][3] * (nv11 - 0.0));
        let eq33_e398_d_n4: f64 = (s.dn[328][4] * (nv11 - 0.0));
        let eq33_e398_d_n5: f64 = (s.dn[328][5] * (nv11 - 0.0));
        let eq33_e398_d_n6: f64 = (s.dn[328][6] * (nv11 - 0.0));
        let eq33_e398_d_n7: f64 = (s.dn[328][7] * (nv11 - 0.0));
        let eq33_e398_d_n8: f64 = (s.dn[328][8] * (nv11 - 0.0));
        let eq33_e398_d_n9: f64 = (s.dn[328][9] * (nv11 - 0.0));
        let eq33_e398_d_n10: f64 = (s.dn[328][10] * (nv11 - 0.0));
        let eq33_e398_d_n11: f64 = ((s.dn[328][11] * (nv11 - 0.0)) + s.v[328]);
        let eq33_e398_d_b0: f64 = (s.db[328][0] * (nv11 - 0.0));
        let eq33_e398_d_b1: f64 = (s.db[328][1] * (nv11 - 0.0));
        let eq33_value: f64 = eq33_e398;
        let eq33_node_derivatives: [f64; 12] = [eq33_e398_d_n0, eq33_e398_d_n1, eq33_e398_d_n2, eq33_e398_d_n3, eq33_e398_d_n4, eq33_e398_d_n5, eq33_e398_d_n6, eq33_e398_d_n7, eq33_e398_d_n8, eq33_e398_d_n9, eq33_e398_d_n10, eq33_e398_d_n11];
        let eq33_branch_derivatives: [f64; 2] = [eq33_e398_d_b0, eq33_e398_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_value: f64 = (nv11 - 0.0);
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (eq34_value),
            11,
            multiplicity * (1.0),
        );
        let eq35_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
        );
        let eq36_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (eq36_value),
        );
        let eq37_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(2),
            Some(4),
            multiplicity * (eq37_value),
        );
        let eq38_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (eq38_value),
        );
        let eq39_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq39_value),
        );
        let eq40_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (eq40_value),
        );
        let eq41_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (eq41_value),
        );
        let eq42_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
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
            Some(5),
            Some(10),
            multiplicity * (eq45_value),
        );
        let eq46_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (eq46_value),
        );
        let eq47_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (eq47_value),
        );
        let eq48_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (eq48_value),
        );
        let (eq49_e482,) = {
    if s.b[610] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e482;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq49_value),
        );
        let (eq50_e491,) = {
    if (!s.b[610]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e491;
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (eq50_value),
        );
        let eq51_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(6),
            Some(3),
            multiplicity * (eq51_value),
        );
        let eq52_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (eq52_value),
        );
        let eq53_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(1),
            Some(3),
            multiplicity * (eq53_value),
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
    ) {
        let (eq54_e516,) = {
    if (s.b[611] && s.b[612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e516;
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (eq54_value),
        );
        let (eq55_e526,) = {
    if (s.b[611] && s.b[612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq55_value: f64 = eq55_e526;
        stamper.stamp_current_const_local(
            Some(9),
            Some(10),
            multiplicity * (eq55_value),
        );
        let (eq56_e536,) = {
    if (s.b[611] && s.b[612]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e536;
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (eq56_value),
        );
        let (eq57_e547,) = {
    if (s.b[611] && (!s.b[612])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq57_value: f64 = eq57_e547;
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (eq57_value),
        );
        let (eq58_e558,) = {
    if (s.b[611] && (!s.b[612])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq58_value: f64 = eq58_e558;
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (eq58_value),
        );
        let (eq59_e569,) = {
    if ((!s.b[611]) && s.b[613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e569;
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (eq59_value),
        );
        let (eq60_e580,) = {
    if ((!s.b[611]) && s.b[613]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e580;
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (eq60_value),
        );
        let (eq61_e592,) = {
    if ((!s.b[611]) && (!s.b[613])) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e592;
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (eq61_value),
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
        let eq14_e266: f64 = (s.v[215] + s.v[220]);
        let eq14_e266_d_n0: f64 = (s.dn[215][0] + s.dn[220][0]);
        let eq14_e266_d_n1: f64 = (s.dn[215][1] + s.dn[220][1]);
        let eq14_e266_d_n2: f64 = (s.dn[215][2] + s.dn[220][2]);
        let eq14_e266_d_n3: f64 = (s.dn[215][3] + s.dn[220][3]);
        let eq14_e266_d_n4: f64 = (s.dn[215][4] + s.dn[220][4]);
        let eq14_e266_d_n5: f64 = (s.dn[215][5] + s.dn[220][5]);
        let eq14_e266_d_n6: f64 = (s.dn[215][6] + s.dn[220][6]);
        let eq14_e266_d_n7: f64 = (s.dn[215][7] + s.dn[220][7]);
        let eq14_e266_d_n8: f64 = (s.dn[215][8] + s.dn[220][8]);
        let eq14_e266_d_n9: f64 = (s.dn[215][9] + s.dn[220][9]);
        let eq14_e266_d_n10: f64 = (s.dn[215][10] + s.dn[220][10]);
        let eq14_e266_d_n11: f64 = (s.dn[215][11] + s.dn[220][11]);
        let eq14_e266_d_b0: f64 = (s.db[215][0] + s.db[220][0]);
        let eq14_e266_d_b1: f64 = (s.db[215][1] + s.db[220][1]);
        let eq14_e268: f64 = (eq14_e266 + s.v[235]);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + s.dn[235][0]);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + s.dn[235][1]);
        let eq14_e268_d_n2: f64 = (eq14_e266_d_n2 + s.dn[235][2]);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + s.dn[235][3]);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + s.dn[235][4]);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + s.dn[235][5]);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + s.dn[235][6]);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + s.dn[235][7]);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + s.dn[235][8]);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + s.dn[235][9]);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + s.dn[235][10]);
        let eq14_e268_d_n11: f64 = (eq14_e266_d_n11 + s.dn[235][11]);
        let eq14_e268_d_b0: f64 = (eq14_e266_d_b0 + s.db[235][0]);
        let eq14_e268_d_b1: f64 = (eq14_e266_d_b1 + s.db[235][1]);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n2: f64 = (p.p3 * eq14_e268_d_n2);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e269_d_n11: f64 = (p.p3 * eq14_e268_d_n11);
        let eq14_e269_d_b0: f64 = (p.p3 * eq14_e268_d_b0);
        let eq14_e269_d_b1: f64 = (p.p3 * eq14_e268_d_b1);
        let eq14_e270_q: f64 = eq14_e269;
        let eq14_e272: f64 = (eq14_e269 * p.p1);
        let eq14_e272_d_n0: f64 = (eq14_e269_d_n0 * p.p1);
        let eq14_e272_d_n1: f64 = (eq14_e269_d_n1 * p.p1);
        let eq14_e272_d_n2: f64 = (eq14_e269_d_n2 * p.p1);
        let eq14_e272_d_n3: f64 = (eq14_e269_d_n3 * p.p1);
        let eq14_e272_d_n4: f64 = (eq14_e269_d_n4 * p.p1);
        let eq14_e272_d_n5: f64 = (eq14_e269_d_n5 * p.p1);
        let eq14_e272_d_n6: f64 = (eq14_e269_d_n6 * p.p1);
        let eq14_e272_d_n7: f64 = (eq14_e269_d_n7 * p.p1);
        let eq14_e272_d_n8: f64 = (eq14_e269_d_n8 * p.p1);
        let eq14_e272_d_n9: f64 = (eq14_e269_d_n9 * p.p1);
        let eq14_e272_d_n10: f64 = (eq14_e269_d_n10 * p.p1);
        let eq14_e272_d_n11: f64 = (eq14_e269_d_n11 * p.p1);
        let eq14_e272_d_b0: f64 = (eq14_e269_d_b0 * p.p1);
        let eq14_e272_d_b1: f64 = (eq14_e269_d_b1 * p.p1);
        let eq14_e272_q: f64 = (eq14_e270_q * p.p1);
        let eq14_e272_q_d_n0: f64 = (eq14_e269_d_n0 * p.p1);
        let eq14_e272_q_d_n1: f64 = (eq14_e269_d_n1 * p.p1);
        let eq14_e272_q_d_n2: f64 = (eq14_e269_d_n2 * p.p1);
        let eq14_e272_q_d_n3: f64 = (eq14_e269_d_n3 * p.p1);
        let eq14_e272_q_d_n4: f64 = (eq14_e269_d_n4 * p.p1);
        let eq14_e272_q_d_n5: f64 = (eq14_e269_d_n5 * p.p1);
        let eq14_e272_q_d_n6: f64 = (eq14_e269_d_n6 * p.p1);
        let eq14_e272_q_d_n7: f64 = (eq14_e269_d_n7 * p.p1);
        let eq14_e272_q_d_n8: f64 = (eq14_e269_d_n8 * p.p1);
        let eq14_e272_q_d_n9: f64 = (eq14_e269_d_n9 * p.p1);
        let eq14_e272_q_d_n10: f64 = (eq14_e269_d_n10 * p.p1);
        let eq14_e272_q_d_n11: f64 = (eq14_e269_d_n11 * p.p1);
        let eq14_e272_q_d_b0: f64 = (eq14_e269_d_b0 * p.p1);
        let eq14_e272_q_d_b1: f64 = (eq14_e269_d_b1 * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e272_q_d_n0, eq14_e272_q_d_n1, eq14_e272_q_d_n2, eq14_e272_q_d_n3, eq14_e272_q_d_n4, eq14_e272_q_d_n5, eq14_e272_q_d_n6, eq14_e272_q_d_n7, eq14_e272_q_d_n8, eq14_e272_q_d_n9, eq14_e272_q_d_n10, eq14_e272_q_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e272_q_d_b0, eq14_e272_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e275: f64 = (p.p3 * s.v[217]);
        let eq15_e275_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq15_e275_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq15_e275_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq15_e275_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq15_e275_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq15_e275_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq15_e275_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq15_e275_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq15_e275_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq15_e275_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq15_e275_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq15_e275_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq15_e275_d_b0: f64 = (p.p3 * s.db[217][0]);
        let eq15_e275_d_b1: f64 = (p.p3 * s.db[217][1]);
        let eq15_e276_q: f64 = eq15_e275;
        let eq15_e278: f64 = (eq15_e275 * p.p1);
        let eq15_e278_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e278_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e278_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e278_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e278_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e278_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e278_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e278_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e278_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e278_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e278_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e278_d_n11: f64 = (eq15_e275_d_n11 * p.p1);
        let eq15_e278_d_b0: f64 = (eq15_e275_d_b0 * p.p1);
        let eq15_e278_d_b1: f64 = (eq15_e275_d_b1 * p.p1);
        let eq15_e278_q: f64 = (eq15_e276_q * p.p1);
        let eq15_e278_q_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e278_q_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e278_q_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e278_q_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e278_q_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e278_q_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e278_q_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e278_q_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e278_q_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e278_q_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e278_q_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e278_q_d_n11: f64 = (eq15_e275_d_n11 * p.p1);
        let eq15_e278_q_d_b0: f64 = (eq15_e275_d_b0 * p.p1);
        let eq15_e278_q_d_b1: f64 = (eq15_e275_d_b1 * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e278_q_d_n0, eq15_e278_q_d_n1, eq15_e278_q_d_n2, eq15_e278_q_d_n3, eq15_e278_q_d_n4, eq15_e278_q_d_n5, eq15_e278_q_d_n6, eq15_e278_q_d_n7, eq15_e278_q_d_n8, eq15_e278_q_d_n9, eq15_e278_q_d_n10, eq15_e278_q_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e278_q_d_b0, eq15_e278_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e282: f64 = (s.v[218] + s.v[221]);
        let eq16_e282_d_n0: f64 = (s.dn[218][0] + s.dn[221][0]);
        let eq16_e282_d_n1: f64 = (s.dn[218][1] + s.dn[221][1]);
        let eq16_e282_d_n2: f64 = (s.dn[218][2] + s.dn[221][2]);
        let eq16_e282_d_n3: f64 = (s.dn[218][3] + s.dn[221][3]);
        let eq16_e282_d_n4: f64 = (s.dn[218][4] + s.dn[221][4]);
        let eq16_e282_d_n5: f64 = (s.dn[218][5] + s.dn[221][5]);
        let eq16_e282_d_n6: f64 = (s.dn[218][6] + s.dn[221][6]);
        let eq16_e282_d_n7: f64 = (s.dn[218][7] + s.dn[221][7]);
        let eq16_e282_d_n8: f64 = (s.dn[218][8] + s.dn[221][8]);
        let eq16_e282_d_n9: f64 = (s.dn[218][9] + s.dn[221][9]);
        let eq16_e282_d_n10: f64 = (s.dn[218][10] + s.dn[221][10]);
        let eq16_e282_d_n11: f64 = (s.dn[218][11] + s.dn[221][11]);
        let eq16_e282_d_b0: f64 = (s.db[218][0] + s.db[221][0]);
        let eq16_e282_d_b1: f64 = (s.db[218][1] + s.db[221][1]);
        let eq16_e284: f64 = (eq16_e282 + s.v[238]);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + s.dn[238][0]);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + s.dn[238][1]);
        let eq16_e284_d_n2: f64 = (eq16_e282_d_n2 + s.dn[238][2]);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + s.dn[238][3]);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + s.dn[238][4]);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + s.dn[238][5]);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + s.dn[238][6]);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + s.dn[238][7]);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + s.dn[238][8]);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + s.dn[238][9]);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + s.dn[238][10]);
        let eq16_e284_d_n11: f64 = (eq16_e282_d_n11 + s.dn[238][11]);
        let eq16_e284_d_b0: f64 = (eq16_e282_d_b0 + s.db[238][0]);
        let eq16_e284_d_b1: f64 = (eq16_e282_d_b1 + s.db[238][1]);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n2: f64 = (p.p3 * eq16_e284_d_n2);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e285_d_n11: f64 = (p.p3 * eq16_e284_d_n11);
        let eq16_e285_d_b0: f64 = (p.p3 * eq16_e284_d_b0);
        let eq16_e285_d_b1: f64 = (p.p3 * eq16_e284_d_b1);
        let eq16_e286_q: f64 = eq16_e285;
        let eq16_e288: f64 = (eq16_e285 * p.p1);
        let eq16_e288_d_n0: f64 = (eq16_e285_d_n0 * p.p1);
        let eq16_e288_d_n1: f64 = (eq16_e285_d_n1 * p.p1);
        let eq16_e288_d_n2: f64 = (eq16_e285_d_n2 * p.p1);
        let eq16_e288_d_n3: f64 = (eq16_e285_d_n3 * p.p1);
        let eq16_e288_d_n4: f64 = (eq16_e285_d_n4 * p.p1);
        let eq16_e288_d_n5: f64 = (eq16_e285_d_n5 * p.p1);
        let eq16_e288_d_n6: f64 = (eq16_e285_d_n6 * p.p1);
        let eq16_e288_d_n7: f64 = (eq16_e285_d_n7 * p.p1);
        let eq16_e288_d_n8: f64 = (eq16_e285_d_n8 * p.p1);
        let eq16_e288_d_n9: f64 = (eq16_e285_d_n9 * p.p1);
        let eq16_e288_d_n10: f64 = (eq16_e285_d_n10 * p.p1);
        let eq16_e288_d_n11: f64 = (eq16_e285_d_n11 * p.p1);
        let eq16_e288_d_b0: f64 = (eq16_e285_d_b0 * p.p1);
        let eq16_e288_d_b1: f64 = (eq16_e285_d_b1 * p.p1);
        let eq16_e288_q: f64 = (eq16_e286_q * p.p1);
        let eq16_e288_q_d_n0: f64 = (eq16_e285_d_n0 * p.p1);
        let eq16_e288_q_d_n1: f64 = (eq16_e285_d_n1 * p.p1);
        let eq16_e288_q_d_n2: f64 = (eq16_e285_d_n2 * p.p1);
        let eq16_e288_q_d_n3: f64 = (eq16_e285_d_n3 * p.p1);
        let eq16_e288_q_d_n4: f64 = (eq16_e285_d_n4 * p.p1);
        let eq16_e288_q_d_n5: f64 = (eq16_e285_d_n5 * p.p1);
        let eq16_e288_q_d_n6: f64 = (eq16_e285_d_n6 * p.p1);
        let eq16_e288_q_d_n7: f64 = (eq16_e285_d_n7 * p.p1);
        let eq16_e288_q_d_n8: f64 = (eq16_e285_d_n8 * p.p1);
        let eq16_e288_q_d_n9: f64 = (eq16_e285_d_n9 * p.p1);
        let eq16_e288_q_d_n10: f64 = (eq16_e285_d_n10 * p.p1);
        let eq16_e288_q_d_n11: f64 = (eq16_e285_d_n11 * p.p1);
        let eq16_e288_q_d_b0: f64 = (eq16_e285_d_b0 * p.p1);
        let eq16_e288_q_d_b1: f64 = (eq16_e285_d_b1 * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e288_q_d_n0, eq16_e288_q_d_n1, eq16_e288_q_d_n2, eq16_e288_q_d_n3, eq16_e288_q_d_n4, eq16_e288_q_d_n5, eq16_e288_q_d_n6, eq16_e288_q_d_n7, eq16_e288_q_d_n8, eq16_e288_q_d_n9, eq16_e288_q_d_n10, eq16_e288_q_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 2] = [eq16_e288_q_d_b0, eq16_e288_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e291: f64 = (p.p3 * s.v[233]);
        let eq17_e291_d_n0: f64 = (p.p3 * s.dn[233][0]);
        let eq17_e291_d_n1: f64 = (p.p3 * s.dn[233][1]);
        let eq17_e291_d_n2: f64 = (p.p3 * s.dn[233][2]);
        let eq17_e291_d_n3: f64 = (p.p3 * s.dn[233][3]);
        let eq17_e291_d_n4: f64 = (p.p3 * s.dn[233][4]);
        let eq17_e291_d_n5: f64 = (p.p3 * s.dn[233][5]);
        let eq17_e291_d_n6: f64 = (p.p3 * s.dn[233][6]);
        let eq17_e291_d_n7: f64 = (p.p3 * s.dn[233][7]);
        let eq17_e291_d_n8: f64 = (p.p3 * s.dn[233][8]);
        let eq17_e291_d_n9: f64 = (p.p3 * s.dn[233][9]);
        let eq17_e291_d_n10: f64 = (p.p3 * s.dn[233][10]);
        let eq17_e291_d_n11: f64 = (p.p3 * s.dn[233][11]);
        let eq17_e291_d_b0: f64 = (p.p3 * s.db[233][0]);
        let eq17_e291_d_b1: f64 = (p.p3 * s.db[233][1]);
        let eq17_e292_q: f64 = eq17_e291;
        let eq17_e294: f64 = (eq17_e291 * p.p1);
        let eq17_e294_d_n0: f64 = (eq17_e291_d_n0 * p.p1);
        let eq17_e294_d_n1: f64 = (eq17_e291_d_n1 * p.p1);
        let eq17_e294_d_n2: f64 = (eq17_e291_d_n2 * p.p1);
        let eq17_e294_d_n3: f64 = (eq17_e291_d_n3 * p.p1);
        let eq17_e294_d_n4: f64 = (eq17_e291_d_n4 * p.p1);
        let eq17_e294_d_n5: f64 = (eq17_e291_d_n5 * p.p1);
        let eq17_e294_d_n6: f64 = (eq17_e291_d_n6 * p.p1);
        let eq17_e294_d_n7: f64 = (eq17_e291_d_n7 * p.p1);
        let eq17_e294_d_n8: f64 = (eq17_e291_d_n8 * p.p1);
        let eq17_e294_d_n9: f64 = (eq17_e291_d_n9 * p.p1);
        let eq17_e294_d_n10: f64 = (eq17_e291_d_n10 * p.p1);
        let eq17_e294_d_n11: f64 = (eq17_e291_d_n11 * p.p1);
        let eq17_e294_d_b0: f64 = (eq17_e291_d_b0 * p.p1);
        let eq17_e294_d_b1: f64 = (eq17_e291_d_b1 * p.p1);
        let eq17_e294_q: f64 = (eq17_e292_q * p.p1);
        let eq17_e294_q_d_n0: f64 = (eq17_e291_d_n0 * p.p1);
        let eq17_e294_q_d_n1: f64 = (eq17_e291_d_n1 * p.p1);
        let eq17_e294_q_d_n2: f64 = (eq17_e291_d_n2 * p.p1);
        let eq17_e294_q_d_n3: f64 = (eq17_e291_d_n3 * p.p1);
        let eq17_e294_q_d_n4: f64 = (eq17_e291_d_n4 * p.p1);
        let eq17_e294_q_d_n5: f64 = (eq17_e291_d_n5 * p.p1);
        let eq17_e294_q_d_n6: f64 = (eq17_e291_d_n6 * p.p1);
        let eq17_e294_q_d_n7: f64 = (eq17_e291_d_n7 * p.p1);
        let eq17_e294_q_d_n8: f64 = (eq17_e291_d_n8 * p.p1);
        let eq17_e294_q_d_n9: f64 = (eq17_e291_d_n9 * p.p1);
        let eq17_e294_q_d_n10: f64 = (eq17_e291_d_n10 * p.p1);
        let eq17_e294_q_d_n11: f64 = (eq17_e291_d_n11 * p.p1);
        let eq17_e294_q_d_b0: f64 = (eq17_e291_d_b0 * p.p1);
        let eq17_e294_q_d_b1: f64 = (eq17_e291_d_b1 * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e294_q_d_n0, eq17_e294_q_d_n1, eq17_e294_q_d_n2, eq17_e294_q_d_n3, eq17_e294_q_d_n4, eq17_e294_q_d_n5, eq17_e294_q_d_n6, eq17_e294_q_d_n7, eq17_e294_q_d_n8, eq17_e294_q_d_n9, eq17_e294_q_d_n10, eq17_e294_q_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e294_q_d_b0, eq17_e294_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e297: f64 = (p.p3 * s.v[222]);
        let eq18_e297_d_n0: f64 = (p.p3 * s.dn[222][0]);
        let eq18_e297_d_n1: f64 = (p.p3 * s.dn[222][1]);
        let eq18_e297_d_n2: f64 = (p.p3 * s.dn[222][2]);
        let eq18_e297_d_n3: f64 = (p.p3 * s.dn[222][3]);
        let eq18_e297_d_n4: f64 = (p.p3 * s.dn[222][4]);
        let eq18_e297_d_n5: f64 = (p.p3 * s.dn[222][5]);
        let eq18_e297_d_n6: f64 = (p.p3 * s.dn[222][6]);
        let eq18_e297_d_n7: f64 = (p.p3 * s.dn[222][7]);
        let eq18_e297_d_n8: f64 = (p.p3 * s.dn[222][8]);
        let eq18_e297_d_n9: f64 = (p.p3 * s.dn[222][9]);
        let eq18_e297_d_n10: f64 = (p.p3 * s.dn[222][10]);
        let eq18_e297_d_n11: f64 = (p.p3 * s.dn[222][11]);
        let eq18_e297_d_b0: f64 = (p.p3 * s.db[222][0]);
        let eq18_e297_d_b1: f64 = (p.p3 * s.db[222][1]);
        let eq18_e298_q: f64 = eq18_e297;
        let eq18_e300: f64 = (eq18_e297 * p.p1);
        let eq18_e300_d_n0: f64 = (eq18_e297_d_n0 * p.p1);
        let eq18_e300_d_n1: f64 = (eq18_e297_d_n1 * p.p1);
        let eq18_e300_d_n2: f64 = (eq18_e297_d_n2 * p.p1);
        let eq18_e300_d_n3: f64 = (eq18_e297_d_n3 * p.p1);
        let eq18_e300_d_n4: f64 = (eq18_e297_d_n4 * p.p1);
        let eq18_e300_d_n5: f64 = (eq18_e297_d_n5 * p.p1);
        let eq18_e300_d_n6: f64 = (eq18_e297_d_n6 * p.p1);
        let eq18_e300_d_n7: f64 = (eq18_e297_d_n7 * p.p1);
        let eq18_e300_d_n8: f64 = (eq18_e297_d_n8 * p.p1);
        let eq18_e300_d_n9: f64 = (eq18_e297_d_n9 * p.p1);
        let eq18_e300_d_n10: f64 = (eq18_e297_d_n10 * p.p1);
        let eq18_e300_d_n11: f64 = (eq18_e297_d_n11 * p.p1);
        let eq18_e300_d_b0: f64 = (eq18_e297_d_b0 * p.p1);
        let eq18_e300_d_b1: f64 = (eq18_e297_d_b1 * p.p1);
        let eq18_e300_q: f64 = (eq18_e298_q * p.p1);
        let eq18_e300_q_d_n0: f64 = (eq18_e297_d_n0 * p.p1);
        let eq18_e300_q_d_n1: f64 = (eq18_e297_d_n1 * p.p1);
        let eq18_e300_q_d_n2: f64 = (eq18_e297_d_n2 * p.p1);
        let eq18_e300_q_d_n3: f64 = (eq18_e297_d_n3 * p.p1);
        let eq18_e300_q_d_n4: f64 = (eq18_e297_d_n4 * p.p1);
        let eq18_e300_q_d_n5: f64 = (eq18_e297_d_n5 * p.p1);
        let eq18_e300_q_d_n6: f64 = (eq18_e297_d_n6 * p.p1);
        let eq18_e300_q_d_n7: f64 = (eq18_e297_d_n7 * p.p1);
        let eq18_e300_q_d_n8: f64 = (eq18_e297_d_n8 * p.p1);
        let eq18_e300_q_d_n9: f64 = (eq18_e297_d_n9 * p.p1);
        let eq18_e300_q_d_n10: f64 = (eq18_e297_d_n10 * p.p1);
        let eq18_e300_q_d_n11: f64 = (eq18_e297_d_n11 * p.p1);
        let eq18_e300_q_d_b0: f64 = (eq18_e297_d_b0 * p.p1);
        let eq18_e300_q_d_b1: f64 = (eq18_e297_d_b1 * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e300_q_d_n0, eq18_e300_q_d_n1, eq18_e300_q_d_n2, eq18_e300_q_d_n3, eq18_e300_q_d_n4, eq18_e300_q_d_n5, eq18_e300_q_d_n6, eq18_e300_q_d_n7, eq18_e300_q_d_n8, eq18_e300_q_d_n9, eq18_e300_q_d_n10, eq18_e300_q_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e300_q_d_b0, eq18_e300_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * s.v[263]);
        let eq19_e305_d_n0: f64 = (eq19_e303 * s.dn[263][0]);
        let eq19_e305_d_n1: f64 = (eq19_e303 * s.dn[263][1]);
        let eq19_e305_d_n2: f64 = (eq19_e303 * s.dn[263][2]);
        let eq19_e305_d_n3: f64 = (eq19_e303 * s.dn[263][3]);
        let eq19_e305_d_n4: f64 = (eq19_e303 * s.dn[263][4]);
        let eq19_e305_d_n5: f64 = (eq19_e303 * s.dn[263][5]);
        let eq19_e305_d_n6: f64 = (eq19_e303 * s.dn[263][6]);
        let eq19_e305_d_n7: f64 = (eq19_e303 * s.dn[263][7]);
        let eq19_e305_d_n8: f64 = (eq19_e303 * s.dn[263][8]);
        let eq19_e305_d_n9: f64 = (eq19_e303 * s.dn[263][9]);
        let eq19_e305_d_n10: f64 = (eq19_e303 * s.dn[263][10]);
        let eq19_e305_d_n11: f64 = (eq19_e303 * s.dn[263][11]);
        let eq19_e305_d_b0: f64 = (eq19_e303 * s.db[263][0]);
        let eq19_e305_d_b1: f64 = (eq19_e303 * s.db[263][1]);
        let eq19_e306_q: f64 = eq19_e305;
        let eq19_e308: f64 = (eq19_e305 * p.p1);
        let eq19_e308_d_n0: f64 = (eq19_e305_d_n0 * p.p1);
        let eq19_e308_d_n1: f64 = (eq19_e305_d_n1 * p.p1);
        let eq19_e308_d_n2: f64 = (eq19_e305_d_n2 * p.p1);
        let eq19_e308_d_n3: f64 = (eq19_e305_d_n3 * p.p1);
        let eq19_e308_d_n4: f64 = (eq19_e305_d_n4 * p.p1);
        let eq19_e308_d_n5: f64 = (eq19_e305_d_n5 * p.p1);
        let eq19_e308_d_n6: f64 = (eq19_e305_d_n6 * p.p1);
        let eq19_e308_d_n7: f64 = (eq19_e305_d_n7 * p.p1);
        let eq19_e308_d_n8: f64 = (eq19_e305_d_n8 * p.p1);
        let eq19_e308_d_n9: f64 = (eq19_e305_d_n9 * p.p1);
        let eq19_e308_d_n10: f64 = (eq19_e305_d_n10 * p.p1);
        let eq19_e308_d_n11: f64 = (eq19_e305_d_n11 * p.p1);
        let eq19_e308_d_b0: f64 = (eq19_e305_d_b0 * p.p1);
        let eq19_e308_d_b1: f64 = (eq19_e305_d_b1 * p.p1);
        let eq19_e308_q: f64 = (eq19_e306_q * p.p1);
        let eq19_e308_q_d_n0: f64 = (eq19_e305_d_n0 * p.p1);
        let eq19_e308_q_d_n1: f64 = (eq19_e305_d_n1 * p.p1);
        let eq19_e308_q_d_n2: f64 = (eq19_e305_d_n2 * p.p1);
        let eq19_e308_q_d_n3: f64 = (eq19_e305_d_n3 * p.p1);
        let eq19_e308_q_d_n4: f64 = (eq19_e305_d_n4 * p.p1);
        let eq19_e308_q_d_n5: f64 = (eq19_e305_d_n5 * p.p1);
        let eq19_e308_q_d_n6: f64 = (eq19_e305_d_n6 * p.p1);
        let eq19_e308_q_d_n7: f64 = (eq19_e305_d_n7 * p.p1);
        let eq19_e308_q_d_n8: f64 = (eq19_e305_d_n8 * p.p1);
        let eq19_e308_q_d_n9: f64 = (eq19_e305_d_n9 * p.p1);
        let eq19_e308_q_d_n10: f64 = (eq19_e305_d_n10 * p.p1);
        let eq19_e308_q_d_n11: f64 = (eq19_e305_d_n11 * p.p1);
        let eq19_e308_q_d_b0: f64 = (eq19_e305_d_b0 * p.p1);
        let eq19_e308_q_d_b1: f64 = (eq19_e305_d_b1 * p.p1);
        let eq19_reactive_node_derivatives: [f64; 12] = [eq19_e308_q_d_n0, eq19_e308_q_d_n1, eq19_e308_q_d_n2, eq19_e308_q_d_n3, eq19_e308_q_d_n4, eq19_e308_q_d_n5, eq19_e308_q_d_n6, eq19_e308_q_d_n7, eq19_e308_q_d_n8, eq19_e308_q_d_n9, eq19_e308_q_d_n10, eq19_e308_q_d_n11];
        let eq19_reactive_branch_derivatives: [f64; 2] = [eq19_e308_q_d_b0, eq19_e308_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * s.v[264]);
        let eq20_e313_d_n0: f64 = (eq20_e311 * s.dn[264][0]);
        let eq20_e313_d_n1: f64 = (eq20_e311 * s.dn[264][1]);
        let eq20_e313_d_n2: f64 = (eq20_e311 * s.dn[264][2]);
        let eq20_e313_d_n3: f64 = (eq20_e311 * s.dn[264][3]);
        let eq20_e313_d_n4: f64 = (eq20_e311 * s.dn[264][4]);
        let eq20_e313_d_n5: f64 = (eq20_e311 * s.dn[264][5]);
        let eq20_e313_d_n6: f64 = (eq20_e311 * s.dn[264][6]);
        let eq20_e313_d_n7: f64 = (eq20_e311 * s.dn[264][7]);
        let eq20_e313_d_n8: f64 = (eq20_e311 * s.dn[264][8]);
        let eq20_e313_d_n9: f64 = (eq20_e311 * s.dn[264][9]);
        let eq20_e313_d_n10: f64 = (eq20_e311 * s.dn[264][10]);
        let eq20_e313_d_n11: f64 = (eq20_e311 * s.dn[264][11]);
        let eq20_e313_d_b0: f64 = (eq20_e311 * s.db[264][0]);
        let eq20_e313_d_b1: f64 = (eq20_e311 * s.db[264][1]);
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
        let eq20_e316_d_n11: f64 = (eq20_e313_d_n11 * p.p1);
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
        let eq20_e316_q_d_n11: f64 = (eq20_e313_d_n11 * p.p1);
        let eq20_e316_q_d_b0: f64 = (eq20_e313_d_b0 * p.p1);
        let eq20_e316_q_d_b1: f64 = (eq20_e313_d_b1 * p.p1);
        let eq20_reactive_node_derivatives: [f64; 12] = [eq20_e316_q_d_n0, eq20_e316_q_d_n1, eq20_e316_q_d_n2, eq20_e316_q_d_n3, eq20_e316_q_d_n4, eq20_e316_q_d_n5, eq20_e316_q_d_n6, eq20_e316_q_d_n7, eq20_e316_q_d_n8, eq20_e316_q_d_n9, eq20_e316_q_d_n10, eq20_e316_q_d_n11];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e316_q_d_b0, eq20_e316_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
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
        let eq23_e332: f64 = (s.v[230] + s.v[242]);
        let eq23_e332_d_n0: f64 = (s.dn[230][0] + s.dn[242][0]);
        let eq23_e332_d_n1: f64 = (s.dn[230][1] + s.dn[242][1]);
        let eq23_e332_d_n2: f64 = (s.dn[230][2] + s.dn[242][2]);
        let eq23_e332_d_n3: f64 = (s.dn[230][3] + s.dn[242][3]);
        let eq23_e332_d_n4: f64 = (s.dn[230][4] + s.dn[242][4]);
        let eq23_e332_d_n5: f64 = (s.dn[230][5] + s.dn[242][5]);
        let eq23_e332_d_n6: f64 = (s.dn[230][6] + s.dn[242][6]);
        let eq23_e332_d_n7: f64 = (s.dn[230][7] + s.dn[242][7]);
        let eq23_e332_d_n8: f64 = (s.dn[230][8] + s.dn[242][8]);
        let eq23_e332_d_n9: f64 = (s.dn[230][9] + s.dn[242][9]);
        let eq23_e332_d_n10: f64 = (s.dn[230][10] + s.dn[242][10]);
        let eq23_e332_d_n11: f64 = (s.dn[230][11] + s.dn[242][11]);
        let eq23_e332_d_b0: f64 = (s.db[230][0] + s.db[242][0]);
        let eq23_e332_d_b1: f64 = (s.db[230][1] + s.db[242][1]);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n2: f64 = (p.p3 * eq23_e332_d_n2);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e333_d_n11: f64 = (p.p3 * eq23_e332_d_n11);
        let eq23_e333_d_b0: f64 = (p.p3 * eq23_e332_d_b0);
        let eq23_e333_d_b1: f64 = (p.p3 * eq23_e332_d_b1);
        let eq23_e334_q: f64 = eq23_e333;
        let eq23_e336: f64 = (eq23_e333 * p.p1);
        let eq23_e336_d_n0: f64 = (eq23_e333_d_n0 * p.p1);
        let eq23_e336_d_n1: f64 = (eq23_e333_d_n1 * p.p1);
        let eq23_e336_d_n2: f64 = (eq23_e333_d_n2 * p.p1);
        let eq23_e336_d_n3: f64 = (eq23_e333_d_n3 * p.p1);
        let eq23_e336_d_n4: f64 = (eq23_e333_d_n4 * p.p1);
        let eq23_e336_d_n5: f64 = (eq23_e333_d_n5 * p.p1);
        let eq23_e336_d_n6: f64 = (eq23_e333_d_n6 * p.p1);
        let eq23_e336_d_n7: f64 = (eq23_e333_d_n7 * p.p1);
        let eq23_e336_d_n8: f64 = (eq23_e333_d_n8 * p.p1);
        let eq23_e336_d_n9: f64 = (eq23_e333_d_n9 * p.p1);
        let eq23_e336_d_n10: f64 = (eq23_e333_d_n10 * p.p1);
        let eq23_e336_d_n11: f64 = (eq23_e333_d_n11 * p.p1);
        let eq23_e336_d_b0: f64 = (eq23_e333_d_b0 * p.p1);
        let eq23_e336_d_b1: f64 = (eq23_e333_d_b1 * p.p1);
        let eq23_e336_q: f64 = (eq23_e334_q * p.p1);
        let eq23_e336_q_d_n0: f64 = (eq23_e333_d_n0 * p.p1);
        let eq23_e336_q_d_n1: f64 = (eq23_e333_d_n1 * p.p1);
        let eq23_e336_q_d_n2: f64 = (eq23_e333_d_n2 * p.p1);
        let eq23_e336_q_d_n3: f64 = (eq23_e333_d_n3 * p.p1);
        let eq23_e336_q_d_n4: f64 = (eq23_e333_d_n4 * p.p1);
        let eq23_e336_q_d_n5: f64 = (eq23_e333_d_n5 * p.p1);
        let eq23_e336_q_d_n6: f64 = (eq23_e333_d_n6 * p.p1);
        let eq23_e336_q_d_n7: f64 = (eq23_e333_d_n7 * p.p1);
        let eq23_e336_q_d_n8: f64 = (eq23_e333_d_n8 * p.p1);
        let eq23_e336_q_d_n9: f64 = (eq23_e333_d_n9 * p.p1);
        let eq23_e336_q_d_n10: f64 = (eq23_e333_d_n10 * p.p1);
        let eq23_e336_q_d_n11: f64 = (eq23_e333_d_n11 * p.p1);
        let eq23_e336_q_d_b0: f64 = (eq23_e333_d_b0 * p.p1);
        let eq23_e336_q_d_b1: f64 = (eq23_e333_d_b1 * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e336_q_d_n0, eq23_e336_q_d_n1, eq23_e336_q_d_n2, eq23_e336_q_d_n3, eq23_e336_q_d_n4, eq23_e336_q_d_n5, eq23_e336_q_d_n6, eq23_e336_q_d_n7, eq23_e336_q_d_n8, eq23_e336_q_d_n9, eq23_e336_q_d_n10, eq23_e336_q_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e336_q_d_b0, eq23_e336_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e351: f64 = (s.v[227] + s.v[243]);
        let eq25_e351_d_n0: f64 = (s.dn[227][0] + s.dn[243][0]);
        let eq25_e351_d_n1: f64 = (s.dn[227][1] + s.dn[243][1]);
        let eq25_e351_d_n2: f64 = (s.dn[227][2] + s.dn[243][2]);
        let eq25_e351_d_n3: f64 = (s.dn[227][3] + s.dn[243][3]);
        let eq25_e351_d_n4: f64 = (s.dn[227][4] + s.dn[243][4]);
        let eq25_e351_d_n5: f64 = (s.dn[227][5] + s.dn[243][5]);
        let eq25_e351_d_n6: f64 = (s.dn[227][6] + s.dn[243][6]);
        let eq25_e351_d_n7: f64 = (s.dn[227][7] + s.dn[243][7]);
        let eq25_e351_d_n8: f64 = (s.dn[227][8] + s.dn[243][8]);
        let eq25_e351_d_n9: f64 = (s.dn[227][9] + s.dn[243][9]);
        let eq25_e351_d_n10: f64 = (s.dn[227][10] + s.dn[243][10]);
        let eq25_e351_d_n11: f64 = (s.dn[227][11] + s.dn[243][11]);
        let eq25_e351_d_b0: f64 = (s.db[227][0] + s.db[243][0]);
        let eq25_e351_d_b1: f64 = (s.db[227][1] + s.db[243][1]);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n2: f64 = (p.p3 * eq25_e351_d_n2);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e352_d_n11: f64 = (p.p3 * eq25_e351_d_n11);
        let eq25_e352_d_b0: f64 = (p.p3 * eq25_e351_d_b0);
        let eq25_e352_d_b1: f64 = (p.p3 * eq25_e351_d_b1);
        let eq25_e353_q: f64 = eq25_e352;
        let eq25_e355: f64 = (eq25_e352 * p.p1);
        let eq25_e355_d_n0: f64 = (eq25_e352_d_n0 * p.p1);
        let eq25_e355_d_n1: f64 = (eq25_e352_d_n1 * p.p1);
        let eq25_e355_d_n2: f64 = (eq25_e352_d_n2 * p.p1);
        let eq25_e355_d_n3: f64 = (eq25_e352_d_n3 * p.p1);
        let eq25_e355_d_n4: f64 = (eq25_e352_d_n4 * p.p1);
        let eq25_e355_d_n5: f64 = (eq25_e352_d_n5 * p.p1);
        let eq25_e355_d_n6: f64 = (eq25_e352_d_n6 * p.p1);
        let eq25_e355_d_n7: f64 = (eq25_e352_d_n7 * p.p1);
        let eq25_e355_d_n8: f64 = (eq25_e352_d_n8 * p.p1);
        let eq25_e355_d_n9: f64 = (eq25_e352_d_n9 * p.p1);
        let eq25_e355_d_n10: f64 = (eq25_e352_d_n10 * p.p1);
        let eq25_e355_d_n11: f64 = (eq25_e352_d_n11 * p.p1);
        let eq25_e355_d_b0: f64 = (eq25_e352_d_b0 * p.p1);
        let eq25_e355_d_b1: f64 = (eq25_e352_d_b1 * p.p1);
        let eq25_e355_q: f64 = (eq25_e353_q * p.p1);
        let eq25_e355_q_d_n0: f64 = (eq25_e352_d_n0 * p.p1);
        let eq25_e355_q_d_n1: f64 = (eq25_e352_d_n1 * p.p1);
        let eq25_e355_q_d_n2: f64 = (eq25_e352_d_n2 * p.p1);
        let eq25_e355_q_d_n3: f64 = (eq25_e352_d_n3 * p.p1);
        let eq25_e355_q_d_n4: f64 = (eq25_e352_d_n4 * p.p1);
        let eq25_e355_q_d_n5: f64 = (eq25_e352_d_n5 * p.p1);
        let eq25_e355_q_d_n6: f64 = (eq25_e352_d_n6 * p.p1);
        let eq25_e355_q_d_n7: f64 = (eq25_e352_d_n7 * p.p1);
        let eq25_e355_q_d_n8: f64 = (eq25_e352_d_n8 * p.p1);
        let eq25_e355_q_d_n9: f64 = (eq25_e352_d_n9 * p.p1);
        let eq25_e355_q_d_n10: f64 = (eq25_e352_d_n10 * p.p1);
        let eq25_e355_q_d_n11: f64 = (eq25_e352_d_n11 * p.p1);
        let eq25_e355_q_d_b0: f64 = (eq25_e352_d_b0 * p.p1);
        let eq25_e355_q_d_b1: f64 = (eq25_e352_d_b1 * p.p1);
        let eq25_reactive_node_derivatives: [f64; 12] = [eq25_e355_q_d_n0, eq25_e355_q_d_n1, eq25_e355_q_d_n2, eq25_e355_q_d_n3, eq25_e355_q_d_n4, eq25_e355_q_d_n5, eq25_e355_q_d_n6, eq25_e355_q_d_n7, eq25_e355_q_d_n8, eq25_e355_q_d_n9, eq25_e355_q_d_n10, eq25_e355_q_d_n11];
        let eq25_reactive_branch_derivatives: [f64; 2] = [eq25_e355_q_d_b0, eq25_e355_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e394_q: f64 = (nv11 - 0.0);
        let eq32_e395: f64 = (s.v[330] * (nv11 - 0.0));
        let eq32_e395_d_n0: f64 = (s.dn[330][0] * (nv11 - 0.0));
        let eq32_e395_d_n1: f64 = (s.dn[330][1] * (nv11 - 0.0));
        let eq32_e395_d_n2: f64 = (s.dn[330][2] * (nv11 - 0.0));
        let eq32_e395_d_n3: f64 = (s.dn[330][3] * (nv11 - 0.0));
        let eq32_e395_d_n4: f64 = (s.dn[330][4] * (nv11 - 0.0));
        let eq32_e395_d_n5: f64 = (s.dn[330][5] * (nv11 - 0.0));
        let eq32_e395_d_n6: f64 = (s.dn[330][6] * (nv11 - 0.0));
        let eq32_e395_d_n7: f64 = (s.dn[330][7] * (nv11 - 0.0));
        let eq32_e395_d_n8: f64 = (s.dn[330][8] * (nv11 - 0.0));
        let eq32_e395_d_n9: f64 = (s.dn[330][9] * (nv11 - 0.0));
        let eq32_e395_d_n10: f64 = (s.dn[330][10] * (nv11 - 0.0));
        let eq32_e395_d_n11: f64 = ((s.dn[330][11] * (nv11 - 0.0)) + s.v[330]);
        let eq32_e395_d_b0: f64 = (s.db[330][0] * (nv11 - 0.0));
        let eq32_e395_d_b1: f64 = (s.db[330][1] * (nv11 - 0.0));
        let eq32_e395_q: f64 = (s.v[330] * eq32_e394_q);
        let eq32_e395_q_d_n0: f64 = (s.dn[330][0] * eq32_e394_q);
        let eq32_e395_q_d_n1: f64 = (s.dn[330][1] * eq32_e394_q);
        let eq32_e395_q_d_n2: f64 = (s.dn[330][2] * eq32_e394_q);
        let eq32_e395_q_d_n3: f64 = (s.dn[330][3] * eq32_e394_q);
        let eq32_e395_q_d_n4: f64 = (s.dn[330][4] * eq32_e394_q);
        let eq32_e395_q_d_n5: f64 = (s.dn[330][5] * eq32_e394_q);
        let eq32_e395_q_d_n6: f64 = (s.dn[330][6] * eq32_e394_q);
        let eq32_e395_q_d_n7: f64 = (s.dn[330][7] * eq32_e394_q);
        let eq32_e395_q_d_n8: f64 = (s.dn[330][8] * eq32_e394_q);
        let eq32_e395_q_d_n9: f64 = (s.dn[330][9] * eq32_e394_q);
        let eq32_e395_q_d_n10: f64 = (s.dn[330][10] * eq32_e394_q);
        let eq32_e395_q_d_n11: f64 = ((s.dn[330][11] * eq32_e394_q) + s.v[330]);
        let eq32_e395_q_d_b0: f64 = (s.db[330][0] * eq32_e394_q);
        let eq32_e395_q_d_b1: f64 = (s.db[330][1] * eq32_e394_q);
        let eq32_reactive_node_derivatives: [f64; 12] = [eq32_e395_q_d_n0, eq32_e395_q_d_n1, eq32_e395_q_d_n2, eq32_e395_q_d_n3, eq32_e395_q_d_n4, eq32_e395_q_d_n5, eq32_e395_q_d_n6, eq32_e395_q_d_n7, eq32_e395_q_d_n8, eq32_e395_q_d_n9, eq32_e395_q_d_n10, eq32_e395_q_d_n11];
        let eq32_reactive_branch_derivatives: [f64; 2] = [eq32_e395_q_d_b0, eq32_e395_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
