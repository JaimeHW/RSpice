#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
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
        s.v[0] = (8.8541878176e-12 * 11.8);

        s.v[1] = (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) });

        s.v[388] = if ((!(if self.param_given[6] { 1.0 } else { 0.0 } != 0.0)) && (if self.param_given[96] { 1.0 } else { 0.0 } != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[388] != 0.0) {
            s.store_scalar(1, (if (p.p96 > (-250.0)) { p.p96 } else { (-250.0) }));
        }

        s.v[2] = (if (p.p5 > 1e-12) { p.p5 } else { 1e-12 });

        s.v[3] = (if (p.p8 > 1e-12) { p.p8 } else { 1e-12 });

        s.v[4] = (if (p.p9 > 1e-18) { p.p9 } else { 1e-18 });

        s.v[5] = (if (p.p10 > 1e-18) { p.p10 } else { 1e-18 });

        s.v[6] = (if (p.p11 > 0.05) { p.p11 } else { 0.05 });

        s.v[7] = (if (p.p12 > 0.05) { p.p12 } else { 0.05 });

        s.v[8] = (if (p.p13 > 0.05) { p.p13 } else { 0.05 });

        s.v[9] = (if (p.p14 > 0.05) { (if (p.p14 < 0.95) { p.p14 } else { 0.95 }) } else { 0.05 });

        s.v[10] = (if (p.p15 > 0.05) { (if (p.p15 < 0.95) { p.p15 } else { 0.95 }) } else { 0.05 });

        s.v[11] = (if (p.p16 > 0.05) { (if (p.p16 < 0.95) { p.p16 } else { 0.95 }) } else { 0.05 });

        s.v[12] = p.p17;

        s.v[13] = p.p18;

        s.v[14] = p.p19;

        s.v[15] = (if (p.p20 > 0.0) { p.p20 } else { 0.0 });

        s.v[16] = (if (p.p21 > 0.0) { p.p21 } else { 0.0 });

        s.v[17] = (if (p.p22 > 0.0) { p.p22 } else { 0.0 });

        s.v[20] = (if (p.p23 > 0.0) { p.p23 } else { 0.0 });

        s.v[21] = (if (p.p24 > 0.0) { p.p24 } else { 0.0 });

        s.v[22] = (if (p.p25 > 0.0) { p.p25 } else { 0.0 });

        s.v[18] = (if (p.p26 > 1e-9) { p.p26 } else { 1e-9 });

        s.v[19] = (if (p.p27 > 1e-9) { p.p27 } else { 1e-9 });

        s.v[23] = (if (p.p28 > 0.0) { p.p28 } else { 0.0 });

        s.v[24] = (if (p.p29 > 0.0) { p.p29 } else { 0.0 });

        s.v[25] = (if (p.p30 > 0.0) { p.p30 } else { 0.0 });

        s.v[26] = (if (p.p31 > 0.01) { p.p31 } else { 0.01 });

        s.v[27] = (if (p.p32 > 0.01) { p.p32 } else { 0.01 });

        s.v[28] = (if (p.p33 > 0.01) { p.p33 } else { 0.01 });

        s.v[29] = (if (p.p34 > 0.0) { p.p34 } else { 0.0 });

        s.v[30] = (if (p.p35 > 0.0) { p.p35 } else { 0.0 });

        s.v[31] = (if (p.p36 > 0.0) { p.p36 } else { 0.0 });

        s.v[32] = p.p37;

        s.v[33] = p.p38;

        s.v[34] = p.p39;

        s.v[35] = p.p40;

        s.v[36] = p.p41;

        s.v[37] = p.p42;

        s.v[38] = (if (p.p43 > 0.1) { p.p43 } else { 0.1 });

        s.v[39] = (if (p.p44 > 0.1) { p.p44 } else { 0.1 });

        s.v[40] = (if (p.p45 > 0.1) { p.p45 } else { 0.1 });

        s.v[41] = (if (p.p46 > 0.1) { p.p46 } else { 0.1 });

        s.v[42] = (if (p.p47 > 0.1) { p.p47 } else { 0.1 });

        s.v[43] = (if (p.p48 > 0.1) { p.p48 } else { 0.1 });

        s.v[44] = p.p7;

        s.v[48] = (if (p.p49 > 0.0) { p.p49 } else { 0.0 });

        s.v[49] = (if (p.p50 > 0.0) { p.p50 } else { 0.0 });

        s.v[50] = (if (p.p51 > 0.0) { p.p51 } else { 0.0 });

        s.v[52] = (if (p.p52 > 0.0) { p.p52 } else { 0.0 });

        s.v[51] = (if (p.p53 > 0.0) { p.p53 } else { 0.0 });

        s.v[54] = (if (p.p55 > 0.1) { p.p55 } else { 0.1 });

        s.v[53] = (if (p.p54 > 0.0) { p.p54 } else { 0.0 });

        s.v[55] = (if (p.p56 > 0.0) { p.p56 } else { 0.0 });

        s.v[56] = p.p57;

        s.v[57] = p.p58;

        s.v[58] = p.p59;

        s.v[59] = p.p60;

        s.v[60] = p.p61;

        s.v[61] = p.p62;

        s.v[62] = (if (p.p63 > 0.1) { p.p63 } else { 0.1 });

        s.v[64] = (if (p.p64 > 0.1) { p.p64 } else { 0.1 });

        s.v[63] = (if (p.p65 > 0.1) { p.p65 } else { 0.1 });

        s.v[75] = (if (p.p76 > 0.1) { p.p76 } else { 0.1 });

        s.v[76] = (if (p.p77 > 0.0) { p.p77 } else { 0.0 });

        s.v[77] = (if (p.p78 > 0.0) { p.p78 } else { 0.0 });

        s.v[45] = 0.0;

        s.v[389] = if (p.p81 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[389] != 0.0) {
            s.store_scalar(45, 1.0);
        }

        if (!(s.v[389] != 0.0)) {
            s.store_scalar(45, 0.0);
        }

        s.v[46] = (if (p.p82 > 0.5) { p.p82 } else { 0.5 });

        s.v[47] = (if (p.p83 > 0.0) { p.p83 } else { 0.0 });

        s.store_offset(78, 1, 273.15);

        s.v[79] = ((ctx.temperature() + p.p102)).max((273.15 + (-250.0)));

        s.store_div_from_scalar(80, s.v[79], 78);

        s.v[81] = (1.3806505e-23 / 1.6021918e-19);

        s.store_scale(82, 78, s.v[81]);

        s.store_div_from_scalar(83, 1.0, 82);

        s.v[84] = (s.v[81] * s.v[79]);

        s.v[85] = (1.0 / s.v[84]);

        s.store_div_ad(89, A::neg(A::mul(A::scale(s.ad_value(78), 0.000702), s.ad_value(78))), A::offset(s.ad_value(78), 1108.0));

        s.store_offset(92, 89, s.v[12]);

        s.store_offset(93, 89, s.v[13]);

        s.store_offset(94, 89, s.v[14]);

        s.v[90] = ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79]));

        s.v[95] = (s.v[12] + s.v[90]);

        s.v[96] = (s.v[13] + s.v[90]);

        s.v[97] = (s.v[14] + s.v[90]);

        s.store_mul_ad(98, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp(A::scale(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), 0.5)));

        s.store_mul_ad(99, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp(A::scale(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), 0.5)));

        s.store_mul_ad(100, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp(A::scale(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), 0.5)));

        s.store_mul_ad(176, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[62])), A::exp(A::scale(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62])))));

        s.store_mul_ad(177, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[64])), A::exp(A::scale(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64])))));

        s.store_mul_ad(178, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[63])), A::exp(A::scale(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63])))));

        s.store_mul_ad_lhs(101, A::scale(s.ad_value(176), s.v[15]), 176);

        s.store_mul_ad_lhs(102, A::scale(s.ad_value(177), s.v[16]), 177);

        s.store_mul_ad_lhs(103, A::scale(s.ad_value(178), s.v[17]), 178);

        s.store_sub_ad(104, A::scale(s.ad_value(80), s.v[6]), A::scale(A::ln(s.ad_value(98)), (2.0 * s.v[84])));

        s.store_sub_ad(105, A::scale(s.ad_value(80), s.v[7]), A::scale(A::ln(s.ad_value(99)), (2.0 * s.v[84])));

        s.store_sub_ad(106, A::scale(s.ad_value(80), s.v[8]), A::scale(A::ln(s.ad_value(100)), (2.0 * s.v[84])));

        s.store_add_ad_rhs(107, 104, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(104)), s.v[85])), 1.0)), s.v[84]));

        s.store_add_ad_rhs(108, 105, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(105)), s.v[85])), 1.0)), s.v[84]));

        s.store_add_ad_rhs(109, 106, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(106)), s.v[85])), 1.0)), s.v[84]));

        s.store_div_from_scalar(119, 1.0, 107);

        s.store_div_from_scalar(120, 1.0, 108);

        s.store_div_from_scalar(121, 1.0, 109);

        s.v[122] = (1.0 - s.v[9]);

        s.v[123] = (1.0 - s.v[10]);

        s.v[124] = (1.0 - s.v[11]);

        s.v[125] = (1.0 / s.v[122]);

        s.v[126] = (1.0 / s.v[123]);

        s.v[127] = (1.0 / s.v[124]);

        s.store_scale_ad(128, A::powf(A::scale(s.ad_value(119), s.v[6]), s.v[9]), s.v[3]);

        s.store_scale_ad(129, A::powf(A::scale(s.ad_value(120), s.v[7]), s.v[10]), s.v[4]);

        s.store_scale_ad(130, A::powf(A::scale(s.ad_value(121), s.v[8]), s.v[11]), s.v[5]);

        s.store_scaled_mul(131, 128, 107, s.v[125]);

        s.store_scaled_mul(132, 129, 108, s.v[126]);

        s.store_scaled_mul(133, 130, 109, s.v[127]);

        s.store_scale(134, 128, 2.0);

        s.store_scale(135, 129, 2.0);

        s.store_scale(136, 130, 2.0);

        s.v[137] = (s.v[0] / s.v[3]);

        s.v[138] = ((s.v[18] * s.v[0]) / s.v[4]);

        s.v[139] = ((s.v[19] * s.v[0]) / s.v[5]);

        s.v[140] = (1.0 / s.v[137]);

        s.v[141] = (1.0 / s.v[138]);

        s.v[142] = (1.0 / s.v[139]);

        s.v[143] = (1.0 / s.v[6]);

        s.v[144] = (1.0 / s.v[7]);

        s.v[145] = (1.0 / s.v[8]);

        s.v[86] = (1.772453850905516 * 0.29214664);

        s.v[87] = (((((-5.0) * 0.29214664) + 6.0) - ((s.v[86]) as f64).powf((-2.0))) / 3.0);

        s.v[88] = ((1.0 - 0.29214664) - s.v[87]);

        s.v[146] = ((0.5 * s.v[95])).max(s.v[84]);

        s.v[147] = ((0.5 * s.v[96])).max(s.v[84]);

        s.v[148] = ((0.5 * s.v[97])).max(s.v[84]);

        s.v[149] = (s.v[146] * s.v[85]);

        s.v[150] = (s.v[147] * s.v[85]);

        s.v[151] = (s.v[148] * s.v[85]);

        s.v[152] = (((((((32.0 * s.v[26]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[146] * s.v[146]) * s.v[146]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[153] = (((((((32.0 * s.v[27]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[147] * s.v[147]) * s.v[147]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[154] = (((((((32.0 * s.v[28]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[148] * s.v[148]) * s.v[148]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.store_scale_ad(155, A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[35]), 1.0), s.v[32]);

        s.store_scale_ad(156, A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[36]), 1.0), s.v[33]);

        s.store_scale_ad(157, A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[37]), 1.0), s.v[34]);

        if !(s.v[155] > 0.0) {
            s.store_scalar(155, 0.0);
        }

        if !(s.v[156] > 0.0) {
            s.store_scalar(156, 0.0);
        }

        if !(s.v[157] > 0.0) {
            s.store_scalar(157, 0.0);
        }

        s.v[158] = ((s.v[44] - 1.0) / s.v[44]);

        s.v[159] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[41])));

        s.v[160] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[42])));

        s.v[161] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[43])));

        s.store_scale_ad(38, A::offset(A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[57]), s.v[56])), 1.0), s.v[38]);

        s.store_scale_ad(39, A::offset(A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[59]), s.v[58])), 1.0), s.v[39]);

        s.store_scale_ad(40, A::offset(A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[61]), s.v[60])), 1.0), s.v[40]);

        s.v[390] = if (s.v[38] <= 0.1) { 1.0 } else { 0.0 };

        if (s.v[390] != 0.0) {
            s.store_scalar(38, 0.1);
        }

        if (s.v[390] != 0.0) {
            s.store_scalar(162, 10.0);
        }

        if (!(s.v[390] != 0.0)) {
            s.store_div_from_scalar(162, 1.0, 38);
        }

        s.v[391] = if (s.v[39] <= 0.1) { 1.0 } else { 0.0 };

        if (s.v[391] != 0.0) {
            s.store_scalar(39, 0.1);
        }

        if (s.v[391] != 0.0) {
            s.store_scalar(163, 10.0);
        }

        if (!(s.v[391] != 0.0)) {
            s.store_div_from_scalar(163, 1.0, 39);
        }

        s.v[392] = if (s.v[40] <= 0.1) { 1.0 } else { 0.0 };

        if (s.v[392] != 0.0) {
            s.store_scalar(40, 0.1);
        }

        if (s.v[392] != 0.0) {
            s.store_scalar(164, 10.0);
        }

        if (!(s.v[392] != 0.0)) {
            s.store_div_from_scalar(164, 1.0, 40);
        }

        s.v[179] = (1.0 - (0.01 * s.v[77]));

        s.store_scale(165, 162, ((-((s.v[159] * s.v[159]) * ((s.v[158]) as f64).powf((s.v[41] - 1.0)))) * s.v[41]));

        s.store_scale(166, 163, ((-((s.v[160] * s.v[160]) * ((s.v[158]) as f64).powf((s.v[42] - 1.0)))) * s.v[42]));

        s.store_scale(167, 164, ((-((s.v[161] * s.v[161]) * ((s.v[158]) as f64).powf((s.v[43] - 1.0)))) * s.v[43]));

        s.store_scale_ad(173, A::powf(s.ad_value(80), s.v[51]), s.v[48]);

        s.store_scale_ad(175, A::powf(s.ad_value(80), s.v[51]), s.v[50]);

        s.store_scale_ad(174, A::powf(s.ad_value(80), s.v[51]), s.v[49]);

        s.store_scale_ad(172, A::powf(s.ad_value(80), s.v[51]), s.v[52]);

        s.v[308] = (p.p87 * 1000000.0);

        s.v[310] = (p.p89 * 1000000.0);

        s.v[309] = (p.p88 * 1000000.0);

        s.v[307] = s.v[308];

        s.v[313] = s.v[62];

        s.v[311] = (1450.0 * 0.0001);

        s.v[312] = (500.0 * 0.0001);

        s.v[368] = 0.6;

        s.v[369] = 0.001;

        s.store_scale(318, 176, 1.45e16);

        s.store_scale_ad(319, A::square(s.ad_value(318)), 1.0 / (s.v[307]));

        s.store_powf(316, 80, (-1.5));

        s.store_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));

        s.store_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));

        s.store_div_ad(322, A::mul(A::scale(s.ad_value(320), 2.0), s.ad_value(321)), A::add(s.ad_value(320), s.ad_value(321)));

        s.store_powf(317, 80, p.p97);

        s.store_scale(324, 317, p.p93);

        s.store_sqrt_ad(323, A::mul(s.ad_value(324), s.ad_value(322)));

        s.store_scale_ad(347, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), (s.v[313] / s.v[85]));

        s.store_scale_ad(348, A::add(A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323))), (s.v[313] / s.v[85]));

        s.v[256] = (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]);

        s.v[257] = (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[258] = (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[263] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[393] = if ((s.v[101] * s.v[256]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[393] != 0.0) {
            s.store_scale_ad(168, A::ln(A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0)), (s.v[84] * s.v[62]));
        }

        if (!(s.v[393] != 0.0)) {
            s.store_scalar(168, 100000000.0);
        }

        s.v[394] = if ((s.v[102] * s.v[257]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[394] != 0.0) {
            s.store_scale_ad(169, A::ln(A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0)), (s.v[84] * s.v[64]));
        }

        if (!(s.v[394] != 0.0)) {
            s.store_scalar(169, 100000000.0);
        }

        s.v[395] = if ((s.v[103] * s.v[258]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[395] != 0.0) {
            s.store_scale_ad(170, A::ln(A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0)), (s.v[84] * s.v[63]));
        }

        if (!(s.v[395] != 0.0)) {
            s.store_scalar(170, 100000000.0);
        }

        s.store_ad(262, &A::min(A::min(s.ad_value(168), s.ad_value(169)), s.ad_value(170)));

        s.v[396] = if ((((s.v[262] * s.v[85])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (s.v[396] != 0.0) {
            s.store_exp_ad(263, A::scale(s.ad_value(262), s.v[85]));
        }

        s.v[397] = if ((s.v[262] * s.v[85]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((!(s.v[396] != 0.0)) && (s.v[397] != 0.0)) {
            s.store_div_from_scalar_ad(263, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[396] != 0.0)) && (!(s.v[397] != 0.0))) {
            s.store_scale_ad(263, A::offset(A::mul(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        s.copy_ad(110, 107);

        s.copy_ad(111, 108);

        s.copy_ad(112, 109);

        s.v[113] = s.v[9];

        s.v[114] = s.v[10];

        s.v[115] = s.v[11];

        s.v[116] = s.v[6];

        s.v[117] = s.v[7];

        s.v[118] = s.v[8];

        s.v[398] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[398] != 0.0) {
            s.store_add(110, 108, 109);
        }

        if (s.v[398] != 0.0) {
            s.store_scalar(113, (0.9 * (s.v[10]).min(s.v[11])));
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
        if (s.v[398] != 0.0) {
            s.store_scalar(116, (s.v[7] + s.v[8]));
        }

        s.v[399] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[399] != 0.0) {
            s.store_add(111, 107, 109);
        }

        if (s.v[399] != 0.0) {
            s.store_scalar(114, (0.9 * (s.v[9]).min(s.v[11])));
        }

        if (s.v[399] != 0.0) {
            s.store_scalar(117, (s.v[6] + s.v[8]));
        }

        s.v[400] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[400] != 0.0) {
            s.store_add(112, 107, 108);
        }

        if (s.v[400] != 0.0) {
            s.store_scalar(115, (0.9 * (s.v[9]).min(s.v[10])));
        }

        if (s.v[400] != 0.0) {
            s.store_scalar(118, (s.v[6] + s.v[7]));
        }

        s.store_ad(264, &A::min(A::min(s.ad_value(110), s.ad_value(111)), s.ad_value(112)));

        s.store_scale(265, 264, 0.1);

        s.store_ad(91, &A::max(A::max(s.ad_value(113), s.ad_value(114)), s.ad_value(115)));

        s.store_mul_ad_rhs(266, 264, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(91)))));

        s.store_offset_ad(267, A::min(A::min(s.ad_value(116), s.ad_value(117)), s.ad_value(118)), (-0.05));

        s.store_add_ad(289, A::add(A::scale(s.ad_value(101), s.v[256]), A::scale(s.ad_value(102), s.v[257])), A::scale(s.ad_value(103), s.v[258]));

        s.v[300] = 0.0;

        s.v[301] = 1.0;

        s.v[303] = 1.0;

        s.v[302] = 0.0;

        s.v[305] = 1.0;

        s.v[304] = 0.0;

        s.v[306] = 0.0;

        s.v[294] = 0.0;

        s.v[295] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[185] = 0.0;

        s.v[186] = 0.0;

        s.v[187] = 0.0;

        s.v[188] = 0.0;

        s.v[189] = 0.0;

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[200] = 0.0;

        s.v[208] = 0.0;

        s.v[259] = 1.0;

        s.v[260] = 1.0;

        s.v[261] = 1.0;

        s.v[195] = 0.0;

        s.v[203] = 0.0;

        s.v[204] = 0.0;

        s.v[285] = 0.0;

        s.v[409] = if ((s.v[256] * s.v[173]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[409] != 0.0) {
            s.store_div_from_scalar(285, s.v[256], 173);
        }

        s.v[410] = if ((s.v[257] * s.v[174]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[410] != 0.0) {
            s.store_add_ad_lhs(285, A::div_from_scalar(s.v[257], s.ad_value(174)), 285);
        }

        s.v[411] = if ((s.v[258] * s.v[175]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[411] != 0.0) {
            s.store_add_ad_lhs(285, A::div_from_scalar(s.v[258], s.ad_value(175)), 285);
        }

        s.v[412] = if (s.v[285] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[412] != 0.0) {
            s.store_add_ad_lhs(171, A::div_from_scalar(1.0, s.ad_value(285)), 172);
        }

        if (!(s.v[412] != 0.0)) {
            s.copy_ad(171, 172);
        }

        s.v[370] = 0.0;

        s.v[372] = 0.0;

        s.v[371] = 0.0;

        s.v[345] = 0.0;

        s.v[338] = 0.0;

        s.v[339] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

        s.v[344] = 0.0;

        s.v[333] = (1.6021918e-19 * s.v[256]);

        s.v[343] = ((((2.0 * s.v[0]) / (1.6021918e-19 * s.v[307]))) as f64).sqrt();

        s.v[314] = ((p.p94 - s.v[343]) - 1e-7);

        s.v[315] = ((4.0 * p.p94) * 1e-7);

        if !(s.v[315] > 0.0) {
            s.store_scalar(315, (-s.v[315]));
        }

        s.store_sqrt_ad(315, A::offset(s.ad_value(315), (s.v[314] * s.v[314])));

        s.store_sub_from_scalar_ad(343, p.p94, A::scale(A::offset(s.ad_value(315), s.v[314]), 0.5));

        s.v[413] = if (s.v[45] > 0.9) { 1.0 } else { 0.0 };

        s.v[414] = if ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0))) { 1.0 } else { 0.0 };

        if ((s.v[413] != 0.0) && (s.v[414] != 0.0)) {
            s.store_scalar(45, 0.0);
        }

        s.v[415] = if (s.v[256] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[413] != 0.0) && (!(s.v[414] != 0.0))) && (s.v[415] != 0.0)) {
            s.store_scalar(301, s.v[62]);
        }

        s.v[416] = if (s.v[258] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[413] != 0.0) && (!(s.v[414] != 0.0))) && (s.v[416] != 0.0)) {
            s.store_scalar(301, s.v[63]);
        }

        s.v[417] = if (s.v[257] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[413] != 0.0) && (!(s.v[414] != 0.0))) && (s.v[417] != 0.0)) {
            s.store_scalar(301, s.v[64]);
        }

        s.v[418] = if (s.v[45] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[418] != 0.0) {
            s.store_scalar(419, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(420, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(421, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(422, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(423, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(424, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(425, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(426, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(427, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(277, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(428, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(429, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(430, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(431, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(432, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(433, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(434, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(435, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(436, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(437, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(438, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(439, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(440, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(441, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(442, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(443, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(444, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(445, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(446, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(447, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(448, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(449, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(450, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(451, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(452, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(453, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(454, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(455, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(456, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(457, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(458, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(459, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(460, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(461, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(462, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(205, 0.4);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(206, 0.65);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(207, 0.8);
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(190, A::neg(s.ad_value(205)), s.v[46]);
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(191, A::neg(s.ad_value(206)), s.v[46]);
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(192, A::neg(s.ad_value(207)), s.v[46]);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(193, 0.1);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(194, 0.2);
        }

        s.v[463] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_add_ad_rhs(424, 190, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(190), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[464] = if (s.v[190] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[465] = if ((((0.5 * (s.v[190] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[465] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(190), (s.v[85] * 0.5)));
        }

        s.v[466] = if ((0.5 * (s.v[190] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) && (s.v[466] != 0.0)) {
            let assign3970_ad_e2385: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign3970_ad_e2385);
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) && (!(s.v[466] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[467] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(190), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
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
        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[467] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[467] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[468] = if ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[469] = if ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[468] != 0.0))) && (s.v[469] != 0.0)) {
            let assign4290_ad_e2951: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4290_ad_e2951, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[468] != 0.0))) && (!(s.v[469] != 0.0))) {
            let assign4300_ad_e3029: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4300_ad_e3029, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[470] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(190), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[471] = if ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[471] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[472] = if ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[471] != 0.0))) && (s.v[472] != 0.0)) {
            let assign4610_ad_e3552: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4610_ad_e3552, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[471] != 0.0))) && (!(s.v[472] != 0.0))) {
            let assign4620_ad_e3630: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4620_ad_e3630, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[473] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(190), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[474] = if ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[475] = if ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[474] != 0.0))) && (s.v[475] != 0.0)) {
            let assign4930_ad_e4153: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4930_ad_e4153, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[474] != 0.0))) && (!(s.v[475] != 0.0))) {
            let assign4940_ad_e4231: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4940_ad_e4231, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(190), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[476] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
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
        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[476] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[476] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[476] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[477] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[477] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[478] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[477] != 0.0))) && (s.v[478] != 0.0)) {
            let assign5300_ad_e4861: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign5300_ad_e4861, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            let assign5310_ad_e4940: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign5310_ad_e4940, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[479] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[479] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[479] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[479] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[480] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[480] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[481] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[480] != 0.0))) && (s.v[481] != 0.0)) {
            let assign5680_ad_e5596: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign5680_ad_e5596, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[480] != 0.0))) && (!(s.v[481] != 0.0))) {
            let assign5690_ad_e5675: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign5690_ad_e5675, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[482] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[482] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[482] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[482] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[483] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[483] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[484] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[483] != 0.0))) && (s.v[484] != 0.0)) {
            let assign6060_ad_e6331: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign6060_ad_e6331, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[483] != 0.0))) && (!(s.v[484] != 0.0))) {
            let assign6070_ad_e6410: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign6070_ad_e6410, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[485] = if (s.v[190] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[485] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 190);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(190), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(432)), A::sub(s.ad_value(190), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(190), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(190), s.ad_value(267)), A::sub(s.ad_value(190), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(190), A::sqrt(A::offset(A::mul(s.ad_value(190), s.ad_value(190)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[486] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[486] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[486] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[486] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[487] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[487] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[487] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(190), s.ad_value(428))));
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
        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[488] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[489] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (s.v[489] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (!(s.v[489] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[490] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (s.v[490] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (!(s.v[490] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[491] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[491] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[492] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[492] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[492] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[493] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[493] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[493] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[494] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[494] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[494] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[495] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[495] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[496] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[495] != 0.0))) && (s.v[496] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[495] != 0.0))) && (!(s.v[496] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[497] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[497] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[498] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (s.v[498] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[499] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (s.v[499] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[500] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (!(s.v[499] != 0.0))) && (s.v[500] != 0.0)) {
            let assign6980_ad_e7644: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign6980_ad_e7644);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (!(s.v[499] != 0.0))) && (!(s.v[500] != 0.0))) {
            let assign6990_ad_e7692: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign6990_ad_e7692);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[501] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[501] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[502] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[503] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (s.v[502] != 0.0)) && (s.v[503] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (s.v[502] != 0.0)) && (!(s.v[503] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (s.v[502] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (!(s.v[502] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[504] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[504] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[504] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[504] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[505] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[505] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[505] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(190), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[506] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[507] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (s.v[507] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (!(s.v[507] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[508] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (s.v[508] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (!(s.v[508] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[509] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[509] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[510] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[510] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[510] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[511] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[511] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[511] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[512] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[512] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[513] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[513] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[514] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[513] != 0.0))) && (s.v[514] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[513] != 0.0))) && (!(s.v[514] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[513] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[515] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[515] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[516] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[516] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[516] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[517] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[517] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[518] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[517] != 0.0))) && (s.v[518] != 0.0)) {
            let assign7790_ad_e8800: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign7790_ad_e8800);
        }

    }

    pub(super) fn stamp_transient_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[517] != 0.0))) && (!(s.v[518] != 0.0))) {
            let assign7800_ad_e8848: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign7800_ad_e8848);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[519] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[519] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[520] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[521] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (s.v[520] != 0.0)) && (s.v[521] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (s.v[520] != 0.0)) && (!(s.v[521] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (s.v[520] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (!(s.v[520] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[522] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[522] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[522] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[522] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[523] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[523] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[523] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(190), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[524] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[525] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (s.v[525] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (!(s.v[525] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[526] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (s.v[526] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (!(s.v[526] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[527] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[527] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[528] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[528] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[528] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[529] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[529] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[529] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[530] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[530] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[530] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[531] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[531] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[532] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[531] != 0.0))) && (s.v[532] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[531] != 0.0))) && (!(s.v[532] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[531] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[533] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[533] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[534] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (s.v[534] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (!(s.v[534] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[535] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (s.v[535] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[536] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (!(s.v[535] != 0.0))) && (s.v[536] != 0.0)) {
            let assign8600_ad_e9956: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign8600_ad_e9956);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (!(s.v[535] != 0.0))) && (!(s.v[536] != 0.0))) {
            let assign8610_ad_e10004: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign8610_ad_e10004);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[537] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[537] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[538] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[539] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (s.v[538] != 0.0)) && (s.v[539] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (s.v[538] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (s.v[538] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (!(s.v[538] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(180, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        s.v[540] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_add_ad_rhs(424, 191, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(191), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[541] = if (s.v[191] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[542] = if ((((0.5 * (s.v[191] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(191), (s.v[85] * 0.5)));
        }

        s.v[543] = if ((0.5 * (s.v[191] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[542] != 0.0))) && (s.v[543] != 0.0)) {
            let assign8860_ad_e10343: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign8860_ad_e10343);
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[542] != 0.0))) && (!(s.v[543] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[544] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(191), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[544] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

    }

    pub(super) fn stamp_transient_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[544] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[545] = if ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[546] = if ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[545] != 0.0))) && (s.v[546] != 0.0)) {
            let assign9180_ad_e10909: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9180_ad_e10909, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[545] != 0.0))) && (!(s.v[546] != 0.0))) {
            let assign9190_ad_e10987: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9190_ad_e10987, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[547] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(191), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[547] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[547] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[548] = if ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[548] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[549] = if ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[548] != 0.0))) && (s.v[549] != 0.0)) {
            let assign9500_ad_e11510: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9500_ad_e11510, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[548] != 0.0))) && (!(s.v[549] != 0.0))) {
            let assign9510_ad_e11588: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9510_ad_e11588, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[550] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(191), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[551] = if ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[551] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[552] = if ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[551] != 0.0))) && (s.v[552] != 0.0)) {
            let assign9820_ad_e12111: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9820_ad_e12111, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[551] != 0.0))) && (!(s.v[552] != 0.0))) {
            let assign9830_ad_e12189: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9830_ad_e12189, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(191), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[553] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[553] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[553] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[553] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[554] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[554] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[555] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[554] != 0.0))) && (s.v[555] != 0.0)) {
            let assign10190_ad_e12819: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10190_ad_e12819, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[554] != 0.0))) && (!(s.v[555] != 0.0))) {
            let assign10200_ad_e12898: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10200_ad_e12898, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[556] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

    }

    pub(super) fn stamp_transient_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[556] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[556] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[556] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[557] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[557] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[558] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[557] != 0.0))) && (s.v[558] != 0.0)) {
            let assign10570_ad_e13554: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10570_ad_e13554, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[557] != 0.0))) && (!(s.v[558] != 0.0))) {
            let assign10580_ad_e13633: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10580_ad_e13633, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[559] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[559] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[559] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[559] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[560] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[560] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[561] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[560] != 0.0))) && (s.v[561] != 0.0)) {
            let assign10950_ad_e14289: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10950_ad_e14289, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[560] != 0.0))) && (!(s.v[561] != 0.0))) {
            let assign10960_ad_e14368: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10960_ad_e14368, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[562] = if (s.v[191] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[562] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[562] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 191);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(191), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(432)), A::sub(s.ad_value(191), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(191), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(191), s.ad_value(267)), A::sub(s.ad_value(191), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(191), A::sqrt(A::offset(A::mul(s.ad_value(191), s.ad_value(191)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[563] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[563] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[563] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[563] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[564] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[564] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[564] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(191), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[565] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[566] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (s.v[566] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (!(s.v[566] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[567] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (s.v[567] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (!(s.v[567] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[568] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[568] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

    }

    pub(super) fn stamp_transient_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[569] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[569] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[569] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[570] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[570] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[570] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[571] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[571] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[571] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[572] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[572] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[573] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[572] != 0.0))) && (s.v[573] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[572] != 0.0))) && (!(s.v[573] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[572] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[574] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[574] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[575] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (s.v[575] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (!(s.v[575] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[576] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (s.v[576] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[577] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (!(s.v[576] != 0.0))) && (s.v[577] != 0.0)) {
            let assign11870_ad_e15602: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign11870_ad_e15602);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (!(s.v[576] != 0.0))) && (!(s.v[577] != 0.0))) {
            let assign11880_ad_e15650: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign11880_ad_e15650);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[578] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[578] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[579] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[580] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (s.v[579] != 0.0)) && (s.v[580] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (s.v[579] != 0.0)) && (!(s.v[580] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (s.v[579] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (!(s.v[579] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[581] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[581] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[581] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[581] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[582] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[582] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[582] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(191), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[583] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[584] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (s.v[584] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (!(s.v[584] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[585] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (s.v[585] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (!(s.v[585] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[586] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[586] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[587] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[587] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[587] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[588] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[588] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[588] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[589] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[589] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[589] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[590] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[590] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[591] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[590] != 0.0))) && (s.v[591] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[590] != 0.0))) && (!(s.v[591] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[590] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[592] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[592] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[593] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (s.v[593] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (!(s.v[593] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[594] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (s.v[594] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[595] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (!(s.v[594] != 0.0))) && (s.v[595] != 0.0)) {
            let assign12680_ad_e16758: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign12680_ad_e16758);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (!(s.v[594] != 0.0))) && (!(s.v[595] != 0.0))) {
            let assign12690_ad_e16806: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign12690_ad_e16806);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[596] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[596] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[597] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[598] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (s.v[597] != 0.0)) && (s.v[598] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (s.v[597] != 0.0)) && (!(s.v[598] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (s.v[597] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (!(s.v[597] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[599] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[599] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[599] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[599] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[600] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[600] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[600] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(191), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[601] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[602] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (s.v[602] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (!(s.v[602] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[603] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (s.v[603] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (!(s.v[603] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[604] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[604] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[605] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[605] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[605] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[606] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[606] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[606] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[607] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[607] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[607] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[608] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[608] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[609] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[608] != 0.0))) && (s.v[609] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[608] != 0.0))) && (!(s.v[609] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[608] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[610] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[610] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[611] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (s.v[611] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[612] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (s.v[612] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[613] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (!(s.v[612] != 0.0))) && (s.v[613] != 0.0)) {
            let assign13490_ad_e17914: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign13490_ad_e17914);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (!(s.v[612] != 0.0))) && (!(s.v[613] != 0.0))) {
            let assign13500_ad_e17962: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign13500_ad_e17962);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[614] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[614] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[615] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[616] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) && (s.v[616] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) && (!(s.v[616] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[615] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(181, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        s.v[617] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_add_ad_rhs(424, 192, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(192), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[618] = if (s.v[192] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[619] = if ((((0.5 * (s.v[192] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[619] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(192), (s.v[85] * 0.5)));
        }

        s.v[620] = if ((0.5 * (s.v[192] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[619] != 0.0))) && (s.v[620] != 0.0)) {
            let assign13750_ad_e18301: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign13750_ad_e18301);
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[619] != 0.0))) && (!(s.v[620] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[621] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(192), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[621] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[621] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[622] = if ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[622] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[623] = if ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[622] != 0.0))) && (s.v[623] != 0.0)) {
            let assign14070_ad_e18867: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14070_ad_e18867, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[622] != 0.0))) && (!(s.v[623] != 0.0))) {
            let assign14080_ad_e18945: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14080_ad_e18945, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[624] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(192), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[624] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[624] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[625] = if ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[625] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[626] = if ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[625] != 0.0))) && (s.v[626] != 0.0)) {
            let assign14390_ad_e19468: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14390_ad_e19468, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[625] != 0.0))) && (!(s.v[626] != 0.0))) {
            let assign14400_ad_e19546: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14400_ad_e19546, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[627] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(192), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[627] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[627] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[628] = if ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[628] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[629] = if ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[629] != 0.0)) {
            let assign14710_ad_e20069: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14710_ad_e20069, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            let assign14720_ad_e20147: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14720_ad_e20147, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(192), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[630] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[630] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[630] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[630] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[631] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[631] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[632] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[631] != 0.0))) && (s.v[632] != 0.0)) {
            let assign15080_ad_e20777: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15080_ad_e20777, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[631] != 0.0))) && (!(s.v[632] != 0.0))) {
            let assign15090_ad_e20856: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15090_ad_e20856, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[633] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[633] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[633] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[633] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[634] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[634] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[635] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[634] != 0.0))) && (s.v[635] != 0.0)) {
            let assign15460_ad_e21512: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15460_ad_e21512, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[634] != 0.0))) && (!(s.v[635] != 0.0))) {
            let assign15470_ad_e21591: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15470_ad_e21591, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[636] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[637] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[637] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[638] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[637] != 0.0))) && (s.v[638] != 0.0)) {
            let assign15840_ad_e22247: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15840_ad_e22247, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[637] != 0.0))) && (!(s.v[638] != 0.0))) {
            let assign15850_ad_e22326: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15850_ad_e22326, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[639] = if (s.v[192] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[639] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[639] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 192);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(192), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(432)), A::sub(s.ad_value(192), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(192), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(192), s.ad_value(267)), A::sub(s.ad_value(192), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(192), A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(192)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[640] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[640] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[640] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[640] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[641] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[641] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[641] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(192), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[642] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[643] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[643] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (!(s.v[643] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[644] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[644] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (!(s.v[644] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[645] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[645] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[646] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[646] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[646] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[647] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[647] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[648] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[648] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[648] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[649] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[649] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[650] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[649] != 0.0))) && (s.v[650] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[649] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[649] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[651] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[651] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[652] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (s.v[652] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (!(s.v[652] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[653] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (s.v[653] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[654] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (!(s.v[653] != 0.0))) && (s.v[654] != 0.0)) {
            let assign16760_ad_e23560: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign16760_ad_e23560);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (!(s.v[653] != 0.0))) && (!(s.v[654] != 0.0))) {
            let assign16770_ad_e23608: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign16770_ad_e23608);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[655] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[655] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[656] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[657] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (s.v[656] != 0.0)) && (s.v[657] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (s.v[656] != 0.0)) && (!(s.v[657] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (s.v[656] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (!(s.v[656] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[658] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[659] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[659] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[659] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(192), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[660] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[661] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[661] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (!(s.v[661] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[662] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[662] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (!(s.v[662] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[663] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[664] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[664] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[664] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[665] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[665] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[665] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[666] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[666] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[666] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[667] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[667] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[668] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[667] != 0.0))) && (s.v[668] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[667] != 0.0))) && (!(s.v[668] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[667] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[669] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[669] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[670] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (s.v[670] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (!(s.v[670] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[671] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (s.v[671] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[672] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (!(s.v[671] != 0.0))) && (s.v[672] != 0.0)) {
            let assign17570_ad_e24716: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign17570_ad_e24716);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (!(s.v[671] != 0.0))) && (!(s.v[672] != 0.0))) {
            let assign17580_ad_e24764: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign17580_ad_e24764);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[673] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[673] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[674] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[675] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (s.v[674] != 0.0)) && (s.v[675] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (s.v[674] != 0.0)) && (!(s.v[675] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (s.v[674] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (!(s.v[674] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[676] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[676] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[676] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[676] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[677] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[677] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[677] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(192), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[678] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[679] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (s.v[679] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (!(s.v[679] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[680] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (s.v[680] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (!(s.v[680] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[681] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[681] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[682] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[682] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[682] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[683] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[683] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[683] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[684] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[684] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[684] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[685] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[685] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[686] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[685] != 0.0))) && (s.v[686] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[685] != 0.0))) && (!(s.v[686] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[685] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[687] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[687] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[688] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (s.v[688] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (!(s.v[688] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[689] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (s.v[689] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[690] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (!(s.v[689] != 0.0))) && (s.v[690] != 0.0)) {
            let assign18380_ad_e25872: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign18380_ad_e25872);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (!(s.v[689] != 0.0))) && (!(s.v[690] != 0.0))) {
            let assign18390_ad_e25920: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign18390_ad_e25920);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[691] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[691] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[692] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[693] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) && (s.v[693] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) && (!(s.v[693] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (!(s.v[692] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(182, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        s.v[694] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_add_ad_rhs(424, 193, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(193), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[695] = if (s.v[193] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[696] = if ((((0.5 * (s.v[193] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[696] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(193), (s.v[85] * 0.5)));
        }

        s.v[697] = if ((0.5 * (s.v[193] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[696] != 0.0))) && (s.v[697] != 0.0)) {
            let assign18640_ad_e26259: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign18640_ad_e26259);
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[696] != 0.0))) && (!(s.v[697] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[698] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(193), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[698] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[698] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[699] = if ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[699] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[700] = if ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[699] != 0.0))) && (s.v[700] != 0.0)) {
            let assign18960_ad_e26825: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign18960_ad_e26825, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[699] != 0.0))) && (!(s.v[700] != 0.0))) {
            let assign18970_ad_e26903: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign18970_ad_e26903, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[701] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(193), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[701] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[701] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[702] = if ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[702] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[703] = if ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[702] != 0.0))) && (s.v[703] != 0.0)) {
            let assign19280_ad_e27426: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19280_ad_e27426, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[702] != 0.0))) && (!(s.v[703] != 0.0))) {
            let assign19290_ad_e27504: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19290_ad_e27504, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[704] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(193), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[704] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[704] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[705] = if ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[705] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[706] = if ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[705] != 0.0))) && (s.v[706] != 0.0)) {
            let assign19600_ad_e28027: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19600_ad_e28027, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[705] != 0.0))) && (!(s.v[706] != 0.0))) {
            let assign19610_ad_e28105: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19610_ad_e28105, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(193), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[707] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[707] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[707] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[707] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[708] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[708] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[709] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[708] != 0.0))) && (s.v[709] != 0.0)) {
            let assign19970_ad_e28735: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19970_ad_e28735, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[708] != 0.0))) && (!(s.v[709] != 0.0))) {
            let assign19980_ad_e28814: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19980_ad_e28814, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[710] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[710] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[710] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[710] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[711] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[711] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[712] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[711] != 0.0))) && (s.v[712] != 0.0)) {
            let assign20350_ad_e29470: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign20350_ad_e29470, 0.5), 1.0)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[711] != 0.0))) && (!(s.v[712] != 0.0))) {
            let assign20360_ad_e29549: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign20360_ad_e29549, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[713] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[713] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[713] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[713] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[714] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[714] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[715] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[714] != 0.0))) && (s.v[715] != 0.0)) {
            let assign20730_ad_e30205: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign20730_ad_e30205, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[714] != 0.0))) && (!(s.v[715] != 0.0))) {
            let assign20740_ad_e30284: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign20740_ad_e30284, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[716] = if (s.v[193] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[716] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[716] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 193);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(193), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(432)), A::sub(s.ad_value(193), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(193), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(193), s.ad_value(267)), A::sub(s.ad_value(193), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(193), A::sqrt(A::offset(A::mul(s.ad_value(193), s.ad_value(193)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[717] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[717] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[717] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[717] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[718] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[718] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[718] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(193), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[719] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[720] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (s.v[720] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (!(s.v[720] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[721] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (s.v[721] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (!(s.v[721] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[722] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[722] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[723] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[723] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[723] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[724] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[724] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[724] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[725] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[725] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[725] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[726] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[726] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[727] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[726] != 0.0))) && (s.v[727] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[726] != 0.0))) && (!(s.v[727] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[726] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[728] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[728] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[729] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (s.v[729] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (!(s.v[729] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[730] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (s.v[730] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[731] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (!(s.v[730] != 0.0))) && (s.v[731] != 0.0)) {
            let assign21650_ad_e31518: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign21650_ad_e31518);
        }

    }
}
