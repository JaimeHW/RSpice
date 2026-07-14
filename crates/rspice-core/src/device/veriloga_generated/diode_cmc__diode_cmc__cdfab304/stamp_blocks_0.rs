#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(0, (8.8541878176e-12 * 11.8));s.store_scalar(1, (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) }));s.b[388] = ((!param_given[6]) && param_given[96]);s.store_scalar(388, if s.b[388] { 1.0 } else { 0.0 });
        if s.b[388] {s.store_scalar(1, (if (p.p96 > (-250.0)) { p.p96 } else { (-250.0) }));}
        s.store_scalar(2, (if (p.p5 > 1e-12) { p.p5 } else { 1e-12 }));s.store_scalar(3, (if (p.p8 > 1e-12) { p.p8 } else { 1e-12 }));s.store_scalar(4, (if (p.p9 > 1e-18) { p.p9 } else { 1e-18 }));s.store_scalar(5, (if (p.p10 > 1e-18) { p.p10 } else { 1e-18 }));s.store_scalar(6, (if (p.p11 > 0.05) { p.p11 } else { 0.05 }));s.store_scalar(7, (if (p.p12 > 0.05) { p.p12 } else { 0.05 }));s.store_scalar(8, (if (p.p13 > 0.05) { p.p13 } else { 0.05 }));s.store_scalar(9, (if (p.p14 > 0.05) { (if (p.p14 < 0.95) { p.p14 } else { 0.95 }) } else { 0.05 }));s.store_scalar(10, (if (p.p15 > 0.05) { (if (p.p15 < 0.95) { p.p15 } else { 0.95 }) } else { 0.05 }));s.store_scalar(11, (if (p.p16 > 0.05) { (if (p.p16 < 0.95) { p.p16 } else { 0.95 }) } else { 0.05 }));s.store_scalar(12, p.p17);s.store_scalar(13, p.p18);s.store_scalar(14, p.p19);s.store_scalar(15, (if (p.p20 > 0.0) { p.p20 } else { 0.0 }));s.store_scalar(16, (if (p.p21 > 0.0) { p.p21 } else { 0.0 }));s.store_scalar(17, (if (p.p22 > 0.0) { p.p22 } else { 0.0 }));s.store_scalar(20, (if (p.p23 > 0.0) { p.p23 } else { 0.0 }));s.store_scalar(21, (if (p.p24 > 0.0) { p.p24 } else { 0.0 }));s.store_scalar(22, (if (p.p25 > 0.0) { p.p25 } else { 0.0 }));s.store_scalar(18, (if (p.p26 > 1e-9) { p.p26 } else { 1e-9 }));s.store_scalar(19, (if (p.p27 > 1e-9) { p.p27 } else { 1e-9 }));s.store_scalar(23, (if (p.p28 > 0.0) { p.p28 } else { 0.0 }));s.store_scalar(24, (if (p.p29 > 0.0) { p.p29 } else { 0.0 }));s.store_scalar(25, (if (p.p30 > 0.0) { p.p30 } else { 0.0 }));s.store_scalar(26, (if (p.p31 > 0.01) { p.p31 } else { 0.01 }));s.store_scalar(27, (if (p.p32 > 0.01) { p.p32 } else { 0.01 }));s.store_scalar(28, (if (p.p33 > 0.01) { p.p33 } else { 0.01 }));s.store_scalar(29, (if (p.p34 > 0.0) { p.p34 } else { 0.0 }));s.store_scalar(30, (if (p.p35 > 0.0) { p.p35 } else { 0.0 }));s.store_scalar(31, (if (p.p36 > 0.0) { p.p36 } else { 0.0 }));s.store_scalar(32, p.p37);s.store_scalar(33, p.p38);s.store_scalar(34, p.p39);s.store_scalar(35, p.p40);s.store_scalar(36, p.p41);s.store_scalar(37, p.p42);s.store_scalar(38, (if (p.p43 > 0.1) { p.p43 } else { 0.1 }));s.store_scalar(39, (if (p.p44 > 0.1) { p.p44 } else { 0.1 }));s.store_scalar(40, (if (p.p45 > 0.1) { p.p45 } else { 0.1 }));s.store_scalar(41, (if (p.p46 > 0.1) { p.p46 } else { 0.1 }));s.store_scalar(42, (if (p.p47 > 0.1) { p.p47 } else { 0.1 }));s.store_scalar(43, (if (p.p48 > 0.1) { p.p48 } else { 0.1 }));s.store_scalar(44, p.p7);s.store_scalar(48, (if (p.p49 > 0.0) { p.p49 } else { 0.0 }));s.store_scalar(49, (if (p.p50 > 0.0) { p.p50 } else { 0.0 }));s.store_scalar(50, (if (p.p51 > 0.0) { p.p51 } else { 0.0 }));s.store_scalar(52, (if (p.p52 > 0.0) { p.p52 } else { 0.0 }));s.store_scalar(51, (if (p.p53 > 0.0) { p.p53 } else { 0.0 }));s.store_scalar(55, (if (p.p56 > 0.0) { p.p56 } else { 0.0 }));s.store_scalar(56, p.p57);s.store_scalar(57, p.p58);s.store_scalar(58, p.p59);s.store_scalar(59, p.p60);s.store_scalar(60, p.p61);s.store_scalar(61, p.p62);s.store_scalar(62, (if (p.p63 > 0.1) { p.p63 } else { 0.1 }));s.store_scalar(64, (if (p.p64 > 0.1) { p.p64 } else { 0.1 }));s.store_scalar(63, (if (p.p65 > 0.1) { p.p65 } else { 0.1 }));s.store_scalar(75, (if (p.p76 > 0.1) { p.p76 } else { 0.1 }));s.store_scalar(76, (if (p.p77 > 0.0) { p.p77 } else { 0.0 }));s.store_scalar(77, (if (p.p78 > 0.0) { p.p78 } else { 0.0 }));s.store_scalar(45, 0.0);s.b[389] = (p.p81 > 0.5);s.store_scalar(389, if s.b[389] { 1.0 } else { 0.0 });
        let (t1,) = {
    if s.b[389] {
        (1.0,)
    } else {
        (s.v[45],)
    }
};
        s.store_scalar(45, t1);
        let (t2,) = {
    if (!s.b[389]) {
        (0.0,)
    } else {
        (s.v[45],)
    }
};
        s.store_scalar(45, t2);s.store_scalar(46, (if (p.p82 > 0.5) { p.p82 } else { 0.5 }));
        let (t3,) = {
    if (p.p83 > 0.0) {
        (p.p83,)
    } else {
        (0.0,)
    }
};
        s.store_scalar(47, t3);s.store_primal_offset(78, 1, 273.15);s.store_scalar(79, ((ctx_temp + p.p102)).max((273.15 + (-250.0))));s.store_primal_div_from_scalar(80, s.v[79], 78);s.store_scalar(81, (1.3806505e-23 / 1.6021918e-19));s.store_primal_scale(82, 78, s.v[81]);s.store_primal_div_from_scalar(83, 1.0, 82);s.store_scalar(84, (s.v[81] * s.v[79]));s.store_scalar(85, (1.0 / s.v[84]));s.store_primal_div_scaled_inputs(89, A::mul_scaled_lhs(s.ad_value(78), 0.000702, s.ad_value(78)), -1.0, A::offset(s.ad_value(78), 1108.0), 1.0);s.store_primal_offset(92, 89, s.v[12]);s.store_primal_offset(93, 89, s.v[13]);s.store_primal_offset(94, 89, s.v[14]);s.store_scalar(90, ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79])));s.store_scalar(95, (s.v[12] + s.v[90]));s.store_scalar(96, (s.v[13] + s.v[90]));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
    ) {
        s.store_scalar(97, (s.v[14] + s.v[90]));s.store_primal_mul_powf_mixed_ai(98, A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), 0.5), 80, (s.v[75] / 2.0));s.store_primal_mul_powf_mixed_ai(99, A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), 0.5), 80, (s.v[75] / 2.0));s.store_primal_mul_powf_mixed_ai(100, A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), 0.5), 80, (s.v[75] / 2.0));s.store_primal_mul_powf_mixed_ai(176, A::exp_scaled_input(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62]))), 80, ((s.v[75] / 2.0) / s.v[62]));s.store_primal_mul_powf_mixed_ai(177, A::exp_scaled_input(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64]))), 80, ((s.v[75] / 2.0) / s.v[64]));s.store_primal_mul_powf_mixed_ai(178, A::exp_scaled_input(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63]))), 80, ((s.v[75] / 2.0) / s.v[63]));s.store_primal_scaled_mul(101, 176, 176, s.v[15]);s.store_primal_scaled_mul(102, 177, 177, s.v[16]);s.store_primal_scaled_mul(103, 178, 178, s.v[17]);s.store_primal_sub_scaled_inputs_ln_rhs(104, 80, s.v[6], 98, (2.0 * s.v[84]));s.store_primal_sub_scaled_inputs_ln_rhs(105, 80, s.v[7], 99, (2.0 * s.v[84]));s.store_primal_sub_scaled_inputs_ln_rhs(106, 80, s.v[8], 100, (2.0 * s.v[84]));s.store_primal_add_scaled_inputs_mixed_ia(107, 104, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(104), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]);s.store_primal_add_scaled_inputs_mixed_ia(108, 105, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(105), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]);s.store_primal_add_scaled_inputs_mixed_ia(109, 106, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(106), (-s.v[85]), ((0.05) * (s.v[85])))), s.v[84]);s.store_primal_div_from_scalar(119, 1.0, 107);s.store_primal_div_from_scalar(120, 1.0, 108);s.store_primal_div_from_scalar(121, 1.0, 109);s.store_scalar(122, (1.0 - s.v[9]));s.store_scalar(123, (1.0 - s.v[10]));s.store_scalar(124, (1.0 - s.v[11]));s.store_scalar(125, (1.0 / s.v[122]));s.store_scalar(126, (1.0 / s.v[123]));s.store_scalar(127, (1.0 / s.v[124]));s.store_primal_scaled_powf_ad(128, A::scale(s.ad_value(119), s.v[6]), s.v[9], s.v[3]);s.store_primal_scaled_powf_ad(129, A::scale(s.ad_value(120), s.v[7]), s.v[10], s.v[4]);s.store_primal_scaled_powf_ad(130, A::scale(s.ad_value(121), s.v[8]), s.v[11], s.v[5]);s.store_primal_scaled_mul(131, 128, 107, s.v[125]);s.store_primal_scaled_mul(132, 129, 108, s.v[126]);s.store_primal_scaled_mul(133, 130, 109, s.v[127]);s.store_primal_scale(134, 128, 2.0);s.store_primal_scale(135, 129, 2.0);s.store_primal_scale(136, 130, 2.0);s.store_scalar(137, (s.v[0] / s.v[3]));s.store_scalar(138, ((s.v[18] * s.v[0]) / s.v[4]));s.store_scalar(139, ((s.v[19] * s.v[0]) / s.v[5]));s.store_scalar(140, (1.0 / s.v[137]));s.store_scalar(141, (1.0 / s.v[138]));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(142, (1.0 / s.v[139]));s.store_scalar(143, (1.0 / s.v[6]));s.store_scalar(144, (1.0 / s.v[7]));s.store_scalar(145, (1.0 / s.v[8]));s.store_scalar(86, (1.772453850905516 * 0.29214664));s.store_scalar(87, (((((-5.0) * 0.29214664) + 6.0) - ((s.v[86]) as f64).powi(((-2.0) as i32))) / 3.0));s.store_scalar(88, ((1.0 - 0.29214664) - s.v[87]));s.store_scalar(146, ((0.5 * s.v[95])).max(s.v[84]));s.store_scalar(147, ((0.5 * s.v[96])).max(s.v[84]));s.store_scalar(148, ((0.5 * s.v[97])).max(s.v[84]));s.store_scalar(149, (s.v[146] * s.v[85]));s.store_scalar(150, (s.v[147] * s.v[85]));s.store_scalar(151, (s.v[148] * s.v[85]));s.store_scalar(152, (((((((32.0 * s.v[26]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[146] * s.v[146]) * s.v[146]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(153, (((((((32.0 * s.v[27]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[147] * s.v[147]) * s.v[147]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(154, (((((((32.0 * s.v[28]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[148] * s.v[148]) * s.v[148]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_primal_offset_scaled(155, 78, (((-s.v[35])) * (s.v[32])), ((((((s.v[79]) * (s.v[35]))) + (1.0))) * (s.v[32])));s.store_primal_offset_scaled(156, 78, (((-s.v[36])) * (s.v[33])), ((((((s.v[79]) * (s.v[36]))) + (1.0))) * (s.v[33])));s.store_primal_offset_scaled(157, 78, (((-s.v[37])) * (s.v[34])), ((((((s.v[79]) * (s.v[37]))) + (1.0))) * (s.v[34])));
        if (!(s.v[155] > 0.0)) {s.store_scalar(155, 0.0);}
        if (!(s.v[156] > 0.0)) {s.store_scalar(156, 0.0);}
        if (!(s.v[157] > 0.0)) {s.store_scalar(157, 0.0);}
        s.store_scalar(158, ((s.v[44] - 1.0) / s.v[44]));s.store_scalar(159, (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[41]))));s.store_scalar(160, (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[42]))));s.store_scalar(161, (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[43]))));s.store_primal_scaled_offset_ad(38, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[57], s.v[56], 1.0), 1.0, s.v[38]);s.store_primal_scaled_offset_ad(39, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[59], s.v[58], 1.0), 1.0, s.v[39]);s.store_primal_scaled_offset_ad(40, A::mul_sub_from_scalar_scaled_offset_self(s.v[79], s.ad_value(78), s.v[61], s.v[60], 1.0), 1.0, s.v[40]);s.b[390] = (s.v[38] <= 0.1);s.store_scalar(390, if s.b[390] { 1.0 } else { 0.0 });
        if s.b[390] {s.store_scalar(38, 0.1);s.store_scalar(162, 10.0);}
        if (!s.b[390]) {s.store_primal_div_from_scalar(162, 1.0, 38);}
        s.b[391] = (s.v[39] <= 0.1);s.store_scalar(391, if s.b[391] { 1.0 } else { 0.0 });
        if s.b[391] {s.store_scalar(39, 0.1);s.store_scalar(163, 10.0);}
        if (!s.b[391]) {s.store_primal_div_from_scalar(163, 1.0, 39);}
        s.b[392] = (s.v[40] <= 0.1);s.store_scalar(392, if s.b[392] { 1.0 } else { 0.0 });
        if s.b[392] {s.store_scalar(40, 0.1);s.store_scalar(164, 10.0);}
        if (!s.b[392]) {s.store_primal_div_from_scalar(164, 1.0, 40);}
        s.store_scalar(179, (1.0 - (0.01 * s.v[77])));s.store_primal_scale(165, 162, ((-((s.v[159] * s.v[159]) * ((s.v[158]) as f64).powf((s.v[41] - 1.0)))) * s.v[41]));s.store_primal_scale(166, 163, ((-((s.v[160] * s.v[160]) * ((s.v[158]) as f64).powf((s.v[42] - 1.0)))) * s.v[42]));s.store_primal_scale(167, 164, ((-((s.v[161] * s.v[161]) * ((s.v[158]) as f64).powf((s.v[43] - 1.0)))) * s.v[43]));s.store_primal_scale_ad(173, A::powf(s.ad_value(80), s.v[51]), s.v[48]);s.store_primal_scale_ad(175, A::powf(s.ad_value(80), s.v[51]), s.v[50]);s.store_primal_scale_ad(174, A::powf(s.ad_value(80), s.v[51]), s.v[49]);s.store_primal_scale_ad(172, A::powf(s.ad_value(80), s.v[51]), s.v[52]);s.store_scalar(308, (p.p87 * 1000000.0));s.store_scalar(310, (p.p89 * 1000000.0));s.store_scalar(309, (p.p88 * 1000000.0));s.store_scalar(307, s.v[308]);s.store_scalar(313, s.v[62]);s.store_scalar(311, (1450.0 * 0.0001));s.store_scalar(312, (500.0 * 0.0001));s.store_scalar(368, 0.6);s.store_scalar(369, 0.001);s.store_primal_scale(318, 176, 1.45e16);s.store_primal_scaled_square(319, 318, 1.0 / (s.v[307]));s.store_primal_powf(316, 80, (-1.5));s.store_primal_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));s.store_primal_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));s.store_primal_div_scaled_product_add_scaled_denominator_indices(322, 320, 321, 2.0, 320, 1.0, 321, 1.0, 1.0);s.store_primal_powf(317, 80, p.p97);s.store_primal_scale(324, 317, p.p93);s.store_primal_sqrt_mul(323, 324, 322);s.store_primal_scaled_ln_ad(347, A::div_from_scalar(s.v[307], s.ad_value(319)), (s.v[313] / s.v[85]));s.store_primal_scaled_add_ad(348, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323)), (s.v[313] / s.v[85]));s.store_scalar(256, (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]));s.store_scalar(257, (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]));s.store_scalar(258, (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
    ) {
        s.store_scalar(263, 0.0);s.store_scalar(281, 0.0);s.store_scalar(282, 0.0);s.store_scalar(283, 0.0);s.b[393] = ((s.v[101] * s.v[256]) > 0.0);s.store_scalar(393, if s.b[393] { 1.0 } else { 0.0 });
        if s.b[393] {s.store_primal_scaled_ln_ad(168, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0), (s.v[84] * s.v[62]));}
        if (!s.b[393]) {s.store_scalar(168, 100000000.0);}
        s.b[394] = ((s.v[102] * s.v[257]) > 0.0);s.store_scalar(394, if s.b[394] { 1.0 } else { 0.0 });
        if s.b[394] {s.store_primal_scaled_ln_ad(169, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0), (s.v[84] * s.v[64]));}
        if (!s.b[394]) {s.store_scalar(169, 100000000.0);}
        s.b[395] = ((s.v[103] * s.v[258]) > 0.0);s.store_scalar(395, if s.b[395] { 1.0 } else { 0.0 });
        if s.b[395] {s.store_primal_scaled_ln_ad(170, A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0), (s.v[84] * s.v[63]));}
        if (!s.b[395]) {s.store_scalar(170, 100000000.0);}
        s.store_min3(262, 168, 169, 170);s.b[396] = ((((s.v[262] * s.v[85])) as f64).abs() < 230.25850929940458);s.store_scalar(396, if s.b[396] { 1.0 } else { 0.0 });
        if s.b[396] {s.store_primal_exp_scaled_input(263, 262, s.v[85]);}
        s.b[397] = ((s.v[262] * s.v[85]) < (-230.25850929940458));s.store_scalar(397, if s.b[397] { 1.0 } else { 0.0 });
        if ((!s.b[396]) && s.b[397]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(263, 1e-100, (-230.25850929940458), A::scale(s.ad_value(262), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((!s.b[396]) && (!s.b[397])) {s.store_primal_scaled_offset_ad(263, A::mul_offset_rhs(A::scale_offset(s.ad_value(262), s.v[85], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(262), s.v[85], (-230.25850929940458)), A::scale_offset(s.ad_value(262), ((s.v[85]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        s.copy_ad(110, 107);s.copy_ad(111, 108);s.copy_ad(112, 109);s.store_scalar(113, s.v[9]);s.store_scalar(114, s.v[10]);s.store_scalar(115, s.v[11]);s.store_scalar(116, s.v[6]);s.store_scalar(117, s.v[7]);s.store_scalar(118, s.v[8]);s.b[398] = (s.v[256] == 0.0);s.store_scalar(398, if s.b[398] { 1.0 } else { 0.0 });
        if s.b[398] {s.store_primal_add(110, 108, 109);s.store_scalar(113, (0.9 * (s.v[10]).min(s.v[11])));s.store_scalar(116, (s.v[7] + s.v[8]));}
        s.b[399] = (s.v[257] == 0.0);s.store_scalar(399, if s.b[399] { 1.0 } else { 0.0 });
        if s.b[399] {s.store_primal_add(111, 107, 109);s.store_scalar(114, (0.9 * (s.v[9]).min(s.v[11])));s.store_scalar(117, (s.v[6] + s.v[8]));}
        s.b[400] = (s.v[258] == 0.0);s.store_scalar(400, if s.b[400] { 1.0 } else { 0.0 });
        if s.b[400] {s.store_primal_add(112, 107, 108);s.store_scalar(115, (0.9 * (s.v[9]).min(s.v[10])));s.store_scalar(118, (s.v[6] + s.v[7]));}
        s.store_min3(264, 110, 111, 112);s.store_primal_scale(265, 264, 0.1);s.store_max3(91, 113, 114, 115);s.store_primal_mul_scale_offset_mixed_ia(266, 264, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(91))), -1.0, 1.0);s.store_primal_offset_min_ad(267, A::min(s.ad_value(116), s.ad_value(117)), s.ad_value(118), (-0.05));s.store_primal_add_scaled_inputs3_indices(289, 101, s.v[256], 102, s.v[257], 103, s.v[258]);s.store_scalar(300, 0.0);s.store_scalar(301, 1.0);s.store_scalar(303, 1.0);s.store_scalar(302, 0.0);s.store_scalar(305, 1.0);s.store_scalar(304, 0.0);s.store_scalar(306, 0.0);s.store_scalar(294, 0.0);s.store_scalar(295, 0.0);s.store_scalar(296, 0.0);s.store_scalar(297, 0.0);s.store_scalar(298, 0.0);s.store_scalar(299, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(196, 0.0);s.store_scalar(197, 0.0);s.store_scalar(185, 0.0);s.store_scalar(186, 0.0);s.store_scalar(187, 0.0);s.store_scalar(188, 0.0);s.store_scalar(189, 0.0);s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);s.store_scalar(200, 0.0);s.store_scalar(208, 0.0);s.store_scalar(259, 1.0);s.store_scalar(260, 1.0);s.store_scalar(261, 1.0);s.store_scalar(195, 0.0);s.store_scalar(203, 0.0);s.store_scalar(204, 0.0);s.store_scalar(285, 0.0);s.b[409] = ((s.v[256] * s.v[173]) > 0.0);s.store_scalar(409, if s.b[409] { 1.0 } else { 0.0 });
        if s.b[409] {s.store_primal_div_from_scalar(285, s.v[256], 173);}
        s.b[410] = ((s.v[257] * s.v[174]) > 0.0);s.store_scalar(410, if s.b[410] { 1.0 } else { 0.0 });
        if s.b[410] {s.store_primal_add_mixed_ai(285, A::div_from_scalar(s.v[257], s.ad_value(174)), 285);}
        s.b[411] = ((s.v[258] * s.v[175]) > 0.0);s.store_scalar(411, if s.b[411] { 1.0 } else { 0.0 });
        if s.b[411] {s.store_primal_add_mixed_ai(285, A::div_from_scalar(s.v[258], s.ad_value(175)), 285);}
        s.b[412] = (s.v[285] > 0.0);s.store_scalar(412, if s.b[412] { 1.0 } else { 0.0 });
        if s.b[412] {s.store_primal_add_mixed_ai(171, A::div_from_scalar(1.0, s.ad_value(285)), 172);}
        if (!s.b[412]) {s.copy_ad(171, 172);}
        s.store_scalar(370, 0.0);s.store_scalar(372, 0.0);s.store_scalar(371, 0.0);s.store_scalar(345, 0.0);s.store_scalar(338, 0.0);s.store_scalar(339, 0.0);s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);s.store_scalar(344, 0.0);s.store_scalar(333, (1.6021918e-19 * s.v[256]));s.store_scalar(343, ((((2.0 * s.v[0]) / (1.6021918e-19 * s.v[307]))) as f64).sqrt());s.store_scalar(314, ((p.p94 - s.v[343]) - 1e-7));s.store_scalar(315, ((4.0 * p.p94) * 1e-7));
        if (!(s.v[315] > 0.0)) {s.store_scalar(315, (-s.v[315]));}
        s.store_sqrt_offset_input(315, 315, (s.v[314] * s.v[314]));s.store_sub_from_scalar_ad(343, p.p94, A::scaled_offset(s.ad_value(315), s.v[314], 0.5));s.b[413] = (s.v[45] > 0.9);s.store_scalar(413, if s.b[413] { 1.0 } else { 0.0 });s.b[414] = ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0)));s.store_scalar(414, if s.b[414] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (s.b[413] && s.b[414]) {
        (0.0,)
    } else {
        (s.v[45],)
    }
};
        s.store_scalar(45, t0);s.b[415] = (s.v[256] > 0.0);s.store_scalar(415, if s.b[415] { 1.0 } else { 0.0 });
        if ((s.b[413] && (!s.b[414])) && s.b[415]) {s.store_scalar(301, s.v[62]);}
        s.b[416] = (s.v[258] > 0.0);s.store_scalar(416, if s.b[416] { 1.0 } else { 0.0 });
        if ((s.b[413] && (!s.b[414])) && s.b[416]) {s.store_scalar(301, s.v[63]);}
        s.b[417] = (s.v[257] > 0.0);s.store_scalar(417, if s.b[417] { 1.0 } else { 0.0 });
        if ((s.b[413] && (!s.b[414])) && s.b[417]) {s.store_scalar(301, s.v[64]);}
        s.b[418] = (s.v[45] == 1.0);s.store_scalar(418, if s.b[418] { 1.0 } else { 0.0 });
        if s.b[418] {s.store_scalar(419, 0.0);s.store_scalar(420, 0.0);s.store_scalar(421, 0.0);s.store_scalar(422, 0.0);s.store_scalar(423, 0.0);s.store_scalar(424, 0.0);s.store_scalar(425, 0.0);s.store_scalar(426, 0.0);s.store_scalar(427, 0.0);s.store_scalar(277, 0.0);s.store_scalar(428, 0.0);s.store_scalar(429, 0.0);s.store_scalar(430, 0.0);s.store_scalar(431, 0.0);s.store_scalar(432, 0.0);s.store_scalar(433, 0.0);s.store_scalar(434, 0.0);s.store_scalar(435, 0.0);s.store_scalar(436, 0.0);s.store_scalar(437, 0.0);s.store_scalar(438, 0.0);s.store_scalar(439, 0.0);s.store_scalar(440, 0.0);s.store_scalar(441, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[418] {s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);s.store_scalar(447, 0.0);s.store_scalar(448, 0.0);s.store_scalar(449, 0.0);s.store_scalar(450, 0.0);s.store_scalar(451, 0.0);s.store_scalar(452, 0.0);s.store_scalar(453, 0.0);s.store_scalar(454, 0.0);s.store_scalar(455, 0.0);s.store_scalar(456, 0.0);s.store_scalar(457, 0.0);s.store_scalar(458, 0.0);s.store_scalar(459, 0.0);s.store_scalar(460, 0.0);s.store_scalar(461, 0.0);s.store_scalar(462, 0.0);s.store_scalar(205, 0.4);s.store_scalar(206, 0.65);s.store_scalar(207, 0.8);s.store_primal_scale(190, 205, (-s.v[46]));s.store_primal_scale(191, 206, (-s.v[46]));s.store_primal_scale(192, 207, (-s.v[46]));s.store_scalar(193, 0.1);s.store_scalar(194, 0.2);}
        s.b[463] = (!(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)));s.store_scalar(463, if s.b[463] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[463]) {s.store_primal_scaled_mul(422, 265, 265, 4.0);s.store_primal_div(423, 265, 266);s.store_primal_add_scaled_product_indices(424, 190, 1.0, 265, 423, 1.0);s.store_primal_add(425, 266, 424);s.store_primal_sub(426, 266, 424);s.store_primal_sqrt_square_add(427, 426, 422);s.store_primal_div_scaled_product_add_scaled_denominator_indices(428, 190, 266, 2.0, 425, 1.0, 427, 1.0, 1.0);}
        s.b[464] = (s.v[190] < s.v[262]);s.store_scalar(464, if s.b[464] { 1.0 } else { 0.0 });s.b[465] = ((((0.5 * (s.v[190] * s.v[85]))) as f64).abs() < 230.25850929940458);s.store_scalar(465, if s.b[465] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[465]) {s.store_primal_exp_scaled_input(430, 190, (s.v[85] * 0.5));}
        s.b[466] = ((0.5 * (s.v[190] * s.v[85])) < (-230.25850929940458));s.store_scalar(466, if s.b[466] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && s.b[466]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(430, 1e-100, (-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[465])) && (!s.b[466])) {s.store_primal_scaled_offset_ad(430, A::mul_offset_rhs(A::scale_offset(s.ad_value(190), (s.v[85] * 0.5), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(190), (s.v[85] * 0.5), (-230.25850929940458)), A::scale_offset(s.ad_value(190), (((s.v[85] * 0.5)) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && s.b[464]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[467] = (s.v[62] < p.p85);s.store_scalar(467, if s.b[467] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[467]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);}
        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[467])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);}
        s.b[468] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(468, if s.b[468] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[468]) {s.store_exp_scaled_input_ad(370, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[469] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(469, if s.b[469] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && s.b[469]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(370, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[468])) && (!s.b[469])) {s.store_scaled_softlimit_poly_offset_lhs_ad(370, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && s.b[464]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[470] = (s.v[64] < p.p85);s.store_scalar(470, if s.b[470] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[470]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);}
        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[470])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);}
        s.b[471] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(471, if s.b[471] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[471]) {s.store_exp_scaled_input_ad(371, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[472] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(472, if s.b[472] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && s.b[472]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(371, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[471])) && (!s.b[472])) {s.store_scaled_softlimit_poly_offset_lhs_ad(371, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && s.b[464]) {s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[473] = (s.v[63] < p.p85);s.store_scalar(473, if s.b[473] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_offset_sub_scaled_inputs_indices(360, 190, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[473]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);}
        if (((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[473])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);}
        s.b[474] = ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(474, if s.b[474] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && s.b[464]) && s.b[474]) {s.store_exp_scaled_input_ad(372, A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[475] = ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(475, if s.b[475] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && s.b[475]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(372, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && s.b[464]) && (!s.b[474])) && (!s.b[475])) {s.store_scaled_softlimit_poly_offset_lhs_ad(372, A::add_scaled_inputs(A::div(s.ad_value(190), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_primal_sqrt_ad(430, A::mul_offset_lhs(A::sub_scaled_inputs(s.ad_value(190), s.v[85], s.ad_value(262), s.v[85]), 1.0, s.ad_value(263)));s.store_primal_scaled_square(363, 318, 1.0 / (s.v[308]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[308], s.ad_value(363)), (s.v[62] / s.v[85]));}
        s.b[476] = (s.v[62] < p.p85);s.store_scalar(476, if s.b[476] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[62]);s.store_sub_from_scalar_scaled_input(350, s.v[62], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[62]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[476]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[62]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[476])) {s.store_scalar(350, s.v[62]);s.store_scalar(359, s.v[62]);s.store_scalar(366, 0.0);}
        s.b[477] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(477, if s.b[477] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[477]) {s.store_exp_scaled_input_ad(281, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[478] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(478, if s.b[478] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && s.b[478]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(281, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[477])) && (!s.b[478])) {s.store_scaled_softlimit_poly_offset_lhs_ad(281, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(370, 281, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[310]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[310], s.ad_value(363)), (s.v[64] / s.v[85]));}
        s.b[479] = (s.v[64] < p.p85);s.store_scalar(479, if s.b[479] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[64]);s.store_sub_from_scalar_scaled_input(350, s.v[64], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[64]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[479]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[64]);s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[479])) {s.store_scalar(350, s.v[64]);s.store_scalar(359, s.v[64]);s.store_scalar(366, 0.0);}
        s.b[480] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(480, if s.b[480] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[480]) {s.store_exp_scaled_input_ad(282, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[481] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(481, if s.b[481] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && s.b[481]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(282, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[480])) && (!s.b[481])) {s.store_scaled_softlimit_poly_offset_lhs_ad(282, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(371, 282, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);s.store_primal_scaled_square(363, 318, 1.0 / (s.v[309]));s.store_primal_scaled_ln_ad(362, A::div_from_scalar(s.v[309], s.ad_value(363)), (s.v[63] / s.v[85]));}
        s.b[482] = (s.v[63] < p.p85);s.store_scalar(482, if s.b[482] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_offset_sub_scaled_inputs_indices(360, 262, p.p86, 362, p.p86, s.v[63]);s.store_sub_from_scalar_scaled_input(350, s.v[63], 362, p.p86);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(360), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(364, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(361, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_scaled_div(365, 314, 315, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(359, 314, 0.5, 315, 0.5, s.v[63]);s.store_offset_sub_from_scalar_ad(314, p.p85, s.ad_value(350), (-0.01));s.store_scalar(315, ((4.0 * p.p85) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, (-0.5), 315, (-0.5), p.p85);s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {
            if (s.v[315] > 0.0) {
            } else {
                s.store_neg(315, 315);
            }
        }
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_sqrt_square_add(315, 314, 315);s.store_offset_add_scaled_inputs_indices(350, 314, 0.5, 315, 0.5, s.v[63]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[482]) {s.store_scaled_mul(366, 364, 365, p.p86);}
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[482])) {s.store_scalar(350, s.v[63]);s.store_scalar(359, s.v[63]);s.store_scalar(366, 0.0);}
        s.b[483] = ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458);s.store_scalar(483, if s.b[483] { 1.0 } else { 0.0 });
        if (((s.b[418] && s.b[463]) && (!s.b[464])) && s.b[483]) {s.store_exp_scaled_input_ad(283, A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85)), s.v[85]);}
        s.b[484] = ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458));s.store_scalar(484, if s.b[484] { 1.0 } else { 0.0 });
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && s.b[484]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(283, 1e-100, (-230.25850929940458), A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && s.b[463]) && (!s.b[464])) && (!s.b[483])) && (!s.b[484])) {s.store_scaled_softlimit_poly_offset_lhs_ad(283, A::add_scaled_inputs(A::div(s.ad_value(262), s.ad_value(359)), s.v[85], A::div_scaled_product(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350)), 1.0, s.ad_value(350), p.p85), s.v[85]), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && s.b[463]) && (!s.b[464])) {s.store_scaled_add_ad(367, A::div_scaled_add_product(s.ad_value(359), 1.0, s.ad_value(262), s.ad_value(366), (-1.0), A::square(s.ad_value(359)), 1.0), A::div_scaled_product(s.ad_value(362), s.ad_value(366), 1.0, s.ad_value(350), p.p85), s.v[85]);s.store_mul_scale_offset_mixed_ia(372, 283, A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0, 1.0);}
        if (s.b[418] && s.b[463]) {s.store_offset(370, 370, (-1.0));s.store_offset(371, 371, (-1.0));s.store_offset(372, 372, (-1.0));s.store_primal_div_from_scalar(429, 1.0, 430);}
        s.b[485] = (s.v[190] > 0.0);s.store_scalar(485, if s.b[485] { 1.0 } else { 0.0 });
        if ((s.b[418] && s.b[463]) && s.b[485]) {s.store_primal_scaled_ln_ad(431, A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(429), 1.0, A::offset(s.ad_value(429), 3.0)))), (s.v[84] * 2.0));}
        if ((s.b[418] && s.b[463]) && (!s.b[485])) {s.store_primal_sub_mixed_ai(431, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(430), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(430), 1.0, A::scale_offset(s.ad_value(430), 3.0, 1.0))))), (s.v[84] * 2.0)), 190);}
        if (s.b[418] && s.b[463]) {s.store_primal_sub(432, 264, 431);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(433, 190, 0.5, 432, 0.5, 190, 432, ((4.0 * s.v[84]) * s.v[84]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(434, 190, 0.5, 267, 0.5, A::add_scaled_square_product(A::sub(s.ad_value(190), s.ad_value(267)), 1.0, s.ad_value(82), s.ad_value(82), 4.0), (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
    ) {
        if (s.b[418] && s.b[463]) {s.store_primal_scaled_sub_mixed_ia(435, 190, A::sqrt_square_offset(s.ad_value(190), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        if (s.b[418] && (!s.b[463])) {s.store_scalar(370, 0.0);s.store_scalar(371, 0.0);s.store_scalar(372, 0.0);s.store_scalar(431, 0.0);s.store_scalar(428, 0.0);s.store_scalar(430, 0.0);s.store_scalar(433, 0.0);s.store_scalar(434, 0.0);s.store_scalar(435, 0.0);}
        s.b[486] = (s.v[256] == 0.0);s.store_scalar(486, if s.b[486] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[486]) {s.store_scalar(268, 0.0);s.store_scalar(291, 0.0);s.store_scalar(269, 0.0);}
        s.b[487] = (s.v[122] == 0.5);s.store_scalar(487, if s.b[487] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[486])) && s.b[487]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(119)));}
        if ((s.b[418] && (!s.b[486])) && (!s.b[487])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);}
        if (s.b[418] && (!s.b[486])) {s.store_add_scaled_product_mixed_aia(269, A::mul_sub_from_scalar_rhs(s.ad_value(131), 1.0, s.ad_value(436)), 1.0, 134, A::sub(s.ad_value(190), s.ad_value(428)), 1.0);s.store_mul(437, 101, 370);}
        s.b[488] = ((s.v[20] == 0.0) && (s.v[23] == 0.0));s.store_scalar(488, if s.b[488] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[486])) && s.b[488]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {s.store_primal_sub(439, 107, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[489] = (s.v[9] == 0.5);s.store_scalar(489, if s.b[489] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && s.b[489]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && (!s.b[489])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[9])));}
        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {s.store_primal_add(442, 440, 441);}
        s.b[490] = (s.v[9] == 0.5);s.store_scalar(490, if s.b[490] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && s.b[490]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[143]);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[488])) && (!s.b[490])) {s.store_primal_powf_scaled_input(436, 439, s.v[143], s.v[9]);}
        if ((s.b[418] && (!s.b[486])) && (!s.b[488])) {s.store_primal_scale(443, 436, s.v[137]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 98, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[20]);}
        s.b[491] = (s.v[23] == 0.0);s.store_scalar(491, if s.b[491] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[486])) && s.b[491]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[122] * s.v[152]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[492] = (((-s.v[9]) * s.v[125]) == (-1.0));s.store_scalar(492, if s.b[492] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[492]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[492])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));}
        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[149]), 447, 450, s.v[149], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[493] = (s.v[457] > 0.0);s.store_scalar(493, if s.b[493] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[493]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[493])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[494] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(494, if s.b[494] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[494]) {s.store_primal_exp_sub(436, 456, 419);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[494])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[495] = (s.v[457] > 0.0);s.store_scalar(495, if s.b[495] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && s.b[495]) {s.copy_ad(458, 421);}
        s.b[496] = (s.v[456] > (-230.25850929940458));s.store_scalar(496, if s.b[496] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) && s.b[496]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) && (!s.b[496])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[491])) && (!s.b[495])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[486])) && (!s.b[491])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[149] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[23], 0.0, 453);}
        s.b[497] = (s.v[29] == 0.0);s.store_scalar(497, if s.b[497] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[486])) && s.b[497]) {s.store_scalar(460, 0.0);}
        s.b[498] = (s.v[9] == 0.5);s.store_scalar(498, if s.b[498] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && s.b[498]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[498])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[143]), ((s.v[6]) * (s.v[143])), s.v[9]);}
        if ((s.b[418] && (!s.b[486])) && (!s.b[497])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[140]) * s.v[125]), (((s.v[6]) * (s.v[140])) * s.v[125]), 436, 1.0);}
        s.b[499] = (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(499, if s.b[499] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[486])) && (!s.b[497])) && s.b[499]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(155), -1.0, s.ad_value(461), 1.0));}
        s.b[500] = (((-s.v[155]) / s.v[461]) < (-230.25850929940458));s.store_scalar(500, if s.b[500] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[499])) && s.b[500]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 155, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[486])) && (!s.b[497])) && (!s.b[499])) && (!s.b[500])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 155, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[486])) && (!s.b[497])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(190), s.ad_value(461), s.ad_value(461)), 436, s.v[29], 0.0);}
        s.b[501] = ((s.v[38] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(501, if s.b[501] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[486])) && s.b[501]) {s.store_scalar(462, 1.0);}
        s.b[502] = (s.v[435] > ((-s.v[158]) * s.v[38]));s.store_scalar(502, if s.b[502] { 1.0 } else { 0.0 });s.b[503] = (s.v[41] == 4.0);s.store_scalar(503, if s.b[503] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) && s.b[503]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));}
        if ((((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) && (!s.b[503])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);}
        if (((s.b[418] && (!s.b[486])) && (!s.b[501])) && s.b[502]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[486])) && (!s.b[501])) && (!s.b[502])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(38), s.v[158]), s.ad_value(165), s.v[159]);}
        if (s.b[418] && (!s.b[486])) {s.store_mul_add_scaled_inputs4_indices_rhs(268, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(291, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
    ) {
        s.b[504] = (s.v[257] == 0.0);s.store_scalar(504, if s.b[504] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[504]) {s.store_scalar(270, 0.0);s.store_scalar(292, 0.0);s.store_scalar(271, 0.0);}
        s.b[505] = (s.v[123] == 0.5);s.store_scalar(505, if s.b[505] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[504])) && s.b[505]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(120)));}
        if ((s.b[418] && (!s.b[504])) && (!s.b[505])) {s.store_primal_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);}
        if (s.b[418] && (!s.b[504])) {s.store_add_scaled_product_mixed_aia(271, A::mul_sub_from_scalar_rhs(s.ad_value(132), 1.0, s.ad_value(436)), 1.0, 135, A::sub(s.ad_value(190), s.ad_value(428)), 1.0);s.store_mul(437, 102, 371);}
        s.b[506] = ((s.v[21] == 0.0) && (s.v[24] == 0.0));s.store_scalar(506, if s.b[506] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[504])) && s.b[506]) {s.store_scalar(439, 0.0);s.store_scalar(442, 0.0);s.store_scalar(443, 0.0);s.store_scalar(444, 0.0);s.store_scalar(438, 0.0);}
        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {s.store_primal_sub(439, 108, 433);s.store_primal_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));}
        s.b[507] = (s.v[10] == 0.5);s.store_scalar(507, if s.b[507] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && s.b[507]) {s.store_scalar(441, 0.0);}
        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && (!s.b[507])) {s.store_primal_scaled_add_mixed_ai(441, A::div_scaled_product(A::square(s.ad_value(440)), A::ln(s.ad_value(440)), 1.0, A::sub_from_scalar(1.0, s.ad_value(440)), 1.0), 440, (1.0 - (2.0 * s.v[10])));}
        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {s.store_primal_add(442, 440, 441);}
        s.b[508] = (s.v[10] == 0.5);s.store_scalar(508, if s.b[508] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && s.b[508]) {s.store_primal_sqrt_scaled_input(436, 439, s.v[144]);}
        if (((s.b[418] && (!s.b[504])) && (!s.b[506])) && (!s.b[508])) {s.store_primal_powf_scaled_input(436, 439, s.v[144], s.v[10]);}
        if ((s.b[418] && (!s.b[504])) && (!s.b[506])) {s.store_primal_scale(443, 436, s.v[138]);s.store_primal_mul_ad_product_lhs_mixed_ia(444, 99, A::offset(s.ad_value(430), (-1.0)), 443);s.store_primal_scaled_mul(438, 444, 442, s.v[21]);}
        s.b[509] = (s.v[24] == 0.0);s.store_scalar(509, if s.b[509] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[504])) && s.b[509]) {s.store_scalar(445, 0.0);}
        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {s.store_primal_div_scaled_inputs_indices(446, 443, (s.v[123] * s.v[153]), 439, 1.0);s.store_primal_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);s.store_primal_square(448, 447);s.store_primal_sqrt_div_scaled_square_offset_denominator(449, 448, 1.0, 1.0, 1.0);s.store_primal_sqrt_abs_ad(450, s.ad_value(449));s.store_primal_mul(451, 449, 450);}
        s.b[510] = (((-s.v[10]) * s.v[126]) == (-1.0));s.store_scalar(510, if s.b[510] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[510]) {s.store_primal_div_from_scalar_offset_product(452, 1.0, 446, 451, 1.0);}
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[510])) {s.store_primal_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));}
        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {s.store_primal_div_scaled_product_add_scaled_denominator_indices(453, 442, 452, 1.0, 442, 1.0, 452, 1.0, 1.0);s.store_primal_sqrt_scaled_input_ad(454, A::div(s.ad_value(446), s.ad_value(450)), 0.375);s.store_primal_add_scaled_product_indices(455, 449, (-1.0), 447, 450, 2.0);s.store_primal_add_scaled_value_products_indices(456, 449, (-s.v[150]), 447, 450, s.v[150], 446, 451, 0.5);s.store_primal_mul_scale_offset_indices(457, 454, 455, 1.0, (-1.0));s.store_primal_square(419, 457);}
        s.b[511] = (s.v[457] > 0.0);s.store_scalar(511, if s.b[511] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[511]) {s.store_primal_div_from_scalar_offset_scaled_input(420, 1.0, 457, s.v[86], 1.0);}
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[511])) {s.store_primal_div_from_scalar_sub_from_scalar_ad(420, 1.0, 1.0, A::scale(s.ad_value(457), s.v[86]));}
        s.b[512] = (((-s.v[419]) + s.v[456]) > (-230.25850929940458));s.store_scalar(512, if s.b[512] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[512]) {s.store_primal_exp_sub(436, 456, 419);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[512])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(436, 1e-100, (-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {s.store_primal_mul_mixed_ai(421, A::add_scaled_inputs_product(s.ad_value(420), 0.29214664, A::square(s.ad_value(420)), s.v[87], A::square(s.ad_value(420)), s.ad_value(420), s.v[88]), 436);}
        s.b[513] = (s.v[457] > 0.0);s.store_scalar(513, if s.b[513] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && s.b[513]) {s.copy_ad(458, 421);}
        s.b[514] = (s.v[456] > (-230.25850929940458));s.store_scalar(514, if s.b[514] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) && s.b[514]) {s.store_primal_exp(436, 456);}
        if ((((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) && (!s.b[514])) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 456, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[418] && (!s.b[504])) && (!s.b[509])) && (!s.b[513])) {s.store_primal_sub_scaled_inputs(458, 436, 2.0, 421, 1.0);}
        if ((s.b[418] && (!s.b[504])) && (!s.b[509])) {s.store_primal_div_scaled_inputs_indices(459, 458, (s.v[150] * (1.772453850905516 * 0.5)), 454, 1.0);s.store_primal_mul3_affine_lhs(445, 444, 459, s.v[24], 0.0, 453);}
        s.b[515] = (s.v[30] == 0.0);s.store_scalar(515, if s.b[515] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[504])) && s.b[515]) {s.store_scalar(460, 0.0);}
        s.b[516] = (s.v[10] == 0.5);s.store_scalar(516, if s.b[516] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && s.b[516]) {s.store_primal_sqrt_scaled_input_ad(436, A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]);}
        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[516])) {s.store_primal_powf_scale_offset_input(436, 434, (-s.v[144]), ((s.v[7]) * (s.v[144])), s.v[10]);}
        if ((s.b[418] && (!s.b[504])) && (!s.b[515])) {s.store_primal_div_scaled_offset_numerator_indices(461, 434, ((-s.v[141]) * s.v[126]), (((s.v[7]) * (s.v[141])) * s.v[126]), 436, 1.0);}
        s.b[517] = (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458);s.store_scalar(517, if s.b[517] { 1.0 } else { 0.0 });
        if (((s.b[418] && (!s.b[504])) && (!s.b[515])) && s.b[517]) {s.store_primal_ad_value(436, A::exp_div_scaled_inputs(s.ad_value(156), -1.0, s.ad_value(461), 1.0));}
        s.b[518] = (((-s.v[156]) / s.v[461]) < (-230.25850929940458));s.store_scalar(518, if s.b[518] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[517])) && s.b[518]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(436, 1e-100, (-230.25850929940458), 156, -1.0, 461, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[418] && (!s.b[504])) && (!s.b[515])) && (!s.b[517])) && (!s.b[518])) {s.store_primal_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(436, 156, -1.0, 461, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[418] && (!s.b[504])) && (!s.b[515])) {s.store_primal_mul_scale_offset_mixed_ai(460, A::mul3(s.ad_value(190), s.ad_value(461), s.ad_value(461)), 436, s.v[30], 0.0);}
        s.b[519] = ((s.v[39] > 1000000.0) || (p.p80 == 0.0));s.store_scalar(519, if s.b[519] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[504])) && s.b[519]) {s.store_scalar(462, 1.0);}
        s.b[520] = (s.v[435] > ((-s.v[158]) * s.v[39]));s.store_scalar(520, if s.b[520] { 1.0 } else { 0.0 });s.b[521] = (s.v[42] == 4.0);s.store_scalar(521, if s.b[521] { 1.0 } else { 0.0 });
        if ((((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) && s.b[521]) {s.store_primal_mul3_ad(436, A::square(A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));}
        if ((((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) && (!s.b[521])) {s.store_primal_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);}
        if (((s.b[418] && (!s.b[504])) && (!s.b[519])) && s.b[520]) {s.store_primal_div_from_scalar_sub_from_scalar_ad(462, 1.0, 1.0, s.ad_value(436));}
        if (((s.b[418] && (!s.b[504])) && (!s.b[519])) && (!s.b[520])) {s.store_primal_offset_mul_ad(462, A::add_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(39), s.v[158]), s.ad_value(166), s.v[160]);}
        if (s.b[418] && (!s.b[504])) {s.store_mul_add_scaled_inputs4_indices_rhs(270, 462, 437, 1.0, 438, 1.0, 445, 1.0, 460, 1.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(292, 462, 438, 1.0, 445, 1.0, 460, 1.0, 0.0);}
        s.b[522] = (s.v[258] == 0.0);s.store_scalar(522, if s.b[522] { 1.0 } else { 0.0 });
        if (s.b[418] && s.b[522]) {s.store_scalar(272, 0.0);s.store_scalar(293, 0.0);s.store_scalar(273, 0.0);}
        s.b[523] = (s.v[124] == 0.5);s.store_scalar(523, if s.b[523] { 1.0 } else { 0.0 });
        if ((s.b[418] && (!s.b[522])) && s.b[523]) {s.store_primal_sqrt_sub_from_scalar_ad(436, 1.0, A::mul(s.ad_value(428), s.ad_value(121)));}
    }
}
