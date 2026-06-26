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
        s.v[1] = (8.8541878176e-12 * 11.8);

        s.v[112] = 0.0;

        s.b[187] = (p.p62 > 0.5);
        s.v[187] = if s.b[187] { 1.0 } else { 0.0 };

        if s.b[187] {
            s.store_scalar(112, 1.0);
        }

        if (!s.b[187]) {
            s.store_scalar(112, 0.0);
        }

        s.v[2] = (273.15 + p.p13);

        s.v[5] = (1.3806505e-23 / 1.6021918e-19);

        s.v[6] = (s.v[5] * s.v[2]);

        s.v[7] = (1.0 / s.v[6]);

        s.v[13] = ((-((0.000702 * s.v[2]) * s.v[2])) / (1108.0 + s.v[2]));

        s.v[16] = (p.p24 + s.v[13]);

        s.v[17] = (p.p25 + s.v[13]);

        s.v[18] = (p.p26 + s.v[13]);

        s.v[46] = (1.0 - p.p21);

        s.v[47] = (1.0 - p.p22);

        s.v[48] = (1.0 - p.p23);

        s.v[49] = (1.0 / s.v[46]);

        s.v[50] = (1.0 / s.v[47]);

        s.v[51] = (1.0 / s.v[48]);

        s.v[61] = (s.v[1] / p.p15);

        s.v[62] = ((p.p33 * s.v[1]) / p.p16);

        s.v[63] = ((p.p34 * s.v[1]) / p.p17);

        s.v[64] = (1.0 / s.v[61]);

        s.v[65] = (1.0 / s.v[62]);

        s.v[66] = (1.0 / s.v[63]);

        s.v[67] = (1.0 / p.p18);

        s.v[68] = (1.0 / p.p19);

        s.v[69] = (1.0 / p.p20);

        s.v[10] = (1.772453850905516 * 0.29214664);

        s.v[11] = (((((-5.0) * 0.29214664) + 6.0) - ((s.v[10]) as f64).powf((-2.0))) / 3.0);

        s.v[12] = ((1.0 - 0.29214664) - s.v[11]);

        s.v[82] = (1.0 - (1.0 / p.p14));

        s.v[83] = (1.0 / (1.0 - ((s.v[82]) as f64).powf(p.p53)));

        s.v[84] = (1.0 / (1.0 - ((s.v[82]) as f64).powf(p.p54)));

        s.v[85] = (1.0 / (1.0 - ((s.v[82]) as f64).powf(p.p55)));

        s.v[86] = (1.0 / p.p50);

        s.v[87] = (1.0 / p.p51);

        s.v[88] = (1.0 / p.p52);

        s.v[89] = (((-((s.v[83] * s.v[83]) * ((s.v[82]) as f64).powf((p.p53 - 1.0)))) * p.p53) * s.v[86]);

        s.v[90] = (((-((s.v[84] * s.v[84]) * ((s.v[82]) as f64).powf((p.p54 - 1.0)))) * p.p54) * s.v[87]);

        s.v[91] = (((-((s.v[85] * s.v[85]) * ((s.v[82]) as f64).powf((p.p55 - 1.0)))) * p.p55) * s.v[88]);

        s.b[188] = ((((p.p56 != 1.0) || (p.p57 != 1.0)) || (p.p58 != 1.0)) || (p.p59 != 1.0));
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if s.b[188] {
            s.store_scalar(111, 1.0);
        }

        if (!s.b[188]) {
            s.store_scalar(111, 0.0);
        }

        s.b[189] = (s.v[111] == 1.0);
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        if s.b[189] {
            s.store_scalar(95, (if ((p.p17 * p.p56) > 1e-18) { (p.p17 * p.p56) } else { 1e-18 }));
        }

        if s.b[189] {
            s.store_scalar(96, (if ((p.p20 * p.p57) > 0.05) { (p.p20 * p.p57) } else { 0.05 }));
        }

        if s.b[189] {
            s.store_scalar(97, (if ((if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) < 0.95) { (if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[189] {
            s.store_scalar(98, (p.p26 * p.p59));
            s.store_offset(100, 98, s.v[13]);
            s.store_sub_from_scalar(105, 1.0, 97);
            s.store_div_from_scalar(106, 1.0, 105);
        }

        s.v[3] = (((ctx_temp + p.p2) + p.p9)).max((273.15 + (-250.0)));

        s.v[4] = (s.v[3] / s.v[2]);

        s.v[8] = (s.v[5] * s.v[3]);

        s.v[9] = (1.0 / s.v[8]);

        s.v[14] = ((-((0.000702 * s.v[3]) * s.v[3])) / (1108.0 + s.v[3]));

        s.v[19] = (p.p24 + s.v[14]);

        s.v[20] = (p.p25 + s.v[14]);

        s.v[21] = (p.p26 + s.v[14]);

        s.v[22] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[16] * s.v[7]) - (s.v[19] * s.v[9])))) as f64).exp());

        s.v[23] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[17] * s.v[7]) - (s.v[20] * s.v[9])))) as f64).exp());

        s.v[24] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[18] * s.v[7]) - (s.v[21] * s.v[9])))) as f64).exp());

        s.v[25] = ((p.p27 * s.v[22]) * s.v[22]);

        s.v[26] = ((p.p28 * s.v[23]) * s.v[23]);

        s.v[27] = ((p.p29 * s.v[24]) * s.v[24]);

        s.v[28] = ((p.p18 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[22]) as f64).ln()));

        s.v[29] = ((p.p19 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[23]) as f64).ln()));

        s.v[30] = ((p.p20 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[24]) as f64).ln()));

        s.v[31] = (s.v[28] + (s.v[8] * (((1.0 + ((((0.05 - s.v[28]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[32] = (s.v[29] + (s.v[8] * (((1.0 + ((((0.05 - s.v[29]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[33] = (s.v[30] + (s.v[8] * (((1.0 + ((((0.05 - s.v[30]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[43] = (1.0 / s.v[31]);

        s.v[44] = (1.0 / s.v[32]);

        s.v[45] = (1.0 / s.v[33]);

        s.v[52] = (p.p15 * (((p.p18 * s.v[43])) as f64).powf(p.p21));

        s.v[53] = (p.p16 * (((p.p19 * s.v[44])) as f64).powf(p.p22));

        s.v[54] = (p.p17 * (((p.p20 * s.v[45])) as f64).powf(p.p23));

        s.v[55] = ((s.v[52] * s.v[31]) * s.v[49]);

        s.v[56] = ((s.v[53] * s.v[32]) * s.v[50]);

        s.v[57] = ((s.v[54] * s.v[33]) * s.v[51]);

        s.v[58] = (2.0 * s.v[52]);

        s.v[59] = (2.0 * s.v[53]);

        s.v[60] = (2.0 * s.v[54]);

        s.v[70] = ((0.5 * s.v[19])).max(s.v[8]);

        s.v[71] = ((0.5 * s.v[20])).max(s.v[8]);

        s.v[72] = ((0.5 * s.v[21])).max(s.v[8]);

        s.v[73] = (s.v[70] * s.v[9]);

        s.v[74] = (s.v[71] * s.v[9]);

        s.v[75] = (s.v[72] * s.v[9]);

        s.v[76] = (((((((32.0 * p.p38) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[70] * s.v[70]) * s.v[70]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[77] = (((((((32.0 * p.p39) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[71] * s.v[71]) * s.v[71]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[78] = (((((((32.0 * p.p40) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[72] * s.v[72]) * s.v[72]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[79] = (p.p44 * (1.0 + (p.p47 * (s.v[3] - s.v[2]))));

        s.v[80] = (p.p45 * (1.0 + (p.p48 * (s.v[3] - s.v[2]))));

        s.v[81] = (p.p46 * (1.0 + (p.p49 * (s.v[3] - s.v[2]))));

        if (!(s.v[79] > 0.0)) {
            s.store_scalar(79, 0.0);
        }

        if (!(s.v[80] > 0.0)) {
            s.store_scalar(80, 0.0);
        }

        if (!(s.v[81] > 0.0)) {
            s.store_scalar(81, 0.0);
        }

        s.b[190] = (s.v[111] == 1.0);
        s.v[190] = if s.b[190] { 1.0 } else { 0.0 };

        if s.b[190] {
            s.store_offset(99, 98, s.v[14]);
            s.store_scale_ad(101, A::exp(A::scale(A::sub(A::scale(s.ad_value(100), s.v[7]), A::scale(s.ad_value(99), s.v[9])), 0.5)), ((s.v[4]) as f64).powf(1.5));
            s.store_sub_scaled_ad_rhs(102, 96, s.v[4], A::scale(A::ln(s.ad_value(101)), (2.0 * s.v[8])));
            s.store_add_ad_rhs(103, 102, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(102)), s.v[9])), 1.0)), s.v[8]));
            s.store_div_from_scalar(104, 1.0, 103);
            s.store_mul_pow_ad_rhs(107, 95, A::mul(s.ad_value(96), s.ad_value(104)), s.ad_value(97));
            s.store_mul3_lhs(108, 107, 103, 106);
            s.store_scale(109, 107, 2.0);
        }

        s.v[143] = (if (p.p3 > 0.0) { p.p3 } else { 0.0 });

        s.v[144] = (if (p.p4 > 0.0) { p.p4 } else { 0.0 });

        s.v[145] = (if (p.p5 > 0.0) { p.p5 } else { 0.0 });

        s.v[0] = (if (p.p6 > 0.0) { p.p6 } else { 0.0 });

        s.v[150] = 0.0;

        s.b[191] = ((s.v[25] * s.v[143]) > 0.0);
        s.v[191] = if s.b[191] { 1.0 } else { 0.0 };

        if s.b[191] {
            s.store_scalar(92, (s.v[8] * ((((p.p12 / (s.v[25] * s.v[143])) + 1.0)) as f64).ln()));
        }

        if (!s.b[191]) {
            s.store_scalar(92, 100000000.0);
        }

        s.b[192] = ((s.v[26] * s.v[144]) > 0.0);
        s.v[192] = if s.b[192] { 1.0 } else { 0.0 };

        if s.b[192] {
            s.store_scalar(93, (s.v[8] * ((((p.p12 / (s.v[26] * s.v[144])) + 1.0)) as f64).ln()));
        }

        if (!s.b[192]) {
            s.store_scalar(93, 100000000.0);
        }

        s.b[193] = ((s.v[27] * s.v[145]) > 0.0);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if s.b[193] {
            s.store_scalar(94, (s.v[8] * ((((p.p12 / (s.v[27] * s.v[145])) + 1.0)) as f64).ln()));
        }

        if (!s.b[193]) {
            s.store_scalar(94, 100000000.0);
        }

        s.store_min3(149, 92, 93, 94);

        s.b[194] = ((((s.v[149] * s.v[9])) as f64).abs() < 230.25850929940458);
        s.v[194] = if s.b[194] { 1.0 } else { 0.0 };

        if s.b[194] {
            s.store_exp_scaled_input(150, 149, s.v[9]);
        }

        s.b[195] = ((s.v[149] * s.v[9]) < 0.0);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if ((!s.b[194]) && s.b[195]) {
            s.store_div_from_scalar_offset_ad(150, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((!s.b[194]) && (!s.b[195])) {
            s.store_scaled_offset_ad(150, A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        s.v[34] = s.v[31];

        s.v[35] = s.v[32];

        s.v[36] = s.v[33];

        s.v[37] = p.p21;

        s.v[38] = p.p22;

        s.v[39] = p.p23;

        s.v[40] = p.p18;

        s.v[41] = p.p19;

        s.v[42] = p.p20;

        s.b[196] = (s.v[143] == 0.0);
        s.v[196] = if s.b[196] { 1.0 } else { 0.0 };

        if s.b[196] {
            s.store_scalar(34, (s.v[32] + s.v[33]));
            s.store_scalar(37, (0.9 * (p.p22).min(p.p23)));
            s.store_scalar(40, (p.p19 + p.p20));
        }

        s.b[197] = (s.v[144] == 0.0);
        s.v[197] = if s.b[197] { 1.0 } else { 0.0 };

        if s.b[197] {
            s.store_scalar(35, (s.v[31] + s.v[33]));
            s.store_scalar(38, (0.9 * (p.p21).min(p.p23)));
            s.store_scalar(41, (p.p18 + p.p20));
        }

        s.b[198] = (s.v[145] == 0.0);
        s.v[198] = if s.b[198] { 1.0 } else { 0.0 };

        if s.b[198] {
            s.store_scalar(36, (s.v[31] + s.v[32]));
            s.store_scalar(39, (0.9 * (p.p21).min(p.p22)));
            s.store_scalar(42, (p.p18 + p.p19));
        }

        s.store_min3(151, 34, 35, 36);

        s.store_scale(152, 151, 0.1);

        s.store_max3(15, 37, 38, 39);

        s.store_mul_sub_from_scalar_ad_rhs(153, 151, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(15))));

        s.store_offset_min_ad(154, A::min(s.ad_value(40), s.ad_value(41)), s.ad_value(42), (-0.05));

        s.v[161] = 0.0;

        s.v[162] = 1.0;

        s.v[164] = 1.0;

        s.v[163] = 0.0;

        s.v[166] = 1.0;

        s.v[165] = 0.0;

        s.v[167] = 0.0;

        s.v[155] = 0.0;

        s.v[156] = 0.0;

        s.v[157] = 0.0;

        s.v[158] = 0.0;

        s.v[159] = 0.0;

        s.v[160] = 0.0;

        s.v[129] = 0.0;

        s.v[130] = 0.0;

        s.v[118] = 0.0;

        s.v[119] = 0.0;

        s.v[120] = 0.0;

        s.v[121] = 0.0;

        s.v[122] = 0.0;

        s.v[131] = 0.0;

        s.v[132] = 0.0;

        s.v[133] = 0.0;

        s.v[139] = 0.0;

        s.v[146] = 1.0;

        s.v[147] = 1.0;

        s.v[148] = 1.0;

        s.v[128] = 0.0;

        s.b[199] = (s.v[112] == 1.0);
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_scalar(200, 0.0);
            s.store_scalar(201, 0.0);
            s.store_scalar(202, 0.0);
            s.store_scalar(209, 0.0);
            s.store_scalar(211, 0.0);
            s.store_scalar(212, 0.0);
            s.store_scalar(213, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(218, 0.0);
            s.store_scalar(219, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[199] {
            s.store_scalar(220, 0.0);
            s.store_scalar(221, 0.0);
            s.store_scalar(222, 0.0);
            s.store_scalar(223, 0.0);
            s.store_scalar(224, 0.0);
            s.store_scalar(225, 0.0);
            s.store_scalar(226, 0.0);
            s.store_scalar(227, 0.0);
            s.store_scalar(228, 0.0);
            s.store_scalar(229, 0.0);
            s.store_scalar(230, 0.0);
            s.store_scalar(231, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(234, 0.0);
            s.store_scalar(235, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(237, 0.0);
            s.store_scalar(238, 0.0);
            s.store_scalar(239, 0.0);
            s.store_scalar(240, 0.0);
            s.store_scalar(241, 0.0);
            s.store_scalar(242, 0.0);
            s.store_scalar(243, 0.0);
            s.store_scalar(244, 0.0);
            s.store_scalar(136, 0.4);
            s.store_scalar(137, 0.65);
            s.store_scalar(138, 0.8);
            s.store_scale(123, 136, (-p.p63));
            s.store_scale(124, 137, (-p.p63));
            s.store_scale(125, 138, (-p.p63));
            s.store_scalar(126, 0.1);
            s.store_scalar(127, 0.2);
            s.store_scalar(216, 0.0);
            s.store_scalar(213, 0.0);
        }

        s.b[248] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[248] = if s.b[248] { 1.0 } else { 0.0 };

        s.b[249] = (s.v[123] < s.v[149]);
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        s.b[250] = (((((-0.5) * (s.v[123] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if (((s.b[199] && s.b[248]) && s.b[249]) && s.b[250]) {
            s.store_exp_scaled_input(211, 123, (s.v[9] * (-0.5)));
        }

        s.b[251] = (((-0.5) * (s.v[123] * s.v[9])) < 0.0);
        s.v[251] = if s.b[251] { 1.0 } else { 0.0 };

        if ((((s.b[199] && s.b[248]) && s.b[249]) && (!s.b[250])) && s.b[251]) {
            let assign2500_ad_e1541: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(123), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(123), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(123), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(211, assign2500_ad_e1541);
        }

        if ((((s.b[199] && s.b[248]) && s.b[249]) && (!s.b[250])) && (!s.b[251])) {
            s.store_scaled_offset_ad(211, A::mul(A::offset(A::scale(s.ad_value(123), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(123), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(123), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && s.b[248]) && s.b[249]) {
            s.store_div_from_scalar(212, 1.0, 211);
            s.store_square(209, 212);
        }

        if ((s.b[199] && s.b[248]) && (!s.b[249])) {
            s.store_mul_offset_ad_lhs(209, A::scale(A::sub(s.ad_value(123), s.ad_value(149)), s.v[9]), 1.0, 150);
            s.store_sqrt(212, 209);
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if (s.b[199] && s.b[248]) {
            s.store_offset(209, 209, (-1.0));
        }

        s.b[252] = (s.v[123] > 0.0);
        s.v[252] = if s.b[252] { 1.0 } else { 0.0 };

        if ((s.b[199] && s.b[248]) && s.b[252]) {
            s.store_scaled_ln_ad(213, A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0)))), (s.v[8] * 2.0));
        }

        if ((s.b[199] && s.b[248]) && (!s.b[252])) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 123);
        }

        if (s.b[199] && s.b[248]) {
            s.store_sub(214, 151, 213);
            s.store_scaled_sub_ad(215, A::add(s.ad_value(123), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(123), s.ad_value(214)), A::sub(s.ad_value(123), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8]))), 0.5);
            s.store_scaled_sub_ad(216, A::add(s.ad_value(123), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(123), s.ad_value(154)), A::sub(s.ad_value(123), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6]))), 0.5);
            s.store_scaled_sub_ad_rhs(217, 123, A::sqrt(A::offset(A::mul(s.ad_value(123), s.ad_value(123)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[253] = (s.v[143] == 0.0);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[253]) {
            s.store_scalar(245, 0.0);
        }

        if (s.b[199] && (!s.b[253])) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.b[254] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[253])) && s.b[254]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[254])) {
            s.store_sub_from_scalar(221, s.v[31], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[255] = (p.p21 == 0.5);
        s.v[255] = if s.b[255] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[254])) && s.b[255]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[254])) && (!s.b[255])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p21)));
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[254])) {
            s.store_add(224, 222, 223);
        }

        s.b[256] = (p.p21 == 0.5);
        s.v[256] = if s.b[256] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[254])) && s.b[256]) {
            s.store_sqrt_scaled_input(218, 221, s.v[67]);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[254])) && (!s.b[256])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[254])) {
            s.store_scale(225, 218, s.v[61]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[22]);
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.b[257] = (p.p35 == 0.0);
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[253])) && s.b[257]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[257])) {
            s.store_scaled_div(228, 225, 221, ((s.v[46]) * (s.v[76])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[258] = (((-p.p21) * s.v[49]) == (-1.0));
        s.v[258] = if s.b[258] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && s.b[258]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && (!s.b[258])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[257])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[259] = (s.v[239] > 0.0);
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && s.b[259]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && (!s.b[259])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[260] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[260] = if s.b[260] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && s.b[260]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && (!s.b[260])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[257])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[261] = (s.v[239] > 0.0);
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && s.b[261]) {
            s.copy_ad(240, 202);
        }

        s.b[262] = (s.v[238] > (-230.25850929940458));
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[253])) && (!s.b[257])) && (!s.b[261])) && s.b[262]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[253])) && (!s.b[257])) && (!s.b[261])) && (!s.b[262])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[257])) && (!s.b[261])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[257])) {
            s.store_scaled_div(241, 240, 236, ((s.v[73]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p35);
        }

        s.b[263] = (p.p41 == 0.0);
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[253])) && s.b[263]) {
            s.store_scalar(242, 0.0);
        }

        s.b[264] = (p.p21 == 0.5);
        s.v[264] = if s.b[264] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[263])) && s.b[264]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[263])) && (!s.b[264])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[263])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), 218, s.v[49]);
        }

        s.b[265] = (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[265] = if s.b[265] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[253])) && (!s.b[263])) && s.b[265]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.b[266] = (((-s.v[79]) / s.v[243]) < 0.0);
        s.v[266] = if s.b[266] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[253])) && (!s.b[263])) && (!s.b[265])) && s.b[266]) {
            let assign3230_ad_e2648: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign3230_ad_e2648);
        }

        if ((((s.b[199] && (!s.b[253])) && (!s.b[263])) && (!s.b[265])) && (!s.b[266])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[253])) && (!s.b[263])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(123), s.ad_value(243)), s.ad_value(243)), 218, p.p41);
        }

        s.b[267] = (p.p50 > 1000.0);
        s.v[267] = if s.b[267] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[253])) && s.b[267]) {
            s.store_scalar(244, 1.0);
        }

        s.b[268] = (s.v[217] > ((-s.v[82]) * p.p50));
        s.v[268] = if s.b[268] { 1.0 } else { 0.0 };

        s.b[269] = (p.p53 == 4.0);
        s.v[269] = if s.b[269] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[253])) && (!s.b[267])) && s.b[268]) && s.b[269]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), 217, s.v[86]);
        }

        if ((((s.b[199] && (!s.b[253])) && (!s.b[267])) && s.b[268]) && (!s.b[269])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[267])) && s.b[268]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[253])) && (!s.b[267])) && (!s.b[268])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89], s.v[83]);
        }

        if (s.b[199] && (!s.b[253])) {
            s.store_mul_scale_ad_lhs(245, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[270] = (s.v[144] == 0.0);
        s.v[270] = if s.b[270] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[270]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[199] && (!s.b[270])) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.b[271] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[271] = if s.b[271] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[270])) && s.b[271]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[271])) {
            s.store_sub_from_scalar(221, s.v[32], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[272] = (p.p22 == 0.5);
        s.v[272] = if s.b[272] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[199] && (!s.b[270])) && (!s.b[271])) && s.b[272]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[271])) && (!s.b[272])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p22)));
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[271])) {
            s.store_add(224, 222, 223);
        }

        s.b[273] = (p.p22 == 0.5);
        s.v[273] = if s.b[273] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[270])) && (!s.b[271])) && s.b[273]) {
            s.store_sqrt_scaled_input(218, 221, s.v[68]);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[271])) && (!s.b[273])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[271])) {
            s.store_scale(225, 218, s.v[62]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[23]);
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.b[274] = (p.p36 == 0.0);
        s.v[274] = if s.b[274] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[270])) && s.b[274]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[274])) {
            s.store_scaled_div(228, 225, 221, ((s.v[47]) * (s.v[77])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[275] = (((-p.p22) * s.v[50]) == (-1.0));
        s.v[275] = if s.b[275] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && s.b[275]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && (!s.b[275])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[274])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[276] = (s.v[239] > 0.0);
        s.v[276] = if s.b[276] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && s.b[276]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && (!s.b[276])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[277] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[277] = if s.b[277] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && s.b[277]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && (!s.b[277])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[274])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[278] = (s.v[239] > 0.0);
        s.v[278] = if s.b[278] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && s.b[278]) {
            s.copy_ad(240, 202);
        }

        s.b[279] = (s.v[238] > (-230.25850929940458));
        s.v[279] = if s.b[279] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[270])) && (!s.b[274])) && (!s.b[278])) && s.b[279]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[270])) && (!s.b[274])) && (!s.b[278])) && (!s.b[279])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[274])) && (!s.b[278])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[274])) {
            s.store_scaled_div(241, 240, 236, ((s.v[74]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p36);
        }

        s.b[280] = (p.p42 == 0.0);
        s.v[280] = if s.b[280] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[270])) && s.b[280]) {
            s.store_scalar(242, 0.0);
        }

        s.b[281] = (p.p22 == 0.5);
        s.v[281] = if s.b[281] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[270])) && (!s.b[280])) && s.b[281]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[280])) && (!s.b[281])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[280])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), 218, s.v[50]);
        }

        s.b[282] = (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[282] = if s.b[282] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[270])) && (!s.b[280])) && s.b[282]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.b[283] = (((-s.v[80]) / s.v[243]) < 0.0);
        s.v[283] = if s.b[283] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[270])) && (!s.b[280])) && (!s.b[282])) && s.b[283]) {
            let assign3930_ad_e3685: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign3930_ad_e3685);
        }

        if ((((s.b[199] && (!s.b[270])) && (!s.b[280])) && (!s.b[282])) && (!s.b[283])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[270])) && (!s.b[280])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(123), s.ad_value(243)), s.ad_value(243)), 218, p.p42);
        }

        s.b[284] = (p.p51 > 1000.0);
        s.v[284] = if s.b[284] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[270])) && s.b[284]) {
            s.store_scalar(244, 1.0);
        }

        s.b[285] = (s.v[217] > ((-s.v[82]) * p.p51));
        s.v[285] = if s.b[285] { 1.0 } else { 0.0 };

        s.b[286] = (p.p54 == 4.0);
        s.v[286] = if s.b[286] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[270])) && (!s.b[284])) && s.b[285]) && s.b[286]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), 217, s.v[87]);
        }

        if ((((s.b[199] && (!s.b[270])) && (!s.b[284])) && s.b[285]) && (!s.b[286])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[284])) && s.b[285]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[270])) && (!s.b[284])) && (!s.b[285])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90], s.v[84]);
        }

        if (s.b[199] && (!s.b[270])) {
            s.store_mul_scale_ad_lhs(246, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[287] = (s.v[145] == 0.0);
        s.v[287] = if s.b[287] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[287]) {
            s.store_scalar(247, 0.0);
        }

        if (s.b[199] && (!s.b[287])) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.b[288] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[287])) && s.b[288]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[288])) {
            s.store_sub_from_scalar(221, s.v[33], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[289] = (p.p23 == 0.5);
        s.v[289] = if s.b[289] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[288])) && s.b[289]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[288])) && (!s.b[289])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p23)));
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[288])) {
            s.store_add(224, 222, 223);
        }

        s.b[290] = (p.p23 == 0.5);
        s.v[290] = if s.b[290] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[288])) && s.b[290]) {
            s.store_sqrt_scaled_input(218, 221, s.v[69]);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[288])) && (!s.b[290])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[288])) {
            s.store_scale(225, 218, s.v[63]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[24]);
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.b[291] = (p.p37 == 0.0);
        s.v[291] = if s.b[291] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[287])) && s.b[291]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[291])) {
            s.store_scaled_div(228, 225, 221, ((s.v[48]) * (s.v[78])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[292] = (((-p.p23) * s.v[51]) == (-1.0));
        s.v[292] = if s.b[292] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && s.b[292]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && (!s.b[292])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[291])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[293] = (s.v[239] > 0.0);
        s.v[293] = if s.b[293] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && s.b[293]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && (!s.b[293])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[294] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[294] = if s.b[294] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && s.b[294]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && (!s.b[294])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[291])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[295] = (s.v[239] > 0.0);
        s.v[295] = if s.b[295] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && s.b[295]) {
            s.copy_ad(240, 202);
        }

        s.b[296] = (s.v[238] > (-230.25850929940458));
        s.v[296] = if s.b[296] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[287])) && (!s.b[291])) && (!s.b[295])) && s.b[296]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[287])) && (!s.b[291])) && (!s.b[295])) && (!s.b[296])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[291])) && (!s.b[295])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[291])) {
            s.store_scaled_div(241, 240, 236, ((s.v[75]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p37);
        }

        s.b[297] = (p.p43 == 0.0);
        s.v[297] = if s.b[297] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[287])) && s.b[297]) {
            s.store_scalar(242, 0.0);
        }

        s.b[298] = (p.p23 == 0.5);
        s.v[298] = if s.b[298] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[297])) && s.b[298]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[297])) && (!s.b[298])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[297])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), 218, s.v[51]);
        }

        s.b[299] = (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[299] = if s.b[299] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[287])) && (!s.b[297])) && s.b[299]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.b[300] = (((-s.v[81]) / s.v[243]) < 0.0);
        s.v[300] = if s.b[300] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[287])) && (!s.b[297])) && (!s.b[299])) && s.b[300]) {
            let assign4630_ad_e4722: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign4630_ad_e4722);
        }

        if ((((s.b[199] && (!s.b[287])) && (!s.b[297])) && (!s.b[299])) && (!s.b[300])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[287])) && (!s.b[297])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(123), s.ad_value(243)), s.ad_value(243)), 218, p.p43);
        }

        s.b[301] = (p.p52 > 1000.0);
        s.v[301] = if s.b[301] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[287])) && s.b[301]) {
            s.store_scalar(244, 1.0);
        }

        s.b[302] = (s.v[217] > ((-s.v[82]) * p.p52));
        s.v[302] = if s.b[302] { 1.0 } else { 0.0 };

        s.b[303] = (p.p55 == 4.0);
        s.v[303] = if s.b[303] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[287])) && (!s.b[301])) && s.b[302]) && s.b[303]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), 217, s.v[88]);
        }

        if ((((s.b[199] && (!s.b[287])) && (!s.b[301])) && s.b[302]) && (!s.b[303])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[301])) && s.b[302]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[287])) && (!s.b[301])) && (!s.b[302])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91], s.v[85]);
        }

        if (s.b[199] && (!s.b[287])) {
            s.store_mul_scale_ad_lhs(247, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        if s.b[199] {
            s.store_add_scaled_ad_lhs(113, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), 247, s.v[145]);
            s.store_scalar(216, 0.0);
            s.store_scalar(213, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[304] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[304] = if s.b[304] { 1.0 } else { 0.0 };

        s.b[305] = (s.v[124] < s.v[149]);
        s.v[305] = if s.b[305] { 1.0 } else { 0.0 };

        s.b[306] = (((((-0.5) * (s.v[124] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[306] = if s.b[306] { 1.0 } else { 0.0 };

        if (((s.b[199] && s.b[304]) && s.b[305]) && s.b[306]) {
            s.store_exp_scaled_input(211, 124, (s.v[9] * (-0.5)));
        }

        s.b[307] = (((-0.5) * (s.v[124] * s.v[9])) < 0.0);
        s.v[307] = if s.b[307] { 1.0 } else { 0.0 };

        if ((((s.b[199] && s.b[304]) && s.b[305]) && (!s.b[306])) && s.b[307]) {
            let assign4900_ad_e5103: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(124), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(124), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(124), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(211, assign4900_ad_e5103);
        }

        if ((((s.b[199] && s.b[304]) && s.b[305]) && (!s.b[306])) && (!s.b[307])) {
            s.store_scaled_offset_ad(211, A::mul(A::offset(A::scale(s.ad_value(124), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(124), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(124), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && s.b[304]) && s.b[305]) {
            s.store_div_from_scalar(212, 1.0, 211);
            s.store_square(209, 212);
        }

        if ((s.b[199] && s.b[304]) && (!s.b[305])) {
            s.store_mul_offset_ad_lhs(209, A::scale(A::sub(s.ad_value(124), s.ad_value(149)), s.v[9]), 1.0, 150);
            s.store_sqrt(212, 209);
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if (s.b[199] && s.b[304]) {
            s.store_offset(209, 209, (-1.0));
        }

        s.b[308] = (s.v[124] > 0.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        if ((s.b[199] && s.b[304]) && s.b[308]) {
            s.store_scaled_ln_ad(213, A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0)))), (s.v[8] * 2.0));
        }

        if ((s.b[199] && s.b[304]) && (!s.b[308])) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 124);
        }

        if (s.b[199] && s.b[304]) {
            s.store_sub(214, 151, 213);
            s.store_scaled_sub_ad(215, A::add(s.ad_value(124), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(124), s.ad_value(214)), A::sub(s.ad_value(124), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8]))), 0.5);
            s.store_scaled_sub_ad(216, A::add(s.ad_value(124), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(124), s.ad_value(154)), A::sub(s.ad_value(124), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6]))), 0.5);
            s.store_scaled_sub_ad_rhs(217, 124, A::sqrt(A::offset(A::mul(s.ad_value(124), s.ad_value(124)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[309] = (s.v[143] == 0.0);
        s.v[309] = if s.b[309] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[309]) {
            s.store_scalar(245, 0.0);
        }

        if (s.b[199] && (!s.b[309])) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.b[310] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[310] = if s.b[310] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[309])) && s.b[310]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[310])) {
            s.store_sub_from_scalar(221, s.v[31], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[311] = (p.p21 == 0.5);
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[310])) && s.b[311]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[310])) && (!s.b[311])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p21)));
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[310])) {
            s.store_add(224, 222, 223);
        }

        s.b[312] = (p.p21 == 0.5);
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[310])) && s.b[312]) {
            s.store_sqrt_scaled_input(218, 221, s.v[67]);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[310])) && (!s.b[312])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[310])) {
            s.store_scale(225, 218, s.v[61]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[22]);
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.b[313] = (p.p35 == 0.0);
        s.v[313] = if s.b[313] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[309])) && s.b[313]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[313])) {
            s.store_scaled_div(228, 225, 221, ((s.v[46]) * (s.v[76])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[314] = (((-p.p21) * s.v[49]) == (-1.0));
        s.v[314] = if s.b[314] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && s.b[314]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && (!s.b[314])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[313])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[315] = (s.v[239] > 0.0);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && s.b[315]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && (!s.b[315])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[316] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && s.b[316]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && (!s.b[316])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[313])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[317] = (s.v[239] > 0.0);
        s.v[317] = if s.b[317] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && s.b[317]) {
            s.copy_ad(240, 202);
        }

        s.b[318] = (s.v[238] > (-230.25850929940458));
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[309])) && (!s.b[313])) && (!s.b[317])) && s.b[318]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[309])) && (!s.b[313])) && (!s.b[317])) && (!s.b[318])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[313])) && (!s.b[317])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[313])) {
            s.store_scaled_div(241, 240, 236, ((s.v[73]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p35);
        }

        s.b[319] = (p.p41 == 0.0);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[309])) && s.b[319]) {
            s.store_scalar(242, 0.0);
        }

        s.b[320] = (p.p21 == 0.5);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[319])) && s.b[320]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[319])) && (!s.b[320])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[319])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), 218, s.v[49]);
        }

        s.b[321] = (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[321] = if s.b[321] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[309])) && (!s.b[319])) && s.b[321]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.b[322] = (((-s.v[79]) / s.v[243]) < 0.0);
        s.v[322] = if s.b[322] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[309])) && (!s.b[319])) && (!s.b[321])) && s.b[322]) {
            let assign5630_ad_e6210: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign5630_ad_e6210);
        }

        if ((((s.b[199] && (!s.b[309])) && (!s.b[319])) && (!s.b[321])) && (!s.b[322])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[309])) && (!s.b[319])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(124), s.ad_value(243)), s.ad_value(243)), 218, p.p41);
        }

        s.b[323] = (p.p50 > 1000.0);
        s.v[323] = if s.b[323] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[309])) && s.b[323]) {
            s.store_scalar(244, 1.0);
        }

        s.b[324] = (s.v[217] > ((-s.v[82]) * p.p50));
        s.v[324] = if s.b[324] { 1.0 } else { 0.0 };

        s.b[325] = (p.p53 == 4.0);
        s.v[325] = if s.b[325] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[309])) && (!s.b[323])) && s.b[324]) && s.b[325]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), 217, s.v[86]);
        }

        if ((((s.b[199] && (!s.b[309])) && (!s.b[323])) && s.b[324]) && (!s.b[325])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[323])) && s.b[324]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[309])) && (!s.b[323])) && (!s.b[324])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89], s.v[83]);
        }

        if (s.b[199] && (!s.b[309])) {
            s.store_mul_scale_ad_lhs(245, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[326] = (s.v[144] == 0.0);
        s.v[326] = if s.b[326] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[326]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[199] && (!s.b[326])) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.b[327] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[327] = if s.b[327] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[326])) && s.b[327]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[327])) {
            s.store_sub_from_scalar(221, s.v[32], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[328] = (p.p22 == 0.5);
        s.v[328] = if s.b[328] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[327])) && s.b[328]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[327])) && (!s.b[328])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p22)));
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[327])) {
            s.store_add(224, 222, 223);
        }

        s.b[329] = (p.p22 == 0.5);
        s.v[329] = if s.b[329] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[327])) && s.b[329]) {
            s.store_sqrt_scaled_input(218, 221, s.v[68]);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[327])) && (!s.b[329])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[327])) {
            s.store_scale(225, 218, s.v[62]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[23]);
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.b[330] = (p.p36 == 0.0);
        s.v[330] = if s.b[330] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[326])) && s.b[330]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[330])) {
            s.store_scaled_div(228, 225, 221, ((s.v[47]) * (s.v[77])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[331] = (((-p.p22) * s.v[50]) == (-1.0));
        s.v[331] = if s.b[331] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && s.b[331]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && (!s.b[331])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[330])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[332] = (s.v[239] > 0.0);
        s.v[332] = if s.b[332] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && s.b[332]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && (!s.b[332])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[333] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[333] = if s.b[333] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && s.b[333]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && (!s.b[333])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[330])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[334] = (s.v[239] > 0.0);
        s.v[334] = if s.b[334] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && s.b[334]) {
            s.copy_ad(240, 202);
        }

        s.b[335] = (s.v[238] > (-230.25850929940458));
        s.v[335] = if s.b[335] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[326])) && (!s.b[330])) && (!s.b[334])) && s.b[335]) {
            s.store_exp(218, 238);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[199] && (!s.b[326])) && (!s.b[330])) && (!s.b[334])) && (!s.b[335])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[330])) && (!s.b[334])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[330])) {
            s.store_scaled_div(241, 240, 236, ((s.v[74]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p36);
        }

        s.b[336] = (p.p42 == 0.0);
        s.v[336] = if s.b[336] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[326])) && s.b[336]) {
            s.store_scalar(242, 0.0);
        }

        s.b[337] = (p.p22 == 0.5);
        s.v[337] = if s.b[337] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[336])) && s.b[337]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[336])) && (!s.b[337])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[336])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), 218, s.v[50]);
        }

        s.b[338] = (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[338] = if s.b[338] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[326])) && (!s.b[336])) && s.b[338]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.b[339] = (((-s.v[80]) / s.v[243]) < 0.0);
        s.v[339] = if s.b[339] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[326])) && (!s.b[336])) && (!s.b[338])) && s.b[339]) {
            let assign6330_ad_e7247: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign6330_ad_e7247);
        }

        if ((((s.b[199] && (!s.b[326])) && (!s.b[336])) && (!s.b[338])) && (!s.b[339])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[326])) && (!s.b[336])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(124), s.ad_value(243)), s.ad_value(243)), 218, p.p42);
        }

        s.b[340] = (p.p51 > 1000.0);
        s.v[340] = if s.b[340] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[326])) && s.b[340]) {
            s.store_scalar(244, 1.0);
        }

        s.b[341] = (s.v[217] > ((-s.v[82]) * p.p51));
        s.v[341] = if s.b[341] { 1.0 } else { 0.0 };

        s.b[342] = (p.p54 == 4.0);
        s.v[342] = if s.b[342] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[326])) && (!s.b[340])) && s.b[341]) && s.b[342]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), 217, s.v[87]);
        }

        if ((((s.b[199] && (!s.b[326])) && (!s.b[340])) && s.b[341]) && (!s.b[342])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[340])) && s.b[341]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[326])) && (!s.b[340])) && (!s.b[341])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90], s.v[84]);
        }

        if (s.b[199] && (!s.b[326])) {
            s.store_mul_scale_ad_lhs(246, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[343] = (s.v[145] == 0.0);
        s.v[343] = if s.b[343] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[343]) {
            s.store_scalar(247, 0.0);
        }

        if (s.b[199] && (!s.b[343])) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.b[344] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[344] = if s.b[344] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[343])) && s.b[344]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[344])) {
            s.store_sub_from_scalar(221, s.v[33], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[345] = (p.p23 == 0.5);
        s.v[345] = if s.b[345] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[344])) && s.b[345]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[344])) && (!s.b[345])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p23)));
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[344])) {
            s.store_add(224, 222, 223);
        }

        s.b[346] = (p.p23 == 0.5);
        s.v[346] = if s.b[346] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[344])) && s.b[346]) {
            s.store_sqrt_scaled_input(218, 221, s.v[69]);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[344])) && (!s.b[346])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[344])) {
            s.store_scale(225, 218, s.v[63]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[24]);
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.b[347] = (p.p37 == 0.0);
        s.v[347] = if s.b[347] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[343])) && s.b[347]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[347])) {
            s.store_scaled_div(228, 225, 221, ((s.v[48]) * (s.v[78])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[348] = (((-p.p23) * s.v[51]) == (-1.0));
        s.v[348] = if s.b[348] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && s.b[348]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && (!s.b[348])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[347])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[349] = (s.v[239] > 0.0);
        s.v[349] = if s.b[349] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && s.b[349]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && (!s.b[349])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[350] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[350] = if s.b[350] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && s.b[350]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && (!s.b[350])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[347])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[351] = (s.v[239] > 0.0);
        s.v[351] = if s.b[351] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && s.b[351]) {
            s.copy_ad(240, 202);
        }

        s.b[352] = (s.v[238] > (-230.25850929940458));
        s.v[352] = if s.b[352] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[343])) && (!s.b[347])) && (!s.b[351])) && s.b[352]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[343])) && (!s.b[347])) && (!s.b[351])) && (!s.b[352])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[347])) && (!s.b[351])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[347])) {
            s.store_scaled_div(241, 240, 236, ((s.v[75]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p37);
        }

        s.b[353] = (p.p43 == 0.0);
        s.v[353] = if s.b[353] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[343])) && s.b[353]) {
            s.store_scalar(242, 0.0);
        }

        s.b[354] = (p.p23 == 0.5);
        s.v[354] = if s.b[354] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[353])) && s.b[354]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[353])) && (!s.b[354])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[353])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), 218, s.v[51]);
        }

        s.b[355] = (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[355] = if s.b[355] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[343])) && (!s.b[353])) && s.b[355]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.b[356] = (((-s.v[81]) / s.v[243]) < 0.0);
        s.v[356] = if s.b[356] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[343])) && (!s.b[353])) && (!s.b[355])) && s.b[356]) {
            let assign7030_ad_e8284: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign7030_ad_e8284);
        }

        if ((((s.b[199] && (!s.b[343])) && (!s.b[353])) && (!s.b[355])) && (!s.b[356])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[343])) && (!s.b[353])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(124), s.ad_value(243)), s.ad_value(243)), 218, p.p43);
        }

        s.b[357] = (p.p52 > 1000.0);
        s.v[357] = if s.b[357] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[343])) && s.b[357]) {
            s.store_scalar(244, 1.0);
        }

        s.b[358] = (s.v[217] > ((-s.v[82]) * p.p52));
        s.v[358] = if s.b[358] { 1.0 } else { 0.0 };

        s.b[359] = (p.p55 == 4.0);
        s.v[359] = if s.b[359] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[343])) && (!s.b[357])) && s.b[358]) && s.b[359]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), 217, s.v[88]);
        }

        if ((((s.b[199] && (!s.b[343])) && (!s.b[357])) && s.b[358]) && (!s.b[359])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[357])) && s.b[358]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[343])) && (!s.b[357])) && (!s.b[358])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91], s.v[85]);
        }

        if (s.b[199] && (!s.b[343])) {
            s.store_mul_scale_ad_lhs(247, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        if s.b[199] {
            s.store_add_scaled_ad_lhs(114, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), 247, s.v[145]);
            s.store_scalar(216, 0.0);
            s.store_scalar(213, 0.0);
        }

        s.b[360] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[360] = if s.b[360] { 1.0 } else { 0.0 };

        s.b[361] = (s.v[125] < s.v[149]);
        s.v[361] = if s.b[361] { 1.0 } else { 0.0 };

        s.b[362] = (((((-0.5) * (s.v[125] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[362] = if s.b[362] { 1.0 } else { 0.0 };

        if (((s.b[199] && s.b[360]) && s.b[361]) && s.b[362]) {
            s.store_exp_scaled_input(211, 125, (s.v[9] * (-0.5)));
        }

        s.b[363] = (((-0.5) * (s.v[125] * s.v[9])) < 0.0);
        s.v[363] = if s.b[363] { 1.0 } else { 0.0 };

        if ((((s.b[199] && s.b[360]) && s.b[361]) && (!s.b[362])) && s.b[363]) {
            let assign7300_ad_e8665: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(125), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(125), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(125), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(211, assign7300_ad_e8665);
        }

        if ((((s.b[199] && s.b[360]) && s.b[361]) && (!s.b[362])) && (!s.b[363])) {
            s.store_scaled_offset_ad(211, A::mul(A::offset(A::scale(s.ad_value(125), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(125), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(125), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && s.b[360]) && s.b[361]) {
            s.store_div_from_scalar(212, 1.0, 211);
            s.store_square(209, 212);
        }

        if ((s.b[199] && s.b[360]) && (!s.b[361])) {
            s.store_mul_offset_ad_lhs(209, A::scale(A::sub(s.ad_value(125), s.ad_value(149)), s.v[9]), 1.0, 150);
            s.store_sqrt(212, 209);
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if (s.b[199] && s.b[360]) {
            s.store_offset(209, 209, (-1.0));
        }

        s.b[364] = (s.v[125] > 0.0);
        s.v[364] = if s.b[364] { 1.0 } else { 0.0 };

        if ((s.b[199] && s.b[360]) && s.b[364]) {
            s.store_scaled_ln_ad(213, A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0)))), (s.v[8] * 2.0));
        }

        if ((s.b[199] && s.b[360]) && (!s.b[364])) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 125);
        }

        if (s.b[199] && s.b[360]) {
            s.store_sub(214, 151, 213);
            s.store_scaled_sub_ad(215, A::add(s.ad_value(125), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(125), s.ad_value(214)), A::sub(s.ad_value(125), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8]))), 0.5);
            s.store_scaled_sub_ad(216, A::add(s.ad_value(125), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(125), s.ad_value(154)), A::sub(s.ad_value(125), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6]))), 0.5);
            s.store_scaled_sub_ad_rhs(217, 125, A::sqrt(A::offset(A::mul(s.ad_value(125), s.ad_value(125)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[365] = (s.v[143] == 0.0);
        s.v[365] = if s.b[365] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[365]) {
            s.store_scalar(245, 0.0);
        }

        if (s.b[199] && (!s.b[365])) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.b[366] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[366] = if s.b[366] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[365])) && s.b[366]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[366])) {
            s.store_sub_from_scalar(221, s.v[31], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[367] = (p.p21 == 0.5);
        s.v[367] = if s.b[367] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[366])) && s.b[367]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[366])) && (!s.b[367])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p21)));
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[366])) {
            s.store_add(224, 222, 223);
        }

        s.b[368] = (p.p21 == 0.5);
        s.v[368] = if s.b[368] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[366])) && s.b[368]) {
            s.store_sqrt_scaled_input(218, 221, s.v[67]);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[366])) && (!s.b[368])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[366])) {
            s.store_scale(225, 218, s.v[61]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[22]);
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[369] = (p.p35 == 0.0);
        s.v[369] = if s.b[369] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[365])) && s.b[369]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[369])) {
            s.store_scaled_div(228, 225, 221, ((s.v[46]) * (s.v[76])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[370] = (((-p.p21) * s.v[49]) == (-1.0));
        s.v[370] = if s.b[370] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && s.b[370]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && (!s.b[370])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[369])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[371] = (s.v[239] > 0.0);
        s.v[371] = if s.b[371] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && s.b[371]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && (!s.b[371])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[372] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[372] = if s.b[372] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && s.b[372]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && (!s.b[372])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[369])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[373] = (s.v[239] > 0.0);
        s.v[373] = if s.b[373] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && s.b[373]) {
            s.copy_ad(240, 202);
        }

        s.b[374] = (s.v[238] > (-230.25850929940458));
        s.v[374] = if s.b[374] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[365])) && (!s.b[369])) && (!s.b[373])) && s.b[374]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[365])) && (!s.b[369])) && (!s.b[373])) && (!s.b[374])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[369])) && (!s.b[373])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[369])) {
            s.store_scaled_div(241, 240, 236, ((s.v[73]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p35);
        }

        s.b[375] = (p.p41 == 0.0);
        s.v[375] = if s.b[375] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[365])) && s.b[375]) {
            s.store_scalar(242, 0.0);
        }

        s.b[376] = (p.p21 == 0.5);
        s.v[376] = if s.b[376] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[375])) && s.b[376]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[375])) && (!s.b[376])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[375])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), 218, s.v[49]);
        }

        s.b[377] = (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[377] = if s.b[377] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[365])) && (!s.b[375])) && s.b[377]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.b[378] = (((-s.v[79]) / s.v[243]) < 0.0);
        s.v[378] = if s.b[378] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[365])) && (!s.b[375])) && (!s.b[377])) && s.b[378]) {
            let assign8030_ad_e9772: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign8030_ad_e9772);
        }

        if ((((s.b[199] && (!s.b[365])) && (!s.b[375])) && (!s.b[377])) && (!s.b[378])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[365])) && (!s.b[375])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(125), s.ad_value(243)), s.ad_value(243)), 218, p.p41);
        }

        s.b[379] = (p.p50 > 1000.0);
        s.v[379] = if s.b[379] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[365])) && s.b[379]) {
            s.store_scalar(244, 1.0);
        }

        s.b[380] = (s.v[217] > ((-s.v[82]) * p.p50));
        s.v[380] = if s.b[380] { 1.0 } else { 0.0 };

        s.b[381] = (p.p53 == 4.0);
        s.v[381] = if s.b[381] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[365])) && (!s.b[379])) && s.b[380]) && s.b[381]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), 217, s.v[86]);
        }

        if ((((s.b[199] && (!s.b[365])) && (!s.b[379])) && s.b[380]) && (!s.b[381])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[379])) && s.b[380]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[365])) && (!s.b[379])) && (!s.b[380])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89], s.v[83]);
        }

        if (s.b[199] && (!s.b[365])) {
            s.store_mul_scale_ad_lhs(245, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[382] = (s.v[144] == 0.0);
        s.v[382] = if s.b[382] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[382]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[199] && (!s.b[382])) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.b[383] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[383] = if s.b[383] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[382])) && s.b[383]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[383])) {
            s.store_sub_from_scalar(221, s.v[32], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[384] = (p.p22 == 0.5);
        s.v[384] = if s.b[384] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[383])) && s.b[384]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[383])) && (!s.b[384])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p22)));
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[383])) {
            s.store_add(224, 222, 223);
        }

        s.b[385] = (p.p22 == 0.5);
        s.v[385] = if s.b[385] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[383])) && s.b[385]) {
            s.store_sqrt_scaled_input(218, 221, s.v[68]);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[383])) && (!s.b[385])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[383])) {
            s.store_scale(225, 218, s.v[62]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[23]);
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.b[386] = (p.p36 == 0.0);
        s.v[386] = if s.b[386] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[382])) && s.b[386]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[386])) {
            s.store_scaled_div(228, 225, 221, ((s.v[47]) * (s.v[77])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[387] = (((-p.p22) * s.v[50]) == (-1.0));
        s.v[387] = if s.b[387] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && s.b[387]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && (!s.b[387])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[386])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[388] = (s.v[239] > 0.0);
        s.v[388] = if s.b[388] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && s.b[388]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && (!s.b[388])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[389] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[389] = if s.b[389] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && s.b[389]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && (!s.b[389])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[386])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[390] = (s.v[239] > 0.0);
        s.v[390] = if s.b[390] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && s.b[390]) {
            s.copy_ad(240, 202);
        }

        s.b[391] = (s.v[238] > (-230.25850929940458));
        s.v[391] = if s.b[391] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[382])) && (!s.b[386])) && (!s.b[390])) && s.b[391]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[382])) && (!s.b[386])) && (!s.b[390])) && (!s.b[391])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[386])) && (!s.b[390])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[386])) {
            s.store_scaled_div(241, 240, 236, ((s.v[74]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p36);
        }

        s.b[392] = (p.p42 == 0.0);
        s.v[392] = if s.b[392] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[382])) && s.b[392]) {
            s.store_scalar(242, 0.0);
        }

        s.b[393] = (p.p22 == 0.5);
        s.v[393] = if s.b[393] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[392])) && s.b[393]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[392])) && (!s.b[393])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[392])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), 218, s.v[50]);
        }

        s.b[394] = (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[382])) && (!s.b[392])) && s.b[394]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.b[395] = (((-s.v[80]) / s.v[243]) < 0.0);
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[382])) && (!s.b[392])) && (!s.b[394])) && s.b[395]) {
            let assign8730_ad_e10809: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign8730_ad_e10809);
        }

        if ((((s.b[199] && (!s.b[382])) && (!s.b[392])) && (!s.b[394])) && (!s.b[395])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[382])) && (!s.b[392])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(125), s.ad_value(243)), s.ad_value(243)), 218, p.p42);
        }

        s.b[396] = (p.p51 > 1000.0);
        s.v[396] = if s.b[396] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[382])) && s.b[396]) {
            s.store_scalar(244, 1.0);
        }

        s.b[397] = (s.v[217] > ((-s.v[82]) * p.p51));
        s.v[397] = if s.b[397] { 1.0 } else { 0.0 };

        s.b[398] = (p.p54 == 4.0);
        s.v[398] = if s.b[398] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[382])) && (!s.b[396])) && s.b[397]) && s.b[398]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), 217, s.v[87]);
        }

        if ((((s.b[199] && (!s.b[382])) && (!s.b[396])) && s.b[397]) && (!s.b[398])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[396])) && s.b[397]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[382])) && (!s.b[396])) && (!s.b[397])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90], s.v[84]);
        }

        if (s.b[199] && (!s.b[382])) {
            s.store_mul_scale_ad_lhs(246, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[399] = (s.v[145] == 0.0);
        s.v[399] = if s.b[399] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[399]) {
            s.store_scalar(247, 0.0);
        }

        if (s.b[199] && (!s.b[399])) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.b[400] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[400] = if s.b[400] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[399])) && s.b[400]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[400])) {
            s.store_sub_from_scalar(221, s.v[33], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[401] = (p.p23 == 0.5);
        s.v[401] = if s.b[401] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[400])) && s.b[401]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[400])) && (!s.b[401])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p23)));
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[400])) {
            s.store_add(224, 222, 223);
        }

        s.b[402] = (p.p23 == 0.5);
        s.v[402] = if s.b[402] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[400])) && s.b[402]) {
            s.store_sqrt_scaled_input(218, 221, s.v[69]);
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[199] && (!s.b[399])) && (!s.b[400])) && (!s.b[402])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[400])) {
            s.store_scale(225, 218, s.v[63]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[24]);
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.b[403] = (p.p37 == 0.0);
        s.v[403] = if s.b[403] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[399])) && s.b[403]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[403])) {
            s.store_scaled_div(228, 225, 221, ((s.v[48]) * (s.v[78])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[404] = (((-p.p23) * s.v[51]) == (-1.0));
        s.v[404] = if s.b[404] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && s.b[404]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && (!s.b[404])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[403])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[405] = (s.v[239] > 0.0);
        s.v[405] = if s.b[405] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && s.b[405]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && (!s.b[405])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[406] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[406] = if s.b[406] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && s.b[406]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && (!s.b[406])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[403])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[407] = (s.v[239] > 0.0);
        s.v[407] = if s.b[407] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && s.b[407]) {
            s.copy_ad(240, 202);
        }

        s.b[408] = (s.v[238] > (-230.25850929940458));
        s.v[408] = if s.b[408] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[399])) && (!s.b[403])) && (!s.b[407])) && s.b[408]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[399])) && (!s.b[403])) && (!s.b[407])) && (!s.b[408])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[403])) && (!s.b[407])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[403])) {
            s.store_scaled_div(241, 240, 236, ((s.v[75]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p37);
        }

        s.b[409] = (p.p43 == 0.0);
        s.v[409] = if s.b[409] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[399])) && s.b[409]) {
            s.store_scalar(242, 0.0);
        }

        s.b[410] = (p.p23 == 0.5);
        s.v[410] = if s.b[410] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[409])) && s.b[410]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]);
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[409])) && (!s.b[410])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[409])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), 218, s.v[51]);
        }

        s.b[411] = (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[411] = if s.b[411] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[399])) && (!s.b[409])) && s.b[411]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.b[412] = (((-s.v[81]) / s.v[243]) < 0.0);
        s.v[412] = if s.b[412] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[399])) && (!s.b[409])) && (!s.b[411])) && s.b[412]) {
            let assign9430_ad_e11846: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign9430_ad_e11846);
        }

        if ((((s.b[199] && (!s.b[399])) && (!s.b[409])) && (!s.b[411])) && (!s.b[412])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[399])) && (!s.b[409])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(125), s.ad_value(243)), s.ad_value(243)), 218, p.p43);
        }

        s.b[413] = (p.p52 > 1000.0);
        s.v[413] = if s.b[413] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[399])) && s.b[413]) {
            s.store_scalar(244, 1.0);
        }

        s.b[414] = (s.v[217] > ((-s.v[82]) * p.p52));
        s.v[414] = if s.b[414] { 1.0 } else { 0.0 };

        s.b[415] = (p.p55 == 4.0);
        s.v[415] = if s.b[415] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[399])) && (!s.b[413])) && s.b[414]) && s.b[415]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), 217, s.v[88]);
        }

        if ((((s.b[199] && (!s.b[399])) && (!s.b[413])) && s.b[414]) && (!s.b[415])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[413])) && s.b[414]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[399])) && (!s.b[413])) && (!s.b[414])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91], s.v[85]);
        }

        if (s.b[199] && (!s.b[399])) {
            s.store_mul_scale_ad_lhs(247, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        if s.b[199] {
            s.store_add_scaled_ad_lhs(115, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), 247, s.v[145]);
            s.store_scalar(216, 0.0);
            s.store_scalar(213, 0.0);
        }

        s.b[416] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[416] = if s.b[416] { 1.0 } else { 0.0 };

        s.b[417] = (s.v[126] < s.v[149]);
        s.v[417] = if s.b[417] { 1.0 } else { 0.0 };

        s.b[418] = (((((-0.5) * (s.v[126] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

        if (((s.b[199] && s.b[416]) && s.b[417]) && s.b[418]) {
            s.store_exp_scaled_input(211, 126, (s.v[9] * (-0.5)));
        }

        s.b[419] = (((-0.5) * (s.v[126] * s.v[9])) < 0.0);
        s.v[419] = if s.b[419] { 1.0 } else { 0.0 };

        if ((((s.b[199] && s.b[416]) && s.b[417]) && (!s.b[418])) && s.b[419]) {
            let assign9700_ad_e12227: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(126), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(126), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(126), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(211, assign9700_ad_e12227);
        }

        if ((((s.b[199] && s.b[416]) && s.b[417]) && (!s.b[418])) && (!s.b[419])) {
            s.store_scaled_offset_ad(211, A::mul(A::offset(A::scale(s.ad_value(126), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(126), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(126), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && s.b[416]) && s.b[417]) {
            s.store_div_from_scalar(212, 1.0, 211);
            s.store_square(209, 212);
        }

        if ((s.b[199] && s.b[416]) && (!s.b[417])) {
            s.store_mul_offset_ad_lhs(209, A::scale(A::sub(s.ad_value(126), s.ad_value(149)), s.v[9]), 1.0, 150);
            s.store_sqrt(212, 209);
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if (s.b[199] && s.b[416]) {
            s.store_offset(209, 209, (-1.0));
        }

        s.b[420] = (s.v[126] > 0.0);
        s.v[420] = if s.b[420] { 1.0 } else { 0.0 };

        if ((s.b[199] && s.b[416]) && s.b[420]) {
            s.store_scaled_ln_ad(213, A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0)))), (s.v[8] * 2.0));
        }

        if ((s.b[199] && s.b[416]) && (!s.b[420])) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 126);
        }

        if (s.b[199] && s.b[416]) {
            s.store_sub(214, 151, 213);
            s.store_scaled_sub_ad(215, A::add(s.ad_value(126), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(126), s.ad_value(214)), A::sub(s.ad_value(126), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8]))), 0.5);
            s.store_scaled_sub_ad(216, A::add(s.ad_value(126), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(126), s.ad_value(154)), A::sub(s.ad_value(126), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6]))), 0.5);
            s.store_scaled_sub_ad_rhs(217, 126, A::sqrt(A::offset(A::mul(s.ad_value(126), s.ad_value(126)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[421] = (s.v[143] == 0.0);
        s.v[421] = if s.b[421] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[421]) {
            s.store_scalar(245, 0.0);
        }

        if (s.b[199] && (!s.b[421])) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.b[422] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[422] = if s.b[422] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[421])) && s.b[422]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[422])) {
            s.store_sub_from_scalar(221, s.v[31], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[423] = (p.p21 == 0.5);
        s.v[423] = if s.b[423] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[422])) && s.b[423]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[422])) && (!s.b[423])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p21)));
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[422])) {
            s.store_add(224, 222, 223);
        }

        s.b[424] = (p.p21 == 0.5);
        s.v[424] = if s.b[424] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[422])) && s.b[424]) {
            s.store_sqrt_scaled_input(218, 221, s.v[67]);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[422])) && (!s.b[424])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[422])) {
            s.store_scale(225, 218, s.v[61]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[22]);
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.b[425] = (p.p35 == 0.0);
        s.v[425] = if s.b[425] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[421])) && s.b[425]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[425])) {
            s.store_scaled_div(228, 225, 221, ((s.v[46]) * (s.v[76])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[426] = (((-p.p21) * s.v[49]) == (-1.0));
        s.v[426] = if s.b[426] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && s.b[426]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && (!s.b[426])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[425])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[427] = (s.v[239] > 0.0);
        s.v[427] = if s.b[427] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && s.b[427]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && (!s.b[427])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[428] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[428] = if s.b[428] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && s.b[428]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && (!s.b[428])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[425])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[429] = (s.v[239] > 0.0);
        s.v[429] = if s.b[429] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && s.b[429]) {
            s.copy_ad(240, 202);
        }

        s.b[430] = (s.v[238] > (-230.25850929940458));
        s.v[430] = if s.b[430] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[421])) && (!s.b[425])) && (!s.b[429])) && s.b[430]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[421])) && (!s.b[425])) && (!s.b[429])) && (!s.b[430])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[425])) && (!s.b[429])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[425])) {
            s.store_scaled_div(241, 240, 236, ((s.v[73]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p35);
        }

        s.b[431] = (p.p41 == 0.0);
        s.v[431] = if s.b[431] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[421])) && s.b[431]) {
            s.store_scalar(242, 0.0);
        }

        s.b[432] = (p.p21 == 0.5);
        s.v[432] = if s.b[432] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[431])) && s.b[432]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[431])) && (!s.b[432])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[431])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), 218, s.v[49]);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[433] = (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[433] = if s.b[433] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[421])) && (!s.b[431])) && s.b[433]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.b[434] = (((-s.v[79]) / s.v[243]) < 0.0);
        s.v[434] = if s.b[434] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[421])) && (!s.b[431])) && (!s.b[433])) && s.b[434]) {
            let assign10430_ad_e13334: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign10430_ad_e13334);
        }

        if ((((s.b[199] && (!s.b[421])) && (!s.b[431])) && (!s.b[433])) && (!s.b[434])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[421])) && (!s.b[431])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(126), s.ad_value(243)), s.ad_value(243)), 218, p.p41);
        }

        s.b[435] = (p.p50 > 1000.0);
        s.v[435] = if s.b[435] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[421])) && s.b[435]) {
            s.store_scalar(244, 1.0);
        }

        s.b[436] = (s.v[217] > ((-s.v[82]) * p.p50));
        s.v[436] = if s.b[436] { 1.0 } else { 0.0 };

        s.b[437] = (p.p53 == 4.0);
        s.v[437] = if s.b[437] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[421])) && (!s.b[435])) && s.b[436]) && s.b[437]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), 217, s.v[86]);
        }

        if ((((s.b[199] && (!s.b[421])) && (!s.b[435])) && s.b[436]) && (!s.b[437])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[435])) && s.b[436]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[421])) && (!s.b[435])) && (!s.b[436])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89], s.v[83]);
        }

        if (s.b[199] && (!s.b[421])) {
            s.store_mul_scale_ad_lhs(245, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[438] = (s.v[144] == 0.0);
        s.v[438] = if s.b[438] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[438]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[199] && (!s.b[438])) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.b[439] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[438])) && s.b[439]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[439])) {
            s.store_sub_from_scalar(221, s.v[32], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[440] = (p.p22 == 0.5);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[439])) && s.b[440]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[439])) && (!s.b[440])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p22)));
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[439])) {
            s.store_add(224, 222, 223);
        }

        s.b[441] = (p.p22 == 0.5);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[439])) && s.b[441]) {
            s.store_sqrt_scaled_input(218, 221, s.v[68]);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[439])) && (!s.b[441])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[439])) {
            s.store_scale(225, 218, s.v[62]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[23]);
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.b[442] = (p.p36 == 0.0);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[438])) && s.b[442]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[442])) {
            s.store_scaled_div(228, 225, 221, ((s.v[47]) * (s.v[77])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[443] = (((-p.p22) * s.v[50]) == (-1.0));
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && s.b[443]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && (!s.b[443])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[442])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[444] = (s.v[239] > 0.0);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && s.b[444]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && (!s.b[444])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[445] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && s.b[445]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && (!s.b[445])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[442])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[446] = (s.v[239] > 0.0);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && s.b[446]) {
            s.copy_ad(240, 202);
        }

        s.b[447] = (s.v[238] > (-230.25850929940458));
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[438])) && (!s.b[442])) && (!s.b[446])) && s.b[447]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[438])) && (!s.b[442])) && (!s.b[446])) && (!s.b[447])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[442])) && (!s.b[446])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[442])) {
            s.store_scaled_div(241, 240, 236, ((s.v[74]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p36);
        }

        s.b[448] = (p.p42 == 0.0);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[438])) && s.b[448]) {
            s.store_scalar(242, 0.0);
        }

        s.b[449] = (p.p22 == 0.5);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[448])) && s.b[449]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[448])) && (!s.b[449])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[448])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), 218, s.v[50]);
        }

        s.b[450] = (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[438])) && (!s.b[448])) && s.b[450]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.b[451] = (((-s.v[80]) / s.v[243]) < 0.0);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[438])) && (!s.b[448])) && (!s.b[450])) && s.b[451]) {
            let assign11130_ad_e14371: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign11130_ad_e14371);
        }

        if ((((s.b[199] && (!s.b[438])) && (!s.b[448])) && (!s.b[450])) && (!s.b[451])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[438])) && (!s.b[448])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(126), s.ad_value(243)), s.ad_value(243)), 218, p.p42);
        }

        s.b[452] = (p.p51 > 1000.0);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[438])) && s.b[452]) {
            s.store_scalar(244, 1.0);
        }

        s.b[453] = (s.v[217] > ((-s.v[82]) * p.p51));
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        s.b[454] = (p.p54 == 4.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[438])) && (!s.b[452])) && s.b[453]) && s.b[454]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), 217, s.v[87]);
        }

        if ((((s.b[199] && (!s.b[438])) && (!s.b[452])) && s.b[453]) && (!s.b[454])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[452])) && s.b[453]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[438])) && (!s.b[452])) && (!s.b[453])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90], s.v[84]);
        }

        if (s.b[199] && (!s.b[438])) {
            s.store_mul_scale_ad_lhs(246, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[455] = (s.v[145] == 0.0);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[455]) {
            s.store_scalar(247, 0.0);
        }

        if (s.b[199] && (!s.b[455])) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.b[456] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[455])) && s.b[456]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[456])) {
            s.store_sub_from_scalar(221, s.v[33], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[457] = (p.p23 == 0.5);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[456])) && s.b[457]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[456])) && (!s.b[457])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p23)));
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[456])) {
            s.store_add(224, 222, 223);
        }

        s.b[458] = (p.p23 == 0.5);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[456])) && s.b[458]) {
            s.store_sqrt_scaled_input(218, 221, s.v[69]);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[456])) && (!s.b[458])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[456])) {
            s.store_scale(225, 218, s.v[63]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[24]);
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.b[459] = (p.p37 == 0.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[455])) && s.b[459]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[459])) {
            s.store_scaled_div(228, 225, 221, ((s.v[48]) * (s.v[78])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[460] = (((-p.p23) * s.v[51]) == (-1.0));
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && s.b[460]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && (!s.b[460])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[459])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[461] = (s.v[239] > 0.0);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && s.b[461]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && (!s.b[461])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[462] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && s.b[462]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && (!s.b[462])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[459])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[463] = (s.v[239] > 0.0);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && s.b[463]) {
            s.copy_ad(240, 202);
        }

        s.b[464] = (s.v[238] > (-230.25850929940458));
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[455])) && (!s.b[459])) && (!s.b[463])) && s.b[464]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[455])) && (!s.b[459])) && (!s.b[463])) && (!s.b[464])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[459])) && (!s.b[463])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[459])) {
            s.store_scaled_div(241, 240, 236, ((s.v[75]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p37);
        }

        s.b[465] = (p.p43 == 0.0);
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[199] && (!s.b[455])) && s.b[465]) {
            s.store_scalar(242, 0.0);
        }

        s.b[466] = (p.p23 == 0.5);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[465])) && s.b[466]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[465])) && (!s.b[466])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[465])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), 218, s.v[51]);
        }

        s.b[467] = (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[455])) && (!s.b[465])) && s.b[467]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.b[468] = (((-s.v[81]) / s.v[243]) < 0.0);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[455])) && (!s.b[465])) && (!s.b[467])) && s.b[468]) {
            let assign11830_ad_e15408: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign11830_ad_e15408);
        }

        if ((((s.b[199] && (!s.b[455])) && (!s.b[465])) && (!s.b[467])) && (!s.b[468])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[455])) && (!s.b[465])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(126), s.ad_value(243)), s.ad_value(243)), 218, p.p43);
        }

        s.b[469] = (p.p52 > 1000.0);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[455])) && s.b[469]) {
            s.store_scalar(244, 1.0);
        }

        s.b[470] = (s.v[217] > ((-s.v[82]) * p.p52));
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        s.b[471] = (p.p55 == 4.0);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[455])) && (!s.b[469])) && s.b[470]) && s.b[471]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), 217, s.v[88]);
        }

        if ((((s.b[199] && (!s.b[455])) && (!s.b[469])) && s.b[470]) && (!s.b[471])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[469])) && s.b[470]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[455])) && (!s.b[469])) && (!s.b[470])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91], s.v[85]);
        }

        if (s.b[199] && (!s.b[455])) {
            s.store_mul_scale_ad_lhs(247, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        if s.b[199] {
            s.store_add_scaled_ad_lhs(116, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), 247, s.v[145]);
            s.store_scalar(216, 0.0);
            s.store_scalar(213, 0.0);
        }

        s.b[472] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        s.b[473] = (s.v[127] < s.v[149]);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        s.b[474] = (((((-0.5) * (s.v[127] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if (((s.b[199] && s.b[472]) && s.b[473]) && s.b[474]) {
            s.store_exp_scaled_input(211, 127, (s.v[9] * (-0.5)));
        }

        s.b[475] = (((-0.5) * (s.v[127] * s.v[9])) < 0.0);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if ((((s.b[199] && s.b[472]) && s.b[473]) && (!s.b[474])) && s.b[475]) {
            let assign12100_ad_e15789: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(127), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(127), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(127), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(211, assign12100_ad_e15789);
        }

        if ((((s.b[199] && s.b[472]) && s.b[473]) && (!s.b[474])) && (!s.b[475])) {
            s.store_scaled_offset_ad(211, A::mul(A::offset(A::scale(s.ad_value(127), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(127), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(127), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && s.b[472]) && s.b[473]) {
            s.store_div_from_scalar(212, 1.0, 211);
            s.store_square(209, 212);
        }

        if ((s.b[199] && s.b[472]) && (!s.b[473])) {
            s.store_mul_offset_ad_lhs(209, A::scale(A::sub(s.ad_value(127), s.ad_value(149)), s.v[9]), 1.0, 150);
            s.store_sqrt(212, 209);
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if (s.b[199] && s.b[472]) {
            s.store_offset(209, 209, (-1.0));
        }

        s.b[476] = (s.v[127] > 0.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if ((s.b[199] && s.b[472]) && s.b[476]) {
            s.store_scaled_ln_ad(213, A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0)))), (s.v[8] * 2.0));
        }

        if ((s.b[199] && s.b[472]) && (!s.b[476])) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 127);
        }

        if (s.b[199] && s.b[472]) {
            s.store_sub(214, 151, 213);
            s.store_scaled_sub_ad(215, A::add(s.ad_value(127), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(127), s.ad_value(214)), A::sub(s.ad_value(127), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8]))), 0.5);
            s.store_scaled_sub_ad(216, A::add(s.ad_value(127), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(127), s.ad_value(154)), A::sub(s.ad_value(127), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6]))), 0.5);
            s.store_scaled_sub_ad_rhs(217, 127, A::sqrt(A::offset(A::mul(s.ad_value(127), s.ad_value(127)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[477] = (s.v[143] == 0.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[477]) {
            s.store_scalar(245, 0.0);
        }

        if (s.b[199] && (!s.b[477])) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.b[478] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[477])) && s.b[478]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[478])) {
            s.store_sub_from_scalar(221, s.v[31], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[479] = (p.p21 == 0.5);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[478])) && s.b[479]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[478])) && (!s.b[479])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p21)));
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[478])) {
            s.store_add(224, 222, 223);
        }

        s.b[480] = (p.p21 == 0.5);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[478])) && s.b[480]) {
            s.store_sqrt_scaled_input(218, 221, s.v[67]);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[478])) && (!s.b[480])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[478])) {
            s.store_scale(225, 218, s.v[61]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[22]);
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.b[481] = (p.p35 == 0.0);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[477])) && s.b[481]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[481])) {
            s.store_scaled_div(228, 225, 221, ((s.v[46]) * (s.v[76])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[482] = (((-p.p21) * s.v[49]) == (-1.0));
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && s.b[482]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && (!s.b[482])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[481])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[483] = (s.v[239] > 0.0);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && s.b[483]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && (!s.b[483])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[484] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && s.b[484]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && (!s.b[484])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[481])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[485] = (s.v[239] > 0.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && s.b[485]) {
            s.copy_ad(240, 202);
        }

        s.b[486] = (s.v[238] > (-230.25850929940458));
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[477])) && (!s.b[481])) && (!s.b[485])) && s.b[486]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[477])) && (!s.b[481])) && (!s.b[485])) && (!s.b[486])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[481])) && (!s.b[485])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[481])) {
            s.store_scaled_div(241, 240, 236, ((s.v[73]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p35);
        }

        s.b[487] = (p.p41 == 0.0);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[477])) && s.b[487]) {
            s.store_scalar(242, 0.0);
        }

        s.b[488] = (p.p21 == 0.5);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[487])) && s.b[488]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[487])) && (!s.b[488])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[487])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), 218, s.v[49]);
        }

        s.b[489] = (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[477])) && (!s.b[487])) && s.b[489]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.b[490] = (((-s.v[79]) / s.v[243]) < 0.0);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[477])) && (!s.b[487])) && (!s.b[489])) && s.b[490]) {
            let assign12830_ad_e16896: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign12830_ad_e16896);
        }

        if ((((s.b[199] && (!s.b[477])) && (!s.b[487])) && (!s.b[489])) && (!s.b[490])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[477])) && (!s.b[487])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(127), s.ad_value(243)), s.ad_value(243)), 218, p.p41);
        }

        s.b[491] = (p.p50 > 1000.0);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[477])) && s.b[491]) {
            s.store_scalar(244, 1.0);
        }

        s.b[492] = (s.v[217] > ((-s.v[82]) * p.p50));
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        s.b[493] = (p.p53 == 4.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[477])) && (!s.b[491])) && s.b[492]) && s.b[493]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), 217, s.v[86]);
        }

        if ((((s.b[199] && (!s.b[477])) && (!s.b[491])) && s.b[492]) && (!s.b[493])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[491])) && s.b[492]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[477])) && (!s.b[491])) && (!s.b[492])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89], s.v[83]);
        }

        if (s.b[199] && (!s.b[477])) {
            s.store_mul_scale_ad_lhs(245, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[494] = (s.v[144] == 0.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[494]) {
            s.store_scalar(246, 0.0);
        }

        if (s.b[199] && (!s.b[494])) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.b[495] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[494])) && s.b[495]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[495])) {
            s.store_sub_from_scalar(221, s.v[32], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[496] = (p.p22 == 0.5);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[495])) && s.b[496]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[495])) && (!s.b[496])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p22)));
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[495])) {
            s.store_add(224, 222, 223);
        }

        s.b[497] = (p.p22 == 0.5);
        s.v[497] = if s.b[497] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[495])) && s.b[497]) {
            s.store_sqrt_scaled_input(218, 221, s.v[68]);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[495])) && (!s.b[497])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[495])) {
            s.store_scale(225, 218, s.v[62]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[23]);
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.b[498] = (p.p36 == 0.0);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[494])) && s.b[498]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[498])) {
            s.store_scaled_div(228, 225, 221, ((s.v[47]) * (s.v[77])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
            s.store_square(230, 229);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[199] && (!s.b[494])) && (!s.b[498])) {
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[499] = (((-p.p22) * s.v[50]) == (-1.0));
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && s.b[499]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && (!s.b[499])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[498])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[500] = (s.v[239] > 0.0);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && s.b[500]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && (!s.b[500])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[501] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && s.b[501]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && (!s.b[501])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[498])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[502] = (s.v[239] > 0.0);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && s.b[502]) {
            s.copy_ad(240, 202);
        }

        s.b[503] = (s.v[238] > (-230.25850929940458));
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[494])) && (!s.b[498])) && (!s.b[502])) && s.b[503]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[494])) && (!s.b[498])) && (!s.b[502])) && (!s.b[503])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[498])) && (!s.b[502])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[498])) {
            s.store_scaled_div(241, 240, 236, ((s.v[74]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p36);
        }

        s.b[504] = (p.p42 == 0.0);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[494])) && s.b[504]) {
            s.store_scalar(242, 0.0);
        }

        s.b[505] = (p.p22 == 0.5);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[504])) && s.b[505]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[504])) && (!s.b[505])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[504])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), 218, s.v[50]);
        }

        s.b[506] = (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[494])) && (!s.b[504])) && s.b[506]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.b[507] = (((-s.v[80]) / s.v[243]) < 0.0);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[494])) && (!s.b[504])) && (!s.b[506])) && s.b[507]) {
            let assign13530_ad_e17933: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign13530_ad_e17933);
        }

        if ((((s.b[199] && (!s.b[494])) && (!s.b[504])) && (!s.b[506])) && (!s.b[507])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[494])) && (!s.b[504])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(127), s.ad_value(243)), s.ad_value(243)), 218, p.p42);
        }

        s.b[508] = (p.p51 > 1000.0);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[494])) && s.b[508]) {
            s.store_scalar(244, 1.0);
        }

        s.b[509] = (s.v[217] > ((-s.v[82]) * p.p51));
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        s.b[510] = (p.p54 == 4.0);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[494])) && (!s.b[508])) && s.b[509]) && s.b[510]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), 217, s.v[87]);
        }

        if ((((s.b[199] && (!s.b[494])) && (!s.b[508])) && s.b[509]) && (!s.b[510])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[508])) && s.b[509]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[494])) && (!s.b[508])) && (!s.b[509])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90], s.v[84]);
        }

        if (s.b[199] && (!s.b[494])) {
            s.store_mul_scale_ad_lhs(246, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        s.b[511] = (s.v[145] == 0.0);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[511]) {
            s.store_scalar(247, 0.0);
        }

        if (s.b[199] && (!s.b[511])) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.b[512] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[511])) && s.b[512]) {
            s.store_scalar(220, 0.0);
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[512])) {
            s.store_sub_from_scalar(221, s.v[33], 215);
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.b[513] = (p.p23 == 0.5);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[512])) && s.b[513]) {
            s.store_scalar(223, 0.0);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[512])) && (!s.b[513])) {
            s.store_scaled_add_ad_lhs(223, A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), 222, (1.0 - (2.0 * p.p23)));
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[512])) {
            s.store_add(224, 222, 223);
        }

        s.b[514] = (p.p23 == 0.5);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[512])) && s.b[514]) {
            s.store_sqrt_scaled_input(218, 221, s.v[69]);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[512])) && (!s.b[514])) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[512])) {
            s.store_scale(225, 218, s.v[63]);
            s.store_mul_scaled_ad_lhs(226, A::offset(s.ad_value(212), (-1.0)), 225, s.v[24]);
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.b[515] = (p.p37 == 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[511])) && s.b[515]) {
            s.store_scalar(227, 0.0);
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[515])) {
            s.store_scaled_div(228, 225, 221, ((s.v[48]) * (s.v[78])));
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
            s.store_square(230, 229);
            s.store_sqrt_div_ad(231, A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0));
            s.store_sqrt(232, 231);
            s.store_mul(233, 231, 232);
        }

        s.b[516] = (((-p.p23) * s.v[51]) == (-1.0));
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && s.b[516]) {
            s.store_div_from_scalar_offset_ad(234, 1.0, A::mul(s.ad_value(228), s.ad_value(233)), 1.0);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && (!s.b[516])) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[515])) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
            s.store_sqrt_scaled_ad(236, A::div(s.ad_value(228), s.ad_value(232)), 0.375);
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
            s.store_mul_offset_lhs(239, 237, (-1.0), 236);
            s.store_square(200, 239);
        }

        s.b[517] = (s.v[239] > 0.0);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && s.b[517]) {
            s.store_div_from_scalar_offset_scaled_input(201, 1.0, 239, s.v[10], 1.0);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && (!s.b[517])) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::scale(s.ad_value(239), s.v[10]));
        }

        s.b[518] = (((-s.v[200]) + s.v[238]) > (-230.25850929940458));
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && s.b[518]) {
            s.store_exp_sub(218, 238, 200);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && (!s.b[518])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[515])) {
            s.store_mul_add_ad_lhs(202, A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12]), 218);
        }

        s.b[519] = (s.v[239] > 0.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && s.b[519]) {
            s.copy_ad(240, 202);
        }

        s.b[520] = (s.v[238] > (-230.25850929940458));
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[511])) && (!s.b[515])) && (!s.b[519])) && s.b[520]) {
            s.store_exp(218, 238);
        }

        if ((((s.b[199] && (!s.b[511])) && (!s.b[515])) && (!s.b[519])) && (!s.b[520])) {
            s.store_div_from_scalar_offset_ad(218, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[515])) && (!s.b[519])) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[515])) {
            s.store_scaled_div(241, 240, 236, ((s.v[75]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(227, A::mul(s.ad_value(226), s.ad_value(241)), 235, p.p37);
        }

        s.b[521] = (p.p43 == 0.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[511])) && s.b[521]) {
            s.store_scalar(242, 0.0);
        }

        s.b[522] = (p.p23 == 0.5);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[521])) && s.b[522]) {
            s.store_sqrt_scaled_ad(218, A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[521])) && (!s.b[522])) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[521])) {
            s.store_scaled_div_ad_lhs(243, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), 218, s.v[51]);
        }

        s.b[523] = (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if (((s.b[199] && (!s.b[511])) && (!s.b[521])) && s.b[523]) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.b[524] = (((-s.v[81]) / s.v[243]) < 0.0);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[511])) && (!s.b[521])) && (!s.b[523])) && s.b[524]) {
            let assign14230_ad_e18970: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign14230_ad_e18970);
        }

        if ((((s.b[199] && (!s.b[511])) && (!s.b[521])) && (!s.b[523])) && (!s.b[524])) {
            s.store_scaled_offset_ad(218, A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[199] && (!s.b[511])) && (!s.b[521])) {
            s.store_mul_scaled_ad_lhs(242, A::mul(A::mul(s.ad_value(127), s.ad_value(243)), s.ad_value(243)), 218, p.p43);
        }

        s.b[525] = (p.p52 > 1000.0);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((s.b[199] && (!s.b[511])) && s.b[525]) {
            s.store_scalar(244, 1.0);
        }

        s.b[526] = (s.v[217] > ((-s.v[82]) * p.p52));
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        s.b[527] = (p.p55 == 4.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if ((((s.b[199] && (!s.b[511])) && (!s.b[525])) && s.b[526]) && s.b[527]) {
            s.store_mul_scaled_ad_lhs(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), 217, s.v[88]);
        }

        if ((((s.b[199] && (!s.b[511])) && (!s.b[525])) && s.b[526]) && (!s.b[527])) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[525])) && s.b[526]) {
            s.store_div_from_scalar_sub_from_scalar_ad(244, 1.0, 1.0, s.ad_value(218));
        }

        if (((s.b[199] && (!s.b[511])) && (!s.b[525])) && (!s.b[526])) {
            s.store_offset_scaled_ad(244, A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91], s.v[85]);
        }

        if (s.b[199] && (!s.b[511])) {
            s.store_mul_scale_ad_lhs(247, A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10, 244);
        }

        if s.b[199] {
            s.store_add_scaled_ad_lhs(117, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), 247, s.v[145]);
            s.store_scalar(161, (((s.v[143] * s.v[25]) + (s.v[144] * s.v[26])) + (s.v[145] * s.v[27])));
            s.store_sub_ad_rhs(121, 116, A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(126), (s.v[9] * s.v[162]))), (-1.0))));
            s.store_sub_ad_rhs(122, 117, A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(127), (s.v[9] * s.v[162]))), (-1.0))));
        }

        s.b[528] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        s.b[529] = ((s.v[116] > 0.0) && (s.v[117] > 0.0));
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        s.b[530] = ((((((s.v[121] / s.v[116]) > 0.001) || ((s.v[122] / s.v[117]) > 0.001)) && (s.v[121] > 0.0)) && (s.v[122] > 0.0)) && (s.v[122] > s.v[121]));
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if (((s.b[199] && s.b[528]) && s.b[529]) && s.b[530]) {
            s.store_div(128, 121, 122);
            s.store_div_ad(164, A::scale(A::ln(s.ad_value(128)), s.v[8]), A::sub(s.ad_value(126), s.ad_value(127)));
            s.store_div_ad_rhs(163, 121, A::offset(A::exp(A::mul(A::scale(s.ad_value(126), s.v[9]), s.ad_value(164))), (-1.0)));
        }

        if (s.b[199] && s.b[528]) {
            s.store_sub_ad(118, A::sub(s.ad_value(113), A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(123), (s.v[9] * s.v[162]))), (-1.0)))), A::mul(s.ad_value(163), A::offset(A::exp(A::mul(A::scale(s.ad_value(123), s.v[9]), s.ad_value(164))), (-1.0))));
            s.store_sub_ad(119, A::sub(s.ad_value(114), A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(124), (s.v[9] * s.v[162]))), (-1.0)))), A::mul(s.ad_value(163), A::offset(A::exp(A::mul(A::scale(s.ad_value(124), s.v[9]), s.ad_value(164))), (-1.0))));
            s.store_sub_ad(120, A::sub(s.ad_value(115), A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(125), (s.v[9] * s.v[162]))), (-1.0)))), A::mul(s.ad_value(163), A::offset(A::exp(A::mul(A::scale(s.ad_value(125), s.v[9]), s.ad_value(164))), (-1.0))));
        }

        s.b[531] = (((s.v[113] < 0.0) && (s.v[114] < 0.0)) && (s.v[115] < 0.0));
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        s.b[532] = (((((((s.v[118] / s.v[113]) > 0.001) || ((s.v[119] / s.v[114]) > 0.001)) || ((s.v[120] / s.v[115]) > 0.001)) && (s.v[118] < 0.0)) && (s.v[119] < 0.0)) && (s.v[120] < 0.0));
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if (((s.b[199] && s.b[528]) && s.b[531]) && s.b[532]) {
            s.store_div(128, 118, 119);
            s.store_div_ad(129, A::scale(A::ln(s.ad_value(128)), (-s.v[8])), A::sub(s.ad_value(123), s.ad_value(124)));
            s.store_div_ad_rhs(131, 124, A::sub(s.ad_value(124), s.ad_value(123)));
        }

    }

    pub(super) fn stamp_transient_block_10(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[199] && s.b[528]) && s.b[531]) && s.b[532]) {
            s.store_mul_ad(132, A::scale(A::offset(s.ad_value(128), (-1.0)), s.v[8]), A::offset(A::pow(s.ad_value(128), s.ad_value(131)), (-1.0)));
            s.store_div_ad_rhs(131, 123, A::sub(s.ad_value(123), s.ad_value(124)));
            s.store_sub_ad_lhs(133, A::add(A::mul(A::pow(s.ad_value(128), s.ad_value(131)), A::sub(s.ad_value(124), s.ad_value(123))), A::mul(s.ad_value(128), s.ad_value(123))), 124);
            s.store_div(130, 132, 133);
            s.store_add(166, 129, 130);
        }

        s.b[533] = (((((s.v[125] * s.v[9]) * s.v[166])) as f64).abs() < 1e-6);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if ((((s.b[199] && s.b[528]) && s.b[531]) && s.b[532]) && s.b[533]) {
            s.store_scalar(167, 1.0);
            s.store_mul_add_ad_rhs(165, 120, A::div_from_scalar(1.0, s.ad_value(125)), A::scale(s.ad_value(166), (0.5 * s.v[9])));
            s.store_div_ad_lhs(166, A::scale(A::mul(A::scale(s.ad_value(120), (-0.5)), s.ad_value(166)), s.v[9]), 125);
        }

        if ((((s.b[199] && s.b[528]) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_scalar(167, 0.0);
            s.store_div_ad(165, A::neg(s.ad_value(120)), A::offset(A::exp(A::mul(A::scale(A::neg(s.ad_value(125)), s.v[9]), s.ad_value(166))), (-1.0)));
        }

        if s.b[199] {
            s.store_scalar(139, (p.p64 * (((s.v[143] * s.v[52]) + (s.v[144] * s.v[53])) + (s.v[145] * s.v[54]))));
        }

        s.b[534] = ((s.v[143] * s.v[52]) <= s.v[139]);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[534]) {
            s.store_scalar(146, 0.0);
        }

        s.b[535] = ((s.v[144] * s.v[53]) <= s.v[139]);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[535]) {
            s.store_scalar(147, 0.0);
        }

        s.b[536] = ((s.v[145] * s.v[54]) <= s.v[139]);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[536]) {
            s.store_scalar(148, 0.0);
        }

        s.b[537] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[537]) {
            s.store_ln_ad(155, A::div_from_scalar((0.5 * p.p12), A::offset(s.ad_value(161), 1e-21)));
            s.store_ln_ad(157, A::div_from_scalar((0.5 * p.p12), A::offset(s.ad_value(163), 1e-21)));
            s.store_ln_ad(159, A::div_from_scalar((0.5 * p.p12), A::offset(A::abs(s.ad_value(165)), 1e-21)));
        }

        if s.b[199] {
            s.store_min_with_scalar(155, 155, 230.25850929940458);
            s.store_exp(156, 155);
            s.store_min_with_scalar(157, 157, 230.25850929940458);
            s.store_exp(158, 157);
            s.store_min_with_scalar(159, 159, 230.25850929940458);
            s.store_exp(160, 159);
        }

        s.v[544] = 0.0;

        s.v[538] = 0.0;

        s.v[540] = 0.0;

        s.v[542] = 0.0;

        s.v[548] = 0.0;

        s.v[549] = 0.0;

        s.v[550] = 0.0;

        s.v[551] = 0.0;

        s.v[552] = 0.0;

        s.v[553] = 0.0;

        s.v[554] = 0.0;

        s.v[555] = 0.0;

        s.v[556] = 0.0;

        s.v[557] = 0.0;

        s.v[558] = 0.0;

        s.v[559] = 0.0;

        s.v[560] = 0.0;

        s.v[561] = 0.0;

        s.v[562] = 0.0;

        s.v[563] = 0.0;

        s.v[564] = 0.0;

        s.v[565] = 0.0;

        s.v[566] = 0.0;

        s.v[567] = 0.0;

        s.v[568] = 0.0;

        s.v[569] = 0.0;

        s.v[570] = 0.0;

        s.v[571] = 0.0;

        s.v[572] = 0.0;

        s.v[573] = 0.0;

        s.v[574] = 0.0;

        s.v[575] = 0.0;

        s.v[576] = 0.0;

        s.v[577] = 0.0;

        s.v[578] = 0.0;

        s.v[579] = 0.0;

        s.v[580] = 0.0;

        s.v[581] = 0.0;

        s.v[582] = 0.0;

        s.v[583] = 0.0;

        s.v[584] = 0.0;

        s.v[585] = 0.0;

        s.v[586] = 0.0;

        s.v[587] = 0.0;

        s.v[588] = 0.0;

        s.v[589] = 0.0;

        s.v[590] = 0.0;

        s.v[591] = 0.0;

        s.v[592] = 0.0;

        s.v[593] = 0.0;

        s.v[594] = 0.0;

        s.v[539] = 0.0;

        s.v[541] = 0.0;

        s.v[543] = 0.0;

        s.store_scaled_voltage(547, ctx, nodes, Some(0), Some(1), p.p1);

        s.b[595] = (s.v[112] == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_scale(134, 547, (s.v[9] * s.v[162]));
        }

        if s.b[595] {
            let assign15380_ad_e19886: A = {
                if (s.v[134] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(134)), 1.0))
                } else {
                    {
                        if (s.v[134] > s.v[155]) {
                            A::mul(s.ad_value(156), A::offset(A::sub(s.ad_value(134), s.ad_value(155)), 1.0))
                        } else {
                            A::exp(s.ad_value(134))
                        }
                    }
                }
            };
            s.store_ad_value(135, assign15380_ad_e19886);
        }

        if s.b[595] {
            s.store_mul_offset_rhs(140, 161, 135, (-1.0));
            s.store_scaled_mul(134, 547, 164, s.v[9]);
        }

        if s.b[595] {
            let assign15410_ad_e19931: A = {
                if (s.v[134] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(134)), 1.0))
                } else {
                    {
                        if (s.v[134] > s.v[157]) {
                            A::mul(s.ad_value(158), A::offset(A::sub(s.ad_value(134), s.ad_value(157)), 1.0))
                        } else {
                            A::exp(s.ad_value(134))
                        }
                    }
                }
            };
            s.store_ad_value(135, assign15410_ad_e19931);
        }

        if s.b[595] {
            s.store_mul_offset_rhs(141, 163, 135, (-1.0));
            s.store_scalar(142, 0.0);
        }

        s.b[596] = (s.v[167] > 0.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_mul_add_ad_rhs(142, 547, s.ad_value(165), A::mul(s.ad_value(547), s.ad_value(166)));
        }

        if (s.b[595] && (!s.b[596])) {
            s.store_mul_scale_ad_lhs(134, A::neg(s.ad_value(547)), s.v[9], 166);
        }

        if (s.b[595] && (!s.b[596])) {
            let assign15470_ad_e20002: A = {
                if (s.v[134] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(134)), 1.0))
                } else {
                    {
                        if (s.v[134] > s.v[159]) {
                            A::mul(s.ad_value(160), A::offset(A::sub(s.ad_value(134), s.ad_value(159)), 1.0))
                        } else {
                            A::exp(s.ad_value(134))
                        }
                    }
                }
            };
            s.store_ad_value(135, assign15470_ad_e20002);
        }

        if (s.b[595] && (!s.b[596])) {
            s.store_mul_scaled_ad_rhs(142, 165, -1.0, A::offset(s.ad_value(135), (-1.0)));
        }

        if s.b[595] {
            s.store_add_ad_lhs(544, A::add(s.ad_value(140), s.ad_value(141)), 142);
            s.store_scalar(597, 0.0);
            s.store_scalar(598, 0.0);
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(598, A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[599] = (s.v[146] > 0.5);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        s.b[600] = (s.v[46] == 0.5);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[599]) && s.b[600]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[43]));
        }

        if ((s.b[595] && s.b[599]) && (!s.b[600])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])), s.v[46]);
        }

        if (s.b[595] && s.b[599]) {
            s.store_add_ad(539, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[58]));
        }

        s.b[601] = (s.v[147] > 0.5);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        s.b[602] = (s.v[47] == 0.5);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[601]) && s.b[602]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[44]));
        }

        if ((s.b[595] && s.b[601]) && (!s.b[602])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])), s.v[47]);
        }

        if (s.b[595] && s.b[601]) {
            s.store_add_ad(541, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[59]));
        }

        s.b[603] = (s.v[148] > 0.5);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        s.b[604] = (s.v[48] == 0.5);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[603]) && s.b[604]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[45]));
        }

        if ((s.b[595] && s.b[603]) && (!s.b[604])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])), s.v[48]);
        }

        if (s.b[595] && s.b[603]) {
            s.store_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[60]));
        }

        if (!s.b[595]) {
            s.store_scalar(564, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[605] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[605]) {
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(558, A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[606] = (s.v[547] < s.v[149]);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        s.b[607] = (((((-0.5) * (s.v[547] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && s.b[605]) && s.b[606]) && s.b[607]) {
            s.store_exp_scaled_input(559, 547, (s.v[9] * (-0.5)));
        }

        s.b[608] = (((-0.5) * (s.v[547] * s.v[9])) < 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && s.b[608]) {
            let assign15880_ad_e20424: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(559, assign15880_ad_e20424);
        }

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && (!s.b[608])) {
            s.store_scaled_offset_ad(559, A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((!s.b[595]) && s.b[605]) && s.b[606]) {
            s.store_div_from_scalar(560, 1.0, 559);
            s.store_square(557, 560);
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[606])) {
            s.store_mul_offset_ad_lhs(557, A::scale(A::sub(s.ad_value(547), s.ad_value(149)), s.v[9]), 1.0, 150);
            s.store_sqrt(560, 557);
            s.store_div_from_scalar(559, 1.0, 560);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_offset(557, 557, (-1.0));
        }

        s.b[609] = (s.v[547] > 0.0);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && s.b[605]) && s.b[609]) {
            s.store_scaled_ln_ad(561, A::add(A::offset(s.ad_value(559), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(559), 1.0), A::offset(s.ad_value(559), 3.0)))), (s.v[8] * 2.0));
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[609])) {
            s.store_sub_ad_lhs(561, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(560), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(560), 1.0), A::offset(A::scale(s.ad_value(560), 3.0), 1.0))))), (s.v[8] * 2.0)), 547);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_sub(562, 151, 561);
            s.store_scaled_sub_ad(563, A::add(s.ad_value(547), s.ad_value(562)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(562)), A::sub(s.ad_value(547), s.ad_value(562))), ((4.0 * s.v[8]) * s.v[8]))), 0.5);
            s.store_scaled_sub_ad(564, A::add(s.ad_value(547), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(154)), A::sub(s.ad_value(547), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6]))), 0.5);
            s.store_scaled_sub_ad_rhs(565, 547, A::sqrt(A::offset(A::mul(s.ad_value(547), s.ad_value(547)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[610] = (s.v[143] == 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[610]) {
            s.store_scalar(538, 0.0);
            s.store_scalar(539, 0.0);
        }

        if ((!s.b[595]) && (!s.b[610])) {
            s.store_scale(567, 557, s.v[25]);
        }

        s.b[611] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[611]) {
            s.store_scalar(568, 0.0);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_sub_from_scalar(569, s.v[31], 563);
            s.store_sub_from_scalar_ad(570, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(569)))));
        }

        s.b[612] = (p.p21 == 0.5);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && s.b[612]) {
            s.store_scalar(571, 0.0);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && (!s.b[612])) {
            s.store_scaled_add_ad_lhs(571, A::div(A::mul(A::square(s.ad_value(570)), A::ln(s.ad_value(570))), A::sub_from_scalar(1.0, s.ad_value(570))), 570, (1.0 - (2.0 * p.p21)));
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_add(572, 570, 571);
        }

        s.b[613] = (p.p21 == 0.5);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && s.b[613]) {
            s.store_sqrt_scaled_input(566, 569, s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && (!s.b[613])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[67]), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_scale(573, 566, s.v[61]);
            s.store_mul_scaled_ad_lhs(574, A::offset(s.ad_value(560), (-1.0)), 573, s.v[22]);
            s.store_scaled_mul(568, 574, 572, p.p30);
        }

        s.b[614] = (p.p35 == 0.0);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[614]) {
            s.store_scalar(575, 0.0);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[614])) {
            s.store_scaled_div(576, 573, 569, ((s.v[46]) * (s.v[76])));
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[73]), 576);
            s.store_square(578, 577);
            s.store_sqrt_div_ad(579, A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
        }

        s.b[615] = (((-p.p21) * s.v[49]) == (-1.0));
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && s.b[615]) {
            s.store_div_from_scalar_offset_ad(582, 1.0, A::mul(s.ad_value(576), s.ad_value(581)), 1.0);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[615])) {
            s.store_powf_ad(582, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0), ((-p.p21) * s.v[49]));
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[614])) {
            s.store_div_ad(583, A::mul(s.ad_value(572), s.ad_value(582)), A::add(s.ad_value(572), s.ad_value(582)));
            s.store_sqrt_scaled_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[73]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[73])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[616] = (s.v[587] > 0.0);
        s.v[616] = if s.b[616] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && s.b[616]) {
            s.store_div_from_scalar_offset_scaled_input(549, 1.0, 587, s.v[10], 1.0);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[616])) {
            s.store_div_from_scalar_sub_from_scalar_ad(549, 1.0, 1.0, A::scale(s.ad_value(587), s.v[10]));
        }

        s.b[617] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && s.b[617]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[617])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[614])) {
            s.store_mul_add_ad_lhs(550, A::add(A::scale(s.ad_value(549), 0.29214664), A::scale(A::square(s.ad_value(549)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(549)), s.ad_value(549)), s.v[12]), 566);
        }

        s.b[618] = (s.v[587] > 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && s.b[618]) {
            s.copy_ad(588, 550);
        }

        s.b[619] = (s.v[586] > (-230.25850929940458));
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && s.b[619]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && (!s.b[619])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) {
            s.store_sub_ad_lhs(588, A::scale(s.ad_value(566), 2.0), 550);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[614])) {
            s.store_scaled_div(589, 588, 584, ((s.v[73]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(575, A::mul(s.ad_value(574), s.ad_value(589)), 583, p.p35);
        }

        s.b[620] = (p.p41 == 0.0);
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[620]) {
            s.store_scalar(590, 0.0);
        }

        s.b[621] = (p.p21 == 0.5);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[621]) {
            s.store_sqrt_scaled_ad(566, A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[621])) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[620])) {
            s.store_scaled_div_ad_lhs(591, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[64]), 566, s.v[49]);
        }

        s.b[622] = (((((-s.v[79]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[622]) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(79)), s.ad_value(591)));
        }

        s.b[623] = (((-s.v[79]) / s.v[591]) < 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && s.b[623]) {
            let assign16620_ad_e21596: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign16620_ad_e21596);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && (!s.b[623])) {
            s.store_scaled_offset_ad(566, A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[620])) {
            s.store_mul_scaled_ad_lhs(590, A::mul(A::mul(s.ad_value(547), s.ad_value(591)), s.ad_value(591)), 566, p.p41);
        }

        s.b[624] = (p.p50 > 1000.0);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[624]) {
            s.store_scalar(592, 1.0);
        }

        s.b[625] = (s.v[565] > ((-s.v[82]) * p.p50));
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        s.b[626] = (p.p53 == 4.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && s.b[626]) {
            s.store_mul_scaled_ad_lhs(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[86]), A::scale(s.ad_value(565), s.v[86])), A::scale(s.ad_value(565), s.v[86])), 565, s.v[86]);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && (!s.b[626])) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[86])), p.p53);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) {
            s.store_div_from_scalar_sub_from_scalar_ad(592, 1.0, 1.0, s.ad_value(566));
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && (!s.b[625])) {
            s.store_offset_scaled_ad(592, A::offset(s.ad_value(565), (s.v[82] * p.p50)), s.v[89], s.v[83]);
        }

        if ((!s.b[595]) && (!s.b[610])) {
            s.store_mul_scale_ad_lhs(538, A::add(A::add(A::add(s.ad_value(567), s.ad_value(568)), s.ad_value(575)), s.ad_value(590)), p.p10, 592);
        }

        s.b[627] = (s.v[46] == 0.5);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[627]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[43]));
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[627])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])), s.v[46]);
        }

        if ((!s.b[595]) && (!s.b[610])) {
            s.store_scaled_add_ad(539, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[58]), p.p11);
        }

        s.b[628] = (s.v[144] == 0.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[628]) {
            s.store_scalar(540, 0.0);
            s.store_scalar(541, 0.0);
        }

        if ((!s.b[595]) && (!s.b[628])) {
            s.store_scale(567, 557, s.v[26]);
        }

        s.b[629] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[629]) {
            s.store_scalar(568, 0.0);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_sub_from_scalar(569, s.v[32], 563);
            s.store_sub_from_scalar_ad(570, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(569)))));
        }

        s.b[630] = (p.p22 == 0.5);
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && s.b[630]) {
            s.store_scalar(571, 0.0);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && (!s.b[630])) {
            s.store_scaled_add_ad_lhs(571, A::div(A::mul(A::square(s.ad_value(570)), A::ln(s.ad_value(570))), A::sub_from_scalar(1.0, s.ad_value(570))), 570, (1.0 - (2.0 * p.p22)));
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_add(572, 570, 571);
        }

        s.b[631] = (p.p22 == 0.5);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && s.b[631]) {
            s.store_sqrt_scaled_input(566, 569, s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && (!s.b[631])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[68]), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_scale(573, 566, s.v[62]);
            s.store_mul_scaled_ad_lhs(574, A::offset(s.ad_value(560), (-1.0)), 573, s.v[23]);
            s.store_scaled_mul(568, 574, 572, p.p31);
        }

        s.b[632] = (p.p36 == 0.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[632]) {
            s.store_scalar(575, 0.0);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[632])) {
            s.store_scaled_div(576, 573, 569, ((s.v[47]) * (s.v[77])));
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[74]), 576);
            s.store_square(578, 577);
            s.store_sqrt_div_ad(579, A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
        }

        s.b[633] = (((-p.p22) * s.v[50]) == (-1.0));
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && s.b[633]) {
            s.store_div_from_scalar_offset_ad(582, 1.0, A::mul(s.ad_value(576), s.ad_value(581)), 1.0);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[633])) {
            s.store_powf_ad(582, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0), ((-p.p22) * s.v[50]));
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[632])) {
            s.store_div_ad(583, A::mul(s.ad_value(572), s.ad_value(582)), A::add(s.ad_value(572), s.ad_value(582)));
            s.store_sqrt_scaled_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[74]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[74])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[634] = (s.v[587] > 0.0);
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && s.b[634]) {
            s.store_div_from_scalar_offset_scaled_input(549, 1.0, 587, s.v[10], 1.0);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[634])) {
            s.store_div_from_scalar_sub_from_scalar_ad(549, 1.0, 1.0, A::scale(s.ad_value(587), s.v[10]));
        }

        s.b[635] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && s.b[635]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[635])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[632])) {
            s.store_mul_add_ad_lhs(550, A::add(A::scale(s.ad_value(549), 0.29214664), A::scale(A::square(s.ad_value(549)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(549)), s.ad_value(549)), s.v[12]), 566);
        }

        s.b[636] = (s.v[587] > 0.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && s.b[636]) {
            s.copy_ad(588, 550);
        }

        s.b[637] = (s.v[586] > (-230.25850929940458));
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && s.b[637]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && (!s.b[637])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) {
            s.store_sub_ad_lhs(588, A::scale(s.ad_value(566), 2.0), 550);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[632])) {
            s.store_scaled_div(589, 588, 584, ((s.v[74]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(575, A::mul(s.ad_value(574), s.ad_value(589)), 583, p.p36);
        }

        s.b[638] = (p.p42 == 0.0);
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[638]) {
            s.store_scalar(590, 0.0);
        }

        s.b[639] = (p.p22 == 0.5);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[639]) {
            s.store_sqrt_scaled_ad(566, A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[639])) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[638])) {
            s.store_scaled_div_ad_lhs(591, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[65]), 566, s.v[50]);
        }

        s.b[640] = (((((-s.v[80]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[640]) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(80)), s.ad_value(591)));
        }

        s.b[641] = (((-s.v[80]) / s.v[591]) < 0.0);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && s.b[641]) {
            let assign17370_ad_e22748: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign17370_ad_e22748);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && (!s.b[641])) {
            s.store_scaled_offset_ad(566, A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[638])) {
            s.store_mul_scaled_ad_lhs(590, A::mul(A::mul(s.ad_value(547), s.ad_value(591)), s.ad_value(591)), 566, p.p42);
        }

        s.b[642] = (p.p51 > 1000.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[642]) {
            s.store_scalar(592, 1.0);
        }

        s.b[643] = (s.v[565] > ((-s.v[82]) * p.p51));
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        s.b[644] = (p.p54 == 4.0);
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && s.b[644]) {
            s.store_mul_scaled_ad_lhs(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[87]), A::scale(s.ad_value(565), s.v[87])), A::scale(s.ad_value(565), s.v[87])), 565, s.v[87]);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && (!s.b[644])) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[87])), p.p54);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) {
            s.store_div_from_scalar_sub_from_scalar_ad(592, 1.0, 1.0, s.ad_value(566));
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && (!s.b[643])) {
            s.store_offset_scaled_ad(592, A::offset(s.ad_value(565), (s.v[82] * p.p51)), s.v[90], s.v[84]);
        }

        if ((!s.b[595]) && (!s.b[628])) {
            s.store_mul_scale_ad_lhs(540, A::add(A::add(A::add(s.ad_value(567), s.ad_value(568)), s.ad_value(575)), s.ad_value(590)), p.p10, 592);
        }

        s.b[645] = (s.v[47] == 0.5);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[645]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[44]));
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[645])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])), s.v[47]);
        }

        if ((!s.b[595]) && (!s.b[628])) {
            s.store_scaled_add_ad(541, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[59]), p.p11);
        }

        s.b[646] = (s.v[145] == 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[646]) {
            s.store_scalar(542, 0.0);
            s.store_scalar(543, 0.0);
        }

        if ((!s.b[595]) && (!s.b[646])) {
            s.store_scale(567, 557, s.v[27]);
        }

        s.b[647] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[647]) {
            s.store_scalar(568, 0.0);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_sub_from_scalar(569, s.v[33], 563);
            s.store_sub_from_scalar_ad(570, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(569)))));
        }

        s.b[648] = (p.p23 == 0.5);
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && s.b[648]) {
            s.store_scalar(571, 0.0);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && (!s.b[648])) {
            s.store_scaled_add_ad_lhs(571, A::div(A::mul(A::square(s.ad_value(570)), A::ln(s.ad_value(570))), A::sub_from_scalar(1.0, s.ad_value(570))), 570, (1.0 - (2.0 * p.p23)));
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_add(572, 570, 571);
        }

        s.b[649] = (p.p23 == 0.5);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && s.b[649]) {
            s.store_sqrt_scaled_input(566, 569, s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && (!s.b[649])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[69]), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_scale(573, 566, s.v[63]);
            s.store_mul_scaled_ad_lhs(574, A::offset(s.ad_value(560), (-1.0)), 573, s.v[24]);
            s.store_scaled_mul(568, 574, 572, p.p32);
        }

        s.b[650] = (p.p37 == 0.0);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[650]) {
            s.store_scalar(575, 0.0);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[650])) {
            s.store_scaled_div(576, 573, 569, ((s.v[48]) * (s.v[78])));
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[75]), 576);
            s.store_square(578, 577);
            s.store_sqrt_div_ad(579, A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
        }

        s.b[651] = (((-p.p23) * s.v[51]) == (-1.0));
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && s.b[651]) {
            s.store_div_from_scalar_offset_ad(582, 1.0, A::mul(s.ad_value(576), s.ad_value(581)), 1.0);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[651])) {
            s.store_powf_ad(582, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0), ((-p.p23) * s.v[51]));
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[650])) {
            s.store_div_ad(583, A::mul(s.ad_value(572), s.ad_value(582)), A::add(s.ad_value(572), s.ad_value(582)));
            s.store_sqrt_scaled_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[75]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[75])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[652] = (s.v[587] > 0.0);
        s.v[652] = if s.b[652] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && s.b[652]) {
            s.store_div_from_scalar_offset_scaled_input(549, 1.0, 587, s.v[10], 1.0);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[652])) {
            s.store_div_from_scalar_sub_from_scalar_ad(549, 1.0, 1.0, A::scale(s.ad_value(587), s.v[10]));
        }

        s.b[653] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && s.b[653]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[653])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[650])) {
            s.store_mul_add_ad_lhs(550, A::add(A::scale(s.ad_value(549), 0.29214664), A::scale(A::square(s.ad_value(549)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(549)), s.ad_value(549)), s.v[12]), 566);
        }

        s.b[654] = (s.v[587] > 0.0);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && s.b[654]) {
            s.copy_ad(588, 550);
        }

        s.b[655] = (s.v[586] > (-230.25850929940458));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && s.b[655]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && (!s.b[655])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) {
            s.store_sub_ad_lhs(588, A::scale(s.ad_value(566), 2.0), 550);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[650])) {
            s.store_scaled_div(589, 588, 584, ((s.v[75]) * ((1.772453850905516 * 0.5))));
            s.store_mul_scaled_ad_lhs(575, A::mul(s.ad_value(574), s.ad_value(589)), 583, p.p37);
        }

        s.b[656] = (p.p43 == 0.0);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[656]) {
            s.store_scalar(590, 0.0);
        }

        s.b[657] = (p.p23 == 0.5);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[657]) {
            s.store_sqrt_scaled_ad(566, A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[657])) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[656])) {
            s.store_scaled_div_ad_lhs(591, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[66]), 566, s.v[51]);
        }

        s.b[658] = (((((-s.v[81]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[658]) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(81)), s.ad_value(591)));
        }

        s.b[659] = (((-s.v[81]) / s.v[591]) < 0.0);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && s.b[659]) {
            let assign18120_ad_e23900: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign18120_ad_e23900);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && (!s.b[659])) {
            s.store_scaled_offset_ad(566, A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[656])) {
            s.store_mul_scaled_ad_lhs(590, A::mul(A::mul(s.ad_value(547), s.ad_value(591)), s.ad_value(591)), 566, p.p43);
        }

        s.b[660] = (p.p52 > 1000.0);
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[660]) {
            s.store_scalar(592, 1.0);
        }

        s.b[661] = (s.v[565] > ((-s.v[82]) * p.p52));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        s.b[662] = (p.p55 == 4.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && s.b[662]) {
            s.store_mul_scaled_ad_lhs(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[88]), A::scale(s.ad_value(565), s.v[88])), A::scale(s.ad_value(565), s.v[88])), 565, s.v[88]);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && (!s.b[662])) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[88])), p.p55);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) {
            s.store_div_from_scalar_sub_from_scalar_ad(592, 1.0, 1.0, s.ad_value(566));
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && (!s.b[661])) {
            s.store_offset_scaled_ad(592, A::offset(s.ad_value(565), (s.v[82] * p.p52)), s.v[91], s.v[85]);
        }

        if ((!s.b[595]) && (!s.b[646])) {
            s.store_mul_scale_ad_lhs(542, A::add(A::add(A::add(s.ad_value(567), s.ad_value(568)), s.ad_value(575)), s.ad_value(590)), p.p10, 592);
        }

        s.b[663] = (s.v[111] == 1.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            let assign18250_ad_e24158: A = {
                if (s.v[547] < p.p60) {
                    {
                        if (((s.v[547] - p.p60) / p.p61) < (-37.0)) {
                            A::constant(p.p60)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(547), (-p.p60)), 1.0 / (p.p61))), 1.0)), p.p61), p.p60)
                        }
                    }
                } else {
                    {
                        if (((s.v[547] - p.p60) / p.p61) > 37.0) {
                            s.ad_value(547)
                        } else {
                            A::add(s.ad_value(547), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p60, s.ad_value(547)), 1.0 / (p.p61))), 1.0)), p.p61))
                        }
                    }
                }
            };
            s.store_ad_value(593, assign18250_ad_e24158);
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(594, A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[664] = (s.v[48] == 0.5);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[664]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(594), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[664])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(593), s.ad_value(594)), s.v[60]), p.p11);
            s.store_sub_ad_lhs(593, A::offset(s.ad_value(547), p.p60), 593);
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(594, A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[665] = (s.v[105] == 0.5);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[665]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::mul(s.ad_value(594), s.ad_value(104)));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[665])) {
            s.store_pow_ad(566, A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))), s.ad_value(105));
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_add_ad(110, A::mul(s.ad_value(108), A::sub_from_scalar(1.0, s.ad_value(566))), A::mul(s.ad_value(109), A::sub(s.ad_value(593), s.ad_value(594))), p.p11);
            s.store_add(543, 543, 110);
        }

        s.b[666] = (s.v[48] == 0.5);
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && s.b[666]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && (!s.b[666])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[663])) {
            s.store_scaled_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[60]), p.p11);
        }

        if (!s.b[595]) {
            s.store_add_scaled_ad_lhs(544, A::add(A::scale(s.ad_value(538), s.v[143]), A::scale(s.ad_value(540), s.v[144])), 542, s.v[145]);
        }

        s.store_add_scaled_ad_lhs(545, A::add(A::scale(s.ad_value(539), s.v[143]), A::scale(s.ad_value(541), s.v[144])), 543, s.v[145]);

        s.store_scaled_abs(546, 544, (2.0 * 1.6021918e-19));

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[1] = (8.8541878176e-12 * 11.8);

        s.v[112] = 0.0;

        s.b[187] = (p.p62 > 0.5);
        s.v[187] = if s.b[187] { 1.0 } else { 0.0 };

        if s.b[187] {
            s.store_scalar(112, 1.0);
        }

        if (!s.b[187]) {
            s.store_scalar(112, 0.0);
        }

        s.v[2] = (273.15 + p.p13);

        s.v[5] = (1.3806505e-23 / 1.6021918e-19);

        s.v[6] = (s.v[5] * s.v[2]);

        s.v[7] = (1.0 / s.v[6]);

        s.v[13] = ((-((0.000702 * s.v[2]) * s.v[2])) / (1108.0 + s.v[2]));

        s.v[16] = (p.p24 + s.v[13]);

        s.v[17] = (p.p25 + s.v[13]);

        s.v[18] = (p.p26 + s.v[13]);

        s.v[46] = (1.0 - p.p21);

        s.v[47] = (1.0 - p.p22);

        s.v[48] = (1.0 - p.p23);

        s.v[49] = (1.0 / s.v[46]);

        s.v[50] = (1.0 / s.v[47]);

        s.v[51] = (1.0 / s.v[48]);

        s.v[61] = (s.v[1] / p.p15);

        s.v[62] = ((p.p33 * s.v[1]) / p.p16);

        s.v[63] = ((p.p34 * s.v[1]) / p.p17);

        s.v[64] = (1.0 / s.v[61]);

        s.v[65] = (1.0 / s.v[62]);

        s.v[66] = (1.0 / s.v[63]);

        s.v[67] = (1.0 / p.p18);

        s.v[68] = (1.0 / p.p19);

        s.v[69] = (1.0 / p.p20);

        s.v[82] = (1.0 - (1.0 / p.p14));

        s.v[86] = (1.0 / p.p50);

        s.v[87] = (1.0 / p.p51);

        s.v[88] = (1.0 / p.p52);

        s.b[188] = ((((p.p56 != 1.0) || (p.p57 != 1.0)) || (p.p58 != 1.0)) || (p.p59 != 1.0));
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if s.b[188] {
            s.store_scalar(111, 1.0);
        }

        if (!s.b[188]) {
            s.store_scalar(111, 0.0);
        }

        s.b[189] = (s.v[111] == 1.0);
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        if s.b[189] {
            s.store_scalar(95, (if ((p.p17 * p.p56) > 1e-18) { (p.p17 * p.p56) } else { 1e-18 }));
        }

        if s.b[189] {
            s.store_scalar(96, (if ((p.p20 * p.p57) > 0.05) { (p.p20 * p.p57) } else { 0.05 }));
        }

        if s.b[189] {
            s.store_scalar(97, (if ((if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) < 0.95) { (if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[189] {
            s.store_scalar(98, (p.p26 * p.p59));
            s.store_offset(100, 98, s.v[13]);
            s.store_sub_from_scalar(105, 1.0, 97);
            s.store_div_from_scalar(106, 1.0, 105);
        }

        s.v[3] = (((ctx_temp + p.p2) + p.p9)).max((273.15 + (-250.0)));

        s.v[4] = (s.v[3] / s.v[2]);

        s.v[8] = (s.v[5] * s.v[3]);

        s.v[9] = (1.0 / s.v[8]);

        s.v[14] = ((-((0.000702 * s.v[3]) * s.v[3])) / (1108.0 + s.v[3]));

        s.v[19] = (p.p24 + s.v[14]);

        s.v[20] = (p.p25 + s.v[14]);

        s.v[21] = (p.p26 + s.v[14]);

        s.v[22] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[16] * s.v[7]) - (s.v[19] * s.v[9])))) as f64).exp());

        s.v[23] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[17] * s.v[7]) - (s.v[20] * s.v[9])))) as f64).exp());

        s.v[24] = (((s.v[4]) as f64).powf(1.5) * (((0.5 * ((s.v[18] * s.v[7]) - (s.v[21] * s.v[9])))) as f64).exp());

        s.v[25] = ((p.p27 * s.v[22]) * s.v[22]);

        s.v[26] = ((p.p28 * s.v[23]) * s.v[23]);

        s.v[27] = ((p.p29 * s.v[24]) * s.v[24]);

        s.v[28] = ((p.p18 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[22]) as f64).ln()));

        s.v[29] = ((p.p19 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[23]) as f64).ln()));

        s.v[30] = ((p.p20 * s.v[4]) - ((2.0 * s.v[8]) * ((s.v[24]) as f64).ln()));

        s.v[31] = (s.v[28] + (s.v[8] * (((1.0 + ((((0.05 - s.v[28]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[32] = (s.v[29] + (s.v[8] * (((1.0 + ((((0.05 - s.v[29]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[33] = (s.v[30] + (s.v[8] * (((1.0 + ((((0.05 - s.v[30]) * s.v[9])) as f64).exp())) as f64).ln()));

        s.v[43] = (1.0 / s.v[31]);

        s.v[44] = (1.0 / s.v[32]);

        s.v[45] = (1.0 / s.v[33]);

        s.v[52] = (p.p15 * (((p.p18 * s.v[43])) as f64).powf(p.p21));

        s.v[53] = (p.p16 * (((p.p19 * s.v[44])) as f64).powf(p.p22));

        s.v[54] = (p.p17 * (((p.p20 * s.v[45])) as f64).powf(p.p23));

        s.v[55] = ((s.v[52] * s.v[31]) * s.v[49]);

        s.v[56] = ((s.v[53] * s.v[32]) * s.v[50]);

        s.v[57] = ((s.v[54] * s.v[33]) * s.v[51]);

        s.v[58] = (2.0 * s.v[52]);

        s.v[59] = (2.0 * s.v[53]);

        s.v[60] = (2.0 * s.v[54]);

        s.v[70] = ((0.5 * s.v[19])).max(s.v[8]);

        s.v[71] = ((0.5 * s.v[20])).max(s.v[8]);

        s.v[72] = ((0.5 * s.v[21])).max(s.v[8]);

        s.v[73] = (s.v[70] * s.v[9]);

        s.v[74] = (s.v[71] * s.v[9]);

        s.v[75] = (s.v[72] * s.v[9]);

        s.v[76] = (((((((32.0 * p.p38) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[70] * s.v[70]) * s.v[70]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[77] = (((((((32.0 * p.p39) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[71] * s.v[71]) * s.v[71]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[78] = (((((((32.0 * p.p40) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[72] * s.v[72]) * s.v[72]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[79] = (p.p44 * (1.0 + (p.p47 * (s.v[3] - s.v[2]))));

        s.v[80] = (p.p45 * (1.0 + (p.p48 * (s.v[3] - s.v[2]))));

        s.v[81] = (p.p46 * (1.0 + (p.p49 * (s.v[3] - s.v[2]))));

        if (!(s.v[79] > 0.0)) {
            s.store_scalar(79, 0.0);
        }

        if (!(s.v[80] > 0.0)) {
            s.store_scalar(80, 0.0);
        }

        if (!(s.v[81] > 0.0)) {
            s.store_scalar(81, 0.0);
        }

        s.b[190] = (s.v[111] == 1.0);
        s.v[190] = if s.b[190] { 1.0 } else { 0.0 };

        if s.b[190] {
            s.store_offset(99, 98, s.v[14]);
            s.store_scale_ad(101, A::exp(A::scale(A::sub(A::scale(s.ad_value(100), s.v[7]), A::scale(s.ad_value(99), s.v[9])), 0.5)), ((s.v[4]) as f64).powf(1.5));
            s.store_sub_scaled_ad_rhs(102, 96, s.v[4], A::scale(A::ln(s.ad_value(101)), (2.0 * s.v[8])));
            s.store_add_ad_rhs(103, 102, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(102)), s.v[9])), 1.0)), s.v[8]));
            s.store_div_from_scalar(104, 1.0, 103);
            s.store_mul_pow_ad_rhs(107, 95, A::mul(s.ad_value(96), s.ad_value(104)), s.ad_value(97));
            s.store_mul3_lhs(108, 107, 103, 106);
            s.store_scale(109, 107, 2.0);
        }

        s.v[143] = (if (p.p3 > 0.0) { p.p3 } else { 0.0 });

        s.v[144] = (if (p.p4 > 0.0) { p.p4 } else { 0.0 });

        s.v[145] = (if (p.p5 > 0.0) { p.p5 } else { 0.0 });

        s.v[0] = (if (p.p6 > 0.0) { p.p6 } else { 0.0 });

        s.v[150] = 0.0;

        s.b[191] = ((s.v[25] * s.v[143]) > 0.0);
        s.v[191] = if s.b[191] { 1.0 } else { 0.0 };

        if s.b[191] {
            s.store_scalar(92, (s.v[8] * ((((p.p12 / (s.v[25] * s.v[143])) + 1.0)) as f64).ln()));
        }

        if (!s.b[191]) {
            s.store_scalar(92, 100000000.0);
        }

        s.b[192] = ((s.v[26] * s.v[144]) > 0.0);
        s.v[192] = if s.b[192] { 1.0 } else { 0.0 };

        if s.b[192] {
            s.store_scalar(93, (s.v[8] * ((((p.p12 / (s.v[26] * s.v[144])) + 1.0)) as f64).ln()));
        }

        if (!s.b[192]) {
            s.store_scalar(93, 100000000.0);
        }

        s.b[193] = ((s.v[27] * s.v[145]) > 0.0);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if s.b[193] {
            s.store_scalar(94, (s.v[8] * ((((p.p12 / (s.v[27] * s.v[145])) + 1.0)) as f64).ln()));
        }

        if (!s.b[193]) {
            s.store_scalar(94, 100000000.0);
        }

        s.store_min3(149, 92, 93, 94);

        s.b[194] = ((((s.v[149] * s.v[9])) as f64).abs() < 230.25850929940458);
        s.v[194] = if s.b[194] { 1.0 } else { 0.0 };

        if s.b[194] {
            s.store_exp_scaled_input(150, 149, s.v[9]);
        }

        s.b[195] = ((s.v[149] * s.v[9]) < 0.0);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if ((!s.b[194]) && s.b[195]) {
            s.store_div_from_scalar_offset_ad(150, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((!s.b[194]) && (!s.b[195])) {
            s.store_scaled_offset_ad(150, A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        s.v[34] = s.v[31];

        s.v[35] = s.v[32];

        s.v[36] = s.v[33];

        s.v[37] = p.p21;

        s.v[38] = p.p22;

        s.v[39] = p.p23;

        s.v[40] = p.p18;

        s.v[41] = p.p19;

        s.v[42] = p.p20;

        s.b[196] = (s.v[143] == 0.0);
        s.v[196] = if s.b[196] { 1.0 } else { 0.0 };

        if s.b[196] {
            s.store_scalar(34, (s.v[32] + s.v[33]));
            s.store_scalar(37, (0.9 * (p.p22).min(p.p23)));
            s.store_scalar(40, (p.p19 + p.p20));
        }

        s.b[197] = (s.v[144] == 0.0);
        s.v[197] = if s.b[197] { 1.0 } else { 0.0 };

        if s.b[197] {
            s.store_scalar(35, (s.v[31] + s.v[33]));
            s.store_scalar(38, (0.9 * (p.p21).min(p.p23)));
            s.store_scalar(41, (p.p18 + p.p20));
        }

        s.b[198] = (s.v[145] == 0.0);
        s.v[198] = if s.b[198] { 1.0 } else { 0.0 };

        if s.b[198] {
            s.store_scalar(36, (s.v[31] + s.v[32]));
            s.store_scalar(39, (0.9 * (p.p21).min(p.p22)));
            s.store_scalar(42, (p.p18 + p.p19));
        }

        s.store_min3(151, 34, 35, 36);

        s.store_scale(152, 151, 0.1);

        s.store_max3(15, 37, 38, 39);

        s.store_mul_sub_from_scalar_ad_rhs(153, 151, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(15))));

        s.store_offset_min_ad(154, A::min(s.ad_value(40), s.ad_value(41)), s.ad_value(42), (-0.05));

        s.v[139] = 0.0;

        s.v[146] = 1.0;

        s.v[147] = 1.0;

        s.v[148] = 1.0;

        s.b[199] = (s.v[112] == 1.0);
        s.v[199] = if s.b[199] { 1.0 } else { 0.0 };

        if s.b[199] {
            s.store_scalar(139, (p.p64 * (((s.v[143] * s.v[52]) + (s.v[144] * s.v[53])) + (s.v[145] * s.v[54]))));
        }

        s.b[534] = ((s.v[143] * s.v[52]) <= s.v[139]);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[534]) {
            s.store_scalar(146, 0.0);
        }

        s.b[535] = ((s.v[144] * s.v[53]) <= s.v[139]);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[535]) {
            s.store_scalar(147, 0.0);
        }

        s.b[536] = ((s.v[145] * s.v[54]) <= s.v[139]);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if (s.b[199] && s.b[536]) {
            s.store_scalar(148, 0.0);
        }

        s.v[548] = 0.0;

        s.v[551] = 0.0;

        s.v[552] = 0.0;

        s.v[553] = 0.0;

        s.v[554] = 0.0;

        s.v[555] = 0.0;

        s.v[556] = 0.0;

        s.v[557] = 0.0;

        s.v[558] = 0.0;

        s.v[559] = 0.0;

        s.v[560] = 0.0;

        s.v[561] = 0.0;

        s.v[562] = 0.0;

        s.v[563] = 0.0;

        s.v[564] = 0.0;

        s.v[565] = 0.0;

        s.v[566] = 0.0;

        s.v[569] = 0.0;

        s.v[573] = 0.0;

        s.v[576] = 0.0;

        s.v[577] = 0.0;

        s.v[578] = 0.0;

        s.v[579] = 0.0;

        s.v[580] = 0.0;

        s.v[581] = 0.0;

        s.v[584] = 0.0;

        s.v[585] = 0.0;

        s.v[586] = 0.0;

        s.v[587] = 0.0;

        s.v[591] = 0.0;

        s.v[593] = 0.0;

        s.v[594] = 0.0;

        s.v[539] = 0.0;

        s.v[541] = 0.0;

        s.v[543] = 0.0;

        s.store_scaled_voltage(547, ctx, nodes, Some(0), Some(1), p.p1);

        s.b[595] = (s.v[112] == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_scalar(597, 0.0);
            s.store_scalar(598, 0.0);
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[595] {
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(598, A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[599] = (s.v[146] > 0.5);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        s.b[600] = (s.v[46] == 0.5);
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[599]) && s.b[600]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[43]));
        }

        if ((s.b[595] && s.b[599]) && (!s.b[600])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])), s.v[46]);
        }

        if (s.b[595] && s.b[599]) {
            s.store_add_ad(539, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[58]));
        }

        s.b[601] = (s.v[147] > 0.5);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        s.b[602] = (s.v[47] == 0.5);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[601]) && s.b[602]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[44]));
        }

        if ((s.b[595] && s.b[601]) && (!s.b[602])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])), s.v[47]);
        }

        if (s.b[595] && s.b[601]) {
            s.store_add_ad(541, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[59]));
        }

        s.b[603] = (s.v[148] > 0.5);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        s.b[604] = (s.v[48] == 0.5);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[595] && s.b[603]) && s.b[604]) {
            s.store_sqrt_sub_from_scalar_ad(597, 1.0, A::scale(s.ad_value(598), s.v[45]));
        }

        if ((s.b[595] && s.b[603]) && (!s.b[604])) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])), s.v[48]);
        }

        if (s.b[595] && s.b[603]) {
            s.store_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[60]));
        }

        if (!s.b[595]) {
            s.store_scalar(564, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[605] = (!(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)));
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[605]) {
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(558, A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[606] = (s.v[547] < s.v[149]);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        s.b[607] = (((((-0.5) * (s.v[547] * s.v[9]))) as f64).abs() < 230.25850929940458);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && s.b[605]) && s.b[606]) && s.b[607]) {
            s.store_exp_scaled_input(559, 547, (s.v[9] * (-0.5)));
        }

        s.b[608] = (((-0.5) * (s.v[547] * s.v[9])) < 0.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && s.b[608]) {
            let assign15880_ad_e20424: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(559, assign15880_ad_e20424);
        }

        if (((((!s.b[595]) && s.b[605]) && s.b[606]) && (!s.b[607])) && (!s.b[608])) {
            s.store_scaled_offset_ad(559, A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((!s.b[595]) && s.b[605]) && s.b[606]) {
            s.store_div_from_scalar(560, 1.0, 559);
            s.store_square(557, 560);
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[606])) {
            s.store_mul_offset_ad_lhs(557, A::scale(A::sub(s.ad_value(547), s.ad_value(149)), s.v[9]), 1.0, 150);
            s.store_sqrt(560, 557);
            s.store_div_from_scalar(559, 1.0, 560);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_offset(557, 557, (-1.0));
        }

        s.b[609] = (s.v[547] > 0.0);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && s.b[605]) && s.b[609]) {
            s.store_scaled_ln_ad(561, A::add(A::offset(s.ad_value(559), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(559), 1.0), A::offset(s.ad_value(559), 3.0)))), (s.v[8] * 2.0));
        }

        if (((!s.b[595]) && s.b[605]) && (!s.b[609])) {
            s.store_sub_ad_lhs(561, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(560), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(560), 1.0), A::offset(A::scale(s.ad_value(560), 3.0), 1.0))))), (s.v[8] * 2.0)), 547);
        }

        if ((!s.b[595]) && s.b[605]) {
            s.store_sub(562, 151, 561);
            s.store_scaled_sub_ad(563, A::add(s.ad_value(547), s.ad_value(562)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(562)), A::sub(s.ad_value(547), s.ad_value(562))), ((4.0 * s.v[8]) * s.v[8]))), 0.5);
            s.store_scaled_sub_ad(564, A::add(s.ad_value(547), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(154)), A::sub(s.ad_value(547), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6]))), 0.5);
            s.store_scaled_sub_ad_rhs(565, 547, A::sqrt(A::offset(A::mul(s.ad_value(547), s.ad_value(547)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[610] = (s.v[143] == 0.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[610]) {
            s.store_scalar(539, 0.0);
        }

        s.b[611] = ((p.p30 == 0.0) && (p.p35 == 0.0));
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_sub_from_scalar(569, s.v[31], 563);
        }

        s.b[613] = (p.p21 == 0.5);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && s.b[613]) {
            s.store_sqrt_scaled_input(566, 569, s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[611])) && (!s.b[613])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[67]), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[611])) {
            s.store_scale(573, 566, s.v[61]);
        }

        s.b[614] = (p.p35 == 0.0);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[614])) {
            s.store_scaled_div(576, 573, 569, ((s.v[46]) * (s.v[76])));
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[73]), 576);
            s.store_square(578, 577);
            s.store_sqrt_div_ad(579, A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[73]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[73])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[617] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[617] = if s.b[617] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && s.b[617]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[617])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[618] = (s.v[587] > 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        s.b[619] = (s.v[586] > (-230.25850929940458));
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && s.b[619]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[614])) && (!s.b[618])) && (!s.b[619])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[620] = (p.p41 == 0.0);
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        s.b[621] = (p.p21 == 0.5);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[621]) {
            s.store_sqrt_scaled_ad(566, A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]);
        }

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[621])) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]), p.p21);
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[620])) {
            s.store_scaled_div_ad_lhs(591, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[64]), 566, s.v[49]);
        }

        s.b[622] = (((((-s.v[79]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && s.b[622]) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(79)), s.ad_value(591)));
        }

        s.b[623] = (((-s.v[79]) / s.v[591]) < 0.0);
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && s.b[623]) {
            let assign16620_ad_e21596: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign16620_ad_e21596);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[620])) && (!s.b[622])) && (!s.b[623])) {
            s.store_scaled_offset_ad(566, A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        s.b[624] = (p.p50 > 1000.0);
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        s.b[625] = (s.v[565] > ((-s.v[82]) * p.p50));
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        s.b[626] = (p.p53 == 4.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && s.b[626]) {
            s.store_mul_scaled_ad_lhs(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[86]), A::scale(s.ad_value(565), s.v[86])), A::scale(s.ad_value(565), s.v[86])), 565, s.v[86]);
        }

        if (((((!s.b[595]) && (!s.b[610])) && (!s.b[624])) && s.b[625]) && (!s.b[626])) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[86])), p.p53);
        }

        s.b[627] = (s.v[46] == 0.5);
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[610])) && s.b[627]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[43]));
        }

        if (((!s.b[595]) && (!s.b[610])) && (!s.b[627])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])), s.v[46]);
        }

        if ((!s.b[595]) && (!s.b[610])) {
            s.store_scaled_add_ad(539, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[58]), p.p11);
        }

        s.b[628] = (s.v[144] == 0.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[628]) {
            s.store_scalar(541, 0.0);
        }

        s.b[629] = ((p.p31 == 0.0) && (p.p36 == 0.0));
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_sub_from_scalar(569, s.v[32], 563);
        }

        s.b[631] = (p.p22 == 0.5);
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && s.b[631]) {
            s.store_sqrt_scaled_input(566, 569, s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[629])) && (!s.b[631])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[68]), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[629])) {
            s.store_scale(573, 566, s.v[62]);
        }

        s.b[632] = (p.p36 == 0.0);
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[632])) {
            s.store_scaled_div(576, 573, 569, ((s.v[47]) * (s.v[77])));
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[74]), 576);
            s.store_square(578, 577);
            s.store_sqrt_div_ad(579, A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[74]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[74])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[635] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && s.b[635]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[635])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[636] = (s.v[587] > 0.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        s.b[637] = (s.v[586] > (-230.25850929940458));
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && s.b[637]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[632])) && (!s.b[636])) && (!s.b[637])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[638] = (p.p42 == 0.0);
        s.v[638] = if s.b[638] { 1.0 } else { 0.0 };

        s.b[639] = (p.p22 == 0.5);
        s.v[639] = if s.b[639] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[639]) {
            s.store_sqrt_scaled_ad(566, A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]);
        }

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[639])) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]), p.p22);
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[638])) {
            s.store_scaled_div_ad_lhs(591, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[65]), 566, s.v[50]);
        }

        s.b[640] = (((((-s.v[80]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && s.b[640]) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(80)), s.ad_value(591)));
        }

        s.b[641] = (((-s.v[80]) / s.v[591]) < 0.0);
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && s.b[641]) {
            let assign17370_ad_e22748: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign17370_ad_e22748);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[638])) && (!s.b[640])) && (!s.b[641])) {
            s.store_scaled_offset_ad(566, A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        s.b[642] = (p.p51 > 1000.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        s.b[643] = (s.v[565] > ((-s.v[82]) * p.p51));
        s.v[643] = if s.b[643] { 1.0 } else { 0.0 };

        s.b[644] = (p.p54 == 4.0);
        s.v[644] = if s.b[644] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && s.b[644]) {
            s.store_mul_scaled_ad_lhs(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[87]), A::scale(s.ad_value(565), s.v[87])), A::scale(s.ad_value(565), s.v[87])), 565, s.v[87]);
        }

        if (((((!s.b[595]) && (!s.b[628])) && (!s.b[642])) && s.b[643]) && (!s.b[644])) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[87])), p.p54);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[645] = (s.v[47] == 0.5);
        s.v[645] = if s.b[645] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[628])) && s.b[645]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[44]));
        }

        if (((!s.b[595]) && (!s.b[628])) && (!s.b[645])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])), s.v[47]);
        }

        if ((!s.b[595]) && (!s.b[628])) {
            s.store_scaled_add_ad(541, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[59]), p.p11);
        }

        s.b[646] = (s.v[145] == 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[646]) {
            s.store_scalar(543, 0.0);
        }

        s.b[647] = ((p.p32 == 0.0) && (p.p37 == 0.0));
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_sub_from_scalar(569, s.v[33], 563);
        }

        s.b[649] = (p.p23 == 0.5);
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && s.b[649]) {
            s.store_sqrt_scaled_input(566, 569, s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[647])) && (!s.b[649])) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[69]), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[647])) {
            s.store_scale(573, 566, s.v[63]);
        }

        s.b[650] = (p.p37 == 0.0);
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[650])) {
            s.store_scaled_div(576, 573, 569, ((s.v[48]) * (s.v[78])));
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[75]), 576);
            s.store_square(578, 577);
            s.store_sqrt_div_ad(579, A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0));
            s.store_sqrt(580, 579);
            s.store_mul(581, 579, 580);
            s.store_sqrt_scaled_ad(584, A::div(s.ad_value(576), s.ad_value(580)), 0.375);
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[75]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[75])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
            s.store_mul_offset_lhs(587, 585, (-1.0), 584);
            s.store_square(548, 587);
        }

        s.b[653] = (((-s.v[548]) + s.v[586]) > (-230.25850929940458));
        s.v[653] = if s.b[653] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && s.b[653]) {
            s.store_exp_sub(566, 586, 548);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[653])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[654] = (s.v[587] > 0.0);
        s.v[654] = if s.b[654] { 1.0 } else { 0.0 };

        s.b[655] = (s.v[586] > (-230.25850929940458));
        s.v[655] = if s.b[655] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && s.b[655]) {
            s.store_exp(566, 586);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[650])) && (!s.b[654])) && (!s.b[655])) {
            s.store_div_from_scalar_offset_ad(566, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[656] = (p.p43 == 0.0);
        s.v[656] = if s.b[656] { 1.0 } else { 0.0 };

        s.b[657] = (p.p23 == 0.5);
        s.v[657] = if s.b[657] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[657]) {
            s.store_sqrt_scaled_ad(566, A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]);
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[657])) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]), p.p23);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[656])) {
            s.store_scaled_div_ad_lhs(591, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[66]), 566, s.v[51]);
        }

        s.b[658] = (((((-s.v[81]) / s.v[591])) as f64).abs() < 230.25850929940458);
        s.v[658] = if s.b[658] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && s.b[658]) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(81)), s.ad_value(591)));
        }

        s.b[659] = (((-s.v[81]) / s.v[591]) < 0.0);
        s.v[659] = if s.b[659] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && s.b[659]) {
            let assign18120_ad_e23900: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign18120_ad_e23900);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[656])) && (!s.b[658])) && (!s.b[659])) {
            s.store_scaled_offset_ad(566, A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        s.b[660] = (p.p52 > 1000.0);
        s.v[660] = if s.b[660] { 1.0 } else { 0.0 };

        s.b[661] = (s.v[565] > ((-s.v[82]) * p.p52));
        s.v[661] = if s.b[661] { 1.0 } else { 0.0 };

        s.b[662] = (p.p55 == 4.0);
        s.v[662] = if s.b[662] { 1.0 } else { 0.0 };

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && s.b[662]) {
            s.store_mul_scaled_ad_lhs(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[88]), A::scale(s.ad_value(565), s.v[88])), A::scale(s.ad_value(565), s.v[88])), 565, s.v[88]);
        }

        if (((((!s.b[595]) && (!s.b[646])) && (!s.b[660])) && s.b[661]) && (!s.b[662])) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[88])), p.p55);
        }

        s.b[663] = (s.v[111] == 1.0);
        s.v[663] = if s.b[663] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            let assign18250_ad_e24158: A = {
                if (s.v[547] < p.p60) {
                    {
                        if (((s.v[547] - p.p60) / p.p61) < (-37.0)) {
                            A::constant(p.p60)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(547), (-p.p60)), 1.0 / (p.p61))), 1.0)), p.p61), p.p60)
                        }
                    }
                } else {
                    {
                        if (((s.v[547] - p.p60) / p.p61) > 37.0) {
                            s.ad_value(547)
                        } else {
                            A::add(s.ad_value(547), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p60, s.ad_value(547)), 1.0 / (p.p61))), 1.0)), p.p61))
                        }
                    }
                }
            };
            s.store_ad_value(593, assign18250_ad_e24158);
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(594, A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[664] = (s.v[48] == 0.5);
        s.v[664] = if s.b[664] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[664]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(594), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[664])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(593), s.ad_value(594)), s.v[60]), p.p11);
            s.store_sub_ad_lhs(593, A::offset(s.ad_value(547), p.p60), 593);
            s.store_scaled_square(551, 152, 4.0);
            s.store_div(552, 152, 153);
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
            s.store_add(554, 153, 553);
            s.store_sub(555, 153, 553);
            s.store_sqrt_square_add(556, 555, 551);
            s.store_scaled_div_ad(594, A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556)), 2.0);
        }

        s.b[665] = (s.v[105] == 0.5);
        s.v[665] = if s.b[665] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && s.b[665]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::mul(s.ad_value(594), s.ad_value(104)));
        }

        if ((((!s.b[595]) && (!s.b[646])) && s.b[663]) && (!s.b[665])) {
            s.store_pow_ad(566, A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))), s.ad_value(105));
        }

        if (((!s.b[595]) && (!s.b[646])) && s.b[663]) {
            s.store_scaled_add_ad(110, A::mul(s.ad_value(108), A::sub_from_scalar(1.0, s.ad_value(566))), A::mul(s.ad_value(109), A::sub(s.ad_value(593), s.ad_value(594))), p.p11);
            s.store_add(543, 543, 110);
        }

        s.b[666] = (s.v[48] == 0.5);
        s.v[666] = if s.b[666] { 1.0 } else { 0.0 };

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && s.b[666]) {
            s.store_sqrt_sub_from_scalar_ad(566, 1.0, A::scale(s.ad_value(558), s.v[45]));
        }

        if ((((!s.b[595]) && (!s.b[646])) && (!s.b[663])) && (!s.b[666])) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])), s.v[48]);
        }

        if (((!s.b[595]) && (!s.b[646])) && (!s.b[663])) {
            s.store_scaled_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[60]), p.p11);
        }

        s.store_add_scaled_ad_lhs(545, A::add(A::scale(s.ad_value(539), s.v[143]), A::scale(s.ad_value(541), s.v[144])), 543, s.v[145]);

    }
}
