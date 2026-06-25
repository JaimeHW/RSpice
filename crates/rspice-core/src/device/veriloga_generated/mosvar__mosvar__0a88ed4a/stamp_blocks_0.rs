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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        s.v[11] = ((3.453e-11 * (p.p20 / 3.9)) / p.p19);

        s.v[12] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p24)) as f64).sqrt() / s.v[11]);

        s.v[13] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p29)) as f64).sqrt() / s.v[11]);

        s.v[109] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p54)) as f64).sqrt() / s.v[11]);

        s.v[144] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[144] != 0.0) {
            s.store_scalar(54, (((0.4 * 5.951993) * p.p30) * ((s.v[11]) as f64).powf(0.6666666666666666)));
        }

        s.v[145] = if (p.p17 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[144] != 0.0) && (s.v[145] != 0.0)) {
            s.store_scale(54, 54, (7.448711 / 5.951993));
        }

        if (!(s.v[144] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        s.v[146] = if (p.p17 < 0.0) { 1.0 } else { 0.0 };

        if (s.v[146] != 0.0) {
            s.store_scalar(84, (0.3333333333333333 * p.p48));
        }

        if (!(s.v[146] != 0.0)) {
            s.store_scalar(84, (0.5 * p.p48));
        }

        s.v[141] = (p.p19 / 1e-9);

        s.v[16] = (if (p.p11 > (-273.0)) { p.p11 } else { (-273.0) });

        s.v[17] = (273.15 + s.v[16]);

        s.v[142] = ((ctx.temperature() + p.p3) - 273.15);

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

        s.v[157] = if (s.v[51] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (s.v[157] != 0.0) {
            s.store_scalar(53, (((-s.v[51])) as f64).exp());
        }

        if (!(s.v[157] != 0.0)) {
            s.store_scalar(53, (1e-200 / (1.0 + ((s.v[51] - 460.51701859880916) * (1.0 + ((0.5 * (s.v[51] - 460.51701859880916)) * (1.0 + ((s.v[51] - 460.51701859880916) * 0.3333333333333333))))))));
        }

        s.v[61] = (2.0 * ((p.p35 * s.v[22]) + (p.p34 * s.v[21])));

        if (p.p16 != 0.0) {
            s.store_scalar(62, ((s.v[29] * s.v[22]) / ((3.0 + ((p.p2 - 1.0) * 9.0)) * s.v[21])));
        }

        if (p.p16 != 0.0) {
            s.store_scalar(64, (s.v[30] / (s.v[22] * s.v[21])));
        }

        if (p.p16 != 0.0) {
            s.store_scalar(68, (s.v[31] / (2.0 * (s.v[22] + p.p33))));
        }

        if (p.p16 != 0.0) {
            s.store_scalar(66, ((s.v[32] * s.v[21]) / (12.0 * (s.v[22] + p.p33))));
        }

        if (p.p16 != 0.0) {
            s.store_ad(62, &{
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
            s.store_ad(64, &{
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
            s.store_ad(68, &{
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
            s.store_ad(66, &{
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
        }

        if (p.p16 != 0.0) {
            s.store_div_from_scalar(65, 1.0, 64);
        }

        if (p.p16 != 0.0) {
            s.store_div_from_scalar(69, 1.0, 68);
        }

        if (p.p16 != 0.0) {
            s.store_div_from_scalar(67, 1.0, 66);
        }

        if (p.p16 != 0.0) {
            s.store_scale(70, 33, (12.0 * (s.v[22] * 1.0 / (s.v[21]))));
        }

        if (!(p.p16 != 0.0)) {
            s.store_scalar(63, 0.0);
        }

        if (!(p.p16 != 0.0)) {
            s.store_scalar(65, 0.0);
        }

        if (!(p.p16 != 0.0)) {
            s.store_scalar(69, 0.0);
        }

        if (!(p.p16 != 0.0)) {
            s.store_scalar(67, 0.0);
        }

        if (!(p.p16 != 0.0)) {
            s.store_scalar(70, 0.0);
        }

        s.store_scale(72, 63, s.v[71]);

        s.store_scale(73, 65, s.v[71]);

        s.store_scale(74, 69, s.v[71]);

        s.store_scale(75, 67, s.v[71]);

        s.v[158] = if (p.p66 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[158] != 0.0) {
            s.store_scalar(76, 0.0);
        }

        if (!(s.v[158] != 0.0)) {
            s.store_scale(76, 70, s.v[71]);
        }

        s.v[127] = 0.0;

        s.v[128] = 0.0;

        if (p.p49 != 0.0) {
            s.store_scalar(125, (((p.p55 * s.v[24]) * s.v[23]) * 1000000000000.0));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(126, ((((2.0 * p.p56) * p.p53) * s.v[24]) * 1000000000000.0));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(137, (((p.p60 * s.v[24]) * s.v[23]) * 1000000000000.0));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(138, ((((2.0 * p.p61) * p.p53) * s.v[24]) * 1000000000000.0));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(119, ((s.v[19]) as f64).powf(p.p52));
        }

        if (p.p49 != 0.0) {
            s.store_mul(125, 125, 119);
        }

        if (p.p49 != 0.0) {
            s.store_mul(126, 126, 119);
        }

        if (p.p49 != 0.0) {
            s.store_mul(137, 137, 119);
        }

        if (p.p49 != 0.0) {
            s.store_mul(138, 138, 119);
        }

        if (p.p49 != 0.0) {
            s.store_scalar(124, (1.0 / p.p50));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(131, (1.0 / p.p51));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(9, (((4.0 * 0.3333333333333333) * (((((2.0 * 1.6021918e-19) * 9.1093826e-31) * p.p50)) as f64).sqrt()) / 1.05457168e-34));
        }

        if (p.p49 != 0.0) {
            s.store_scale(122, 9, p.p19);
        }

        if (p.p49 != 0.0) {
            s.copy_ad(123, 122);
        }

        if (p.p49 != 0.0) {
            s.store_scalar(9, (((4.0 * 0.3333333333333333) * (((((2.0 * 1.6021918e-19) * 9.1093826e-31) * p.p51)) as f64).sqrt()) / 1.05457168e-34));
        }

        if (p.p49 != 0.0) {
            s.store_scale(132, 9, p.p19);
        }

        if (p.p49 != 0.0) {
            s.copy_ad(133, 132);
        }

        s.v[159] = if (p.p59 < 0.0) { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && (s.v[159] != 0.0)) {
            s.store_scalar(120, (((-0.495) * p.p58) / p.p59));
        }

        if ((p.p49 != 0.0) && (!(s.v[159] != 0.0))) {
            s.store_scalar(120, 0.0);
        }

        s.v[160] = if (p.p64 < 0.0) { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && (s.v[160] != 0.0)) {
            s.store_scalar(130, (((-0.495) * p.p63) / p.p64));
        }

        if ((p.p49 != 0.0) && (!(s.v[160] != 0.0))) {
            s.store_scalar(130, 0.0);
        }

        if (p.p49 != 0.0) {
            s.store_scalar(93, (0.5 * ((p.p17 * s.v[47]) + s.v[42])));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(134, (0.5 * ((p.p17 * s.v[135]) + s.v[42])));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(121, (p.p57 * s.v[25]));
        }

        if (p.p49 != 0.0) {
            s.store_scalar(129, (p.p62 * s.v[25]));
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(125, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(126, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(137, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(138, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(121, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(129, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(120, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(130, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(124, 0.1);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(131, 0.1);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(122, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(123, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(132, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(133, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(93, 0.0);
        }

        if (!(p.p49 != 0.0)) {
            s.store_scalar(134, 0.0);
        }

        let assign1480_ad_e1156: A = {
    if ((p.p17 * ((nv4 - nv5) - p.p27)) > 1e-16) {
        A::scale(A::add(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::sqrt(A::offset(A::mul(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17)), p.p28))), 0.5)
    } else {
        let assign1480_ad_e1155: A = {
            if ((-(p.p17 * ((nv4 - nv5) - p.p27))) > 1e-16) {
                A::div_from_scalar((0.5 * p.p28), A::add(A::neg(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17)), A::sqrt(A::offset(A::mul(A::neg(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17)), A::neg(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17))), p.p28))))
            } else {
                A::scale(A::offset(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17), (((1e-32 + p.p28)) as f64).sqrt()), 0.5)
            }
        };
        assign1480_ad_e1155
    }
};
        s.store_offset_ad(108, A::scale(assign1480_ad_e1156, p.p26), 1.0);

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

        s.store_offset_ad(47, A::scale(A::ln(A::scale(s.ad_value(107), s.v[10])), (2.0 * s.v[25])), s.v[42]);

        s.store_scale_ad(12, A::sqrt(A::scale(s.ad_value(107), ((2.0 * 1.6021918e-19) * 1.045e-10))), 1.0 / (s.v[11]));

        s.v[161] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[161] != 0.0) {
            s.store_sqrt_ad(55, A::mul(A::square(s.ad_value(12)), s.ad_value(47)));
        }

        if (s.v[161] != 0.0) {
            s.store_mul_ad(56, A::scale(s.ad_value(54), 0.75), A::powf(s.ad_value(55), 0.6666666666666666));
        }

        if (s.v[161] != 0.0) {
            s.store_add(47, 47, 56);
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
        if (s.v[161] != 0.0) {
            s.store_mul_ad_rhs(12, 12, A::offset(A::div(A::scale(s.ad_value(56), (2.0 * 0.6666666666666666)), s.ad_value(55)), 1.0));
        }

        s.v[6] = ((s.v[26]) as f64).sqrt();

        s.store_scale(34, 12, s.v[6]);

        s.store_square(36, 34);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_offset_scaled(43, 34, 0.7071067811865475, 1.0);

        s.store_div_from_scalar(44, 1.0, 43);

        s.store_scale(40, 43, 1e-5);

        s.store_scale(50, 47, s.v[26]);

        s.v[162] = if (s.v[50] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (s.v[162] != 0.0) {
            s.store_exp_ad(52, A::neg(s.ad_value(50)));
        }

        if (!(s.v[162] != 0.0)) {
            s.store_div_from_scalar_ad(52, 1e-200, A::offset(A::mul(A::offset(s.ad_value(50), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(50), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(50), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        s.store_offset_scaled(60, 34, (((((((-1.25)) as f64).exp() + 1.25) - 1.0)) as f64).sqrt(), 1.25);

        s.v[116] = (1.25 + (s.v[110] * (((((((-1.25)) as f64).exp() + 1.25) - 1.0)) as f64).sqrt()));

        s.store_scale_ad(77, A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-s.v[28])), p.p17);

        s.store_scale(78, 77, s.v[26]);

        s.v[184] = if (((s.v[78]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if (s.v[184] != 0.0) {
            s.store_scale_ad(165, A::square(s.ad_value(44)), (0.1666666666666667 * 0.7071067811865475));
        }

        if (s.v[184] != 0.0) {
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(165)), 1.0));
        }

        s.v[185] = if (s.v[78] < (-s.v[40])) { 1.0 } else { 0.0 };

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_neg(166, 78);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_mul_ad_lhs(167, A::scale(s.ad_value(166), 1.25), 44);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_scale_ad(174, A::sub(A::offset(s.ad_value(167), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(167), (-6.0)), A::offset(s.ad_value(167), (-6.0))), 64.0))), 0.5);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub(164, 166, 174);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad(169, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::offset(s.ad_value(174), 1.0)));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad_lhs(171, A::scale(s.ad_value(164), 2.0), 36);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad_lhs(173, A::ln(A::mul(s.ad_value(169), s.ad_value(37))), 174);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add(186, 169, 171);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad(187, A::square(s.ad_value(186)), A::mul(A::sub(A::mul(A::scale(s.ad_value(171), 0.5), s.ad_value(171)), s.ad_value(169)), s.ad_value(173)));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad_rhs(168, 174, A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(186)), s.ad_value(173)), A::add(s.ad_value(187), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(186), s.ad_value(173)), s.ad_value(173)), s.ad_value(187)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), s.ad_value(169))))));
        }

        s.v[188] = if (s.v[168] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) && (s.v[188] != 0.0)) {
            s.store_exp(175, 168);
        }

        if (((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) && (!(s.v[188] != 0.0))) {
            s.store_scale_ad(175, A::offset(A::mul(A::offset(s.ad_value(168), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(168), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(168), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_div_from_scalar(176, 1.0, 175);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_div_from_scalar_ad(164, 1.0, A::offset(A::square(s.ad_value(168)), 2.0));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub(164, 166, 168);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_mul(165, 52, 176);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad(177, A::scale(s.ad_value(164), 2.0), A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(175), (-1.0)), s.ad_value(165)), s.ad_value(52))));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(175), s.ad_value(168)), (-1.0)), s.ad_value(165)), A::mul(s.ad_value(52), A::offset(s.ad_value(168), (-1.0))))));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(175), s.ad_value(165))));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul(A::scale(s.ad_value(178), 2.0), s.ad_value(164)));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad(79, A::neg(s.ad_value(168)), A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_div_from_scalar_ad(163, 1.0, A::offset(A::scale(s.ad_value(34), 0.7324648775608221), 1.25));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_mul_ad_lhs(179, A::offset(A::mul(A::scale(s.ad_value(43), 1.25), s.ad_value(163)), (-1.0)), 163);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_mul_ad(182, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(179), s.ad_value(78)), 1.0));
        }

        s.v[189] = if ((-s.v[182]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[189] != 0.0)) {
            s.store_exp_ad(164, A::neg(s.ad_value(182)));
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[189] != 0.0))) {
            s.store_div_from_scalar_ad(164, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_from_scalar(181, 1.0, 164);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_ad(180, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(181)))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_offset(172, 50, 3.0);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
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

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub(164, 78, 174);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_exp_ad(165, A::neg(s.ad_value(174)));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_max_from_scalar_ad(169, 1e-40, A::sub(A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(165), s.ad_value(174)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(174), 1.0))))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_from_scalar_ad(170, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(165)));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad(171, A::scale(s.ad_value(164), 2.0), A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(165)), s.ad_value(52))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad(173, A::sub(s.ad_value(50), s.ad_value(174)), A::ln(A::div(s.ad_value(169), s.ad_value(36))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add(190, 169, 171);
        }

        s.v[192] = if (((s.v[173]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[192] != 0.0)) {
            s.copy_ad(183, 174);
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[192] != 0.0))) {
            s.store_add_ad(191, A::square(s.ad_value(190)), A::mul(A::sub(A::mul(A::scale(s.ad_value(171), 0.5), s.ad_value(171)), A::mul(s.ad_value(169), s.ad_value(170))), s.ad_value(173)));
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[192] != 0.0))) {
            let assign2190_ad_e2144: A = A::add(s.ad_value(174), A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(190)), s.ad_value(173)), A::add(s.ad_value(191), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(190), s.ad_value(173)), s.ad_value(173)), s.ad_value(191)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), A::mul(s.ad_value(169), s.ad_value(170)))))));
            s.store_ad(183, &assign2190_ad_e2144);
        }

        s.v[193] = if (s.v[183] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[193] != 0.0)) {
            s.store_exp(175, 183);
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[193] != 0.0)) {
            s.store_div_from_scalar(176, 1.0, 175);
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[193] != 0.0)) {
            s.store_mul(175, 52, 175);
        }

        s.v[194] = if (s.v[183] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (s.v[194] != 0.0)) {
            s.store_exp_ad(175, A::sub(s.ad_value(183), s.ad_value(50)));
        }

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (s.v[194] != 0.0)) {
            s.store_div(176, 52, 175);
        }

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (!(s.v[194] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (!(s.v[194] != 0.0))) {
            s.store_div_from_scalar_ad(176, 1e-100, A::offset(A::mul(A::offset(s.ad_value(183), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(183), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(183), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_div_from_scalar_ad(164, 1.0, A::offset(A::square(s.ad_value(183)), 2.0));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub(164, 78, 183);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad(177, A::scale(s.ad_value(164), 2.0), A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(176)), s.ad_value(175)), s.ad_value(52))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(176), s.ad_value(183)), (-1.0)), s.ad_value(175)), A::mul(s.ad_value(52), A::offset(s.ad_value(183), 1.0)))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(176), s.ad_value(175))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul(A::scale(s.ad_value(178), 2.0), s.ad_value(164)));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad_rhs(79, 183, A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        s.v[195] = if (p.p29 < 1e27) { 1.0 } else { 0.0 };

        if (s.v[195] != 0.0) {
            s.store_scale_ad(80, A::sub(s.ad_value(77), A::scale(s.ad_value(79), s.v[25])), (((-p.p17) * p.p18) * s.v[26]));
        }

        s.v[217] = if (((s.v[80]) as f64).abs() <= s.v[41]) { 1.0 } else { 0.0 };

        if ((s.v[195] != 0.0) && (s.v[217] != 0.0)) {
            s.store_scalar(198, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
        }

        if ((s.v[195] != 0.0) && (s.v[217] != 0.0)) {
            s.store_mul_ad(81, A::scale(s.ad_value(80), s.v[46]), A::offset(A::mul(A::scale(A::mul(s.ad_value(80), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35]), s.ad_value(198)), 1.0));
        }

        s.v[218] = if (s.v[80] < (-s.v[41])) { 1.0 } else { 0.0 };

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_neg(199, 80);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_scale(200, 199, (1.25 * s.v[46]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_scale_ad(207, A::sub(A::offset(s.ad_value(200), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(200), (-6.0)), A::offset(s.ad_value(200), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub(197, 199, 207);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad(202, A::square(s.ad_value(197)), A::scale(A::offset(s.ad_value(207), 1.0), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_offset_scaled(204, 197, 2.0, (-s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad_lhs(206, A::ln(A::scale(s.ad_value(202), s.v[39])), 207);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add(219, 202, 204);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad(220, A::square(s.ad_value(219)), A::mul(A::sub(A::mul(A::scale(s.ad_value(204), 0.5), s.ad_value(204)), s.ad_value(202)), s.ad_value(206)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad_rhs(201, 207, A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(219)), s.ad_value(206)), A::add(s.ad_value(220), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(219), s.ad_value(206)), s.ad_value(206)), s.ad_value(220)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), s.ad_value(202))))));
        }

        s.v[221] = if (s.v[201] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) && (s.v[221] != 0.0)) {
            s.store_exp(208, 201);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) && (!(s.v[221] != 0.0))) {
            s.store_scale_ad(208, A::offset(A::mul(A::offset(s.ad_value(201), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(201), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(201), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_div_from_scalar(209, 1.0, 208);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_div_from_scalar_ad(197, 1.0, A::offset(A::square(s.ad_value(201)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub(197, 199, 201);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_mul(198, 53, 209);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad(210, A::scale(s.ad_value(197), 2.0), A::scale(A::add(A::sub(A::offset(s.ad_value(208), (-1.0)), s.ad_value(198)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(208), s.ad_value(201)), (-1.0)), s.ad_value(198)), A::mul(s.ad_value(53), A::offset(s.ad_value(201), (-1.0)))), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(208), s.ad_value(198)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul(A::scale(s.ad_value(211), 2.0), s.ad_value(197)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad(81, A::neg(s.ad_value(201)), A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_scalar(196, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_mul_ad_lhs(212, A::offset(A::scale(s.ad_value(196), (s.v[45] * 1.25)), (-1.0)), 196);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_mul_ad(215, A::scale(s.ad_value(80), s.v[46]), A::offset(A::mul(s.ad_value(212), s.ad_value(80)), 1.0));
        }

        s.v[222] = if ((-s.v[215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[222] != 0.0)) {
            s.store_exp_ad(197, A::neg(s.ad_value(215)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[222] != 0.0))) {
            s.store_div_from_scalar_ad(197, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_from_scalar(214, 1.0, 197);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_ad(213, A::offset(s.ad_value(80), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(80), (s.v[38] * 0.25)), s.ad_value(214))), s.v[35]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_scalar(205, (s.v[51] + 3.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
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

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub(197, 80, 207);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_exp_ad(198, A::neg(s.ad_value(207)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_max_from_scalar_ad(202, 1e-40, A::sub(A::square(s.ad_value(197)), A::scale(A::sub(A::offset(A::add(s.ad_value(198), s.ad_value(207)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(207), 1.0))), s.v[38])));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_from_scalar_ad(203, 1.0, A::scale(s.ad_value(198), (0.5 * s.v[38])));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad(204, A::scale(s.ad_value(197), 2.0), A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(198)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad(206, A::sub_from_scalar(s.v[51], s.ad_value(207)), A::ln(A::scale(s.ad_value(202), 1.0 / (s.v[38]))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add(223, 202, 204);
        }

        s.v[225] = if (((s.v[206]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[225] != 0.0)) {
            s.copy_ad(216, 207);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[225] != 0.0))) {
            s.store_add_ad(224, A::square(s.ad_value(223)), A::mul(A::sub(A::mul(A::scale(s.ad_value(204), 0.5), s.ad_value(204)), A::mul(s.ad_value(202), s.ad_value(203))), s.ad_value(206)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[225] != 0.0))) {
            let assign2840_ad_e3281: A = A::add(s.ad_value(207), A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(223)), s.ad_value(206)), A::add(s.ad_value(224), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(223), s.ad_value(206)), s.ad_value(206)), s.ad_value(224)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), A::mul(s.ad_value(202), s.ad_value(203)))))));
            s.store_ad(216, &assign2840_ad_e3281);
        }

        s.v[226] = if (s.v[216] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[226] != 0.0)) {
            s.store_exp(208, 216);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[226] != 0.0)) {
            s.store_div_from_scalar(209, 1.0, 208);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[226] != 0.0)) {
            s.store_mul(208, 53, 208);
        }

        s.v[227] = if (s.v[216] > (s.v[51] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (s.v[227] != 0.0)) {
            s.store_exp_ad(208, A::offset(s.ad_value(216), (-s.v[51])));
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
        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (s.v[227] != 0.0)) {
            s.store_div(209, 53, 208);
        }

        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (!(s.v[227] != 0.0))) {
            s.store_div_from_scalar_ad(208, 1e-100, A::offset(A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (!(s.v[227] != 0.0))) {
            s.store_div_from_scalar_ad(209, 1e-100, A::offset(A::mul(A::offset(s.ad_value(216), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(216), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(216), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_div_from_scalar_ad(197, 1.0, A::offset(A::square(s.ad_value(216)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub(197, 80, 216);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad(210, A::scale(s.ad_value(197), 2.0), A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(209)), s.ad_value(208)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(209), s.ad_value(216)), (-1.0)), s.ad_value(208)), A::mul(s.ad_value(53), A::offset(s.ad_value(216), 1.0))), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(209), s.ad_value(208)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul(A::scale(s.ad_value(211), 2.0), s.ad_value(197)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad_rhs(81, 216, A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if (s.v[195] != 0.0) {
            s.store_scale(82, 81, (((-p.p17) * p.p18) * s.v[25]));
        }

        if (s.v[195] != 0.0) {
            s.store_scaled_sub(78, 77, 82, 1.0 / (s.v[25]));
        }

        s.v[249] = if (((s.v[78]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if ((s.v[195] != 0.0) && (s.v[249] != 0.0)) {
            s.store_scale_ad(230, A::square(s.ad_value(44)), (0.1666666666666667 * 0.7071067811865475));
        }

        if ((s.v[195] != 0.0) && (s.v[249] != 0.0)) {
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(230)), 1.0));
        }

        s.v[250] = if (s.v[78] < (-s.v[40])) { 1.0 } else { 0.0 };

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_neg(231, 78);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_mul_ad_lhs(232, A::scale(s.ad_value(231), 1.25), 44);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_scale_ad(239, A::sub(A::offset(s.ad_value(232), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(232), (-6.0)), A::offset(s.ad_value(232), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub(229, 231, 239);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad(234, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::offset(s.ad_value(239), 1.0)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad_lhs(236, A::scale(s.ad_value(229), 2.0), 36);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad_lhs(238, A::ln(A::mul(s.ad_value(234), s.ad_value(37))), 239);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add(251, 234, 236);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad(252, A::square(s.ad_value(251)), A::mul(A::sub(A::mul(A::scale(s.ad_value(236), 0.5), s.ad_value(236)), s.ad_value(234)), s.ad_value(238)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad_rhs(233, 239, A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(251)), s.ad_value(238)), A::add(s.ad_value(252), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(251), s.ad_value(238)), s.ad_value(238)), s.ad_value(252)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), s.ad_value(234))))));
        }

        s.v[253] = if (s.v[233] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) && (s.v[253] != 0.0)) {
            s.store_exp(240, 233);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) && (!(s.v[253] != 0.0))) {
            s.store_scale_ad(240, A::offset(A::mul(A::offset(s.ad_value(233), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(233), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(233), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_div_from_scalar(241, 1.0, 240);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_div_from_scalar_ad(229, 1.0, A::offset(A::square(s.ad_value(233)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub(229, 231, 233);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_mul(230, 52, 241);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad(242, A::scale(s.ad_value(229), 2.0), A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(240), (-1.0)), s.ad_value(230)), s.ad_value(52))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(240), s.ad_value(233)), (-1.0)), s.ad_value(230)), A::mul(s.ad_value(52), A::offset(s.ad_value(233), (-1.0))))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(240), s.ad_value(230))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul(A::scale(s.ad_value(243), 2.0), s.ad_value(229)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad(79, A::neg(s.ad_value(233)), A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_div_from_scalar_ad(228, 1.0, A::offset(A::scale(s.ad_value(34), 0.7324648775608221), 1.25));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_mul_ad_lhs(244, A::offset(A::mul(A::scale(s.ad_value(43), 1.25), s.ad_value(228)), (-1.0)), 228);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_mul_ad(247, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(244), s.ad_value(78)), 1.0));
        }

        s.v[254] = if ((-s.v[247]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[254] != 0.0)) {
            s.store_exp_ad(229, A::neg(s.ad_value(247)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_from_scalar(246, 1.0, 229);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_ad(245, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(246)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_offset(237, 50, 3.0);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
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

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub(229, 78, 239);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_exp_ad(230, A::neg(s.ad_value(239)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_max_from_scalar_ad(234, 1e-40, A::sub(A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(230), s.ad_value(239)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(239), 1.0))))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_from_scalar_ad(235, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(230)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad(236, A::scale(s.ad_value(229), 2.0), A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(230)), s.ad_value(52))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad(238, A::sub(s.ad_value(50), s.ad_value(239)), A::ln(A::div(s.ad_value(234), s.ad_value(36))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add(255, 234, 236);
        }

        s.v[257] = if (((s.v[238]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[257] != 0.0)) {
            s.copy_ad(248, 239);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_add_ad(256, A::square(s.ad_value(255)), A::mul(A::sub(A::mul(A::scale(s.ad_value(236), 0.5), s.ad_value(236)), A::mul(s.ad_value(234), s.ad_value(235))), s.ad_value(238)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[257] != 0.0))) {
            let assign3490_ad_e4447: A = A::add(s.ad_value(239), A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(255)), s.ad_value(238)), A::add(s.ad_value(256), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(255), s.ad_value(238)), s.ad_value(238)), s.ad_value(256)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), A::mul(s.ad_value(234), s.ad_value(235)))))));
            s.store_ad(248, &assign3490_ad_e4447);
        }

        s.v[258] = if (s.v[248] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[258] != 0.0)) {
            s.store_exp(240, 248);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[258] != 0.0)) {
            s.store_div_from_scalar(241, 1.0, 240);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[258] != 0.0)) {
            s.store_mul(240, 52, 240);
        }

        s.v[259] = if (s.v[248] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (s.v[259] != 0.0)) {
            s.store_exp_ad(240, A::sub(s.ad_value(248), s.ad_value(50)));
        }

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (s.v[259] != 0.0)) {
            s.store_div(241, 52, 240);
        }

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (!(s.v[259] != 0.0))) {
            s.store_div_from_scalar_ad(240, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (!(s.v[259] != 0.0))) {
            s.store_div_from_scalar_ad(241, 1e-100, A::offset(A::mul(A::offset(s.ad_value(248), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(248), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(248), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1.0, A::offset(A::square(s.ad_value(248)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub(229, 78, 248);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad(242, A::scale(s.ad_value(229), 2.0), A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(241)), s.ad_value(240)), s.ad_value(52))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(241), s.ad_value(248)), (-1.0)), s.ad_value(240)), A::mul(s.ad_value(52), A::offset(s.ad_value(248), 1.0)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(241), s.ad_value(240))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul(A::scale(s.ad_value(243), 2.0), s.ad_value(229)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad_rhs(79, 248, A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if (!(s.v[195] != 0.0)) {
            s.store_scalar(82, 0.0);
        }

        s.v[260] = if ((s.v[78] <= 0.0) || (p.p21 < 1.0)) { 1.0 } else { 0.0 };

        if (s.v[260] != 0.0) {
            s.store_scalar(90, 0.0);
        }

        if (!(s.v[260] != 0.0)) {
            s.store_scalar(83, 0.0);
        }

        s.v[261] = if (s.v[79] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(s.v[260] != 0.0)) && (s.v[261] != 0.0)) {
            s.store_exp(83, 79);
        }

        if ((!(s.v[260] != 0.0)) && (s.v[261] != 0.0)) {
            s.store_div_from_scalar(85, 1.0, 83);
        }

        if ((!(s.v[260] != 0.0)) && (s.v[261] != 0.0)) {
            s.store_mul(83, 52, 83);
        }

        if ((!(s.v[260] != 0.0)) && (s.v[261] != 0.0)) {
            s.store_mul_ad_rhs(87, 52, A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(85)), s.ad_value(79)), (-1.0)));
        }

        s.v[262] = if (s.v[79] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (s.v[262] != 0.0)) {
            s.store_exp_ad(83, A::sub(s.ad_value(79), s.ad_value(50)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (s.v[262] != 0.0)) {
            s.store_div(85, 52, 83);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (s.v[262] != 0.0)) {
            s.store_sub_ad_rhs(87, 83, A::mul(s.ad_value(52), A::offset(s.ad_value(79), 1.0)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (!(s.v[262] != 0.0))) {
            s.store_div_from_scalar_ad(83, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (!(s.v[262] != 0.0))) {
            s.store_div_from_scalar_ad(85, 1e-100, A::offset(A::mul(A::offset(s.ad_value(79), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(79), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(79), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (!(s.v[262] != 0.0))) {
            s.store_sub_ad_rhs(87, 83, A::mul(s.ad_value(52), A::offset(s.ad_value(79), 1.0)));
        }

        s.v[263] = if (s.v[79] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(s.v[260] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_mul_ad(86, A::mul(A::scale(s.ad_value(79), 0.5), s.ad_value(79)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(79), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25)))));
        }

        if ((!(s.v[260] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_mul_ad(87, A::mul(A::mul(A::mul(A::scale(s.ad_value(52), 0.1666666666666667), s.ad_value(79)), s.ad_value(79)), s.ad_value(79)), A::offset(A::scale(s.ad_value(79), 1.75), 1.0));
        }

        if ((!(s.v[260] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_sqrt_ad(6, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(79), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25)))));
        }

        if ((!(s.v[260] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_mul_ad_lhs(88, A::scale(s.ad_value(79), 0.7071067811865475), 6);
        }

        if ((!(s.v[260] != 0.0)) && (!(s.v[263] != 0.0))) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(79), (-1.0)), 85);
        }

        if ((!(s.v[260] != 0.0)) && (!(s.v[263] != 0.0))) {
            s.store_sqrt(88, 86);
        }

        if (!(s.v[260] != 0.0)) {
            s.store_mul_ad_rhs(89, 34, A::sqrt(A::add(s.ad_value(86), s.ad_value(87))));
        }

        if (!(s.v[260] != 0.0)) {
            s.store_div_ad(90, A::mul(A::scale(s.ad_value(36), s.v[25]), s.ad_value(87)), A::add(s.ad_value(89), A::mul(s.ad_value(34), s.ad_value(88))));
        }

        s.store_neg(92, 90);

        s.store_scale_ad(94, A::add(s.ad_value(77), A::voltage(ctx, &nodes, Some(6), None)), s.v[26]);

        s.v[281] = if (((s.v[94]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if (s.v[281] != 0.0) {
            s.store_div(95, 94, 43);
        }

        s.v[282] = if (s.v[94] > s.v[40]) { 1.0 } else { 0.0 };

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_div_ad_lhs(276, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_mul_ad(277, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(276), s.ad_value(94)), 1.0));
        }

        s.v[283] = if (s.v[277] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (s.v[283] != 0.0)) {
            s.store_exp_ad(275, A::neg(s.ad_value(277)));
        }

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (!(s.v[283] != 0.0))) {
            s.store_div_from_scalar_ad(275, 1e-200, A::offset(A::mul(A::offset(s.ad_value(277), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(277), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(277), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_from_scalar(278, 1.0, 275);
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_ad(279, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(278)))));
        }

        s.v[284] = if (s.v[279] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (s.v[284] != 0.0)) {
            s.store_exp_ad(271, A::neg(s.ad_value(279)));
        }

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (!(s.v[284] != 0.0))) {
            s.store_div_from_scalar_ad(271, 1e-200, A::offset(A::mul(A::offset(s.ad_value(279), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(279), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(279), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(271)));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(94), s.ad_value(279)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(271))));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_ad(274, A::mul(A::sub(s.ad_value(94), s.ad_value(279)), A::sub(s.ad_value(94), s.ad_value(279))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(279), (-1.0)), s.ad_value(271))));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul(A::scale(s.ad_value(272), 4.0), s.ad_value(274)));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_div_ad(280, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_add(95, 279, 280);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_neg(264, 94);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_div_ad_lhs(265, A::scale(s.ad_value(264), 1.25), 43);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_scale_ad(266, A::sub(A::offset(s.ad_value(265), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(265), (-6.0)), A::offset(s.ad_value(265), (-6.0))), 64.0))), 0.5);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(267, A::mul(A::sub(s.ad_value(264), s.ad_value(266)), A::sub(s.ad_value(264), s.ad_value(266))), A::mul(s.ad_value(36), A::offset(s.ad_value(266), 1.0)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_ad_lhs(268, A::scale(A::sub(s.ad_value(264), s.ad_value(266)), 2.0), 36);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_ad_lhs(269, A::ln(A::div(s.ad_value(267), s.ad_value(36))), 266);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add(285, 267, 268);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(286, A::square(s.ad_value(285)), A::mul(A::sub(A::mul(A::scale(s.ad_value(268), 0.5), s.ad_value(268)), s.ad_value(267)), s.ad_value(269)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad_rhs(270, 266, A::div(A::mul(A::mul(s.ad_value(267), s.ad_value(285)), s.ad_value(269)), A::add(s.ad_value(286), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(285), s.ad_value(269)), s.ad_value(269)), s.ad_value(286)), s.ad_value(268)), A::sub(A::scale(A::square(s.ad_value(268)), 0.3333333333333333), s.ad_value(267))))));
        }

        s.v[287] = if (((s.v[270]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) && (s.v[287] != 0.0)) {
            s.store_exp(271, 270);
        }

        s.v[288] = if (s.v[270] < (-230.25850929940458)) { 1.0 } else { 0.0 };

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
        if ((((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) && (!(s.v[287] != 0.0))) && (s.v[288] != 0.0)) {
            s.store_div_from_scalar_ad(271, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_scale_ad(271, A::offset(A::mul(A::offset(s.ad_value(270), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(270), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(270), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(271)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(264), s.ad_value(270)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(271), (-1.0))));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(274, A::mul(A::sub(s.ad_value(264), s.ad_value(270)), A::sub(s.ad_value(264), s.ad_value(270))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(270), 1.0), s.ad_value(271))));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul(A::scale(s.ad_value(272), 4.0), s.ad_value(274)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_div_ad(278, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_neg_ad(95, A::add(s.ad_value(270), s.ad_value(278)));
        }

        s.store_scale(96, 95, s.v[25]);

        s.v[289] = if (p.p29 < 1e27) { 1.0 } else { 0.0 };

        if (s.v[289] != 0.0) {
            s.store_scale_ad(97, A::sub(s.ad_value(77), A::scale(s.ad_value(95), s.v[25])), (((-p.p17) * p.p18) * s.v[26]));
        }

        s.v[311] = if (((s.v[97]) as f64).abs() <= s.v[41]) { 1.0 } else { 0.0 };

        if ((s.v[289] != 0.0) && (s.v[311] != 0.0)) {
            s.store_scalar(292, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
        }

        if ((s.v[289] != 0.0) && (s.v[311] != 0.0)) {
            s.store_mul_ad(98, A::scale(s.ad_value(97), s.v[46]), A::offset(A::mul(A::scale(A::mul(s.ad_value(97), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35]), s.ad_value(292)), 1.0));
        }

        s.v[312] = if (s.v[97] < (-s.v[41])) { 1.0 } else { 0.0 };

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_neg(293, 97);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_scale(294, 293, (1.25 * s.v[46]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_scale_ad(301, A::sub(A::offset(s.ad_value(294), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(294), (-6.0)), A::offset(s.ad_value(294), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub(291, 293, 301);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad(296, A::square(s.ad_value(291)), A::scale(A::offset(s.ad_value(301), 1.0), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_offset_scaled(298, 291, 2.0, (-s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad_lhs(300, A::ln(A::scale(s.ad_value(296), s.v[39])), 301);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add(313, 296, 298);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad(314, A::square(s.ad_value(313)), A::mul(A::sub(A::mul(A::scale(s.ad_value(298), 0.5), s.ad_value(298)), s.ad_value(296)), s.ad_value(300)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad_rhs(295, 301, A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(313)), s.ad_value(300)), A::add(s.ad_value(314), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(313), s.ad_value(300)), s.ad_value(300)), s.ad_value(314)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), s.ad_value(296))))));
        }

        s.v[315] = if (s.v[295] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) && (s.v[315] != 0.0)) {
            s.store_exp(302, 295);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) && (!(s.v[315] != 0.0))) {
            s.store_scale_ad(302, A::offset(A::mul(A::offset(s.ad_value(295), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(295), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(295), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_div_from_scalar(303, 1.0, 302);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_div_from_scalar_ad(291, 1.0, A::offset(A::square(s.ad_value(295)), 2.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub(291, 293, 295);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_mul(292, 53, 303);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad(304, A::scale(s.ad_value(291), 2.0), A::scale(A::add(A::sub(A::offset(s.ad_value(302), (-1.0)), s.ad_value(292)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(302), s.ad_value(295)), (-1.0)), s.ad_value(292)), A::mul(s.ad_value(53), A::offset(s.ad_value(295), (-1.0)))), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(302), s.ad_value(292)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul(A::scale(s.ad_value(305), 2.0), s.ad_value(291)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad(98, A::neg(s.ad_value(295)), A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_scalar(290, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_mul_ad_lhs(306, A::offset(A::scale(s.ad_value(290), (s.v[45] * 1.25)), (-1.0)), 290);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_mul_ad(309, A::scale(s.ad_value(97), s.v[46]), A::offset(A::mul(s.ad_value(306), s.ad_value(97)), 1.0));
        }

        s.v[316] = if ((-s.v[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[316] != 0.0)) {
            s.store_exp_ad(291, A::neg(s.ad_value(309)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[316] != 0.0))) {
            s.store_div_from_scalar_ad(291, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_from_scalar(308, 1.0, 291);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_ad(307, A::offset(s.ad_value(97), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(97), (s.v[38] * 0.25)), s.ad_value(308))), s.v[35]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_scalar(299, (s.v[51] + 3.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
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

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub(291, 97, 301);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_exp_ad(292, A::neg(s.ad_value(301)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_max_from_scalar_ad(296, 1e-40, A::sub(A::square(s.ad_value(291)), A::scale(A::sub(A::offset(A::add(s.ad_value(292), s.ad_value(301)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(301), 1.0))), s.v[38])));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_from_scalar_ad(297, 1.0, A::scale(s.ad_value(292), (0.5 * s.v[38])));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(298, A::scale(s.ad_value(291), 2.0), A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(292)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(300, A::sub_from_scalar(s.v[51], s.ad_value(301)), A::ln(A::scale(s.ad_value(296), 1.0 / (s.v[38]))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add(317, 296, 298);
        }

        s.v[319] = if (((s.v[300]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[319] != 0.0)) {
            s.copy_ad(310, 301);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[319] != 0.0))) {
            s.store_add_ad(318, A::square(s.ad_value(317)), A::mul(A::sub(A::mul(A::scale(s.ad_value(298), 0.5), s.ad_value(298)), A::mul(s.ad_value(296), s.ad_value(297))), s.ad_value(300)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[319] != 0.0))) {
            let assign4810_ad_e6543: A = A::add(s.ad_value(301), A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(317)), s.ad_value(300)), A::add(s.ad_value(318), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(317), s.ad_value(300)), s.ad_value(300)), s.ad_value(318)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), A::mul(s.ad_value(296), s.ad_value(297)))))));
            s.store_ad(310, &assign4810_ad_e6543);
        }

        s.v[320] = if (s.v[310] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[320] != 0.0)) {
            s.store_exp(302, 310);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[320] != 0.0)) {
            s.store_div_from_scalar(303, 1.0, 302);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[320] != 0.0)) {
            s.store_mul(302, 53, 302);
        }

        s.v[321] = if (s.v[310] > (s.v[51] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (s.v[321] != 0.0)) {
            s.store_exp_ad(302, A::offset(s.ad_value(310), (-s.v[51])));
        }

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (s.v[321] != 0.0)) {
            s.store_div(303, 53, 302);
        }

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (!(s.v[321] != 0.0))) {
            s.store_div_from_scalar_ad(302, 1e-100, A::offset(A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (!(s.v[321] != 0.0))) {
            s.store_div_from_scalar_ad(303, 1e-100, A::offset(A::mul(A::offset(s.ad_value(310), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(310), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(310), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_div_from_scalar_ad(291, 1.0, A::offset(A::square(s.ad_value(310)), 2.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub(291, 97, 310);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(304, A::scale(s.ad_value(291), 2.0), A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(303)), s.ad_value(302)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(303), s.ad_value(310)), (-1.0)), s.ad_value(302)), A::mul(s.ad_value(53), A::offset(s.ad_value(310), 1.0))), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(303), s.ad_value(302)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul(A::scale(s.ad_value(305), 2.0), s.ad_value(291)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad_rhs(98, 310, A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if (s.v[289] != 0.0) {
            s.store_scale(99, 98, (((-p.p17) * p.p18) * s.v[25]));
        }

        if (s.v[289] != 0.0) {
            s.store_scale_ad(94, A::sub(A::add(s.ad_value(77), A::voltage(ctx, &nodes, Some(6), None)), s.ad_value(99)), 1.0 / (s.v[25]));
        }

        s.v[339] = if (((s.v[94]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if ((s.v[289] != 0.0) && (s.v[339] != 0.0)) {
            s.store_div(95, 94, 43);
        }

        s.v[340] = if (s.v[94] > s.v[40]) { 1.0 } else { 0.0 };

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_div_ad_lhs(334, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_mul_ad(335, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(334), s.ad_value(94)), 1.0));
        }

        s.v[341] = if (s.v[335] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (s.v[341] != 0.0)) {
            s.store_exp_ad(333, A::neg(s.ad_value(335)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (!(s.v[341] != 0.0))) {
            s.store_div_from_scalar_ad(333, 1e-200, A::offset(A::mul(A::offset(s.ad_value(335), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(335), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(335), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_from_scalar(336, 1.0, 333);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_ad(337, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(336)))));
        }

        s.v[342] = if (s.v[337] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (s.v[342] != 0.0)) {
            s.store_exp_ad(329, A::neg(s.ad_value(337)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (!(s.v[342] != 0.0))) {
            s.store_div_from_scalar_ad(329, 1e-200, A::offset(A::mul(A::offset(s.ad_value(337), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(337), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(337), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(329)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(94), s.ad_value(337)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(329))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_ad(332, A::mul(A::sub(s.ad_value(94), s.ad_value(337)), A::sub(s.ad_value(94), s.ad_value(337))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(337), (-1.0)), s.ad_value(329))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul(A::scale(s.ad_value(330), 4.0), s.ad_value(332)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_div_ad(338, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_add(95, 337, 338);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_neg(322, 94);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_div_ad_lhs(323, A::scale(s.ad_value(322), 1.25), 43);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_scale_ad(324, A::sub(A::offset(s.ad_value(323), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(323), (-6.0)), A::offset(s.ad_value(323), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(325, A::mul(A::sub(s.ad_value(322), s.ad_value(324)), A::sub(s.ad_value(322), s.ad_value(324))), A::mul(s.ad_value(36), A::offset(s.ad_value(324), 1.0)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_ad_lhs(326, A::scale(A::sub(s.ad_value(322), s.ad_value(324)), 2.0), 36);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_ad_lhs(327, A::ln(A::div(s.ad_value(325), s.ad_value(36))), 324);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add(343, 325, 326);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(344, A::square(s.ad_value(343)), A::mul(A::sub(A::mul(A::scale(s.ad_value(326), 0.5), s.ad_value(326)), s.ad_value(325)), s.ad_value(327)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad_rhs(328, 324, A::div(A::mul(A::mul(s.ad_value(325), s.ad_value(343)), s.ad_value(327)), A::add(s.ad_value(344), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(343), s.ad_value(327)), s.ad_value(327)), s.ad_value(344)), s.ad_value(326)), A::sub(A::scale(A::square(s.ad_value(326)), 0.3333333333333333), s.ad_value(325))))));
        }

        s.v[345] = if (((s.v[328]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) && (s.v[345] != 0.0)) {
            s.store_exp(329, 328);
        }

        s.v[346] = if (s.v[328] < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) && (!(s.v[345] != 0.0))) && (s.v[346] != 0.0)) {
            s.store_div_from_scalar_ad(329, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) && (!(s.v[345] != 0.0))) && (!(s.v[346] != 0.0))) {
            s.store_scale_ad(329, A::offset(A::mul(A::offset(s.ad_value(328), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(328), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(328), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(329)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(322), s.ad_value(328)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(329), (-1.0))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(332, A::mul(A::sub(s.ad_value(322), s.ad_value(328)), A::sub(s.ad_value(322), s.ad_value(328))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(328), 1.0), s.ad_value(329))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul(A::scale(s.ad_value(330), 4.0), s.ad_value(332)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_div_ad(336, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_neg_ad(95, A::add(s.ad_value(328), s.ad_value(336)));
        }

        if (s.v[289] != 0.0) {
            s.store_scale(96, 95, s.v[25]);
        }

        if (!(s.v[289] != 0.0)) {
            s.store_scalar(99, 0.0);
        }

        s.v[83] = 0.0;

        s.v[347] = if (s.v[95] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (s.v[347] != 0.0) {
            s.store_exp(83, 95);
        }

        if (s.v[347] != 0.0) {
            s.store_div_from_scalar(85, 1.0, 83);
        }

        s.v[348] = if (s.v[95] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((!(s.v[347] != 0.0)) && (s.v[348] != 0.0)) {
            s.store_exp_ad(83, A::sub(s.ad_value(50), s.ad_value(95)));
        }

        if ((!(s.v[347] != 0.0)) && (s.v[348] != 0.0)) {
            s.store_mul(85, 52, 83);
        }

        if ((!(s.v[347] != 0.0)) && (!(s.v[348] != 0.0))) {
            s.store_div_from_scalar_ad(85, 1e-100, A::offset(A::mul(A::offset(s.ad_value(95), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(95), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(95), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        s.v[349] = if (s.v[95] < (-s.v[40])) { 1.0 } else { 0.0 };

        if (s.v[349] != 0.0) {
            s.store_offset_ad(86, A::add(s.ad_value(85), s.ad_value(95)), (-1.0));
        }

        if (s.v[349] != 0.0) {
            s.store_neg_ad(88, A::sqrt(s.ad_value(86)));
        }

        s.v[350] = if (((s.v[95]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if ((!(s.v[349] != 0.0)) && (s.v[350] != 0.0)) {
            s.store_sub_from_scalar_ad(6, 1.0, A::mul(A::scale(s.ad_value(95), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(95), 0.25))));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[350] != 0.0)) {
            s.store_mul_ad_lhs(86, A::mul(A::scale(s.ad_value(95), 0.5), s.ad_value(95)), 6);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[350] != 0.0)) {
            s.store_mul_ad(88, A::scale(s.ad_value(95), 0.7071067811865475), A::sqrt(s.ad_value(6)));
        }

        if ((!(s.v[349] != 0.0)) && (!(s.v[350] != 0.0))) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(95), (-1.0)), 85);
        }

        if ((!(s.v[349] != 0.0)) && (!(s.v[350] != 0.0))) {
            s.store_sqrt(88, 86);
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
        let nv6 = ctx.node_voltage(nodes[6]);
        s.store_mul_ad_lhs(91, A::scale(s.ad_value(88), s.v[25]), 34);

        s.store_scale_ad(139, A::mul(A::scale(A::offset(s.ad_value(140), 1.0), 1.62), A::offset(s.ad_value(140), 1.0)), ((1.0 + (0.37 * s.v[141])) * ((1.0 + (0.37 * s.v[141])) * (s.v[20] * (((s.v[20]) as f64).sqrt() * (s.v[25] * s.v[25]))))));

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
        let assign5600_ad_e7830: A = A::add(A::voltage(ctx, &nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None)), A::sqrt(A::add(A::mul(A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None)), A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None))), s.ad_value(139)))), 0.5));
        assign5600_ad_e7830
    } else {
        let assign5600_ad_e7873: A = {
            if ((nv6 - (-nv6)) > 1e-16) {
                let assign5600_ad_e7858: A = A::div(A::scale(s.ad_value(139), 0.5), A::add(A::sub(A::voltage(ctx, &nodes, Some(6), None), A::neg(A::voltage(ctx, &nodes, Some(6), None))), A::sqrt(A::add(A::mul(A::sub(A::voltage(ctx, &nodes, Some(6), None), A::neg(A::voltage(ctx, &nodes, Some(6), None))), A::sub(A::voltage(ctx, &nodes, Some(6), None), A::neg(A::voltage(ctx, &nodes, Some(6), None)))), s.ad_value(139)))));
                A::add(A::voltage(ctx, &nodes, Some(6), None), assign5600_ad_e7858)
            } else {
                A::add(A::voltage(ctx, &nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None)), A::sqrt(A::offset(s.ad_value(139), 1e-32))), 0.5))
            }
        };
        assign5600_ad_e7873
    }
};
        s.store_add_ad(59, assign5600_ad_e7802, A::mul(s.ad_value(84), assign5600_ad_e7874));

        s.v[58] = s.v[11];

        s.v[351] = if (s.v[54] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[351] != 0.0) {
            s.store_div_from_scalar_ad(58, s.v[11], A::offset(A::mul(s.ad_value(54), A::powf(A::offset(A::square(s.ad_value(59)), s.v[57]), ((-1.0) * 0.1666666666666667))), 1.0));
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
        s.store_scale_ad(100, A::exp(A::scale(assign5640_ad_e7964, (-1.0))), s.v[25]);

        s.store_sqrt(101, 100);

        s.store_mul_ad_lhs(102, A::mul(s.ad_value(12), s.ad_value(58)), 101);

        s.store_scale_ad(103, A::sub(A::sqrt(A::offset(A::square(s.ad_value(77)), 0.04)), s.ad_value(77)), 0.5);

        s.store_div_ad(104, A::mul(s.ad_value(70), s.ad_value(102)), A::offset(A::scale(s.ad_value(103), p.p41), 1.0));

        s.v[352] = if (p.p66 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[352] != 0.0) {
            s.store_scale(76, 104, s.v[71]);
        }

        s.v[136] = 0.0;

        s.v[353] = if ((p.p18 * p.p17) == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[353] != 0.0) {
            s.store_scalar(136, (p.p18 * s.v[42]));
        }

        s.store_scale_ad(114, A::sub(A::voltage(ctx, &nodes, Some(4), Some(1)), s.ad_value(136)), (p.p17 * s.v[26]));

        s.v[354] = if ((p.p49 != 0.0) && ((s.v[126] > 0.0) || (s.v[138] > 0.0))) { 1.0 } else { 0.0 };

        s.v[372] = if (((s.v[114]) as f64).abs() <= s.v[113]) { 1.0 } else { 0.0 };

        if ((s.v[354] != 0.0) && (s.v[372] != 0.0)) {
            s.store_scale(115, 114, 1.0 / (s.v[112]));
        }

        s.v[373] = if (s.v[114] > s.v[113]) { 1.0 } else { 0.0 };

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_scalar(367, ((((s.v[112] * 1.25) / s.v[116]) - 1.0) / s.v[116]));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_mul_ad(368, A::scale(s.ad_value(114), 1.0 / (s.v[112])), A::offset(A::mul(s.ad_value(367), s.ad_value(114)), 1.0));
        }

        s.v[374] = if (s.v[368] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) && (s.v[374] != 0.0)) {
            s.store_exp_ad(366, A::neg(s.ad_value(368)));
        }

        if ((((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) && (!(s.v[374] != 0.0))) {
            s.store_div_from_scalar_ad(366, 1e-200, A::offset(A::mul(A::offset(s.ad_value(368), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(368), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(368), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_sub_from_scalar(369, 1.0, 366);
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_sub_ad(370, A::offset(s.ad_value(114), (0.5 * s.v[111])), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(114), (0.25 * s.v[111])), s.ad_value(369))), s.v[110]));
        }

        s.v[375] = if (s.v[370] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) && (s.v[375] != 0.0)) {
            s.store_exp_ad(362, A::neg(s.ad_value(370)));
        }

        if ((((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) && (!(s.v[375] != 0.0))) {
            s.store_div_from_scalar_ad(362, 1e-200, A::offset(A::mul(A::offset(s.ad_value(370), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(370), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(370), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_sub_from_scalar_ad(363, 1.0, A::scale(s.ad_value(362), (0.5 * s.v[111])));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_add_ad(364, A::scale(A::sub(s.ad_value(114), s.ad_value(370)), 2.0), A::scale(A::sub_from_scalar(1.0, s.ad_value(362)), s.v[111]));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_sub_ad(365, A::mul(A::sub(s.ad_value(114), s.ad_value(370)), A::sub(s.ad_value(114), s.ad_value(370))), A::scale(A::add(A::offset(s.ad_value(370), (-1.0)), s.ad_value(362)), s.v[111]));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_sub_ad(366, A::square(s.ad_value(364)), A::mul(A::scale(s.ad_value(363), 4.0), s.ad_value(365)));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_div_ad(371, A::scale(s.ad_value(365), 2.0), A::add(s.ad_value(364), A::sqrt(s.ad_value(366))));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (s.v[373] != 0.0)) {
            s.store_add(115, 370, 371);
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_neg(355, 114);
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_scale(356, 355, (1.25 * 1.0 / (s.v[112])));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_scale_ad(357, A::sub(A::offset(s.ad_value(356), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(356), (-6.0)), A::offset(s.ad_value(356), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_add_ad(358, A::mul(A::sub(s.ad_value(355), s.ad_value(357)), A::sub(s.ad_value(355), s.ad_value(357))), A::scale(A::offset(s.ad_value(357), 1.0), s.v[111]));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_offset_ad(359, A::scale(A::sub(s.ad_value(355), s.ad_value(357)), 2.0), (-s.v[111]));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_sub_ad_lhs(360, A::ln(A::scale(s.ad_value(358), 1.0 / (s.v[111]))), 357);
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_add(376, 358, 359);
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_add_ad(377, A::square(s.ad_value(376)), A::mul(A::sub(A::mul(A::scale(s.ad_value(359), 0.5), s.ad_value(359)), s.ad_value(358)), s.ad_value(360)));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_add_ad_rhs(361, 357, A::div(A::mul(A::mul(s.ad_value(358), s.ad_value(376)), s.ad_value(360)), A::add(s.ad_value(377), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(376), s.ad_value(360)), s.ad_value(360)), s.ad_value(377)), s.ad_value(359)), A::sub(A::scale(A::square(s.ad_value(359)), 0.3333333333333333), s.ad_value(358))))));
        }

        s.v[378] = if (((s.v[361]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) && (s.v[378] != 0.0)) {
            s.store_exp(362, 361);
        }

        s.v[379] = if (s.v[361] < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) && (!(s.v[378] != 0.0))) && (s.v[379] != 0.0)) {
            s.store_div_from_scalar_ad(362, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(361)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(361)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(361)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) && (!(s.v[378] != 0.0))) && (!(s.v[379] != 0.0))) {
            s.store_scale_ad(362, A::offset(A::mul(A::offset(s.ad_value(361), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(361), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(361), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_sub_from_scalar_ad(363, 1.0, A::scale(s.ad_value(362), (0.5 * s.v[111])));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_add_ad(364, A::scale(A::sub(s.ad_value(355), s.ad_value(361)), 2.0), A::scale(A::offset(s.ad_value(362), (-1.0)), s.v[111]));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_add_ad(365, A::mul(A::sub(s.ad_value(355), s.ad_value(361)), A::sub(s.ad_value(355), s.ad_value(361))), A::scale(A::sub(A::offset(s.ad_value(361), 1.0), s.ad_value(362)), s.v[111]));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_sub_ad(366, A::square(s.ad_value(364)), A::mul(A::scale(s.ad_value(363), 4.0), s.ad_value(365)));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_div_ad(369, A::scale(s.ad_value(365), 2.0), A::add(s.ad_value(364), A::sqrt(s.ad_value(366))));
        }

        if (((s.v[354] != 0.0) && (!(s.v[372] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_neg_ad(115, A::add(s.ad_value(361), s.ad_value(369)));
        }

        if (s.v[354] != 0.0) {
            s.store_scaled_sub(118, 114, 115, s.v[25]);
        }

        if (!(s.v[354] != 0.0)) {
            s.store_scalar(118, 0.0);
        }

        if (!(s.v[354] != 0.0)) {
            s.store_scalar(115, 0.0);
        }

        s.v[4] = 0.0;

        s.v[5] = 0.0;

        s.v[380] = if ((s.v[126] > 0.0) || (s.v[138] > 0.0)) { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && (s.v[380] != 0.0)) {
            s.store_ad(127, &A::scale(A::voltage(ctx, &nodes, Some(4), Some(1)), p.p17));
        }

        if ((p.p49 != 0.0) && (s.v[380] != 0.0)) {
            s.store_scalar(5, 0.0);
        }

        s.v[391] = if ((p.p18 == 1.0) && (s.v[138] > 0.0)) { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) {
            s.store_add_ad_lhs(382, A::scale(s.ad_value(118), p.p17), 129);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) {
            let assign6250_ad_e8826: A = {
                if ((-s.v[382]) > 1e-16) {
                    A::add(s.ad_value(382), A::scale(A::add(A::neg(s.ad_value(382)), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(382)), A::neg(s.ad_value(382))), 0.01))), 0.5))
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
            s.store_ad(383, &assign6250_ad_e8826);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) {
            s.store_mul_ad_lhs(384, A::sqrt(A::offset(A::square(s.ad_value(118)), 1e-6)), 131);
        }

        s.v[392] = if (p.p64 < 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (s.v[392] != 0.0)) {
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
            s.store_ad(384, &assign6280_ad_e8915);
        }

        s.v[393] = if (0.0 == 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (s.v[393] != 0.0)) {
            s.store_neg_ad(385, A::add(A::scale(s.ad_value(115), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(134)), s.ad_value(383)), s.v[26])));
        }

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (!(s.v[393] != 0.0))) {
            s.store_neg_ad(385, A::add(A::scale(s.ad_value(115), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(93)), s.ad_value(383)), s.v[26])));
        }

        s.v[394] = if (s.v[385] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (s.v[394] != 0.0)) {
            s.store_ln_ad(390, A::offset(A::exp(s.ad_value(385)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (!(s.v[394] != 0.0))) {
            s.copy_ad(390, 385);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) {
            s.store_add_ad_rhs(386, 385, A::scale(s.ad_value(127), (p.p17 * s.v[26])));
        }

        s.v[395] = if (s.v[386] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (s.v[395] != 0.0)) {
            s.store_ln_ad(387, A::offset(A::exp(s.ad_value(386)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (!(s.v[395] != 0.0))) {
            s.copy_ad(387, 386);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) {
            s.store_mul_ad_rhs(389, 133, A::offset(A::mul(s.ad_value(384), A::offset(A::scale(s.ad_value(384), p.p64), p.p63)), (-1.5)));
        }

        s.v[396] = if (s.v[389] > 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (s.v[396] != 0.0)) {
            s.store_offset_ad(388, A::mul(s.ad_value(389), A::offset(A::mul(A::scale(s.ad_value(389), 0.5), A::offset(A::scale(s.ad_value(389), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[397] = if (s.v[389] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (!(s.v[396] != 0.0))) && (s.v[397] != 0.0)) {
            s.store_exp(388, 389);
        }

        if (((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) && (!(s.v[396] != 0.0))) && (!(s.v[397] != 0.0))) {
            s.store_div_from_scalar_ad(388, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[391] != 0.0)) {
            s.store_mul_ad(5, A::scale(A::mul(s.ad_value(138), s.ad_value(388)), p.p17), A::sub(s.ad_value(387), s.ad_value(390)));
        }

        s.v[398] = if (s.v[126] > 0.0) { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) {
            s.store_add_ad_lhs(381, A::scale(s.ad_value(118), p.p17), 121);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) {
            let assign6480_ad_e9234: A = {
                if (s.v[381] > 1e-16) {
                    A::sub(s.ad_value(381), A::scale(A::add(s.ad_value(381), A::sqrt(A::offset(A::mul(s.ad_value(381), s.ad_value(381)), 0.01))), 0.5))
                } else {
                    {
                        if ((-s.v[381]) > 1e-16) {
                            A::sub(s.ad_value(381), A::div_from_scalar((0.5 * 0.01), A::add(A::neg(s.ad_value(381)), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(381)), A::neg(s.ad_value(381))), 0.01)))))
                        } else {
                            A::sub(s.ad_value(381), A::scale(A::offset(s.ad_value(381), (((1e-32 + 0.01)) as f64).sqrt()), 0.5))
                        }
                    }
                }
            };
            s.store_ad(383, &assign6480_ad_e9234);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) {
            s.store_mul_ad_lhs(384, A::sqrt(A::offset(A::square(s.ad_value(118)), 1e-6)), 124);
        }

        s.v[399] = if (p.p59 < 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (s.v[399] != 0.0)) {
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
            s.store_ad(384, &assign6510_ad_e9323);
        }

        s.v[400] = if (0.0 == 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (s.v[400] != 0.0)) {
            s.store_add_ad(385, A::scale(s.ad_value(115), p.p17), A::scale(A::sub(s.ad_value(383), s.ad_value(134)), s.v[26]));
        }

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (!(s.v[400] != 0.0))) {
            s.store_add_ad(385, A::scale(s.ad_value(115), p.p17), A::scale(A::sub(s.ad_value(383), s.ad_value(93)), s.v[26]));
        }

        s.v[401] = if (s.v[385] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (s.v[401] != 0.0)) {
            s.store_ln_ad(390, A::offset(A::exp(s.ad_value(385)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (!(s.v[401] != 0.0))) {
            s.copy_ad(390, 385);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) {
            s.store_sub_ad_rhs(386, 385, A::scale(s.ad_value(127), (p.p17 * s.v[26])));
        }

        s.v[402] = if (s.v[386] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (s.v[402] != 0.0)) {
            s.store_ln_ad(387, A::offset(A::exp(s.ad_value(386)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (!(s.v[402] != 0.0))) {
            s.copy_ad(387, 386);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) {
            s.store_mul_ad_rhs(389, 123, A::offset(A::mul(s.ad_value(384), A::offset(A::scale(s.ad_value(384), p.p59), p.p58)), (-1.5)));
        }

        s.v[403] = if (((s.v[389]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (s.v[403] != 0.0)) {
            s.store_exp(388, 389);
        }

        s.v[404] = if (s.v[389] < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (!(s.v[403] != 0.0))) && (s.v[404] != 0.0)) {
            s.store_div_from_scalar_ad(388, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(389)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) && (!(s.v[403] != 0.0))) && (!(s.v[404] != 0.0))) {
            s.store_scale_ad(388, A::offset(A::mul(A::offset(s.ad_value(389), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(389), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(389), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((p.p49 != 0.0) && (s.v[380] != 0.0)) && (s.v[398] != 0.0)) {
            s.store_add_ad_rhs(5, 5, A::mul(A::scale(A::mul(s.ad_value(126), s.ad_value(388)), p.p17), A::sub(s.ad_value(390), s.ad_value(387))));
        }

        s.v[405] = if ((s.v[125] > 0.0) || (s.v[137] > 0.0)) { 1.0 } else { 0.0 };

        if ((p.p49 != 0.0) && (s.v[405] != 0.0)) {
            s.store_ad(128, &A::scale(A::voltage(ctx, &nodes, Some(4), Some(5)), p.p17));
        }

        if ((p.p49 != 0.0) && (s.v[405] != 0.0)) {
            s.store_scaled_sub(117, 78, 95, s.v[25]);
        }

        if ((p.p49 != 0.0) && (s.v[405] != 0.0)) {
            s.store_scalar(4, 0.0);
        }

        s.v[416] = if ((p.p18 == 1.0) && (s.v[137] > 0.0)) { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) {
            s.store_add_ad_lhs(407, A::scale(s.ad_value(117), p.p17), 129);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) {
            let assign6750_ad_e9682: A = {
                if ((-s.v[407]) > 1e-16) {
                    A::add(s.ad_value(407), A::scale(A::add(A::neg(s.ad_value(407)), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(407)), A::neg(s.ad_value(407))), 0.01))), 0.5))
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
            s.store_ad(408, &assign6750_ad_e9682);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) {
            s.store_mul_ad_lhs(409, A::sqrt(A::offset(A::square(s.ad_value(117)), 1e-6)), 131);
        }

        s.v[417] = if (p.p64 < 0.0) { 1.0 } else { 0.0 };

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
        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) {
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
            s.store_ad(409, &assign6780_ad_e9771);
        }

        s.v[418] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (s.v[418] != 0.0)) {
            s.store_neg_ad(410, A::add(A::scale(s.ad_value(95), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(134)), s.ad_value(408)), s.v[26])));
        }

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (!(s.v[418] != 0.0))) {
            s.store_neg_ad(410, A::add(A::scale(s.ad_value(95), p.p17), A::scale(A::add(A::sub_from_scalar(s.v[42], s.ad_value(93)), s.ad_value(408)), s.v[26])));
        }

        s.v[419] = if (s.v[410] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (s.v[419] != 0.0)) {
            s.store_ln_ad(415, A::offset(A::exp(s.ad_value(410)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (!(s.v[419] != 0.0))) {
            s.copy_ad(415, 410);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) {
            s.store_add_ad_rhs(411, 410, A::scale(s.ad_value(128), (p.p17 * s.v[26])));
        }

        s.v[420] = if (s.v[411] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (s.v[420] != 0.0)) {
            s.store_ln_ad(412, A::offset(A::exp(s.ad_value(411)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.copy_ad(412, 411);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) {
            s.store_mul_ad_rhs(414, 132, A::offset(A::mul(s.ad_value(409), A::offset(A::scale(s.ad_value(409), p.p64), p.p63)), (-1.5)));
        }

        s.v[421] = if (s.v[414] > 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (s.v[421] != 0.0)) {
            s.store_offset_ad(413, A::mul(s.ad_value(414), A::offset(A::mul(A::scale(s.ad_value(414), 0.5), A::offset(A::scale(s.ad_value(414), 0.3333333333333333), 1.0)), 1.0)), 1.0);
        }

        s.v[422] = if (s.v[414] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (!(s.v[421] != 0.0))) && (s.v[422] != 0.0)) {
            s.store_exp(413, 414);
        }

        if (((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) {
            s.store_div_from_scalar_ad(413, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[416] != 0.0)) {
            s.store_mul_ad(4, A::scale(A::mul(s.ad_value(137), s.ad_value(413)), p.p17), A::sub(s.ad_value(412), s.ad_value(415)));
        }

        s.v[423] = if (s.v[125] > 0.0) { 1.0 } else { 0.0 };

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) {
            s.store_add_ad_lhs(406, A::scale(s.ad_value(117), p.p17), 121);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) {
            let assign6980_ad_e10090: A = {
                if (s.v[406] > 1e-16) {
                    A::sub(s.ad_value(406), A::scale(A::add(s.ad_value(406), A::sqrt(A::offset(A::mul(s.ad_value(406), s.ad_value(406)), 0.01))), 0.5))
                } else {
                    {
                        if ((-s.v[406]) > 1e-16) {
                            A::sub(s.ad_value(406), A::div_from_scalar((0.5 * 0.01), A::add(A::neg(s.ad_value(406)), A::sqrt(A::offset(A::mul(A::neg(s.ad_value(406)), A::neg(s.ad_value(406))), 0.01)))))
                        } else {
                            A::sub(s.ad_value(406), A::scale(A::offset(s.ad_value(406), (((1e-32 + 0.01)) as f64).sqrt()), 0.5))
                        }
                    }
                }
            };
            s.store_ad(408, &assign6980_ad_e10090);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) {
            s.store_mul_ad_lhs(409, A::sqrt(A::offset(A::square(s.ad_value(117)), 1e-6)), 124);
        }

        s.v[424] = if (p.p59 < 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (s.v[424] != 0.0)) {
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
            s.store_ad(409, &assign7010_ad_e10179);
        }

        s.v[425] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (s.v[425] != 0.0)) {
            s.store_add_ad(410, A::scale(s.ad_value(95), p.p17), A::scale(A::sub(s.ad_value(408), s.ad_value(134)), s.v[26]));
        }

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (!(s.v[425] != 0.0))) {
            s.store_add_ad(410, A::scale(s.ad_value(95), p.p17), A::scale(A::sub(s.ad_value(408), s.ad_value(93)), s.v[26]));
        }

        s.v[426] = if (s.v[410] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (s.v[426] != 0.0)) {
            s.store_ln_ad(415, A::offset(A::exp(s.ad_value(410)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (!(s.v[426] != 0.0))) {
            s.copy_ad(415, 410);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) {
            s.store_sub_ad_rhs(411, 410, A::scale(s.ad_value(128), (p.p17 * s.v[26])));
        }

        s.v[427] = if (s.v[411] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (s.v[427] != 0.0)) {
            s.store_ln_ad(412, A::offset(A::exp(s.ad_value(411)), 1.0));
        }

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (!(s.v[427] != 0.0))) {
            s.copy_ad(412, 411);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) {
            s.store_mul_ad_rhs(414, 122, A::offset(A::mul(s.ad_value(409), A::offset(A::scale(s.ad_value(409), p.p59), p.p58)), (-1.5)));
        }

        s.v[428] = if (((s.v[414]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (s.v[428] != 0.0)) {
            s.store_exp(413, 414);
        }

        s.v[429] = if (s.v[414] < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (!(s.v[428] != 0.0))) && (s.v[429] != 0.0)) {
            s.store_div_from_scalar_ad(413, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(414)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) && (!(s.v[428] != 0.0))) && (!(s.v[429] != 0.0))) {
            s.store_scale_ad(413, A::offset(A::mul(A::offset(s.ad_value(414), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(414), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(414), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((p.p49 != 0.0) && (s.v[405] != 0.0)) && (s.v[423] != 0.0)) {
            s.store_add_ad_rhs(4, 4, A::mul(A::scale(A::mul(s.ad_value(125), s.ad_value(413)), p.p17), A::sub(s.ad_value(415), s.ad_value(412))));
        }

        s.store_scale_ad(3, A::mul(A::scale(A::sub(A::sub(s.ad_value(77), s.ad_value(96)), s.ad_value(99)), (s.v[23] * s.v[24])), s.ad_value(58)), p.p17);

        s.store_ad(105, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p22));

        s.store_ad(106, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(1)), s.v[61]));

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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        s.v[11] = ((3.453e-11 * (p.p20 / 3.9)) / p.p19);

        s.v[12] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p24)) as f64).sqrt() / s.v[11]);

        s.v[13] = ((((((2.0 * 1.6021918e-19) * 1.045e-10) * p.p29)) as f64).sqrt() / s.v[11]);

        s.v[144] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[144] != 0.0) {
            s.store_scalar(54, (((0.4 * 5.951993) * p.p30) * ((s.v[11]) as f64).powf(0.6666666666666666)));
        }

        s.v[145] = if (p.p17 < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[144] != 0.0) && (s.v[145] != 0.0)) {
            s.store_scale(54, 54, (7.448711 / 5.951993));
        }

        if (!(s.v[144] != 0.0)) {
            s.store_scalar(54, 0.0);
        }

        s.v[146] = if (p.p17 < 0.0) { 1.0 } else { 0.0 };

        if (s.v[146] != 0.0) {
            s.store_scalar(84, (0.3333333333333333 * p.p48));
        }

        if (!(s.v[146] != 0.0)) {
            s.store_scalar(84, (0.5 * p.p48));
        }

        s.v[141] = (p.p19 / 1e-9);

        s.v[16] = (if (p.p11 > (-273.0)) { p.p11 } else { (-273.0) });

        s.v[17] = (273.15 + s.v[16]);

        s.v[142] = ((ctx.temperature() + p.p3) - 273.15);

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

        s.v[157] = if (s.v[51] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (s.v[157] != 0.0) {
            s.store_scalar(53, (((-s.v[51])) as f64).exp());
        }

        if (!(s.v[157] != 0.0)) {
            s.store_scalar(53, (1e-200 / (1.0 + ((s.v[51] - 460.51701859880916) * (1.0 + ((0.5 * (s.v[51] - 460.51701859880916)) * (1.0 + ((s.v[51] - 460.51701859880916) * 0.3333333333333333))))))));
        }

        s.v[61] = (2.0 * ((p.p35 * s.v[22]) + (p.p34 * s.v[21])));

        let assign1480_ad_e1156: A = {
    if ((p.p17 * ((nv4 - nv5) - p.p27)) > 1e-16) {
        A::scale(A::add(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::sqrt(A::offset(A::mul(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17), A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17)), p.p28))), 0.5)
    } else {
        let assign1480_ad_e1155: A = {
            if ((-(p.p17 * ((nv4 - nv5) - p.p27))) > 1e-16) {
                A::div_from_scalar((0.5 * p.p28), A::add(A::neg(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17)), A::sqrt(A::offset(A::mul(A::neg(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17)), A::neg(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17))), p.p28))))
            } else {
                A::scale(A::offset(A::scale(A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-p.p27)), p.p17), (((1e-32 + p.p28)) as f64).sqrt()), 0.5)
            }
        };
        assign1480_ad_e1155
    }
};
        s.store_offset_ad(108, A::scale(assign1480_ad_e1156, p.p26), 1.0);

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

        s.store_offset_ad(47, A::scale(A::ln(A::scale(s.ad_value(107), s.v[10])), (2.0 * s.v[25])), s.v[42]);

        s.store_scale_ad(12, A::sqrt(A::scale(s.ad_value(107), ((2.0 * 1.6021918e-19) * 1.045e-10))), 1.0 / (s.v[11]));

        s.v[161] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[161] != 0.0) {
            s.store_sqrt_ad(55, A::mul(A::square(s.ad_value(12)), s.ad_value(47)));
        }

        if (s.v[161] != 0.0) {
            s.store_mul_ad(56, A::scale(s.ad_value(54), 0.75), A::powf(s.ad_value(55), 0.6666666666666666));
        }

        if (s.v[161] != 0.0) {
            s.store_add(47, 47, 56);
        }

        if (s.v[161] != 0.0) {
            s.store_mul_ad_rhs(12, 12, A::offset(A::div(A::scale(s.ad_value(56), (2.0 * 0.6666666666666666)), s.ad_value(55)), 1.0));
        }

        s.v[6] = ((s.v[26]) as f64).sqrt();

        s.store_scale(34, 12, s.v[6]);

        s.store_square(36, 34);

        s.store_div_from_scalar(37, 1.0, 36);

        s.store_offset_scaled(43, 34, 0.7071067811865475, 1.0);

        s.store_div_from_scalar(44, 1.0, 43);

        s.store_scale(40, 43, 1e-5);

        s.store_scale(50, 47, s.v[26]);

        s.v[162] = if (s.v[50] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (s.v[162] != 0.0) {
            s.store_exp_ad(52, A::neg(s.ad_value(50)));
        }

        if (!(s.v[162] != 0.0)) {
            s.store_div_from_scalar_ad(52, 1e-200, A::offset(A::mul(A::offset(s.ad_value(50), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(50), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(50), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        s.store_offset_scaled(60, 34, (((((((-1.25)) as f64).exp() + 1.25) - 1.0)) as f64).sqrt(), 1.25);

        s.store_scale_ad(77, A::offset(A::voltage(ctx, &nodes, Some(4), Some(5)), (-s.v[28])), p.p17);

        s.store_scale(78, 77, s.v[26]);

        s.v[184] = if (((s.v[78]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if (s.v[184] != 0.0) {
            s.store_scale_ad(165, A::square(s.ad_value(44)), (0.1666666666666667 * 0.7071067811865475));
        }

        if (s.v[184] != 0.0) {
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(165)), 1.0));
        }

        s.v[185] = if (s.v[78] < (-s.v[40])) { 1.0 } else { 0.0 };

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_neg(166, 78);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_mul_ad_lhs(167, A::scale(s.ad_value(166), 1.25), 44);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_scale_ad(174, A::sub(A::offset(s.ad_value(167), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(167), (-6.0)), A::offset(s.ad_value(167), (-6.0))), 64.0))), 0.5);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub(164, 166, 174);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad(169, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::offset(s.ad_value(174), 1.0)));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad_lhs(171, A::scale(s.ad_value(164), 2.0), 36);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad_lhs(173, A::ln(A::mul(s.ad_value(169), s.ad_value(37))), 174);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add(186, 169, 171);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad(187, A::square(s.ad_value(186)), A::mul(A::sub(A::mul(A::scale(s.ad_value(171), 0.5), s.ad_value(171)), s.ad_value(169)), s.ad_value(173)));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad_rhs(168, 174, A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(186)), s.ad_value(173)), A::add(s.ad_value(187), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(186), s.ad_value(173)), s.ad_value(173)), s.ad_value(187)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), s.ad_value(169))))));
        }

        s.v[188] = if (s.v[168] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) && (s.v[188] != 0.0)) {
            s.store_exp(175, 168);
        }

        if (((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) && (!(s.v[188] != 0.0))) {
            s.store_scale_ad(175, A::offset(A::mul(A::offset(s.ad_value(168), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(168), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(168), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_div_from_scalar(176, 1.0, 175);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_div_from_scalar_ad(164, 1.0, A::offset(A::square(s.ad_value(168)), 2.0));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub(164, 166, 168);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_mul(165, 52, 176);
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_add_ad(177, A::scale(s.ad_value(164), 2.0), A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(175), (-1.0)), s.ad_value(165)), s.ad_value(52))));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(175), s.ad_value(168)), (-1.0)), s.ad_value(165)), A::mul(s.ad_value(52), A::offset(s.ad_value(168), (-1.0))))));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(175), s.ad_value(165))));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul(A::scale(s.ad_value(178), 2.0), s.ad_value(164)));
        }

        if ((!(s.v[184] != 0.0)) && (s.v[185] != 0.0)) {
            s.store_sub_ad(79, A::neg(s.ad_value(168)), A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_div_from_scalar_ad(163, 1.0, A::offset(A::scale(s.ad_value(34), 0.7324648775608221), 1.25));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_mul_ad_lhs(179, A::offset(A::mul(A::scale(s.ad_value(43), 1.25), s.ad_value(163)), (-1.0)), 163);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_mul_ad(182, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(179), s.ad_value(78)), 1.0));
        }

        s.v[189] = if ((-s.v[182]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[189] != 0.0)) {
            s.store_exp_ad(164, A::neg(s.ad_value(182)));
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[189] != 0.0))) {
            s.store_div_from_scalar_ad(164, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(182))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_from_scalar(181, 1.0, 164);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_ad(180, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(181)))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_offset(172, 50, 3.0);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
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

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub(164, 78, 174);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_exp_ad(165, A::neg(s.ad_value(174)));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_max_from_scalar_ad(169, 1e-40, A::sub(A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(165), s.ad_value(174)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(174), 1.0))))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_from_scalar_ad(170, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(165)));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad(171, A::scale(s.ad_value(164), 2.0), A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(165)), s.ad_value(52))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad(173, A::sub(s.ad_value(50), s.ad_value(174)), A::ln(A::div(s.ad_value(169), s.ad_value(36))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add(190, 169, 171);
        }

        s.v[192] = if (((s.v[173]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[192] != 0.0)) {
            s.copy_ad(183, 174);
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[192] != 0.0))) {
            s.store_add_ad(191, A::square(s.ad_value(190)), A::mul(A::sub(A::mul(A::scale(s.ad_value(171), 0.5), s.ad_value(171)), A::mul(s.ad_value(169), s.ad_value(170))), s.ad_value(173)));
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[192] != 0.0))) {
            let assign2190_ad_e2144: A = A::add(s.ad_value(174), A::div(A::mul(A::mul(s.ad_value(169), s.ad_value(190)), s.ad_value(173)), A::add(s.ad_value(191), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(190), s.ad_value(173)), s.ad_value(173)), s.ad_value(191)), s.ad_value(171)), A::sub(A::scale(A::square(s.ad_value(171)), 0.3333333333333333), A::mul(s.ad_value(169), s.ad_value(170)))))));
            s.store_ad(183, &assign2190_ad_e2144);
        }

        s.v[193] = if (s.v[183] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[193] != 0.0)) {
            s.store_exp(175, 183);
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[193] != 0.0)) {
            s.store_div_from_scalar(176, 1.0, 175);
        }

        if (((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (s.v[193] != 0.0)) {
            s.store_mul(175, 52, 175);
        }

        s.v[194] = if (s.v[183] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (s.v[194] != 0.0)) {
            s.store_exp_ad(175, A::sub(s.ad_value(183), s.ad_value(50)));
        }

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (s.v[194] != 0.0)) {
            s.store_div(176, 52, 175);
        }

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (!(s.v[194] != 0.0))) {
            s.store_div_from_scalar_ad(175, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(183)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) && (!(s.v[193] != 0.0))) && (!(s.v[194] != 0.0))) {
            s.store_div_from_scalar_ad(176, 1e-100, A::offset(A::mul(A::offset(s.ad_value(183), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(183), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(183), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_div_from_scalar_ad(164, 1.0, A::offset(A::square(s.ad_value(183)), 2.0));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub(164, 78, 183);
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad(177, A::scale(s.ad_value(164), 2.0), A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(176)), s.ad_value(175)), s.ad_value(52))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_ad(178, A::square(s.ad_value(164)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(176), s.ad_value(183)), (-1.0)), s.ad_value(175)), A::mul(s.ad_value(52), A::offset(s.ad_value(183), 1.0)))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_from_scalar_ad(164, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(176), s.ad_value(175))));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_sub_ad(164, A::square(s.ad_value(177)), A::mul(A::scale(s.ad_value(178), 2.0), s.ad_value(164)));
        }

        if ((!(s.v[184] != 0.0)) && (!(s.v[185] != 0.0))) {
            s.store_add_ad_rhs(79, 183, A::div(A::scale(s.ad_value(178), 2.0), A::add(s.ad_value(177), A::sqrt(s.ad_value(164)))));
        }

        s.v[195] = if (p.p29 < 1e27) { 1.0 } else { 0.0 };

        if (s.v[195] != 0.0) {
            s.store_scale_ad(80, A::sub(s.ad_value(77), A::scale(s.ad_value(79), s.v[25])), (((-p.p17) * p.p18) * s.v[26]));
        }

        s.v[217] = if (((s.v[80]) as f64).abs() <= s.v[41]) { 1.0 } else { 0.0 };

        if ((s.v[195] != 0.0) && (s.v[217] != 0.0)) {
            s.store_scalar(198, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
        }

        if ((s.v[195] != 0.0) && (s.v[217] != 0.0)) {
            s.store_mul_ad(81, A::scale(s.ad_value(80), s.v[46]), A::offset(A::mul(A::scale(A::mul(s.ad_value(80), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35]), s.ad_value(198)), 1.0));
        }

        s.v[218] = if (s.v[80] < (-s.v[41])) { 1.0 } else { 0.0 };

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_neg(199, 80);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_scale(200, 199, (1.25 * s.v[46]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_scale_ad(207, A::sub(A::offset(s.ad_value(200), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(200), (-6.0)), A::offset(s.ad_value(200), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub(197, 199, 207);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad(202, A::square(s.ad_value(197)), A::scale(A::offset(s.ad_value(207), 1.0), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_offset_scaled(204, 197, 2.0, (-s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad_lhs(206, A::ln(A::scale(s.ad_value(202), s.v[39])), 207);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add(219, 202, 204);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad(220, A::square(s.ad_value(219)), A::mul(A::sub(A::mul(A::scale(s.ad_value(204), 0.5), s.ad_value(204)), s.ad_value(202)), s.ad_value(206)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad_rhs(201, 207, A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(219)), s.ad_value(206)), A::add(s.ad_value(220), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(219), s.ad_value(206)), s.ad_value(206)), s.ad_value(220)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), s.ad_value(202))))));
        }

        s.v[221] = if (s.v[201] < 230.25850929940458) { 1.0 } else { 0.0 };

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
        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) && (s.v[221] != 0.0)) {
            s.store_exp(208, 201);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) && (!(s.v[221] != 0.0))) {
            s.store_scale_ad(208, A::offset(A::mul(A::offset(s.ad_value(201), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(201), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(201), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_div_from_scalar(209, 1.0, 208);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_div_from_scalar_ad(197, 1.0, A::offset(A::square(s.ad_value(201)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub(197, 199, 201);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_mul(198, 53, 209);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_add_ad(210, A::scale(s.ad_value(197), 2.0), A::scale(A::add(A::sub(A::offset(s.ad_value(208), (-1.0)), s.ad_value(198)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(208), s.ad_value(201)), (-1.0)), s.ad_value(198)), A::mul(s.ad_value(53), A::offset(s.ad_value(201), (-1.0)))), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(208), s.ad_value(198)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul(A::scale(s.ad_value(211), 2.0), s.ad_value(197)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (s.v[218] != 0.0)) {
            s.store_sub_ad(81, A::neg(s.ad_value(201)), A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_scalar(196, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_mul_ad_lhs(212, A::offset(A::scale(s.ad_value(196), (s.v[45] * 1.25)), (-1.0)), 196);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_mul_ad(215, A::scale(s.ad_value(80), s.v[46]), A::offset(A::mul(s.ad_value(212), s.ad_value(80)), 1.0));
        }

        s.v[222] = if ((-s.v[215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[222] != 0.0)) {
            s.store_exp_ad(197, A::neg(s.ad_value(215)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[222] != 0.0))) {
            s.store_div_from_scalar_ad(197, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(215))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_from_scalar(214, 1.0, 197);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_ad(213, A::offset(s.ad_value(80), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(80), (s.v[38] * 0.25)), s.ad_value(214))), s.v[35]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_scalar(205, (s.v[51] + 3.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
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

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub(197, 80, 207);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_exp_ad(198, A::neg(s.ad_value(207)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_max_from_scalar_ad(202, 1e-40, A::sub(A::square(s.ad_value(197)), A::scale(A::sub(A::offset(A::add(s.ad_value(198), s.ad_value(207)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(207), 1.0))), s.v[38])));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_from_scalar_ad(203, 1.0, A::scale(s.ad_value(198), (0.5 * s.v[38])));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad(204, A::scale(s.ad_value(197), 2.0), A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(198)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad(206, A::sub_from_scalar(s.v[51], s.ad_value(207)), A::ln(A::scale(s.ad_value(202), 1.0 / (s.v[38]))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add(223, 202, 204);
        }

        s.v[225] = if (((s.v[206]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[225] != 0.0)) {
            s.copy_ad(216, 207);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[225] != 0.0))) {
            s.store_add_ad(224, A::square(s.ad_value(223)), A::mul(A::sub(A::mul(A::scale(s.ad_value(204), 0.5), s.ad_value(204)), A::mul(s.ad_value(202), s.ad_value(203))), s.ad_value(206)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[225] != 0.0))) {
            let assign2840_ad_e3281: A = A::add(s.ad_value(207), A::div(A::mul(A::mul(s.ad_value(202), s.ad_value(223)), s.ad_value(206)), A::add(s.ad_value(224), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(223), s.ad_value(206)), s.ad_value(206)), s.ad_value(224)), s.ad_value(204)), A::sub(A::scale(A::square(s.ad_value(204)), 0.3333333333333333), A::mul(s.ad_value(202), s.ad_value(203)))))));
            s.store_ad(216, &assign2840_ad_e3281);
        }

        s.v[226] = if (s.v[216] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[226] != 0.0)) {
            s.store_exp(208, 216);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[226] != 0.0)) {
            s.store_div_from_scalar(209, 1.0, 208);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (s.v[226] != 0.0)) {
            s.store_mul(208, 53, 208);
        }

        s.v[227] = if (s.v[216] > (s.v[51] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (s.v[227] != 0.0)) {
            s.store_exp_ad(208, A::offset(s.ad_value(216), (-s.v[51])));
        }

        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (s.v[227] != 0.0)) {
            s.store_div(209, 53, 208);
        }

        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (!(s.v[227] != 0.0))) {
            s.store_div_from_scalar_ad(208, 1e-100, A::offset(A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) && (!(s.v[226] != 0.0))) && (!(s.v[227] != 0.0))) {
            s.store_div_from_scalar_ad(209, 1e-100, A::offset(A::mul(A::offset(s.ad_value(216), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(216), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(216), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_div_from_scalar_ad(197, 1.0, A::offset(A::square(s.ad_value(216)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub(197, 80, 216);
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad(210, A::scale(s.ad_value(197), 2.0), A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(209)), s.ad_value(208)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_ad(211, A::square(s.ad_value(197)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(209), s.ad_value(216)), (-1.0)), s.ad_value(208)), A::mul(s.ad_value(53), A::offset(s.ad_value(216), 1.0))), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_from_scalar_ad(197, 2.0, A::scale(A::add(s.ad_value(209), s.ad_value(208)), s.v[38]));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_sub_ad(197, A::square(s.ad_value(210)), A::mul(A::scale(s.ad_value(211), 2.0), s.ad_value(197)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[217] != 0.0))) && (!(s.v[218] != 0.0))) {
            s.store_add_ad_rhs(81, 216, A::div(A::scale(s.ad_value(211), 2.0), A::add(s.ad_value(210), A::sqrt(s.ad_value(197)))));
        }

        if (s.v[195] != 0.0) {
            s.store_scale(82, 81, (((-p.p17) * p.p18) * s.v[25]));
        }

        if (s.v[195] != 0.0) {
            s.store_scaled_sub(78, 77, 82, 1.0 / (s.v[25]));
        }

        s.v[249] = if (((s.v[78]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if ((s.v[195] != 0.0) && (s.v[249] != 0.0)) {
            s.store_scale_ad(230, A::square(s.ad_value(44)), (0.1666666666666667 * 0.7071067811865475));
        }

        if ((s.v[195] != 0.0) && (s.v[249] != 0.0)) {
            s.store_mul_ad(79, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(A::mul(A::mul(s.ad_value(78), A::sub_from_scalar(1.0, s.ad_value(52))), s.ad_value(34)), s.ad_value(230)), 1.0));
        }

        s.v[250] = if (s.v[78] < (-s.v[40])) { 1.0 } else { 0.0 };

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_neg(231, 78);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_mul_ad_lhs(232, A::scale(s.ad_value(231), 1.25), 44);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_scale_ad(239, A::sub(A::offset(s.ad_value(232), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(232), (-6.0)), A::offset(s.ad_value(232), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub(229, 231, 239);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad(234, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::offset(s.ad_value(239), 1.0)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad_lhs(236, A::scale(s.ad_value(229), 2.0), 36);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad_lhs(238, A::ln(A::mul(s.ad_value(234), s.ad_value(37))), 239);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add(251, 234, 236);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad(252, A::square(s.ad_value(251)), A::mul(A::sub(A::mul(A::scale(s.ad_value(236), 0.5), s.ad_value(236)), s.ad_value(234)), s.ad_value(238)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad_rhs(233, 239, A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(251)), s.ad_value(238)), A::add(s.ad_value(252), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(251), s.ad_value(238)), s.ad_value(238)), s.ad_value(252)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), s.ad_value(234))))));
        }

        s.v[253] = if (s.v[233] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) && (s.v[253] != 0.0)) {
            s.store_exp(240, 233);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) && (!(s.v[253] != 0.0))) {
            s.store_scale_ad(240, A::offset(A::mul(A::offset(s.ad_value(233), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(233), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(233), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_div_from_scalar(241, 1.0, 240);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_div_from_scalar_ad(229, 1.0, A::offset(A::square(s.ad_value(233)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub(229, 231, 233);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_mul(230, 52, 241);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_add_ad(242, A::scale(s.ad_value(229), 2.0), A::mul(s.ad_value(36), A::add(A::sub(A::offset(s.ad_value(240), (-1.0)), s.ad_value(230)), s.ad_value(52))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::add(A::add(A::offset(A::sub(s.ad_value(240), s.ad_value(233)), (-1.0)), s.ad_value(230)), A::mul(s.ad_value(52), A::offset(s.ad_value(233), (-1.0))))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(240), s.ad_value(230))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul(A::scale(s.ad_value(243), 2.0), s.ad_value(229)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (s.v[250] != 0.0)) {
            s.store_sub_ad(79, A::neg(s.ad_value(233)), A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_div_from_scalar_ad(228, 1.0, A::offset(A::scale(s.ad_value(34), 0.7324648775608221), 1.25));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_mul_ad_lhs(244, A::offset(A::mul(A::scale(s.ad_value(43), 1.25), s.ad_value(228)), (-1.0)), 228);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_mul_ad(247, A::mul(s.ad_value(78), s.ad_value(44)), A::offset(A::mul(s.ad_value(244), s.ad_value(78)), 1.0));
        }

        s.v[254] = if ((-s.v[247]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[254] != 0.0)) {
            s.store_exp_ad(229, A::neg(s.ad_value(247)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(247))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_from_scalar(246, 1.0, 229);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_ad(245, A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(78), A::scale(s.ad_value(36), 0.25)), s.ad_value(246)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_offset(237, 50, 3.0);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
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

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub(229, 78, 239);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_exp_ad(230, A::neg(s.ad_value(239)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_max_from_scalar_ad(234, 1e-40, A::sub(A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::offset(A::add(s.ad_value(230), s.ad_value(239)), (-1.0)), A::mul(s.ad_value(52), A::offset(s.ad_value(239), 1.0))))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_from_scalar_ad(235, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(230)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad(236, A::scale(s.ad_value(229), 2.0), A::mul(s.ad_value(36), A::sub(A::sub_from_scalar(1.0, s.ad_value(230)), s.ad_value(52))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad(238, A::sub(s.ad_value(50), s.ad_value(239)), A::ln(A::div(s.ad_value(234), s.ad_value(36))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add(255, 234, 236);
        }

        s.v[257] = if (((s.v[238]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[257] != 0.0)) {
            s.copy_ad(248, 239);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_add_ad(256, A::square(s.ad_value(255)), A::mul(A::sub(A::mul(A::scale(s.ad_value(236), 0.5), s.ad_value(236)), A::mul(s.ad_value(234), s.ad_value(235))), s.ad_value(238)));
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[257] != 0.0))) {
            let assign3490_ad_e4447: A = A::add(s.ad_value(239), A::div(A::mul(A::mul(s.ad_value(234), s.ad_value(255)), s.ad_value(238)), A::add(s.ad_value(256), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(255), s.ad_value(238)), s.ad_value(238)), s.ad_value(256)), s.ad_value(236)), A::sub(A::scale(A::square(s.ad_value(236)), 0.3333333333333333), A::mul(s.ad_value(234), s.ad_value(235)))))));
            s.store_ad(248, &assign3490_ad_e4447);
        }

        s.v[258] = if (s.v[248] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[258] != 0.0)) {
            s.store_exp(240, 248);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[258] != 0.0)) {
            s.store_div_from_scalar(241, 1.0, 240);
        }

        if ((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (s.v[258] != 0.0)) {
            s.store_mul(240, 52, 240);
        }

        s.v[259] = if (s.v[248] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (s.v[259] != 0.0)) {
            s.store_exp_ad(240, A::sub(s.ad_value(248), s.ad_value(50)));
        }

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (s.v[259] != 0.0)) {
            s.store_div(241, 52, 240);
        }

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (!(s.v[259] != 0.0))) {
            s.store_div_from_scalar_ad(240, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(248)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) && (!(s.v[258] != 0.0))) && (!(s.v[259] != 0.0))) {
            s.store_div_from_scalar_ad(241, 1e-100, A::offset(A::mul(A::offset(s.ad_value(248), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(248), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(248), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1.0, A::offset(A::square(s.ad_value(248)), 2.0));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub(229, 78, 248);
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad(242, A::scale(s.ad_value(229), 2.0), A::mul(s.ad_value(36), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(241)), s.ad_value(240)), s.ad_value(52))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_ad(243, A::square(s.ad_value(229)), A::mul(s.ad_value(36), A::sub(A::add(A::offset(A::add(s.ad_value(241), s.ad_value(248)), (-1.0)), s.ad_value(240)), A::mul(s.ad_value(52), A::offset(s.ad_value(248), 1.0)))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_from_scalar_ad(229, 2.0, A::mul(s.ad_value(36), A::add(s.ad_value(241), s.ad_value(240))));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_sub_ad(229, A::square(s.ad_value(242)), A::mul(A::scale(s.ad_value(243), 2.0), s.ad_value(229)));
        }

        if (((s.v[195] != 0.0) && (!(s.v[249] != 0.0))) && (!(s.v[250] != 0.0))) {
            s.store_add_ad_rhs(79, 248, A::div(A::scale(s.ad_value(243), 2.0), A::add(s.ad_value(242), A::sqrt(s.ad_value(229)))));
        }

        if (!(s.v[195] != 0.0)) {
            s.store_scalar(82, 0.0);
        }

        s.v[260] = if ((s.v[78] <= 0.0) || (p.p21 < 1.0)) { 1.0 } else { 0.0 };

        if (!(s.v[260] != 0.0)) {
            s.store_scalar(83, 0.0);
        }

        s.v[261] = if (s.v[79] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(s.v[260] != 0.0)) && (s.v[261] != 0.0)) {
            s.store_exp(83, 79);
        }

        if ((!(s.v[260] != 0.0)) && (s.v[261] != 0.0)) {
            s.store_div_from_scalar(85, 1.0, 83);
        }

        if ((!(s.v[260] != 0.0)) && (s.v[261] != 0.0)) {
            s.store_mul(83, 52, 83);
        }

        s.v[262] = if (s.v[79] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (s.v[262] != 0.0)) {
            s.store_exp_ad(83, A::sub(s.ad_value(79), s.ad_value(50)));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (s.v[262] != 0.0)) {
            s.store_div(85, 52, 83);
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (!(s.v[262] != 0.0))) {
            s.store_div_from_scalar_ad(83, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub(s.ad_value(50), s.ad_value(79)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((!(s.v[260] != 0.0)) && (!(s.v[261] != 0.0))) && (!(s.v[262] != 0.0))) {
            s.store_div_from_scalar_ad(85, 1e-100, A::offset(A::mul(A::offset(s.ad_value(79), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(79), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(79), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        s.v[263] = if (s.v[79] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(s.v[260] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_mul_ad(86, A::mul(A::scale(s.ad_value(79), 0.5), s.ad_value(79)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(79), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25)))));
        }

        if ((!(s.v[260] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_sqrt_ad(6, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(79), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(79), 0.25)))));
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
        if ((!(s.v[260] != 0.0)) && (s.v[263] != 0.0)) {
            s.store_mul_ad_lhs(88, A::scale(s.ad_value(79), 0.7071067811865475), 6);
        }

        if ((!(s.v[260] != 0.0)) && (!(s.v[263] != 0.0))) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(79), (-1.0)), 85);
        }

        if ((!(s.v[260] != 0.0)) && (!(s.v[263] != 0.0))) {
            s.store_sqrt(88, 86);
        }

        s.store_scale_ad(94, A::add(s.ad_value(77), A::voltage(ctx, &nodes, Some(6), None)), s.v[26]);

        s.v[281] = if (((s.v[94]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if (s.v[281] != 0.0) {
            s.store_div(95, 94, 43);
        }

        s.v[282] = if (s.v[94] > s.v[40]) { 1.0 } else { 0.0 };

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_div_ad_lhs(276, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_mul_ad(277, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(276), s.ad_value(94)), 1.0));
        }

        s.v[283] = if (s.v[277] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (s.v[283] != 0.0)) {
            s.store_exp_ad(275, A::neg(s.ad_value(277)));
        }

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (!(s.v[283] != 0.0))) {
            s.store_div_from_scalar_ad(275, 1e-200, A::offset(A::mul(A::offset(s.ad_value(277), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(277), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(277), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_from_scalar(278, 1.0, 275);
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_ad(279, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(278)))));
        }

        s.v[284] = if (s.v[279] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (s.v[284] != 0.0)) {
            s.store_exp_ad(271, A::neg(s.ad_value(279)));
        }

        if (((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) && (!(s.v[284] != 0.0))) {
            s.store_div_from_scalar_ad(271, 1e-200, A::offset(A::mul(A::offset(s.ad_value(279), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(279), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(279), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(271)));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(94), s.ad_value(279)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(271))));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_ad(274, A::mul(A::sub(s.ad_value(94), s.ad_value(279)), A::sub(s.ad_value(94), s.ad_value(279))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(279), (-1.0)), s.ad_value(271))));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul(A::scale(s.ad_value(272), 4.0), s.ad_value(274)));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_div_ad(280, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
        }

        if ((!(s.v[281] != 0.0)) && (s.v[282] != 0.0)) {
            s.store_add(95, 279, 280);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_neg(264, 94);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_div_ad_lhs(265, A::scale(s.ad_value(264), 1.25), 43);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_scale_ad(266, A::sub(A::offset(s.ad_value(265), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(265), (-6.0)), A::offset(s.ad_value(265), (-6.0))), 64.0))), 0.5);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(267, A::mul(A::sub(s.ad_value(264), s.ad_value(266)), A::sub(s.ad_value(264), s.ad_value(266))), A::mul(s.ad_value(36), A::offset(s.ad_value(266), 1.0)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_ad_lhs(268, A::scale(A::sub(s.ad_value(264), s.ad_value(266)), 2.0), 36);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_ad_lhs(269, A::ln(A::div(s.ad_value(267), s.ad_value(36))), 266);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add(285, 267, 268);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(286, A::square(s.ad_value(285)), A::mul(A::sub(A::mul(A::scale(s.ad_value(268), 0.5), s.ad_value(268)), s.ad_value(267)), s.ad_value(269)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad_rhs(270, 266, A::div(A::mul(A::mul(s.ad_value(267), s.ad_value(285)), s.ad_value(269)), A::add(s.ad_value(286), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(285), s.ad_value(269)), s.ad_value(269)), s.ad_value(286)), s.ad_value(268)), A::sub(A::scale(A::square(s.ad_value(268)), 0.3333333333333333), s.ad_value(267))))));
        }

        s.v[287] = if (((s.v[270]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) && (s.v[287] != 0.0)) {
            s.store_exp(271, 270);
        }

        s.v[288] = if (s.v[270] < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) && (!(s.v[287] != 0.0))) && (s.v[288] != 0.0)) {
            s.store_div_from_scalar_ad(271, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(270)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if ((((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_scale_ad(271, A::offset(A::mul(A::offset(s.ad_value(270), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(270), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(270), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_from_scalar_ad(272, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(271)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(273, A::scale(A::sub(s.ad_value(264), s.ad_value(270)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(271), (-1.0))));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_add_ad(274, A::mul(A::sub(s.ad_value(264), s.ad_value(270)), A::sub(s.ad_value(264), s.ad_value(270))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(270), 1.0), s.ad_value(271))));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_sub_ad(275, A::square(s.ad_value(273)), A::mul(A::scale(s.ad_value(272), 4.0), s.ad_value(274)));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_div_ad(278, A::scale(s.ad_value(274), 2.0), A::add(s.ad_value(273), A::sqrt(s.ad_value(275))));
        }

        if ((!(s.v[281] != 0.0)) && (!(s.v[282] != 0.0))) {
            s.store_neg_ad(95, A::add(s.ad_value(270), s.ad_value(278)));
        }

        s.store_scale(96, 95, s.v[25]);

        s.v[289] = if (p.p29 < 1e27) { 1.0 } else { 0.0 };

        if (s.v[289] != 0.0) {
            s.store_scale_ad(97, A::sub(s.ad_value(77), A::scale(s.ad_value(95), s.v[25])), (((-p.p17) * p.p18) * s.v[26]));
        }

        s.v[311] = if (((s.v[97]) as f64).abs() <= s.v[41]) { 1.0 } else { 0.0 };

        if ((s.v[289] != 0.0) && (s.v[311] != 0.0)) {
            s.store_scalar(292, (((s.v[46] * s.v[46]) * 0.1666666666666667) * 0.7071067811865475));
        }

        if ((s.v[289] != 0.0) && (s.v[311] != 0.0)) {
            s.store_mul_ad(98, A::scale(s.ad_value(97), s.v[46]), A::offset(A::mul(A::scale(A::mul(s.ad_value(97), A::sub_from_scalar(1.0, s.ad_value(53))), s.v[35]), s.ad_value(292)), 1.0));
        }

        s.v[312] = if (s.v[97] < (-s.v[41])) { 1.0 } else { 0.0 };

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_neg(293, 97);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_scale(294, 293, (1.25 * s.v[46]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_scale_ad(301, A::sub(A::offset(s.ad_value(294), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(294), (-6.0)), A::offset(s.ad_value(294), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub(291, 293, 301);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad(296, A::square(s.ad_value(291)), A::scale(A::offset(s.ad_value(301), 1.0), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_offset_scaled(298, 291, 2.0, (-s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad_lhs(300, A::ln(A::scale(s.ad_value(296), s.v[39])), 301);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add(313, 296, 298);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad(314, A::square(s.ad_value(313)), A::mul(A::sub(A::mul(A::scale(s.ad_value(298), 0.5), s.ad_value(298)), s.ad_value(296)), s.ad_value(300)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad_rhs(295, 301, A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(313)), s.ad_value(300)), A::add(s.ad_value(314), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(313), s.ad_value(300)), s.ad_value(300)), s.ad_value(314)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), s.ad_value(296))))));
        }

        s.v[315] = if (s.v[295] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) && (s.v[315] != 0.0)) {
            s.store_exp(302, 295);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) && (!(s.v[315] != 0.0))) {
            s.store_scale_ad(302, A::offset(A::mul(A::offset(s.ad_value(295), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(295), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(295), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_div_from_scalar(303, 1.0, 302);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_div_from_scalar_ad(291, 1.0, A::offset(A::square(s.ad_value(295)), 2.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub(291, 293, 295);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_mul(292, 53, 303);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_add_ad(304, A::scale(s.ad_value(291), 2.0), A::scale(A::add(A::sub(A::offset(s.ad_value(302), (-1.0)), s.ad_value(292)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::add(A::add(A::offset(A::sub(s.ad_value(302), s.ad_value(295)), (-1.0)), s.ad_value(292)), A::mul(s.ad_value(53), A::offset(s.ad_value(295), (-1.0)))), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(302), s.ad_value(292)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul(A::scale(s.ad_value(305), 2.0), s.ad_value(291)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sub_ad(98, A::neg(s.ad_value(295)), A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_scalar(290, (1.0 / (1.25 + (s.v[35] * 0.7324648775608221))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_mul_ad_lhs(306, A::offset(A::scale(s.ad_value(290), (s.v[45] * 1.25)), (-1.0)), 290);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_mul_ad(309, A::scale(s.ad_value(97), s.v[46]), A::offset(A::mul(s.ad_value(306), s.ad_value(97)), 1.0));
        }

        s.v[316] = if ((-s.v[309]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[316] != 0.0)) {
            s.store_exp_ad(291, A::neg(s.ad_value(309)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[316] != 0.0))) {
            s.store_div_from_scalar_ad(291, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(309))), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_from_scalar(308, 1.0, 291);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_ad(307, A::offset(s.ad_value(97), (s.v[38] * 0.5)), A::scale(A::sqrt(A::sub(A::offset(s.ad_value(97), (s.v[38] * 0.25)), s.ad_value(308))), s.v[35]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_scalar(299, (s.v[51] + 3.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
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

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub(291, 97, 301);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_exp_ad(292, A::neg(s.ad_value(301)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_max_from_scalar_ad(296, 1e-40, A::sub(A::square(s.ad_value(291)), A::scale(A::sub(A::offset(A::add(s.ad_value(292), s.ad_value(301)), (-1.0)), A::mul(s.ad_value(53), A::offset(s.ad_value(301), 1.0))), s.v[38])));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_from_scalar_ad(297, 1.0, A::scale(s.ad_value(292), (0.5 * s.v[38])));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(298, A::scale(s.ad_value(291), 2.0), A::scale(A::sub(A::sub_from_scalar(1.0, s.ad_value(292)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(300, A::sub_from_scalar(s.v[51], s.ad_value(301)), A::ln(A::scale(s.ad_value(296), 1.0 / (s.v[38]))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add(317, 296, 298);
        }

        s.v[319] = if (((s.v[300]) as f64).abs() < 1e-120) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[319] != 0.0)) {
            s.copy_ad(310, 301);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[319] != 0.0))) {
            s.store_add_ad(318, A::square(s.ad_value(317)), A::mul(A::sub(A::mul(A::scale(s.ad_value(298), 0.5), s.ad_value(298)), A::mul(s.ad_value(296), s.ad_value(297))), s.ad_value(300)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[319] != 0.0))) {
            let assign4810_ad_e6543: A = A::add(s.ad_value(301), A::div(A::mul(A::mul(s.ad_value(296), s.ad_value(317)), s.ad_value(300)), A::add(s.ad_value(318), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(317), s.ad_value(300)), s.ad_value(300)), s.ad_value(318)), s.ad_value(298)), A::sub(A::scale(A::square(s.ad_value(298)), 0.3333333333333333), A::mul(s.ad_value(296), s.ad_value(297)))))));
            s.store_ad(310, &assign4810_ad_e6543);
        }

        s.v[320] = if (s.v[310] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[320] != 0.0)) {
            s.store_exp(302, 310);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[320] != 0.0)) {
            s.store_div_from_scalar(303, 1.0, 302);
        }

        if ((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (s.v[320] != 0.0)) {
            s.store_mul(302, 53, 302);
        }

        s.v[321] = if (s.v[310] > (s.v[51] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (s.v[321] != 0.0)) {
            s.store_exp_ad(302, A::offset(s.ad_value(310), (-s.v[51])));
        }

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (s.v[321] != 0.0)) {
            s.store_div(303, 53, 302);
        }

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (!(s.v[321] != 0.0))) {
            s.store_div_from_scalar_ad(302, 1e-100, A::offset(A::mul(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(A::sub_from_scalar(s.v[51], s.ad_value(310)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) && (!(s.v[320] != 0.0))) && (!(s.v[321] != 0.0))) {
            s.store_div_from_scalar_ad(303, 1e-100, A::offset(A::mul(A::offset(s.ad_value(310), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(310), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(310), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_div_from_scalar_ad(291, 1.0, A::offset(A::square(s.ad_value(310)), 2.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub(291, 97, 310);
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad(304, A::scale(s.ad_value(291), 2.0), A::scale(A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(303)), s.ad_value(302)), s.ad_value(53)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_ad(305, A::square(s.ad_value(291)), A::scale(A::sub(A::add(A::offset(A::add(s.ad_value(303), s.ad_value(310)), (-1.0)), s.ad_value(302)), A::mul(s.ad_value(53), A::offset(s.ad_value(310), 1.0))), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_from_scalar_ad(291, 2.0, A::scale(A::add(s.ad_value(303), s.ad_value(302)), s.v[38]));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_sub_ad(291, A::square(s.ad_value(304)), A::mul(A::scale(s.ad_value(305), 2.0), s.ad_value(291)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[311] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_add_ad_rhs(98, 310, A::div(A::scale(s.ad_value(305), 2.0), A::add(s.ad_value(304), A::sqrt(s.ad_value(291)))));
        }

        if (s.v[289] != 0.0) {
            s.store_scale(99, 98, (((-p.p17) * p.p18) * s.v[25]));
        }

        if (s.v[289] != 0.0) {
            s.store_scale_ad(94, A::sub(A::add(s.ad_value(77), A::voltage(ctx, &nodes, Some(6), None)), s.ad_value(99)), 1.0 / (s.v[25]));
        }

        s.v[339] = if (((s.v[94]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if ((s.v[289] != 0.0) && (s.v[339] != 0.0)) {
            s.store_div(95, 94, 43);
        }

        s.v[340] = if (s.v[94] > s.v[40]) { 1.0 } else { 0.0 };

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_div_ad_lhs(334, A::offset(A::div(A::scale(s.ad_value(43), 1.25), s.ad_value(60)), (-1.0)), 60);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_mul_ad(335, A::div(s.ad_value(94), s.ad_value(43)), A::offset(A::mul(s.ad_value(334), s.ad_value(94)), 1.0));
        }

        s.v[341] = if (s.v[335] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (s.v[341] != 0.0)) {
            s.store_exp_ad(333, A::neg(s.ad_value(335)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (!(s.v[341] != 0.0))) {
            s.store_div_from_scalar_ad(333, 1e-200, A::offset(A::mul(A::offset(s.ad_value(335), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(335), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(335), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_from_scalar(336, 1.0, 333);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_ad(337, A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.5)), A::mul(s.ad_value(34), A::sqrt(A::sub(A::add(s.ad_value(94), A::scale(s.ad_value(36), 0.25)), s.ad_value(336)))));
        }

        s.v[342] = if (s.v[337] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (s.v[342] != 0.0)) {
            s.store_exp_ad(329, A::neg(s.ad_value(337)));
        }

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) && (!(s.v[342] != 0.0))) {
            s.store_div_from_scalar_ad(329, 1e-200, A::offset(A::mul(A::offset(s.ad_value(337), (-460.51701859880916)), A::offset(A::mul(A::scale(A::offset(s.ad_value(337), (-460.51701859880916)), 0.5), A::offset(A::scale(A::offset(s.ad_value(337), (-460.51701859880916)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(329)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(94), s.ad_value(337)), 2.0), A::mul(s.ad_value(36), A::sub_from_scalar(1.0, s.ad_value(329))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_ad(332, A::mul(A::sub(s.ad_value(94), s.ad_value(337)), A::sub(s.ad_value(94), s.ad_value(337))), A::mul(s.ad_value(36), A::add(A::offset(s.ad_value(337), (-1.0)), s.ad_value(329))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul(A::scale(s.ad_value(330), 4.0), s.ad_value(332)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_div_ad(338, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_add(95, 337, 338);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_neg(322, 94);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_div_ad_lhs(323, A::scale(s.ad_value(322), 1.25), 43);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_scale_ad(324, A::sub(A::offset(s.ad_value(323), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(323), (-6.0)), A::offset(s.ad_value(323), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(325, A::mul(A::sub(s.ad_value(322), s.ad_value(324)), A::sub(s.ad_value(322), s.ad_value(324))), A::mul(s.ad_value(36), A::offset(s.ad_value(324), 1.0)));
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
        let nv6 = ctx.node_voltage(nodes[6]);
        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_ad_lhs(326, A::scale(A::sub(s.ad_value(322), s.ad_value(324)), 2.0), 36);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_ad_lhs(327, A::ln(A::div(s.ad_value(325), s.ad_value(36))), 324);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add(343, 325, 326);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(344, A::square(s.ad_value(343)), A::mul(A::sub(A::mul(A::scale(s.ad_value(326), 0.5), s.ad_value(326)), s.ad_value(325)), s.ad_value(327)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad_rhs(328, 324, A::div(A::mul(A::mul(s.ad_value(325), s.ad_value(343)), s.ad_value(327)), A::add(s.ad_value(344), A::mul(A::mul(A::div(A::mul(A::mul(s.ad_value(343), s.ad_value(327)), s.ad_value(327)), s.ad_value(344)), s.ad_value(326)), A::sub(A::scale(A::square(s.ad_value(326)), 0.3333333333333333), s.ad_value(325))))));
        }

        s.v[345] = if (((s.v[328]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) && (s.v[345] != 0.0)) {
            s.store_exp(329, 328);
        }

        s.v[346] = if (s.v[328] < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) && (!(s.v[345] != 0.0))) && (s.v[346] != 0.0)) {
            s.store_div_from_scalar_ad(329, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), A::offset(A::mul(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.5), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(328)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        if (((((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) && (!(s.v[345] != 0.0))) && (!(s.v[346] != 0.0))) {
            s.store_scale_ad(329, A::offset(A::mul(A::offset(s.ad_value(328), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(328), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(328), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0), 1e100);
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_from_scalar_ad(330, 1.0, A::mul(A::scale(s.ad_value(36), 0.5), s.ad_value(329)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(331, A::scale(A::sub(s.ad_value(322), s.ad_value(328)), 2.0), A::mul(s.ad_value(36), A::offset(s.ad_value(329), (-1.0))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_add_ad(332, A::mul(A::sub(s.ad_value(322), s.ad_value(328)), A::sub(s.ad_value(322), s.ad_value(328))), A::mul(s.ad_value(36), A::sub(A::offset(s.ad_value(328), 1.0), s.ad_value(329))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_sub_ad(333, A::square(s.ad_value(331)), A::mul(A::scale(s.ad_value(330), 4.0), s.ad_value(332)));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_div_ad(336, A::scale(s.ad_value(332), 2.0), A::add(s.ad_value(331), A::sqrt(s.ad_value(333))));
        }

        if (((s.v[289] != 0.0) && (!(s.v[339] != 0.0))) && (!(s.v[340] != 0.0))) {
            s.store_neg_ad(95, A::add(s.ad_value(328), s.ad_value(336)));
        }

        if (s.v[289] != 0.0) {
            s.store_scale(96, 95, s.v[25]);
        }

        if (!(s.v[289] != 0.0)) {
            s.store_scalar(99, 0.0);
        }

        s.v[83] = 0.0;

        s.v[347] = if (s.v[95] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (s.v[347] != 0.0) {
            s.store_exp(83, 95);
        }

        if (s.v[347] != 0.0) {
            s.store_div_from_scalar(85, 1.0, 83);
        }

        s.v[348] = if (s.v[95] > (s.v[50] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((!(s.v[347] != 0.0)) && (s.v[348] != 0.0)) {
            s.store_exp_ad(83, A::sub(s.ad_value(50), s.ad_value(95)));
        }

        if ((!(s.v[347] != 0.0)) && (s.v[348] != 0.0)) {
            s.store_mul(85, 52, 83);
        }

        if ((!(s.v[347] != 0.0)) && (!(s.v[348] != 0.0))) {
            s.store_div_from_scalar_ad(85, 1e-100, A::offset(A::mul(A::offset(s.ad_value(95), (-230.25850929940458)), A::offset(A::mul(A::scale(A::offset(s.ad_value(95), (-230.25850929940458)), 0.5), A::offset(A::scale(A::offset(s.ad_value(95), (-230.25850929940458)), 0.3333333333333333), 1.0)), 1.0)), 1.0));
        }

        s.v[349] = if (s.v[95] < (-s.v[40])) { 1.0 } else { 0.0 };

        if (s.v[349] != 0.0) {
            s.store_offset_ad(86, A::add(s.ad_value(85), s.ad_value(95)), (-1.0));
        }

        if (s.v[349] != 0.0) {
            s.store_neg_ad(88, A::sqrt(s.ad_value(86)));
        }

        s.v[350] = if (((s.v[95]) as f64).abs() <= s.v[40]) { 1.0 } else { 0.0 };

        if ((!(s.v[349] != 0.0)) && (s.v[350] != 0.0)) {
            s.store_sub_from_scalar_ad(6, 1.0, A::mul(A::scale(s.ad_value(95), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(95), 0.25))));
        }

        if ((!(s.v[349] != 0.0)) && (s.v[350] != 0.0)) {
            s.store_mul_ad_lhs(86, A::mul(A::scale(s.ad_value(95), 0.5), s.ad_value(95)), 6);
        }

        if ((!(s.v[349] != 0.0)) && (s.v[350] != 0.0)) {
            s.store_mul_ad(88, A::scale(s.ad_value(95), 0.7071067811865475), A::sqrt(s.ad_value(6)));
        }

        if ((!(s.v[349] != 0.0)) && (!(s.v[350] != 0.0))) {
            s.store_add_ad_lhs(86, A::offset(s.ad_value(95), (-1.0)), 85);
        }

        if ((!(s.v[349] != 0.0)) && (!(s.v[350] != 0.0))) {
            s.store_sqrt(88, 86);
        }

        s.store_mul_ad_lhs(91, A::scale(s.ad_value(88), s.v[25]), 34);

        s.store_scale_ad(139, A::mul(A::scale(A::offset(s.ad_value(140), 1.0), 1.62), A::offset(s.ad_value(140), 1.0)), ((1.0 + (0.37 * s.v[141])) * ((1.0 + (0.37 * s.v[141])) * (s.v[20] * (((s.v[20]) as f64).sqrt() * (s.v[25] * s.v[25]))))));

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
        let assign5600_ad_e7830: A = A::add(A::voltage(ctx, &nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None)), A::sqrt(A::add(A::mul(A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None)), A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None))), s.ad_value(139)))), 0.5));
        assign5600_ad_e7830
    } else {
        let assign5600_ad_e7873: A = {
            if ((nv6 - (-nv6)) > 1e-16) {
                let assign5600_ad_e7858: A = A::div(A::scale(s.ad_value(139), 0.5), A::add(A::sub(A::voltage(ctx, &nodes, Some(6), None), A::neg(A::voltage(ctx, &nodes, Some(6), None))), A::sqrt(A::add(A::mul(A::sub(A::voltage(ctx, &nodes, Some(6), None), A::neg(A::voltage(ctx, &nodes, Some(6), None))), A::sub(A::voltage(ctx, &nodes, Some(6), None), A::neg(A::voltage(ctx, &nodes, Some(6), None)))), s.ad_value(139)))));
                A::add(A::voltage(ctx, &nodes, Some(6), None), assign5600_ad_e7858)
            } else {
                A::add(A::voltage(ctx, &nodes, Some(6), None), A::scale(A::add(A::sub(A::neg(A::voltage(ctx, &nodes, Some(6), None)), A::voltage(ctx, &nodes, Some(6), None)), A::sqrt(A::offset(s.ad_value(139), 1e-32))), 0.5))
            }
        };
        assign5600_ad_e7873
    }
};
        s.store_add_ad(59, assign5600_ad_e7802, A::mul(s.ad_value(84), assign5600_ad_e7874));

        s.v[58] = s.v[11];

        s.v[351] = if (s.v[54] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[351] != 0.0) {
            s.store_div_from_scalar_ad(58, s.v[11], A::offset(A::mul(s.ad_value(54), A::powf(A::offset(A::square(s.ad_value(59)), s.v[57]), ((-1.0) * 0.1666666666666667))), 1.0));
        }

        s.store_scale_ad(3, A::mul(A::scale(A::sub(A::sub(s.ad_value(77), s.ad_value(96)), s.ad_value(99)), (s.v[23] * s.v[24])), s.ad_value(58)), p.p17);

        s.store_ad(105, &A::scale(A::voltage(ctx, &nodes, Some(6), None), p.p22));

        s.store_ad(106, &A::scale(A::voltage(ctx, &nodes, Some(3), Some(1)), s.v[61]));

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
        let nv6 = ctx.node_voltage(nodes[6]);
        let eq0_value: f64 = (nv6 - 0.0);
        stamper.stamp_current(
            Some(nodes[6]),
            None,
            self.multiplicity * (eq0_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * 1.0),
            ],
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv3 = ctx.node_voltage(nodes[3]);
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
        stamper.stamp_current_dense(
            Some(nodes[0]),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
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
        stamper.stamp_current_dense(
            Some(nodes[3]),
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
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
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[1]),
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
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
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
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
        let (eq5_e101,) = {
    if (!(p.p16 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e101;
        stamper.stamp_potential(
            branches[0],
            eq5_value,
            &[
            ],
        );
    }
}
