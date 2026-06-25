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
        s.v[1] = (8.8541878176e-12 * 11.8);

        s.v[112] = 0.0;

        s.v[187] = if (p.p62 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[187] != 0.0) {
            s.store_scalar(112, 1.0);
        }

        if (!(s.v[187] != 0.0)) {
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

        s.v[188] = if ((((p.p56 != 1.0) || (p.p57 != 1.0)) || (p.p58 != 1.0)) || (p.p59 != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[188] != 0.0) {
            s.store_scalar(111, 1.0);
        }

        if (!(s.v[188] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        s.v[189] = if (s.v[111] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[189] != 0.0) {
            s.store_scalar(95, (if ((p.p17 * p.p56) > 1e-18) { (p.p17 * p.p56) } else { 1e-18 }));
        }

        if (s.v[189] != 0.0) {
            s.store_scalar(96, (if ((p.p20 * p.p57) > 0.05) { (p.p20 * p.p57) } else { 0.05 }));
        }

        if (s.v[189] != 0.0) {
            s.store_scalar(97, (if ((if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) < 0.95) { (if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) } else { 0.95 }));
        }

        if (s.v[189] != 0.0) {
            s.store_scalar(98, (p.p26 * p.p59));
        }

        if (s.v[189] != 0.0) {
            s.store_offset(100, 98, s.v[13]);
        }

        if (s.v[189] != 0.0) {
            s.store_sub_from_scalar(105, 1.0, 97);
        }

        if (s.v[189] != 0.0) {
            s.store_div_from_scalar(106, 1.0, 105);
        }

        s.v[3] = (((ctx.temperature() + p.p2) + p.p9)).max((273.15 + (-250.0)));

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

        if !(s.v[79] > 0.0) {
            s.store_scalar(79, 0.0);
        }

        if !(s.v[80] > 0.0) {
            s.store_scalar(80, 0.0);
        }

        if !(s.v[81] > 0.0) {
            s.store_scalar(81, 0.0);
        }

        s.v[190] = if (s.v[111] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[190] != 0.0) {
            s.store_offset(99, 98, s.v[14]);
        }

        if (s.v[190] != 0.0) {
            s.store_scale_ad(101, A::exp(A::scale(A::sub(A::scale(s.ad_value(100), s.v[7]), A::scale(s.ad_value(99), s.v[9])), 0.5)), ((s.v[4]) as f64).powf(1.5));
        }

        if (s.v[190] != 0.0) {
            s.store_sub_ad(102, A::scale(s.ad_value(96), s.v[4]), A::scale(A::ln(s.ad_value(101)), (2.0 * s.v[8])));
        }

        if (s.v[190] != 0.0) {
            s.store_add_ad_rhs(103, 102, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(102)), s.v[9])), 1.0)), s.v[8]));
        }

        if (s.v[190] != 0.0) {
            s.store_div_from_scalar(104, 1.0, 103);
        }

        if (s.v[190] != 0.0) {
            s.store_mul_ad_rhs(107, 95, A::pow(A::mul(s.ad_value(96), s.ad_value(104)), s.ad_value(97)));
        }

        if (s.v[190] != 0.0) {
            s.store_mul_ad_lhs(108, A::mul(s.ad_value(107), s.ad_value(103)), 106);
        }

        if (s.v[190] != 0.0) {
            s.store_scale(109, 107, 2.0);
        }

        s.v[143] = (if (p.p3 > 0.0) { p.p3 } else { 0.0 });

        s.v[144] = (if (p.p4 > 0.0) { p.p4 } else { 0.0 });

        s.v[145] = (if (p.p5 > 0.0) { p.p5 } else { 0.0 });

        s.v[0] = (if (p.p6 > 0.0) { p.p6 } else { 0.0 });

        s.v[150] = 0.0;

        s.v[191] = if ((s.v[25] * s.v[143]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[191] != 0.0) {
            s.store_scalar(92, (s.v[8] * ((((p.p12 / (s.v[25] * s.v[143])) + 1.0)) as f64).ln()));
        }

        if (!(s.v[191] != 0.0)) {
            s.store_scalar(92, 100000000.0);
        }

        s.v[192] = if ((s.v[26] * s.v[144]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[192] != 0.0) {
            s.store_scalar(93, (s.v[8] * ((((p.p12 / (s.v[26] * s.v[144])) + 1.0)) as f64).ln()));
        }

        if (!(s.v[192] != 0.0)) {
            s.store_scalar(93, 100000000.0);
        }

        s.v[193] = if ((s.v[27] * s.v[145]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[193] != 0.0) {
            s.store_scalar(94, (s.v[8] * ((((p.p12 / (s.v[27] * s.v[145])) + 1.0)) as f64).ln()));
        }

        if (!(s.v[193] != 0.0)) {
            s.store_scalar(94, 100000000.0);
        }

        s.store_ad(149, &A::min(A::min(s.ad_value(92), s.ad_value(93)), s.ad_value(94)));

        s.v[194] = if ((((s.v[149] * s.v[9])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (s.v[194] != 0.0) {
            s.store_exp_ad(150, A::scale(s.ad_value(149), s.v[9]));
        }

        s.v[195] = if ((s.v[149] * s.v[9]) < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[194] != 0.0)) && (s.v[195] != 0.0)) {
            s.store_div_from_scalar_ad(150, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[194] != 0.0)) && (!(s.v[195] != 0.0))) {
            s.store_scale_ad(150, A::offset(A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
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

        s.v[196] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[196] != 0.0) {
            s.store_scalar(34, (s.v[32] + s.v[33]));
        }

        if (s.v[196] != 0.0) {
            s.store_scalar(37, (0.9 * (p.p22).min(p.p23)));
        }

        if (s.v[196] != 0.0) {
            s.store_scalar(40, (p.p19 + p.p20));
        }

        s.v[197] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[197] != 0.0) {
            s.store_scalar(35, (s.v[31] + s.v[33]));
        }

        if (s.v[197] != 0.0) {
            s.store_scalar(38, (0.9 * (p.p21).min(p.p23)));
        }

        if (s.v[197] != 0.0) {
            s.store_scalar(41, (p.p18 + p.p20));
        }

        s.v[198] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[198] != 0.0) {
            s.store_scalar(36, (s.v[31] + s.v[32]));
        }

        if (s.v[198] != 0.0) {
            s.store_scalar(39, (0.9 * (p.p21).min(p.p22)));
        }

        if (s.v[198] != 0.0) {
            s.store_scalar(42, (p.p18 + p.p19));
        }

        s.store_ad(151, &A::min(A::min(s.ad_value(34), s.ad_value(35)), s.ad_value(36)));

        s.store_scale(152, 151, 0.1);

        s.store_ad(15, &A::max(A::max(s.ad_value(37), s.ad_value(38)), s.ad_value(39)));

        s.store_mul_ad_rhs(153, 151, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(15)))));

        s.store_offset_ad(154, A::min(A::min(s.ad_value(40), s.ad_value(41)), s.ad_value(42)), (-0.05));

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

        s.v[199] = if (s.v[112] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[199] != 0.0) {
            s.store_scalar(200, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(201, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(202, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(209, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(211, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(212, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(213, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(214, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(215, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(216, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(217, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(218, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(219, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(220, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(221, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(222, 0.0);
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
        if (s.v[199] != 0.0) {
            s.store_scalar(223, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(224, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(225, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(226, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(227, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(228, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(229, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(230, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(231, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(232, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(233, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(234, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(235, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(236, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(237, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(238, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(239, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(240, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(241, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(242, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(243, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(244, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(136, 0.4);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(137, 0.65);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(138, 0.8);
        }

        if (s.v[199] != 0.0) {
            s.store_scale_ad(123, A::neg(s.ad_value(136)), p.p63);
        }

        if (s.v[199] != 0.0) {
            s.store_scale_ad(124, A::neg(s.ad_value(137)), p.p63);
        }

        if (s.v[199] != 0.0) {
            s.store_scale_ad(125, A::neg(s.ad_value(138)), p.p63);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(126, 0.1);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(127, 0.2);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(216, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(213, 0.0);
        }

        s.v[248] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        s.v[249] = if (s.v[123] < s.v[149]) { 1.0 } else { 0.0 };

        s.v[250] = if (((((-0.5) * (s.v[123] * s.v[9]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (s.v[249] != 0.0)) && (s.v[250] != 0.0)) {
            s.store_exp_ad(211, A::scale(s.ad_value(123), (s.v[9] * (-0.5))));
        }

        s.v[251] = if (((-0.5) * (s.v[123] * s.v[9])) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (s.v[249] != 0.0)) && (!(s.v[250] != 0.0))) && (s.v[251] != 0.0)) {
            let assign2500_ad_e1541: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(123), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(123), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(123), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(211, &assign2500_ad_e1541);
        }

        if (((((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (s.v[249] != 0.0)) && (!(s.v[250] != 0.0))) && (!(s.v[251] != 0.0))) {
            s.store_scale_ad(211, A::offset(A::mul(A::offset(A::scale(s.ad_value(123), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(123), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(123), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (s.v[249] != 0.0)) {
            s.store_div_from_scalar(212, 1.0, 211);
        }

        if (((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (s.v[249] != 0.0)) {
            s.store_square(209, 212);
        }

        if (((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (!(s.v[249] != 0.0))) {
            s.store_mul_ad_lhs(209, A::offset(A::scale(A::sub(s.ad_value(123), s.ad_value(149)), s.v[9]), 1.0), 150);
        }

        if (((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (!(s.v[249] != 0.0))) {
            s.store_sqrt(212, 209);
        }

        if (((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (!(s.v[249] != 0.0))) {
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if ((s.v[199] != 0.0) && (s.v[248] != 0.0)) {
            s.store_offset(209, 209, (-1.0));
        }

        s.v[252] = if (s.v[123] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (s.v[252] != 0.0)) {
            s.store_scale_ad(213, A::ln(A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0))))), (s.v[8] * 2.0));
        }

        if (((s.v[199] != 0.0) && (s.v[248] != 0.0)) && (!(s.v[252] != 0.0))) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 123);
        }

        if ((s.v[199] != 0.0) && (s.v[248] != 0.0)) {
            s.store_sub(214, 151, 213);
        }

        if ((s.v[199] != 0.0) && (s.v[248] != 0.0)) {
            s.store_scale_ad(215, A::sub(A::add(s.ad_value(123), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(123), s.ad_value(214)), A::sub(s.ad_value(123), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[248] != 0.0)) {
            s.store_scale_ad(216, A::sub(A::add(s.ad_value(123), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(123), s.ad_value(154)), A::sub(s.ad_value(123), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[248] != 0.0)) {
            s.store_scale_ad(217, A::sub(s.ad_value(123), A::sqrt(A::offset(A::mul(s.ad_value(123), s.ad_value(123)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[253] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[253] != 0.0)) {
            s.store_scalar(245, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.v[254] = if ((p.p30 == 0.0) && (p.p35 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (s.v[254] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[31], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[255] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) && (s.v[255] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) && (!(s.v[255] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p21)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[256] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) && (s.v[256] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) && (!(s.v[256] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_scale(225, 218, s.v[61]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[22]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[254] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.v[257] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (s.v[257] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[46]), s.ad_value(221)), s.v[76]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[258] = if (((-p.p21) * s.v[49]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (s.v[258] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (!(s.v[258] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[259] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (s.v[259] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (!(s.v[259] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[260] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (s.v[260] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (!(s.v[260] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[261] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (s.v[261] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[262] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (!(s.v[261] != 0.0))) && (s.v[262] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (!(s.v[261] != 0.0))) && (!(s.v[262] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) && (!(s.v[261] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[73]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[257] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p35);
        }

        s.v[263] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (s.v[263] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[264] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[263] != 0.0))) && (s.v[264] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[263] != 0.0))) && (!(s.v[264] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[263] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), s.ad_value(218)), s.v[49]);
        }

        s.v[265] = if (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[263] != 0.0))) && (s.v[265] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.v[266] = if (((-s.v[79]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[263] != 0.0))) && (!(s.v[265] != 0.0))) && (s.v[266] != 0.0)) {
            let assign3230_ad_e2648: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign3230_ad_e2648);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[263] != 0.0))) && (!(s.v[265] != 0.0))) && (!(s.v[266] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[263] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(123), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p41);
        }

        s.v[267] = if (p.p50 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (s.v[267] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[268] = if (s.v[217] > ((-s.v[82]) * p.p50)) { 1.0 } else { 0.0 };

        s.v[269] = if (p.p53 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[267] != 0.0))) && (s.v[268] != 0.0)) && (s.v[269] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[267] != 0.0))) && (s.v[268] != 0.0)) && (!(s.v[269] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[267] != 0.0))) && (s.v[268] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) && (!(s.v[267] != 0.0))) && (!(s.v[268] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89]), s.v[83]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[253] != 0.0))) {
            s.store_mul_ad_lhs(245, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[270] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[270] != 0.0)) {
            s.store_scalar(246, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.v[271] = if ((p.p31 == 0.0) && (p.p36 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (s.v[271] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[32], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[272] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) && (s.v[272] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) && (!(s.v[272] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p22)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[273] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) && (s.v[273] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) && (!(s.v[273] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) {
            s.store_scale(225, 218, s.v[62]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[23]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[271] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.v[274] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (s.v[274] != 0.0)) {
            s.store_scalar(227, 0.0);
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
        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[47]), s.ad_value(221)), s.v[77]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[275] = if (((-p.p22) * s.v[50]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (s.v[275] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (!(s.v[275] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[276] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (s.v[276] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (!(s.v[276] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[277] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (s.v[277] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (!(s.v[277] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[278] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (s.v[278] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[279] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (!(s.v[278] != 0.0))) && (s.v[279] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (!(s.v[278] != 0.0))) && (!(s.v[279] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) && (!(s.v[278] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[74]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[274] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p36);
        }

        s.v[280] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (s.v[280] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[281] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[280] != 0.0))) && (s.v[281] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[280] != 0.0))) && (!(s.v[281] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[280] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), s.ad_value(218)), s.v[50]);
        }

        s.v[282] = if (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[280] != 0.0))) && (s.v[282] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.v[283] = if (((-s.v[80]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[280] != 0.0))) && (!(s.v[282] != 0.0))) && (s.v[283] != 0.0)) {
            let assign3930_ad_e3685: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign3930_ad_e3685);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[280] != 0.0))) && (!(s.v[282] != 0.0))) && (!(s.v[283] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[280] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(123), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p42);
        }

        s.v[284] = if (p.p51 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (s.v[284] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[285] = if (s.v[217] > ((-s.v[82]) * p.p51)) { 1.0 } else { 0.0 };

        s.v[286] = if (p.p54 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[284] != 0.0))) && (s.v[285] != 0.0)) && (s.v[286] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[284] != 0.0))) && (s.v[285] != 0.0)) && (!(s.v[286] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[284] != 0.0))) && (s.v[285] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) && (!(s.v[284] != 0.0))) && (!(s.v[285] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90]), s.v[84]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[270] != 0.0))) {
            s.store_mul_ad_lhs(246, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[287] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[287] != 0.0)) {
            s.store_scalar(247, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.v[288] = if ((p.p32 == 0.0) && (p.p37 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (s.v[288] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[33], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[289] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) && (s.v[289] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) && (!(s.v[289] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p23)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[290] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) && (s.v[290] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) && (!(s.v[290] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_scale(225, 218, s.v[63]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[24]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[288] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.v[291] = if (p.p37 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (s.v[291] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[48]), s.ad_value(221)), s.v[78]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[292] = if (((-p.p23) * s.v[51]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (s.v[292] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (!(s.v[292] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[293] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (s.v[293] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (!(s.v[293] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[294] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (s.v[294] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (!(s.v[294] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[295] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (s.v[295] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[296] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (!(s.v[295] != 0.0))) && (s.v[296] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (!(s.v[295] != 0.0))) && (!(s.v[296] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) && (!(s.v[295] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[75]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[291] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p37);
        }

        s.v[297] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (s.v[297] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[298] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[297] != 0.0))) && (s.v[298] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[297] != 0.0))) && (!(s.v[298] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[297] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), s.ad_value(218)), s.v[51]);
        }

        s.v[299] = if (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[297] != 0.0))) && (s.v[299] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.v[300] = if (((-s.v[81]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[297] != 0.0))) && (!(s.v[299] != 0.0))) && (s.v[300] != 0.0)) {
            let assign4630_ad_e4722: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign4630_ad_e4722);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[297] != 0.0))) && (!(s.v[299] != 0.0))) && (!(s.v[300] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[297] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(123), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p43);
        }

        s.v[301] = if (p.p52 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (s.v[301] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[302] = if (s.v[217] > ((-s.v[82]) * p.p52)) { 1.0 } else { 0.0 };

        s.v[303] = if (p.p55 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[301] != 0.0))) && (s.v[302] != 0.0)) && (s.v[303] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[301] != 0.0))) && (s.v[302] != 0.0)) && (!(s.v[303] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[301] != 0.0))) && (s.v[302] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) && (!(s.v[301] != 0.0))) && (!(s.v[302] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91]), s.v[85]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[287] != 0.0))) {
            s.store_mul_ad_lhs(247, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        if (s.v[199] != 0.0) {
            s.store_add_ad(113, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), A::scale(s.ad_value(247), s.v[145]));
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(216, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(213, 0.0);
        }

        s.v[304] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        s.v[305] = if (s.v[124] < s.v[149]) { 1.0 } else { 0.0 };

        s.v[306] = if (((((-0.5) * (s.v[124] * s.v[9]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (s.v[305] != 0.0)) && (s.v[306] != 0.0)) {
            s.store_exp_ad(211, A::scale(s.ad_value(124), (s.v[9] * (-0.5))));
        }

        s.v[307] = if (((-0.5) * (s.v[124] * s.v[9])) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (s.v[305] != 0.0)) && (!(s.v[306] != 0.0))) && (s.v[307] != 0.0)) {
            let assign4900_ad_e5103: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(124), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(124), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(124), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(211, &assign4900_ad_e5103);
        }

        if (((((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (s.v[305] != 0.0)) && (!(s.v[306] != 0.0))) && (!(s.v[307] != 0.0))) {
            s.store_scale_ad(211, A::offset(A::mul(A::offset(A::scale(s.ad_value(124), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(124), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(124), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (s.v[305] != 0.0)) {
            s.store_div_from_scalar(212, 1.0, 211);
        }

        if (((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (s.v[305] != 0.0)) {
            s.store_square(209, 212);
        }

        if (((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (!(s.v[305] != 0.0))) {
            s.store_mul_ad_lhs(209, A::offset(A::scale(A::sub(s.ad_value(124), s.ad_value(149)), s.v[9]), 1.0), 150);
        }

        if (((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (!(s.v[305] != 0.0))) {
            s.store_sqrt(212, 209);
        }

        if (((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (!(s.v[305] != 0.0))) {
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if ((s.v[199] != 0.0) && (s.v[304] != 0.0)) {
            s.store_offset(209, 209, (-1.0));
        }

        s.v[308] = if (s.v[124] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (s.v[308] != 0.0)) {
            s.store_scale_ad(213, A::ln(A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0))))), (s.v[8] * 2.0));
        }

        if (((s.v[199] != 0.0) && (s.v[304] != 0.0)) && (!(s.v[308] != 0.0))) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 124);
        }

        if ((s.v[199] != 0.0) && (s.v[304] != 0.0)) {
            s.store_sub(214, 151, 213);
        }

        if ((s.v[199] != 0.0) && (s.v[304] != 0.0)) {
            s.store_scale_ad(215, A::sub(A::add(s.ad_value(124), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(124), s.ad_value(214)), A::sub(s.ad_value(124), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[304] != 0.0)) {
            s.store_scale_ad(216, A::sub(A::add(s.ad_value(124), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(124), s.ad_value(154)), A::sub(s.ad_value(124), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[304] != 0.0)) {
            s.store_scale_ad(217, A::sub(s.ad_value(124), A::sqrt(A::offset(A::mul(s.ad_value(124), s.ad_value(124)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
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
        s.v[309] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[309] != 0.0)) {
            s.store_scalar(245, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.v[310] = if ((p.p30 == 0.0) && (p.p35 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (s.v[310] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[31], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[311] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) && (s.v[311] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) && (!(s.v[311] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p21)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[312] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) && (s.v[312] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) && (!(s.v[312] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) {
            s.store_scale(225, 218, s.v[61]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[22]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[310] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.v[313] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (s.v[313] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[46]), s.ad_value(221)), s.v[76]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[314] = if (((-p.p21) * s.v[49]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (s.v[314] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (!(s.v[314] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[315] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (s.v[315] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (!(s.v[315] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[316] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (s.v[316] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (!(s.v[316] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[317] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (s.v[317] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[318] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (!(s.v[317] != 0.0))) && (s.v[318] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (!(s.v[317] != 0.0))) && (!(s.v[318] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) && (!(s.v[317] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[73]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[313] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p35);
        }

        s.v[319] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (s.v[319] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[320] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[319] != 0.0))) && (s.v[320] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[319] != 0.0))) && (!(s.v[320] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[319] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), s.ad_value(218)), s.v[49]);
        }

        s.v[321] = if (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[319] != 0.0))) && (s.v[321] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.v[322] = if (((-s.v[79]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[319] != 0.0))) && (!(s.v[321] != 0.0))) && (s.v[322] != 0.0)) {
            let assign5630_ad_e6210: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign5630_ad_e6210);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[319] != 0.0))) && (!(s.v[321] != 0.0))) && (!(s.v[322] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[319] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(124), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p41);
        }

        s.v[323] = if (p.p50 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (s.v[323] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[324] = if (s.v[217] > ((-s.v[82]) * p.p50)) { 1.0 } else { 0.0 };

        s.v[325] = if (p.p53 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[323] != 0.0))) && (s.v[324] != 0.0)) && (s.v[325] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[323] != 0.0))) && (s.v[324] != 0.0)) && (!(s.v[325] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[323] != 0.0))) && (s.v[324] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) && (!(s.v[323] != 0.0))) && (!(s.v[324] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89]), s.v[83]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[309] != 0.0))) {
            s.store_mul_ad_lhs(245, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[326] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[326] != 0.0)) {
            s.store_scalar(246, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.v[327] = if ((p.p31 == 0.0) && (p.p36 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (s.v[327] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[32], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[328] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) && (s.v[328] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) && (!(s.v[328] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p22)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[329] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) && (s.v[329] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) && (!(s.v[329] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) {
            s.store_scale(225, 218, s.v[62]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[23]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[327] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.v[330] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (s.v[330] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[47]), s.ad_value(221)), s.v[77]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[331] = if (((-p.p22) * s.v[50]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (s.v[331] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (!(s.v[331] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[332] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (s.v[332] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (!(s.v[332] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[333] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (s.v[333] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (!(s.v[333] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[334] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (s.v[334] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[335] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (!(s.v[334] != 0.0))) && (s.v[335] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (!(s.v[334] != 0.0))) && (!(s.v[335] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) && (!(s.v[334] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[74]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[330] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p36);
        }

        s.v[336] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (s.v[336] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[337] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[336] != 0.0))) && (s.v[337] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[336] != 0.0))) && (!(s.v[337] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[336] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), s.ad_value(218)), s.v[50]);
        }

        s.v[338] = if (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[336] != 0.0))) && (s.v[338] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.v[339] = if (((-s.v[80]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[336] != 0.0))) && (!(s.v[338] != 0.0))) && (s.v[339] != 0.0)) {
            let assign6330_ad_e7247: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign6330_ad_e7247);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[336] != 0.0))) && (!(s.v[338] != 0.0))) && (!(s.v[339] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[336] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(124), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p42);
        }

        s.v[340] = if (p.p51 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (s.v[340] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[341] = if (s.v[217] > ((-s.v[82]) * p.p51)) { 1.0 } else { 0.0 };

        s.v[342] = if (p.p54 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[340] != 0.0))) && (s.v[341] != 0.0)) && (s.v[342] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[340] != 0.0))) && (s.v[341] != 0.0)) && (!(s.v[342] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[340] != 0.0))) && (s.v[341] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) && (!(s.v[340] != 0.0))) && (!(s.v[341] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90]), s.v[84]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[326] != 0.0))) {
            s.store_mul_ad_lhs(246, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[343] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[343] != 0.0)) {
            s.store_scalar(247, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.v[344] = if ((p.p32 == 0.0) && (p.p37 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (s.v[344] != 0.0)) {
            s.store_scalar(220, 0.0);
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
        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[33], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[345] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) && (s.v[345] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) && (!(s.v[345] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p23)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[346] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) && (s.v[346] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) && (!(s.v[346] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) {
            s.store_scale(225, 218, s.v[63]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[24]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[344] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.v[347] = if (p.p37 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (s.v[347] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[48]), s.ad_value(221)), s.v[78]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[348] = if (((-p.p23) * s.v[51]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (s.v[348] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (!(s.v[348] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[349] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (s.v[349] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (!(s.v[349] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[350] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (s.v[350] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (!(s.v[350] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[351] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (s.v[351] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[352] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (!(s.v[351] != 0.0))) && (s.v[352] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (!(s.v[351] != 0.0))) && (!(s.v[352] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) && (!(s.v[351] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[75]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[347] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p37);
        }

        s.v[353] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (s.v[353] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[354] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[353] != 0.0))) && (s.v[354] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[353] != 0.0))) && (!(s.v[354] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[353] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), s.ad_value(218)), s.v[51]);
        }

        s.v[355] = if (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[353] != 0.0))) && (s.v[355] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.v[356] = if (((-s.v[81]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[353] != 0.0))) && (!(s.v[355] != 0.0))) && (s.v[356] != 0.0)) {
            let assign7030_ad_e8284: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign7030_ad_e8284);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[353] != 0.0))) && (!(s.v[355] != 0.0))) && (!(s.v[356] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[353] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(124), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p43);
        }

        s.v[357] = if (p.p52 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (s.v[357] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[358] = if (s.v[217] > ((-s.v[82]) * p.p52)) { 1.0 } else { 0.0 };

        s.v[359] = if (p.p55 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[357] != 0.0))) && (s.v[358] != 0.0)) && (s.v[359] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[357] != 0.0))) && (s.v[358] != 0.0)) && (!(s.v[359] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[357] != 0.0))) && (s.v[358] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) && (!(s.v[357] != 0.0))) && (!(s.v[358] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91]), s.v[85]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[343] != 0.0))) {
            s.store_mul_ad_lhs(247, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        if (s.v[199] != 0.0) {
            s.store_add_ad(114, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), A::scale(s.ad_value(247), s.v[145]));
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(216, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(213, 0.0);
        }

        s.v[360] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        s.v[361] = if (s.v[125] < s.v[149]) { 1.0 } else { 0.0 };

        s.v[362] = if (((((-0.5) * (s.v[125] * s.v[9]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (s.v[361] != 0.0)) && (s.v[362] != 0.0)) {
            s.store_exp_ad(211, A::scale(s.ad_value(125), (s.v[9] * (-0.5))));
        }

        s.v[363] = if (((-0.5) * (s.v[125] * s.v[9])) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (s.v[361] != 0.0)) && (!(s.v[362] != 0.0))) && (s.v[363] != 0.0)) {
            let assign7300_ad_e8665: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(125), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(125), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(125), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(211, &assign7300_ad_e8665);
        }

        if (((((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (s.v[361] != 0.0)) && (!(s.v[362] != 0.0))) && (!(s.v[363] != 0.0))) {
            s.store_scale_ad(211, A::offset(A::mul(A::offset(A::scale(s.ad_value(125), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(125), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(125), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (s.v[361] != 0.0)) {
            s.store_div_from_scalar(212, 1.0, 211);
        }

        if (((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (s.v[361] != 0.0)) {
            s.store_square(209, 212);
        }

        if (((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (!(s.v[361] != 0.0))) {
            s.store_mul_ad_lhs(209, A::offset(A::scale(A::sub(s.ad_value(125), s.ad_value(149)), s.v[9]), 1.0), 150);
        }

        if (((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (!(s.v[361] != 0.0))) {
            s.store_sqrt(212, 209);
        }

        if (((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (!(s.v[361] != 0.0))) {
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if ((s.v[199] != 0.0) && (s.v[360] != 0.0)) {
            s.store_offset(209, 209, (-1.0));
        }

        s.v[364] = if (s.v[125] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (s.v[364] != 0.0)) {
            s.store_scale_ad(213, A::ln(A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0))))), (s.v[8] * 2.0));
        }

        if (((s.v[199] != 0.0) && (s.v[360] != 0.0)) && (!(s.v[364] != 0.0))) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 125);
        }

        if ((s.v[199] != 0.0) && (s.v[360] != 0.0)) {
            s.store_sub(214, 151, 213);
        }

        if ((s.v[199] != 0.0) && (s.v[360] != 0.0)) {
            s.store_scale_ad(215, A::sub(A::add(s.ad_value(125), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(125), s.ad_value(214)), A::sub(s.ad_value(125), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[360] != 0.0)) {
            s.store_scale_ad(216, A::sub(A::add(s.ad_value(125), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(125), s.ad_value(154)), A::sub(s.ad_value(125), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[360] != 0.0)) {
            s.store_scale_ad(217, A::sub(s.ad_value(125), A::sqrt(A::offset(A::mul(s.ad_value(125), s.ad_value(125)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[365] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[365] != 0.0)) {
            s.store_scalar(245, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.v[366] = if ((p.p30 == 0.0) && (p.p35 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (s.v[366] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[31], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[367] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) && (s.v[367] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) && (!(s.v[367] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p21)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[368] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) && (s.v[368] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) && (!(s.v[368] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) {
            s.store_scale(225, 218, s.v[61]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[22]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[366] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.v[369] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (s.v[369] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[46]), s.ad_value(221)), s.v[76]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[370] = if (((-p.p21) * s.v[49]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (s.v[370] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (!(s.v[370] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[371] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (s.v[371] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (!(s.v[371] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[372] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (s.v[372] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (!(s.v[372] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[373] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (s.v[373] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[374] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (!(s.v[373] != 0.0))) && (s.v[374] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (!(s.v[373] != 0.0))) && (!(s.v[374] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) && (!(s.v[373] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[73]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[369] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p35);
        }

        s.v[375] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (s.v[375] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[376] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[375] != 0.0))) && (s.v[376] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[375] != 0.0))) && (!(s.v[376] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[375] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), s.ad_value(218)), s.v[49]);
        }

        s.v[377] = if (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

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
        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[375] != 0.0))) && (s.v[377] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.v[378] = if (((-s.v[79]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[375] != 0.0))) && (!(s.v[377] != 0.0))) && (s.v[378] != 0.0)) {
            let assign8030_ad_e9772: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign8030_ad_e9772);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[375] != 0.0))) && (!(s.v[377] != 0.0))) && (!(s.v[378] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[375] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(125), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p41);
        }

        s.v[379] = if (p.p50 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (s.v[379] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[380] = if (s.v[217] > ((-s.v[82]) * p.p50)) { 1.0 } else { 0.0 };

        s.v[381] = if (p.p53 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[379] != 0.0))) && (s.v[380] != 0.0)) && (s.v[381] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[379] != 0.0))) && (s.v[380] != 0.0)) && (!(s.v[381] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[379] != 0.0))) && (s.v[380] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) && (!(s.v[379] != 0.0))) && (!(s.v[380] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89]), s.v[83]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[365] != 0.0))) {
            s.store_mul_ad_lhs(245, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[382] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[382] != 0.0)) {
            s.store_scalar(246, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.v[383] = if ((p.p31 == 0.0) && (p.p36 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (s.v[383] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[32], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[384] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) && (s.v[384] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) && (!(s.v[384] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p22)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[385] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) && (s.v[385] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) && (!(s.v[385] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) {
            s.store_scale(225, 218, s.v[62]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[23]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[383] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.v[386] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (s.v[386] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[47]), s.ad_value(221)), s.v[77]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[387] = if (((-p.p22) * s.v[50]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (s.v[387] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (!(s.v[387] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[388] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (s.v[388] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (!(s.v[388] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[389] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (s.v[389] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (!(s.v[389] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[390] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (s.v[390] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[391] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (!(s.v[390] != 0.0))) && (s.v[391] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (!(s.v[390] != 0.0))) && (!(s.v[391] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) && (!(s.v[390] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[74]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[386] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p36);
        }

        s.v[392] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (s.v[392] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[393] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[392] != 0.0))) && (s.v[393] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[392] != 0.0))) && (!(s.v[393] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[392] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), s.ad_value(218)), s.v[50]);
        }

        s.v[394] = if (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[392] != 0.0))) && (s.v[394] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.v[395] = if (((-s.v[80]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[392] != 0.0))) && (!(s.v[394] != 0.0))) && (s.v[395] != 0.0)) {
            let assign8730_ad_e10809: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign8730_ad_e10809);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[392] != 0.0))) && (!(s.v[394] != 0.0))) && (!(s.v[395] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[392] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(125), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p42);
        }

        s.v[396] = if (p.p51 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (s.v[396] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[397] = if (s.v[217] > ((-s.v[82]) * p.p51)) { 1.0 } else { 0.0 };

        s.v[398] = if (p.p54 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[396] != 0.0))) && (s.v[397] != 0.0)) && (s.v[398] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[396] != 0.0))) && (s.v[397] != 0.0)) && (!(s.v[398] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[396] != 0.0))) && (s.v[397] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) && (!(s.v[396] != 0.0))) && (!(s.v[397] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90]), s.v[84]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[382] != 0.0))) {
            s.store_mul_ad_lhs(246, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[399] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[399] != 0.0)) {
            s.store_scalar(247, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.v[400] = if ((p.p32 == 0.0) && (p.p37 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (s.v[400] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[33], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[401] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) && (s.v[401] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) && (!(s.v[401] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p23)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[402] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) && (s.v[402] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) && (!(s.v[402] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) {
            s.store_scale(225, 218, s.v[63]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[24]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[400] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.v[403] = if (p.p37 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (s.v[403] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[48]), s.ad_value(221)), s.v[78]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[404] = if (((-p.p23) * s.v[51]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (s.v[404] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (!(s.v[404] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[405] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (s.v[405] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (!(s.v[405] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[406] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (s.v[406] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (!(s.v[406] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[407] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (s.v[407] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[408] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (!(s.v[407] != 0.0))) && (s.v[408] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (!(s.v[407] != 0.0))) && (!(s.v[408] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) && (!(s.v[407] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[75]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[403] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p37);
        }

        s.v[409] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (s.v[409] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[410] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[409] != 0.0))) && (s.v[410] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[409] != 0.0))) && (!(s.v[410] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[409] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), s.ad_value(218)), s.v[51]);
        }

        s.v[411] = if (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[409] != 0.0))) && (s.v[411] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.v[412] = if (((-s.v[81]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[409] != 0.0))) && (!(s.v[411] != 0.0))) && (s.v[412] != 0.0)) {
            let assign9430_ad_e11846: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign9430_ad_e11846);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[409] != 0.0))) && (!(s.v[411] != 0.0))) && (!(s.v[412] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
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
        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[409] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(125), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p43);
        }

        s.v[413] = if (p.p52 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (s.v[413] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[414] = if (s.v[217] > ((-s.v[82]) * p.p52)) { 1.0 } else { 0.0 };

        s.v[415] = if (p.p55 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[413] != 0.0))) && (s.v[414] != 0.0)) && (s.v[415] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[413] != 0.0))) && (s.v[414] != 0.0)) && (!(s.v[415] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[413] != 0.0))) && (s.v[414] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) && (!(s.v[413] != 0.0))) && (!(s.v[414] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91]), s.v[85]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[399] != 0.0))) {
            s.store_mul_ad_lhs(247, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        if (s.v[199] != 0.0) {
            s.store_add_ad(115, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), A::scale(s.ad_value(247), s.v[145]));
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(216, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(213, 0.0);
        }

        s.v[416] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        s.v[417] = if (s.v[126] < s.v[149]) { 1.0 } else { 0.0 };

        s.v[418] = if (((((-0.5) * (s.v[126] * s.v[9]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) && (s.v[418] != 0.0)) {
            s.store_exp_ad(211, A::scale(s.ad_value(126), (s.v[9] * (-0.5))));
        }

        s.v[419] = if (((-0.5) * (s.v[126] * s.v[9])) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) && (!(s.v[418] != 0.0))) && (s.v[419] != 0.0)) {
            let assign9700_ad_e12227: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(126), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(126), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(126), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(211, &assign9700_ad_e12227);
        }

        if (((((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) && (!(s.v[418] != 0.0))) && (!(s.v[419] != 0.0))) {
            s.store_scale_ad(211, A::offset(A::mul(A::offset(A::scale(s.ad_value(126), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(126), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(126), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) {
            s.store_div_from_scalar(212, 1.0, 211);
        }

        if (((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (s.v[417] != 0.0)) {
            s.store_square(209, 212);
        }

        if (((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (!(s.v[417] != 0.0))) {
            s.store_mul_ad_lhs(209, A::offset(A::scale(A::sub(s.ad_value(126), s.ad_value(149)), s.v[9]), 1.0), 150);
        }

        if (((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (!(s.v[417] != 0.0))) {
            s.store_sqrt(212, 209);
        }

        if (((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (!(s.v[417] != 0.0))) {
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if ((s.v[199] != 0.0) && (s.v[416] != 0.0)) {
            s.store_offset(209, 209, (-1.0));
        }

        s.v[420] = if (s.v[126] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (s.v[420] != 0.0)) {
            s.store_scale_ad(213, A::ln(A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0))))), (s.v[8] * 2.0));
        }

        if (((s.v[199] != 0.0) && (s.v[416] != 0.0)) && (!(s.v[420] != 0.0))) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 126);
        }

        if ((s.v[199] != 0.0) && (s.v[416] != 0.0)) {
            s.store_sub(214, 151, 213);
        }

        if ((s.v[199] != 0.0) && (s.v[416] != 0.0)) {
            s.store_scale_ad(215, A::sub(A::add(s.ad_value(126), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(126), s.ad_value(214)), A::sub(s.ad_value(126), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[416] != 0.0)) {
            s.store_scale_ad(216, A::sub(A::add(s.ad_value(126), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(126), s.ad_value(154)), A::sub(s.ad_value(126), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[416] != 0.0)) {
            s.store_scale_ad(217, A::sub(s.ad_value(126), A::sqrt(A::offset(A::mul(s.ad_value(126), s.ad_value(126)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[421] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[421] != 0.0)) {
            s.store_scalar(245, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.v[422] = if ((p.p30 == 0.0) && (p.p35 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (s.v[422] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[31], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[423] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) && (s.v[423] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) && (!(s.v[423] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p21)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[424] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) && (s.v[424] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) && (!(s.v[424] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) {
            s.store_scale(225, 218, s.v[61]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[22]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[422] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.v[425] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (s.v[425] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[46]), s.ad_value(221)), s.v[76]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[426] = if (((-p.p21) * s.v[49]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (s.v[426] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (!(s.v[426] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[427] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (s.v[427] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (!(s.v[427] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[428] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (s.v[428] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (!(s.v[428] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[429] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (s.v[429] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[430] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (!(s.v[429] != 0.0))) && (s.v[430] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (!(s.v[429] != 0.0))) && (!(s.v[430] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) && (!(s.v[429] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[73]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[425] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p35);
        }

        s.v[431] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (s.v[431] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[432] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[431] != 0.0))) && (s.v[432] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[431] != 0.0))) && (!(s.v[432] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[431] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), s.ad_value(218)), s.v[49]);
        }

        s.v[433] = if (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[431] != 0.0))) && (s.v[433] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.v[434] = if (((-s.v[79]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[431] != 0.0))) && (!(s.v[433] != 0.0))) && (s.v[434] != 0.0)) {
            let assign10430_ad_e13334: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign10430_ad_e13334);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[431] != 0.0))) && (!(s.v[433] != 0.0))) && (!(s.v[434] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[431] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(126), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p41);
        }

        s.v[435] = if (p.p50 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (s.v[435] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[436] = if (s.v[217] > ((-s.v[82]) * p.p50)) { 1.0 } else { 0.0 };

        s.v[437] = if (p.p53 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[435] != 0.0))) && (s.v[436] != 0.0)) && (s.v[437] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[435] != 0.0))) && (s.v[436] != 0.0)) && (!(s.v[437] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[435] != 0.0))) && (s.v[436] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) && (!(s.v[435] != 0.0))) && (!(s.v[436] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89]), s.v[83]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[421] != 0.0))) {
            s.store_mul_ad_lhs(245, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[438] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[438] != 0.0)) {
            s.store_scalar(246, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.v[439] = if ((p.p31 == 0.0) && (p.p36 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (s.v[439] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[32], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[440] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) && (s.v[440] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) && (!(s.v[440] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p22)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[441] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) && (s.v[441] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) && (!(s.v[441] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) {
            s.store_scale(225, 218, s.v[62]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[23]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[439] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.v[442] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (s.v[442] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[47]), s.ad_value(221)), s.v[77]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[443] = if (((-p.p22) * s.v[50]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (s.v[443] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (!(s.v[443] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[444] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (s.v[444] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (!(s.v[444] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[445] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (s.v[445] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (!(s.v[445] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
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
        s.v[446] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (s.v[446] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[447] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (!(s.v[446] != 0.0))) && (s.v[447] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (!(s.v[446] != 0.0))) && (!(s.v[447] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) && (!(s.v[446] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[74]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[442] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p36);
        }

        s.v[448] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (s.v[448] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[449] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[448] != 0.0))) && (s.v[449] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[448] != 0.0))) && (!(s.v[449] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[448] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), s.ad_value(218)), s.v[50]);
        }

        s.v[450] = if (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[448] != 0.0))) && (s.v[450] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.v[451] = if (((-s.v[80]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[448] != 0.0))) && (!(s.v[450] != 0.0))) && (s.v[451] != 0.0)) {
            let assign11130_ad_e14371: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign11130_ad_e14371);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[448] != 0.0))) && (!(s.v[450] != 0.0))) && (!(s.v[451] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[448] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(126), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p42);
        }

        s.v[452] = if (p.p51 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (s.v[452] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[453] = if (s.v[217] > ((-s.v[82]) * p.p51)) { 1.0 } else { 0.0 };

        s.v[454] = if (p.p54 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[452] != 0.0))) && (s.v[453] != 0.0)) && (s.v[454] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[452] != 0.0))) && (s.v[453] != 0.0)) && (!(s.v[454] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[452] != 0.0))) && (s.v[453] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) && (!(s.v[452] != 0.0))) && (!(s.v[453] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90]), s.v[84]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[438] != 0.0))) {
            s.store_mul_ad_lhs(246, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[455] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[455] != 0.0)) {
            s.store_scalar(247, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.v[456] = if ((p.p32 == 0.0) && (p.p37 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (s.v[456] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[33], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[457] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) && (s.v[457] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) && (!(s.v[457] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p23)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[458] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) && (s.v[458] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) && (!(s.v[458] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) {
            s.store_scale(225, 218, s.v[63]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[24]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[456] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.v[459] = if (p.p37 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (s.v[459] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[48]), s.ad_value(221)), s.v[78]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[460] = if (((-p.p23) * s.v[51]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (s.v[460] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (!(s.v[460] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[461] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (s.v[461] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (!(s.v[461] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[462] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (s.v[462] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (!(s.v[462] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[463] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (s.v[463] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[464] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (!(s.v[463] != 0.0))) && (s.v[464] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (!(s.v[463] != 0.0))) && (!(s.v[464] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) && (!(s.v[463] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[75]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[459] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p37);
        }

        s.v[465] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (s.v[465] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[466] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[465] != 0.0))) && (s.v[466] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[465] != 0.0))) && (!(s.v[466] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[465] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), s.ad_value(218)), s.v[51]);
        }

        s.v[467] = if (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[465] != 0.0))) && (s.v[467] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.v[468] = if (((-s.v[81]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[465] != 0.0))) && (!(s.v[467] != 0.0))) && (s.v[468] != 0.0)) {
            let assign11830_ad_e15408: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign11830_ad_e15408);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[465] != 0.0))) && (!(s.v[467] != 0.0))) && (!(s.v[468] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[465] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(126), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p43);
        }

        s.v[469] = if (p.p52 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (s.v[469] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[470] = if (s.v[217] > ((-s.v[82]) * p.p52)) { 1.0 } else { 0.0 };

        s.v[471] = if (p.p55 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[469] != 0.0))) && (s.v[470] != 0.0)) && (s.v[471] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[469] != 0.0))) && (s.v[470] != 0.0)) && (!(s.v[471] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[469] != 0.0))) && (s.v[470] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) && (!(s.v[469] != 0.0))) && (!(s.v[470] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91]), s.v[85]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[455] != 0.0))) {
            s.store_mul_ad_lhs(247, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        if (s.v[199] != 0.0) {
            s.store_add_ad(116, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), A::scale(s.ad_value(247), s.v[145]));
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(216, 0.0);
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(213, 0.0);
        }

        s.v[472] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        s.v[473] = if (s.v[127] < s.v[149]) { 1.0 } else { 0.0 };

        s.v[474] = if (((((-0.5) * (s.v[127] * s.v[9]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (s.v[473] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_exp_ad(211, A::scale(s.ad_value(127), (s.v[9] * (-0.5))));
        }

        s.v[475] = if (((-0.5) * (s.v[127] * s.v[9])) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (s.v[473] != 0.0)) && (!(s.v[474] != 0.0))) && (s.v[475] != 0.0)) {
            let assign12100_ad_e15789: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(127), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(127), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(127), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(211, &assign12100_ad_e15789);
        }

        if (((((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (s.v[473] != 0.0)) && (!(s.v[474] != 0.0))) && (!(s.v[475] != 0.0))) {
            s.store_scale_ad(211, A::offset(A::mul(A::offset(A::scale(s.ad_value(127), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(127), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(127), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_div_from_scalar(212, 1.0, 211);
        }

        if (((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_square(209, 212);
        }

        if (((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_mul_ad_lhs(209, A::offset(A::scale(A::sub(s.ad_value(127), s.ad_value(149)), s.v[9]), 1.0), 150);
        }

        if (((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_sqrt(212, 209);
        }

        if (((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_div_from_scalar(211, 1.0, 212);
        }

        if ((s.v[199] != 0.0) && (s.v[472] != 0.0)) {
            s.store_offset(209, 209, (-1.0));
        }

        s.v[476] = if (s.v[127] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (s.v[476] != 0.0)) {
            s.store_scale_ad(213, A::ln(A::add(A::offset(s.ad_value(211), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(211), 1.0), A::offset(s.ad_value(211), 3.0))))), (s.v[8] * 2.0));
        }

        if (((s.v[199] != 0.0) && (s.v[472] != 0.0)) && (!(s.v[476] != 0.0))) {
            s.store_sub_ad_lhs(213, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(212), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(212), 1.0), A::offset(A::scale(s.ad_value(212), 3.0), 1.0))))), (s.v[8] * 2.0)), 127);
        }

        if ((s.v[199] != 0.0) && (s.v[472] != 0.0)) {
            s.store_sub(214, 151, 213);
        }

        if ((s.v[199] != 0.0) && (s.v[472] != 0.0)) {
            s.store_scale_ad(215, A::sub(A::add(s.ad_value(127), s.ad_value(214)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(127), s.ad_value(214)), A::sub(s.ad_value(127), s.ad_value(214))), ((4.0 * s.v[8]) * s.v[8])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[472] != 0.0)) {
            s.store_scale_ad(216, A::sub(A::add(s.ad_value(127), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(127), s.ad_value(154)), A::sub(s.ad_value(127), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])))), 0.5);
        }

        if ((s.v[199] != 0.0) && (s.v[472] != 0.0)) {
            s.store_scale_ad(217, A::sub(s.ad_value(127), A::sqrt(A::offset(A::mul(s.ad_value(127), s.ad_value(127)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[477] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[477] != 0.0)) {
            s.store_scalar(245, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) {
            s.store_scale(219, 209, s.v[25]);
        }

        s.v[478] = if ((p.p30 == 0.0) && (p.p35 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (s.v[478] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[31], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[479] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) && (!(s.v[479] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p21)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[480] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) && (s.v[480] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) && (!(s.v[480] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            s.store_scale(225, 218, s.v[61]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[22]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p30);
        }

        s.v[481] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (s.v[481] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[46]), s.ad_value(221)), s.v[76]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[73]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
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
        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[482] = if (((-p.p21) * s.v[49]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (!(s.v[482] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p21) * s.v[49]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[73]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[73])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[483] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (s.v[483] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (!(s.v[483] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[484] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (s.v[484] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (!(s.v[484] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[485] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (s.v[485] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[486] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (!(s.v[485] != 0.0))) && (s.v[486] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (!(s.v[485] != 0.0))) && (!(s.v[486] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) && (!(s.v[485] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[73]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[481] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p35);
        }

        s.v[487] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (s.v[487] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[488] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[487] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[487] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[67]), p.p21);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[487] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p18, s.ad_value(216)), s.v[64]), s.ad_value(218)), s.v[49]);
        }

        s.v[489] = if (((((-s.v[79]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[487] != 0.0))) && (s.v[489] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(79)), s.ad_value(243)));
        }

        s.v[490] = if (((-s.v[79]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[487] != 0.0))) && (!(s.v[489] != 0.0))) && (s.v[490] != 0.0)) {
            let assign12830_ad_e16896: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign12830_ad_e16896);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[487] != 0.0))) && (!(s.v[489] != 0.0))) && (!(s.v[490] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[487] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(127), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p41);
        }

        s.v[491] = if (p.p50 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (s.v[491] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[492] = if (s.v[217] > ((-s.v[82]) * p.p50)) { 1.0 } else { 0.0 };

        s.v[493] = if (p.p53 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[492] != 0.0)) && (s.v[493] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[86]), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86])), A::scale(s.ad_value(217), s.v[86]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[492] != 0.0)) && (!(s.v[493] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[86])), p.p53);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[492] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[492] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p50)), s.v[89]), s.v[83]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[477] != 0.0))) {
            s.store_mul_ad_lhs(245, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[494] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[494] != 0.0)) {
            s.store_scalar(246, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) {
            s.store_scale(219, 209, s.v[26]);
        }

        s.v[495] = if ((p.p31 == 0.0) && (p.p36 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (s.v[495] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[32], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[496] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) && (s.v[496] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) && (!(s.v[496] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p22)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[497] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) && (s.v[497] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) && (!(s.v[497] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_scale(225, 218, s.v[62]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[23]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p31);
        }

        s.v[498] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (s.v[498] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[47]), s.ad_value(221)), s.v[77]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[74]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[499] = if (((-p.p22) * s.v[50]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (s.v[499] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (!(s.v[499] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p22) * s.v[50]));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[74]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[74])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[500] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (s.v[500] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (!(s.v[500] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[501] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (s.v[501] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (!(s.v[501] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[502] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (s.v[502] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[503] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (!(s.v[502] != 0.0))) && (s.v[503] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (!(s.v[502] != 0.0))) && (!(s.v[503] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) && (!(s.v[502] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[74]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p36);
        }

        s.v[504] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (s.v[504] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[505] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[504] != 0.0))) && (s.v[505] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[504] != 0.0))) && (!(s.v[505] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[68]), p.p22);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[504] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p19, s.ad_value(216)), s.v[65]), s.ad_value(218)), s.v[50]);
        }

        s.v[506] = if (((((-s.v[80]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(80)), s.ad_value(243)));
        }

        s.v[507] = if (((-s.v[80]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (s.v[507] != 0.0)) {
            let assign13530_ad_e17933: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign13530_ad_e17933);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (!(s.v[507] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[504] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(127), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p42);
        }

        s.v[508] = if (p.p51 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (s.v[508] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[509] = if (s.v[217] > ((-s.v[82]) * p.p51)) { 1.0 } else { 0.0 };

        s.v[510] = if (p.p54 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[508] != 0.0))) && (s.v[509] != 0.0)) && (s.v[510] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[87]), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87])), A::scale(s.ad_value(217), s.v[87]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[508] != 0.0))) && (s.v[509] != 0.0)) && (!(s.v[510] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[87])), p.p54);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[508] != 0.0))) && (s.v[509] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) && (!(s.v[508] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p51)), s.v[90]), s.v[84]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[494] != 0.0))) {
            s.store_mul_ad_lhs(246, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        s.v[511] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[511] != 0.0)) {
            s.store_scalar(247, 0.0);
        }

        if ((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) {
            s.store_scale(219, 209, s.v[27]);
        }

        s.v[512] = if ((p.p32 == 0.0) && (p.p37 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (s.v[512] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_sub_from_scalar(221, s.v[33], 215);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_sub_from_scalar_ad(222, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(213), s.ad_value(221)))));
        }

        s.v[513] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) && (s.v[513] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) && (!(s.v[513] != 0.0))) {
            s.store_scale_ad(223, A::add(A::div(A::mul(A::square(s.ad_value(222)), A::ln(s.ad_value(222))), A::sub_from_scalar(1.0, s.ad_value(222))), s.ad_value(222)), (1.0 - (2.0 * p.p23)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_add(224, 222, 223);
        }

        s.v[514] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) && (s.v[514] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(s.ad_value(221), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) && (!(s.v[514] != 0.0))) {
            s.store_powf_ad(218, A::scale(s.ad_value(221), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_scale(225, 218, s.v[63]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_scale_ad(226, A::mul(A::offset(s.ad_value(212), (-1.0)), s.ad_value(225)), s.v[24]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_scaled_mul(220, 226, 224, p.p32);
        }

        s.v[515] = if (p.p37 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (s.v[515] != 0.0)) {
            s.store_scalar(227, 0.0);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(228, A::div(A::scale(s.ad_value(225), s.v[48]), s.ad_value(221)), s.v[78]);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_div_from_scalar(229, (0.666666666666667 * s.v[75]), 228);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_square(230, 229);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_sqrt_ad(231, A::div(A::square(s.ad_value(230)), A::offset(A::square(s.ad_value(230)), 1.0)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_sqrt(232, 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_mul(233, 231, 232);
        }

        s.v[516] = if (((-p.p23) * s.v[51]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[516] != 0.0)) {
            s.store_div_from_scalar_ad(234, 1.0, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[516] != 0.0))) {
            s.store_powf_ad(234, A::offset(A::mul(s.ad_value(228), s.ad_value(233)), 1.0), ((-p.p23) * s.v[51]));
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
        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_div_ad(235, A::mul(s.ad_value(224), s.ad_value(234)), A::add(s.ad_value(224), s.ad_value(234)));
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_sqrt_ad(236, A::scale(A::div(s.ad_value(228), s.ad_value(232)), 0.375));
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_sub_ad_lhs(237, A::scale(A::mul(s.ad_value(229), s.ad_value(232)), 2.0), 231);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_add_ad(238, A::sub(A::mul(A::scale(s.ad_value(229), s.v[75]), s.ad_value(232)), A::scale(s.ad_value(231), s.v[75])), A::scale(A::mul(s.ad_value(228), s.ad_value(233)), 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_mul_ad_lhs(239, A::offset(s.ad_value(237), (-1.0)), 236);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_square(200, 239);
        }

        s.v[517] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[517] != 0.0)) {
            s.store_div_from_scalar_ad(201, 1.0, A::offset(A::scale(s.ad_value(239), s.v[10]), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[517] != 0.0))) {
            s.store_div_from_scalar_ad(201, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(239), s.v[10])));
        }

        s.v[518] = if (((-s.v[200]) + s.v[238]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[518] != 0.0)) {
            s.store_exp_ad(218, A::sub(s.ad_value(238), s.ad_value(200)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[518] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(238), s.ad_value(200))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_mul_ad_lhs(202, A::add(A::add(A::scale(s.ad_value(201), 0.29214664), A::scale(A::square(s.ad_value(201)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(201)), s.ad_value(201)), s.v[12])), 218);
        }

        s.v[519] = if (s.v[239] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[519] != 0.0)) {
            s.copy_ad(240, 202);
        }

        s.v[520] = if (s.v[238] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[519] != 0.0))) && (s.v[520] != 0.0)) {
            s.store_exp(218, 238);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[519] != 0.0))) && (!(s.v[520] != 0.0))) {
            s.store_div_from_scalar_ad(218, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(238)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[519] != 0.0))) {
            s.store_sub_ad_lhs(240, A::scale(s.ad_value(218), 2.0), 202);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(241, A::div(A::scale(s.ad_value(240), s.v[75]), s.ad_value(236)), (1.772453850905516 * 0.5));
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(227, A::mul(A::mul(s.ad_value(226), s.ad_value(241)), s.ad_value(235)), p.p37);
        }

        s.v[521] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (s.v[521] != 0.0)) {
            s.store_scalar(242, 0.0);
        }

        s.v[522] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[521] != 0.0))) && (s.v[522] != 0.0)) {
            s.store_sqrt_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[521] != 0.0))) && (!(s.v[522] != 0.0))) {
            s.store_powf_ad(218, A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[69]), p.p23);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[521] != 0.0))) {
            s.store_scale_ad(243, A::div(A::scale(A::sub_from_scalar(p.p20, s.ad_value(216)), s.v[66]), s.ad_value(218)), s.v[51]);
        }

        s.v[523] = if (((((-s.v[81]) / s.v[243])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[521] != 0.0))) && (s.v[523] != 0.0)) {
            s.store_exp_ad(218, A::div(A::neg(s.ad_value(81)), s.ad_value(243)));
        }

        s.v[524] = if (((-s.v[81]) / s.v[243]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[521] != 0.0))) && (!(s.v[523] != 0.0))) && (s.v[524] != 0.0)) {
            let assign14230_ad_e18970: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(243))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(218, 1e-100, assign14230_ad_e18970);
        }

        if (((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[521] != 0.0))) && (!(s.v[523] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_scale_ad(218, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(243)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[521] != 0.0))) {
            s.store_scale_ad(242, A::mul(A::mul(A::mul(s.ad_value(127), s.ad_value(243)), s.ad_value(243)), s.ad_value(218)), p.p43);
        }

        s.v[525] = if (p.p52 > 1000.0) { 1.0 } else { 0.0 };

        if (((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (s.v[525] != 0.0)) {
            s.store_scalar(244, 1.0);
        }

        s.v[526] = if (s.v[217] > ((-s.v[82]) * p.p52)) { 1.0 } else { 0.0 };

        s.v[527] = if (p.p55 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[525] != 0.0))) && (s.v[526] != 0.0)) && (s.v[527] != 0.0)) {
            s.store_mul_ad(218, A::mul(A::mul(A::scale(s.ad_value(217), s.v[88]), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88])), A::scale(s.ad_value(217), s.v[88]));
        }

        if (((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[525] != 0.0))) && (s.v[526] != 0.0)) && (!(s.v[527] != 0.0))) {
            s.store_powf_ad(218, A::abs(A::scale(s.ad_value(217), s.v[88])), p.p55);
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[525] != 0.0))) && (s.v[526] != 0.0)) {
            s.store_div_from_scalar_ad(244, 1.0, A::sub_from_scalar(1.0, s.ad_value(218)));
        }

        if ((((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) && (!(s.v[525] != 0.0))) && (!(s.v[526] != 0.0))) {
            s.store_offset_ad(244, A::scale(A::offset(s.ad_value(217), (s.v[82] * p.p52)), s.v[91]), s.v[85]);
        }

        if ((s.v[199] != 0.0) && (!(s.v[511] != 0.0))) {
            s.store_mul_ad_lhs(247, A::scale(A::add(A::add(A::add(s.ad_value(219), s.ad_value(220)), s.ad_value(227)), s.ad_value(242)), p.p10), 244);
        }

        if (s.v[199] != 0.0) {
            s.store_add_ad(117, A::add(A::scale(s.ad_value(245), s.v[143]), A::scale(s.ad_value(246), s.v[144])), A::scale(s.ad_value(247), s.v[145]));
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(161, (((s.v[143] * s.v[25]) + (s.v[144] * s.v[26])) + (s.v[145] * s.v[27])));
        }

        if (s.v[199] != 0.0) {
            s.store_sub_ad_rhs(121, 116, A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(126), (s.v[9] * s.v[162]))), (-1.0))));
        }

        if (s.v[199] != 0.0) {
            s.store_sub_ad_rhs(122, 117, A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(127), (s.v[9] * s.v[162]))), (-1.0))));
        }

        s.v[528] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        s.v[529] = if ((s.v[116] > 0.0) && (s.v[117] > 0.0)) { 1.0 } else { 0.0 };

        s.v[530] = if ((((((s.v[121] / s.v[116]) > 0.001) || ((s.v[122] / s.v[117]) > 0.001)) && (s.v[121] > 0.0)) && (s.v[122] > 0.0)) && (s.v[122] > s.v[121])) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[529] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_div(128, 121, 122);
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[529] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_div_ad(164, A::scale(A::ln(s.ad_value(128)), s.v[8]), A::sub(s.ad_value(126), s.ad_value(127)));
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[529] != 0.0)) && (s.v[530] != 0.0)) {
            s.store_div_ad_rhs(163, 121, A::offset(A::exp(A::mul(A::scale(s.ad_value(126), s.v[9]), s.ad_value(164))), (-1.0)));
        }

        if ((s.v[199] != 0.0) && (s.v[528] != 0.0)) {
            s.store_sub_ad(118, A::sub(s.ad_value(113), A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(123), (s.v[9] * s.v[162]))), (-1.0)))), A::mul(s.ad_value(163), A::offset(A::exp(A::mul(A::scale(s.ad_value(123), s.v[9]), s.ad_value(164))), (-1.0))));
        }

        if ((s.v[199] != 0.0) && (s.v[528] != 0.0)) {
            s.store_sub_ad(119, A::sub(s.ad_value(114), A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(124), (s.v[9] * s.v[162]))), (-1.0)))), A::mul(s.ad_value(163), A::offset(A::exp(A::mul(A::scale(s.ad_value(124), s.v[9]), s.ad_value(164))), (-1.0))));
        }

        if ((s.v[199] != 0.0) && (s.v[528] != 0.0)) {
            s.store_sub_ad(120, A::sub(s.ad_value(115), A::mul(s.ad_value(161), A::offset(A::exp(A::scale(s.ad_value(125), (s.v[9] * s.v[162]))), (-1.0)))), A::mul(s.ad_value(163), A::offset(A::exp(A::mul(A::scale(s.ad_value(125), s.v[9]), s.ad_value(164))), (-1.0))));
        }

        s.v[531] = if (((s.v[113] < 0.0) && (s.v[114] < 0.0)) && (s.v[115] < 0.0)) { 1.0 } else { 0.0 };

        s.v[532] = if (((((((s.v[118] / s.v[113]) > 0.001) || ((s.v[119] / s.v[114]) > 0.001)) || ((s.v[120] / s.v[115]) > 0.001)) && (s.v[118] < 0.0)) && (s.v[119] < 0.0)) && (s.v[120] < 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div(128, 118, 119);
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad(129, A::scale(A::ln(s.ad_value(128)), (-s.v[8])), A::sub(s.ad_value(123), s.ad_value(124)));
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_rhs(131, 124, A::sub(s.ad_value(124), s.ad_value(123)));
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_mul_ad(132, A::scale(A::offset(s.ad_value(128), (-1.0)), s.v[8]), A::offset(A::pow(s.ad_value(128), s.ad_value(131)), (-1.0)));
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div_ad_rhs(131, 123, A::sub(s.ad_value(123), s.ad_value(124)));
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_sub_ad_lhs(133, A::add(A::mul(A::pow(s.ad_value(128), s.ad_value(131)), A::sub(s.ad_value(124), s.ad_value(123))), A::mul(s.ad_value(128), s.ad_value(123))), 124);
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_div(130, 132, 133);
        }

        if ((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) {
            s.store_add(166, 129, 130);
        }

        s.v[533] = if (((((s.v[125] * s.v[9]) * s.v[166])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_scalar(167, 1.0);
        }

        if (((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_mul_ad_rhs(165, 120, A::add(A::div_from_scalar(1.0, s.ad_value(125)), A::scale(s.ad_value(166), (0.5 * s.v[9]))));
        }

        if (((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (s.v[533] != 0.0)) {
            s.store_div_ad_lhs(166, A::scale(A::mul(A::scale(s.ad_value(120), (-0.5)), s.ad_value(166)), s.v[9]), 125);
        }

        if (((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_scalar(167, 0.0);
        }

        if (((((s.v[199] != 0.0) && (s.v[528] != 0.0)) && (s.v[531] != 0.0)) && (s.v[532] != 0.0)) && (!(s.v[533] != 0.0))) {
            s.store_div_ad(165, A::neg(s.ad_value(120)), A::offset(A::exp(A::mul(A::scale(A::neg(s.ad_value(125)), s.v[9]), s.ad_value(166))), (-1.0)));
        }

        if (s.v[199] != 0.0) {
            s.store_scalar(139, (p.p64 * (((s.v[143] * s.v[52]) + (s.v[144] * s.v[53])) + (s.v[145] * s.v[54]))));
        }

        s.v[534] = if ((s.v[143] * s.v[52]) <= s.v[139]) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[534] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        s.v[535] = if ((s.v[144] * s.v[53]) <= s.v[139]) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[535] != 0.0)) {
            s.store_scalar(147, 0.0);
        }

        s.v[536] = if ((s.v[145] * s.v[54]) <= s.v[139]) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[536] != 0.0)) {
            s.store_scalar(148, 0.0);
        }

        s.v[537] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[537] != 0.0)) {
            s.store_ln_ad(155, A::div_from_scalar((0.5 * p.p12), A::offset(s.ad_value(161), 1e-21)));
        }

        if ((s.v[199] != 0.0) && (s.v[537] != 0.0)) {
            s.store_ln_ad(157, A::div_from_scalar((0.5 * p.p12), A::offset(s.ad_value(163), 1e-21)));
        }

        if ((s.v[199] != 0.0) && (s.v[537] != 0.0)) {
            s.store_ln_ad(159, A::div_from_scalar((0.5 * p.p12), A::offset(A::abs(s.ad_value(165)), 1e-21)));
        }

        if (s.v[199] != 0.0) {
            s.store_ad(155, &A::min_with_scalar(s.ad_value(155), 230.25850929940458));
        }

        if (s.v[199] != 0.0) {
            s.store_exp(156, 155);
        }

        if (s.v[199] != 0.0) {
            s.store_ad(157, &A::min_with_scalar(s.ad_value(157), 230.25850929940458));
        }

        if (s.v[199] != 0.0) {
            s.store_exp(158, 157);
        }

        if (s.v[199] != 0.0) {
            s.store_ad(159, &A::min_with_scalar(s.ad_value(159), 230.25850929940458));
        }

        if (s.v[199] != 0.0) {
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

        s.store_ad(547, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(1)), p.p1));

        s.v[595] = if (s.v[112] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[595] != 0.0) {
            s.store_scale(134, 547, (s.v[9] * s.v[162]));
        }

        if (s.v[595] != 0.0) {
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
            s.store_ad(135, &assign15380_ad_e19886);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_rhs(140, 161, A::offset(s.ad_value(135), (-1.0)));
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_lhs(134, A::scale(s.ad_value(547), s.v[9]), 164);
        }

        if (s.v[595] != 0.0) {
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
            s.store_ad(135, &assign15410_ad_e19931);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_rhs(141, 163, A::offset(s.ad_value(135), (-1.0)));
        }

        if (s.v[595] != 0.0) {
            s.store_scalar(142, 0.0);
        }

        s.v[596] = if (s.v[167] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[595] != 0.0) && (s.v[596] != 0.0)) {
            s.store_mul_ad_rhs(142, 547, A::add(s.ad_value(165), A::mul(s.ad_value(547), s.ad_value(166))));
        }

        if ((s.v[595] != 0.0) && (!(s.v[596] != 0.0))) {
            s.store_mul_ad_lhs(134, A::scale(A::neg(s.ad_value(547)), s.v[9]), 166);
        }

        if ((s.v[595] != 0.0) && (!(s.v[596] != 0.0))) {
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
            s.store_ad(135, &assign15470_ad_e20002);
        }

        if ((s.v[595] != 0.0) && (!(s.v[596] != 0.0))) {
            s.store_mul_ad(142, A::neg(s.ad_value(165)), A::offset(s.ad_value(135), (-1.0)));
        }

        if (s.v[595] != 0.0) {
            s.store_add_ad_lhs(544, A::add(s.ad_value(140), s.ad_value(141)), 142);
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
        if (s.v[595] != 0.0) {
            s.store_scalar(597, 0.0);
        }

        if (s.v[595] != 0.0) {
            s.store_scalar(598, 0.0);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if (s.v[595] != 0.0) {
            s.store_div(552, 152, 153);
        }

        if (s.v[595] != 0.0) {
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if (s.v[595] != 0.0) {
            s.store_add(554, 153, 553);
        }

        if (s.v[595] != 0.0) {
            s.store_sub(555, 153, 553);
        }

        if (s.v[595] != 0.0) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

        if (s.v[595] != 0.0) {
            s.store_scale_ad(598, A::div(A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
        }

        s.v[599] = if (s.v[146] > 0.5) { 1.0 } else { 0.0 };

        s.v[600] = if (s.v[46] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[595] != 0.0) && (s.v[599] != 0.0)) && (s.v[600] != 0.0)) {
            s.store_sqrt_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])));
        }

        if (((s.v[595] != 0.0) && (s.v[599] != 0.0)) && (!(s.v[600] != 0.0))) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])), s.v[46]);
        }

        if ((s.v[595] != 0.0) && (s.v[599] != 0.0)) {
            s.store_add_ad(539, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[58]));
        }

        s.v[601] = if (s.v[147] > 0.5) { 1.0 } else { 0.0 };

        s.v[602] = if (s.v[47] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[595] != 0.0) && (s.v[601] != 0.0)) && (s.v[602] != 0.0)) {
            s.store_sqrt_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])));
        }

        if (((s.v[595] != 0.0) && (s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])), s.v[47]);
        }

        if ((s.v[595] != 0.0) && (s.v[601] != 0.0)) {
            s.store_add_ad(541, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[59]));
        }

        s.v[603] = if (s.v[148] > 0.5) { 1.0 } else { 0.0 };

        s.v[604] = if (s.v[48] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[595] != 0.0) && (s.v[603] != 0.0)) && (s.v[604] != 0.0)) {
            s.store_sqrt_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])));
        }

        if (((s.v[595] != 0.0) && (s.v[603] != 0.0)) && (!(s.v[604] != 0.0))) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])), s.v[48]);
        }

        if ((s.v[595] != 0.0) && (s.v[603] != 0.0)) {
            s.store_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[60]));
        }

        if (!(s.v[595] != 0.0)) {
            s.store_scalar(564, 0.0);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_scalar(561, 0.0);
        }

        s.v[605] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_div(552, 152, 153);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_add(554, 153, 553);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_sub(555, 153, 553);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(558, A::div(A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
        }

        s.v[606] = if (s.v[547] < s.v[149]) { 1.0 } else { 0.0 };

        s.v[607] = if (((((-0.5) * (s.v[547] * s.v[9]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) && (s.v[607] != 0.0)) {
            s.store_exp_ad(559, A::scale(s.ad_value(547), (s.v[9] * (-0.5))));
        }

        s.v[608] = if (((-0.5) * (s.v[547] * s.v[9])) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) && (!(s.v[607] != 0.0))) && (s.v[608] != 0.0)) {
            let assign15880_ad_e20424: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(559, &assign15880_ad_e20424);
        }

        if (((((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) && (!(s.v[607] != 0.0))) && (!(s.v[608] != 0.0))) {
            s.store_scale_ad(559, A::offset(A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) {
            s.store_div_from_scalar(560, 1.0, 559);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) {
            s.store_square(557, 560);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) {
            s.store_mul_ad_lhs(557, A::offset(A::scale(A::sub(s.ad_value(547), s.ad_value(149)), s.v[9]), 1.0), 150);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) {
            s.store_sqrt(560, 557);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) {
            s.store_div_from_scalar(559, 1.0, 560);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_offset(557, 557, (-1.0));
        }

        s.v[609] = if (s.v[547] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[609] != 0.0)) {
            s.store_scale_ad(561, A::ln(A::add(A::offset(s.ad_value(559), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(559), 1.0), A::offset(s.ad_value(559), 3.0))))), (s.v[8] * 2.0));
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[609] != 0.0))) {
            s.store_sub_ad_lhs(561, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(560), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(560), 1.0), A::offset(A::scale(s.ad_value(560), 3.0), 1.0))))), (s.v[8] * 2.0)), 547);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_sub(562, 151, 561);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(563, A::sub(A::add(s.ad_value(547), s.ad_value(562)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(562)), A::sub(s.ad_value(547), s.ad_value(562))), ((4.0 * s.v[8]) * s.v[8])))), 0.5);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(564, A::sub(A::add(s.ad_value(547), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(154)), A::sub(s.ad_value(547), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])))), 0.5);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(565, A::sub(s.ad_value(547), A::sqrt(A::offset(A::mul(s.ad_value(547), s.ad_value(547)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[610] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[610] != 0.0)) {
            s.store_scalar(538, 0.0);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[610] != 0.0)) {
            s.store_scalar(539, 0.0);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) {
            s.store_scale(567, 557, s.v[25]);
        }

        s.v[611] = if ((p.p30 == 0.0) && (p.p35 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (s.v[611] != 0.0)) {
            s.store_scalar(568, 0.0);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_sub_from_scalar(569, s.v[31], 563);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_sub_from_scalar_ad(570, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(569)))));
        }

        s.v[612] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) && (s.v[612] != 0.0)) {
            s.store_scalar(571, 0.0);
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) && (!(s.v[612] != 0.0))) {
            s.store_scale_ad(571, A::add(A::div(A::mul(A::square(s.ad_value(570)), A::ln(s.ad_value(570))), A::sub_from_scalar(1.0, s.ad_value(570))), s.ad_value(570)), (1.0 - (2.0 * p.p21)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_add(572, 570, 571);
        }

        s.v[613] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) && (s.v[613] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(s.ad_value(569), s.v[67]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) && (!(s.v[613] != 0.0))) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[67]), p.p21);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_scale(573, 566, s.v[61]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_scale_ad(574, A::mul(A::offset(s.ad_value(560), (-1.0)), s.ad_value(573)), s.v[22]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_scaled_mul(568, 574, 572, p.p30);
        }

        s.v[614] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (s.v[614] != 0.0)) {
            s.store_scalar(575, 0.0);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_scale_ad(576, A::div(A::scale(s.ad_value(573), s.v[46]), s.ad_value(569)), s.v[76]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[73]), 576);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_square(578, 577);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sqrt_ad(579, A::div(A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sqrt(580, 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_mul(581, 579, 580);
        }

        s.v[615] = if (((-p.p21) * s.v[49]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) {
            s.store_div_from_scalar_ad(582, 1.0, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[615] != 0.0))) {
            s.store_powf_ad(582, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0), ((-p.p21) * s.v[49]));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_div_ad(583, A::mul(s.ad_value(572), s.ad_value(582)), A::add(s.ad_value(572), s.ad_value(582)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sqrt_ad(584, A::scale(A::div(s.ad_value(576), s.ad_value(580)), 0.375));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[73]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[73])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_mul_ad_lhs(587, A::offset(s.ad_value(585), (-1.0)), 584);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_square(548, 587);
        }

        s.v[616] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[616] != 0.0)) {
            s.store_div_from_scalar_ad(549, 1.0, A::offset(A::scale(s.ad_value(587), s.v[10]), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[616] != 0.0))) {
            s.store_div_from_scalar_ad(549, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(587), s.v[10])));
        }

        s.v[617] = if (((-s.v[548]) + s.v[586]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[617] != 0.0)) {
            s.store_exp_ad(566, A::sub(s.ad_value(586), s.ad_value(548)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[617] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_mul_ad_lhs(550, A::add(A::add(A::scale(s.ad_value(549), 0.29214664), A::scale(A::square(s.ad_value(549)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(549)), s.ad_value(549)), s.v[12])), 566);
        }

        s.v[618] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[618] != 0.0)) {
            s.copy_ad(588, 550);
        }

        s.v[619] = if (s.v[586] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[618] != 0.0))) && (s.v[619] != 0.0)) {
            s.store_exp(566, 586);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[618] != 0.0))) && (!(s.v[619] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[618] != 0.0))) {
            s.store_sub_ad_lhs(588, A::scale(s.ad_value(566), 2.0), 550);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_scale_ad(589, A::div(A::scale(s.ad_value(588), s.v[73]), s.ad_value(584)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_scale_ad(575, A::mul(A::mul(s.ad_value(574), s.ad_value(589)), s.ad_value(583)), p.p35);
        }

        s.v[620] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (s.v[620] != 0.0)) {
            s.store_scalar(590, 0.0);
        }

        s.v[621] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (!(s.v[621] != 0.0))) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]), p.p21);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) {
            s.store_scale_ad(591, A::div(A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[64]), s.ad_value(566)), s.v[49]);
        }

        s.v[622] = if (((((-s.v[79]) / s.v[591])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (s.v[622] != 0.0)) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(79)), s.ad_value(591)));
        }

        s.v[623] = if (((-s.v[79]) / s.v[591]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (!(s.v[622] != 0.0))) && (s.v[623] != 0.0)) {
            let assign16620_ad_e21596: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign16620_ad_e21596);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (!(s.v[622] != 0.0))) && (!(s.v[623] != 0.0))) {
            s.store_scale_ad(566, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) {
            s.store_scale_ad(590, A::mul(A::mul(A::mul(s.ad_value(547), s.ad_value(591)), s.ad_value(591)), s.ad_value(566)), p.p41);
        }

        s.v[624] = if (p.p50 > 1000.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (s.v[624] != 0.0)) {
            s.store_scalar(592, 1.0);
        }

        s.v[625] = if (s.v[565] > ((-s.v[82]) * p.p50)) { 1.0 } else { 0.0 };

        s.v[626] = if (p.p53 == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[624] != 0.0))) && (s.v[625] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_mul_ad(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[86]), A::scale(s.ad_value(565), s.v[86])), A::scale(s.ad_value(565), s.v[86])), A::scale(s.ad_value(565), s.v[86]));
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[624] != 0.0))) && (s.v[625] != 0.0)) && (!(s.v[626] != 0.0))) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[86])), p.p53);
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[624] != 0.0))) && (s.v[625] != 0.0)) {
            s.store_div_from_scalar_ad(592, 1.0, A::sub_from_scalar(1.0, s.ad_value(566)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[624] != 0.0))) && (!(s.v[625] != 0.0))) {
            s.store_offset_ad(592, A::scale(A::offset(s.ad_value(565), (s.v[82] * p.p50)), s.v[89]), s.v[83]);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) {
            s.store_mul_ad_lhs(538, A::scale(A::add(A::add(A::add(s.ad_value(567), s.ad_value(568)), s.ad_value(575)), s.ad_value(590)), p.p10), 592);
        }

        s.v[627] = if (s.v[46] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[627] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])), s.v[46]);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) {
            s.store_scale_ad(539, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[58])), p.p11);
        }

        s.v[628] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[628] != 0.0)) {
            s.store_scalar(540, 0.0);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[628] != 0.0)) {
            s.store_scalar(541, 0.0);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) {
            s.store_scale(567, 557, s.v[26]);
        }

        s.v[629] = if ((p.p31 == 0.0) && (p.p36 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[629] != 0.0)) {
            s.store_scalar(568, 0.0);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_sub_from_scalar(569, s.v[32], 563);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_sub_from_scalar_ad(570, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(569)))));
        }

        s.v[630] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(571, 0.0);
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) && (!(s.v[630] != 0.0))) {
            s.store_scale_ad(571, A::add(A::div(A::mul(A::square(s.ad_value(570)), A::ln(s.ad_value(570))), A::sub_from_scalar(1.0, s.ad_value(570))), s.ad_value(570)), (1.0 - (2.0 * p.p22)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_add(572, 570, 571);
        }

        s.v[631] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) && (s.v[631] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(s.ad_value(569), s.v[68]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) && (!(s.v[631] != 0.0))) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[68]), p.p22);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_scale(573, 566, s.v[62]);
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
        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_scale_ad(574, A::mul(A::offset(s.ad_value(560), (-1.0)), s.ad_value(573)), s.v[23]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_scaled_mul(568, 574, 572, p.p31);
        }

        s.v[632] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[632] != 0.0)) {
            s.store_scalar(575, 0.0);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_scale_ad(576, A::div(A::scale(s.ad_value(573), s.v[47]), s.ad_value(569)), s.v[77]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[74]), 576);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_square(578, 577);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sqrt_ad(579, A::div(A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sqrt(580, 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_mul(581, 579, 580);
        }

        s.v[633] = if (((-p.p22) * s.v[50]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_div_from_scalar_ad(582, 1.0, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[633] != 0.0))) {
            s.store_powf_ad(582, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0), ((-p.p22) * s.v[50]));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_div_ad(583, A::mul(s.ad_value(572), s.ad_value(582)), A::add(s.ad_value(572), s.ad_value(582)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sqrt_ad(584, A::scale(A::div(s.ad_value(576), s.ad_value(580)), 0.375));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[74]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[74])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_mul_ad_lhs(587, A::offset(s.ad_value(585), (-1.0)), 584);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_square(548, 587);
        }

        s.v[634] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (s.v[634] != 0.0)) {
            s.store_div_from_scalar_ad(549, 1.0, A::offset(A::scale(s.ad_value(587), s.v[10]), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[634] != 0.0))) {
            s.store_div_from_scalar_ad(549, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(587), s.v[10])));
        }

        s.v[635] = if (((-s.v[548]) + s.v[586]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (s.v[635] != 0.0)) {
            s.store_exp_ad(566, A::sub(s.ad_value(586), s.ad_value(548)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[635] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_mul_ad_lhs(550, A::add(A::add(A::scale(s.ad_value(549), 0.29214664), A::scale(A::square(s.ad_value(549)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(549)), s.ad_value(549)), s.v[12])), 566);
        }

        s.v[636] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (s.v[636] != 0.0)) {
            s.copy_ad(588, 550);
        }

        s.v[637] = if (s.v[586] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[636] != 0.0))) && (s.v[637] != 0.0)) {
            s.store_exp(566, 586);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[636] != 0.0))) && (!(s.v[637] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_sub_ad_lhs(588, A::scale(s.ad_value(566), 2.0), 550);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_scale_ad(589, A::div(A::scale(s.ad_value(588), s.v[74]), s.ad_value(584)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_scale_ad(575, A::mul(A::mul(s.ad_value(574), s.ad_value(589)), s.ad_value(583)), p.p36);
        }

        s.v[638] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[638] != 0.0)) {
            s.store_scalar(590, 0.0);
        }

        s.v[639] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (s.v[639] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (!(s.v[639] != 0.0))) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]), p.p22);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) {
            s.store_scale_ad(591, A::div(A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[65]), s.ad_value(566)), s.v[50]);
        }

        s.v[640] = if (((((-s.v[80]) / s.v[591])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (s.v[640] != 0.0)) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(80)), s.ad_value(591)));
        }

        s.v[641] = if (((-s.v[80]) / s.v[591]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (!(s.v[640] != 0.0))) && (s.v[641] != 0.0)) {
            let assign17370_ad_e22748: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign17370_ad_e22748);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (!(s.v[640] != 0.0))) && (!(s.v[641] != 0.0))) {
            s.store_scale_ad(566, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) {
            s.store_scale_ad(590, A::mul(A::mul(A::mul(s.ad_value(547), s.ad_value(591)), s.ad_value(591)), s.ad_value(566)), p.p42);
        }

        s.v[642] = if (p.p51 > 1000.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(592, 1.0);
        }

        s.v[643] = if (s.v[565] > ((-s.v[82]) * p.p51)) { 1.0 } else { 0.0 };

        s.v[644] = if (p.p54 == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[643] != 0.0)) && (s.v[644] != 0.0)) {
            s.store_mul_ad(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[87]), A::scale(s.ad_value(565), s.v[87])), A::scale(s.ad_value(565), s.v[87])), A::scale(s.ad_value(565), s.v[87]));
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[643] != 0.0)) && (!(s.v[644] != 0.0))) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[87])), p.p54);
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[643] != 0.0)) {
            s.store_div_from_scalar_ad(592, 1.0, A::sub_from_scalar(1.0, s.ad_value(566)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[642] != 0.0))) && (!(s.v[643] != 0.0))) {
            s.store_offset_ad(592, A::scale(A::offset(s.ad_value(565), (s.v[82] * p.p51)), s.v[90]), s.v[84]);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) {
            s.store_mul_ad_lhs(540, A::scale(A::add(A::add(A::add(s.ad_value(567), s.ad_value(568)), s.ad_value(575)), s.ad_value(590)), p.p10), 592);
        }

        s.v[645] = if (s.v[47] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[645] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])), s.v[47]);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) {
            s.store_scale_ad(541, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[59])), p.p11);
        }

        s.v[646] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[646] != 0.0)) {
            s.store_scalar(542, 0.0);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[646] != 0.0)) {
            s.store_scalar(543, 0.0);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) {
            s.store_scale(567, 557, s.v[27]);
        }

        s.v[647] = if ((p.p32 == 0.0) && (p.p37 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[647] != 0.0)) {
            s.store_scalar(568, 0.0);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_sub_from_scalar(569, s.v[33], 563);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_sub_from_scalar_ad(570, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(569)))));
        }

        s.v[648] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) && (s.v[648] != 0.0)) {
            s.store_scalar(571, 0.0);
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) && (!(s.v[648] != 0.0))) {
            s.store_scale_ad(571, A::add(A::div(A::mul(A::square(s.ad_value(570)), A::ln(s.ad_value(570))), A::sub_from_scalar(1.0, s.ad_value(570))), s.ad_value(570)), (1.0 - (2.0 * p.p23)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_add(572, 570, 571);
        }

        s.v[649] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) && (s.v[649] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(s.ad_value(569), s.v[69]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) && (!(s.v[649] != 0.0))) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[69]), p.p23);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_scale(573, 566, s.v[63]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_scale_ad(574, A::mul(A::offset(s.ad_value(560), (-1.0)), s.ad_value(573)), s.v[24]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_scaled_mul(568, 574, 572, p.p32);
        }

        s.v[650] = if (p.p37 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[650] != 0.0)) {
            s.store_scalar(575, 0.0);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_scale_ad(576, A::div(A::scale(s.ad_value(573), s.v[48]), s.ad_value(569)), s.v[78]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[75]), 576);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_square(578, 577);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sqrt_ad(579, A::div(A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sqrt(580, 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_mul(581, 579, 580);
        }

        s.v[651] = if (((-p.p23) * s.v[51]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (s.v[651] != 0.0)) {
            s.store_div_from_scalar_ad(582, 1.0, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[651] != 0.0))) {
            s.store_powf_ad(582, A::offset(A::mul(s.ad_value(576), s.ad_value(581)), 1.0), ((-p.p23) * s.v[51]));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_div_ad(583, A::mul(s.ad_value(572), s.ad_value(582)), A::add(s.ad_value(572), s.ad_value(582)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sqrt_ad(584, A::scale(A::div(s.ad_value(576), s.ad_value(580)), 0.375));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[75]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[75])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_mul_ad_lhs(587, A::offset(s.ad_value(585), (-1.0)), 584);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_square(548, 587);
        }

        s.v[652] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (s.v[652] != 0.0)) {
            s.store_div_from_scalar_ad(549, 1.0, A::offset(A::scale(s.ad_value(587), s.v[10]), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[652] != 0.0))) {
            s.store_div_from_scalar_ad(549, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(587), s.v[10])));
        }

        s.v[653] = if (((-s.v[548]) + s.v[586]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (s.v[653] != 0.0)) {
            s.store_exp_ad(566, A::sub(s.ad_value(586), s.ad_value(548)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[653] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_mul_ad_lhs(550, A::add(A::add(A::scale(s.ad_value(549), 0.29214664), A::scale(A::square(s.ad_value(549)), s.v[11])), A::scale(A::mul(A::square(s.ad_value(549)), s.ad_value(549)), s.v[12])), 566);
        }

        s.v[654] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (s.v[654] != 0.0)) {
            s.copy_ad(588, 550);
        }

        s.v[655] = if (s.v[586] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[654] != 0.0))) && (s.v[655] != 0.0)) {
            s.store_exp(566, 586);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[654] != 0.0))) && (!(s.v[655] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[654] != 0.0))) {
            s.store_sub_ad_lhs(588, A::scale(s.ad_value(566), 2.0), 550);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_scale_ad(589, A::div(A::scale(s.ad_value(588), s.v[75]), s.ad_value(584)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_scale_ad(575, A::mul(A::mul(s.ad_value(574), s.ad_value(589)), s.ad_value(583)), p.p37);
        }

        s.v[656] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[656] != 0.0)) {
            s.store_scalar(590, 0.0);
        }

        s.v[657] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (s.v[657] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (!(s.v[657] != 0.0))) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]), p.p23);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) {
            s.store_scale_ad(591, A::div(A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[66]), s.ad_value(566)), s.v[51]);
        }

        s.v[658] = if (((((-s.v[81]) / s.v[591])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (s.v[658] != 0.0)) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(81)), s.ad_value(591)));
        }

        s.v[659] = if (((-s.v[81]) / s.v[591]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (!(s.v[658] != 0.0))) && (s.v[659] != 0.0)) {
            let assign18120_ad_e23900: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign18120_ad_e23900);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (!(s.v[658] != 0.0))) && (!(s.v[659] != 0.0))) {
            s.store_scale_ad(566, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) {
            s.store_scale_ad(590, A::mul(A::mul(A::mul(s.ad_value(547), s.ad_value(591)), s.ad_value(591)), s.ad_value(566)), p.p43);
        }

        s.v[660] = if (p.p52 > 1000.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(592, 1.0);
        }

        s.v[661] = if (s.v[565] > ((-s.v[82]) * p.p52)) { 1.0 } else { 0.0 };

        s.v[662] = if (p.p55 == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_mul_ad(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[88]), A::scale(s.ad_value(565), s.v[88])), A::scale(s.ad_value(565), s.v[88])), A::scale(s.ad_value(565), s.v[88]));
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[88])), p.p55);
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[661] != 0.0)) {
            s.store_div_from_scalar_ad(592, 1.0, A::sub_from_scalar(1.0, s.ad_value(566)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[660] != 0.0))) && (!(s.v[661] != 0.0))) {
            s.store_offset_ad(592, A::scale(A::offset(s.ad_value(565), (s.v[82] * p.p52)), s.v[91]), s.v[85]);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) {
            s.store_mul_ad_lhs(542, A::scale(A::add(A::add(A::add(s.ad_value(567), s.ad_value(568)), s.ad_value(575)), s.ad_value(590)), p.p10), 592);
        }

        s.v[663] = if (s.v[111] == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
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
            s.store_ad(593, &assign18250_ad_e24158);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_div(552, 152, 153);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add(554, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sub(555, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

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
        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(594, A::div(A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
        }

        s.v[664] = if (s.v[48] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (s.v[664] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (!(s.v[664] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])), s.v[48]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(543, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(593), s.ad_value(594)), s.v[60])), p.p11);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sub_ad_lhs(593, A::offset(s.ad_value(547), p.p60), 593);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_div(552, 152, 153);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add(554, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sub(555, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(594, A::div(A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
        }

        s.v[665] = if (s.v[105] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (s.v[665] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (!(s.v[665] != 0.0))) {
            s.store_ad(566, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))), s.ad_value(105)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(110, A::add(A::mul(s.ad_value(108), A::sub_from_scalar(1.0, s.ad_value(566))), A::mul(s.ad_value(109), A::sub(s.ad_value(593), s.ad_value(594)))), p.p11);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add(543, 543, 110);
        }

        s.v[666] = if (s.v[48] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[666] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[666] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])), s.v[48]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(543, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[60])), p.p11);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_add_ad(544, A::add(A::scale(s.ad_value(538), s.v[143]), A::scale(s.ad_value(540), s.v[144])), A::scale(s.ad_value(542), s.v[145]));
        }

        s.store_add_ad(545, A::add(A::scale(s.ad_value(539), s.v[143]), A::scale(s.ad_value(541), s.v[144])), A::scale(s.ad_value(543), s.v[145]));

        s.store_scale_ad(546, A::abs(s.ad_value(544)), (2.0 * 1.6021918e-19));

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
        s.v[1] = (8.8541878176e-12 * 11.8);

        s.v[112] = 0.0;

        s.v[187] = if (p.p62 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[187] != 0.0) {
            s.store_scalar(112, 1.0);
        }

        if (!(s.v[187] != 0.0)) {
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

        s.v[188] = if ((((p.p56 != 1.0) || (p.p57 != 1.0)) || (p.p58 != 1.0)) || (p.p59 != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[188] != 0.0) {
            s.store_scalar(111, 1.0);
        }

        if (!(s.v[188] != 0.0)) {
            s.store_scalar(111, 0.0);
        }

        s.v[189] = if (s.v[111] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[189] != 0.0) {
            s.store_scalar(95, (if ((p.p17 * p.p56) > 1e-18) { (p.p17 * p.p56) } else { 1e-18 }));
        }

        if (s.v[189] != 0.0) {
            s.store_scalar(96, (if ((p.p20 * p.p57) > 0.05) { (p.p20 * p.p57) } else { 0.05 }));
        }

        if (s.v[189] != 0.0) {
            s.store_scalar(97, (if ((if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) < 0.95) { (if ((p.p23 * p.p58) > 0.05) { (p.p23 * p.p58) } else { 0.05 }) } else { 0.95 }));
        }

        if (s.v[189] != 0.0) {
            s.store_scalar(98, (p.p26 * p.p59));
        }

        if (s.v[189] != 0.0) {
            s.store_offset(100, 98, s.v[13]);
        }

        if (s.v[189] != 0.0) {
            s.store_sub_from_scalar(105, 1.0, 97);
        }

        if (s.v[189] != 0.0) {
            s.store_div_from_scalar(106, 1.0, 105);
        }

        s.v[3] = (((ctx.temperature() + p.p2) + p.p9)).max((273.15 + (-250.0)));

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

        if !(s.v[79] > 0.0) {
            s.store_scalar(79, 0.0);
        }

        if !(s.v[80] > 0.0) {
            s.store_scalar(80, 0.0);
        }

        if !(s.v[81] > 0.0) {
            s.store_scalar(81, 0.0);
        }

        s.v[190] = if (s.v[111] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[190] != 0.0) {
            s.store_offset(99, 98, s.v[14]);
        }

        if (s.v[190] != 0.0) {
            s.store_scale_ad(101, A::exp(A::scale(A::sub(A::scale(s.ad_value(100), s.v[7]), A::scale(s.ad_value(99), s.v[9])), 0.5)), ((s.v[4]) as f64).powf(1.5));
        }

        if (s.v[190] != 0.0) {
            s.store_sub_ad(102, A::scale(s.ad_value(96), s.v[4]), A::scale(A::ln(s.ad_value(101)), (2.0 * s.v[8])));
        }

        if (s.v[190] != 0.0) {
            s.store_add_ad_rhs(103, 102, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(102)), s.v[9])), 1.0)), s.v[8]));
        }

        if (s.v[190] != 0.0) {
            s.store_div_from_scalar(104, 1.0, 103);
        }

        if (s.v[190] != 0.0) {
            s.store_mul_ad_rhs(107, 95, A::pow(A::mul(s.ad_value(96), s.ad_value(104)), s.ad_value(97)));
        }

        if (s.v[190] != 0.0) {
            s.store_mul_ad_lhs(108, A::mul(s.ad_value(107), s.ad_value(103)), 106);
        }

        if (s.v[190] != 0.0) {
            s.store_scale(109, 107, 2.0);
        }

        s.v[143] = (if (p.p3 > 0.0) { p.p3 } else { 0.0 });

        s.v[144] = (if (p.p4 > 0.0) { p.p4 } else { 0.0 });

        s.v[145] = (if (p.p5 > 0.0) { p.p5 } else { 0.0 });

        s.v[0] = (if (p.p6 > 0.0) { p.p6 } else { 0.0 });

        s.v[150] = 0.0;

        s.v[191] = if ((s.v[25] * s.v[143]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[191] != 0.0) {
            s.store_scalar(92, (s.v[8] * ((((p.p12 / (s.v[25] * s.v[143])) + 1.0)) as f64).ln()));
        }

        if (!(s.v[191] != 0.0)) {
            s.store_scalar(92, 100000000.0);
        }

        s.v[192] = if ((s.v[26] * s.v[144]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[192] != 0.0) {
            s.store_scalar(93, (s.v[8] * ((((p.p12 / (s.v[26] * s.v[144])) + 1.0)) as f64).ln()));
        }

        if (!(s.v[192] != 0.0)) {
            s.store_scalar(93, 100000000.0);
        }

        s.v[193] = if ((s.v[27] * s.v[145]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[193] != 0.0) {
            s.store_scalar(94, (s.v[8] * ((((p.p12 / (s.v[27] * s.v[145])) + 1.0)) as f64).ln()));
        }

        if (!(s.v[193] != 0.0)) {
            s.store_scalar(94, 100000000.0);
        }

        s.store_ad(149, &A::min(A::min(s.ad_value(92), s.ad_value(93)), s.ad_value(94)));

        s.v[194] = if ((((s.v[149] * s.v[9])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (s.v[194] != 0.0) {
            s.store_exp_ad(150, A::scale(s.ad_value(149), s.v[9]));
        }

        s.v[195] = if ((s.v[149] * s.v[9]) < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[194] != 0.0)) && (s.v[195] != 0.0)) {
            s.store_div_from_scalar_ad(150, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(149), s.v[9])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[194] != 0.0)) && (!(s.v[195] != 0.0))) {
            s.store_scale_ad(150, A::offset(A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(149), s.v[9]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
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

        s.v[196] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[196] != 0.0) {
            s.store_scalar(34, (s.v[32] + s.v[33]));
        }

        if (s.v[196] != 0.0) {
            s.store_scalar(37, (0.9 * (p.p22).min(p.p23)));
        }

        if (s.v[196] != 0.0) {
            s.store_scalar(40, (p.p19 + p.p20));
        }

        s.v[197] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[197] != 0.0) {
            s.store_scalar(35, (s.v[31] + s.v[33]));
        }

        if (s.v[197] != 0.0) {
            s.store_scalar(38, (0.9 * (p.p21).min(p.p23)));
        }

        if (s.v[197] != 0.0) {
            s.store_scalar(41, (p.p18 + p.p20));
        }

        s.v[198] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[198] != 0.0) {
            s.store_scalar(36, (s.v[31] + s.v[32]));
        }

        if (s.v[198] != 0.0) {
            s.store_scalar(39, (0.9 * (p.p21).min(p.p22)));
        }

        if (s.v[198] != 0.0) {
            s.store_scalar(42, (p.p18 + p.p19));
        }

        s.store_ad(151, &A::min(A::min(s.ad_value(34), s.ad_value(35)), s.ad_value(36)));

        s.store_scale(152, 151, 0.1);

        s.store_ad(15, &A::max(A::max(s.ad_value(37), s.ad_value(38)), s.ad_value(39)));

        s.store_mul_ad_rhs(153, 151, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(15)))));

        s.store_offset_ad(154, A::min(A::min(s.ad_value(40), s.ad_value(41)), s.ad_value(42)), (-0.05));

        s.v[139] = 0.0;

        s.v[146] = 1.0;

        s.v[147] = 1.0;

        s.v[148] = 1.0;

        s.v[199] = if (s.v[112] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[199] != 0.0) {
            s.store_scalar(139, (p.p64 * (((s.v[143] * s.v[52]) + (s.v[144] * s.v[53])) + (s.v[145] * s.v[54]))));
        }

        s.v[534] = if ((s.v[143] * s.v[52]) <= s.v[139]) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[534] != 0.0)) {
            s.store_scalar(146, 0.0);
        }

        s.v[535] = if ((s.v[144] * s.v[53]) <= s.v[139]) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[535] != 0.0)) {
            s.store_scalar(147, 0.0);
        }

        s.v[536] = if ((s.v[145] * s.v[54]) <= s.v[139]) { 1.0 } else { 0.0 };

        if ((s.v[199] != 0.0) && (s.v[536] != 0.0)) {
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

        s.store_ad(547, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(1)), p.p1));

        s.v[595] = if (s.v[112] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[595] != 0.0) {
            s.store_scalar(597, 0.0);
        }

        if (s.v[595] != 0.0) {
            s.store_scalar(598, 0.0);
        }

        if (s.v[595] != 0.0) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if (s.v[595] != 0.0) {
            s.store_div(552, 152, 153);
        }

        if (s.v[595] != 0.0) {
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if (s.v[595] != 0.0) {
            s.store_add(554, 153, 553);
        }

        if (s.v[595] != 0.0) {
            s.store_sub(555, 153, 553);
        }

        if (s.v[595] != 0.0) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

        if (s.v[595] != 0.0) {
            s.store_scale_ad(598, A::div(A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
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
        s.v[599] = if (s.v[146] > 0.5) { 1.0 } else { 0.0 };

        s.v[600] = if (s.v[46] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[595] != 0.0) && (s.v[599] != 0.0)) && (s.v[600] != 0.0)) {
            s.store_sqrt_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])));
        }

        if (((s.v[595] != 0.0) && (s.v[599] != 0.0)) && (!(s.v[600] != 0.0))) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[43])), s.v[46]);
        }

        if ((s.v[595] != 0.0) && (s.v[599] != 0.0)) {
            s.store_add_ad(539, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[58]));
        }

        s.v[601] = if (s.v[147] > 0.5) { 1.0 } else { 0.0 };

        s.v[602] = if (s.v[47] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[595] != 0.0) && (s.v[601] != 0.0)) && (s.v[602] != 0.0)) {
            s.store_sqrt_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])));
        }

        if (((s.v[595] != 0.0) && (s.v[601] != 0.0)) && (!(s.v[602] != 0.0))) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[44])), s.v[47]);
        }

        if ((s.v[595] != 0.0) && (s.v[601] != 0.0)) {
            s.store_add_ad(541, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[59]));
        }

        s.v[603] = if (s.v[148] > 0.5) { 1.0 } else { 0.0 };

        s.v[604] = if (s.v[48] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[595] != 0.0) && (s.v[603] != 0.0)) && (s.v[604] != 0.0)) {
            s.store_sqrt_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])));
        }

        if (((s.v[595] != 0.0) && (s.v[603] != 0.0)) && (!(s.v[604] != 0.0))) {
            s.store_powf_ad(597, A::sub_from_scalar(1.0, A::scale(s.ad_value(598), s.v[45])), s.v[48]);
        }

        if ((s.v[595] != 0.0) && (s.v[603] != 0.0)) {
            s.store_add_ad(543, A::scale(A::sub_from_scalar(1.0, s.ad_value(597)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(598)), s.v[60]));
        }

        if (!(s.v[595] != 0.0)) {
            s.store_scalar(564, 0.0);
        }

        if (!(s.v[595] != 0.0)) {
            s.store_scalar(561, 0.0);
        }

        s.v[605] = if !(((s.v[143] == 0.0) && (s.v[144] == 0.0)) && (s.v[145] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_div(552, 152, 153);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_add_ad_rhs(553, 547, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_add(554, 153, 553);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_sub(555, 153, 553);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(558, A::div(A::mul(s.ad_value(547), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
        }

        s.v[606] = if (s.v[547] < s.v[149]) { 1.0 } else { 0.0 };

        s.v[607] = if (((((-0.5) * (s.v[547] * s.v[9]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) && (s.v[607] != 0.0)) {
            s.store_exp_ad(559, A::scale(s.ad_value(547), (s.v[9] * (-0.5))));
        }

        s.v[608] = if (((-0.5) * (s.v[547] * s.v[9])) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) && (!(s.v[607] != 0.0))) && (s.v[608] != 0.0)) {
            let assign15880_ad_e20424: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(547), (s.v[9] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(559, &assign15880_ad_e20424);
        }

        if (((((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) && (!(s.v[607] != 0.0))) && (!(s.v[608] != 0.0))) {
            s.store_scale_ad(559, A::offset(A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(547), (s.v[9] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) {
            s.store_div_from_scalar(560, 1.0, 559);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[606] != 0.0)) {
            s.store_square(557, 560);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) {
            s.store_mul_ad_lhs(557, A::offset(A::scale(A::sub(s.ad_value(547), s.ad_value(149)), s.v[9]), 1.0), 150);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) {
            s.store_sqrt(560, 557);
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[606] != 0.0))) {
            s.store_div_from_scalar(559, 1.0, 560);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_offset(557, 557, (-1.0));
        }

        s.v[609] = if (s.v[547] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (s.v[609] != 0.0)) {
            s.store_scale_ad(561, A::ln(A::add(A::offset(s.ad_value(559), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(559), 1.0), A::offset(s.ad_value(559), 3.0))))), (s.v[8] * 2.0));
        }

        if (((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) && (!(s.v[609] != 0.0))) {
            s.store_sub_ad_lhs(561, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(560), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(560), 1.0), A::offset(A::scale(s.ad_value(560), 3.0), 1.0))))), (s.v[8] * 2.0)), 547);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_sub(562, 151, 561);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(563, A::sub(A::add(s.ad_value(547), s.ad_value(562)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(562)), A::sub(s.ad_value(547), s.ad_value(562))), ((4.0 * s.v[8]) * s.v[8])))), 0.5);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(564, A::sub(A::add(s.ad_value(547), s.ad_value(154)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(547), s.ad_value(154)), A::sub(s.ad_value(547), s.ad_value(154))), ((4.0 * s.v[6]) * s.v[6])))), 0.5);
        }

        if ((!(s.v[595] != 0.0)) && (s.v[605] != 0.0)) {
            s.store_scale_ad(565, A::sub(s.ad_value(547), A::sqrt(A::offset(A::mul(s.ad_value(547), s.ad_value(547)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[610] = if (s.v[143] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[610] != 0.0)) {
            s.store_scalar(539, 0.0);
        }

        s.v[611] = if ((p.p30 == 0.0) && (p.p35 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_sub_from_scalar(569, s.v[31], 563);
        }

        s.v[613] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) && (s.v[613] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(s.ad_value(569), s.v[67]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) && (!(s.v[613] != 0.0))) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[67]), p.p21);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_scale(573, 566, s.v[61]);
        }

        s.v[614] = if (p.p35 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_scale_ad(576, A::div(A::scale(s.ad_value(573), s.v[46]), s.ad_value(569)), s.v[76]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[73]), 576);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_square(578, 577);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sqrt_ad(579, A::div(A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sqrt(580, 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_mul(581, 579, 580);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sqrt_ad(584, A::scale(A::div(s.ad_value(576), s.ad_value(580)), 0.375));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[73]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[73])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_mul_ad_lhs(587, A::offset(s.ad_value(585), (-1.0)), 584);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) {
            s.store_square(548, 587);
        }

        s.v[617] = if (((-s.v[548]) + s.v[586]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[617] != 0.0)) {
            s.store_exp_ad(566, A::sub(s.ad_value(586), s.ad_value(548)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[617] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[618] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        s.v[619] = if (s.v[586] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[618] != 0.0))) && (s.v[619] != 0.0)) {
            s.store_exp(566, 586);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[618] != 0.0))) && (!(s.v[619] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[620] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        s.v[621] = if (p.p21 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (!(s.v[621] != 0.0))) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[67]), p.p21);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) {
            s.store_scale_ad(591, A::div(A::scale(A::sub_from_scalar(p.p18, s.ad_value(564)), s.v[64]), s.ad_value(566)), s.v[49]);
        }

        s.v[622] = if (((((-s.v[79]) / s.v[591])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (s.v[622] != 0.0)) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(79)), s.ad_value(591)));
        }

        s.v[623] = if (((-s.v[79]) / s.v[591]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (!(s.v[622] != 0.0))) && (s.v[623] != 0.0)) {
            let assign16620_ad_e21596: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(79)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign16620_ad_e21596);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[620] != 0.0))) && (!(s.v[622] != 0.0))) && (!(s.v[623] != 0.0))) {
            s.store_scale_ad(566, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(79)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        s.v[624] = if (p.p50 > 1000.0) { 1.0 } else { 0.0 };

        s.v[625] = if (s.v[565] > ((-s.v[82]) * p.p50)) { 1.0 } else { 0.0 };

        s.v[626] = if (p.p53 == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[624] != 0.0))) && (s.v[625] != 0.0)) && (s.v[626] != 0.0)) {
            s.store_mul_ad(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[86]), A::scale(s.ad_value(565), s.v[86])), A::scale(s.ad_value(565), s.v[86])), A::scale(s.ad_value(565), s.v[86]));
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[624] != 0.0))) && (s.v[625] != 0.0)) && (!(s.v[626] != 0.0))) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[86])), p.p53);
        }

        s.v[627] = if (s.v[46] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) && (!(s.v[627] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[43])), s.v[46]);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[610] != 0.0))) {
            s.store_scale_ad(539, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[55]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[58])), p.p11);
        }

        s.v[628] = if (s.v[144] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[628] != 0.0)) {
            s.store_scalar(541, 0.0);
        }

        s.v[629] = if ((p.p31 == 0.0) && (p.p36 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_sub_from_scalar(569, s.v[32], 563);
        }

        s.v[631] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) && (s.v[631] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(s.ad_value(569), s.v[68]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) && (!(s.v[631] != 0.0))) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[68]), p.p22);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            s.store_scale(573, 566, s.v[62]);
        }

        s.v[632] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_scale_ad(576, A::div(A::scale(s.ad_value(573), s.v[47]), s.ad_value(569)), s.v[77]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[74]), 576);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_square(578, 577);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sqrt_ad(579, A::div(A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sqrt(580, 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_mul(581, 579, 580);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sqrt_ad(584, A::scale(A::div(s.ad_value(576), s.ad_value(580)), 0.375));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[74]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[74])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_mul_ad_lhs(587, A::offset(s.ad_value(585), (-1.0)), 584);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) {
            s.store_square(548, 587);
        }

        s.v[635] = if (((-s.v[548]) + s.v[586]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (s.v[635] != 0.0)) {
            s.store_exp_ad(566, A::sub(s.ad_value(586), s.ad_value(548)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[635] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[636] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        s.v[637] = if (s.v[586] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[636] != 0.0))) && (s.v[637] != 0.0)) {
            s.store_exp(566, 586);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[632] != 0.0))) && (!(s.v[636] != 0.0))) && (!(s.v[637] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[638] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        s.v[639] = if (p.p22 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (s.v[639] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (!(s.v[639] != 0.0))) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[68]), p.p22);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) {
            s.store_scale_ad(591, A::div(A::scale(A::sub_from_scalar(p.p19, s.ad_value(564)), s.v[65]), s.ad_value(566)), s.v[50]);
        }

        s.v[640] = if (((((-s.v[80]) / s.v[591])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (s.v[640] != 0.0)) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(80)), s.ad_value(591)));
        }

        s.v[641] = if (((-s.v[80]) / s.v[591]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (!(s.v[640] != 0.0))) && (s.v[641] != 0.0)) {
            let assign17370_ad_e22748: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(80)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign17370_ad_e22748);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[638] != 0.0))) && (!(s.v[640] != 0.0))) && (!(s.v[641] != 0.0))) {
            s.store_scale_ad(566, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(80)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        s.v[642] = if (p.p51 > 1000.0) { 1.0 } else { 0.0 };

        s.v[643] = if (s.v[565] > ((-s.v[82]) * p.p51)) { 1.0 } else { 0.0 };

        s.v[644] = if (p.p54 == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[643] != 0.0)) && (s.v[644] != 0.0)) {
            s.store_mul_ad(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[87]), A::scale(s.ad_value(565), s.v[87])), A::scale(s.ad_value(565), s.v[87])), A::scale(s.ad_value(565), s.v[87]));
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[643] != 0.0)) && (!(s.v[644] != 0.0))) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[87])), p.p54);
        }

        s.v[645] = if (s.v[47] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[645] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[44])), s.v[47]);
        }

        if ((!(s.v[595] != 0.0)) && (!(s.v[628] != 0.0))) {
            s.store_scale_ad(541, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[56]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[59])), p.p11);
        }

        s.v[646] = if (s.v[145] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[595] != 0.0)) && (s.v[646] != 0.0)) {
            s.store_scalar(543, 0.0);
        }

        s.v[647] = if ((p.p32 == 0.0) && (p.p37 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_sub_from_scalar(569, s.v[33], 563);
        }

        s.v[649] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) && (s.v[649] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(s.ad_value(569), s.v[69]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) && (!(s.v[649] != 0.0))) {
            s.store_powf_ad(566, A::scale(s.ad_value(569), s.v[69]), p.p23);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_scale(573, 566, s.v[63]);
        }

        s.v[650] = if (p.p37 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_scale_ad(576, A::div(A::scale(s.ad_value(573), s.v[48]), s.ad_value(569)), s.v[78]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_div_from_scalar(577, (0.666666666666667 * s.v[75]), 576);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_square(578, 577);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sqrt_ad(579, A::div(A::square(s.ad_value(578)), A::offset(A::square(s.ad_value(578)), 1.0)));
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
        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sqrt(580, 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_mul(581, 579, 580);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sqrt_ad(584, A::scale(A::div(s.ad_value(576), s.ad_value(580)), 0.375));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_sub_ad_lhs(585, A::scale(A::mul(s.ad_value(577), s.ad_value(580)), 2.0), 579);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_add_ad(586, A::sub(A::mul(A::scale(s.ad_value(577), s.v[75]), s.ad_value(580)), A::scale(s.ad_value(579), s.v[75])), A::scale(A::mul(s.ad_value(576), s.ad_value(581)), 0.5));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_mul_ad_lhs(587, A::offset(s.ad_value(585), (-1.0)), 584);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_square(548, 587);
        }

        s.v[653] = if (((-s.v[548]) + s.v[586]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (s.v[653] != 0.0)) {
            s.store_exp_ad(566, A::sub(s.ad_value(586), s.ad_value(548)));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[653] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(586), s.ad_value(548))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[654] = if (s.v[587] > 0.0) { 1.0 } else { 0.0 };

        s.v[655] = if (s.v[586] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[654] != 0.0))) && (s.v[655] != 0.0)) {
            s.store_exp(566, 586);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[650] != 0.0))) && (!(s.v[654] != 0.0))) && (!(s.v[655] != 0.0))) {
            s.store_div_from_scalar_ad(566, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(586)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[656] = if (p.p43 == 0.0) { 1.0 } else { 0.0 };

        s.v[657] = if (p.p23 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (s.v[657] != 0.0)) {
            s.store_sqrt_ad(566, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (!(s.v[657] != 0.0))) {
            s.store_powf_ad(566, A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[69]), p.p23);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) {
            s.store_scale_ad(591, A::div(A::scale(A::sub_from_scalar(p.p20, s.ad_value(564)), s.v[66]), s.ad_value(566)), s.v[51]);
        }

        s.v[658] = if (((((-s.v[81]) / s.v[591])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (s.v[658] != 0.0)) {
            s.store_exp_ad(566, A::div(A::neg(s.ad_value(81)), s.ad_value(591)));
        }

        s.v[659] = if (((-s.v[81]) / s.v[591]) < 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (!(s.v[658] != 0.0))) && (s.v[659] != 0.0)) {
            let assign18120_ad_e23900: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(81)), s.ad_value(591))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(566, 1e-100, assign18120_ad_e23900);
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[656] != 0.0))) && (!(s.v[658] != 0.0))) && (!(s.v[659] != 0.0))) {
            s.store_scale_ad(566, A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(81)), s.ad_value(591)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        s.v[660] = if (p.p52 > 1000.0) { 1.0 } else { 0.0 };

        s.v[661] = if (s.v[565] > ((-s.v[82]) * p.p52)) { 1.0 } else { 0.0 };

        s.v[662] = if (p.p55 == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_mul_ad(566, A::mul(A::mul(A::scale(s.ad_value(565), s.v[88]), A::scale(s.ad_value(565), s.v[88])), A::scale(s.ad_value(565), s.v[88])), A::scale(s.ad_value(565), s.v[88]));
        }

        if (((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) {
            s.store_powf_ad(566, A::abs(A::scale(s.ad_value(565), s.v[88])), p.p55);
        }

        s.v[663] = if (s.v[111] == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
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
            s.store_ad(593, &assign18250_ad_e24158);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_div(552, 152, 153);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add(554, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sub(555, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(594, A::div(A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
        }

        s.v[664] = if (s.v[48] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (s.v[664] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (!(s.v[664] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(594), s.v[45])), s.v[48]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(543, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(593), s.ad_value(594)), s.v[60])), p.p11);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sub_ad_lhs(593, A::offset(s.ad_value(547), p.p60), 593);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_mul_ad_lhs(551, A::scale(s.ad_value(152), 4.0), 152);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_div(552, 152, 153);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add_ad_rhs(553, 593, A::mul(s.ad_value(152), s.ad_value(552)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add(554, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sub(555, 153, 553);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_sqrt_ad(556, A::add(A::square(s.ad_value(555)), s.ad_value(551)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(594, A::div(A::mul(s.ad_value(593), s.ad_value(153)), A::add(s.ad_value(554), s.ad_value(556))), 2.0);
        }

        s.v[665] = if (s.v[105] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (s.v[665] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) && (!(s.v[665] != 0.0))) {
            s.store_ad(566, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(594), s.ad_value(104))), s.ad_value(105)));
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scale_ad(110, A::add(A::mul(s.ad_value(108), A::sub_from_scalar(1.0, s.ad_value(566))), A::mul(s.ad_value(109), A::sub(s.ad_value(593), s.ad_value(594)))), p.p11);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_add(543, 543, 110);
        }

        s.v[666] = if (s.v[48] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[666] != 0.0)) {
            s.store_sqrt_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])));
        }

        if ((((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[666] != 0.0))) {
            s.store_powf_ad(566, A::sub_from_scalar(1.0, A::scale(s.ad_value(558), s.v[45])), s.v[48]);
        }

        if (((!(s.v[595] != 0.0)) && (!(s.v[646] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(543, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(566)), s.v[57]), A::scale(A::sub(s.ad_value(547), s.ad_value(558)), s.v[60])), p.p11);
        }

        s.store_add_ad(545, A::add(A::scale(s.ad_value(539), s.v[143]), A::scale(s.ad_value(541), s.v[144])), A::scale(s.ad_value(543), s.v[145]));

    }
}
