#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        s.v[11] = ((3.453e-11 * (p.p20 / 3.9)) / p.p19);

        s.v[12] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p24)) as f64).sqrt() / s.v[11]);

        s.v[13] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p29)) as f64).sqrt() / s.v[11]);

        s.v[109] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p54)) as f64).sqrt() / s.v[11]);

        s.b[144] = (p.p30 > 0.0);
        s.v[144] = if s.b[144] { 1.0 } else { 0.0 };

        if s.b[144] {
            s.store_scalar(54, (((0.4 * 5.951993) * p.p30) * ((s.v[11]) as f64).powf(0.6666666666666666)));
        }

        s.b[145] = (p.p17 < 0.0);
        s.v[145] = if s.b[145] { 1.0 } else { 0.0 };

        if (s.b[144] && s.b[145]) {
            s.store_scale(54, 54, (7.448711 / 5.951993));
        }

        if (!s.b[144]) {
            s.store_scalar(54, 0.0);
        }

        s.b[146] = (p.p17 < 0.0);
        s.v[146] = if s.b[146] { 1.0 } else { 0.0 };

        if s.b[146] {
            s.store_scalar(84, (0.3333333333333333 * p.p48));
        }

        if (!s.b[146]) {
            s.store_scalar(84, (0.5 * p.p48));
        }

        s.v[141] = (p.p19 / 1e-9);

        s.v[16] = (if (p.p11 > (-273.0)) { p.p11 } else { (-273.0) });

        s.v[17] = (273.15 + s.v[16]);

        s.v[142] = ((ctx_temp + p.p3) - 273.15);

        s.v[14] = (s.v[142] + 273.15);

        s.v[15] = (s.v[14] * s.v[14]);

        s.v[18] = (s.v[14] - s.v[17]);

        s.v[19] = (s.v[14] / s.v[17]);

        s.v[20] = (s.v[17] / s.v[14]);

        s.v[25] = ((s.v[14] * 1.3806505e-23) / 1.6021918e-19);

        s.v[57] = ((100.0 * s.v[25]) * s.v[25]);

        s.v[26] = (1.0 / s.v[25]);

        s.v[28] = (p.p23 + (s.v[18] * p.p42));

        s.v[27] = ((s.v[20]) as f64).powf(p.p43);

        s.v[29] = (p.p36 * s.v[27]);

        s.v[27] = ((s.v[20]) as f64).powf(p.p44);

        s.v[30] = (p.p37 * s.v[27]);

        s.v[27] = ((s.v[20]) as f64).powf(p.p45);

        s.v[31] = (p.p38 * s.v[27]);

        s.v[27] = ((s.v[20]) as f64).powf(p.p46);

        s.v[32] = (p.p39 * s.v[27]);

        s.v[27] = ((s.v[19]) as f64).powf(p.p47);

        s.v[33] = (p.p40 * s.v[27]);

        s.v[71] = ((4.0 * 1.3806505e-23) * s.v[14]);

        s.v[21] = p.p1;

        s.v[22] = p.p0;

        s.v[23] = (s.v[21] + p.p31);

        s.v[24] = (s.v[22] + p.p32);

        s.v[42] = (1.179 - (s.v[14] * (9.025e-5 + (s.v[14] * 3.05e-7))));

        s.v[48] = ((((1.045 + (0.00045 * s.v[14])) * ((0.523 + (0.0014 * s.v[14])) - (1.48e-6 * s.v[15]))) * s.v[15]) / 90000.0);

        s.v[48] = (s.v[48]).max(0.001);

        s.v[7] = ((s.v[48]) as f64).sqrt();

        s.v[8] = ((s.v[7]) as f64).sqrt();

        s.v[10] = (1.0 / ((2.5e25 * s.v[7]) * s.v[8]));

        s.v[47] = (s.v[42] + ((2.0 * s.v[25]) * (((p.p24 * s.v[10])) as f64).ln()));

        s.v[49] = (s.v[42] + ((2.0 * s.v[25]) * (((p.p29 * s.v[10])) as f64).ln()));

        s.v[135] = (s.v[42] + (6.0 * s.v[25]));

        s.v[6] = ((s.v[26]) as f64).sqrt();

        s.v[35] = (s.v[13] * s.v[6]);

        s.v[38] = (s.v[35] * s.v[35]);

        s.v[39] = (1.0 / s.v[38]);

        s.v[45] = (1.0 + (s.v[35] * 0.7071067811865475));

        s.v[46] = (1.0 / s.v[45]);

        s.v[41] = (1e-5 * s.v[45]);

        s.v[51] = (s.v[49] * s.v[26]);

        s.v[110] = (s.v[109] * s.v[6]);

        s.v[111] = (s.v[110] * s.v[110]);

        s.v[112] = (1.0 + (s.v[110] * 0.7071067811865475));

        s.v[113] = (1e-5 * s.v[112]);

        s.v[116] = (1.25 + (s.v[110] * (((((((-1.25)) as f64).exp() + 1.25) - 1.0)) as f64).sqrt()));

        s.b[157] = (s.v[51] < 460.51701859880916);
        s.v[157] = if s.b[157] { 1.0 } else { 0.0 };

        if s.b[157] {
            s.store_scalar(53, (((-s.v[51])) as f64).exp());
        }

        if (!s.b[157]) {
            s.store_scalar(53, (1e-200 / (1.0 + ((s.v[51] - 460.51701859880916) * (1.0 + ((0.5 * (s.v[51] - 460.51701859880916)) * (1.0 + ((s.v[51] - 460.51701859880916) * 0.3333333333333333))))))));
        }

        s.v[61] = (2.0 * ((p.p35 * s.v[22]) + (p.p34 * s.v[21])));

        if (p.p16 != 0.0) {
            s.store_scalar(62, ((s.v[29] * s.v[22]) / ((3.0 + ((p.p2 - 1.0) * 9.0)) * s.v[21])));
            s.store_scalar(64, (s.v[30] / (s.v[22] * s.v[21])));
            s.store_scalar(68, (s.v[31] / (2.0 * (s.v[22] + p.p33))));
            s.store_scalar(66, ((s.v[32] * s.v[21]) / (12.0 * (s.v[22] + p.p33))));
        }

        if (p.p16 != 0.0) {
            s.store_ad_value(62, {
                if (s.v[62] > 0.001) {
                    {
                        if (s.v[62] < 1000.0) {
                            s.ad_value(62)
                        } else {
                            A::constant(1000.0)
                        }
                    }
                } else {
                    A::constant(0.001)
                }
            });
        }

        if (p.p16 != 0.0) {
            s.store_ad_value(64, {
                if (s.v[64] > 0.001) {
                    {
                        if (s.v[64] < 100.0) {
                            s.ad_value(64)
                        } else {
                            A::constant(100.0)
                        }
                    }
                } else {
                    A::constant(0.001)
                }
            });
        }

        if (p.p16 != 0.0) {
            s.store_ad_value(68, {
                if (s.v[68] > 0.001) {
                    {
                        if (s.v[68] < 1000.0) {
                            s.ad_value(68)
                        } else {
                            A::constant(1000.0)
                        }
                    }
                } else {
                    A::constant(0.001)
                }
            });
        }

        if (p.p16 != 0.0) {
            s.store_ad_value(66, {
                if (s.v[66] > 0.001) {
                    {
                        if (s.v[66] < 1000.0) {
                            s.ad_value(66)
                        } else {
                            A::constant(1000.0)
                        }
                    }
                } else {
                    A::constant(0.001)
                }
            });
        }

        if (p.p16 != 0.0) {
            s.store_scalar(33, (if (s.v[33] > 0.001) { (if (s.v[33] < 20.0) { s.v[33] } else { 20.0 }) } else { 0.001 }));
        }

        if (p.p16 != 0.0) {
            s.store_div_from_scalar(63, 1.0, 62);
            s.store_div_from_scalar(65, 1.0, 64);
            s.store_div_from_scalar(69, 1.0, 68);
            s.store_div_from_scalar(67, 1.0, 66);
            s.store_scale(70, 33, (12.0 * (s.v[22] * 1.0 / (s.v[21]))));
        }

        if (p.p16 == 0.0) {
            s.store_scalar(63, 0.0);
            s.store_scalar(65, 0.0);
            s.store_scalar(69, 0.0);
            s.store_scalar(67, 0.0);
            s.store_scalar(70, 0.0);
        }

        s.store_scale(72, 63, s.v[71]);

        s.store_scale(73, 65, s.v[71]);

        s.store_scale(74, 69, s.v[71]);

        s.store_scale(75, 67, s.v[71]);

        s.b[158] = (p.p66 == 0.0);
        s.v[158] = if s.b[158] { 1.0 } else { 0.0 };

        if s.b[158] {
            s.store_scalar(76, 0.0);
        }

        if (!s.b[158]) {
            s.store_scale(76, 70, s.v[71]);
        }

        s.v[127] = 0.0;

        s.v[128] = 0.0;

        if (p.p49 != 0.0) {
            s.store_scalar(125, (((p.p55 * s.v[24]) * s.v[23]) * 1000000000000.0));
            s.store_scalar(126, ((((2.0 * p.p56) * p.p53) * s.v[24]) * 1000000000000.0));
            s.store_scalar(137, (((p.p60 * s.v[24]) * s.v[23]) * 1000000000000.0));
            s.store_scalar(138, ((((2.0 * p.p61) * p.p53) * s.v[24]) * 1000000000000.0));
            s.store_scalar(119, ((s.v[19]) as f64).powf(p.p52));
            s.store_mul(125, 125, 119);
            s.store_mul(126, 126, 119);
            s.store_mul(137, 137, 119);
            s.store_mul(138, 138, 119);
            s.store_scalar(124, (1.0 / p.p50));
            s.store_scalar(131, (1.0 / p.p51));
            s.store_scalar(9, (((4.0 * 0.3333333333333333) * (((((2.0 * 1.6021918e-19) * 9.1093826e-31) * p.p50)) as f64).sqrt()) / 1.05457168e-34));
            s.store_scale(122, 9, p.p19);
            s.copy_ad(123, 122);
            s.store_scalar(9, (((4.0 * 0.3333333333333333) * (((((2.0 * 1.6021918e-19) * 9.1093826e-31) * p.p51)) as f64).sqrt()) / 1.05457168e-34));
            s.store_scale(132, 9, p.p19);
            s.copy_ad(133, 132);
        }

        s.b[159] = (p.p59 < 0.0);
        s.v[159] = if s.b[159] { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && s.b[159]) {
            s.store_scalar(120, (((-0.495) * p.p58) / p.p59));
        }

        if ((p.p49 != 0.0) && (!s.b[159])) {
            s.store_scalar(120, 0.0);
        }

        s.b[160] = (p.p64 < 0.0);
        s.v[160] = if s.b[160] { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && s.b[160]) {
            s.store_scalar(130, (((-0.495) * p.p63) / p.p64));
        }

        if ((p.p49 != 0.0) && (!s.b[160])) {
            s.store_scalar(130, 0.0);
        }

        if (p.p49 != 0.0) {
            s.store_scalar(93, (0.5 * ((p.p17 * s.v[47]) + s.v[42])));
            s.store_scalar(134, (0.5 * ((p.p17 * s.v[135]) + s.v[42])));
            s.store_scalar(121, (p.p57 * s.v[25]));
            s.store_scalar(129, (p.p62 * s.v[25]));
        }

        if (p.p49 == 0.0) {
            s.store_scalar(125, 0.0);
            s.store_scalar(126, 0.0);
            s.store_scalar(137, 0.0);
            s.store_scalar(138, 0.0);
            s.store_scalar(121, 0.0);
            s.store_scalar(129, 0.0);
            s.store_scalar(120, 0.0);
            s.store_scalar(130, 0.0);
            s.store_scalar(124, 0.1);
            s.store_scalar(131, 0.1);
            s.store_scalar(122, 0.0);
            s.store_scalar(123, 0.0);
            s.store_scalar(132, 0.0);
            s.store_scalar(133, 0.0);
            s.store_scalar(93, 0.0);
            s.store_scalar(134, 0.0);
        }

        let assign1480_ad_e1156: A = {
    if ((p.p17 * ((nv4 - nv5) - p.p27)) > 1e-16) {
        A::scale(A::add(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::sqrt(A::offset(A::mul_scaled_output(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), (p.p17 * p.p17)), p.p28))), 0.5)
    } else {
        let assign1480_ad_e1155: A = {
            if ((-(p.p17 * ((nv4 - nv5) - p.p27))) > 1e-16) {
                A::div_from_scalar((0.5 * p.p28), A::add(A::neg(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17)), A::sqrt(A::offset(A::mul_scaled_output(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), 1.0), p.p28))))
            } else {
                A::scale(A::offset(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), (((1e-32 + p.p28)) as f64).sqrt()), 0.5)
            }
        };
        assign1480_ad_e1155
    }
};
        s.store_offset_scaled_ad(108, assign1480_ad_e1156, p.p26, 1.0);

        let assign1490_ad_e1221: A = {
    if ((p.p25 - s.v[108]) > 1e-16) {
        A::sub_from_scalar(p.p25, A::scale(A::add(A::sub_from_scalar(p.p25, s.ad_value(108)), A::sqrt(A::offset(A::mul(A::sub_from_scalar(p.p25, s.ad_value(108)), A::sub_from_scalar(p.p25, s.ad_value(108))), 1e-6))), 0.5))
    } else {
        let assign1490_ad_e1220: A = {
            if ((s.v[108] - p.p25) > 1e-16) {
                A::sub_from_scalar(p.p25, A::div_from_scalar((0.5 * 1e-6), A::add(A::offset(s.ad_value(108), (-p.p25)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(108), (-p.p25)), A::offset(s.ad_value(108), (-p.p25))), 1e-6)))))
            } else {
                A::sub_from_scalar(p.p25, A::scale(A::offset(A::sub_from_scalar(p.p25, s.ad_value(108)), (((1e-32 + 1e-6)) as f64).sqrt()), 0.5))
            }
        };
        assign1490_ad_e1220
    }
};
        s.store_scale_ad(107, assign1490_ad_e1221, p.p24);

        s.store_scale(140, 107, 1.0000000000000001e-23);

        s.store_offset_scaled_ad(47, A::ln_scaled_input(s.ad_value(107), s.v[10]), (2.0 * s.v[25]), s.v[42]);

        s.store_scaled_sqrt_scaled_input(12, 107, ((2.0 * 1.6021918e-19) * 1.045e-10), 1.0 / (s.v[11]));

        s.b[161] = (p.p30 > 0.0);
        s.v[161] = if s.b[161] { 1.0 } else { 0.0 };

        if s.b[161] {
            s.store_sqrt_mul_ad(55, A::square(s.ad_value(12)), s.ad_value(47));
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[161] {
            s.store_mul_scaled_ad_rhs(56, 54, 0.75, A::powf(s.ad_value(55), 0.6666666666666666));
            s.store_add(47, 47, 56);
            s.store_mul_offset_ad_rhs(12, 12, A::div(A::scale(s.ad_value(56), (2.0 * 0.6666666666666666)), s.ad_value(55)), 1.0);
        }

        s.v[6] = ((s.v[26]) as f64).sqrt();

        s.store_scale(34, 12, s.v[6]);

        s.store_square(36, 34);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_offset_scaled(43, 34, 0.7071067811865475, 1.0);

        s.store_div_from_scalar(44, 1.0, 43);

        s.store_scale(40, 43, 1e-5);

        s.store_scale(50, 47, s.v[26]);

        s.b[162] = (s.v[50] < 460.51701859880916);
        s.v[162] = if s.b[162] { 1.0 } else { 0.0 };

        if s.b[162] {
            s.store_exp_neg_input(52, 50);
        }

        if (!s.b[162]) {
            s.store_div_from_scalar_offset_ad(52, 1e-200, A::mul(A::offset(s.ad_value(50), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(50), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(50), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        s.store_offset_scaled(60, 34, (((((((-1.25)) as f64).exp() + 1.25) - 1.0)) as f64).sqrt(), 1.25);

        s.v[116] = (1.25 + (s.v[110] * (((((((-1.25)) as f64).exp() + 1.25) - 1.0)) as f64).sqrt()));

        s.store_scale_ad(77, A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-s.v[28])), p.p17);

        s.store_scale(78, 77, s.v[26]);

        s.b[184] = (((s.v[78]) as f64).abs() <= s.v[40]);
        s.v[184] = if s.b[184] { 1.0 } else { 0.0 };

        if s.b[184] {
            s.store_scaled_square(165, 44, (0.1666666666666667 * 0.7071067811865475));
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(165)), 1.0));
        }

        s.b[185] = (s.v[78] < (-s.v[40]));
        s.v[185] = if s.b[185] { 1.0 } else { 0.0 };

        if ((!s.b[184]) && s.b[185]) {
            s.store_neg(166, 78);
            s.store_scaled_mul(167, 166, 44, 1.25);
            s.store_scaled_sub_ad(174, A::offset(s.ad_value(167), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(167), (-6.0)), A::offset(s.ad_value(167), (-6.0))), 64.0)), 0.5);
            s.store_sub(164, 166, 174);
            s.store_add_ad(169, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::offset(s.ad_value(174), 1.0)));
            s.store_sub_ad_lhs(171, A::scale(s.ad_value(164), 2.0), 36);
            s.store_sub_ad_lhs(173, A::ln(A::mul(s.ad_value(169), s.ad_value(37))), 174);
            s.store_add(186, 169, 171);
            s.store_add_ad(187, A::square(s.ad_value(186)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(171), 0.5, s.ad_value(171)), s.ad_value(169)), s.ad_value(173)));
            s.store_add_ad_rhs(168, 174, A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(186)), s.ad_value(173)), A::add(s.ad_value(187), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(186), s.ad_value(173)), s.ad_value(173)), s.ad_value(187)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), s.ad_value(169))))));
        }

        s.b[188] = (s.v[168] < 230.25850929940458);
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && s.b[185]) && s.b[188]) {
            s.store_exp(175, 168);
        }

        if (((!s.b[184]) && s.b[185]) && (!s.b[188])) {
            s.store_scaled_offset_ad(175, A::mul(A::offset(s.ad_value(168), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(168), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(168), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((!s.b[184]) && s.b[185]) {
            s.store_div_from_scalar(176, 1.0, 175);
            s.store_div_from_scalar_offset_ad(164, 1.0, A::square(s.ad_value(168)), 2.0);
            s.store_sub(164, 166, 168);
            s.store_mul(165, 52, 176);
            s.store_add_scaled_ad_rhs(177, 164, 2.0, A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(175), (-1.0)), s.ad_value(165)), s.ad_value(52))));
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(175), s.ad_value(168)), (-1.0)), s.ad_value(165)), A::mul(s.ad_value(52), A::offset(s.ad_value(168), (-1.0))))));
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(175), s.ad_value(165))));
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul_scaled_lhs(s.ad_value(178), 2.0, s.ad_value(164)));
            s.store_sub_scaled_ad_rhs(79, 168, -1.0, A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_div_from_scalar_offset_scaled_input(163, 1.0, 34, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(179, A::mul_scaled_lhs(s.ad_value(43), 1.25, s.ad_value(163)), (-1.0), 163);
            s.store_mul_ad(182, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(179), s.ad_value(78)), 1.0));
        }

        s.b[189] = ((-s.v[182]) > (-230.25850929940458));
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && (!s.b[185])) && s.b[189]) {
            s.store_exp_neg_input(164, 182);
        }

        if (((!s.b[184]) && (!s.b[185])) && (!s.b[189])) {
            s.store_div_from_scalar_offset_ad(164, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_sub_from_scalar(181, 1.0, 164);
            s.store_sub_ad(180, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(181)))));
            s.store_offset(172, 50, 3.0);
        }

        if ((!s.b[184]) && (!s.b[185])) {
            let assign2080_ad_e1952: A = {
                if ((s.v[172] - s.v[180]) > 1e-16) {
                    A::sub(s.ad_value(172), A::scale(A::add(A::sub(s.ad_value(172), s.ad_value(180)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(172), s.ad_value(180)), A::sub(s.ad_value(172), s.ad_value(180))), 5.0))), 0.5))
                } else {
                    let assign2080_ad_e1951: A = {
                        if ((s.v[180] - s.v[172]) > 1e-16) {
                            A::sub(s.ad_value(172), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(180), s.ad_value(172)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(180), s.ad_value(172)), A::sub(s.ad_value(180), s.ad_value(172))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(172), A::scale(A::offset(A::sub(s.ad_value(172), s.ad_value(180)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign2080_ad_e1951
                }
            };
            s.store_sub_ad(174, assign2080_ad_e1952, A::scale(A::sub(s.ad_value(172), A::sqrt(A::offset(A::square(s.ad_value(172)), 5.0))), 0.5));
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_sub(164, 78, 174);
            s.store_exp_neg_input(165, 174);
            s.store_max_from_scalar_ad(169, 1e-40, A::sub(A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(165), s.ad_value(174)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(174), 1.0))))));
            s.store_sub_from_scalar_ad(170, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(165)));
            s.store_add_scaled_ad_rhs(171, 164, 2.0, A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(165)), s.ad_value(52))));
            s.store_add_ad(173, A::sub(s.ad_value(50), s.ad_value(174)), A::ln(A::div(s.ad_value(169), s.ad_value(36))));
            s.store_add(190, 169, 171);
        }

        s.b[192] = (((s.v[173]) as f64).abs() < 1e-120);
        s.v[192] = if s.b[192] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && (!s.b[185])) && s.b[192]) {
            s.copy_ad(183, 174);
        }

        if (((!s.b[184]) && (!s.b[185])) && (!s.b[192])) {
            s.store_add_ad(191, A::square(s.ad_value(190)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(171), 0.5, s.ad_value(171)), A::mul(s.ad_value(169), s.ad_value(170))), s.ad_value(173)));
        }

        if (((!s.b[184]) && (!s.b[185])) && (!s.b[192])) {
            let assign2190_ad_e2144: A = A::add(s.ad_value(174), A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(190)), s.ad_value(173)), A::add(s.ad_value(191), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(190), s.ad_value(173)), s.ad_value(173)), s.ad_value(191)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), A::mul(s.ad_value(169), s.ad_value(170)))))));
            s.store_ad_value(183, assign2190_ad_e2144);
        }

        s.b[193] = (s.v[183] < 230.25850929940458);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && (!s.b[185])) && s.b[193]) {
            s.store_exp(175, 183);
            s.store_div_from_scalar(176, 1.0, 175);
            s.store_mul(175, 52, 175);
        }

        s.b[194] = (s.v[183] > (s.v[50] - 230.25850929940458));
        s.v[194] = if s.b[194] { 1.0 } else { 0.0 };

        if ((((!s.b[184]) && (!s.b[185])) && (!s.b[193])) && s.b[194]) {
            s.store_exp_sub(175, 183, 50);
            s.store_div(176, 52, 175);
        }

        if ((((!s.b[184]) && (!s.b[185])) && (!s.b[193])) && (!s.b[194])) {
            s.store_div_from_scalar_offset_ad(175, 1e-100, A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(176, 1e-100, A::mul(A::offset(s.ad_value(183), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(183), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(183), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_div_from_scalar_offset_ad(164, 1.0, A::square(s.ad_value(183)), 2.0);
            s.store_sub(164, 78, 183);
            s.store_add_scaled_ad_rhs(177, 164, 2.0, A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(176)), s.ad_value(175)), s.ad_value(52))));
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(176), s.ad_value(183)), (-1.0)), s.ad_value(175)), A::mul(s.ad_value(52), A::offset(s.ad_value(183), 1.0)))));
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(176), s.ad_value(175))));
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul_scaled_lhs(s.ad_value(178), 2.0, s.ad_value(164)));
            s.store_add_ad_rhs(79, 183, A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        s.b[195] = (p.p29 < 1e27);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if s.b[195] {
            s.store_sub_scaled_inputs(80, 77, (((-p.p17) * p.p18) * s.v[26]), 79, ((s.v[25]) * ((((-p.p17) * p.p18) * s.v[26]))));
        }

        s.b[217] = (((s.v[80]) as f64).abs() <= s.v[41]);
        s.v[217] = if s.b[217] { 1.0 } else { 0.0 };

        if (s.b[195] && s.b[217]) {
            s.store_scalar(198, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
            s.store_mul_scaled_ad_rhs(81, 80, s.v[46], A::offset(A::mul_scaled_lhs(A::mul(s.ad_value(80), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35], s.ad_value(198)), 1.0));
        }

        s.b[218] = (s.v[80] < (-s.v[41]));
        s.v[218] = if s.b[218] { 1.0 } else { 0.0 };

        if ((s.b[195] && (!s.b[217])) && s.b[218]) {
            s.store_neg(199, 80);
            s.store_scale(200, 199, (1.25 * s.v[46]));
            s.store_scaled_sub_ad(207, A::offset(s.ad_value(200), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(200), (-6.0)), A::offset(s.ad_value(200), (-6.0))), 64.0)), 0.5);
            s.store_sub(197, 199, 207);
            s.store_add_ad(202, A::square(s.ad_value(197)), A::scale(A::offset(s.ad_value(207), 1.0), s.v[38]));
            s.store_offset_scaled(204, 197, 2.0, (-s.v[38]));
            s.store_sub_ad_lhs(206, A::ln_scaled_input(s.ad_value(202), s.v[39]), 207);
            s.store_add(219, 202, 204);
            s.store_add_ad(220, A::square(s.ad_value(219)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(204), 0.5, s.ad_value(204)), s.ad_value(202)), s.ad_value(206)));
            s.store_add_ad_rhs(201, 207, A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(219)), s.ad_value(206)), A::add(s.ad_value(220), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(219), s.ad_value(206)), s.ad_value(206)), s.ad_value(220)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), s.ad_value(202))))));
        }

        s.b[221] = (s.v[201] < 230.25850929940458);
        s.v[221] = if s.b[221] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && s.b[218]) && s.b[221]) {
            s.store_exp(208, 201);
        }

        if (((s.b[195] && (!s.b[217])) && s.b[218]) && (!s.b[221])) {
            s.store_scaled_offset_ad(208, A::mul(A::offset(s.ad_value(201), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(201), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(201), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[195] && (!s.b[217])) && s.b[218]) {
            s.store_div_from_scalar(209, 1.0, 208);
            s.store_div_from_scalar_offset_ad(197, 1.0, A::square(s.ad_value(201)), 2.0);
            s.store_sub(197, 199, 201);
            s.store_mul(198, 53, 209);
            s.store_add_scaled_ad_rhs(210, 197, 2.0, A::scale(A::add(A::sub(A::offset(s.ad_value(208), (-1.0)), s.ad_value(198)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(208), s.ad_value(201)), (-1.0)), s.ad_value(198)), A::mul(s.ad_value(53), A::offset(s.ad_value(201), (-1.0)))), s.v[38]));
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(208), s.ad_value(198)), s.v[38]));
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul_scaled_lhs(s.ad_value(211), 2.0, s.ad_value(197)));
            s.store_sub_scaled_ad_rhs(81, 201, -1.0, A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_scalar(196, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
            s.store_mul_offset_ad_lhs(212, A::scale(s.ad_value(196), (s.v[45] * 1.25)), (-1.0), 196);
            s.store_mul_scaled_ad_rhs(215, 80, s.v[46], A::offset(A::mul(s.ad_value(212), s.ad_value(80)), 1.0));
        }

        s.b[222] = ((-s.v[215]) > (-230.25850929940458));
        s.v[222] = if s.b[222] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && s.b[222]) {
            s.store_exp_neg_input(197, 215);
        }

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[222])) {
            s.store_div_from_scalar_offset_ad(197, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_sub_from_scalar(214, 1.0, 197);
            s.store_sub_ad(213, A::offset(s.ad_value(80), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(80), (s.v[38] * 0.25)), s.ad_value(214))), s.v[35]));
            s.store_scalar(205, (s.v[51] + 3.0));
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            let assign2730_ad_e3069: A = {
                if ((s.v[205] - s.v[213]) > 1e-16) {
                    A::sub(s.ad_value(205), A::scale(A::add(A::sub(s.ad_value(205), s.ad_value(213)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(205), s.ad_value(213)), A::sub(s.ad_value(205), s.ad_value(213))), 5.0))), 0.5))
                } else {
                    let assign2730_ad_e3068: A = {
                        if ((s.v[213] - s.v[205]) > 1e-16) {
                            A::sub(s.ad_value(205), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(213), s.ad_value(205)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(213), s.ad_value(205)), A::sub(s.ad_value(213), s.ad_value(205))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(205), A::scale(A::offset(A::sub(s.ad_value(205), s.ad_value(213)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign2730_ad_e3068
                }
            };
            s.store_sub_ad(207, assign2730_ad_e3069, A::scale(A::sub(s.ad_value(205), A::sqrt(A::offset(A::square(s.ad_value(205)), 5.0))), 0.5));
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_sub(197, 80, 207);
            s.store_exp_neg_input(198, 207);
            s.store_max_from_scalar_ad(202, 1e-40, A::sub(A::square(s.ad_value(197)), A::scale(A::sub(A::offset(A::add(s.ad_value(198), s.ad_value(207)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(207), 1.0))), s.v[38])));
            s.store_sub_from_scalar_ad(203, 1.0, A::scale(s.ad_value(198), (0.5 * s.v[38])));
            s.store_add_scaled_ad_rhs(204, 197, 2.0, A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(198)), s.ad_value(53)), s.v[38]));
            s.store_add_ad(206, A::sub_from_scalar(s.v[51], s.ad_value(207)), A::ln_scaled_input(s.ad_value(202), 1.0 / (s.v[38])));
            s.store_add(223, 202, 204);
        }

        s.b[225] = (((s.v[206]) as f64).abs() < 1e-120);
        s.v[225] = if s.b[225] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && s.b[225]) {
            s.copy_ad(216, 207);
        }

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[225])) {
            s.store_add_ad(224, A::square(s.ad_value(223)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(204), 0.5, s.ad_value(204)), A::mul(s.ad_value(202), s.ad_value(203))), s.ad_value(206)));
        }

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[225])) {
            let assign2840_ad_e3281: A = A::add(s.ad_value(207), A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(223)), s.ad_value(206)), A::add(s.ad_value(224), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(223), s.ad_value(206)), s.ad_value(206)), s.ad_value(224)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), A::mul(s.ad_value(202), s.ad_value(203)))))));
            s.store_ad_value(216, assign2840_ad_e3281);
        }

    }

    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[226] = (s.v[216] < 230.25850929940458);
        s.v[226] = if s.b[226] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && s.b[226]) {
            s.store_exp(208, 216);
            s.store_div_from_scalar(209, 1.0, 208);
            s.store_mul(208, 53, 208);
        }

        s.b[227] = (s.v[216] > (s.v[51] - 230.25850929940458));
        s.v[227] = if s.b[227] { 1.0 } else { 0.0 };

        if ((((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[226])) && s.b[227]) {
            s.store_exp_offset_input(208, 216, (-s.v[51]));
            s.store_div(209, 53, 208);
        }

        if ((((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[226])) && (!s.b[227])) {
            s.store_div_from_scalar_offset_ad(208, 1e-100, A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(209, 1e-100, A::mul(A::offset(s.ad_value(216), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(216), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(216), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_div_from_scalar_offset_ad(197, 1.0, A::square(s.ad_value(216)), 2.0);
            s.store_sub(197, 80, 216);
            s.store_add_scaled_ad_rhs(210, 197, 2.0, A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(209)), s.ad_value(208)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(209), s.ad_value(216)), (-1.0)), s.ad_value(208)), A::mul(s.ad_value(53), A::offset(s.ad_value(216), 1.0))), s.v[38]));
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(209), s.ad_value(208)), s.v[38]));
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul_scaled_lhs(s.ad_value(211), 2.0, s.ad_value(197)));
            s.store_add_ad_rhs(81, 216, A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if s.b[195] {
            s.store_scale(82, 81, (((-p.p17) * p.p18) * s.v[25]));
            s.store_scaled_sub(78, 77, 82, 1.0 / (s.v[25]));
        }

        s.b[249] = (((s.v[78]) as f64).abs() <= s.v[40]);
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if (s.b[195] && s.b[249]) {
            s.store_scaled_square(230, 44, (0.1666666666666667 * 0.7071067811865475));
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(230)), 1.0));
        }

        s.b[250] = (s.v[78] < (-s.v[40]));
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if ((s.b[195] && (!s.b[249])) && s.b[250]) {
            s.store_neg(231, 78);
            s.store_scaled_mul(232, 231, 44, 1.25);
            s.store_scaled_sub_ad(239, A::offset(s.ad_value(232), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(232), (-6.0)), A::offset(s.ad_value(232), (-6.0))), 64.0)), 0.5);
            s.store_sub(229, 231, 239);
            s.store_add_ad(234, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::offset(s.ad_value(239), 1.0)));
            s.store_sub_ad_lhs(236, A::scale(s.ad_value(229), 2.0), 36);
            s.store_sub_ad_lhs(238, A::ln(A::mul(s.ad_value(234), s.ad_value(37))), 239);
            s.store_add(251, 234, 236);
            s.store_add_ad(252, A::square(s.ad_value(251)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(236), 0.5, s.ad_value(236)), s.ad_value(234)), s.ad_value(238)));
            s.store_add_ad_rhs(233, 239, A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(251)), s.ad_value(238)), A::add(s.ad_value(252), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(251), s.ad_value(238)), s.ad_value(238)), s.ad_value(252)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), s.ad_value(234))))));
        }

        s.b[253] = (s.v[233] < 230.25850929940458);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && s.b[250]) && s.b[253]) {
            s.store_exp(240, 233);
        }

        if (((s.b[195] && (!s.b[249])) && s.b[250]) && (!s.b[253])) {
            s.store_scaled_offset_ad(240, A::mul(A::offset(s.ad_value(233), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(233), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(233), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[195] && (!s.b[249])) && s.b[250]) {
            s.store_div_from_scalar(241, 1.0, 240);
            s.store_div_from_scalar_offset_ad(229, 1.0, A::square(s.ad_value(233)), 2.0);
            s.store_sub(229, 231, 233);
            s.store_mul(230, 52, 241);
            s.store_add_scaled_ad_rhs(242, 229, 2.0, A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(240), (-1.0)), s.ad_value(230)), s.ad_value(52))));
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(240), s.ad_value(233)), (-1.0)), s.ad_value(230)), A::mul(s.ad_value(52), A::offset(s.ad_value(233), (-1.0))))));
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(240), s.ad_value(230))));
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul_scaled_lhs(s.ad_value(243), 2.0, s.ad_value(229)));
            s.store_sub_scaled_ad_rhs(79, 233, -1.0, A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_div_from_scalar_offset_scaled_input(228, 1.0, 34, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(244, A::mul_scaled_lhs(s.ad_value(43), 1.25, s.ad_value(228)), (-1.0), 228);
            s.store_mul_ad(247, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(244), s.ad_value(78)), 1.0));
        }

        s.b[254] = ((-s.v[247]) > (-230.25850929940458));
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && s.b[254]) {
            s.store_exp_neg_input(229, 247);
        }

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[254])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_sub_from_scalar(246, 1.0, 229);
            s.store_sub_ad(245, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(246)))));
            s.store_offset(237, 50, 3.0);
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            let assign3380_ad_e4235: A = {
                if ((s.v[237] - s.v[245]) > 1e-16) {
                    A::sub(s.ad_value(237), A::scale(A::add(A::sub(s.ad_value(237), s.ad_value(245)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(237), s.ad_value(245)), A::sub(s.ad_value(237), s.ad_value(245))), 5.0))), 0.5))
                } else {
                    let assign3380_ad_e4234: A = {
                        if ((s.v[245] - s.v[237]) > 1e-16) {
                            A::sub(s.ad_value(237), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(245), s.ad_value(237)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(245), s.ad_value(237)), A::sub(s.ad_value(245), s.ad_value(237))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(237), A::scale(A::offset(A::sub(s.ad_value(237), s.ad_value(245)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign3380_ad_e4234
                }
            };
            s.store_sub_ad(239, assign3380_ad_e4235, A::scale(A::sub(s.ad_value(237), A::sqrt(A::offset(A::square(s.ad_value(237)), 5.0))), 0.5));
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_sub(229, 78, 239);
            s.store_exp_neg_input(230, 239);
            s.store_max_from_scalar_ad(234, 1e-40, A::sub(A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(230), s.ad_value(239)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(239), 1.0))))));
            s.store_sub_from_scalar_ad(235, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(230)));
            s.store_add_scaled_ad_rhs(236, 229, 2.0, A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(230)), s.ad_value(52))));
            s.store_add_ad(238, A::sub(s.ad_value(50), s.ad_value(239)), A::ln(A::div(s.ad_value(234), s.ad_value(36))));
            s.store_add(255, 234, 236);
        }

        s.b[257] = (((s.v[238]) as f64).abs() < 1e-120);
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && s.b[257]) {
            s.copy_ad(248, 239);
        }

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[257])) {
            s.store_add_ad(256, A::square(s.ad_value(255)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(236), 0.5, s.ad_value(236)), A::mul(s.ad_value(234), s.ad_value(235))), s.ad_value(238)));
        }

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[257])) {
            let assign3490_ad_e4447: A = A::add(s.ad_value(239), A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(255)), s.ad_value(238)), A::add(s.ad_value(256), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(255), s.ad_value(238)), s.ad_value(238)), s.ad_value(256)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), A::mul(s.ad_value(234), s.ad_value(235)))))));
            s.store_ad_value(248, assign3490_ad_e4447);
        }

        s.b[258] = (s.v[248] < 230.25850929940458);
        s.v[258] = if s.b[258] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && s.b[258]) {
            s.store_exp(240, 248);
            s.store_div_from_scalar(241, 1.0, 240);
            s.store_mul(240, 52, 240);
        }

        s.b[259] = (s.v[248] > (s.v[50] - 230.25850929940458));
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if ((((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[258])) && s.b[259]) {
            s.store_exp_sub(240, 248, 50);
            s.store_div(241, 52, 240);
        }

        if ((((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[258])) && (!s.b[259])) {
            s.store_div_from_scalar_offset_ad(240, 1e-100, A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(241, 1e-100, A::mul(A::offset(s.ad_value(248), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(248), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(248), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_div_from_scalar_offset_ad(229, 1.0, A::square(s.ad_value(248)), 2.0);
            s.store_sub(229, 78, 248);
            s.store_add_scaled_ad_rhs(242, 229, 2.0, A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(241)), s.ad_value(240)), s.ad_value(52))));
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(241), s.ad_value(248)), (-1.0)), s.ad_value(240)), A::mul(s.ad_value(52), A::offset(s.ad_value(248), 1.0)))));
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(241), s.ad_value(240))));
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul_scaled_lhs(s.ad_value(243), 2.0, s.ad_value(229)));
            s.store_add_ad_rhs(79, 248, A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if (!s.b[195]) {
            s.store_scalar(82, 0.0);
        }

        s.b[260] = ((s.v[78] <= 0.0) || (p.p21 < 1.0));
        s.v[260] = if s.b[260] { 1.0 } else { 0.0 };

        if s.b[260] {
            s.store_scalar(90, 0.0);
        }

        if (!s.b[260]) {
            s.store_scalar(83, 0.0);
        }

        s.b[261] = (s.v[79] < 230.25850929940458);
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        if ((!s.b[260]) && s.b[261]) {
            s.store_exp(83, 79);
            s.store_div_from_scalar(85, 1.0, 83);
            s.store_mul(83, 52, 83);
            s.store_mul_offset_ad_rhs(87, 52, A::sub(A::div_from_scalar(1.0, s.ad_value(85)), s.ad_value(79)), (-1.0));
        }

        s.b[262] = (s.v[79] > (s.v[50] - 230.25850929940458));
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[261])) && s.b[262]) {
            s.store_exp_sub(83, 79, 50);
            s.store_div(85, 52, 83);
            s.store_sub_ad_rhs(87, 83, A::mul(s.ad_value(52), A::offset(s.ad_value(79), 1.0)));
        }

        if (((!s.b[260]) && (!s.b[261])) && (!s.b[262])) {
            s.store_div_from_scalar_offset_ad(83, 1e-100, A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(85, 1e-100, A::mul(A::offset(s.ad_value(79), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(79), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(79), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_sub_ad_rhs(87, 83, A::mul(s.ad_value(52), A::offset(s.ad_value(79), 1.0)));
        }

        s.b[263] = (s.v[79] < 1e-5);
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if ((!s.b[260]) && s.b[263]) {
            s.store_mul_ad(86, A::mul_scaled_lhs(s.ad_value(79), 0.5, s.ad_value(79)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(79), 0.3333333333333333, A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25)))));
            s.store_mul_ad(87, A::mul(A::mul(A::mul_scaled_lhs(s.ad_value(52), 0.1666666666666667, s.ad_value(79)), s.ad_value(79)), s.ad_value(79)), A::offset(A::scale(s.ad_value(79), 1.75), 1.0));
            s.store_sqrt_sub_from_scalar_ad(6, 1.0, A::mul_scaled_lhs(s.ad_value(79), 0.3333333333333333, A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25))));
            s.store_scaled_mul(88, 79, 6, 0.7071067811865475);
        }

        if ((!s.b[260]) && (!s.b[263])) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(79), (-1.0)), 85);
            s.store_sqrt(88, 86);
        }

        if (!s.b[260]) {
            s.store_mul_sqrt_ad_rhs(89, 34, A::add(s.ad_value(86), s.ad_value(87)));
            s.store_div_ad(90, A::mul_scaled_lhs(s.ad_value(36), s.v[25], s.ad_value(87)), A::add(s.ad_value(89), A::mul(s.ad_value(34), s.ad_value(88))));
        }

        s.store_neg(92, 90);

        s.store_scale_ad(94, A::add(s.ad_value(77), A::voltage(ctx, nodes, Some(6), None)), s.v[26]);

        s.b[281] = (((s.v[94]) as f64).abs() <= s.v[40]);
        s.v[281] = if s.b[281] { 1.0 } else { 0.0 };

        if s.b[281] {
            s.store_div(95, 94, 43);
        }

        s.b[282] = (s.v[94] > s.v[40]);
        s.v[282] = if s.b[282] { 1.0 } else { 0.0 };

        if ((!s.b[281]) && s.b[282]) {
            s.store_div_ad_lhs(276, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
            s.store_mul_ad(277, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(276), s.ad_value(94)), 1.0));
        }

        s.b[283] = (s.v[277] < 460.51701859880916);
        s.v[283] = if s.b[283] { 1.0 } else { 0.0 };

        if (((!s.b[281]) && s.b[282]) && s.b[283]) {
            s.store_exp_neg_input(275, 277);
        }

        if (((!s.b[281]) && s.b[282]) && (!s.b[283])) {
            s.store_div_from_scalar_offset_ad(275, 1e-200, A::mul(A::offset(s.ad_value(277), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(277), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(277), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[281]) && s.b[282]) {
            s.store_sub_from_scalar(278, 1.0, 275);
            s.store_sub_ad(279, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(278)))));
        }

        s.b[284] = (s.v[279] < 460.51701859880916);
        s.v[284] = if s.b[284] { 1.0 } else { 0.0 };

        if (((!s.b[281]) && s.b[282]) && s.b[284]) {
            s.store_exp_neg_input(271, 279);
        }

        if (((!s.b[281]) && s.b[282]) && (!s.b[284])) {
            s.store_div_from_scalar_offset_ad(271, 1e-200, A::mul(A::offset(s.ad_value(279), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(279), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(279), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[281]) && s.b[282]) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(271)));
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(94), s.ad_value(279)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(271))));
            s.store_sub_ad(274, A::mul(A::sub(s.ad_value(94), s.ad_value(279)), A::sub(s.ad_value(94), s.ad_value(279))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(279), (-1.0)), s.ad_value(271))));
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul_scaled_lhs(s.ad_value(272), 4.0, s.ad_value(274)));
            s.store_div_ad(280, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
            s.store_add(95, 279, 280);
        }

        if ((!s.b[281]) && (!s.b[282])) {
            s.store_neg(264, 94);
            s.store_scaled_div(265, 264, 43, 1.25);
        }

    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((!s.b[281]) && (!s.b[282])) {
            s.store_scaled_sub_ad(266, A::offset(s.ad_value(265), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(265), (-6.0)), A::offset(s.ad_value(265), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(267, A::mul(A::sub(s.ad_value(264), s.ad_value(266)), A::sub(s.ad_value(264), s.ad_value(266))), A::mul(s.ad_value(36), A::offset(s.ad_value(266), 1.0)));
            s.store_sub_ad_lhs(268, A::scale(A::sub(s.ad_value(264), s.ad_value(266)), 2.0), 36);
            s.store_sub_ad_lhs(269, A::ln(A::div(s.ad_value(267), s.ad_value(36))), 266);
            s.store_add(285, 267, 268);
            s.store_add_ad(286, A::square(s.ad_value(285)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(268), 0.5, s.ad_value(268)), s.ad_value(267)), s.ad_value(269)));
            s.store_add_ad_rhs(270, 266, A::div(A::mul(A::mul(s.ad_value(267), s.ad_value(285)), s.ad_value(269)), A::add(s.ad_value(286), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(285), s.ad_value(269)), s.ad_value(269)), s.ad_value(286)), s.ad_value(268)), A::sub(A::scale(A::square(s.ad_value(268)), 0.3333333333333333), s.ad_value(267))))));
        }

        s.b[287] = (((s.v[270]) as f64).abs() < 230.25850929940458);
        s.v[287] = if s.b[287] { 1.0 } else { 0.0 };

        if (((!s.b[281]) && (!s.b[282])) && s.b[287]) {
            s.store_exp(271, 270);
        }

        s.b[288] = (s.v[270] < (-230.25850929940458));
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

        if ((((!s.b[281]) && (!s.b[282])) && (!s.b[287])) && s.b[288]) {
            s.store_div_from_scalar_offset_ad(271, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((!s.b[281]) && (!s.b[282])) && (!s.b[287])) && (!s.b[288])) {
            s.store_scaled_offset_ad(271, A::mul(A::offset(s.ad_value(270), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(270), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(270), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((!s.b[281]) && (!s.b[282])) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(271)));
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(264), s.ad_value(270)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(271), (-1.0))));
            s.store_add_ad(274, A::mul(A::sub(s.ad_value(264), s.ad_value(270)), A::sub(s.ad_value(264), s.ad_value(270))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(270), 1.0), s.ad_value(271))));
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul_scaled_lhs(s.ad_value(272), 4.0, s.ad_value(274)));
            s.store_div_ad(278, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
            s.store_neg_ad(95, A::add(s.ad_value(270), s.ad_value(278)));
        }

        s.store_scale(96, 95, s.v[25]);

        s.b[289] = (p.p29 < 1e27);
        s.v[289] = if s.b[289] { 1.0 } else { 0.0 };

        if s.b[289] {
            s.store_sub_scaled_inputs(97, 77, (((-p.p17) * p.p18) * s.v[26]), 95, ((s.v[25]) * ((((-p.p17) * p.p18) * s.v[26]))));
        }

        s.b[311] = (((s.v[97]) as f64).abs() <= s.v[41]);
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

        if (s.b[289] && s.b[311]) {
            s.store_scalar(292, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
            s.store_mul_scaled_ad_rhs(98, 97, s.v[46], A::offset(A::mul_scaled_lhs(A::mul(s.ad_value(97), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35], s.ad_value(292)), 1.0));
        }

        s.b[312] = (s.v[97] < (-s.v[41]));
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if ((s.b[289] && (!s.b[311])) && s.b[312]) {
            s.store_neg(293, 97);
            s.store_scale(294, 293, (1.25 * s.v[46]));
            s.store_scaled_sub_ad(301, A::offset(s.ad_value(294), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(294), (-6.0)), A::offset(s.ad_value(294), (-6.0))), 64.0)), 0.5);
            s.store_sub(291, 293, 301);
            s.store_add_ad(296, A::square(s.ad_value(291)), A::scale(A::offset(s.ad_value(301), 1.0), s.v[38]));
            s.store_offset_scaled(298, 291, 2.0, (-s.v[38]));
            s.store_sub_ad_lhs(300, A::ln_scaled_input(s.ad_value(296), s.v[39]), 301);
            s.store_add(313, 296, 298);
            s.store_add_ad(314, A::square(s.ad_value(313)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(298), 0.5, s.ad_value(298)), s.ad_value(296)), s.ad_value(300)));
            s.store_add_ad_rhs(295, 301, A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(313)), s.ad_value(300)), A::add(s.ad_value(314), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(313), s.ad_value(300)), s.ad_value(300)), s.ad_value(314)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), s.ad_value(296))))));
        }

        s.b[315] = (s.v[295] < 230.25850929940458);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && s.b[312]) && s.b[315]) {
            s.store_exp(302, 295);
        }

        if (((s.b[289] && (!s.b[311])) && s.b[312]) && (!s.b[315])) {
            s.store_scaled_offset_ad(302, A::mul(A::offset(s.ad_value(295), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(295), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(295), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[289] && (!s.b[311])) && s.b[312]) {
            s.store_div_from_scalar(303, 1.0, 302);
            s.store_div_from_scalar_offset_ad(291, 1.0, A::square(s.ad_value(295)), 2.0);
            s.store_sub(291, 293, 295);
            s.store_mul(292, 53, 303);
            s.store_add_scaled_ad_rhs(304, 291, 2.0, A::scale(A::add(A::sub(A::offset(s.ad_value(302), (-1.0)), s.ad_value(292)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(302), s.ad_value(295)), (-1.0)), s.ad_value(292)), A::mul(s.ad_value(53), A::offset(s.ad_value(295), (-1.0)))), s.v[38]));
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(302), s.ad_value(292)), s.v[38]));
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul_scaled_lhs(s.ad_value(305), 2.0, s.ad_value(291)));
            s.store_sub_scaled_ad_rhs(98, 295, -1.0, A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_scalar(290, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
            s.store_mul_offset_ad_lhs(306, A::scale(s.ad_value(290), (s.v[45] * 1.25)), (-1.0), 290);
            s.store_mul_scaled_ad_rhs(309, 97, s.v[46], A::offset(A::mul(s.ad_value(306), s.ad_value(97)), 1.0));
        }

        s.b[316] = ((-s.v[309]) > (-230.25850929940458));
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && s.b[316]) {
            s.store_exp_neg_input(291, 309);
        }

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[316])) {
            s.store_div_from_scalar_offset_ad(291, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_sub_from_scalar(308, 1.0, 291);
            s.store_sub_ad(307, A::offset(s.ad_value(97), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(97), (s.v[38] * 0.25)), s.ad_value(308))), s.v[35]));
            s.store_scalar(299, (s.v[51] + 3.0));
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            let assign4700_ad_e6331: A = {
                if ((s.v[299] - s.v[307]) > 1e-16) {
                    A::sub(s.ad_value(299), A::scale(A::add(A::sub(s.ad_value(299), s.ad_value(307)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(299), s.ad_value(307)), A::sub(s.ad_value(299), s.ad_value(307))), 5.0))), 0.5))
                } else {
                    let assign4700_ad_e6330: A = {
                        if ((s.v[307] - s.v[299]) > 1e-16) {
                            A::sub(s.ad_value(299), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(307), s.ad_value(299)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(307), s.ad_value(299)), A::sub(s.ad_value(307), s.ad_value(299))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(299), A::scale(A::offset(A::sub(s.ad_value(299), s.ad_value(307)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign4700_ad_e6330
                }
            };
            s.store_sub_ad(301, assign4700_ad_e6331, A::scale(A::sub(s.ad_value(299), A::sqrt(A::offset(A::square(s.ad_value(299)), 5.0))), 0.5));
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_sub(291, 97, 301);
            s.store_exp_neg_input(292, 301);
            s.store_max_from_scalar_ad(296, 1e-40, A::sub(A::square(s.ad_value(291)), A::scale(A::sub(A::offset(A::add(s.ad_value(292), s.ad_value(301)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(301), 1.0))), s.v[38])));
            s.store_sub_from_scalar_ad(297, 1.0, A::scale(s.ad_value(292), (0.5 * s.v[38])));
            s.store_add_scaled_ad_rhs(298, 291, 2.0, A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(292)), s.ad_value(53)), s.v[38]));
            s.store_add_ad(300, A::sub_from_scalar(s.v[51], s.ad_value(301)), A::ln_scaled_input(s.ad_value(296), 1.0 / (s.v[38])));
            s.store_add(317, 296, 298);
        }

        s.b[319] = (((s.v[300]) as f64).abs() < 1e-120);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && s.b[319]) {
            s.copy_ad(310, 301);
        }

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[319])) {
            s.store_add_ad(318, A::square(s.ad_value(317)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(298), 0.5, s.ad_value(298)), A::mul(s.ad_value(296), s.ad_value(297))), s.ad_value(300)));
        }

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[319])) {
            let assign4810_ad_e6543: A = A::add(s.ad_value(301), A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(317)), s.ad_value(300)), A::add(s.ad_value(318), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(317), s.ad_value(300)), s.ad_value(300)), s.ad_value(318)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), A::mul(s.ad_value(296), s.ad_value(297)))))));
            s.store_ad_value(310, assign4810_ad_e6543);
        }

        s.b[320] = (s.v[310] < 230.25850929940458);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && s.b[320]) {
            s.store_exp(302, 310);
            s.store_div_from_scalar(303, 1.0, 302);
            s.store_mul(302, 53, 302);
        }

        s.b[321] = (s.v[310] > (s.v[51] - 230.25850929940458));
        s.v[321] = if s.b[321] { 1.0 } else { 0.0 };

        if ((((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[320])) && s.b[321]) {
            s.store_exp_offset_input(302, 310, (-s.v[51]));
            s.store_div(303, 53, 302);
        }

        if ((((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[320])) && (!s.b[321])) {
            s.store_div_from_scalar_offset_ad(302, 1e-100, A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(303, 1e-100, A::mul(A::offset(s.ad_value(310), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(310), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(310), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_div_from_scalar_offset_ad(291, 1.0, A::square(s.ad_value(310)), 2.0);
            s.store_sub(291, 97, 310);
            s.store_add_scaled_ad_rhs(304, 291, 2.0, A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(303)), s.ad_value(302)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(303), s.ad_value(310)), (-1.0)), s.ad_value(302)), A::mul(s.ad_value(53), A::offset(s.ad_value(310), 1.0))), s.v[38]));
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(303), s.ad_value(302)), s.v[38]));
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul_scaled_lhs(s.ad_value(305), 2.0, s.ad_value(291)));
            s.store_add_ad_rhs(98, 310, A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if s.b[289] {
            s.store_scale(99, 98, (((-p.p17) * p.p18) * s.v[25]));
            s.store_scaled_sub_ad_lhs(94, A::add(s.ad_value(77), A::voltage(ctx, nodes, Some(6), None)), 99, 1.0 / (s.v[25]));
        }

        s.b[339] = (((s.v[94]) as f64).abs() <= s.v[40]);
        s.v[339] = if s.b[339] { 1.0 } else { 0.0 };

        if (s.b[289] && s.b[339]) {
            s.store_div(95, 94, 43);
        }

        s.b[340] = (s.v[94] > s.v[40]);
        s.v[340] = if s.b[340] { 1.0 } else { 0.0 };

        if ((s.b[289] && (!s.b[339])) && s.b[340]) {
            s.store_div_ad_lhs(334, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
            s.store_mul_ad(335, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(334), s.ad_value(94)), 1.0));
        }

        s.b[341] = (s.v[335] < 460.51701859880916);
        s.v[341] = if s.b[341] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[339])) && s.b[340]) && s.b[341]) {
            s.store_exp_neg_input(333, 335);
        }

        if (((s.b[289] && (!s.b[339])) && s.b[340]) && (!s.b[341])) {
            s.store_div_from_scalar_offset_ad(333, 1e-200, A::mul(A::offset(s.ad_value(335), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(335), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(335), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[339])) && s.b[340]) {
            s.store_sub_from_scalar(336, 1.0, 333);
            s.store_sub_ad(337, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(336)))));
        }

        s.b[342] = (s.v[337] < 460.51701859880916);
        s.v[342] = if s.b[342] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[339])) && s.b[340]) && s.b[342]) {
            s.store_exp_neg_input(329, 337);
        }

        if (((s.b[289] && (!s.b[339])) && s.b[340]) && (!s.b[342])) {
            s.store_div_from_scalar_offset_ad(329, 1e-200, A::mul(A::offset(s.ad_value(337), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(337), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(337), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[339])) && s.b[340]) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(329)));
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(94), s.ad_value(337)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(329))));
            s.store_sub_ad(332, A::mul(A::sub(s.ad_value(94), s.ad_value(337)), A::sub(s.ad_value(94), s.ad_value(337))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(337), (-1.0)), s.ad_value(329))));
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul_scaled_lhs(s.ad_value(330), 4.0, s.ad_value(332)));
            s.store_div_ad(338, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
            s.store_add(95, 337, 338);
        }

        if ((s.b[289] && (!s.b[339])) && (!s.b[340])) {
            s.store_neg(322, 94);
            s.store_scaled_div(323, 322, 43, 1.25);
            s.store_scaled_sub_ad(324, A::offset(s.ad_value(323), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(323), (-6.0)), A::offset(s.ad_value(323), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(325, A::mul(A::sub(s.ad_value(322), s.ad_value(324)), A::sub(s.ad_value(322), s.ad_value(324))), A::mul(s.ad_value(36), A::offset(s.ad_value(324), 1.0)));
            s.store_sub_ad_lhs(326, A::scale(A::sub(s.ad_value(322), s.ad_value(324)), 2.0), 36);
            s.store_sub_ad_lhs(327, A::ln(A::div(s.ad_value(325), s.ad_value(36))), 324);
            s.store_add(343, 325, 326);
            s.store_add_ad(344, A::square(s.ad_value(343)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(326), 0.5, s.ad_value(326)), s.ad_value(325)), s.ad_value(327)));
            s.store_add_ad_rhs(328, 324, A::div(A::mul(A::mul(s.ad_value(325), s.ad_value(343)), s.ad_value(327)), A::add(s.ad_value(344), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(343), s.ad_value(327)), s.ad_value(327)), s.ad_value(344)), s.ad_value(326)), A::sub(A::scale(A::square(s.ad_value(326)), 0.3333333333333333), s.ad_value(325))))));
        }

        s.b[345] = (((s.v[328]) as f64).abs() < 230.25850929940458);
        s.v[345] = if s.b[345] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[339])) && (!s.b[340])) && s.b[345]) {
            s.store_exp(329, 328);
        }

        s.b[346] = (s.v[328] < (-230.25850929940458));
        s.v[346] = if s.b[346] { 1.0 } else { 0.0 };

        if ((((s.b[289] && (!s.b[339])) && (!s.b[340])) && (!s.b[345])) && s.b[346]) {
            s.store_div_from_scalar_offset_ad(329, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[289] && (!s.b[339])) && (!s.b[340])) && (!s.b[345])) && (!s.b[346])) {
            s.store_scaled_offset_ad(329, A::mul(A::offset(s.ad_value(328), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(328), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(328), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[289] && (!s.b[339])) && (!s.b[340])) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(329)));
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(322), s.ad_value(328)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(329), (-1.0))));
            s.store_add_ad(332, A::mul(A::sub(s.ad_value(322), s.ad_value(328)), A::sub(s.ad_value(322), s.ad_value(328))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(328), 1.0), s.ad_value(329))));
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul_scaled_lhs(s.ad_value(330), 4.0, s.ad_value(332)));
            s.store_div_ad(336, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
            s.store_neg_ad(95, A::add(s.ad_value(328), s.ad_value(336)));
        }

        if s.b[289] {
            s.store_scale(96, 95, s.v[25]);
        }

        if (!s.b[289]) {
            s.store_scalar(99, 0.0);
        }

        s.v[83] = 0.0;

        s.b[347] = (s.v[95] < 230.25850929940458);
        s.v[347] = if s.b[347] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        if s.b[347] {
            s.store_exp(83, 95);
            s.store_div_from_scalar(85, 1.0, 83);
        }

        s.b[348] = (s.v[95] > (s.v[50] - 230.25850929940458));
        s.v[348] = if s.b[348] { 1.0 } else { 0.0 };

        if ((!s.b[347]) && s.b[348]) {
            s.store_exp_sub(83, 50, 95);
            s.store_mul(85, 52, 83);
        }

        if ((!s.b[347]) && (!s.b[348])) {
            s.store_div_from_scalar_offset_ad(85, 1e-100, A::mul(A::offset(s.ad_value(95), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(95), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(95), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        s.b[349] = (s.v[95] < (-s.v[40]));
        s.v[349] = if s.b[349] { 1.0 } else { 0.0 };

        if s.b[349] {
            s.store_offset_add(86, 85, 95, (-1.0));
            s.store_neg_ad(88, A::sqrt(s.ad_value(86)));
        }

        s.b[350] = (((s.v[95]) as f64).abs() <= s.v[40]);
        s.v[350] = if s.b[350] { 1.0 } else { 0.0 };

        if ((!s.b[349]) && s.b[350]) {
            s.store_sub_from_scalar_ad(6, 1.0, A::mul_scaled_lhs(s.ad_value(95), 0.3333333333333333, A::sub_from_scalar(1.0, A::scale(s.ad_value(95), 0.25))));
            s.store_mul_ad_lhs(86, A::mul_scaled_lhs(s.ad_value(95), 0.5, s.ad_value(95)), 6);
            s.store_mul_scaled_ad_rhs(88, 95, 0.7071067811865475, A::sqrt(s.ad_value(6)));
        }

        if ((!s.b[349]) && (!s.b[350])) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(95), (-1.0)), 85);
            s.store_sqrt(88, 86);
        }

        s.store_scaled_mul(91, 88, 34, s.v[25]);

        s.store_scaled_mul_ad(139, A::offset(s.ad_value(140), 1.0), A::offset(s.ad_value(140), 1.0), (1.62 * ((1.0 + (0.37 * s.v[141])) * ((1.0 + (0.37 * s.v[141])) * (s.v[20] * (((s.v[20]) as f64).sqrt() * (s.v[25] * s.v[25])))))));

        let assign5600_ad_e7802: A = {
    if ((s.v[91] - (-s.v[91])) > 1e-16) {
        A::sub(A::scale(A::add(A::sub(s.ad_value(91), A::neg(s.ad_value(91))), A::sqrt(A::add(A::mul(A::sub(s.ad_value(91), A::neg(s.ad_value(91))), A::sub(s.ad_value(91), A::neg(s.ad_value(91)))), s.ad_value(139)))), 0.5), s.ad_value(91))
    } else {
        let assign5600_ad_e7801: A = {
            if (((-s.v[91]) - s.v[91]) > 1e-16) {
                A::sub(A::div(A::scale(s.ad_value(139), 0.5), A::add(A::sub(A::neg(s.ad_value(91)), s.ad_value(91)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(91)), s.ad_value(91)), A::sub(A::neg(s.ad_value(91)), s.ad_value(91))), s.ad_value(139))))), s.ad_value(91))
            } else {
                A::sub(A::scale(A::add(A::sub(s.ad_value(91), A::neg(s.ad_value(91))), A::sqrt(A::offset(s.ad_value(139), 1e-32))), 0.5), s.ad_value(91))
            }
        };
        assign5600_ad_e7801
    }
};
        let assign5600_ad_e7874: A = {
    if (((-nv6) - nv6) > 1e-16) {
        let assign5600_ad_e7830: A = A::add(A::voltage(ctx, nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None)), A::sqrt(A::add(A::mul(A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None)), A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None))), s.ad_value(139)))), 0.5));
        assign5600_ad_e7830
    } else {
        let assign5600_ad_e7873: A = {
            if ((nv6 - (-nv6)) > 1e-16) {
                let assign5600_ad_e7858: A = A::div(A::scale(s.ad_value(139), 0.5), A::add(A::sub(A::voltage(ctx, nodes, Some(6), None), A::neg(A::voltage(ctx, nodes, Some(6), None))), A::sqrt(A::add(A::mul(A::sub(A::voltage(ctx, nodes, Some(6), None), A::neg(A::voltage(ctx, nodes, Some(6), None))), A::sub(A::voltage(ctx, nodes, Some(6), None), A::neg(A::voltage(ctx, nodes, Some(6), None)))), s.ad_value(139)))));
                A::add(A::voltage(ctx, nodes, Some(6), None), assign5600_ad_e7858)
            } else {
                A::add(A::voltage(ctx, nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None)), A::sqrt(A::offset(s.ad_value(139), 1e-32))), 0.5))
            }
        };
        assign5600_ad_e7873
    }
};
        s.store_add_ad(59, assign5600_ad_e7802, A::mul(s.ad_value(84), assign5600_ad_e7874));

        s.v[58] = s.v[11];

        s.b[351] = (s.v[54] > 0.0);
        s.v[351] = if s.b[351] { 1.0 } else { 0.0 };

        if s.b[351] {
            s.store_div_from_scalar_offset_ad(58, s.v[11], A::mul(s.ad_value(54), A::powf(A::offset(A::square(s.ad_value(59)), s.v[57]), ((-1.0) * 0.1666666666666667))), 1.0);
        }

        let assign5640_ad_e7964: A = {
    if ((10.0 - s.v[79]) > 1e-16) {
        A::sub_from_scalar(10.0, A::scale(A::add(A::sub_from_scalar(10.0, s.ad_value(79)), A::sqrt(A::offset(A::mul(A::sub_from_scalar(10.0, s.ad_value(79)), A::sub_from_scalar(10.0, s.ad_value(79))), 0.01))), 0.5))
    } else {
        let assign5640_ad_e7963: A = {
            if ((s.v[79] - 10.0) > 1e-16) {
                A::sub_from_scalar(10.0, A::div_from_scalar((0.5 * 0.01), A::add(A::offset(s.ad_value(79), (-10.0)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(79), (-10.0)), A::offset(s.ad_value(79), (-10.0))), 0.01)))))
            } else {
                A::sub_from_scalar(10.0, A::scale(A::offset(A::sub_from_scalar(10.0, s.ad_value(79)), (((1e-32 + 0.01)) as f64).sqrt()), 0.5))
            }
        };
        assign5640_ad_e7963
    }
};
        s.store_scale_ad(100, A::exp_scaled_input(assign5640_ad_e7964, (-1.0)), s.v[25]);

        s.store_sqrt(101, 100);

        s.store_mul3_lhs(102, 12, 58, 101);

        s.store_scaled_sub_ad_lhs(103, A::sqrt(A::offset(A::square(s.ad_value(77)), 0.04)), 77, 0.5);

        s.store_div_ad(104, A::mul(s.ad_value(70), s.ad_value(102)), A::offset(A::scale(s.ad_value(103), p.p41), 1.0));

        s.b[352] = (p.p66 == 2.0);
        s.v[352] = if s.b[352] { 1.0 } else { 0.0 };

        if s.b[352] {
            s.store_scale(76, 104, s.v[71]);
        }

        s.v[136] = 0.0;

        s.b[353] = ((p.p18 * p.p17) == (-1.0));
        s.v[353] = if s.b[353] { 1.0 } else { 0.0 };

        if s.b[353] {
            s.store_scalar(136, (p.p18 * s.v[42]));
        }

        s.store_scale_ad(114, A::sub(A::voltage(ctx, nodes, Some(4), Some(1)), s.ad_value(136)), (p.p17 * s.v[26]));

        s.b[354] = ((p.p49 != 0.0) && ((s.v[126] > 0.0) || (s.v[138] > 0.0)));
        s.v[354] = if s.b[354] { 1.0 } else { 0.0 };

        s.b[372] = (((s.v[114]) as f64).abs() <= s.v[113]);
        s.v[372] = if s.b[372] { 1.0 } else { 0.0 };

        if (s.b[354] && s.b[372]) {
            s.store_scale(115, 114, 1.0 / (s.v[112]));
        }

        s.b[373] = (s.v[114] > s.v[113]);
        s.v[373] = if s.b[373] { 1.0 } else { 0.0 };

        if ((s.b[354] && (!s.b[372])) && s.b[373]) {
            s.store_scalar(367, ((((s.v[112] * 1.25) / s.v[116]) - 1.0) / s.v[116]));
            s.store_mul_scaled_ad_rhs(368, 114, 1.0 / (s.v[112]), A::offset(A::mul(s.ad_value(367), s.ad_value(114)), 1.0));
        }

        s.b[374] = (s.v[368] < 460.51701859880916);
        s.v[374] = if s.b[374] { 1.0 } else { 0.0 };

        if (((s.b[354] && (!s.b[372])) && s.b[373]) && s.b[374]) {
            s.store_exp_neg_input(366, 368);
        }

        if (((s.b[354] && (!s.b[372])) && s.b[373]) && (!s.b[374])) {
            s.store_div_from_scalar_offset_ad(366, 1e-200, A::mul(A::offset(s.ad_value(368), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(368), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(368), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[354] && (!s.b[372])) && s.b[373]) {
            s.store_sub_from_scalar(369, 1.0, 366);
            s.store_sub_ad(370, A::offset(s.ad_value(114), (0.5 * s.v[111])), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(114), (0.25 * s.v[111])), s.ad_value(369))), s.v[110]));
        }

        s.b[375] = (s.v[370] < 460.51701859880916);
        s.v[375] = if s.b[375] { 1.0 } else { 0.0 };

        if (((s.b[354] && (!s.b[372])) && s.b[373]) && s.b[375]) {
            s.store_exp_neg_input(362, 370);
        }

        if (((s.b[354] && (!s.b[372])) && s.b[373]) && (!s.b[375])) {
            s.store_div_from_scalar_offset_ad(362, 1e-200, A::mul(A::offset(s.ad_value(370), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(370), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(370), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[354] && (!s.b[372])) && s.b[373]) {
            s.store_sub_from_scalar_ad(363, 1.0, A::scale(s.ad_value(362), (0.5 * s.v[111])));
            s.store_add_ad(364, A::scale(A::sub(s.ad_value(114), s.ad_value(370)), 2.0), A::scale(A::sub_from_scalar(1.0, s.ad_value(362)), s.v[111]));
            s.store_sub_ad(365, A::mul(A::sub(s.ad_value(114), s.ad_value(370)), A::sub(s.ad_value(114), s.ad_value(370))), A::scale(A::add(A::offset(s.ad_value(370), (-1.0)), s.ad_value(362)), s.v[111]));
            s.store_sub_ad(366, A::square(s.ad_value(364)), A::mul_scaled_lhs(s.ad_value(363), 4.0, s.ad_value(365)));
            s.store_div_ad(371, A::scale(s.ad_value(365), 2.0), A::add(s.ad_value(364), A::sqrt(s.ad_value(366))));
            s.store_add(115, 370, 371);
        }

        if ((s.b[354] && (!s.b[372])) && (!s.b[373])) {
            s.store_neg(355, 114);
            s.store_scale(356, 355, (1.25 * 1.0 / (s.v[112])));
            s.store_scaled_sub_ad(357, A::offset(s.ad_value(356), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(356), (-6.0)), A::offset(s.ad_value(356), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(358, A::mul(A::sub(s.ad_value(355), s.ad_value(357)), A::sub(s.ad_value(355), s.ad_value(357))), A::scale(A::offset(s.ad_value(357), 1.0), s.v[111]));
            s.store_offset_scaled_sub(359, 355, 357, 2.0, (-s.v[111]));
            s.store_sub_ad_lhs(360, A::ln_scaled_input(s.ad_value(358), 1.0 / (s.v[111])), 357);
            s.store_add(376, 358, 359);
            s.store_add_ad(377, A::square(s.ad_value(376)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(359), 0.5, s.ad_value(359)), s.ad_value(358)), s.ad_value(360)));
            s.store_add_ad_rhs(361, 357, A::div(A::mul(A::mul(s.ad_value(358), s.ad_value(376)), s.ad_value(360)), A::add(s.ad_value(377), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(376), s.ad_value(360)), s.ad_value(360)), s.ad_value(377)), s.ad_value(359)), A::sub(A::scale(A::square(s.ad_value(359)), 0.3333333333333333), s.ad_value(358))))));
        }

        s.b[378] = (((s.v[361]) as f64).abs() < 230.25850929940458);
        s.v[378] = if s.b[378] { 1.0 } else { 0.0 };

        if (((s.b[354] && (!s.b[372])) && (!s.b[373])) && s.b[378]) {
            s.store_exp(362, 361);
        }

        s.b[379] = (s.v[361] < (-230.25850929940458));
        s.v[379] = if s.b[379] { 1.0 } else { 0.0 };

        if ((((s.b[354] && (!s.b[372])) && (!s.b[373])) && (!s.b[378])) && s.b[379]) {
            s.store_div_from_scalar_offset_ad(362, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(361)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(361)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(361)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[354] && (!s.b[372])) && (!s.b[373])) && (!s.b[378])) && (!s.b[379])) {
            s.store_scaled_offset_ad(362, A::mul(A::offset(s.ad_value(361), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(361), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(361), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[354] && (!s.b[372])) && (!s.b[373])) {
            s.store_sub_from_scalar_ad(363, 1.0, A::scale(s.ad_value(362), (0.5 * s.v[111])));
            s.store_add_ad(364, A::scale(A::sub(s.ad_value(355), s.ad_value(361)), 2.0), A::scale(A::offset(s.ad_value(362), (-1.0)), s.v[111]));
            s.store_add_ad(365, A::mul(A::sub(s.ad_value(355), s.ad_value(361)), A::sub(s.ad_value(355), s.ad_value(361))), A::scale(A::sub(A::offset(s.ad_value(361), 1.0), s.ad_value(362)), s.v[111]));
            s.store_sub_ad(366, A::square(s.ad_value(364)), A::mul_scaled_lhs(s.ad_value(363), 4.0, s.ad_value(365)));
            s.store_div_ad(369, A::scale(s.ad_value(365), 2.0), A::add(s.ad_value(364), A::sqrt(s.ad_value(366))));
            s.store_neg_ad(115, A::add(s.ad_value(361), s.ad_value(369)));
        }

        if s.b[354] {
            s.store_scaled_sub(118, 114, 115, s.v[25]);
        }

        if (!s.b[354]) {
            s.store_scalar(118, 0.0);
            s.store_scalar(115, 0.0);
        }

        s.v[4] = 0.0;

        s.v[5] = 0.0;

        s.b[380] = ((s.v[126] > 0.0) || (s.v[138] > 0.0));
        s.v[380] = if s.b[380] { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && s.b[380]) {
            s.store_scaled_voltage(127, ctx, nodes, Some(4), Some(1), p.p17);
            s.store_scalar(5, 0.0);
        }

        s.b[391] = ((p.p18 == 1.0) && (s.v[138] > 0.0));
        s.v[391] = if s.b[391] { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && s.b[380]) && s.b[391]) {
            s.store_add_ad_lhs(382, A::scale(s.ad_value(118), p.p17), 129);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[391]) {
            let assign6250_ad_e8826: A = {
                if ((-s.v[382]) > 1e-16) {
                    A::add(s.ad_value(382), A::scale(A::add(A::neg(s.ad_value(382)), A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(382), s.ad_value(382), 1.0), 0.01))), 0.5))
                } else {
                    {
                        if (s.v[382] > 1e-16) {
                            A::add(s.ad_value(382), A::div_from_scalar((0.5 * 0.01), A::add(s.ad_value(382), A::sqrt(A::offset(A::mul(s.ad_value(382), s.ad_value(382)), 0.01)))))
                        } else {
                            A::add(s.ad_value(382), A::scale(A::offset(A::neg(s.ad_value(382)), (((1e-32 + 0.01)) as f64).sqrt()), 0.5))
                        }
                    }
                }
            };
            s.store_ad_value(383, assign6250_ad_e8826);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[391]) {
            s.store_mul_sqrt_ad_lhs(384, A::offset(A::square(s.ad_value(118)), 1e-6), 131);
        }

        s.b[392] = (p.p64 < 0.0);
        s.v[392] = if s.b[392] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && s.b[392]) {
            let assign6280_ad_e8915: A = {
                if ((s.v[130] - s.v[384]) > 1e-16) {
                    A::sub(s.ad_value(130), A::scale(A::add(A::sub(s.ad_value(130), s.ad_value(384)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(130), s.ad_value(384)), A::sub(s.ad_value(130), s.ad_value(384))), 1e-6))), 0.5))
                } else {
                    let assign6280_ad_e8914: A = {
                        if ((s.v[384] - s.v[130]) > 1e-16) {
                            A::sub(s.ad_value(130), A::div_from_scalar((0.5 * 1e-6), A::add(A::sub(s.ad_value(384), s.ad_value(130)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(384), s.ad_value(130)), A::sub(s.ad_value(384), s.ad_value(130))), 1e-6)))))
                        } else {
                            A::sub(s.ad_value(130), A::scale(A::offset(A::sub(s.ad_value(130), s.ad_value(384)), (((1e-32 + 1e-6)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign6280_ad_e8914
                }
            };
            s.store_ad_value(384, assign6280_ad_e8915);
        }

        s.b[393] = (0.0 == 0.0);
        s.v[393] = if s.b[393] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && s.b[393]) {
            s.store_neg_ad(385, A::add(A::scale(s.ad_value(115), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(134)), s.ad_value(383)), s.v[26])));
        }

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && (!s.b[393])) {
            s.store_neg_ad(385, A::add(A::scale(s.ad_value(115), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(93)), s.ad_value(383)), s.v[26])));
        }

        s.b[394] = (s.v[385] < 230.25850929940458);
        s.v[394] = if s.b[394] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && s.b[394]) {
            s.store_ln_one_plus_exp(390, 385);
        }

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && (!s.b[394])) {
            s.copy_ad(390, 385);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[391]) {
            s.store_add_ad_rhs(386, 385, A::scale(s.ad_value(127), (p.p17 * s.v[26])));
        }

        s.b[395] = (s.v[386] < 230.25850929940458);
        s.v[395] = if s.b[395] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && s.b[395]) {
            s.store_ln_one_plus_exp(387, 386);
        }

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && (!s.b[395])) {
            s.copy_ad(387, 386);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[391]) {
            s.store_mul_ad_affine_product_rhs(389, 133, s.ad_value(384), A::offset(A::scale(s.ad_value(384), p.p64), p.p63), 1.0, (-1.5));
        }

        s.b[396] = (s.v[389] > 0.0);
        s.v[396] = if s.b[396] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && s.b[396]) {
            s.store_offset_mul_ad(388, s.ad_value(389), A::offset(A::mul_scaled_lhs(s.ad_value(389), 0.5, A::offset(A::scale(s.ad_value(389), 0.3333333333333333), 1.0)), 1.0), 1.0);
        }

        s.b[397] = (s.v[389] > (-230.25850929940458));
        s.v[397] = if s.b[397] { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && (!s.b[396])) && s.b[397]) {
            s.store_exp(388, 389);
        }

        if (((((p.p49 != 0.0) && s.b[380]) && s.b[391]) && (!s.b[396])) && (!s.b[397])) {
            s.store_div_from_scalar_offset_ad(388, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[391]) {
            s.store_scaled_mul_ad(5, A::mul(s.ad_value(138), s.ad_value(388)), A::sub(s.ad_value(387), s.ad_value(390)), p.p17);
        }

        s.b[398] = (s.v[126] > 0.0);
        s.v[398] = if s.b[398] { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && s.b[380]) && s.b[398]) {
            s.store_add_ad_lhs(381, A::scale(s.ad_value(118), p.p17), 121);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[398]) {
            let assign6480_ad_e9234: A = {
                if (s.v[381] > 1e-16) {
                    A::sub(s.ad_value(381), A::scale(A::add(s.ad_value(381), A::sqrt(A::offset(A::mul(s.ad_value(381), s.ad_value(381)), 0.01))), 0.5))
                } else {
                    {
                        if ((-s.v[381]) > 1e-16) {
                            A::sub(s.ad_value(381), A::div_from_scalar((0.5 * 0.01), A::add(A::neg(s.ad_value(381)), A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(381), s.ad_value(381), 1.0), 0.01)))))
                        } else {
                            A::sub(s.ad_value(381), A::scale(A::offset(s.ad_value(381), (((1e-32 + 0.01)) as f64).sqrt()), 0.5))
                        }
                    }
                }
            };
            s.store_ad_value(383, assign6480_ad_e9234);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[398]) {
            s.store_mul_sqrt_ad_lhs(384, A::offset(A::square(s.ad_value(118)), 1e-6), 124);
        }

        s.b[399] = (p.p59 < 0.0);
        s.v[399] = if s.b[399] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && s.b[399]) {
            let assign6510_ad_e9323: A = {
                if ((s.v[120] - s.v[384]) > 1e-16) {
                    A::sub(s.ad_value(120), A::scale(A::add(A::sub(s.ad_value(120), s.ad_value(384)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(120), s.ad_value(384)), A::sub(s.ad_value(120), s.ad_value(384))), 1e-6))), 0.5))
                } else {
                    let assign6510_ad_e9322: A = {
                        if ((s.v[384] - s.v[120]) > 1e-16) {
                            A::sub(s.ad_value(120), A::div_from_scalar((0.5 * 1e-6), A::add(A::sub(s.ad_value(384), s.ad_value(120)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(384), s.ad_value(120)), A::sub(s.ad_value(384), s.ad_value(120))), 1e-6)))))
                        } else {
                            A::sub(s.ad_value(120), A::scale(A::offset(A::sub(s.ad_value(120), s.ad_value(384)), (((1e-32 + 1e-6)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign6510_ad_e9322
                }
            };
            s.store_ad_value(384, assign6510_ad_e9323);
        }

        s.b[400] = (0.0 == 0.0);
        s.v[400] = if s.b[400] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && s.b[400]) {
            s.store_add_scaled_ad_rhs(385, 115, p.p17, A::scale(A::sub(s.ad_value(383), s.ad_value(134)), s.v[26]));
        }

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && (!s.b[400])) {
            s.store_add_scaled_ad_rhs(385, 115, p.p17, A::scale(A::sub(s.ad_value(383), s.ad_value(93)), s.v[26]));
        }

        s.b[401] = (s.v[385] < 230.25850929940458);
        s.v[401] = if s.b[401] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && s.b[401]) {
            s.store_ln_one_plus_exp(390, 385);
        }

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && (!s.b[401])) {
            s.copy_ad(390, 385);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[398]) {
            s.store_sub_ad_rhs(386, 385, A::scale(s.ad_value(127), (p.p17 * s.v[26])));
        }

    }

    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[402] = (s.v[386] < 230.25850929940458);
        s.v[402] = if s.b[402] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && s.b[402]) {
            s.store_ln_one_plus_exp(387, 386);
        }

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && (!s.b[402])) {
            s.copy_ad(387, 386);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[398]) {
            s.store_mul_ad_affine_product_rhs(389, 123, s.ad_value(384), A::offset(A::scale(s.ad_value(384), p.p59), p.p58), 1.0, (-1.5));
        }

        s.b[403] = (((s.v[389]) as f64).abs() < 230.25850929940458);
        s.v[403] = if s.b[403] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && s.b[403]) {
            s.store_exp(388, 389);
        }

        s.b[404] = (s.v[389] < (-230.25850929940458));
        s.v[404] = if s.b[404] { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && (!s.b[403])) && s.b[404]) {
            s.store_div_from_scalar_offset_ad(388, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((((p.p49 != 0.0) && s.b[380]) && s.b[398]) && (!s.b[403])) && (!s.b[404])) {
            s.store_scaled_offset_ad(388, A::mul(A::offset(s.ad_value(389), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(389), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(389), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if (((p.p49 != 0.0) && s.b[380]) && s.b[398]) {
            s.store_add_ad_rhs(5, 5, A::mul_scaled_lhs(A::mul(s.ad_value(126), s.ad_value(388)), p.p17, A::sub(s.ad_value(390), s.ad_value(387))));
        }

        s.b[405] = ((s.v[125] > 0.0) || (s.v[137] > 0.0));
        s.v[405] = if s.b[405] { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && s.b[405]) {
            s.store_scaled_voltage(128, ctx, nodes, Some(4), Some(5), p.p17);
            s.store_scaled_sub(117, 78, 95, s.v[25]);
            s.store_scalar(4, 0.0);
        }

        s.b[416] = ((p.p18 == 1.0) && (s.v[137] > 0.0));
        s.v[416] = if s.b[416] { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && s.b[405]) && s.b[416]) {
            s.store_add_ad_lhs(407, A::scale(s.ad_value(117), p.p17), 129);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[416]) {
            let assign6750_ad_e9682: A = {
                if ((-s.v[407]) > 1e-16) {
                    A::add(s.ad_value(407), A::scale(A::add(A::neg(s.ad_value(407)), A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(407), s.ad_value(407), 1.0), 0.01))), 0.5))
                } else {
                    {
                        if (s.v[407] > 1e-16) {
                            A::add(s.ad_value(407), A::div_from_scalar((0.5 * 0.01), A::add(s.ad_value(407), A::sqrt(A::offset(A::mul(s.ad_value(407), s.ad_value(407)), 0.01)))))
                        } else {
                            A::add(s.ad_value(407), A::scale(A::offset(A::neg(s.ad_value(407)), (((1e-32 + 0.01)) as f64).sqrt()), 0.5))
                        }
                    }
                }
            };
            s.store_ad_value(408, assign6750_ad_e9682);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[416]) {
            s.store_mul_sqrt_ad_lhs(409, A::offset(A::square(s.ad_value(117)), 1e-6), 131);
        }

        s.b[417] = (p.p64 < 0.0);
        s.v[417] = if s.b[417] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && s.b[417]) {
            let assign6780_ad_e9771: A = {
                if ((s.v[130] - s.v[409]) > 1e-16) {
                    A::sub(s.ad_value(130), A::scale(A::add(A::sub(s.ad_value(130), s.ad_value(409)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(130), s.ad_value(409)), A::sub(s.ad_value(130), s.ad_value(409))), 1e-6))), 0.5))
                } else {
                    let assign6780_ad_e9770: A = {
                        if ((s.v[409] - s.v[130]) > 1e-16) {
                            A::sub(s.ad_value(130), A::div_from_scalar((0.5 * 1e-6), A::add(A::sub(s.ad_value(409), s.ad_value(130)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(409), s.ad_value(130)), A::sub(s.ad_value(409), s.ad_value(130))), 1e-6)))))
                        } else {
                            A::sub(s.ad_value(130), A::scale(A::offset(A::sub(s.ad_value(130), s.ad_value(409)), (((1e-32 + 1e-6)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign6780_ad_e9770
                }
            };
            s.store_ad_value(409, assign6780_ad_e9771);
        }

        s.b[418] = (1.0 == 0.0);
        s.v[418] = if s.b[418] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && s.b[418]) {
            s.store_neg_ad(410, A::add(A::scale(s.ad_value(95), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(134)), s.ad_value(408)), s.v[26])));
        }

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && (!s.b[418])) {
            s.store_neg_ad(410, A::add(A::scale(s.ad_value(95), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(93)), s.ad_value(408)), s.v[26])));
        }

        s.b[419] = (s.v[410] < 230.25850929940458);
        s.v[419] = if s.b[419] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && s.b[419]) {
            s.store_ln_one_plus_exp(415, 410);
        }

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && (!s.b[419])) {
            s.copy_ad(415, 410);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[416]) {
            s.store_add_ad_rhs(411, 410, A::scale(s.ad_value(128), (p.p17 * s.v[26])));
        }

        s.b[420] = (s.v[411] < 230.25850929940458);
        s.v[420] = if s.b[420] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && s.b[420]) {
            s.store_ln_one_plus_exp(412, 411);
        }

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && (!s.b[420])) {
            s.copy_ad(412, 411);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[416]) {
            s.store_mul_ad_affine_product_rhs(414, 132, s.ad_value(409), A::offset(A::scale(s.ad_value(409), p.p64), p.p63), 1.0, (-1.5));
        }

        s.b[421] = (s.v[414] > 0.0);
        s.v[421] = if s.b[421] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && s.b[421]) {
            s.store_offset_mul_ad(413, s.ad_value(414), A::offset(A::mul_scaled_lhs(s.ad_value(414), 0.5, A::offset(A::scale(s.ad_value(414), 0.3333333333333333), 1.0)), 1.0), 1.0);
        }

        s.b[422] = (s.v[414] > (-230.25850929940458));
        s.v[422] = if s.b[422] { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && (!s.b[421])) && s.b[422]) {
            s.store_exp(413, 414);
        }

        if (((((p.p49 != 0.0) && s.b[405]) && s.b[416]) && (!s.b[421])) && (!s.b[422])) {
            s.store_div_from_scalar_offset_ad(413, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[416]) {
            s.store_scaled_mul_ad(4, A::mul(s.ad_value(137), s.ad_value(413)), A::sub(s.ad_value(412), s.ad_value(415)), p.p17);
        }

        s.b[423] = (s.v[125] > 0.0);
        s.v[423] = if s.b[423] { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && s.b[405]) && s.b[423]) {
            s.store_add_ad_lhs(406, A::scale(s.ad_value(117), p.p17), 121);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[423]) {
            let assign6980_ad_e10090: A = {
                if (s.v[406] > 1e-16) {
                    A::sub(s.ad_value(406), A::scale(A::add(s.ad_value(406), A::sqrt(A::offset(A::mul(s.ad_value(406), s.ad_value(406)), 0.01))), 0.5))
                } else {
                    {
                        if ((-s.v[406]) > 1e-16) {
                            A::sub(s.ad_value(406), A::div_from_scalar((0.5 * 0.01), A::add(A::neg(s.ad_value(406)), A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(406), s.ad_value(406), 1.0), 0.01)))))
                        } else {
                            A::sub(s.ad_value(406), A::scale(A::offset(s.ad_value(406), (((1e-32 + 0.01)) as f64).sqrt()), 0.5))
                        }
                    }
                }
            };
            s.store_ad_value(408, assign6980_ad_e10090);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[423]) {
            s.store_mul_sqrt_ad_lhs(409, A::offset(A::square(s.ad_value(117)), 1e-6), 124);
        }

        s.b[424] = (p.p59 < 0.0);
        s.v[424] = if s.b[424] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && s.b[424]) {
            let assign7010_ad_e10179: A = {
                if ((s.v[120] - s.v[409]) > 1e-16) {
                    A::sub(s.ad_value(120), A::scale(A::add(A::sub(s.ad_value(120), s.ad_value(409)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(120), s.ad_value(409)), A::sub(s.ad_value(120), s.ad_value(409))), 1e-6))), 0.5))
                } else {
                    let assign7010_ad_e10178: A = {
                        if ((s.v[409] - s.v[120]) > 1e-16) {
                            A::sub(s.ad_value(120), A::div_from_scalar((0.5 * 1e-6), A::add(A::sub(s.ad_value(409), s.ad_value(120)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(409), s.ad_value(120)), A::sub(s.ad_value(409), s.ad_value(120))), 1e-6)))))
                        } else {
                            A::sub(s.ad_value(120), A::scale(A::offset(A::sub(s.ad_value(120), s.ad_value(409)), (((1e-32 + 1e-6)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign7010_ad_e10178
                }
            };
            s.store_ad_value(409, assign7010_ad_e10179);
        }

        s.b[425] = (1.0 == 0.0);
        s.v[425] = if s.b[425] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && s.b[425]) {
            s.store_add_scaled_ad_rhs(410, 95, p.p17, A::scale(A::sub(s.ad_value(408), s.ad_value(134)), s.v[26]));
        }

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && (!s.b[425])) {
            s.store_add_scaled_ad_rhs(410, 95, p.p17, A::scale(A::sub(s.ad_value(408), s.ad_value(93)), s.v[26]));
        }

        s.b[426] = (s.v[410] < 230.25850929940458);
        s.v[426] = if s.b[426] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && s.b[426]) {
            s.store_ln_one_plus_exp(415, 410);
        }

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && (!s.b[426])) {
            s.copy_ad(415, 410);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[423]) {
            s.store_sub_ad_rhs(411, 410, A::scale(s.ad_value(128), (p.p17 * s.v[26])));
        }

        s.b[427] = (s.v[411] < 230.25850929940458);
        s.v[427] = if s.b[427] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && s.b[427]) {
            s.store_ln_one_plus_exp(412, 411);
        }

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && (!s.b[427])) {
            s.copy_ad(412, 411);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[423]) {
            s.store_mul_ad_affine_product_rhs(414, 122, s.ad_value(409), A::offset(A::scale(s.ad_value(409), p.p59), p.p58), 1.0, (-1.5));
        }

        s.b[428] = (((s.v[414]) as f64).abs() < 230.25850929940458);
        s.v[428] = if s.b[428] { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && s.b[428]) {
            s.store_exp(413, 414);
        }

        s.b[429] = (s.v[414] < (-230.25850929940458));
        s.v[429] = if s.b[429] { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && (!s.b[428])) && s.b[429]) {
            s.store_div_from_scalar_offset_ad(413, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if (((((p.p49 != 0.0) && s.b[405]) && s.b[423]) && (!s.b[428])) && (!s.b[429])) {
            s.store_scaled_offset_ad(413, A::mul(A::offset(s.ad_value(414), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(414), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(414), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if (((p.p49 != 0.0) && s.b[405]) && s.b[423]) {
            s.store_add_ad_rhs(4, 4, A::mul_scaled_lhs(A::mul(s.ad_value(125), s.ad_value(413)), p.p17, A::sub(s.ad_value(415), s.ad_value(412))));
        }

        s.store_mul_scaled_ad_lhs(3, A::sub(A::sub(s.ad_value(77), s.ad_value(96)), s.ad_value(99)), 58, ((s.v[23] * s.v[24]) * p.p17));

        s.store_scaled_voltage(105, ctx, nodes, Some(6), None, p.p22);

        s.store_scaled_voltage(106, ctx, nodes, Some(3), Some(1), s.v[61]);

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        s.v[11] = ((3.453e-11 * (p.p20 / 3.9)) / p.p19);

        s.v[12] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p24)) as f64).sqrt() / s.v[11]);

        s.v[13] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p29)) as f64).sqrt() / s.v[11]);

        s.b[144] = (p.p30 > 0.0);
        s.v[144] = if s.b[144] { 1.0 } else { 0.0 };

        if s.b[144] {
            s.store_scalar(54, (((0.4 * 5.951993) * p.p30) * ((s.v[11]) as f64).powf(0.6666666666666666)));
        }

        s.b[145] = (p.p17 < 0.0);
        s.v[145] = if s.b[145] { 1.0 } else { 0.0 };

        if (s.b[144] && s.b[145]) {
            s.store_scale(54, 54, (7.448711 / 5.951993));
        }

        if (!s.b[144]) {
            s.store_scalar(54, 0.0);
        }

        s.b[146] = (p.p17 < 0.0);
        s.v[146] = if s.b[146] { 1.0 } else { 0.0 };

        if s.b[146] {
            s.store_scalar(84, (0.3333333333333333 * p.p48));
        }

        if (!s.b[146]) {
            s.store_scalar(84, (0.5 * p.p48));
        }

        s.v[141] = (p.p19 / 1e-9);

        s.v[16] = (if (p.p11 > (-273.0)) { p.p11 } else { (-273.0) });

        s.v[17] = (273.15 + s.v[16]);

        s.v[142] = ((ctx_temp + p.p3) - 273.15);

        s.v[14] = (s.v[142] + 273.15);

        s.v[15] = (s.v[14] * s.v[14]);

        s.v[18] = (s.v[14] - s.v[17]);

        s.v[20] = (s.v[17] / s.v[14]);

        s.v[25] = ((s.v[14] * 1.3806505e-23) / 1.6021918e-19);

        s.v[57] = ((100.0 * s.v[25]) * s.v[25]);

        s.v[26] = (1.0 / s.v[25]);

        s.v[28] = (p.p23 + (s.v[18] * p.p42));

        s.v[21] = p.p1;

        s.v[22] = p.p0;

        s.v[23] = (s.v[21] + p.p31);

        s.v[24] = (s.v[22] + p.p32);

        s.v[42] = (1.179 - (s.v[14] * (9.025e-5 + (s.v[14] * 3.05e-7))));

        s.v[48] = ((((1.045 + (0.00045 * s.v[14])) * ((0.523 + (0.0014 * s.v[14])) - (1.48e-6 * s.v[15]))) * s.v[15]) / 90000.0);

        s.v[48] = (s.v[48]).max(0.001);

        s.v[7] = ((s.v[48]) as f64).sqrt();

        s.v[8] = ((s.v[7]) as f64).sqrt();

        s.v[10] = (1.0 / ((2.5e25 * s.v[7]) * s.v[8]));

        s.v[47] = (s.v[42] + ((2.0 * s.v[25]) * (((p.p24 * s.v[10])) as f64).ln()));

        s.v[49] = (s.v[42] + ((2.0 * s.v[25]) * (((p.p29 * s.v[10])) as f64).ln()));

        s.v[6] = ((s.v[26]) as f64).sqrt();

        s.v[35] = (s.v[13] * s.v[6]);

        s.v[38] = (s.v[35] * s.v[35]);

        s.v[39] = (1.0 / s.v[38]);

        s.v[45] = (1.0 + (s.v[35] * 0.7071067811865475));

        s.v[46] = (1.0 / s.v[45]);

        s.v[41] = (1e-5 * s.v[45]);

        s.v[51] = (s.v[49] * s.v[26]);

        s.b[157] = (s.v[51] < 460.51701859880916);
        s.v[157] = if s.b[157] { 1.0 } else { 0.0 };

        if s.b[157] {
            s.store_scalar(53, (((-s.v[51])) as f64).exp());
        }

        if (!s.b[157]) {
            s.store_scalar(53, (1e-200 / (1.0 + ((s.v[51] - 460.51701859880916) * (1.0 + ((0.5 * (s.v[51] - 460.51701859880916)) * (1.0 + ((s.v[51] - 460.51701859880916) * 0.3333333333333333))))))));
        }

        s.v[61] = (2.0 * ((p.p35 * s.v[22]) + (p.p34 * s.v[21])));

        let assign1480_ad_e1156: A = {
    if ((p.p17 * ((nv4 - nv5) - p.p27)) > 1e-16) {
        A::scale(A::add(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::sqrt(A::offset(A::mul_scaled_output(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), (p.p17 * p.p17)), p.p28))), 0.5)
    } else {
        let assign1480_ad_e1155: A = {
            if ((-(p.p17 * ((nv4 - nv5) - p.p27))) > 1e-16) {
                A::div_from_scalar((0.5 * p.p28), A::add(A::neg(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17)), A::sqrt(A::offset(A::mul_scaled_output(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), 1.0), p.p28))))
            } else {
                A::scale(A::offset(A::scale(A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-p.p27)), p.p17), (((1e-32 + p.p28)) as f64).sqrt()), 0.5)
            }
        };
        assign1480_ad_e1155
    }
};
        s.store_offset_scaled_ad(108, assign1480_ad_e1156, p.p26, 1.0);

        let assign1490_ad_e1221: A = {
    if ((p.p25 - s.v[108]) > 1e-16) {
        A::sub_from_scalar(p.p25, A::scale(A::add(A::sub_from_scalar(p.p25, s.ad_value(108)), A::sqrt(A::offset(A::mul(A::sub_from_scalar(p.p25, s.ad_value(108)), A::sub_from_scalar(p.p25, s.ad_value(108))), 1e-6))), 0.5))
    } else {
        let assign1490_ad_e1220: A = {
            if ((s.v[108] - p.p25) > 1e-16) {
                A::sub_from_scalar(p.p25, A::div_from_scalar((0.5 * 1e-6), A::add(A::offset(s.ad_value(108), (-p.p25)), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(108), (-p.p25)), A::offset(s.ad_value(108), (-p.p25))), 1e-6)))))
            } else {
                A::sub_from_scalar(p.p25, A::scale(A::offset(A::sub_from_scalar(p.p25, s.ad_value(108)), (((1e-32 + 1e-6)) as f64).sqrt()), 0.5))
            }
        };
        assign1490_ad_e1220
    }
};
        s.store_scale_ad(107, assign1490_ad_e1221, p.p24);

        s.store_scale(140, 107, 1.0000000000000001e-23);

        s.store_offset_scaled_ad(47, A::ln_scaled_input(s.ad_value(107), s.v[10]), (2.0 * s.v[25]), s.v[42]);

        s.store_scaled_sqrt_scaled_input(12, 107, ((2.0 * 1.6021918e-19) * 1.045e-10), 1.0 / (s.v[11]));

        s.b[161] = (p.p30 > 0.0);
        s.v[161] = if s.b[161] { 1.0 } else { 0.0 };

        if s.b[161] {
            s.store_sqrt_mul_ad(55, A::square(s.ad_value(12)), s.ad_value(47));
            s.store_mul_scaled_ad_rhs(56, 54, 0.75, A::powf(s.ad_value(55), 0.6666666666666666));
            s.store_add(47, 47, 56);
            s.store_mul_offset_ad_rhs(12, 12, A::div(A::scale(s.ad_value(56), (2.0 * 0.6666666666666666)), s.ad_value(55)), 1.0);
        }

        s.v[6] = ((s.v[26]) as f64).sqrt();

        s.store_scale(34, 12, s.v[6]);

        s.store_square(36, 34);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_offset_scaled(43, 34, 0.7071067811865475, 1.0);

        s.store_div_from_scalar(44, 1.0, 43);

        s.store_scale(40, 43, 1e-5);

        s.store_scale(50, 47, s.v[26]);

        s.b[162] = (s.v[50] < 460.51701859880916);
        s.v[162] = if s.b[162] { 1.0 } else { 0.0 };

        if s.b[162] {
            s.store_exp_neg_input(52, 50);
        }

        if (!s.b[162]) {
            s.store_div_from_scalar_offset_ad(52, 1e-200, A::mul(A::offset(s.ad_value(50), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(50), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(50), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        s.store_offset_scaled(60, 34, (((((((-1.25)) as f64).exp() + 1.25) - 1.0)) as f64).sqrt(), 1.25);

        s.store_scale_ad(77, A::offset(A::voltage(ctx, nodes, Some(4), Some(5)), (-s.v[28])), p.p17);

        s.store_scale(78, 77, s.v[26]);

        s.b[184] = (((s.v[78]) as f64).abs() <= s.v[40]);
        s.v[184] = if s.b[184] { 1.0 } else { 0.0 };

        if s.b[184] {
            s.store_scaled_square(165, 44, (0.1666666666666667 * 0.7071067811865475));
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(165)), 1.0));
        }

        s.b[185] = (s.v[78] < (-s.v[40]));
        s.v[185] = if s.b[185] { 1.0 } else { 0.0 };

        if ((!s.b[184]) && s.b[185]) {
            s.store_neg(166, 78);
            s.store_scaled_mul(167, 166, 44, 1.25);
            s.store_scaled_sub_ad(174, A::offset(s.ad_value(167), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(167), (-6.0)), A::offset(s.ad_value(167), (-6.0))), 64.0)), 0.5);
            s.store_sub(164, 166, 174);
            s.store_add_ad(169, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::offset(s.ad_value(174), 1.0)));
            s.store_sub_ad_lhs(171, A::scale(s.ad_value(164), 2.0), 36);
            s.store_sub_ad_lhs(173, A::ln(A::mul(s.ad_value(169), s.ad_value(37))), 174);
            s.store_add(186, 169, 171);
            s.store_add_ad(187, A::square(s.ad_value(186)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(171), 0.5, s.ad_value(171)), s.ad_value(169)), s.ad_value(173)));
            s.store_add_ad_rhs(168, 174, A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(186)), s.ad_value(173)), A::add(s.ad_value(187), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(186), s.ad_value(173)), s.ad_value(173)), s.ad_value(187)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), s.ad_value(169))))));
        }

        s.b[188] = (s.v[168] < 230.25850929940458);
        s.v[188] = if s.b[188] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && s.b[185]) && s.b[188]) {
            s.store_exp(175, 168);
        }

        if (((!s.b[184]) && s.b[185]) && (!s.b[188])) {
            s.store_scaled_offset_ad(175, A::mul(A::offset(s.ad_value(168), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(168), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(168), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((!s.b[184]) && s.b[185]) {
            s.store_div_from_scalar(176, 1.0, 175);
            s.store_div_from_scalar_offset_ad(164, 1.0, A::square(s.ad_value(168)), 2.0);
            s.store_sub(164, 166, 168);
            s.store_mul(165, 52, 176);
            s.store_add_scaled_ad_rhs(177, 164, 2.0, A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(175), (-1.0)), s.ad_value(165)), s.ad_value(52))));
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(175), s.ad_value(168)), (-1.0)), s.ad_value(165)), A::mul(s.ad_value(52), A::offset(s.ad_value(168), (-1.0))))));
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(175), s.ad_value(165))));
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul_scaled_lhs(s.ad_value(178), 2.0, s.ad_value(164)));
            s.store_sub_scaled_ad_rhs(79, 168, -1.0, A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_div_from_scalar_offset_scaled_input(163, 1.0, 34, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(179, A::mul_scaled_lhs(s.ad_value(43), 1.25, s.ad_value(163)), (-1.0), 163);
            s.store_mul_ad(182, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(179), s.ad_value(78)), 1.0));
        }

        s.b[189] = ((-s.v[182]) > (-230.25850929940458));
        s.v[189] = if s.b[189] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && (!s.b[185])) && s.b[189]) {
            s.store_exp_neg_input(164, 182);
        }

        if (((!s.b[184]) && (!s.b[185])) && (!s.b[189])) {
            s.store_div_from_scalar_offset_ad(164, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_sub_from_scalar(181, 1.0, 164);
            s.store_sub_ad(180, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(181)))));
            s.store_offset(172, 50, 3.0);
        }

        if ((!s.b[184]) && (!s.b[185])) {
            let assign2080_ad_e1952: A = {
                if ((s.v[172] - s.v[180]) > 1e-16) {
                    A::sub(s.ad_value(172), A::scale(A::add(A::sub(s.ad_value(172), s.ad_value(180)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(172), s.ad_value(180)), A::sub(s.ad_value(172), s.ad_value(180))), 5.0))), 0.5))
                } else {
                    let assign2080_ad_e1951: A = {
                        if ((s.v[180] - s.v[172]) > 1e-16) {
                            A::sub(s.ad_value(172), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(180), s.ad_value(172)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(180), s.ad_value(172)), A::sub(s.ad_value(180), s.ad_value(172))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(172), A::scale(A::offset(A::sub(s.ad_value(172), s.ad_value(180)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign2080_ad_e1951
                }
            };
            s.store_sub_ad(174, assign2080_ad_e1952, A::scale(A::sub(s.ad_value(172), A::sqrt(A::offset(A::square(s.ad_value(172)), 5.0))), 0.5));
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_sub(164, 78, 174);
            s.store_exp_neg_input(165, 174);
            s.store_max_from_scalar_ad(169, 1e-40, A::sub(A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(165), s.ad_value(174)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(174), 1.0))))));
            s.store_sub_from_scalar_ad(170, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(165)));
            s.store_add_scaled_ad_rhs(171, 164, 2.0, A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(165)), s.ad_value(52))));
            s.store_add_ad(173, A::sub(s.ad_value(50), s.ad_value(174)), A::ln(A::div(s.ad_value(169), s.ad_value(36))));
            s.store_add(190, 169, 171);
        }

        s.b[192] = (((s.v[173]) as f64).abs() < 1e-120);
        s.v[192] = if s.b[192] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && (!s.b[185])) && s.b[192]) {
            s.copy_ad(183, 174);
        }

        if (((!s.b[184]) && (!s.b[185])) && (!s.b[192])) {
            s.store_add_ad(191, A::square(s.ad_value(190)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(171), 0.5, s.ad_value(171)), A::mul(s.ad_value(169), s.ad_value(170))), s.ad_value(173)));
        }

        if (((!s.b[184]) && (!s.b[185])) && (!s.b[192])) {
            let assign2190_ad_e2144: A = A::add(s.ad_value(174), A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(190)), s.ad_value(173)), A::add(s.ad_value(191), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(190), s.ad_value(173)), s.ad_value(173)), s.ad_value(191)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), A::mul(s.ad_value(169), s.ad_value(170)))))));
            s.store_ad_value(183, assign2190_ad_e2144);
        }

        s.b[193] = (s.v[183] < 230.25850929940458);
        s.v[193] = if s.b[193] { 1.0 } else { 0.0 };

        if (((!s.b[184]) && (!s.b[185])) && s.b[193]) {
            s.store_exp(175, 183);
            s.store_div_from_scalar(176, 1.0, 175);
            s.store_mul(175, 52, 175);
        }

        s.b[194] = (s.v[183] > (s.v[50] - 230.25850929940458));
        s.v[194] = if s.b[194] { 1.0 } else { 0.0 };

        if ((((!s.b[184]) && (!s.b[185])) && (!s.b[193])) && s.b[194]) {
            s.store_exp_sub(175, 183, 50);
            s.store_div(176, 52, 175);
        }

        if ((((!s.b[184]) && (!s.b[185])) && (!s.b[193])) && (!s.b[194])) {
            s.store_div_from_scalar_offset_ad(175, 1e-100, A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(176, 1e-100, A::mul(A::offset(s.ad_value(183), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(183), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(183), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[184]) && (!s.b[185])) {
            s.store_div_from_scalar_offset_ad(164, 1.0, A::square(s.ad_value(183)), 2.0);
            s.store_sub(164, 78, 183);
            s.store_add_scaled_ad_rhs(177, 164, 2.0, A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(176)), s.ad_value(175)), s.ad_value(52))));
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(176), s.ad_value(183)), (-1.0)), s.ad_value(175)), A::mul(s.ad_value(52), A::offset(s.ad_value(183), 1.0)))));
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(176), s.ad_value(175))));
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul_scaled_lhs(s.ad_value(178), 2.0, s.ad_value(164)));
            s.store_add_ad_rhs(79, 183, A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        s.b[195] = (p.p29 < 1e27);
        s.v[195] = if s.b[195] { 1.0 } else { 0.0 };

        if s.b[195] {
            s.store_sub_scaled_inputs(80, 77, (((-p.p17) * p.p18) * s.v[26]), 79, ((s.v[25]) * ((((-p.p17) * p.p18) * s.v[26]))));
        }

        s.b[217] = (((s.v[80]) as f64).abs() <= s.v[41]);
        s.v[217] = if s.b[217] { 1.0 } else { 0.0 };

        if (s.b[195] && s.b[217]) {
            s.store_scalar(198, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
            s.store_mul_scaled_ad_rhs(81, 80, s.v[46], A::offset(A::mul_scaled_lhs(A::mul(s.ad_value(80), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35], s.ad_value(198)), 1.0));
        }

        s.b[218] = (s.v[80] < (-s.v[41]));
        s.v[218] = if s.b[218] { 1.0 } else { 0.0 };

        if ((s.b[195] && (!s.b[217])) && s.b[218]) {
            s.store_neg(199, 80);
            s.store_scale(200, 199, (1.25 * s.v[46]));
            s.store_scaled_sub_ad(207, A::offset(s.ad_value(200), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(200), (-6.0)), A::offset(s.ad_value(200), (-6.0))), 64.0)), 0.5);
            s.store_sub(197, 199, 207);
            s.store_add_ad(202, A::square(s.ad_value(197)), A::scale(A::offset(s.ad_value(207), 1.0), s.v[38]));
            s.store_offset_scaled(204, 197, 2.0, (-s.v[38]));
            s.store_sub_ad_lhs(206, A::ln_scaled_input(s.ad_value(202), s.v[39]), 207);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[195] && (!s.b[217])) && s.b[218]) {
            s.store_add(219, 202, 204);
            s.store_add_ad(220, A::square(s.ad_value(219)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(204), 0.5, s.ad_value(204)), s.ad_value(202)), s.ad_value(206)));
            s.store_add_ad_rhs(201, 207, A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(219)), s.ad_value(206)), A::add(s.ad_value(220), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(219), s.ad_value(206)), s.ad_value(206)), s.ad_value(220)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), s.ad_value(202))))));
        }

        s.b[221] = (s.v[201] < 230.25850929940458);
        s.v[221] = if s.b[221] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && s.b[218]) && s.b[221]) {
            s.store_exp(208, 201);
        }

        if (((s.b[195] && (!s.b[217])) && s.b[218]) && (!s.b[221])) {
            s.store_scaled_offset_ad(208, A::mul(A::offset(s.ad_value(201), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(201), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(201), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[195] && (!s.b[217])) && s.b[218]) {
            s.store_div_from_scalar(209, 1.0, 208);
            s.store_div_from_scalar_offset_ad(197, 1.0, A::square(s.ad_value(201)), 2.0);
            s.store_sub(197, 199, 201);
            s.store_mul(198, 53, 209);
            s.store_add_scaled_ad_rhs(210, 197, 2.0, A::scale(A::add(A::sub(A::offset(s.ad_value(208), (-1.0)), s.ad_value(198)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(208), s.ad_value(201)), (-1.0)), s.ad_value(198)), A::mul(s.ad_value(53), A::offset(s.ad_value(201), (-1.0)))), s.v[38]));
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(208), s.ad_value(198)), s.v[38]));
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul_scaled_lhs(s.ad_value(211), 2.0, s.ad_value(197)));
            s.store_sub_scaled_ad_rhs(81, 201, -1.0, A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_scalar(196, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
            s.store_mul_offset_ad_lhs(212, A::scale(s.ad_value(196), (s.v[45] * 1.25)), (-1.0), 196);
            s.store_mul_scaled_ad_rhs(215, 80, s.v[46], A::offset(A::mul(s.ad_value(212), s.ad_value(80)), 1.0));
        }

        s.b[222] = ((-s.v[215]) > (-230.25850929940458));
        s.v[222] = if s.b[222] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && s.b[222]) {
            s.store_exp_neg_input(197, 215);
        }

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[222])) {
            s.store_div_from_scalar_offset_ad(197, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_sub_from_scalar(214, 1.0, 197);
            s.store_sub_ad(213, A::offset(s.ad_value(80), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(80), (s.v[38] * 0.25)), s.ad_value(214))), s.v[35]));
            s.store_scalar(205, (s.v[51] + 3.0));
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            let assign2730_ad_e3069: A = {
                if ((s.v[205] - s.v[213]) > 1e-16) {
                    A::sub(s.ad_value(205), A::scale(A::add(A::sub(s.ad_value(205), s.ad_value(213)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(205), s.ad_value(213)), A::sub(s.ad_value(205), s.ad_value(213))), 5.0))), 0.5))
                } else {
                    let assign2730_ad_e3068: A = {
                        if ((s.v[213] - s.v[205]) > 1e-16) {
                            A::sub(s.ad_value(205), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(213), s.ad_value(205)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(213), s.ad_value(205)), A::sub(s.ad_value(213), s.ad_value(205))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(205), A::scale(A::offset(A::sub(s.ad_value(205), s.ad_value(213)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign2730_ad_e3068
                }
            };
            s.store_sub_ad(207, assign2730_ad_e3069, A::scale(A::sub(s.ad_value(205), A::sqrt(A::offset(A::square(s.ad_value(205)), 5.0))), 0.5));
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_sub(197, 80, 207);
            s.store_exp_neg_input(198, 207);
            s.store_max_from_scalar_ad(202, 1e-40, A::sub(A::square(s.ad_value(197)), A::scale(A::sub(A::offset(A::add(s.ad_value(198), s.ad_value(207)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(207), 1.0))), s.v[38])));
            s.store_sub_from_scalar_ad(203, 1.0, A::scale(s.ad_value(198), (0.5 * s.v[38])));
            s.store_add_scaled_ad_rhs(204, 197, 2.0, A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(198)), s.ad_value(53)), s.v[38]));
            s.store_add_ad(206, A::sub_from_scalar(s.v[51], s.ad_value(207)), A::ln_scaled_input(s.ad_value(202), 1.0 / (s.v[38])));
            s.store_add(223, 202, 204);
        }

        s.b[225] = (((s.v[206]) as f64).abs() < 1e-120);
        s.v[225] = if s.b[225] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && s.b[225]) {
            s.copy_ad(216, 207);
        }

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[225])) {
            s.store_add_ad(224, A::square(s.ad_value(223)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(204), 0.5, s.ad_value(204)), A::mul(s.ad_value(202), s.ad_value(203))), s.ad_value(206)));
        }

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[225])) {
            let assign2840_ad_e3281: A = A::add(s.ad_value(207), A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(223)), s.ad_value(206)), A::add(s.ad_value(224), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(223), s.ad_value(206)), s.ad_value(206)), s.ad_value(224)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), A::mul(s.ad_value(202), s.ad_value(203)))))));
            s.store_ad_value(216, assign2840_ad_e3281);
        }

        s.b[226] = (s.v[216] < 230.25850929940458);
        s.v[226] = if s.b[226] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[217])) && (!s.b[218])) && s.b[226]) {
            s.store_exp(208, 216);
            s.store_div_from_scalar(209, 1.0, 208);
            s.store_mul(208, 53, 208);
        }

        s.b[227] = (s.v[216] > (s.v[51] - 230.25850929940458));
        s.v[227] = if s.b[227] { 1.0 } else { 0.0 };

        if ((((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[226])) && s.b[227]) {
            s.store_exp_offset_input(208, 216, (-s.v[51]));
            s.store_div(209, 53, 208);
        }

        if ((((s.b[195] && (!s.b[217])) && (!s.b[218])) && (!s.b[226])) && (!s.b[227])) {
            s.store_div_from_scalar_offset_ad(208, 1e-100, A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(209, 1e-100, A::mul(A::offset(s.ad_value(216), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(216), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(216), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[217])) && (!s.b[218])) {
            s.store_div_from_scalar_offset_ad(197, 1.0, A::square(s.ad_value(216)), 2.0);
            s.store_sub(197, 80, 216);
            s.store_add_scaled_ad_rhs(210, 197, 2.0, A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(209)), s.ad_value(208)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(209), s.ad_value(216)), (-1.0)), s.ad_value(208)), A::mul(s.ad_value(53), A::offset(s.ad_value(216), 1.0))), s.v[38]));
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(209), s.ad_value(208)), s.v[38]));
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul_scaled_lhs(s.ad_value(211), 2.0, s.ad_value(197)));
            s.store_add_ad_rhs(81, 216, A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if s.b[195] {
            s.store_scale(82, 81, (((-p.p17) * p.p18) * s.v[25]));
            s.store_scaled_sub(78, 77, 82, 1.0 / (s.v[25]));
        }

        s.b[249] = (((s.v[78]) as f64).abs() <= s.v[40]);
        s.v[249] = if s.b[249] { 1.0 } else { 0.0 };

        if (s.b[195] && s.b[249]) {
            s.store_scaled_square(230, 44, (0.1666666666666667 * 0.7071067811865475));
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(230)), 1.0));
        }

        s.b[250] = (s.v[78] < (-s.v[40]));
        s.v[250] = if s.b[250] { 1.0 } else { 0.0 };

        if ((s.b[195] && (!s.b[249])) && s.b[250]) {
            s.store_neg(231, 78);
            s.store_scaled_mul(232, 231, 44, 1.25);
            s.store_scaled_sub_ad(239, A::offset(s.ad_value(232), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(232), (-6.0)), A::offset(s.ad_value(232), (-6.0))), 64.0)), 0.5);
            s.store_sub(229, 231, 239);
            s.store_add_ad(234, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::offset(s.ad_value(239), 1.0)));
            s.store_sub_ad_lhs(236, A::scale(s.ad_value(229), 2.0), 36);
            s.store_sub_ad_lhs(238, A::ln(A::mul(s.ad_value(234), s.ad_value(37))), 239);
            s.store_add(251, 234, 236);
            s.store_add_ad(252, A::square(s.ad_value(251)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(236), 0.5, s.ad_value(236)), s.ad_value(234)), s.ad_value(238)));
            s.store_add_ad_rhs(233, 239, A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(251)), s.ad_value(238)), A::add(s.ad_value(252), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(251), s.ad_value(238)), s.ad_value(238)), s.ad_value(252)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), s.ad_value(234))))));
        }

        s.b[253] = (s.v[233] < 230.25850929940458);
        s.v[253] = if s.b[253] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && s.b[250]) && s.b[253]) {
            s.store_exp(240, 233);
        }

        if (((s.b[195] && (!s.b[249])) && s.b[250]) && (!s.b[253])) {
            s.store_scaled_offset_ad(240, A::mul(A::offset(s.ad_value(233), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(233), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(233), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[195] && (!s.b[249])) && s.b[250]) {
            s.store_div_from_scalar(241, 1.0, 240);
            s.store_div_from_scalar_offset_ad(229, 1.0, A::square(s.ad_value(233)), 2.0);
            s.store_sub(229, 231, 233);
            s.store_mul(230, 52, 241);
            s.store_add_scaled_ad_rhs(242, 229, 2.0, A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(240), (-1.0)), s.ad_value(230)), s.ad_value(52))));
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(240), s.ad_value(233)), (-1.0)), s.ad_value(230)), A::mul(s.ad_value(52), A::offset(s.ad_value(233), (-1.0))))));
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(240), s.ad_value(230))));
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul_scaled_lhs(s.ad_value(243), 2.0, s.ad_value(229)));
            s.store_sub_scaled_ad_rhs(79, 233, -1.0, A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_div_from_scalar_offset_scaled_input(228, 1.0, 34, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(244, A::mul_scaled_lhs(s.ad_value(43), 1.25, s.ad_value(228)), (-1.0), 228);
            s.store_mul_ad(247, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(244), s.ad_value(78)), 1.0));
        }

        s.b[254] = ((-s.v[247]) > (-230.25850929940458));
        s.v[254] = if s.b[254] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && s.b[254]) {
            s.store_exp_neg_input(229, 247);
        }

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[254])) {
            s.store_div_from_scalar_offset_ad(229, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_sub_from_scalar(246, 1.0, 229);
            s.store_sub_ad(245, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(246)))));
            s.store_offset(237, 50, 3.0);
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            let assign3380_ad_e4235: A = {
                if ((s.v[237] - s.v[245]) > 1e-16) {
                    A::sub(s.ad_value(237), A::scale(A::add(A::sub(s.ad_value(237), s.ad_value(245)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(237), s.ad_value(245)), A::sub(s.ad_value(237), s.ad_value(245))), 5.0))), 0.5))
                } else {
                    let assign3380_ad_e4234: A = {
                        if ((s.v[245] - s.v[237]) > 1e-16) {
                            A::sub(s.ad_value(237), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(245), s.ad_value(237)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(245), s.ad_value(237)), A::sub(s.ad_value(245), s.ad_value(237))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(237), A::scale(A::offset(A::sub(s.ad_value(237), s.ad_value(245)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign3380_ad_e4234
                }
            };
            s.store_sub_ad(239, assign3380_ad_e4235, A::scale(A::sub(s.ad_value(237), A::sqrt(A::offset(A::square(s.ad_value(237)), 5.0))), 0.5));
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_sub(229, 78, 239);
            s.store_exp_neg_input(230, 239);
            s.store_max_from_scalar_ad(234, 1e-40, A::sub(A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(230), s.ad_value(239)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(239), 1.0))))));
            s.store_sub_from_scalar_ad(235, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(230)));
            s.store_add_scaled_ad_rhs(236, 229, 2.0, A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(230)), s.ad_value(52))));
            s.store_add_ad(238, A::sub(s.ad_value(50), s.ad_value(239)), A::ln(A::div(s.ad_value(234), s.ad_value(36))));
            s.store_add(255, 234, 236);
        }

        s.b[257] = (((s.v[238]) as f64).abs() < 1e-120);
        s.v[257] = if s.b[257] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && s.b[257]) {
            s.copy_ad(248, 239);
        }

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[257])) {
            s.store_add_ad(256, A::square(s.ad_value(255)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(236), 0.5, s.ad_value(236)), A::mul(s.ad_value(234), s.ad_value(235))), s.ad_value(238)));
        }

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[257])) {
            let assign3490_ad_e4447: A = A::add(s.ad_value(239), A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(255)), s.ad_value(238)), A::add(s.ad_value(256), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(255), s.ad_value(238)), s.ad_value(238)), s.ad_value(256)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), A::mul(s.ad_value(234), s.ad_value(235)))))));
            s.store_ad_value(248, assign3490_ad_e4447);
        }

        s.b[258] = (s.v[248] < 230.25850929940458);
        s.v[258] = if s.b[258] { 1.0 } else { 0.0 };

        if (((s.b[195] && (!s.b[249])) && (!s.b[250])) && s.b[258]) {
            s.store_exp(240, 248);
            s.store_div_from_scalar(241, 1.0, 240);
            s.store_mul(240, 52, 240);
        }

        s.b[259] = (s.v[248] > (s.v[50] - 230.25850929940458));
        s.v[259] = if s.b[259] { 1.0 } else { 0.0 };

        if ((((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[258])) && s.b[259]) {
            s.store_exp_sub(240, 248, 50);
            s.store_div(241, 52, 240);
        }

        if ((((s.b[195] && (!s.b[249])) && (!s.b[250])) && (!s.b[258])) && (!s.b[259])) {
            s.store_div_from_scalar_offset_ad(240, 1e-100, A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(241, 1e-100, A::mul(A::offset(s.ad_value(248), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(248), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(248), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[195] && (!s.b[249])) && (!s.b[250])) {
            s.store_div_from_scalar_offset_ad(229, 1.0, A::square(s.ad_value(248)), 2.0);
            s.store_sub(229, 78, 248);
            s.store_add_scaled_ad_rhs(242, 229, 2.0, A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(241)), s.ad_value(240)), s.ad_value(52))));
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(241), s.ad_value(248)), (-1.0)), s.ad_value(240)), A::mul(s.ad_value(52), A::offset(s.ad_value(248), 1.0)))));
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(241), s.ad_value(240))));
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul_scaled_lhs(s.ad_value(243), 2.0, s.ad_value(229)));
            s.store_add_ad_rhs(79, 248, A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if (!s.b[195]) {
            s.store_scalar(82, 0.0);
        }

        s.b[260] = ((s.v[78] <= 0.0) || (p.p21 < 1.0));
        s.v[260] = if s.b[260] { 1.0 } else { 0.0 };

        if (!s.b[260]) {
            s.store_scalar(83, 0.0);
        }

        s.b[261] = (s.v[79] < 230.25850929940458);
        s.v[261] = if s.b[261] { 1.0 } else { 0.0 };

        if ((!s.b[260]) && s.b[261]) {
            s.store_exp(83, 79);
            s.store_div_from_scalar(85, 1.0, 83);
            s.store_mul(83, 52, 83);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[262] = (s.v[79] > (s.v[50] - 230.25850929940458));
        s.v[262] = if s.b[262] { 1.0 } else { 0.0 };

        if (((!s.b[260]) && (!s.b[261])) && s.b[262]) {
            s.store_exp_sub(83, 79, 50);
            s.store_div(85, 52, 83);
        }

        if (((!s.b[260]) && (!s.b[261])) && (!s.b[262])) {
            s.store_div_from_scalar_offset_ad(83, 1e-100, A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(85, 1e-100, A::mul(A::offset(s.ad_value(79), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(79), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(79), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        s.b[263] = (s.v[79] < 1e-5);
        s.v[263] = if s.b[263] { 1.0 } else { 0.0 };

        if ((!s.b[260]) && s.b[263]) {
            s.store_mul_ad(86, A::mul_scaled_lhs(s.ad_value(79), 0.5, s.ad_value(79)), A::sub_from_scalar(1.0, A::mul_scaled_lhs(s.ad_value(79), 0.3333333333333333, A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25)))));
            s.store_sqrt_sub_from_scalar_ad(6, 1.0, A::mul_scaled_lhs(s.ad_value(79), 0.3333333333333333, A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25))));
            s.store_scaled_mul(88, 79, 6, 0.7071067811865475);
        }

        if ((!s.b[260]) && (!s.b[263])) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(79), (-1.0)), 85);
            s.store_sqrt(88, 86);
        }

        s.store_scale_ad(94, A::add(s.ad_value(77), A::voltage(ctx, nodes, Some(6), None)), s.v[26]);

        s.b[281] = (((s.v[94]) as f64).abs() <= s.v[40]);
        s.v[281] = if s.b[281] { 1.0 } else { 0.0 };

        if s.b[281] {
            s.store_div(95, 94, 43);
        }

        s.b[282] = (s.v[94] > s.v[40]);
        s.v[282] = if s.b[282] { 1.0 } else { 0.0 };

        if ((!s.b[281]) && s.b[282]) {
            s.store_div_ad_lhs(276, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
            s.store_mul_ad(277, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(276), s.ad_value(94)), 1.0));
        }

        s.b[283] = (s.v[277] < 460.51701859880916);
        s.v[283] = if s.b[283] { 1.0 } else { 0.0 };

        if (((!s.b[281]) && s.b[282]) && s.b[283]) {
            s.store_exp_neg_input(275, 277);
        }

        if (((!s.b[281]) && s.b[282]) && (!s.b[283])) {
            s.store_div_from_scalar_offset_ad(275, 1e-200, A::mul(A::offset(s.ad_value(277), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(277), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(277), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[281]) && s.b[282]) {
            s.store_sub_from_scalar(278, 1.0, 275);
            s.store_sub_ad(279, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(278)))));
        }

        s.b[284] = (s.v[279] < 460.51701859880916);
        s.v[284] = if s.b[284] { 1.0 } else { 0.0 };

        if (((!s.b[281]) && s.b[282]) && s.b[284]) {
            s.store_exp_neg_input(271, 279);
        }

        if (((!s.b[281]) && s.b[282]) && (!s.b[284])) {
            s.store_div_from_scalar_offset_ad(271, 1e-200, A::mul(A::offset(s.ad_value(279), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(279), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(279), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((!s.b[281]) && s.b[282]) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(271)));
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(94), s.ad_value(279)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(271))));
            s.store_sub_ad(274, A::mul(A::sub(s.ad_value(94), s.ad_value(279)), A::sub(s.ad_value(94), s.ad_value(279))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(279), (-1.0)), s.ad_value(271))));
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul_scaled_lhs(s.ad_value(272), 4.0, s.ad_value(274)));
            s.store_div_ad(280, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
            s.store_add(95, 279, 280);
        }

        if ((!s.b[281]) && (!s.b[282])) {
            s.store_neg(264, 94);
            s.store_scaled_div(265, 264, 43, 1.25);
            s.store_scaled_sub_ad(266, A::offset(s.ad_value(265), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(265), (-6.0)), A::offset(s.ad_value(265), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(267, A::mul(A::sub(s.ad_value(264), s.ad_value(266)), A::sub(s.ad_value(264), s.ad_value(266))), A::mul(s.ad_value(36), A::offset(s.ad_value(266), 1.0)));
            s.store_sub_ad_lhs(268, A::scale(A::sub(s.ad_value(264), s.ad_value(266)), 2.0), 36);
            s.store_sub_ad_lhs(269, A::ln(A::div(s.ad_value(267), s.ad_value(36))), 266);
            s.store_add(285, 267, 268);
            s.store_add_ad(286, A::square(s.ad_value(285)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(268), 0.5, s.ad_value(268)), s.ad_value(267)), s.ad_value(269)));
            s.store_add_ad_rhs(270, 266, A::div(A::mul(A::mul(s.ad_value(267), s.ad_value(285)), s.ad_value(269)), A::add(s.ad_value(286), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(285), s.ad_value(269)), s.ad_value(269)), s.ad_value(286)), s.ad_value(268)), A::sub(A::scale(A::square(s.ad_value(268)), 0.3333333333333333), s.ad_value(267))))));
        }

        s.b[287] = (((s.v[270]) as f64).abs() < 230.25850929940458);
        s.v[287] = if s.b[287] { 1.0 } else { 0.0 };

        if (((!s.b[281]) && (!s.b[282])) && s.b[287]) {
            s.store_exp(271, 270);
        }

        s.b[288] = (s.v[270] < (-230.25850929940458));
        s.v[288] = if s.b[288] { 1.0 } else { 0.0 };

        if ((((!s.b[281]) && (!s.b[282])) && (!s.b[287])) && s.b[288]) {
            s.store_div_from_scalar_offset_ad(271, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((!s.b[281]) && (!s.b[282])) && (!s.b[287])) && (!s.b[288])) {
            s.store_scaled_offset_ad(271, A::mul(A::offset(s.ad_value(270), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(270), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(270), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((!s.b[281]) && (!s.b[282])) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(271)));
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(264), s.ad_value(270)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(271), (-1.0))));
            s.store_add_ad(274, A::mul(A::sub(s.ad_value(264), s.ad_value(270)), A::sub(s.ad_value(264), s.ad_value(270))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(270), 1.0), s.ad_value(271))));
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul_scaled_lhs(s.ad_value(272), 4.0, s.ad_value(274)));
            s.store_div_ad(278, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
            s.store_neg_ad(95, A::add(s.ad_value(270), s.ad_value(278)));
        }

        s.store_scale(96, 95, s.v[25]);

        s.b[289] = (p.p29 < 1e27);
        s.v[289] = if s.b[289] { 1.0 } else { 0.0 };

        if s.b[289] {
            s.store_sub_scaled_inputs(97, 77, (((-p.p17) * p.p18) * s.v[26]), 95, ((s.v[25]) * ((((-p.p17) * p.p18) * s.v[26]))));
        }

        s.b[311] = (((s.v[97]) as f64).abs() <= s.v[41]);
        s.v[311] = if s.b[311] { 1.0 } else { 0.0 };

        if (s.b[289] && s.b[311]) {
            s.store_scalar(292, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
            s.store_mul_scaled_ad_rhs(98, 97, s.v[46], A::offset(A::mul_scaled_lhs(A::mul(s.ad_value(97), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35], s.ad_value(292)), 1.0));
        }

        s.b[312] = (s.v[97] < (-s.v[41]));
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if ((s.b[289] && (!s.b[311])) && s.b[312]) {
            s.store_neg(293, 97);
            s.store_scale(294, 293, (1.25 * s.v[46]));
            s.store_scaled_sub_ad(301, A::offset(s.ad_value(294), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(294), (-6.0)), A::offset(s.ad_value(294), (-6.0))), 64.0)), 0.5);
            s.store_sub(291, 293, 301);
            s.store_add_ad(296, A::square(s.ad_value(291)), A::scale(A::offset(s.ad_value(301), 1.0), s.v[38]));
            s.store_offset_scaled(298, 291, 2.0, (-s.v[38]));
            s.store_sub_ad_lhs(300, A::ln_scaled_input(s.ad_value(296), s.v[39]), 301);
            s.store_add(313, 296, 298);
            s.store_add_ad(314, A::square(s.ad_value(313)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(298), 0.5, s.ad_value(298)), s.ad_value(296)), s.ad_value(300)));
            s.store_add_ad_rhs(295, 301, A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(313)), s.ad_value(300)), A::add(s.ad_value(314), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(313), s.ad_value(300)), s.ad_value(300)), s.ad_value(314)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), s.ad_value(296))))));
        }

        s.b[315] = (s.v[295] < 230.25850929940458);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && s.b[312]) && s.b[315]) {
            s.store_exp(302, 295);
        }

        if (((s.b[289] && (!s.b[311])) && s.b[312]) && (!s.b[315])) {
            s.store_scaled_offset_ad(302, A::mul(A::offset(s.ad_value(295), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(295), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(295), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[289] && (!s.b[311])) && s.b[312]) {
            s.store_div_from_scalar(303, 1.0, 302);
            s.store_div_from_scalar_offset_ad(291, 1.0, A::square(s.ad_value(295)), 2.0);
            s.store_sub(291, 293, 295);
            s.store_mul(292, 53, 303);
            s.store_add_scaled_ad_rhs(304, 291, 2.0, A::scale(A::add(A::sub(A::offset(s.ad_value(302), (-1.0)), s.ad_value(292)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(302), s.ad_value(295)), (-1.0)), s.ad_value(292)), A::mul(s.ad_value(53), A::offset(s.ad_value(295), (-1.0)))), s.v[38]));
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(302), s.ad_value(292)), s.v[38]));
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul_scaled_lhs(s.ad_value(305), 2.0, s.ad_value(291)));
            s.store_sub_scaled_ad_rhs(98, 295, -1.0, A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_scalar(290, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
            s.store_mul_offset_ad_lhs(306, A::scale(s.ad_value(290), (s.v[45] * 1.25)), (-1.0), 290);
            s.store_mul_scaled_ad_rhs(309, 97, s.v[46], A::offset(A::mul(s.ad_value(306), s.ad_value(97)), 1.0));
        }

        s.b[316] = ((-s.v[309]) > (-230.25850929940458));
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && s.b[316]) {
            s.store_exp_neg_input(291, 309);
        }

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[316])) {
            s.store_div_from_scalar_offset_ad(291, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_sub_from_scalar(308, 1.0, 291);
            s.store_sub_ad(307, A::offset(s.ad_value(97), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(97), (s.v[38] * 0.25)), s.ad_value(308))), s.v[35]));
            s.store_scalar(299, (s.v[51] + 3.0));
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            let assign4700_ad_e6331: A = {
                if ((s.v[299] - s.v[307]) > 1e-16) {
                    A::sub(s.ad_value(299), A::scale(A::add(A::sub(s.ad_value(299), s.ad_value(307)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(299), s.ad_value(307)), A::sub(s.ad_value(299), s.ad_value(307))), 5.0))), 0.5))
                } else {
                    let assign4700_ad_e6330: A = {
                        if ((s.v[307] - s.v[299]) > 1e-16) {
                            A::sub(s.ad_value(299), A::div_from_scalar((0.5 * 5.0), A::add(A::sub(s.ad_value(307), s.ad_value(299)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(307), s.ad_value(299)), A::sub(s.ad_value(307), s.ad_value(299))), 5.0)))))
                        } else {
                            A::sub(s.ad_value(299), A::scale(A::offset(A::sub(s.ad_value(299), s.ad_value(307)), (((1e-32 + 5.0)) as f64).sqrt()), 0.5))
                        }
                    };
                    assign4700_ad_e6330
                }
            };
            s.store_sub_ad(301, assign4700_ad_e6331, A::scale(A::sub(s.ad_value(299), A::sqrt(A::offset(A::square(s.ad_value(299)), 5.0))), 0.5));
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_sub(291, 97, 301);
            s.store_exp_neg_input(292, 301);
            s.store_max_from_scalar_ad(296, 1e-40, A::sub(A::square(s.ad_value(291)), A::scale(A::sub(A::offset(A::add(s.ad_value(292), s.ad_value(301)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(301), 1.0))), s.v[38])));
            s.store_sub_from_scalar_ad(297, 1.0, A::scale(s.ad_value(292), (0.5 * s.v[38])));
            s.store_add_scaled_ad_rhs(298, 291, 2.0, A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(292)), s.ad_value(53)), s.v[38]));
            s.store_add_ad(300, A::sub_from_scalar(s.v[51], s.ad_value(301)), A::ln_scaled_input(s.ad_value(296), 1.0 / (s.v[38])));
            s.store_add(317, 296, 298);
        }

        s.b[319] = (((s.v[300]) as f64).abs() < 1e-120);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && s.b[319]) {
            s.copy_ad(310, 301);
        }

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[319])) {
            s.store_add_ad(318, A::square(s.ad_value(317)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(298), 0.5, s.ad_value(298)), A::mul(s.ad_value(296), s.ad_value(297))), s.ad_value(300)));
        }

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[319])) {
            let assign4810_ad_e6543: A = A::add(s.ad_value(301), A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(317)), s.ad_value(300)), A::add(s.ad_value(318), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(317), s.ad_value(300)), s.ad_value(300)), s.ad_value(318)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), A::mul(s.ad_value(296), s.ad_value(297)))))));
            s.store_ad_value(310, assign4810_ad_e6543);
        }

        s.b[320] = (s.v[310] < 230.25850929940458);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[311])) && (!s.b[312])) && s.b[320]) {
            s.store_exp(302, 310);
            s.store_div_from_scalar(303, 1.0, 302);
            s.store_mul(302, 53, 302);
        }

        s.b[321] = (s.v[310] > (s.v[51] - 230.25850929940458));
        s.v[321] = if s.b[321] { 1.0 } else { 0.0 };

        if ((((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[320])) && s.b[321]) {
            s.store_exp_offset_input(302, 310, (-s.v[51]));
            s.store_div(303, 53, 302);
        }

        if ((((s.b[289] && (!s.b[311])) && (!s.b[312])) && (!s.b[320])) && (!s.b[321])) {
            s.store_div_from_scalar_offset_ad(302, 1e-100, A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(303, 1e-100, A::mul(A::offset(s.ad_value(310), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(310), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(310), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[311])) && (!s.b[312])) {
            s.store_div_from_scalar_offset_ad(291, 1.0, A::square(s.ad_value(310)), 2.0);
            s.store_sub(291, 97, 310);
            s.store_add_scaled_ad_rhs(304, 291, 2.0, A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(303)), s.ad_value(302)), s.ad_value(53)), s.v[38]));
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(303), s.ad_value(310)), (-1.0)), s.ad_value(302)), A::mul(s.ad_value(53), A::offset(s.ad_value(310), 1.0))), s.v[38]));
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(303), s.ad_value(302)), s.v[38]));
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul_scaled_lhs(s.ad_value(305), 2.0, s.ad_value(291)));
            s.store_add_ad_rhs(98, 310, A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if s.b[289] {
            s.store_scale(99, 98, (((-p.p17) * p.p18) * s.v[25]));
            s.store_scaled_sub_ad_lhs(94, A::add(s.ad_value(77), A::voltage(ctx, nodes, Some(6), None)), 99, 1.0 / (s.v[25]));
        }

        s.b[339] = (((s.v[94]) as f64).abs() <= s.v[40]);
        s.v[339] = if s.b[339] { 1.0 } else { 0.0 };

        if (s.b[289] && s.b[339]) {
            s.store_div(95, 94, 43);
        }

        s.b[340] = (s.v[94] > s.v[40]);
        s.v[340] = if s.b[340] { 1.0 } else { 0.0 };

        if ((s.b[289] && (!s.b[339])) && s.b[340]) {
            s.store_div_ad_lhs(334, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
            s.store_mul_ad(335, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(334), s.ad_value(94)), 1.0));
        }

        s.b[341] = (s.v[335] < 460.51701859880916);
        s.v[341] = if s.b[341] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[339])) && s.b[340]) && s.b[341]) {
            s.store_exp_neg_input(333, 335);
        }

        if (((s.b[289] && (!s.b[339])) && s.b[340]) && (!s.b[341])) {
            s.store_div_from_scalar_offset_ad(333, 1e-200, A::mul(A::offset(s.ad_value(335), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(335), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(335), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[339])) && s.b[340]) {
            s.store_sub_from_scalar(336, 1.0, 333);
            s.store_sub_ad(337, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(336)))));
        }

        s.b[342] = (s.v[337] < 460.51701859880916);
        s.v[342] = if s.b[342] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        if (((s.b[289] && (!s.b[339])) && s.b[340]) && s.b[342]) {
            s.store_exp_neg_input(329, 337);
        }

        if (((s.b[289] && (!s.b[339])) && s.b[340]) && (!s.b[342])) {
            s.store_div_from_scalar_offset_ad(329, 1e-200, A::mul(A::offset(s.ad_value(337), (-460.51701859880916)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(337), (-460.51701859880916)), 0.5, A::offset(A::scale(A::offset(s.ad_value(337), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[289] && (!s.b[339])) && s.b[340]) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(329)));
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(94), s.ad_value(337)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(329))));
            s.store_sub_ad(332, A::mul(A::sub(s.ad_value(94), s.ad_value(337)), A::sub(s.ad_value(94), s.ad_value(337))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(337), (-1.0)), s.ad_value(329))));
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul_scaled_lhs(s.ad_value(330), 4.0, s.ad_value(332)));
            s.store_div_ad(338, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
            s.store_add(95, 337, 338);
        }

        if ((s.b[289] && (!s.b[339])) && (!s.b[340])) {
            s.store_neg(322, 94);
            s.store_scaled_div(323, 322, 43, 1.25);
            s.store_scaled_sub_ad(324, A::offset(s.ad_value(323), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(323), (-6.0)), A::offset(s.ad_value(323), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(325, A::mul(A::sub(s.ad_value(322), s.ad_value(324)), A::sub(s.ad_value(322), s.ad_value(324))), A::mul(s.ad_value(36), A::offset(s.ad_value(324), 1.0)));
            s.store_sub_ad_lhs(326, A::scale(A::sub(s.ad_value(322), s.ad_value(324)), 2.0), 36);
            s.store_sub_ad_lhs(327, A::ln(A::div(s.ad_value(325), s.ad_value(36))), 324);
            s.store_add(343, 325, 326);
            s.store_add_ad(344, A::square(s.ad_value(343)), A::mul(A::sub(A::mul_scaled_lhs(s.ad_value(326), 0.5, s.ad_value(326)), s.ad_value(325)), s.ad_value(327)));
            s.store_add_ad_rhs(328, 324, A::div(A::mul(A::mul(s.ad_value(325), s.ad_value(343)), s.ad_value(327)), A::add(s.ad_value(344), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(343), s.ad_value(327)), s.ad_value(327)), s.ad_value(344)), s.ad_value(326)), A::sub(A::scale(A::square(s.ad_value(326)), 0.3333333333333333), s.ad_value(325))))));
        }

        s.b[345] = (((s.v[328]) as f64).abs() < 230.25850929940458);
        s.v[345] = if s.b[345] { 1.0 } else { 0.0 };

        if (((s.b[289] && (!s.b[339])) && (!s.b[340])) && s.b[345]) {
            s.store_exp(329, 328);
        }

        s.b[346] = (s.v[328] < (-230.25850929940458));
        s.v[346] = if s.b[346] { 1.0 } else { 0.0 };

        if ((((s.b[289] && (!s.b[339])) && (!s.b[340])) && (!s.b[345])) && s.b[346]) {
            s.store_div_from_scalar_offset_ad(329, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), A::offset(A::mul_scaled_lhs(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.5, A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        if ((((s.b[289] && (!s.b[339])) && (!s.b[340])) && (!s.b[345])) && (!s.b[346])) {
            s.store_scaled_offset_ad(329, A::mul(A::offset(s.ad_value(328), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(328), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(328), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0, 1e100);
        }

        if ((s.b[289] && (!s.b[339])) && (!s.b[340])) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul_scaled_lhs(s.ad_value(36), 0.5, s.ad_value(329)));
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(322), s.ad_value(328)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(329), (-1.0))));
            s.store_add_ad(332, A::mul(A::sub(s.ad_value(322), s.ad_value(328)), A::sub(s.ad_value(322), s.ad_value(328))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(328), 1.0), s.ad_value(329))));
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul_scaled_lhs(s.ad_value(330), 4.0, s.ad_value(332)));
            s.store_div_ad(336, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
            s.store_neg_ad(95, A::add(s.ad_value(328), s.ad_value(336)));
        }

        if s.b[289] {
            s.store_scale(96, 95, s.v[25]);
        }

        if (!s.b[289]) {
            s.store_scalar(99, 0.0);
        }

        s.v[83] = 0.0;

        s.b[347] = (s.v[95] < 230.25850929940458);
        s.v[347] = if s.b[347] { 1.0 } else { 0.0 };

        if s.b[347] {
            s.store_exp(83, 95);
            s.store_div_from_scalar(85, 1.0, 83);
        }

        s.b[348] = (s.v[95] > (s.v[50] - 230.25850929940458));
        s.v[348] = if s.b[348] { 1.0 } else { 0.0 };

        if ((!s.b[347]) && s.b[348]) {
            s.store_exp_sub(83, 50, 95);
            s.store_mul(85, 52, 83);
        }

        if ((!s.b[347]) && (!s.b[348])) {
            s.store_div_from_scalar_offset_ad(85, 1e-100, A::mul(A::offset(s.ad_value(95), (-230.25850929940458)), A::offset(A::mul_scaled_lhs(A::offset(s.ad_value(95), (-230.25850929940458)), 0.5, A::offset(A::scale(A::offset(s.ad_value(95), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        s.b[349] = (s.v[95] < (-s.v[40]));
        s.v[349] = if s.b[349] { 1.0 } else { 0.0 };

        if s.b[349] {
            s.store_offset_add(86, 85, 95, (-1.0));
            s.store_neg_ad(88, A::sqrt(s.ad_value(86)));
        }

        s.b[350] = (((s.v[95]) as f64).abs() <= s.v[40]);
        s.v[350] = if s.b[350] { 1.0 } else { 0.0 };

        if ((!s.b[349]) && s.b[350]) {
            s.store_sub_from_scalar_ad(6, 1.0, A::mul_scaled_lhs(s.ad_value(95), 0.3333333333333333, A::sub_from_scalar(1.0, A::scale(s.ad_value(95), 0.25))));
            s.store_mul_ad_lhs(86, A::mul_scaled_lhs(s.ad_value(95), 0.5, s.ad_value(95)), 6);
            s.store_mul_scaled_ad_rhs(88, 95, 0.7071067811865475, A::sqrt(s.ad_value(6)));
        }

        if ((!s.b[349]) && (!s.b[350])) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(95), (-1.0)), 85);
            s.store_sqrt(88, 86);
        }

        s.store_scaled_mul(91, 88, 34, s.v[25]);

        s.store_scaled_mul_ad(139, A::offset(s.ad_value(140), 1.0), A::offset(s.ad_value(140), 1.0), (1.62 * ((1.0 + (0.37 * s.v[141])) * ((1.0 + (0.37 * s.v[141])) * (s.v[20] * (((s.v[20]) as f64).sqrt() * (s.v[25] * s.v[25])))))));

        let assign5600_ad_e7802: A = {
    if ((s.v[91] - (-s.v[91])) > 1e-16) {
        A::sub(A::scale(A::add(A::sub(s.ad_value(91), A::neg(s.ad_value(91))), A::sqrt(A::add(A::mul(A::sub(s.ad_value(91), A::neg(s.ad_value(91))), A::sub(s.ad_value(91), A::neg(s.ad_value(91)))), s.ad_value(139)))), 0.5), s.ad_value(91))
    } else {
        let assign5600_ad_e7801: A = {
            if (((-s.v[91]) - s.v[91]) > 1e-16) {
                A::sub(A::div(A::scale(s.ad_value(139), 0.5), A::add(A::sub(A::neg(s.ad_value(91)), s.ad_value(91)), A::sqrt(A::add(A::mul(A::sub(A::neg(s.ad_value(91)), s.ad_value(91)), A::sub(A::neg(s.ad_value(91)), s.ad_value(91))), s.ad_value(139))))), s.ad_value(91))
            } else {
                A::sub(A::scale(A::add(A::sub(s.ad_value(91), A::neg(s.ad_value(91))), A::sqrt(A::offset(s.ad_value(139), 1e-32))), 0.5), s.ad_value(91))
            }
        };
        assign5600_ad_e7801
    }
};
        let assign5600_ad_e7874: A = {
    if (((-nv6) - nv6) > 1e-16) {
        let assign5600_ad_e7830: A = A::add(A::voltage(ctx, nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None)), A::sqrt(A::add(A::mul(A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None)), A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None))), s.ad_value(139)))), 0.5));
        assign5600_ad_e7830
    } else {
        let assign5600_ad_e7873: A = {
            if ((nv6 - (-nv6)) > 1e-16) {
                let assign5600_ad_e7858: A = A::div(A::scale(s.ad_value(139), 0.5), A::add(A::sub(A::voltage(ctx, nodes, Some(6), None), A::neg(A::voltage(ctx, nodes, Some(6), None))), A::sqrt(A::add(A::mul(A::sub(A::voltage(ctx, nodes, Some(6), None), A::neg(A::voltage(ctx, nodes, Some(6), None))), A::sub(A::voltage(ctx, nodes, Some(6), None), A::neg(A::voltage(ctx, nodes, Some(6), None)))), s.ad_value(139)))));
                A::add(A::voltage(ctx, nodes, Some(6), None), assign5600_ad_e7858)
            } else {
                A::add(A::voltage(ctx, nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, nodes, Some(6), None)), A::voltage(ctx, nodes, Some(6), None)), A::sqrt(A::offset(s.ad_value(139), 1e-32))), 0.5))
            }
        };
        assign5600_ad_e7873
    }
};
        s.store_add_ad(59, assign5600_ad_e7802, A::mul(s.ad_value(84), assign5600_ad_e7874));

        s.v[58] = s.v[11];

        s.b[351] = (s.v[54] > 0.0);
        s.v[351] = if s.b[351] { 1.0 } else { 0.0 };

        if s.b[351] {
            s.store_div_from_scalar_offset_ad(58, s.v[11], A::mul(s.ad_value(54), A::powf(A::offset(A::square(s.ad_value(59)), s.v[57]), ((-1.0) * 0.1666666666666667))), 1.0);
        }

        s.store_mul_scaled_ad_lhs(3, A::sub(A::sub(s.ad_value(77), s.ad_value(96)), s.ad_value(99)), 58, ((s.v[23] * s.v[24]) * p.p17));

        s.store_scaled_voltage(105, ctx, nodes, Some(6), None, p.p22);

        s.store_scaled_voltage(106, ctx, nodes, Some(3), Some(1), s.v[61]);

    }

    pub(super) fn stamp_transient_equations_block_0(
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let eq0_value: f64 = (nv6 - 0.0);
        stamper.stamp_current_node1_local(
            Some(6),
            None,
            multiplicity * (eq0_value),
            6,
            multiplicity * (1.0),
        );
        let (eq1_e76, eq1_e76_d_n0, eq1_e76_d_n1, eq1_e76_d_n2, eq1_e76_d_n3, eq1_e76_d_n4, eq1_e76_d_n5, eq1_e76_d_n6, eq1_e76_d_b0, eq1_e76_d_b1, eq1_e76_d_b2, eq1_e76_d_b3,) = {
    if (p.p16 != 0.0) {
        let eq1_e74: f64 = ((nv0 - nv3) * s.v[63]);
        let eq1_e74_d_n0: f64 = (s.v[63] + ((nv0 - nv3) * s.dn[63][0]));
        let eq1_e74_d_n1: f64 = ((nv0 - nv3) * s.dn[63][1]);
        let eq1_e74_d_n2: f64 = ((nv0 - nv3) * s.dn[63][2]);
        let eq1_e74_d_n3: f64 = ((-s.v[63]) + ((nv0 - nv3) * s.dn[63][3]));
        let eq1_e74_d_n4: f64 = ((nv0 - nv3) * s.dn[63][4]);
        let eq1_e74_d_n5: f64 = ((nv0 - nv3) * s.dn[63][5]);
        let eq1_e74_d_n6: f64 = ((nv0 - nv3) * s.dn[63][6]);
        let eq1_e74_d_b0: f64 = ((nv0 - nv3) * s.db[63][0]);
        let eq1_e74_d_b1: f64 = ((nv0 - nv3) * s.db[63][1]);
        let eq1_e74_d_b2: f64 = ((nv0 - nv3) * s.db[63][2]);
        let eq1_e74_d_b3: f64 = ((nv0 - nv3) * s.db[63][3]);
        (eq1_e74, eq1_e74_d_n0, eq1_e74_d_n1, eq1_e74_d_n2, eq1_e74_d_n3, eq1_e74_d_n4, eq1_e74_d_n5, eq1_e74_d_n6, eq1_e74_d_b0, eq1_e74_d_b1, eq1_e74_d_b2, eq1_e74_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e76;
        let eq1_node_derivatives: [f64; 7] = [eq1_e76_d_n0, eq1_e76_d_n1, eq1_e76_d_n2, eq1_e76_d_n3, eq1_e76_d_n4, eq1_e76_d_n5, eq1_e76_d_n6];
        let eq1_branch_derivatives: [f64; 4] = [eq1_e76_d_b0, eq1_e76_d_b1, eq1_e76_d_b2, eq1_e76_d_b3];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e82, eq2_e82_d_n0, eq2_e82_d_n1, eq2_e82_d_n2, eq2_e82_d_n3, eq2_e82_d_n4, eq2_e82_d_n5, eq2_e82_d_n6, eq2_e82_d_b0, eq2_e82_d_b1, eq2_e82_d_b2, eq2_e82_d_b3,) = {
    if (p.p16 != 0.0) {
        let eq2_e80: f64 = ((nv3 - nv4) * s.v[65]);
        let eq2_e80_d_n0: f64 = ((nv3 - nv4) * s.dn[65][0]);
        let eq2_e80_d_n1: f64 = ((nv3 - nv4) * s.dn[65][1]);
        let eq2_e80_d_n2: f64 = ((nv3 - nv4) * s.dn[65][2]);
        let eq2_e80_d_n3: f64 = (s.v[65] + ((nv3 - nv4) * s.dn[65][3]));
        let eq2_e80_d_n4: f64 = ((-s.v[65]) + ((nv3 - nv4) * s.dn[65][4]));
        let eq2_e80_d_n5: f64 = ((nv3 - nv4) * s.dn[65][5]);
        let eq2_e80_d_n6: f64 = ((nv3 - nv4) * s.dn[65][6]);
        let eq2_e80_d_b0: f64 = ((nv3 - nv4) * s.db[65][0]);
        let eq2_e80_d_b1: f64 = ((nv3 - nv4) * s.db[65][1]);
        let eq2_e80_d_b2: f64 = ((nv3 - nv4) * s.db[65][2]);
        let eq2_e80_d_b3: f64 = ((nv3 - nv4) * s.db[65][3]);
        (eq2_e80, eq2_e80_d_n0, eq2_e80_d_n1, eq2_e80_d_n2, eq2_e80_d_n3, eq2_e80_d_n4, eq2_e80_d_n5, eq2_e80_d_n6, eq2_e80_d_b0, eq2_e80_d_b1, eq2_e80_d_b2, eq2_e80_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e82;
        let eq2_node_derivatives: [f64; 7] = [eq2_e82_d_n0, eq2_e82_d_n1, eq2_e82_d_n2, eq2_e82_d_n3, eq2_e82_d_n4, eq2_e82_d_n5, eq2_e82_d_n6];
        let eq2_branch_derivatives: [f64; 4] = [eq2_e82_d_b0, eq2_e82_d_b1, eq2_e82_d_b2, eq2_e82_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(4),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e90, eq3_e90_d_n0, eq3_e90_d_n1, eq3_e90_d_n2, eq3_e90_d_n3, eq3_e90_d_n4, eq3_e90_d_n5, eq3_e90_d_n6, eq3_e90_d_b0, eq3_e90_d_b1, eq3_e90_d_b2, eq3_e90_d_b3,) = {
    if (p.p16 != 0.0) {
        let eq3_e87: f64 = (s.v[67] + s.v[104]);
        let eq3_e87_d_n0: f64 = (s.dn[67][0] + s.dn[104][0]);
        let eq3_e87_d_n1: f64 = (s.dn[67][1] + s.dn[104][1]);
        let eq3_e87_d_n2: f64 = (s.dn[67][2] + s.dn[104][2]);
        let eq3_e87_d_n3: f64 = (s.dn[67][3] + s.dn[104][3]);
        let eq3_e87_d_n4: f64 = (s.dn[67][4] + s.dn[104][4]);
        let eq3_e87_d_n5: f64 = (s.dn[67][5] + s.dn[104][5]);
        let eq3_e87_d_n6: f64 = (s.dn[67][6] + s.dn[104][6]);
        let eq3_e87_d_b0: f64 = (s.db[67][0] + s.db[104][0]);
        let eq3_e87_d_b1: f64 = (s.db[67][1] + s.db[104][1]);
        let eq3_e87_d_b2: f64 = (s.db[67][2] + s.db[104][2]);
        let eq3_e87_d_b3: f64 = (s.db[67][3] + s.db[104][3]);
        let eq3_e88: f64 = ((nv5 - nv1) * eq3_e87);
        let eq3_e88_d_n0: f64 = ((nv5 - nv1) * eq3_e87_d_n0);
        let eq3_e88_d_n1: f64 = ((-eq3_e87) + ((nv5 - nv1) * eq3_e87_d_n1));
        let eq3_e88_d_n2: f64 = ((nv5 - nv1) * eq3_e87_d_n2);
        let eq3_e88_d_n3: f64 = ((nv5 - nv1) * eq3_e87_d_n3);
        let eq3_e88_d_n4: f64 = ((nv5 - nv1) * eq3_e87_d_n4);
        let eq3_e88_d_n5: f64 = (eq3_e87 + ((nv5 - nv1) * eq3_e87_d_n5));
        let eq3_e88_d_n6: f64 = ((nv5 - nv1) * eq3_e87_d_n6);
        let eq3_e88_d_b0: f64 = ((nv5 - nv1) * eq3_e87_d_b0);
        let eq3_e88_d_b1: f64 = ((nv5 - nv1) * eq3_e87_d_b1);
        let eq3_e88_d_b2: f64 = ((nv5 - nv1) * eq3_e87_d_b2);
        let eq3_e88_d_b3: f64 = ((nv5 - nv1) * eq3_e87_d_b3);
        (eq3_e88, eq3_e88_d_n0, eq3_e88_d_n1, eq3_e88_d_n2, eq3_e88_d_n3, eq3_e88_d_n4, eq3_e88_d_n5, eq3_e88_d_n6, eq3_e88_d_b0, eq3_e88_d_b1, eq3_e88_d_b2, eq3_e88_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e90;
        let eq3_node_derivatives: [f64; 7] = [eq3_e90_d_n0, eq3_e90_d_n1, eq3_e90_d_n2, eq3_e90_d_n3, eq3_e90_d_n4, eq3_e90_d_n5, eq3_e90_d_n6];
        let eq3_branch_derivatives: [f64; 4] = [eq3_e90_d_b0, eq3_e90_d_b1, eq3_e90_d_b2, eq3_e90_d_b3];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(1),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e96, eq4_e96_d_n0, eq4_e96_d_n1, eq4_e96_d_n2, eq4_e96_d_n3, eq4_e96_d_n4, eq4_e96_d_n5, eq4_e96_d_n6, eq4_e96_d_b0, eq4_e96_d_b1, eq4_e96_d_b2, eq4_e96_d_b3,) = {
    if (p.p16 != 0.0) {
        let eq4_e94: f64 = ((nv1 - nv2) * s.v[69]);
        let eq4_e94_d_n0: f64 = ((nv1 - nv2) * s.dn[69][0]);
        let eq4_e94_d_n1: f64 = (s.v[69] + ((nv1 - nv2) * s.dn[69][1]));
        let eq4_e94_d_n2: f64 = ((-s.v[69]) + ((nv1 - nv2) * s.dn[69][2]));
        let eq4_e94_d_n3: f64 = ((nv1 - nv2) * s.dn[69][3]);
        let eq4_e94_d_n4: f64 = ((nv1 - nv2) * s.dn[69][4]);
        let eq4_e94_d_n5: f64 = ((nv1 - nv2) * s.dn[69][5]);
        let eq4_e94_d_n6: f64 = ((nv1 - nv2) * s.dn[69][6]);
        let eq4_e94_d_b0: f64 = ((nv1 - nv2) * s.db[69][0]);
        let eq4_e94_d_b1: f64 = ((nv1 - nv2) * s.db[69][1]);
        let eq4_e94_d_b2: f64 = ((nv1 - nv2) * s.db[69][2]);
        let eq4_e94_d_b3: f64 = ((nv1 - nv2) * s.db[69][3]);
        (eq4_e94, eq4_e94_d_n0, eq4_e94_d_n1, eq4_e94_d_n2, eq4_e94_d_n3, eq4_e94_d_n4, eq4_e94_d_n5, eq4_e94_d_n6, eq4_e94_d_b0, eq4_e94_d_b1, eq4_e94_d_b2, eq4_e94_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e96;
        let eq4_node_derivatives: [f64; 7] = [eq4_e96_d_n0, eq4_e96_d_n1, eq4_e96_d_n2, eq4_e96_d_n3, eq4_e96_d_n4, eq4_e96_d_n5, eq4_e96_d_n6];
        let eq4_branch_derivatives: [f64; 4] = [eq4_e96_d_b0, eq4_e96_d_b1, eq4_e96_d_b2, eq4_e96_d_b3];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e101,) = {
    if (p.p16 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e101;
        stamper.stamp_potential_const_local(
            0,
            eq5_value,
        );
        let (eq6_e106,) = {
    if (p.p16 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e106;
        stamper.stamp_potential_const_local(
            1,
            eq6_value,
        );
        let (eq7_e111,) = {
    if (p.p16 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e111;
        stamper.stamp_potential_const_local(
            2,
            eq7_value,
        );
        let (eq8_e116,) = {
    if (p.p16 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e116;
        stamper.stamp_potential_const_local(
            3,
            eq8_value,
        );
        let eq9_e119: f64 = (p.p17 * s.v[4]);
        let eq9_e119_d_n0: f64 = (p.p17 * s.dn[4][0]);
        let eq9_e119_d_n1: f64 = (p.p17 * s.dn[4][1]);
        let eq9_e119_d_n2: f64 = (p.p17 * s.dn[4][2]);
        let eq9_e119_d_n3: f64 = (p.p17 * s.dn[4][3]);
        let eq9_e119_d_n4: f64 = (p.p17 * s.dn[4][4]);
        let eq9_e119_d_n5: f64 = (p.p17 * s.dn[4][5]);
        let eq9_e119_d_n6: f64 = (p.p17 * s.dn[4][6]);
        let eq9_e119_d_b0: f64 = (p.p17 * s.db[4][0]);
        let eq9_e119_d_b1: f64 = (p.p17 * s.db[4][1]);
        let eq9_e119_d_b2: f64 = (p.p17 * s.db[4][2]);
        let eq9_e119_d_b3: f64 = (p.p17 * s.db[4][3]);
        let eq9_value: f64 = eq9_e119;
        let eq9_node_derivatives: [f64; 7] = [eq9_e119_d_n0, eq9_e119_d_n1, eq9_e119_d_n2, eq9_e119_d_n3, eq9_e119_d_n4, eq9_e119_d_n5, eq9_e119_d_n6];
        let eq9_branch_derivatives: [f64; 4] = [eq9_e119_d_b0, eq9_e119_d_b1, eq9_e119_d_b2, eq9_e119_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(5),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e122: f64 = (p.p17 * s.v[5]);
        let eq10_e122_d_n0: f64 = (p.p17 * s.dn[5][0]);
        let eq10_e122_d_n1: f64 = (p.p17 * s.dn[5][1]);
        let eq10_e122_d_n2: f64 = (p.p17 * s.dn[5][2]);
        let eq10_e122_d_n3: f64 = (p.p17 * s.dn[5][3]);
        let eq10_e122_d_n4: f64 = (p.p17 * s.dn[5][4]);
        let eq10_e122_d_n5: f64 = (p.p17 * s.dn[5][5]);
        let eq10_e122_d_n6: f64 = (p.p17 * s.dn[5][6]);
        let eq10_e122_d_b0: f64 = (p.p17 * s.db[5][0]);
        let eq10_e122_d_b1: f64 = (p.p17 * s.db[5][1]);
        let eq10_e122_d_b2: f64 = (p.p17 * s.db[5][2]);
        let eq10_e122_d_b3: f64 = (p.p17 * s.db[5][3]);
        let eq10_value: f64 = eq10_e122;
        let eq10_node_derivatives: [f64; 7] = [eq10_e122_d_n0, eq10_e122_d_n1, eq10_e122_d_n2, eq10_e122_d_n3, eq10_e122_d_n4, eq10_e122_d_n5, eq10_e122_d_n6];
        let eq10_branch_derivatives: [f64; 4] = [eq10_e122_d_b0, eq10_e122_d_b1, eq10_e122_d_b2, eq10_e122_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(1),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e124: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[3]);
        let eq11_e124_d_n0: f64 = (s.dn[3][0] * ddt_scale);
        let eq11_e124_d_n1: f64 = (s.dn[3][1] * ddt_scale);
        let eq11_e124_d_n2: f64 = (s.dn[3][2] * ddt_scale);
        let eq11_e124_d_n3: f64 = (s.dn[3][3] * ddt_scale);
        let eq11_e124_d_n4: f64 = (s.dn[3][4] * ddt_scale);
        let eq11_e124_d_n5: f64 = (s.dn[3][5] * ddt_scale);
        let eq11_e124_d_n6: f64 = (s.dn[3][6] * ddt_scale);
        let eq11_e124_d_b0: f64 = (s.db[3][0] * ddt_scale);
        let eq11_e124_d_b1: f64 = (s.db[3][1] * ddt_scale);
        let eq11_e124_d_b2: f64 = (s.db[3][2] * ddt_scale);
        let eq11_e124_d_b3: f64 = (s.db[3][3] * ddt_scale);
        let eq11_value: f64 = eq11_e124;
        let eq11_node_derivatives: [f64; 7] = [eq11_e124_d_n0, eq11_e124_d_n1, eq11_e124_d_n2, eq11_e124_d_n3, eq11_e124_d_n4, eq11_e124_d_n5, eq11_e124_d_n6];
        let eq11_branch_derivatives: [f64; 4] = [eq11_e124_d_b0, eq11_e124_d_b1, eq11_e124_d_b2, eq11_e124_d_b3];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(5),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e126: f64 = (-s.v[92]);
        let eq12_e126_d_n0: f64 = (-s.dn[92][0]);
        let eq12_e126_d_n1: f64 = (-s.dn[92][1]);
        let eq12_e126_d_n2: f64 = (-s.dn[92][2]);
        let eq12_e126_d_n3: f64 = (-s.dn[92][3]);
        let eq12_e126_d_n4: f64 = (-s.dn[92][4]);
        let eq12_e126_d_n5: f64 = (-s.dn[92][5]);
        let eq12_e126_d_n6: f64 = (-s.dn[92][6]);
        let eq12_e126_d_b0: f64 = (-s.db[92][0]);
        let eq12_e126_d_b1: f64 = (-s.db[92][1]);
        let eq12_e126_d_b2: f64 = (-s.db[92][2]);
        let eq12_e126_d_b3: f64 = (-s.db[92][3]);
        let eq12_value: f64 = eq12_e126;
        let eq12_node_derivatives: [f64; 7] = [eq12_e126_d_n0, eq12_e126_d_n1, eq12_e126_d_n2, eq12_e126_d_n3, eq12_e126_d_n4, eq12_e126_d_n5, eq12_e126_d_n6];
        let eq12_branch_derivatives: [f64; 4] = [eq12_e126_d_b0, eq12_e126_d_b1, eq12_e126_d_b2, eq12_e126_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[105]);
        let eq13_e128_d_n0: f64 = (s.dn[105][0] * ddt_scale);
        let eq13_e128_d_n1: f64 = (s.dn[105][1] * ddt_scale);
        let eq13_e128_d_n2: f64 = (s.dn[105][2] * ddt_scale);
        let eq13_e128_d_n3: f64 = (s.dn[105][3] * ddt_scale);
        let eq13_e128_d_n4: f64 = (s.dn[105][4] * ddt_scale);
        let eq13_e128_d_n5: f64 = (s.dn[105][5] * ddt_scale);
        let eq13_e128_d_n6: f64 = (s.dn[105][6] * ddt_scale);
        let eq13_e128_d_b0: f64 = (s.db[105][0] * ddt_scale);
        let eq13_e128_d_b1: f64 = (s.db[105][1] * ddt_scale);
        let eq13_e128_d_b2: f64 = (s.db[105][2] * ddt_scale);
        let eq13_e128_d_b3: f64 = (s.db[105][3] * ddt_scale);
        let eq13_value: f64 = eq13_e128;
        let eq13_node_derivatives: [f64; 7] = [eq13_e128_d_n0, eq13_e128_d_n1, eq13_e128_d_n2, eq13_e128_d_n3, eq13_e128_d_n4, eq13_e128_d_n5, eq13_e128_d_n6];
        let eq13_branch_derivatives: [f64; 4] = [eq13_e128_d_b0, eq13_e128_d_b1, eq13_e128_d_b2, eq13_e128_d_b3];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e130: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[106]);
        let eq14_e130_d_n0: f64 = (s.dn[106][0] * ddt_scale);
        let eq14_e130_d_n1: f64 = (s.dn[106][1] * ddt_scale);
        let eq14_e130_d_n2: f64 = (s.dn[106][2] * ddt_scale);
        let eq14_e130_d_n3: f64 = (s.dn[106][3] * ddt_scale);
        let eq14_e130_d_n4: f64 = (s.dn[106][4] * ddt_scale);
        let eq14_e130_d_n5: f64 = (s.dn[106][5] * ddt_scale);
        let eq14_e130_d_n6: f64 = (s.dn[106][6] * ddt_scale);
        let eq14_e130_d_b0: f64 = (s.db[106][0] * ddt_scale);
        let eq14_e130_d_b1: f64 = (s.db[106][1] * ddt_scale);
        let eq14_e130_d_b2: f64 = (s.db[106][2] * ddt_scale);
        let eq14_e130_d_b3: f64 = (s.db[106][3] * ddt_scale);
        let eq14_value: f64 = eq14_e130;
        let eq14_node_derivatives: [f64; 7] = [eq14_e130_d_n0, eq14_e130_d_n1, eq14_e130_d_n2, eq14_e130_d_n3, eq14_e130_d_n4, eq14_e130_d_n5, eq14_e130_d_n6];
        let eq14_branch_derivatives: [f64; 4] = [eq14_e130_d_b0, eq14_e130_d_b1, eq14_e130_d_b2, eq14_e130_d_b3];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(1),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e141,) = {
    if (p.p49 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e141;
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (eq15_value),
        );
        let (eq16_e152,) = {
    if (p.p49 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e152;
        stamper.stamp_current_const_local(
            Some(4),
            Some(1),
            multiplicity * (eq16_value),
        );
        let (eq17_e158,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e158;
        stamper.stamp_current_const_local(
            Some(0),
            Some(3),
            multiplicity * (eq17_value),
        );
        let (eq18_e164,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e164;
        stamper.stamp_current_const_local(
            Some(3),
            Some(4),
            multiplicity * (eq18_value),
        );
        let (eq19_e170,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e170;
        stamper.stamp_current_const_local(
            Some(1),
            Some(2),
            multiplicity * (eq19_value),
        );
        let (eq20_e176,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e176;
        stamper.stamp_current_const_local(
            Some(5),
            Some(1),
            multiplicity * (eq20_value),
        );
        let (eq21_e182,) = {
    if (p.p16 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e182;
        stamper.stamp_current_const_local(
            Some(5),
            Some(1),
            multiplicity * (eq21_value),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq11_e124_q: f64 = s.v[3];
        let eq11_reactive_node_derivatives: [f64; 7] = [s.dn[3][0], s.dn[3][1], s.dn[3][2], s.dn[3][3], s.dn[3][4], s.dn[3][5], s.dn[3][6]];
        let eq11_reactive_branch_derivatives: [f64; 4] = [s.db[3][0], s.db[3][1], s.db[3][2], s.db[3][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e128_q: f64 = s.v[105];
        let eq13_reactive_node_derivatives: [f64; 7] = [s.dn[105][0], s.dn[105][1], s.dn[105][2], s.dn[105][3], s.dn[105][4], s.dn[105][5], s.dn[105][6]];
        let eq13_reactive_branch_derivatives: [f64; 4] = [s.db[105][0], s.db[105][1], s.db[105][2], s.db[105][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e130_q: f64 = s.v[106];
        let eq14_reactive_node_derivatives: [f64; 7] = [s.dn[106][0], s.dn[106][1], s.dn[106][2], s.dn[106][3], s.dn[106][4], s.dn[106][5], s.dn[106][6]];
        let eq14_reactive_branch_derivatives: [f64; 4] = [s.db[106][0], s.db[106][1], s.db[106][2], s.db[106][3]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[1]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
