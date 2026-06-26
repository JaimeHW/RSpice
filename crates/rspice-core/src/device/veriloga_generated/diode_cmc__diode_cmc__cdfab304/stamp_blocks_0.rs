#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[0] = (8.8541878176e-12 * 11.8);

        s.v[1] = (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) });

        s.b[388] = ((!param_given[6]) && param_given[96]);
        s.v[388] = if s.b[388] { 1.0 } else { 0.0 };

        if s.b[388] {
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

        s.b[389] = (p.p81 > 0.5);
        s.v[389] = if s.b[389] { 1.0 } else { 0.0 };

        if s.b[389] {
            s.store_scalar(45, 1.0);
        }

        if (!s.b[389]) {
            s.store_scalar(45, 0.0);
        }

        s.v[46] = (if (p.p82 > 0.5) { p.p82 } else { 0.5 });

        s.v[47] = (if (p.p83 > 0.0) { p.p83 } else { 0.0 });

        s.store_offset(78, 1, 273.15);

        s.v[79] = ((ctx_temp + p.p102)).max((273.15 + (-250.0)));

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

        s.store_scaled_square(101, 176, s.v[15]);

        s.store_scaled_square(102, 177, s.v[16]);

        s.store_scaled_square(103, 178, s.v[17]);

        s.store_sub_scaled_ad_rhs(104, 80, s.v[6], A::scale(A::ln(s.ad_value(98)), (2.0 * s.v[84])));

        s.store_sub_scaled_ad_rhs(105, 80, s.v[7], A::scale(A::ln(s.ad_value(99)), (2.0 * s.v[84])));

        s.store_sub_scaled_ad_rhs(106, 80, s.v[8], A::scale(A::ln(s.ad_value(100)), (2.0 * s.v[84])));

        s.store_add_ad_rhs(107, 104, A::scale(A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(104)), s.v[85])), s.v[84]));

        s.store_add_ad_rhs(108, 105, A::scale(A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(105)), s.v[85])), s.v[84]));

        s.store_add_ad_rhs(109, 106, A::scale(A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(106)), s.v[85])), s.v[84]));

        s.store_div_from_scalar(119, 1.0, 107);

        s.store_div_from_scalar(120, 1.0, 108);

        s.store_div_from_scalar(121, 1.0, 109);

        s.v[122] = (1.0 - s.v[9]);

        s.v[123] = (1.0 - s.v[10]);

        s.v[124] = (1.0 - s.v[11]);

        s.v[125] = (1.0 / s.v[122]);

        s.v[126] = (1.0 / s.v[123]);

        s.v[127] = (1.0 / s.v[124]);

        s.store_scaled_powf_ad(128, A::scale(s.ad_value(119), s.v[6]), s.v[9], s.v[3]);

        s.store_scaled_powf_ad(129, A::scale(s.ad_value(120), s.v[7]), s.v[10], s.v[4]);

        s.store_scaled_powf_ad(130, A::scale(s.ad_value(121), s.v[8]), s.v[11], s.v[5]);

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

        if (!(s.v[155] > 0.0)) {
            s.store_scalar(155, 0.0);
        }

        if (!(s.v[156] > 0.0)) {
            s.store_scalar(156, 0.0);
        }

        if (!(s.v[157] > 0.0)) {
            s.store_scalar(157, 0.0);
        }

        s.v[158] = ((s.v[44] - 1.0) / s.v[44]);

        s.v[159] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[41])));

        s.v[160] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[42])));

        s.v[161] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[43])));

        s.store_scaled_offset_ad(38, A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[57]), s.v[56])), 1.0, s.v[38]);

        s.store_scaled_offset_ad(39, A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[59]), s.v[58])), 1.0, s.v[39]);

        s.store_scaled_offset_ad(40, A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[61]), s.v[60])), 1.0, s.v[40]);

        s.b[390] = (s.v[38] <= 0.1);
        s.v[390] = if s.b[390] { 1.0 } else { 0.0 };

        if s.b[390] {
            s.store_scalar(38, 0.1);
            s.store_scalar(162, 10.0);
        }

        if (!s.b[390]) {
            s.store_div_from_scalar(162, 1.0, 38);
        }

        s.b[391] = (s.v[39] <= 0.1);
        s.v[391] = if s.b[391] { 1.0 } else { 0.0 };

        if s.b[391] {
            s.store_scalar(39, 0.1);
            s.store_scalar(163, 10.0);
        }

        if (!s.b[391]) {
            s.store_div_from_scalar(163, 1.0, 39);
        }

        s.b[392] = (s.v[40] <= 0.1);
        s.v[392] = if s.b[392] { 1.0 } else { 0.0 };

        if s.b[392] {
            s.store_scalar(40, 0.1);
            s.store_scalar(164, 10.0);
        }

        if (!s.b[392]) {
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

        s.store_scaled_square(319, 318, 1.0 / (s.v[307]));

        s.store_powf(316, 80, (-1.5));

        s.store_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));

        s.store_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));

        s.store_div_ad(322, A::mul(A::scale(s.ad_value(320), 2.0), s.ad_value(321)), A::add(s.ad_value(320), s.ad_value(321)));

        s.store_powf(317, 80, p.p97);

        s.store_scale(324, 317, p.p93);

        s.store_sqrt_mul(323, 324, 322);

        s.store_scaled_ln_ad(347, A::div_from_scalar(s.v[307], s.ad_value(319)), (s.v[313] / s.v[85]));

        s.store_scaled_add_ad(348, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323)), (s.v[313] / s.v[85]));

        s.v[256] = (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]);

        s.v[257] = (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[258] = (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[263] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.b[393] = ((s.v[101] * s.v[256]) > 0.0);
        s.v[393] = if s.b[393] { 1.0 } else { 0.0 };

        if s.b[393] {
            s.store_scaled_ln_ad(168, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0), (s.v[84] * s.v[62]));
        }

        if (!s.b[393]) {
            s.store_scalar(168, 100000000.0);
        }

        s.b[394] = ((s.v[102] * s.v[257]) > 0.0);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if s.b[394] {
            s.store_scaled_ln_ad(169, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0), (s.v[84] * s.v[64]));
        }

        if (!s.b[394]) {
            s.store_scalar(169, 100000000.0);
        }

        s.b[395] = ((s.v[103] * s.v[258]) > 0.0);
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if s.b[395] {
            s.store_scaled_ln_ad(170, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0), (s.v[84] * s.v[63]));
        }

        if (!s.b[395]) {
            s.store_scalar(170, 100000000.0);
        }

        s.store_min3(262, 168, 169, 170);

        s.b[396] = ((((s.v[262] * s.v[85])) as f64).abs() < 230.25850929940458);
        s.v[396] = if s.b[396] { 1.0 } else { 0.0 };

        if s.b[396] {
            s.store_exp_scaled_input(263, 262, s.v[85]);
        }

        s.b[397] = ((s.v[262] * s.v[85]) < (-230.25850929940458));
        s.v[397] = if s.b[397] { 1.0 } else { 0.0 };

        if ((!s.b[396]) && s.b[397]) {
            s.store_div_from_scalar_offset_ad(263, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((!s.b[396]) && (!s.b[397])) {
            s.store_scaled_offset_ad(263, A::mul(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
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

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[398] = (s.v[256] == 0.0);
        s.v[398] = if s.b[398] { 1.0 } else { 0.0 };

        if s.b[398] {
            s.store_add(110, 108, 109);
            s.store_scalar(113, (0.9 * (s.v[10]).min(s.v[11])));
            s.store_scalar(116, (s.v[7] + s.v[8]));
        }

        s.b[399] = (s.v[257] == 0.0);
        s.v[399] = if s.b[399] { 1.0 } else { 0.0 };

        if s.b[399] {
            s.store_add(111, 107, 109);
            s.store_scalar(114, (0.9 * (s.v[9]).min(s.v[11])));
            s.store_scalar(117, (s.v[6] + s.v[8]));
        }

        s.b[400] = (s.v[258] == 0.0);
        s.v[400] = if s.b[400] { 1.0 } else { 0.0 };

        if s.b[400] {
            s.store_add(112, 107, 108);
            s.store_scalar(115, (0.9 * (s.v[9]).min(s.v[10])));
            s.store_scalar(118, (s.v[6] + s.v[7]));
        }

        s.store_min3(264, 110, 111, 112);

        s.store_scale(265, 264, 0.1);

        s.store_max3(91, 113, 114, 115);

        s.store_mul_sub_from_scalar_ad_rhs(266, 264, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(91))));

        s.store_offset_min_ad(267, A::min(s.ad_value(116), s.ad_value(117)), s.ad_value(118), (-0.05));

        s.store_add_scaled_ad_lhs(289, A::add(A::scale(s.ad_value(101), s.v[256]), A::scale(s.ad_value(102), s.v[257])), 103, s.v[258]);

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

        s.b[409] = ((s.v[256] * s.v[173]) > 0.0);
        s.v[409] = if s.b[409] { 1.0 } else { 0.0 };

        if s.b[409] {
            s.store_div_from_scalar(285, s.v[256], 173);
        }

        s.b[410] = ((s.v[257] * s.v[174]) > 0.0);
        s.v[410] = if s.b[410] { 1.0 } else { 0.0 };

        if s.b[410] {
            s.store_add_ad_lhs(285, A::div_from_scalar(s.v[257], s.ad_value(174)), 285);
        }

        s.b[411] = ((s.v[258] * s.v[175]) > 0.0);
        s.v[411] = if s.b[411] { 1.0 } else { 0.0 };

        if s.b[411] {
            s.store_add_ad_lhs(285, A::div_from_scalar(s.v[258], s.ad_value(175)), 285);
        }

        s.b[412] = (s.v[285] > 0.0);
        s.v[412] = if s.b[412] { 1.0 } else { 0.0 };

        if s.b[412] {
            s.store_add_ad_lhs(171, A::div_from_scalar(1.0, s.ad_value(285)), 172);
        }

        if (!s.b[412]) {
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

        if (!(s.v[315] > 0.0)) {
            s.store_scalar(315, (-s.v[315]));
        }

        s.store_sqrt_offset_input(315, 315, (s.v[314] * s.v[314]));

        s.store_sub_from_scalar_ad(343, p.p94, A::scale(A::offset(s.ad_value(315), s.v[314]), 0.5));

        s.b[413] = (s.v[45] > 0.9);
        s.v[413] = if s.b[413] { 1.0 } else { 0.0 };

        s.b[414] = ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0)));
        s.v[414] = if s.b[414] { 1.0 } else { 0.0 };

        if (s.b[413] && s.b[414]) {
            s.store_scalar(45, 0.0);
        }

        s.b[415] = (s.v[256] > 0.0);
        s.v[415] = if s.b[415] { 1.0 } else { 0.0 };

        if ((s.b[413] && (!s.b[414])) && s.b[415]) {
            s.store_scalar(301, s.v[62]);
        }

        s.b[416] = (s.v[258] > 0.0);
        s.v[416] = if s.b[416] { 1.0 } else { 0.0 };

        if ((s.b[413] && (!s.b[414])) && s.b[416]) {
            s.store_scalar(301, s.v[63]);
        }

        s.b[417] = (s.v[257] > 0.0);
        s.v[417] = if s.b[417] { 1.0 } else { 0.0 };

        if ((s.b[413] && (!s.b[414])) && s.b[417]) {
            s.store_scalar(301, s.v[64]);
        }

        s.b[418] = (s.v[45] == 1.0);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

        if s.b[418] {
            s.store_scalar(419, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(421, 0.0);
            s.store_scalar(422, 0.0);
            s.store_scalar(423, 0.0);
            s.store_scalar(424, 0.0);
            s.store_scalar(425, 0.0);
            s.store_scalar(426, 0.0);
            s.store_scalar(427, 0.0);
            s.store_scalar(277, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(429, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(432, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
            s.store_scalar(436, 0.0);
            s.store_scalar(437, 0.0);
            s.store_scalar(438, 0.0);
            s.store_scalar(439, 0.0);
            s.store_scalar(440, 0.0);
            s.store_scalar(441, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(448, 0.0);
            s.store_scalar(449, 0.0);
            s.store_scalar(450, 0.0);
            s.store_scalar(451, 0.0);
            s.store_scalar(452, 0.0);
            s.store_scalar(453, 0.0);
            s.store_scalar(454, 0.0);
            s.store_scalar(455, 0.0);
            s.store_scalar(456, 0.0);
            s.store_scalar(457, 0.0);
            s.store_scalar(458, 0.0);
            s.store_scalar(459, 0.0);
            s.store_scalar(460, 0.0);
            s.store_scalar(461, 0.0);
            s.store_scalar(462, 0.0);
            s.store_scalar(205, 0.4);
            s.store_scalar(206, 0.65);
            s.store_scalar(207, 0.8);
            s.store_scale(190, 205, (-s.v[46]));
            s.store_scale(191, 206, (-s.v[46]));
            s.store_scale(192, 207, (-s.v[46]));
            s.store_scalar(193, 0.1);
            s.store_scalar(194, 0.2);
        }

        s.b[463] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[463]) {
            s.store_scaled_square(422, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_ad_rhs(424, 190, A::mul(s.ad_value(265), s.ad_value(423)));
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_scaled_div_ad(428, A::mul(s.ad_value(190), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427)), 2.0);
        }

        s.b[464] = (s.v[190] < s.v[262]);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        s.b[465] = ((((0.5 * (s.v[190] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[465]) {
            s.store_exp_scaled_input(430, 190, (s.v[85] * 0.5));
        }

        s.b[466] = ((0.5 * (s.v[190] * s.v[85])) < (-230.25850929940458));
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && s.b[466]) {
            let assign3970_ad_e2385: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(430, assign3970_ad_e2385);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && (!s.b[466])) {
            s.store_scaled_offset_ad(430, A::mul(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[467] = (s.v[62] < p.p85);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_offset_scaled_sub(360, 190, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[467])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[468] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[468]) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[469] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && s.b[469]) {
            let assign4290_ad_e2951: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4290_ad_e2951, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && (!s.b[469])) {
            let assign4300_ad_e3029: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(370, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4300_ad_e3029, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[470] = (s.v[64] < p.p85);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_offset_scaled_sub(360, 190, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[470])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[471] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[471]) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[472] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && s.b[472]) {
            let assign4610_ad_e3552: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4610_ad_e3552, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && (!s.b[472])) {
            let assign4620_ad_e3630: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(371, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4620_ad_e3630, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && s.b[464]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[473] = (s.v[63] < p.p85);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_offset_scaled_sub(360, 190, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[473])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[474] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[474]) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[475] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && s.b[475]) {
            let assign4930_ad_e4153: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4930_ad_e4153, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && (!s.b[475])) {
            let assign4940_ad_e4231: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(372, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4940_ad_e4231, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_sqrt_mul_ad(430, A::offset(A::scale(A::sub(s.ad_value(190), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[476] = (s.v[62] < p.p85);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[476])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[477] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[477]) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[478] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && s.b[478]) {
            let assign5300_ad_e4861: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign5300_ad_e4861, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && (!s.b[478])) {
            let assign5310_ad_e4940: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(281, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign5310_ad_e4940, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(370, A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[479] = (s.v[64] < p.p85);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[479])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[480] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[480]) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[481] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && s.b[481]) {
            let assign5680_ad_e5596: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign5680_ad_e5596, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && (!s.b[481])) {
            let assign5690_ad_e5675: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(282, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign5690_ad_e5675, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(371, A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[482] = (s.v[63] < p.p85);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[482])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[483] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[483]) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[484] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && s.b[484]) {
            let assign6060_ad_e6331: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign6060_ad_e6331, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && (!s.b[484])) {
            let assign6070_ad_e6410: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(283, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign6070_ad_e6410, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[463]) && (!s.b[464])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(372, A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 283);
        }

        if (s.b[418] && s.b[463]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[485] = (s.v[190] > 0.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[463]) && s.b[485]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[463]) && (!s.b[485])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 190);
        }

        if (s.b[418] && s.b[463]) {
            s.store_sub(432, 264, 431);
            s.store_scaled_sub_ad(433, A::add(s.ad_value(190), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(432)), A::sub(s.ad_value(190), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), 0.5);
            s.store_scaled_sub_ad(434, A::add(s.ad_value(190), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(190), s.ad_value(267)), A::sub(s.ad_value(190), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82)))), 0.5);
            s.store_scaled_sub_ad_rhs(435, 190, A::sqrt(A::offset(A::mul(s.ad_value(190), s.ad_value(190)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[463])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(428, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[418] && (!s.b[463])) {
            s.store_scalar(430, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[486] = (s.v[256] == 0.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[486]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[487] = (s.v[122] == 0.5);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[487]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[487])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[486])) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(190), s.ad_value(428))));
            s.store_mul(437, 101, 370);
        }

        s.b[488] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[488]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[489] = (s.v[9] == 0.5);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && s.b[489]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && (!s.b[489])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {
            s.store_add(442, 440, 441);
        }

        s.b[490] = (s.v[9] == 0.5);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && s.b[490]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && (!s.b[490])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_rhs(444, 98, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[491] = (s.v[23] == 0.0);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[491]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_scaled_div(446, 443, 439, ((s.v[122]) * (s.v[152])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[492] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[492]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[492])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[493] = (s.v[457] > 0.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[493]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[493])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[494] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[494]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[494])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[495] = (s.v[457] > 0.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[495]) {
            s.copy_ad(458, 421);
        }

        s.b[496] = (s.v[456] > (-230.25850929940458));
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) && s.b[496]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) && (!s.b[496])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {
            s.store_scaled_div(459, 458, 454, ((s.v[149]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[23]);
        }

        s.b[497] = (s.v[29] == 0.0);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[497]) {
            s.store_scalar(460, 0.0);
        }

        s.b[498] = (s.v[9] == 0.5);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && s.b[498]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[498])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[497])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), 436, s.v[125]);
        }

        s.b[499] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && s.b[499]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.b[500] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[499])) && s.b[500]) {
            let assign6980_ad_e7644: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign6980_ad_e7644);
        }

        if ((((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[499])) && (!s.b[500])) {
            let assign6990_ad_e7692: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign6990_ad_e7692);
        }

        if ((s.b[418] && (!s.b[486])) && (!s.b[497])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[501] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[486])) && s.b[501]) {
            s.store_scalar(462, 1.0);
        }

        s.b[502] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        s.b[503] = (s.v[41] == 4.0);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) && s.b[503]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) && (!s.b[503])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[486])) && (!s.b[501])) && (!s.b[502])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[486])) {
            s.store_mul_add_ad_lhs(268, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(291, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        s.b[504] = (s.v[257] == 0.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[504]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[505] = (s.v[123] == 0.5);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[505]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[505])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[504])) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(190), s.ad_value(428))));
            s.store_mul(437, 102, 371);
        }

        s.b[506] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[506]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[507] = (s.v[10] == 0.5);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && s.b[507]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && (!s.b[507])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {
            s.store_add(442, 440, 441);
        }

        s.b[508] = (s.v[10] == 0.5);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && s.b[508]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && (!s.b[508])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_rhs(444, 99, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[509] = (s.v[24] == 0.0);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[509]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_scaled_div(446, 443, 439, ((s.v[123]) * (s.v[153])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[510] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[510]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[510])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[511] = (s.v[457] > 0.0);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[511]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[511])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[512] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[512]) {
            s.store_exp_sub(436, 456, 419);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[512])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[513] = (s.v[457] > 0.0);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[513]) {
            s.copy_ad(458, 421);
        }

        s.b[514] = (s.v[456] > (-230.25850929940458));
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) && s.b[514]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) && (!s.b[514])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {
            s.store_scaled_div(459, 458, 454, ((s.v[150]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[24]);
        }

        s.b[515] = (s.v[30] == 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[515]) {
            s.store_scalar(460, 0.0);
        }

        s.b[516] = (s.v[10] == 0.5);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && s.b[516]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[516])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[515])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), 436, s.v[126]);
        }

        s.b[517] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && s.b[517]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.b[518] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[517])) && s.b[518]) {
            let assign7790_ad_e8800: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign7790_ad_e8800);
        }

        if ((((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[517])) && (!s.b[518])) {
            let assign7800_ad_e8848: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign7800_ad_e8848);
        }

        if ((s.b[418] && (!s.b[504])) && (!s.b[515])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[519] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[504])) && s.b[519]) {
            s.store_scalar(462, 1.0);
        }

        s.b[520] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        s.b[521] = (s.v[42] == 4.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) && s.b[521]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) && (!s.b[521])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[504])) && (!s.b[519])) && (!s.b[520])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[504])) {
            s.store_mul_add_ad_lhs(270, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(292, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        s.b[522] = (s.v[258] == 0.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[522]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[523] = (s.v[124] == 0.5);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[523]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[523])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[522])) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(190), s.ad_value(428))));
            s.store_mul(437, 103, 372);
        }

        s.b[524] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[524]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[525] = (s.v[11] == 0.5);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && s.b[525]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && (!s.b[525])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {
            s.store_add(442, 440, 441);
        }

        s.b[526] = (s.v[11] == 0.5);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && s.b[526]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[524])) && (!s.b[526])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[524])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_rhs(444, 100, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[527] = (s.v[25] == 0.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[527]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_scaled_div(446, 443, 439, ((s.v[124]) * (s.v[154])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[528] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[528]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[528])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[529] = (s.v[457] > 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[529]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[529])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[530] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[530]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[530])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[531] = (s.v[457] > 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && s.b[531]) {
            s.copy_ad(458, 421);
        }

        s.b[532] = (s.v[456] > (-230.25850929940458));
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) && s.b[532]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) && (!s.b[532])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[527])) && (!s.b[531])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[527])) {
            s.store_scaled_div(459, 458, 454, ((s.v[151]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[25]);
        }

        s.b[533] = (s.v[31] == 0.0);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[533]) {
            s.store_scalar(460, 0.0);
        }

        s.b[534] = (s.v[11] == 0.5);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && s.b[534]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[534])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[533])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), 436, s.v[127]);
        }

        s.b[535] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[522])) && (!s.b[533])) && s.b[535]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.b[536] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[535])) && s.b[536]) {
            let assign8600_ad_e9956: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign8600_ad_e9956);
        }

        if ((((s.b[418] && (!s.b[522])) && (!s.b[533])) && (!s.b[535])) && (!s.b[536])) {
            let assign8610_ad_e10004: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign8610_ad_e10004);
        }

        if ((s.b[418] && (!s.b[522])) && (!s.b[533])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[537] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[522])) && s.b[537]) {
            s.store_scalar(462, 1.0);
        }

        s.b[538] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        s.b[539] = (s.v[43] == 4.0);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) && s.b[539]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) && (!s.b[539])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[537])) && s.b[538]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[522])) && (!s.b[537])) && (!s.b[538])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[522])) {
            s.store_mul_add_ad_lhs(272, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(293, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        if s.b[418] {
            s.store_add_scaled_ad_lhs(180, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), 272, s.v[258]);
        }

        s.b[540] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[540]) {
            s.store_scaled_square(422, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_ad_rhs(424, 191, A::mul(s.ad_value(265), s.ad_value(423)));
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_scaled_div_ad(428, A::mul(s.ad_value(191), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427)), 2.0);
        }

        s.b[541] = (s.v[191] < s.v[262]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        s.b[542] = ((((0.5 * (s.v[191] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[542]) {
            s.store_exp_scaled_input(430, 191, (s.v[85] * 0.5));
        }

        s.b[543] = ((0.5 * (s.v[191] * s.v[85])) < (-230.25850929940458));
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[542])) && s.b[543]) {
            let assign8860_ad_e10343: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(430, assign8860_ad_e10343);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[542])) && (!s.b[543])) {
            s.store_scaled_offset_ad(430, A::mul(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[544] = (s.v[62] < p.p85);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_offset_scaled_sub(360, 191, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[544]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[544])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[545] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[545]) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[546] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && s.b[546]) {
            let assign9180_ad_e10909: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9180_ad_e10909, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[545])) && (!s.b[546])) {
            let assign9190_ad_e10987: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(370, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9190_ad_e10987, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[547] = (s.v[64] < p.p85);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_offset_scaled_sub(360, 191, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[547]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[547])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[548] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[548]) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[549] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[548])) && s.b[549]) {
            let assign9500_ad_e11510: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9500_ad_e11510, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[548])) && (!s.b[549])) {
            let assign9510_ad_e11588: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(371, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9510_ad_e11588, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && s.b[541]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[550] = (s.v[63] < p.p85);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_offset_scaled_sub(360, 191, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[550]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[550])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[551] = ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && s.b[541]) && s.b[551]) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[552] = ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[551])) && s.b[552]) {
            let assign9820_ad_e12111: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9820_ad_e12111, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[540]) && s.b[541]) && (!s.b[551])) && (!s.b[552])) {
            let assign9830_ad_e12189: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(372, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9830_ad_e12189, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_sqrt_mul_ad(430, A::offset(A::scale(A::sub(s.ad_value(191), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[553] = (s.v[62] < p.p85);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[553]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[553])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[554] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[554]) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[555] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && s.b[555]) {
            let assign10190_ad_e12819: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10190_ad_e12819, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[554])) && (!s.b[555])) {
            let assign10200_ad_e12898: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(281, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10200_ad_e12898, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(370, A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[556] = (s.v[64] < p.p85);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[556]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[556])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[557] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[557]) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[558] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[557])) && s.b[558]) {
            let assign10570_ad_e13554: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10570_ad_e13554, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[557])) && (!s.b[558])) {
            let assign10580_ad_e13633: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(282, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10580_ad_e13633, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(371, A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[559] = (s.v[63] < p.p85);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[559]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[559])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[560] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[540]) && (!s.b[541])) && s.b[560]) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[561] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[560])) && s.b[561]) {
            let assign10950_ad_e14289: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10950_ad_e14289, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[540]) && (!s.b[541])) && (!s.b[560])) && (!s.b[561])) {
            let assign10960_ad_e14368: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(283, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10960_ad_e14368, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[540]) && (!s.b[541])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(372, A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 283);
        }

        if (s.b[418] && s.b[540]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[562] = (s.v[191] > 0.0);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[540]) && s.b[562]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[540]) && (!s.b[562])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 191);
        }

        if (s.b[418] && s.b[540]) {
            s.store_sub(432, 264, 431);
            s.store_scaled_sub_ad(433, A::add(s.ad_value(191), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(432)), A::sub(s.ad_value(191), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), 0.5);
            s.store_scaled_sub_ad(434, A::add(s.ad_value(191), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(191), s.ad_value(267)), A::sub(s.ad_value(191), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82)))), 0.5);
            s.store_scaled_sub_ad_rhs(435, 191, A::sqrt(A::offset(A::mul(s.ad_value(191), s.ad_value(191)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[540])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(431, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[418] && (!s.b[540])) {
            s.store_scalar(428, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[563] = (s.v[256] == 0.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[563]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[564] = (s.v[122] == 0.5);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[564]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[564])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[563])) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(191), s.ad_value(428))));
            s.store_mul(437, 101, 370);
        }

        s.b[565] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[565]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[566] = (s.v[9] == 0.5);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && s.b[566]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && (!s.b[566])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {
            s.store_add(442, 440, 441);
        }

        s.b[567] = (s.v[9] == 0.5);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && s.b[567]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[565])) && (!s.b[567])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[565])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_rhs(444, 98, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[568] = (s.v[23] == 0.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[568]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_scaled_div(446, 443, 439, ((s.v[122]) * (s.v[152])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[569] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[569]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[569])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[570] = (s.v[457] > 0.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[570]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[570])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[571] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[571]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[571])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[572] = (s.v[457] > 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && s.b[572]) {
            s.copy_ad(458, 421);
        }

        s.b[573] = (s.v[456] > (-230.25850929940458));
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) && s.b[573]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) && (!s.b[573])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[568])) && (!s.b[572])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[568])) {
            s.store_scaled_div(459, 458, 454, ((s.v[149]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[23]);
        }

        s.b[574] = (s.v[29] == 0.0);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[574]) {
            s.store_scalar(460, 0.0);
        }

        s.b[575] = (s.v[9] == 0.5);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && s.b[575]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[575])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[574])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), 436, s.v[125]);
        }

        s.b[576] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[563])) && (!s.b[574])) && s.b[576]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.b[577] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[576])) && s.b[577]) {
            let assign11870_ad_e15602: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign11870_ad_e15602);
        }

        if ((((s.b[418] && (!s.b[563])) && (!s.b[574])) && (!s.b[576])) && (!s.b[577])) {
            let assign11880_ad_e15650: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign11880_ad_e15650);
        }

        if ((s.b[418] && (!s.b[563])) && (!s.b[574])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[578] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[563])) && s.b[578]) {
            s.store_scalar(462, 1.0);
        }

        s.b[579] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        s.b[580] = (s.v[41] == 4.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) && s.b[580]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) && (!s.b[580])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[578])) && s.b[579]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[563])) && (!s.b[578])) && (!s.b[579])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[563])) {
            s.store_mul_add_ad_lhs(268, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(291, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        s.b[581] = (s.v[257] == 0.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[581]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[582] = (s.v[123] == 0.5);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[582]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[582])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[581])) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(191), s.ad_value(428))));
            s.store_mul(437, 102, 371);
        }

        s.b[583] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[583]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[584] = (s.v[10] == 0.5);
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && s.b[584]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && (!s.b[584])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {
            s.store_add(442, 440, 441);
        }

        s.b[585] = (s.v[10] == 0.5);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && s.b[585]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[583])) && (!s.b[585])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[583])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_rhs(444, 99, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[586] = (s.v[24] == 0.0);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[586]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_scaled_div(446, 443, 439, ((s.v[123]) * (s.v[153])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[587] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[587]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[587])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[588] = (s.v[457] > 0.0);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[588]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[588])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[589] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[589]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[589])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[590] = (s.v[457] > 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && s.b[590]) {
            s.copy_ad(458, 421);
        }

        s.b[591] = (s.v[456] > (-230.25850929940458));
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) && s.b[591]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) && (!s.b[591])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[586])) && (!s.b[590])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[586])) {
            s.store_scaled_div(459, 458, 454, ((s.v[150]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[24]);
        }

        s.b[592] = (s.v[30] == 0.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[592]) {
            s.store_scalar(460, 0.0);
        }

        s.b[593] = (s.v[10] == 0.5);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && s.b[593]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[593])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[592])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), 436, s.v[126]);
        }

        s.b[594] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[581])) && (!s.b[592])) && s.b[594]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.b[595] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[594])) && s.b[595]) {
            let assign12680_ad_e16758: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign12680_ad_e16758);
        }

        if ((((s.b[418] && (!s.b[581])) && (!s.b[592])) && (!s.b[594])) && (!s.b[595])) {
            let assign12690_ad_e16806: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign12690_ad_e16806);
        }

        if ((s.b[418] && (!s.b[581])) && (!s.b[592])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[596] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[581])) && s.b[596]) {
            s.store_scalar(462, 1.0);
        }

        s.b[597] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        s.b[598] = (s.v[42] == 4.0);
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) && s.b[598]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) && (!s.b[598])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[596])) && s.b[597]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[581])) && (!s.b[596])) && (!s.b[597])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[581])) {
            s.store_mul_add_ad_lhs(270, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(292, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        s.b[599] = (s.v[258] == 0.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[599]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[600] = (s.v[124] == 0.5);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[600]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[600])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[599])) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(191), s.ad_value(428))));
            s.store_mul(437, 103, 372);
        }

        s.b[601] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[601]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[602] = (s.v[11] == 0.5);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && s.b[602]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && (!s.b[602])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {
            s.store_add(442, 440, 441);
        }

        s.b[603] = (s.v[11] == 0.5);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && s.b[603]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[601])) && (!s.b[603])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[601])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_rhs(444, 100, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[604] = (s.v[25] == 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[604]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_scaled_div(446, 443, 439, ((s.v[124]) * (s.v[154])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[605] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[605]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[605])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[606] = (s.v[457] > 0.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[606]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[606])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[607] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[607]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[607])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[608] = (s.v[457] > 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && s.b[608]) {
            s.copy_ad(458, 421);
        }

        s.b[609] = (s.v[456] > (-230.25850929940458));
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) && s.b[609]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) && (!s.b[609])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[604])) && (!s.b[608])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[604])) {
            s.store_scaled_div(459, 458, 454, ((s.v[151]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[25]);
        }

        s.b[610] = (s.v[31] == 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[610]) {
            s.store_scalar(460, 0.0);
        }

        s.b[611] = (s.v[11] == 0.5);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && s.b[611]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[611])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[610])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), 436, s.v[127]);
        }

        s.b[612] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[599])) && (!s.b[610])) && s.b[612]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.b[613] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[612])) && s.b[613]) {
            let assign13490_ad_e17914: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign13490_ad_e17914);
        }

        if ((((s.b[418] && (!s.b[599])) && (!s.b[610])) && (!s.b[612])) && (!s.b[613])) {
            let assign13500_ad_e17962: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign13500_ad_e17962);
        }

        if ((s.b[418] && (!s.b[599])) && (!s.b[610])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[614] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[599])) && s.b[614]) {
            s.store_scalar(462, 1.0);
        }

        s.b[615] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        s.b[616] = (s.v[43] == 4.0);
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) && s.b[616]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) && (!s.b[616])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[614])) && s.b[615]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[599])) && (!s.b[614])) && (!s.b[615])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[599])) {
            s.store_mul_add_ad_lhs(272, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(293, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        if s.b[418] {
            s.store_add_scaled_ad_lhs(181, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), 272, s.v[258]);
        }

        s.b[617] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[617]) {
            s.store_scaled_square(422, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_ad_rhs(424, 192, A::mul(s.ad_value(265), s.ad_value(423)));
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_scaled_div_ad(428, A::mul(s.ad_value(192), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427)), 2.0);
        }

        s.b[618] = (s.v[192] < s.v[262]);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        s.b[619] = ((((0.5 * (s.v[192] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[619]) {
            s.store_exp_scaled_input(430, 192, (s.v[85] * 0.5));
        }

        s.b[620] = ((0.5 * (s.v[192] * s.v[85])) < (-230.25850929940458));
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[619])) && s.b[620]) {
            let assign13750_ad_e18301: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(430, assign13750_ad_e18301);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[619])) && (!s.b[620])) {
            s.store_scaled_offset_ad(430, A::mul(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[621] = (s.v[62] < p.p85);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_offset_scaled_sub(360, 192, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[621]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[621])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[622] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[622]) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[623] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && s.b[623]) {
            let assign14070_ad_e18867: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14070_ad_e18867, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[622])) && (!s.b[623])) {
            let assign14080_ad_e18945: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(370, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14080_ad_e18945, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[624] = (s.v[64] < p.p85);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_offset_scaled_sub(360, 192, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[624]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[624])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[625] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[625]) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[626] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[625])) && s.b[626]) {
            let assign14390_ad_e19468: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14390_ad_e19468, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[625])) && (!s.b[626])) {
            let assign14400_ad_e19546: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(371, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14400_ad_e19546, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && s.b[618]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[627] = (s.v[63] < p.p85);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_offset_scaled_sub(360, 192, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[627]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[627])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[628] = ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && s.b[618]) && s.b[628]) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[629] = ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[628])) && s.b[629]) {
            let assign14710_ad_e20069: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14710_ad_e20069, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[617]) && s.b[618]) && (!s.b[628])) && (!s.b[629])) {
            let assign14720_ad_e20147: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(372, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14720_ad_e20147, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_sqrt_mul_ad(430, A::offset(A::scale(A::sub(s.ad_value(192), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[630] = (s.v[62] < p.p85);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[630]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[630])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[631] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[631]) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[632] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && s.b[632]) {
            let assign15080_ad_e20777: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15080_ad_e20777, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[631])) && (!s.b[632])) {
            let assign15090_ad_e20856: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(281, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15090_ad_e20856, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(370, A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[633] = (s.v[64] < p.p85);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[633]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[633])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[634] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[634]) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[635] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[634])) && s.b[635]) {
            let assign15460_ad_e21512: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15460_ad_e21512, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[634])) && (!s.b[635])) {
            let assign15470_ad_e21591: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(282, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15470_ad_e21591, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(371, A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[636] = (s.v[63] < p.p85);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[636]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[636])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[637] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[617]) && (!s.b[618])) && s.b[637]) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[638] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[637])) && s.b[638]) {
            let assign15840_ad_e22247: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15840_ad_e22247, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[617]) && (!s.b[618])) && (!s.b[637])) && (!s.b[638])) {
            let assign15850_ad_e22326: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(283, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15850_ad_e22326, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[617]) && (!s.b[618])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(372, A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 283);
        }

        if (s.b[418] && s.b[617]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[639] = (s.v[192] > 0.0);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[617]) && s.b[639]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[617]) && (!s.b[639])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 192);
        }

        if (s.b[418] && s.b[617]) {
            s.store_sub(432, 264, 431);
            s.store_scaled_sub_ad(433, A::add(s.ad_value(192), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(432)), A::sub(s.ad_value(192), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), 0.5);
            s.store_scaled_sub_ad(434, A::add(s.ad_value(192), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(192), s.ad_value(267)), A::sub(s.ad_value(192), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82)))), 0.5);
            s.store_scaled_sub_ad_rhs(435, 192, A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(192)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[617])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[418] && (!s.b[617])) {
            s.store_scalar(431, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(433, 0.0);
            s.store_scalar(434, 0.0);
            s.store_scalar(435, 0.0);
        }

        s.b[640] = (s.v[256] == 0.0);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[640]) {
            s.store_scalar(268, 0.0);
            s.store_scalar(291, 0.0);
            s.store_scalar(269, 0.0);
        }

        s.b[641] = (s.v[122] == 0.5);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[641]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[641])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if (s.b[418] && (!s.b[640])) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(192), s.ad_value(428))));
            s.store_mul(437, 101, 370);
        }

        s.b[642] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[642]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[642])) {
            s.store_sub(439, 107, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[643] = (s.v[9] == 0.5);
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && s.b[643]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && (!s.b[643])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[9])));
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[642])) {
            s.store_add(442, 440, 441);
        }

        s.b[644] = (s.v[9] == 0.5);
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && s.b[644]) {
            s.store_sqrt_scaled_input(436, 439, s.v[143]);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[642])) && (!s.b[644])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[642])) {
            s.store_scale(443, 436, s.v[137]);
            s.store_mul_ad_product_rhs(444, 98, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.b[645] = (s.v[23] == 0.0);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[645]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_scaled_div(446, 443, 439, ((s.v[122]) * (s.v[152])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[646] = (((-s.v[9]) * s.v[125]) == (-1.0));
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[646]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[646])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[647] = (s.v[457] > 0.0);
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[647]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[647])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[648] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[648]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[648])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[649] = (s.v[457] > 0.0);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && s.b[649]) {
            s.copy_ad(458, 421);
        }

        s.b[650] = (s.v[456] > (-230.25850929940458));
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[649])) && s.b[650]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[649])) && (!s.b[650])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[645])) && (!s.b[649])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[645])) {
            s.store_scaled_div(459, 458, 454, ((s.v[149]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[23]);
        }

        s.b[651] = (s.v[29] == 0.0);
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[651]) {
            s.store_scalar(460, 0.0);
        }

        s.b[652] = (s.v[9] == 0.5);
        s.v[652] = if s.b[652] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[651])) && s.b[652]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[651])) && (!s.b[652])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[651])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), 436, s.v[125]);
        }

        s.b[653] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[640])) && (!s.b[651])) && s.b[653]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.b[654] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[640])) && (!s.b[651])) && (!s.b[653])) && s.b[654]) {
            let assign16760_ad_e23560: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign16760_ad_e23560);
        }

        if ((((s.b[418] && (!s.b[640])) && (!s.b[651])) && (!s.b[653])) && (!s.b[654])) {
            let assign16770_ad_e23608: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign16770_ad_e23608);
        }

        if ((s.b[418] && (!s.b[640])) && (!s.b[651])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), 436, s.v[29]);
        }

        s.b[655] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[640])) && s.b[655]) {
            s.store_scalar(462, 1.0);
        }

        s.b[656] = (s.v[435] > ((-s.v[158]) * s.v[38]));
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        s.b[657] = (s.v[41] == 4.0);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[640])) && (!s.b[655])) && s.b[656]) && s.b[657]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if ((((s.b[418] && (!s.b[640])) && (!s.b[655])) && s.b[656]) && (!s.b[657])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[655])) && s.b[656]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[640])) && (!s.b[655])) && (!s.b[656])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165), s.v[159]);
        }

        if (s.b[418] && (!s.b[640])) {
            s.store_mul_add_ad_lhs(268, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(291, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        s.b[658] = (s.v[257] == 0.0);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[658]) {
            s.store_scalar(270, 0.0);
            s.store_scalar(292, 0.0);
            s.store_scalar(271, 0.0);
        }

        s.b[659] = (s.v[123] == 0.5);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[659]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[659])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if (s.b[418] && (!s.b[658])) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(192), s.ad_value(428))));
            s.store_mul(437, 102, 371);
        }

        s.b[660] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[660]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[660])) {
            s.store_sub(439, 108, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[661] = (s.v[10] == 0.5);
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && s.b[661]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && (!s.b[661])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[10])));
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[660])) {
            s.store_add(442, 440, 441);
        }

        s.b[662] = (s.v[10] == 0.5);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && s.b[662]) {
            s.store_sqrt_scaled_input(436, 439, s.v[144]);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[660])) && (!s.b[662])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[660])) {
            s.store_scale(443, 436, s.v[138]);
            s.store_mul_ad_product_rhs(444, 99, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.b[663] = (s.v[24] == 0.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[663]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_scaled_div(446, 443, 439, ((s.v[123]) * (s.v[153])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[664] = (((-s.v[10]) * s.v[126]) == (-1.0));
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[664]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[664])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[665] = (s.v[457] > 0.0);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[665]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[665])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[666] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[666]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[666])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[667] = (s.v[457] > 0.0);
        s.v[667] = if s.b[667] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && s.b[667]) {
            s.copy_ad(458, 421);
        }

        s.b[668] = (s.v[456] > (-230.25850929940458));
        s.v[668] = if s.b[668] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[667])) && s.b[668]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[667])) && (!s.b[668])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[663])) && (!s.b[667])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[663])) {
            s.store_scaled_div(459, 458, 454, ((s.v[150]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[24]);
        }

        s.b[669] = (s.v[30] == 0.0);
        s.v[669] = if s.b[669] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[669]) {
            s.store_scalar(460, 0.0);
        }

        s.b[670] = (s.v[10] == 0.5);
        s.v[670] = if s.b[670] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[669])) && s.b[670]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[669])) && (!s.b[670])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[669])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), 436, s.v[126]);
        }

        s.b[671] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[671] = if s.b[671] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[658])) && (!s.b[669])) && s.b[671]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.b[672] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));
        s.v[672] = if s.b[672] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[658])) && (!s.b[669])) && (!s.b[671])) && s.b[672]) {
            let assign17570_ad_e24716: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign17570_ad_e24716);
        }

        if ((((s.b[418] && (!s.b[658])) && (!s.b[669])) && (!s.b[671])) && (!s.b[672])) {
            let assign17580_ad_e24764: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign17580_ad_e24764);
        }

        if ((s.b[418] && (!s.b[658])) && (!s.b[669])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), 436, s.v[30]);
        }

        s.b[673] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));
        s.v[673] = if s.b[673] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[658])) && s.b[673]) {
            s.store_scalar(462, 1.0);
        }

        s.b[674] = (s.v[435] > ((-s.v[158]) * s.v[39]));
        s.v[674] = if s.b[674] { 1.0 } else { 0.0 };

        s.b[675] = (s.v[42] == 4.0);
        s.v[675] = if s.b[675] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[658])) && (!s.b[673])) && s.b[674]) && s.b[675]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if ((((s.b[418] && (!s.b[658])) && (!s.b[673])) && s.b[674]) && (!s.b[675])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[673])) && s.b[674]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[658])) && (!s.b[673])) && (!s.b[674])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166), s.v[160]);
        }

        if (s.b[418] && (!s.b[658])) {
            s.store_mul_add_ad_lhs(270, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(292, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        s.b[676] = (s.v[258] == 0.0);
        s.v[676] = if s.b[676] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[676]) {
            s.store_scalar(272, 0.0);
            s.store_scalar(293, 0.0);
            s.store_scalar(273, 0.0);
        }

        s.b[677] = (s.v[124] == 0.5);
        s.v[677] = if s.b[677] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[677]) {
            s.store_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[677])) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if (s.b[418] && (!s.b[676])) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(192), s.ad_value(428))));
            s.store_mul(437, 103, 372);
        }

        s.b[678] = ((s.v[22] == 0.0) && (s.v[25] == 0.0));
        s.v[678] = if s.b[678] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[678]) {
            s.store_scalar(439, 0.0);
            s.store_scalar(442, 0.0);
            s.store_scalar(443, 0.0);
            s.store_scalar(444, 0.0);
            s.store_scalar(438, 0.0);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[678])) {
            s.store_sub(439, 109, 433);
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.b[679] = (s.v[11] == 0.5);
        s.v[679] = if s.b[679] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && s.b[679]) {
            s.store_scalar(441, 0.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && (!s.b[679])) {
            s.store_scaled_add_ad_lhs(441, A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), 440, (1.0 - (2.0 * s.v[11])));
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[678])) {
            s.store_add(442, 440, 441);
        }

        s.b[680] = (s.v[11] == 0.5);
        s.v[680] = if s.b[680] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && s.b[680]) {
            s.store_sqrt_scaled_input(436, 439, s.v[145]);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[678])) && (!s.b[680])) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[678])) {
            s.store_scale(443, 436, s.v[139]);
            s.store_mul_ad_product_rhs(444, 100, A::offset(s.ad_value(430), (-1.0)), s.ad_value(443));
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.b[681] = (s.v[25] == 0.0);
        s.v[681] = if s.b[681] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[681]) {
            s.store_scalar(445, 0.0);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_scaled_div(446, 443, 439, ((s.v[124]) * (s.v[154])));
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
            s.store_square(448, 447);
            s.store_sqrt_div_ad(449, A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0));
            s.store_sqrt_abs_ad(450, s.ad_value(449));
            s.store_mul(451, 449, 450);
        }

        s.b[682] = (((-s.v[11]) * s.v[127]) == (-1.0));
        s.v[682] = if s.b[682] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[682]) {
            s.store_div_from_scalar_offset_ad(452, 1.0, A::mul(s.ad_value(446), s.ad_value(451)), 1.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[682])) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
            s.store_sqrt_scaled_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
            s.store_mul_offset_lhs(457, 455, (-1.0), 454);
            s.store_square(419, 457);
        }

        s.b[683] = (s.v[457] > 0.0);
        s.v[683] = if s.b[683] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[683]) {
            s.store_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[683])) {
            s.store_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));
        }

        s.b[684] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));
        s.v[684] = if s.b[684] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[684]) {
            s.store_exp_sub(436, 456, 419);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[684])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_mul_add_ad_lhs(421, A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88]), 436);
        }

        s.b[685] = (s.v[457] > 0.0);
        s.v[685] = if s.b[685] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && s.b[685]) {
            s.copy_ad(458, 421);
        }

        s.b[686] = (s.v[456] > (-230.25850929940458));
        s.v[686] = if s.b[686] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[685])) && s.b[686]) {
            s.store_exp(436, 456);
        }

        if ((((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[685])) && (!s.b[686])) {
            s.store_div_from_scalar_offset_ad(436, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[681])) && (!s.b[685])) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[681])) {
            s.store_scaled_div(459, 458, 454, ((s.v[151]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(445, A::mul(s.ad_value(444), s.ad_value(459)), 453, s.v[25]);
        }

        s.b[687] = (s.v[31] == 0.0);
        s.v[687] = if s.b[687] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[687]) {
            s.store_scalar(460, 0.0);
        }

        s.b[688] = (s.v[11] == 0.5);
        s.v[688] = if s.b[688] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[687])) && s.b[688]) {
            s.store_sqrt_scaled_ad(436, A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[687])) && (!s.b[688])) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[687])) {
            s.store_scaled_div_ad_lhs(461, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), 436, s.v[127]);
        }

        s.b[689] = (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458);
        s.v[689] = if s.b[689] { 1.0 } else { 0.0 };

        if (((s.b[418] && (!s.b[676])) && (!s.b[687])) && s.b[689]) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.b[690] = (((-s.v[157]) / s.v[461]) < (-230.25850929940458));
        s.v[690] = if s.b[690] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[676])) && (!s.b[687])) && (!s.b[689])) && s.b[690]) {
            let assign18380_ad_e25872: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign18380_ad_e25872);
        }

        if ((((s.b[418] && (!s.b[676])) && (!s.b[687])) && (!s.b[689])) && (!s.b[690])) {
            let assign18390_ad_e25920: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(436, assign18390_ad_e25920);
        }

        if ((s.b[418] && (!s.b[676])) && (!s.b[687])) {
            s.store_mul_scaled_ad_lhs(460, A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), 436, s.v[31]);
        }

        s.b[691] = ((s.v[40] > 1000000.0) || (p.p80 == 0.0));
        s.v[691] = if s.b[691] { 1.0 } else { 0.0 };

        if ((s.b[418] && (!s.b[676])) && s.b[691]) {
            s.store_scalar(462, 1.0);
        }

        s.b[692] = (s.v[435] > ((-s.v[158]) * s.v[40]));
        s.v[692] = if s.b[692] { 1.0 } else { 0.0 };

        s.b[693] = (s.v[43] == 4.0);
        s.v[693] = if s.b[693] { 1.0 } else { 0.0 };

        if ((((s.b[418] && (!s.b[676])) && (!s.b[691])) && s.b[692]) && s.b[693]) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if ((((s.b[418] && (!s.b[676])) && (!s.b[691])) && s.b[692]) && (!s.b[693])) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[691])) && s.b[692]) {
            s.store_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));
        }

        if (((s.b[418] && (!s.b[676])) && (!s.b[691])) && (!s.b[692])) {
            s.store_offset_mul_ad(462, A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167), s.v[161]);
        }

        if (s.b[418] && (!s.b[676])) {
            s.store_mul_add_ad_lhs(272, A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460), 462);
            s.store_mul_add_ad_lhs(293, A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460), 462);
        }

        if s.b[418] {
            s.store_add_scaled_ad_lhs(182, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), 272, s.v[258]);
        }

        s.b[694] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));
        s.v[694] = if s.b[694] { 1.0 } else { 0.0 };

        if (s.b[418] && s.b[694]) {
            s.store_scaled_square(422, 265, 4.0);
            s.store_div(423, 265, 266);
            s.store_add_ad_rhs(424, 193, A::mul(s.ad_value(265), s.ad_value(423)));
            s.store_add(425, 266, 424);
            s.store_sub(426, 266, 424);
            s.store_sqrt_square_add(427, 426, 422);
            s.store_scaled_div_ad(428, A::mul(s.ad_value(193), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427)), 2.0);
        }

        s.b[695] = (s.v[193] < s.v[262]);
        s.v[695] = if s.b[695] { 1.0 } else { 0.0 };

        s.b[696] = ((((0.5 * (s.v[193] * s.v[85]))) as f64).abs() < 230.25850929940458);
        s.v[696] = if s.b[696] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[696]) {
            s.store_exp_scaled_input(430, 193, (s.v[85] * 0.5));
        }

        s.b[697] = ((0.5 * (s.v[193] * s.v[85])) < (-230.25850929940458));
        s.v[697] = if s.b[697] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[696])) && s.b[697]) {
            let assign18640_ad_e26259: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(430, assign18640_ad_e26259);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[696])) && (!s.b[697])) {
            s.store_scaled_offset_ad(430, A::mul(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[698] = (s.v[62] < p.p85);
        s.v[698] = if s.b[698] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_offset_scaled_sub(360, 193, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[698]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[698])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
        }

        s.b[699] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[699] = if s.b[699] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[699]) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[700] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[700] = if s.b[700] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && s.b[700]) {
            let assign18960_ad_e26825: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(370, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign18960_ad_e26825, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[699])) && (!s.b[700])) {
            let assign18970_ad_e26903: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(370, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign18970_ad_e26903, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[701] = (s.v[64] < p.p85);
        s.v[701] = if s.b[701] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_offset_scaled_sub(360, 193, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[701]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[701])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
        }

        s.b[702] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[702] = if s.b[702] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[702]) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[703] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[703] = if s.b[703] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[702])) && s.b[703]) {
            let assign19280_ad_e27426: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(371, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19280_ad_e27426, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[702])) && (!s.b[703])) {
            let assign19290_ad_e27504: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(371, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19290_ad_e27504, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && s.b[695]) {
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[704] = (s.v[63] < p.p85);
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_offset_scaled_sub(360, 193, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[704]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
        }

        if (((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[704])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
        }

        s.b[705] = ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[705] = if s.b[705] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && s.b[695]) && s.b[705]) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[706] = ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[706] = if s.b[706] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[705])) && s.b[706]) {
            let assign19600_ad_e28027: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(372, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19600_ad_e28027, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[694]) && s.b[695]) && (!s.b[705])) && (!s.b[706])) {
            let assign19610_ad_e28105: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(372, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19610_ad_e28105, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_sqrt_mul_ad(430, A::offset(A::scale(A::sub(s.ad_value(193), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263));
            s.store_scaled_square(363, 318, 1.0 / (s.v[308]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));
        }

        s.b[707] = (s.v[62] < p.p85);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[62]);
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[62]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[707]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[62]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[707])) {
            s.store_scalar(350, s.v[62]);
            s.store_scalar(359, s.v[62]);
            s.store_scalar(366, 0.0);
        }

        s.b[708] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[708] = if s.b[708] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[708]) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[709] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[709] = if s.b[709] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && s.b[709]) {
            let assign19970_ad_e28735: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(281, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19970_ad_e28735, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[708])) && (!s.b[709])) {
            let assign19980_ad_e28814: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(281, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19980_ad_e28814, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(370, A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 281);
            s.store_scaled_square(363, 318, 1.0 / (s.v[310]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));
        }

        s.b[710] = (s.v[64] < p.p85);
        s.v[710] = if s.b[710] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[64]);
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[64]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[710]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[64]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[710])) {
            s.store_scalar(350, s.v[64]);
            s.store_scalar(359, s.v[64]);
            s.store_scalar(366, 0.0);
        }

        s.b[711] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[711] = if s.b[711] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[711]) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[712] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[712] = if s.b[712] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[711])) && s.b[712]) {
            let assign20350_ad_e29470: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(282, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign20350_ad_e29470, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[711])) && (!s.b[712])) {
            let assign20360_ad_e29549: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(282, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign20360_ad_e29549, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(371, A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 282);
            s.store_scaled_square(363, 318, 1.0 / (s.v[309]));
            s.store_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));
        }

        s.b[713] = (s.v[63] < p.p85);
        s.v[713] = if s.b[713] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_offset_scaled_sub(360, 262, 362, p.p86, s.v[63]);
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);
            s.store_offset_scaled_add(359, 314, 315, 0.5, s.v[63]);
            s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_ad_value(315, {
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[713]) {
            s.store_sqrt_square_add(315, 314, 315);
            s.store_offset_scaled_add(350, 314, 315, 0.5, s.v[63]);
            s.store_scaled_mul(366, 364, 365, p.p86);
        }

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[713])) {
            s.store_scalar(350, s.v[63]);
            s.store_scalar(359, s.v[63]);
            s.store_scalar(366, 0.0);
        }

        s.b[714] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);
        s.v[714] = if s.b[714] { 1.0 } else { 0.0 };

        if (((s.b[418] && s.b[694]) && (!s.b[695])) && s.b[714]) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.b[715] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));
        s.v[715] = if s.b[715] { 1.0 } else { 0.0 };

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[714])) && s.b[715]) {
            let assign20730_ad_e30205: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_offset_ad(283, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign20730_ad_e30205, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[418] && s.b[694]) && (!s.b[695])) && (!s.b[714])) && (!s.b[715])) {
            let assign20740_ad_e30284: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scaled_offset_ad(283, A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign20740_ad_e30284, 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[418] && s.b[694]) && (!s.b[695])) {
            s.store_scaled_add_ad(367, A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85)), s.v[85]);
            s.store_mul_ad_affine_product_lhs(372, A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367), 1.0, 1.0, 283);
        }

        if (s.b[418] && s.b[694]) {
            s.store_offset(370, 370, (-1.0));
            s.store_offset(371, 371, (-1.0));
            s.store_offset(372, 372, (-1.0));
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.b[716] = (s.v[193] > 0.0);
        s.v[716] = if s.b[716] { 1.0 } else { 0.0 };

        if ((s.b[418] && s.b[694]) && s.b[716]) {
            s.store_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));
        }

        if ((s.b[418] && s.b[694]) && (!s.b[716])) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 193);
        }

        if (s.b[418] && s.b[694]) {
            s.store_sub(432, 264, 431);
            s.store_scaled_sub_ad(433, A::add(s.ad_value(193), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(432)), A::sub(s.ad_value(193), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84]))), 0.5);
            s.store_scaled_sub_ad(434, A::add(s.ad_value(193), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(193), s.ad_value(267)), A::sub(s.ad_value(193), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82)))), 0.5);
            s.store_scaled_sub_ad_rhs(435, 193, A::sqrt(A::offset(A::mul(s.ad_value(193), s.ad_value(193)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        if (s.b[418] && (!s.b[694])) {
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
        }

    }
}
