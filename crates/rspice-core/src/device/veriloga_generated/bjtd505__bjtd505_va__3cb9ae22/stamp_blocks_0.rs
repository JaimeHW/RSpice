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
        s.b[439] = (p.p3 == 1.0);
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if s.b[439] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[439]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[150] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.b[440] = (p.p137 == 0.0);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if s.b[440] {
            s.store_scalar(315, 1e-12);
        }

        if (!s.b[440]) {
            s.store_scalar(315, p.p137);
        }

        s.store_scale(316, 315, p.p1);

        s.v[52] = 0.001;

        s.v[312] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[259] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.b[441] = ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if s.b[441] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[441]) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        let assign270_e676: f64 = (2.0 - s.v[76]);
        let assign270_e677: f64 = (2.0_f64).powf(assign270_e676);
        s.v[79] = assign270_e677;

        s.v[259] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.b[442] = ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if s.b[442] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[442]) {
            s.store_scalar(88, ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p116;

        let assign340_e748: f64 = (1.0 / s.v[87]);
        s.v[86] = assign340_e748;

        s.v[168] = 1.0;

        s.v[196] = 0.0;

        s.v[228] = 0.0;

        s.v[211] = 0.0;

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

        s.v[254] = ((s.v[4]) as f64).ln();

        s.store_scaled_offset(259, 74, (((-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))) + ((-0.05))), 10.0);

        s.b[443] = ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05);
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if s.b[443] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(259)), 0.1, 0.05);
        }

        if (!s.b[443]) {
            s.store_add_scaled_inputs_ad(70, A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.1);
        }

        s.store_scaled_offset(259, 88, (((-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))) + ((-0.05))), 10.0);

        s.b[444] = ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        let (assign660_e900,) = {
    if s.b[444] {
        let assign660_e894: f64 = (s.v[259]).exp();
        let assign660_e895: f64 = (1.0 + assign660_e894);
        let assign660_e896: f64 = (assign660_e895).ln();
        let assign660_e897: f64 = (0.1 * assign660_e896);
        let assign660_e898: f64 = (0.05 + assign660_e897);
        (assign660_e898,)
    } else {
        (s.v[85],)
    }
};
        s.v[85] = assign660_e900;

        let (assign670_e924,) = {
    if (!s.b[444]) {
        let assign670_e906: f64 = (p.p117 * s.v[2]);
        let assign670_e908: f64 = (assign670_e906 * s.v[2]);
        let assign670_e911: f64 = (s.v[2] + p.p118);
        let assign670_e912: f64 = (assign670_e908 / assign670_e911);
        let assign670_e913: f64 = (s.v[88] - assign670_e912);
        let assign670_e917: f64 = (-s.v[259]);
        let assign670_e918: f64 = (assign670_e917).exp();
        let assign670_e919: f64 = (1.0 + assign670_e918);
        let assign670_e920: f64 = (assign670_e919).ln();
        let assign670_e921: f64 = (0.1 * assign670_e920);
        let assign670_e922: f64 = (assign670_e913 + assign670_e921);
        (assign670_e922,)
    } else {
        (s.v[85],)
    }
};
        s.v[85] = assign670_e924;

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p65 * s.v[4])) + ((1.0 - s.v[4]) * p.p104));

        s.v[259] = ((0.05 - s.v[13]) / s.v[6]);

        s.b[445] = (0.05 < s.v[13]);
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if s.b[445] {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[445]) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p63 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[15]) / s.v[6]);

        s.b[446] = (0.05 < s.v[15]);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if s.b[446] {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[446]) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p79 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[21]) / s.v[6]);

        s.b[447] = (0.05 < s.v[21]);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if s.b[447] {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[447]) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p70 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[18]) / s.v[6]);

        s.b[448] = (0.05 < s.v[18]);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if s.b[448] {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[448]) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[254]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[20]) / s.v[6]);

        s.b[449] = (0.05 < s.v[20]);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        let (assign910_e1160,) = {
    if s.b[449] {
        let assign910_e1154: f64 = (s.v[259]).exp();
        let assign910_e1155: f64 = (1.0 + assign910_e1154);
        let assign910_e1156: f64 = (assign910_e1155).ln();
        let assign910_e1157: f64 = (s.v[6] * assign910_e1156);
        let assign910_e1158: f64 = (s.v[20] + assign910_e1157);
        (assign910_e1158,)
    } else {
        (s.v[19],)
    }
};
        s.v[19] = assign910_e1160;

        let (assign920_e1174,) = {
    if (!s.b[449]) {
        let assign920_e1167: f64 = (-s.v[259]);
        let assign920_e1168: f64 = (assign920_e1167).exp();
        let assign920_e1169: f64 = (1.0 + assign920_e1168);
        let assign920_e1170: f64 = (assign920_e1169).ln();
        let assign920_e1171: f64 = (s.v[6] * assign920_e1170);
        let assign920_e1172: f64 = (0.05 + assign920_e1171);
        (assign920_e1172,)
    } else {
        (s.v[19],)
    }
};
        s.v[19] = assign920_e1174;

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p26 * s.v[4])) + ((1.0 - s.v[4]) * p.p108));

        s.v[259] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[450] = (0.05 < s.v[56]);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        let (assign960_e1210,) = {
    if s.b[450] {
        let assign960_e1204: f64 = (s.v[259]).exp();
        let assign960_e1205: f64 = (1.0 + assign960_e1204);
        let assign960_e1206: f64 = (assign960_e1205).ln();
        let assign960_e1207: f64 = (s.v[6] * assign960_e1206);
        let assign960_e1208: f64 = (s.v[56] + assign960_e1207);
        (assign960_e1208,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign960_e1210;

        let (assign970_e1224,) = {
    if (!s.b[450]) {
        let assign970_e1217: f64 = (-s.v[259]);
        let assign970_e1218: f64 = (assign970_e1217).exp();
        let assign970_e1219: f64 = (1.0 + assign970_e1218);
        let assign970_e1220: f64 = (assign970_e1219).ln();
        let assign970_e1221: f64 = (s.v[6] * assign970_e1220);
        let assign970_e1222: f64 = (0.05 + assign970_e1221);
        (assign970_e1222,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign970_e1224;

        s.store_div_from_scalar(65, 1.0, 14);

        let assign990_e1230: f64 = (1.0 / s.v[19]);
        s.v[67] = assign990_e1230;

        s.store_powf_scaled_input(73, 65, p.p65, p.p66);

        let assign1010_e1238: f64 = (s.v[75] * s.v[67]);
        let assign1010_e1240: f64 = (assign1010_e1238).powf(s.v[76]);
        s.v[90] = assign1010_e1240;

        s.store_scale(23, 73, p.p64);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p70, s.ad_value(17)), p.p71), (1.0 - p.p74), p.p74);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p69);

        s.store_scale(25, 27, p.p74);

        s.v[28] = (p.p53 * (((s.v[254] * p.p96)) as f64).exp());

        s.b[451] = (s.v[28] < s.v[316]);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if s.b[451] {
            s.copy_ad(28, 316);
        }

        s.v[29] = (p.p55 * (((s.v[254] * (p.p97 - p.p95))) as f64).exp());

        s.v[30] = (p.p54 * (((s.v[254] * p.p100)) as f64).exp());

        s.b[452] = (s.v[30] < s.v[316]);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if s.b[452] {
            s.copy_ad(30, 316);
        }

        s.v[32] = (p.p56 * (((s.v[254] * p.p101)) as f64).exp());

        s.v[31] = (p.p59 * (((s.v[254] * p.p98)) as f64).exp());

        s.b[453] = (p.p121 != 0.0);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if s.b[453] {
            s.store_scalar(50, (p.p9 * (1.0 + (s.v[12] * p.p121))));
            s.store_scaled_offset(259, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[454] = (s.v[50] < 1.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if (s.b[453] && s.b[454]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[453] && (!s.b[454])) {
            s.store_add_scaled_inputs_ad_rhs(50, 50, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]);
        }

        if s.b[453] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[453]) {
            s.store_scalar(48, p.p9);
        }

        s.b[455] = (p.p122 != 0.0);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if s.b[455] {
            s.store_scalar(51, (p.p10 * (1.0 + (s.v[12] * p.p122))));
            s.store_scaled_offset(259, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[456] = (s.v[51] < 1.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if (s.b[455] && s.b[456]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[455] && (!s.b[456])) {
            s.store_add_scaled_inputs_ad_rhs(51, 51, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]);
        }

        if s.b[455] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[455]) {
            s.store_scalar(49, p.p10);
        }

        s.v[311] = (p.p42 * (1.0 + (p.p123 * s.v[12])));

        s.v[261] = (s.v[312] * s.v[312]);

        s.v[262] = (s.v[311] * s.v[311]);

        s.b[457] = (s.v[311] < 0.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if s.b[457] {
            s.store_scalar(310, ((0.5 * s.v[261]) / ((((s.v[262] + s.v[261])) as f64).sqrt() - s.v[311])));
        }

        if (!s.b[457]) {
            s.store_scalar(310, (0.5 * ((((s.v[262] + s.v[261])) as f64).sqrt() + s.v[311])));
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_from_scalar((s.v[254] * (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), A::exp(A::div_from_scalar(((-p.p104) * s.v[10]), s.ad_value(48))), p.p8);

        s.v[36] = (p.p11 * (((s.v[254] * (1.0 - p.p97))) as f64).exp());

        s.v[37] = (p.p29 * (((s.v[254] * (1.0 - p.p102))) as f64).exp());

        s.v[42] = ((p.p15 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p16)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p16)) as f64).exp());

        s.v[43] = ((p.p28 * (((s.v[254] * ((4.0 - p.p102) + p.p120))) as f64).exp()) * ((((-p.p111) * s.v[10])) as f64).exp());

        s.store_powf_scaled_input(255, 70, s.v[72], (-0.5));

        s.store_div_from_scalar(256, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(255), p.p34), s.ad_value(256), (p.p65 * (s.v[72] * s.v[72])), 0.0, 65);

        let assign1590_e1753: f64 = (1.0 / s.v[19]);
        s.v[67] = assign1590_e1753;

        let assign1600_e1756: f64 = (s.v[85] * s.v[86]);
        let assign1600_e1758: f64 = (-0.5);
        let assign1600_e1759: f64 = (assign1600_e1756).powf(assign1600_e1758);
        s.v[257] = assign1600_e1759;

        let assign1610_e1762: f64 = (1.0 / s.v[90]);
        s.v[258] = assign1610_e1762;

        let assign1620_e1765: f64 = (p.p36 * s.v[85]);
        let assign1620_e1767: f64 = (assign1620_e1765 * s.v[85]);
        let assign1620_e1769: f64 = (assign1620_e1767 * s.v[257]);
        let assign1620_e1771: f64 = (assign1620_e1769 * s.v[258]);
        let assign1620_e1773: f64 = (assign1620_e1771 * s.v[75]);
        let assign1620_e1775: f64 = (assign1620_e1773 * s.v[67]);
        let assign1620_e1777: f64 = (assign1620_e1775 * s.v[86]);
        let assign1620_e1779: f64 = (assign1620_e1777 * s.v[86]);
        s.v[83] = assign1620_e1779;

        s.v[255] = (((s.v[254] * p.p95)) as f64).exp();

        s.store_scale(40, 27, (p.p13 * s.v[255]));

        s.store_scale(41, 256, (p.p12 * s.v[255]));

        s.v[93] = ((p.p85 * (((s.v[254] * (p.p97 - 2.0))) as f64).exp()) * ((((-p.p119) * s.v[10])) as f64).exp());

        s.v[94] = (p.p86 * (((s.v[254] * ((p.p95 + p.p97) - 1.0))) as f64).exp());

        s.v[95] = (p.p87 * (((s.v[254] * (p.p98 - 1.0))) as f64).exp());

        s.v[96] = ((p.p88 * (s.v[94] + s.v[95])) / (p.p86 + p.p87));

        s.v[97] = (p.p89 * (((s.v[254] * (p.p99 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.b[459] = (s.v[2] < 525.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if s.b[459] {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!s.b[459]) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p91 * (((s.v[254] * p.p95)) as f64).exp());

        s.store_scaled_voltage(230, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(231, ctx, nodes, Some(5), Some(7), p.p3);

        s.store_scaled_voltage(232, ctx, nodes, Some(5), Some(3), p.p3);

        s.store_scaled_voltage(233, ctx, nodes, Some(4), Some(3), p.p3);

        s.store_scaled_voltage(234, ctx, nodes, Some(4), Some(5), p.p3);

        s.store_scaled_voltage(236, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(240, ctx, nodes, Some(1), Some(4), p.p3);

        s.store_scaled_voltage(243, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(244, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(238, ctx, nodes, Some(9), Some(6), p.p3);

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scaled_voltage(237, ctx, nodes, Some(8), Some(9), p.p3);

        s.store_add_scaled_inputs4_indices(235, 234, 1.0, 231, 1.0, 236, -1.0, 238, -1.0);

        s.store_add_scaled_inputs4_indices(242, 240, 1.0, 244, (-1.0), 235, 1.0, 237, -1.0);

        s.store_add(241, 244, 242);

        s.b[466] = ((s.v[231] * s.v[8]) < p.p134);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if s.b[466] {
            s.store_exp_scaled_input(245, 231, s.v[8]);
        }

        if (!s.b[466]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(245, 275, 231, s.v[8], (((-p.p134)) + (1.0)));
        }

        s.b[467] = (((s.v[232] * s.v[8]) / s.v[48]) < p.p134);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_exp_div_scaled_inputs_indices(246, 232, s.v[8], 48, 1.0);
        }

        if (!s.b[467]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(246, 275, A::div_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(48), 1.0), (((-p.p134)) + (1.0)));
        }

        s.b[468] = ((s.v[235] * s.v[8]) < p.p134);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if s.b[468] {
            s.store_exp_scaled_input(248, 235, s.v[8]);
        }

        if (!s.b[468]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(248, 275, 235, s.v[8], (((-p.p134)) + (1.0)));
        }

        s.b[469] = ((s.v[234] * s.v[8]) < p.p134);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if (!s.b[469]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.b[470] = ((s.v[241] * s.v[8]) < p.p134);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if s.b[470] {
            s.store_exp_scaled_input(249, 241, s.v[8]);
        }

        if (!s.b[470]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(249, 275, 241, s.v[8], (((-p.p134)) + (1.0)));
        }

        s.b[471] = (((s.v[241] - s.v[16]) * s.v[8]) < p.p134);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if s.b[471] {
            s.store_exp_scaled_input_ad(252, A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[471]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(252, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[472] = (((s.v[235] - s.v[16]) * s.v[8]) < p.p134);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if s.b[472] {
            s.store_exp_scaled_input_ad(250, A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[472]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(250, 275, A::sub_scaled_inputs(s.ad_value(235), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[473] = (((s.v[231] - s.v[16]) * s.v[8]) < p.p134);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if s.b[473] {
            s.store_exp_scaled_input_ad(251, A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[473]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(251, 275, A::sub_scaled_inputs(s.ad_value(231), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[474] = (((s.v[230] - s.v[16]) * s.v[8]) < p.p134);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if s.b[474] {
            s.store_exp_scaled_input_ad(253, A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[474]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(253, 275, A::sub_scaled_inputs(s.ad_value(230), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(104, 251, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(105, 253, 4.0, 1.0);

        s.store_div_scaled_value_offset_denominator(106, s.ad_value(253), 2.0, s.ad_value(105), 1.0, 1.0);

        s.b[475] = (s.v[106] < p.p136);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if s.b[475] {
            s.store_scalar(106, p.p136);
        }

        s.store_add_scaled_inputs3_mixed_iia(107, 104, s.v[6], 105, ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(104), 1.0, 1.0, A::offset(s.ad_value(105), 1.0), 1.0)), (-s.v[6]));

        s.store_scaled_add(108, 107, 236, 1.0 / (s.v[31]));

        s.b[476] = (s.v[108] > 0.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        s.b[477] = (s.v[230] < 100.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[477]) {
            s.copy_ad(277, 230);
        }

        if (s.b[476] && (!s.b[477])) {
            s.store_offset_ln_ad(277, A::offset(s.ad_value(230), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[476] {
            s.store_add_scaled_inputs3_mixed_iai(109, 16, 1.0, A::ln(A::scale_offset(s.ad_value(108), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), 277, -1.0);
            s.store_scale(272, 16, 0.2);
            s.store_square(261, 272);
            s.store_square(262, 109);
        }

        s.b[478] = (s.v[109] < 0.0);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[478]) {
            s.store_div_scaled_inputs_mixed_ia(110, 261, 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)), 1.0);
        }

        if (s.b[476] && (!s.b[478])) {
            s.store_scaled_add_ad_lhs(110, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 109, 0.5);
        }

        if s.b[476] {
            s.store_div_scaled_product_offset_rhs(111, s.ad_value(110), s.ad_value(110), (p.p61 * p.p60), 1.0, A::scaled_offset(s.ad_value(110), (p.p61 * s.v[31]), p.p60), 1.0);
            s.store_div(265, 108, 111);
            s.store_scaled_offset(259, 265, (-1.0), 1.0 / (p.p62));
        }

        s.b[479] = (s.v[265] < 1.0);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[479]) {
            s.store_offset_scaled_ad(263, A::ln_one_plus_exp(s.ad_value(259)), p.p62, 1.0);
        }

        if (s.b[476] && (!s.b[479])) {
            s.store_add_scaled_inputs_ad_rhs(263, 265, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p62);
        }

        if s.b[476] {
            s.store_scale(112, 263, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
            s.store_scale(113, 110, 1.0 / ((p.p61 * p.p60)));
            s.store_div_scaled_offset_numerator(114, A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(112), s.ad_value(113), A::offset(s.ad_value(113), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(112), 2.0, A::offset(s.ad_value(113), 1.0)), 1.0);
            s.store_div_ad(115, A::add_scaled_sub_value_product(1.0, s.ad_value(114), 1.0, s.ad_value(106), s.ad_value(114), 1.0), A::offset(A::mul(s.ad_value(106), s.ad_value(114)), 1.0));
            s.store_scaled_mul(117, 108, 115, ((0.5 * s.v[31]) * s.v[8]));
            s.store_add_scaled_offset_product_rhs_mixed_iia(266, 117, 2.0, 106, A::add(s.ad_value(106), s.ad_value(117)), 1.0, 1.0);
            s.store_scaled_offset(118, 117, (-1.0), 0.5);
            s.store_add_ad_lhs(260, A::square(s.ad_value(118)), 266);
        }

        s.b[480] = (s.v[117] >= 1.0);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[480]) {
            s.store_add_ad_rhs(119, 118, A::sqrt(s.ad_value(260)));
        }

        if (s.b[476] && (!s.b[480])) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(119, 266, A::sqrt(s.ad_value(260)), 1.0, 118, -1.0);
        }

        s.b[481] = (s.v[119] < p.p135);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[481]) {
            s.store_scalar(119, p.p135);
        }

        if s.b[476] {
            s.store_mul_ad_product_rhs(121, 119, A::offset(s.ad_value(119), 1.0), A::exp_scaled_input(s.ad_value(16), s.v[8]));
            s.store_scaled_offset(123, 108, (-p.p61), (0.5 * p.p60));
            s.store_scale(124, 108, ((p.p60 * s.v[31]) * p.p61));
            s.store_add_ad_rhs(125, 123, A::sqrt(A::add(A::square(s.ad_value(123)), s.ad_value(124))));
        }

        s.b[482] = (p.p72 == 0.0);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[482]) {
            s.store_scale(126, 17, 0.1);
        }

        if (s.b[476] && (!s.b[482])) {
            s.store_mul_offset_ad_rhs(126, 17, A::div_scaled_inputs(s.ad_value(108), 2.0, A::add(s.ad_value(108), s.ad_value(111)), 1.0), 0.1);
        }

        if s.b[476] {
            s.store_div_scaled_value_offset_denominator(127, s.ad_value(108), p.p61, s.ad_value(108), p.p61, 1.0);
            s.store_div_from_scalar_offset_input(199, p.p61, 108, p.p61);
        }

        if (!s.b[476]) {
            s.store_scalar(111, 0.0);
            s.store_div_scaled_value_offset_denominator(119, s.ad_value(251), 2.0, s.ad_value(104), 1.0, 1.0);
            s.copy_ad(121, 245);
        }

        s.b[483] = ((((s.v[236]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[107]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[104] + s.v[105]))));
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if ((!s.b[476]) && s.b[483]) {
            s.store_scaled_add(128, 119, 106, 0.5);
            s.store_div_scaled_value_offset_denominator(115, s.ad_value(128), 1.0, s.ad_value(128), 1.0, 1.0);
        }

        if ((!s.b[476]) && (!s.b[483])) {
            s.store_div_ad_rhs(115, 107, A::add_scaled_inputs3(s.ad_value(107), 1.0, s.ad_value(231), 1.0, s.ad_value(230), -1.0));
        }

        if (!s.b[476]) {
            s.copy_ad(125, 236);
            s.store_scale(126, 17, 0.1);
            s.copy_ad(127, 108);
            s.store_sub_from_scalar_scaled_input(199, 1.0, 127, 1.0 / (p.p61));
        }

        s.store_scale(129, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(273, 14, 0.1);

        s.store_div_scaled_inputs2_indices(259, 232, 1.0, 129, (-1.0), 273, 1.0);

        s.b[484] = (s.v[232] < s.v[129]);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_add_scaled_product_right_ad(130, 232, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[484]) {
            s.store_add_scaled_product_right_ad(130, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_scaled_inputs3_mixed_aii(131, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p66))), 1.0, 232, 3.0, 130, (-3.0));

        s.b[485] = (p.p73 == 1.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.copy_ad(132, 230);
        }

        s.b[486] = (p.p73 == 2.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if ((!s.b[485]) && s.b[486]) {
            s.store_add(132, 230, 125);
        }

        if ((!s.b[485]) && (!s.b[486])) {
            s.copy_ad(132, 231);
        }

        s.store_div_ad(133, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(134, 17, 1.0, A::powf(s.ad_value(133), ((-1.0) / p.p71)));

        s.store_div_scaled_inputs2_indices(259, 132, 1.0, 134, (-1.0), 126, 1.0);

        s.b[487] = (s.v[132] < s.v[134]);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_add_scaled_product_right_ad(135, 132, 1.0, 126, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[487]) {
            s.store_add_scaled_product_right_ad(135, 134, 1.0, 126, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf(136, 199, p.p75);

        s.store_add_ad(137, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(136), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(135), s.ad_value(17))), (1.0 - p.p71))), 1.0 / ((1.0 - p.p71))), A::mul3(s.ad_value(136), s.ad_value(133), A::sub(s.ad_value(132), s.ad_value(135))));

        s.store_add_scaled_product_value_ad(138, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(137)), 1.0, 25, 230, 1.0);

        s.store_scale(139, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(140, 139, 246);

        s.store_div_scaled_value_offset_denominator(142, s.ad_value(140), 1.0, A::sqrt(A::offset(s.ad_value(140), 1.0)), 1.0, 1.0);

        s.store_pow_ad(122, s.ad_value(121), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(141, 139, 122);

        s.store_div_scaled_value_offset_denominator(143, s.ad_value(141), 1.0, A::sqrt(A::offset(s.ad_value(141), 1.0)), 1.0, 1.0);

        s.b[488] = (p.p91 == 0.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_add_ad(144, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));
        }

        if (!s.b[488]) {
            s.store_offset_scaled_div(269, 131, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_div_scaled_inputs_indices(270, 138, (-(s.v[99] * s.v[8])), 40, 1.0);
            s.store_scaled_sub_ad(144, A::exp(s.ad_value(269)), A::exp(s.ad_value(270)), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 144);

        s.b[489] = (s.v[144] < 0.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_div_from_scalar_sub_ad(145, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144));
        }

        if (!s.b[489]) {
            s.store_scaled_add_ad_lhs(145, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 144, 0.5);
        }

        s.store_mul_offset_ad_rhs(146, 145, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_scaled_mul(147, 35, 122, p.p14);

        s.store_mul(148, 35, 246);

        s.store_div_scaled_inputs2_indices(149, 148, 1.0, 147, (-1.0), 146, 1.0);

        s.store_scale(259, 232, 10000.0);

        s.b[490] = (s.v[232] < 0.0);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.store_scaled_ln_one_plus_exp(276, 259, 0.0001);
        }

        if (!s.b[490]) {
            s.store_add_scaled_inputs_ad_rhs(276, 232, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.0001);
        }

        let assign3460_e3234: f64 = (s.v[276] / p.p139);
        s.v[278] = assign3460_e3234;

        s.b[491] = (s.v[278] < p.p134);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if (!s.b[491]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_scaled_offset(259, 232, (-p.p141), 1000.0);

        s.b[493] = (((s.v[232] * s.v[8]) / p.p16) < p.p134);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if s.b[493] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p16)));
        }

        if (!s.b[493]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 232, (s.v[8] * 1.0 / (p.p16)), (((-p.p134)) + (1.0)));
        }

        s.b[494] = (p.p23 == 1.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        s.b[495] = (((s.v[232] - s.v[55]) * s.v[8]) < p.p134);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign3630_e3365,) = {
    if (s.b[494] && s.b[495]) {
        let assign3630_e3360: f64 = (s.v[232] - s.v[55]);
        let assign3630_e3362: f64 = (assign3630_e3360 * s.v[8]);
        let assign3630_e3363: f64 = (assign3630_e3362).exp();
        (assign3630_e3363,)
    } else {
        (s.v[278],)
    }
};
        s.v[278] = assign3630_e3365;

        if (s.b[494] && (!s.b[495])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        let (assign3650_e3390,) = {
    if (s.b[494] && (!s.b[495])) {
        let assign3650_e3382: f64 = (s.v[232] - s.v[55]);
        let assign3650_e3384: f64 = (assign3650_e3382 * s.v[8]);
        let assign3650_e3386: f64 = (assign3650_e3384 - p.p134);
        let assign3650_e3387: f64 = (1.0 + assign3650_e3386);
        let assign3650_e3388: f64 = (s.v[275] * assign3650_e3387);
        (assign3650_e3388,)
    } else {
        (s.v[278],)
    }
};
        s.v[278] = assign3650_e3390;

        s.b[496] = (((s.v[149] / s.v[35]) - 1000.0) < 40.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if (s.b[494] && (!s.b[496])) {
            s.store_scalar(275, ((40.0) as f64).exp());
        }

        s.b[498] = (((s.v[233] * s.v[8]) / p.p18) < p.p134);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p18)));
        }

        if (!s.b[498]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 233, (s.v[8] * 1.0 / (p.p18)), (((-p.p134)) + (1.0)));
        }

        s.b[499] = (p.p23 == 1.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        s.b[500] = (((s.v[233] - s.v[55]) * s.v[8]) < p.p134);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        let (assign3800_e3578,) = {
    if (s.b[499] && s.b[500]) {
        let assign3800_e3573: f64 = (s.v[233] - s.v[55]);
        let assign3800_e3575: f64 = (assign3800_e3573 * s.v[8]);
        let assign3800_e3576: f64 = (assign3800_e3575).exp();
        (assign3800_e3576,)
    } else {
        (s.v[278],)
    }
};
        s.v[278] = assign3800_e3578;

        if (s.b[499] && (!s.b[500])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        let (assign3820_e3603,) = {
    if (s.b[499] && (!s.b[500])) {
        let assign3820_e3595: f64 = (s.v[233] - s.v[55]);
        let assign3820_e3597: f64 = (assign3820_e3595 * s.v[8]);
        let assign3820_e3599: f64 = (assign3820_e3597 - p.p134);
        let assign3820_e3600: f64 = (1.0 + assign3820_e3599);
        let assign3820_e3601: f64 = (s.v[275] * assign3820_e3600);
        (assign3820_e3601,)
    } else {
        (s.v[278],)
    }
};
        s.v[278] = assign3820_e3603;

        s.b[501] = (((s.v[232] * s.v[8]) / p.p20) < p.p134);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p20)));
        }

        if (!s.b[501]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 232, (s.v[8] * 1.0 / (p.p20)), (((-p.p134)) + (1.0)));
        }

        s.b[502] = (((s.v[233] * s.v[8]) / p.p22) < p.p134);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if s.b[502] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p22)));
        }

        if (!s.b[502]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 233, (s.v[8] * 1.0 / (p.p22)), (((-p.p134)) + (1.0)));
        }

        s.b[503] = (((s.v[235] * s.v[8]) / p.p31) < p.p134);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if s.b[503] {
            s.store_exp_scaled_input(276, 235, (s.v[8] * 1.0 / (p.p31)));
        }

        if (!s.b[503]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 235, (s.v[8] * 1.0 / (p.p31)), (((-p.p134)) + (1.0)));
        }

        s.b[504] = (((s.v[233] * s.v[8]) / p.p133) < p.p134);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if s.b[504] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p133)));
        }

        if (!s.b[504]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 233, (s.v[8] * 1.0 / (p.p133)), (((-p.p134)) + (1.0)));
        }

        s.b[505] = (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[232] < 0.0));
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        s.b[506] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p134);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (s.b[505] && (!s.b[506])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if s.b[505] {
            s.store_mul(255, 232, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt_square_offset(s.ad_value(255), 1e-30), ((-2.0) - p.p66)), A::sub(A::scale_offset(A::scale(s.ad_value(255), (3.0 * (p.p66 - 1.0))), (-p.p66), (((1.0 - (p.p66 * p.p66))) * (p.p66))), A::mul3_scaled_output(s.ad_value(255), s.ad_value(255), A::offset(s.ad_value(255), (p.p66 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(255, s.ad_value(232), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0);
        }

        s.b[507] = (s.v[255] < (-0.001));
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        s.b[508] = (s.v[255] < p.p134);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if ((s.b[505] && s.b[507]) && (!s.b[508])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.b[509] = (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[230] < 0.0));
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        let (assign4240_e4061,) = {
    if s.b[509] {
        let assign4240_e4054: f64 = (s.v[230] * s.v[67]);
        let assign4240_e4055: f64 = (1.0 - assign4240_e4054);
        let assign4240_e4058: f64 = (1.0 - s.v[76]);
        let assign4240_e4059: f64 = (assign4240_e4055).powf(assign4240_e4058);
        (assign4240_e4059,)
    } else {
        (s.v[77],)
    }
};
        s.v[77] = assign4240_e4061;

        s.b[510] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p134);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (s.b[509] && (!s.b[510])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        let (assign4290_e4122,) = {
    if s.b[509] {
        let assign4290_e4120: f64 = (s.v[230] * s.v[67]);
        (assign4290_e4120,)
    } else {
        (s.v[257],)
    }
};
        s.v[257] = assign4290_e4122;

        let (assign4300_e4166,) = {
    if s.b[509] {
        let assign4300_e4126: f64 = (s.v[257] * s.v[257]);
        let assign4300_e4128: f64 = (assign4300_e4126 + 1e-30);
        let assign4300_e4129: f64 = (assign4300_e4128).sqrt();
        let assign4300_e4131: f64 = (-2.0);
        let assign4300_e4133: f64 = (assign4300_e4131 - s.v[76]);
        let assign4300_e4134: f64 = (assign4300_e4129).powf(assign4300_e4133);
        let assign4300_e4139: f64 = (s.v[76] * s.v[76]);
        let assign4300_e4140: f64 = (1.0 - assign4300_e4139);
        let assign4300_e4143: f64 = (3.0 * s.v[257]);
        let assign4300_e4146: f64 = (s.v[76] - 1.0);
        let assign4300_e4147: f64 = (assign4300_e4143 * assign4300_e4146);
        let assign4300_e4148: f64 = (assign4300_e4140 - assign4300_e4147);
        let assign4300_e4149: f64 = (s.v[76] * assign4300_e4148);
        let assign4300_e4152: f64 = (6.0 * s.v[257]);
        let assign4300_e4154: f64 = (assign4300_e4152 * s.v[257]);
        let assign4300_e4157: f64 = (s.v[76] - 1.0);
        let assign4300_e4159: f64 = (assign4300_e4157 + s.v[257]);
        let assign4300_e4160: f64 = (assign4300_e4154 * assign4300_e4159);
        let assign4300_e4161: f64 = (assign4300_e4149 - assign4300_e4160);
        let assign4300_e4162: f64 = (assign4300_e4134 * assign4300_e4161);
        let assign4300_e4164: f64 = (assign4300_e4162 * 0.16666666666666666);
        (assign4300_e4164,)
    } else {
        (s.v[80],)
    }
};
        s.v[80] = assign4300_e4166;

        let (assign4310_e4178,) = {
    if s.b[509] {
        let assign4310_e4170: f64 = (s.v[230] * s.v[79]);
        let assign4310_e4172: f64 = (assign4310_e4170 * s.v[83]);
        let assign4310_e4175: f64 = (s.v[85] * s.v[80]);
        let assign4310_e4176: f64 = (assign4310_e4172 / assign4310_e4175);
        (assign4310_e4176,)
    } else {
        (s.v[257],)
    }
};
        s.v[257] = assign4310_e4178;

        s.b[511] = (s.v[257] < (-0.001));
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        s.b[512] = (s.v[257] < p.p134);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[509] && s.b[511]) && (!s.b[512])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_mul(158, 139, 248);

        s.store_scale(159, 250, 4.0);

        s.store_div_scaled_inputs2_mixed_iia(161, 158, 1.0, 139, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(158), 1.0)), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(160, s.ad_value(159), 1.0, A::sqrt(A::offset(s.ad_value(159), 1.0)), 1.0, 1.0);

        s.b[513] = ((p.p5 > 0.0) && (p.p32 > 0.0));
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_div_scaled_offset_numerator(164, s.ad_value(249), ((p.p32 * 2.0) * s.v[43]), ((-1.0) * ((p.p32 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(249), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);
            s.store_scalar(165, 0.0);
        }

        s.b[514] = (p.p5 == 1.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if (s.b[513] && s.b[514]) {
            s.store_scalar(271, ((p.p32 * s.v[43]) * s.v[32]));
            s.store_offset_scaled_ad(166, A::ln_scaled_input(s.ad_value(271), s.v[8]), (-s.v[6]), ((2.0) * (s.v[6])));
            s.store_sub(264, 241, 166);
            s.store_scalar(261, (0.11 * 0.11));
            s.store_square(262, 264);
        }

        s.b[515] = (s.v[264] < 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[513] && s.b[514]) && s.b[515]) {
            s.store_div_scaled_inputs_mixed_ia(167, 261, 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)), 1.0);
        }

        if ((s.b[513] && s.b[514]) && (!s.b[515])) {
            s.store_scaled_add_ad_lhs(167, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 264, 0.5);
        }

        if (s.b[513] && s.b[514]) {
            s.store_div_ad_rhs(168, 167, A::add_scaled_inputs4(s.ad_value(271), 1.0, s.ad_value(164), s.v[32], s.ad_value(165), s.v[32], s.ad_value(167), 1.0));
        }

        if (s.b[513] && (!s.b[514])) {
            s.store_scalar(166, 0.0);
            s.store_scalar(264, 0.0);
            s.store_scalar(167, 0.0);
            s.store_scalar(168, 1.0);
        }

        s.b[516] = (p.p83 == 1.0);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_add(322, 234, 230);
            s.store_scalar(261, (1e-6 * 1e-6));
            s.store_scaled_mul(262, 322, 322, ((-1.0) * (-1.0)));
        }

        s.store_add_ad(172, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 172);

        s.b[519] = (s.v[172] < 0.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_div_from_scalar_sub_ad(173, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172));
        }

        if (!s.b[519]) {
            s.store_scaled_add_ad_lhs(173, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 172, 0.5);
        }

        s.store_mul_offset_ad_rhs(174, 173, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_div_from_scalar(176, s.v[29], 174);

        s.b[520] = (s.v[176] < s.v[316]);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if s.b[520] {
            s.copy_ad(176, 316);
        }

        s.store_scale(175, 176, 3.0);

        s.b[521] = (s.v[149] > 0.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        s.b[522] = (p.p38 == 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        s.b[523] = (s.v[230] < p.p43);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        s.b[524] = (((-s.v[149]) / p.p41) < p.p134);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[524]) {
            s.store_exp_scaled_input(308, 149, (-1.0 / (p.p41)));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[524])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(308, 275, 149, (-1.0 / (p.p41)), (((-p.p134)) + (1.0)));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_sub_from_scalar_lhs(309, p.p43, 230, 308);
        }

        s.b[525] = (((-s.v[310]) * ((s.v[309]) as f64).powf(p.p40)) < p.p134);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[525]) {
            s.store_exp_mul_scaled_lhs_mixed_ia(313, 310, -1.0, A::powf(s.ad_value(309), p.p40));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[525])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, A::powf(s.ad_value(309), p.p40)), (((-p.p134)) + (1.0)));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_ad_product_lhs(196, A::div_from_scalar(p.p39, s.ad_value(310)), s.ad_value(309), 313);
        }

        s.b[526] = (p.p38 == 2.0);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        s.b[527] = (s.v[230] < s.v[16]);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_scalar(185, ((2.0 * p.p45) / (p.p44 * p.p44)));
            s.store_div_scaled_inputs2_indices(260, 16, 1.0, 230, (-1.0), 199, 1.0);
            s.store_sqrt_ad(186, A::div_scaled_inputs(s.ad_value(260), 2.0, s.ad_value(185), 1.0));
        }

        s.b[528] = (p.p7 == 0.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[528]) {
            s.store_scalar(187, p.p44);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[528])) {
            s.store_sub_from_scalar_scaled_input(116, 1.0, 115, 0.5);
            s.store_scaled_mul(187, 116, 116, p.p44);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_product_sqrt_square_sum_denominator(188, 186, 187, 1.0, 186, 187, 1.0);
            s.store_div_scaled_inputs2_indices(189, 16, 1.0, 230, (-1.0), 188, 1.0);
            s.store_add_ad_rhs(190, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), s.ad_value(199), 0.5));
        }

        s.b[529] = (p.p7 == 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[529]) {
            s.copy_ad(191, 190);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[529])) {
            s.store_offset_scaled(192, 115, ((2.0) * ((2.0 * p.p46))), (((2.0 * p.p46)) + (1.0)));
            s.store_scalar(193, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
            s.store_sub_ad_rhs(194, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), A::sub(s.ad_value(193), A::div_scaled_inputs(s.ad_value(149), 1.0, s.ad_value(192), p.p61)), 0.5));
            s.store_add_scaled_product(260, A::mul3_scaled_output(s.ad_value(189), s.ad_value(189), s.ad_value(127), (0.1 * 1.0 / (p.p61))), 1.0, A::sub(s.ad_value(194), s.ad_value(190)), A::sub(s.ad_value(194), s.ad_value(190)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_indices(191, 194, 0.5, 190, 0.5, 260, 0.5);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_inputs2_indices(267, 191, 1.0, 189, (-1.0), 191, 1.0);
        }

        s.b[530] = (((s.v[267]) as f64).abs() > 1e-7);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[530]) {
            s.store_div_scaled_inputs_indices(195, 188, 0.5, 267, 1.0);
            s.store_mul_ad(196, A::mul3(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(191), s.ad_value(195)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0), A::div(s.ad_value(187), s.ad_value(195)), 1.0))));
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[530])) {
            s.store_mul_ad_product_rhs(196, 0, s.ad_value(187), A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)));
        }

        s.b[531] = (p.p38 == 3.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        s.b[532] = (s.v[230] < p.p43);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad(200, A::powf(A::sub_from_scalar(p.p43, s.ad_value(230)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(149), 1.0, s.ad_value(149), p.p47, 1.0)), p.p48));
        }

        s.b[533] = (p.p7 == 0.0);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[533]) {
            s.copy_ad(201, 200);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_scaled_offset(202, 149, (-p.p51), 1.0 / (p.p47));
            s.store_scaled_offset(259, 202, (-1.0), 1.0 / (p.p50));
        }

        s.b[534] = (s.v[202] < 1.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && s.b[534]) {
            s.store_offset_scaled_ad(203, A::ln_one_plus_exp(s.ad_value(259)), p.p50, 1.0);
        }

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && (!s.b[534])) {
            s.store_add_scaled_inputs_ad_rhs(203, 202, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p50);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_mul_powf_ad_rhs(201, 200, s.ad_value(203), p.p49);
        }

        s.b[535] = (((-s.v[310]) * s.v[201]) < p.p134);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[535]) {
            s.store_exp_mul_scaled_lhs_indices(313, 310, -1.0, 201);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[535])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, s.ad_value(201)), (((-p.p134)) + (1.0)));
        }

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad_lhs(196, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p39, s.ad_value(310)), p.p43, s.ad_value(230)), 313);
        }

        s.b[536] = (s.v[196] > 0.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (p.p52 == 1.0);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if ((s.b[521] && s.b[536]) && s.b[537]) {
            s.store_add_scaled_inputs3(197, A::div_from_scalar(s.v[6], A::mul(s.ad_value(149), A::add(s.ad_value(30), s.ad_value(175)))), 1.0, A::div(s.ad_value(146), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(175))), 1.0);
        }

        s.b[538] = (p.p38 == 3.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) {
            s.store_scaled_sub(259, 196, 197, 1000000.0);
        }

        s.b[539] = (s.v[196] < s.v[197]);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && s.b[539]) {
            s.store_sub_scaled_inputs_ad_rhs(196, 196, 1.0, A::ln_one_plus_exp(s.ad_value(259)), 1e-6);
        }

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && (!s.b[539])) {
            s.store_sub_scaled_inputs_ad_rhs(196, 197, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 1e-6);
        }

        s.store_scaled_mul(204, 23, 131, (1.0 - p.p67));

        s.store_div_scaled_inputs2_indices(259, 233, 1.0, 129, (-1.0), 273, 1.0);

        s.b[541] = (s.v[233] < s.v[129]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if s.b[541] {
            s.store_add_scaled_product_right_ad(205, 233, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[541]) {
            s.store_add_scaled_product_right_ad(205, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(206, 23, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(205), s.ad_value(65))), (1.0 - p.p66)), 1.0 / ((1.0 - p.p66))), p.p67, s.ad_value(233), ((3.0) * (p.p67)), s.ad_value(205), (((-3.0)) * (p.p67)), 0.0);

        s.store_scaled_mul(207, 24, 138, p.p76);

        s.v[208] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(212, 142, 173, (0.5 * s.v[208]));

        s.store_scaled_mul(213, 143, 173, (0.5 * s.v[208]));

        s.store_scale(274, 17, 0.1);

        s.store_div_scaled_inputs2_indices(259, 235, 1.0, 134, (-1.0), 274, 1.0);

        s.b[542] = (s.v[235] < s.v[134]);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_add_scaled_product_right_ad(214, 235, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[542]) {
            s.store_add_scaled_product_right_ad(214, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(215, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(214), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(235), s.ad_value(214)), 1.0);

        s.store_mul_add_scaled_product_rhs(216, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(215)), ((1.0 - p.p76) * (1.0 - p.p32)), s.ad_value(25), s.ad_value(235), ((1.0 - p.p76) * (1.0 - p.p32)));

        s.store_div_scaled_inputs2_indices(259, 241, 1.0, 134, (-1.0), 274, 1.0);

        s.b[543] = (s.v[241] < s.v[134]);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_add_scaled_product_right_ad(217, 241, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[543]) {
            s.store_add_scaled_product_right_ad(217, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(218, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(241), s.ad_value(217)), 1.0);

        s.store_mul_add_scaled_product_rhs(219, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(218)), ((1.0 - p.p76) * p.p32), s.ad_value(25), s.ad_value(241), ((1.0 - p.p76) * p.p32));

        s.store_scaled_powf_ad(220, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p84), (s.v[93] * s.v[36]));

        s.b[544] = ((s.v[232] / (p.p84 * s.v[6])) < p.p134);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(276, 232, 1.0 / ((p.p84 * s.v[6])));
        }

        if (!s.b[544]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 232, 1.0 / ((p.p84 * s.v[6])), (((-p.p134)) + (1.0)));
        }

        s.store_mul(222, 220, 276);

        s.v[223] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_offset_ad_rhs(224, 115, (0.5 * s.v[223]), A::add(s.ad_value(119), s.ad_value(106)), 2.0);

        s.b[545] = (p.p78 == 0.0);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_add_scaled_inputs(229, 161, (s.v[208] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 160, (s.v[223] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[546] = ((((s.v[235] - s.v[22]) / p.p90) * s.v[8]) < p.p134);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((!s.b[545]) && s.b[546]) {
            s.store_exp_scaled_input_ad(170, A::sub(s.ad_value(235), s.ad_value(22)), (1.0 / (p.p90) * s.v[8]));
        }

        if ((!s.b[545]) && (!s.b[546])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(170, 275, A::sub_scaled_inputs(s.ad_value(235), (1.0 / (p.p90) * s.v[8]), s.ad_value(22), (1.0 / (p.p90) * s.v[8])), (((-p.p134)) + (1.0)));
        }

        if (!s.b[545]) {
            s.store_div_scaled_value_offset_denominator(229, s.ad_value(248), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(170), 4.0, 1.0)), 1.0, 1.0);
        }

        s.b[547] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0));
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scale(229, 229, s.v[150]);
        }

        s.b[548] = (p.p78 == 0.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[547] && s.b[548]) {
            s.store_mul(162, 139, 249);
            s.store_div_scaled_inputs2_mixed_iia(163, 162, 1.0, 139, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0), 1.0);
            s.store_scale(225, 252, 4.0);
            s.store_div_scaled_value_offset_denominator(226, s.ad_value(225), 1.0, A::sqrt(A::offset(s.ad_value(225), 1.0)), 1.0, 1.0);
            s.store_add_scaled_inputs(227, 163, (s.v[208] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 226, (s.v[223] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[549] = (((s.v[241] - s.v[22]) * s.v[8]) < p.p134);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((s.b[547] && (!s.b[548])) && s.b[549]) {
            s.store_exp_scaled_input_ad(171, A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]);
        }

        if ((s.b[547] && (!s.b[548])) && (!s.b[549])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(171, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(22), s.v[8]), (((-p.p134)) + (1.0)));
        }

        if (s.b[547] && (!s.b[548])) {
            s.store_div_scaled_value_offset_denominator(227, s.ad_value(249), (((2.0 * p.p32) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(171), 4.0, 1.0)), 1.0, 1.0);
        }

        if s.b[547] {
            s.store_mul(228, 168, 227);
        }

        s.b[550] = (p.p6 == 1.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_offset_powf_ad(179, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (-p.p66), (-3.0));
            s.store_div_scaled_inputs2_indices(268, 232, 1.0, 129, (-1.0), 273, 1.0);
        }

        s.b[551] = (s.v[268] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (s.b[550] && s.b[551]) {
            s.store_div_from_scalar_offset_ad(180, 1.0, A::exp(s.ad_value(268)), 1.0);
        }

        if (s.b[550] && (!s.b[551])) {
            s.store_div_ad(180, A::exp_scaled_input(s.ad_value(268), -1.0), A::offset(A::exp_scaled_input(s.ad_value(268), -1.0), 1.0));
        }

        if s.b[550] {
            s.store_offset_mul(178, 179, 180, 3.0);
            s.store_scaled_mul(181, 23, 178, (1.0 - p.p67));
            s.store_mul_ad(184, A::div_scaled_product(s.ad_value(139), s.ad_value(246), s.v[8], s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(140), 1.0))));
            s.store_scaled_mul(182, 173, 184, (0.5 * s.v[208]));
            s.store_scale(183, 222, 1.0 / ((p.p84 * s.v[6])));
            s.store_mul_add_scaled_inputs3_offset_rhs(211, 234, s.ad_value(181), 0.2, s.ad_value(182), 0.2, s.ad_value(183), 0.2, 0.0);
            s.store_scale(221, 222, (1.0 - p.p94));
            s.store_add_scaled_inputs(307, 212, 1.0, 222, p.p94);
            s.store_add_scaled_inputs(210, 307, p.p93, 213, 1.0);
            s.store_scale(209, 307, (1.0 - p.p93));
        }

        if (!s.b[550]) {
            s.copy_ad(209, 212);
            s.copy_ad(210, 213);
            s.copy_ad(221, 222);
        }

        s.store_div_scaled_inputs2_indices(303, 148, 1.0, 147, 1.0, 146, 1.0);

        s.b[556] = (s.v[303] > 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_div_scaled_inputs2_indices(305, 209, 1.0, 210, 1.0, 303, 1.0);
        }

        if (!s.b[556]) {
            s.store_scaled_mul(305, 173, 146, s.v[94]);
        }

        s.b[557] = (p.p130 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if s.b[557] {
            s.store_scale(306, 305, p.p93);
        }

        s.b[558] = (p.p130 == 2.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if ((!s.b[557]) && s.b[558]) {
            s.store_scale(306, 305, p.p131);
        }

        if ((!s.b[557]) && (!s.b[558])) {
            s.store_scalar(306, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[439] = (p.p3 == 1.0);
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if s.b[439] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[439]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[150] = (1.0 - p.p32);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.b[440] = (p.p137 == 0.0);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if s.b[440] {
            s.store_scalar(315, 1e-12);
        }

        if (!s.b[440]) {
            s.store_scalar(315, p.p137);
        }

        s.store_scale(316, 315, p.p1);

        s.v[52] = 0.001;

        s.v[312] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p66));

        s.v[259] = (((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) - 0.05) / 0.1);

        s.b[441] = ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) < 0.05);
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if s.b[441] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[441]) {
            s.store_scalar(74, ((p.p113 + (((p.p114 * s.v[3]) * s.v[3]) / (s.v[3] + p.p115))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p113;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p70;

        s.v[76] = p.p71;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[259] = (((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) - 0.05) / 0.1);

        s.b[442] = ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) < 0.05);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if s.b[442] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[442]) {
            s.store_scalar(88, ((p.p116 + (((p.p117 * s.v[3]) * s.v[3]) / (s.v[3] + p.p118))) + (0.1 * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p116;

        s.v[86] = (1.0 / s.v[87]);

        s.v[168] = 1.0;

        s.v[196] = 0.0;

        s.v[228] = 0.0;

        s.v[211] = 0.0;

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

        s.v[254] = ((s.v[4]) as f64).ln();

        s.store_scaled_offset(259, 74, (((-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))) + ((-0.05))), 10.0);

        s.b[443] = ((s.v[74] - (((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115))) < 0.05);
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if s.b[443] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(259)), 0.1, 0.05);
        }

        if (!s.b[443]) {
            s.store_add_scaled_inputs_ad(70, A::offset(s.ad_value(74), (-(((p.p114 * s.v[2]) * s.v[2]) / (s.v[2] + p.p115)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.1);
        }

        s.store_scaled_offset(259, 88, (((-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))) + ((-0.05))), 10.0);

        s.b[444] = ((s.v[88] - (((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118))) < 0.05);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if s.b[444] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(259)), 0.1, 0.05);
        }

        if (!s.b[444]) {
            s.store_add_scaled_inputs_ad(85, A::offset(s.ad_value(88), (-(((p.p117 * s.v[2]) * s.v[2]) / (s.v[2] + p.p118)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.1);
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p65 * s.v[4])) + ((1.0 - s.v[4]) * p.p104));

        s.v[259] = ((0.05 - s.v[13]) / s.v[6]);

        s.b[445] = (0.05 < s.v[13]);
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if s.b[445] {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[445]) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p63 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[15]) / s.v[6]);

        s.b[446] = (0.05 < s.v[15]);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if s.b[446] {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[446]) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p79 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[21]) / s.v[6]);

        s.b[447] = (0.05 < s.v[21]);
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if s.b[447] {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[447]) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p70 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[18]) / s.v[6]);

        s.b[448] = (0.05 < s.v[18]);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if s.b[448] {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[448]) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[254]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[259] = ((0.05 - s.v[20]) / s.v[6]);

        s.b[449] = (0.05 < s.v[20]);
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if s.b[449] {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[449]) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[254]) + (p.p26 * s.v[4])) + ((1.0 - s.v[4]) * p.p108));

        s.v[259] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[450] = (0.05 < s.v[56]);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if s.b[450] {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[259]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[450]) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[259])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_scaled_input(73, 65, p.p65, p.p66);

        s.store_powf_scaled_input(90, 67, s.v[75], s.v[76]);

        s.store_scale(23, 73, p.p64);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p70, s.ad_value(17)), p.p71), (1.0 - p.p74), p.p74);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p69);

        s.store_scale(25, 27, p.p74);

        s.v[28] = (p.p53 * (((s.v[254] * p.p96)) as f64).exp());

        s.b[451] = (s.v[28] < s.v[316]);
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if s.b[451] {
            s.copy_ad(28, 316);
        }

        s.v[29] = (p.p55 * (((s.v[254] * (p.p97 - p.p95))) as f64).exp());

        s.v[30] = (p.p54 * (((s.v[254] * p.p100)) as f64).exp());

        s.b[452] = (s.v[30] < s.v[316]);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if s.b[452] {
            s.copy_ad(30, 316);
        }

        s.v[32] = (p.p56 * (((s.v[254] * p.p101)) as f64).exp());

        s.v[31] = (p.p59 * (((s.v[254] * p.p98)) as f64).exp());

        s.b[453] = (p.p121 != 0.0);
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if s.b[453] {
            s.store_scalar(50, (p.p9 * (1.0 + (s.v[12] * p.p121))));
            s.store_scaled_offset(259, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[454] = (s.v[50] < 1.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if (s.b[453] && s.b[454]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[453] && (!s.b[454])) {
            s.store_add_scaled_inputs_ad_rhs(50, 50, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]);
        }

        if s.b[453] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[453]) {
            s.store_scalar(48, p.p9);
        }

        s.b[455] = (p.p122 != 0.0);
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if s.b[455] {
            s.store_scalar(51, (p.p10 * (1.0 + (s.v[12] * p.p122))));
            s.store_scaled_offset(259, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[456] = (s.v[51] < 1.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if (s.b[455] && s.b[456]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(259)), s.v[52], 1.0);
        }

        if (s.b[455] && (!s.b[456])) {
            s.store_add_scaled_inputs_ad_rhs(51, 51, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), s.v[52]);
        }

        if s.b[455] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[455]) {
            s.store_scalar(49, p.p10);
        }

        s.v[311] = (p.p42 * (1.0 + (p.p123 * s.v[12])));

        s.v[261] = (s.v[312] * s.v[312]);

        s.v[262] = (s.v[311] * s.v[311]);

        s.b[457] = (s.v[311] < 0.0);
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if s.b[457] {
            s.store_scalar(310, ((0.5 * s.v[261]) / ((((s.v[262] + s.v[261])) as f64).sqrt() - s.v[311])));
        }

        if (!s.b[457]) {
            s.store_scalar(310, (0.5 * ((((s.v[262] + s.v[261])) as f64).sqrt() + s.v[311])));
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_from_scalar((s.v[254] * (((4.0 - p.p97) - p.p95) + p.p120)), s.ad_value(48))), A::exp(A::div_from_scalar(((-p.p104) * s.v[10]), s.ad_value(48))), p.p8);

        s.v[36] = (p.p11 * (((s.v[254] * (1.0 - p.p97))) as f64).exp());

        s.v[37] = (p.p29 * (((s.v[254] * (1.0 - p.p102))) as f64).exp());

        s.v[42] = ((p.p15 * ((((s.v[254] * ((4.0 - p.p96) + p.p120)) / p.p16)) as f64).exp()) * (((((-p.p110) * s.v[10]) / p.p16)) as f64).exp());

        s.v[43] = ((p.p28 * (((s.v[254] * ((4.0 - p.p102) + p.p120))) as f64).exp()) * ((((-p.p111) * s.v[10])) as f64).exp());

        s.store_powf_scaled_input(255, 70, s.v[72], (-0.5));

        s.store_div_from_scalar(256, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(255), p.p34), s.ad_value(256), (p.p65 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_scaled_input(257, 85, s.v[86], (-0.5));

        s.store_div_from_scalar(258, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(257), p.p36), s.ad_value(258), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.v[255] = (((s.v[254] * p.p95)) as f64).exp();

        s.store_scale(40, 27, (p.p13 * s.v[255]));

        s.store_scale(41, 256, (p.p12 * s.v[255]));

        s.v[93] = ((p.p85 * (((s.v[254] * (p.p97 - 2.0))) as f64).exp()) * ((((-p.p119) * s.v[10])) as f64).exp());

        s.v[94] = (p.p86 * (((s.v[254] * ((p.p95 + p.p97) - 1.0))) as f64).exp());

        s.v[95] = (p.p87 * (((s.v[254] * (p.p98 - 1.0))) as f64).exp());

        s.v[96] = ((p.p88 * (s.v[94] + s.v[95])) / (p.p86 + p.p87));

        s.v[97] = (p.p89 * (((s.v[254] * (p.p99 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.b[459] = (s.v[2] < 525.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if s.b[459] {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!s.b[459]) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p91 * (((s.v[254] * p.p95)) as f64).exp());

        s.store_scaled_voltage(230, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(231, ctx, nodes, Some(5), Some(7), p.p3);

        s.store_scaled_voltage(232, ctx, nodes, Some(5), Some(3), p.p3);

        s.store_scaled_voltage(233, ctx, nodes, Some(4), Some(3), p.p3);

        s.store_scaled_voltage(234, ctx, nodes, Some(4), Some(5), p.p3);

        s.store_scaled_voltage(236, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(240, ctx, nodes, Some(1), Some(4), p.p3);

        s.store_scaled_voltage(243, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(244, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(238, ctx, nodes, Some(9), Some(6), p.p3);

        s.store_scaled_voltage(237, ctx, nodes, Some(8), Some(9), p.p3);

        s.store_add_scaled_inputs4_indices(235, 234, 1.0, 231, 1.0, 236, -1.0, 238, -1.0);

        s.store_add_scaled_inputs4_indices(242, 240, 1.0, 244, (-1.0), 235, 1.0, 237, -1.0);

        s.store_add(241, 244, 242);

        s.b[466] = ((s.v[231] * s.v[8]) < p.p134);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        if s.b[466] {
            s.store_exp_scaled_input(245, 231, s.v[8]);
        }

        if (!s.b[466]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(245, 275, 231, s.v[8], (((-p.p134)) + (1.0)));
        }

        s.b[467] = (((s.v[232] * s.v[8]) / s.v[48]) < p.p134);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_exp_div_scaled_inputs_indices(246, 232, s.v[8], 48, 1.0);
        }

        if (!s.b[467]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(246, 275, A::div_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(48), 1.0), (((-p.p134)) + (1.0)));
        }

        s.b[468] = ((s.v[235] * s.v[8]) < p.p134);
        s.v[468] = if s.b[468] { 1.0 } else { 0.0 };

        if s.b[468] {
            s.store_exp_scaled_input(248, 235, s.v[8]);
        }

        if (!s.b[468]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(248, 275, 235, s.v[8], (((-p.p134)) + (1.0)));
        }

        s.b[469] = ((s.v[234] * s.v[8]) < p.p134);
        s.v[469] = if s.b[469] { 1.0 } else { 0.0 };

        if (!s.b[469]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.b[470] = ((s.v[241] * s.v[8]) < p.p134);
        s.v[470] = if s.b[470] { 1.0 } else { 0.0 };

        if s.b[470] {
            s.store_exp_scaled_input(249, 241, s.v[8]);
        }

        if (!s.b[470]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(249, 275, 241, s.v[8], (((-p.p134)) + (1.0)));
        }

        s.b[471] = (((s.v[241] - s.v[16]) * s.v[8]) < p.p134);
        s.v[471] = if s.b[471] { 1.0 } else { 0.0 };

        if s.b[471] {
            s.store_exp_scaled_input_ad(252, A::sub(s.ad_value(241), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[471]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[471]) {
            s.store_mul_offset_ad_rhs(252, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[472] = (((s.v[235] - s.v[16]) * s.v[8]) < p.p134);
        s.v[472] = if s.b[472] { 1.0 } else { 0.0 };

        if s.b[472] {
            s.store_exp_scaled_input_ad(250, A::sub(s.ad_value(235), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[472]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(250, 275, A::sub_scaled_inputs(s.ad_value(235), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[473] = (((s.v[231] - s.v[16]) * s.v[8]) < p.p134);
        s.v[473] = if s.b[473] { 1.0 } else { 0.0 };

        if s.b[473] {
            s.store_exp_scaled_input_ad(251, A::sub(s.ad_value(231), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[473]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(251, 275, A::sub_scaled_inputs(s.ad_value(231), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[474] = (((s.v[230] - s.v[16]) * s.v[8]) < p.p134);
        s.v[474] = if s.b[474] { 1.0 } else { 0.0 };

        if s.b[474] {
            s.store_exp_scaled_input_ad(253, A::sub(s.ad_value(230), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[474]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(253, 275, A::sub_scaled_inputs(s.ad_value(230), s.v[8], s.ad_value(16), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(104, 251, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(105, 253, 4.0, 1.0);

        s.store_div_scaled_value_offset_denominator(106, s.ad_value(253), 2.0, s.ad_value(105), 1.0, 1.0);

        s.b[475] = (s.v[106] < p.p136);
        s.v[475] = if s.b[475] { 1.0 } else { 0.0 };

        if s.b[475] {
            s.store_scalar(106, p.p136);
        }

        s.store_add_scaled_inputs3_mixed_iia(107, 104, s.v[6], 105, ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(104), 1.0, 1.0, A::offset(s.ad_value(105), 1.0), 1.0)), (-s.v[6]));

        s.store_scaled_add(108, 107, 236, 1.0 / (s.v[31]));

        s.b[476] = (s.v[108] > 0.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        s.b[477] = (s.v[230] < 100.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[477]) {
            s.copy_ad(277, 230);
        }

        if (s.b[476] && (!s.b[477])) {
            s.store_offset_ln_ad(277, A::offset(s.ad_value(230), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[476] {
            s.store_add_scaled_inputs3_mixed_iai(109, 16, 1.0, A::ln(A::scale_offset(s.ad_value(108), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), 277, -1.0);
            s.store_scale(272, 16, 0.2);
            s.store_square(261, 272);
            s.store_square(262, 109);
        }

        s.b[478] = (s.v[109] < 0.0);
        s.v[478] = if s.b[478] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[478]) {
            s.store_div_scaled_inputs_mixed_ia(110, 261, 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(109)), 1.0);
        }

        if (s.b[476] && (!s.b[478])) {
            s.store_scaled_add_ad_lhs(110, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 109, 0.5);
        }

        if s.b[476] {
            s.store_div_scaled_product_offset_rhs(111, s.ad_value(110), s.ad_value(110), (p.p61 * p.p60), 1.0, A::scaled_offset(s.ad_value(110), (p.p61 * s.v[31]), p.p60), 1.0);
            s.store_div(265, 108, 111);
            s.store_scaled_offset(259, 265, (-1.0), 1.0 / (p.p62));
        }

        s.b[479] = (s.v[265] < 1.0);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[479]) {
            s.store_offset_scaled_ad(263, A::ln_one_plus_exp(s.ad_value(259)), p.p62, 1.0);
        }

        if (s.b[476] && (!s.b[479])) {
            s.store_add_scaled_inputs_ad_rhs(263, 265, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p62);
        }

        if s.b[476] {
            s.store_scale(112, 263, 1.0 / ((1.0 + (p.p62 * (((1.0 + ((((-1.0) / p.p62)) as f64).exp())) as f64).ln()))));
            s.store_scale(113, 110, 1.0 / ((p.p61 * p.p60)));
            s.store_div_scaled_offset_numerator(114, A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(112), s.ad_value(113), A::offset(s.ad_value(113), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(112), 2.0, A::offset(s.ad_value(113), 1.0)), 1.0);
            s.store_div_ad(115, A::add_scaled_sub_value_product(1.0, s.ad_value(114), 1.0, s.ad_value(106), s.ad_value(114), 1.0), A::offset(A::mul(s.ad_value(106), s.ad_value(114)), 1.0));
            s.store_scaled_mul(117, 108, 115, ((0.5 * s.v[31]) * s.v[8]));
            s.store_add_scaled_offset_product_rhs_mixed_iia(266, 117, 2.0, 106, A::add(s.ad_value(106), s.ad_value(117)), 1.0, 1.0);
            s.store_scaled_offset(118, 117, (-1.0), 0.5);
            s.store_add_ad_lhs(260, A::square(s.ad_value(118)), 266);
        }

        s.b[480] = (s.v[117] >= 1.0);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[480]) {
            s.store_add_ad_rhs(119, 118, A::sqrt(s.ad_value(260)));
        }

        if (s.b[476] && (!s.b[480])) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(119, 266, A::sqrt(s.ad_value(260)), 1.0, 118, -1.0);
        }

        s.b[481] = (s.v[119] < p.p135);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[481]) {
            s.store_scalar(119, p.p135);
        }

        if s.b[476] {
            s.store_mul_ad_product_rhs(121, 119, A::offset(s.ad_value(119), 1.0), A::exp_scaled_input(s.ad_value(16), s.v[8]));
            s.store_scaled_offset(123, 108, (-p.p61), (0.5 * p.p60));
            s.store_scale(124, 108, ((p.p60 * s.v[31]) * p.p61));
            s.store_add_ad_rhs(125, 123, A::sqrt(A::add(A::square(s.ad_value(123)), s.ad_value(124))));
        }

        s.b[482] = (p.p72 == 0.0);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if (s.b[476] && s.b[482]) {
            s.store_scale(126, 17, 0.1);
        }

        if (s.b[476] && (!s.b[482])) {
            s.store_mul_offset_ad_rhs(126, 17, A::div_scaled_inputs(s.ad_value(108), 2.0, A::add(s.ad_value(108), s.ad_value(111)), 1.0), 0.1);
        }

        if s.b[476] {
            s.store_div_scaled_value_offset_denominator(127, s.ad_value(108), p.p61, s.ad_value(108), p.p61, 1.0);
            s.store_div_from_scalar_offset_input(199, p.p61, 108, p.p61);
        }

        if (!s.b[476]) {
            s.store_scalar(111, 0.0);
            s.store_div_scaled_value_offset_denominator(119, s.ad_value(251), 2.0, s.ad_value(104), 1.0, 1.0);
            s.copy_ad(121, 245);
        }

        s.b[483] = ((((s.v[236]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[107]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[104] + s.v[105]))));
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if ((!s.b[476]) && s.b[483]) {
            s.store_scaled_add(128, 119, 106, 0.5);
            s.store_div_scaled_value_offset_denominator(115, s.ad_value(128), 1.0, s.ad_value(128), 1.0, 1.0);
        }

        if ((!s.b[476]) && (!s.b[483])) {
            s.store_div_ad_rhs(115, 107, A::add_scaled_inputs3(s.ad_value(107), 1.0, s.ad_value(231), 1.0, s.ad_value(230), -1.0));
        }

        if (!s.b[476]) {
            s.copy_ad(125, 236);
            s.store_scale(126, 17, 0.1);
            s.copy_ad(127, 108);
            s.store_sub_from_scalar_scaled_input(199, 1.0, 127, 1.0 / (p.p61));
        }

        s.store_scale(129, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p66))));

        s.store_scale(273, 14, 0.1);

        s.store_div_scaled_inputs2_indices(259, 232, 1.0, 129, (-1.0), 273, 1.0);

        s.b[484] = (s.v[232] < s.v[129]);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_add_scaled_product_right_ad(130, 232, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[484]) {
            s.store_add_scaled_product_right_ad(130, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (1.0 - p.p66));

        s.store_add_scaled_inputs3_mixed_aii(131, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p66))), 1.0, 232, 3.0, 130, (-3.0));

        s.b[485] = (p.p73 == 1.0);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.copy_ad(132, 230);
        }

        s.b[486] = (p.p73 == 2.0);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if ((!s.b[485]) && s.b[486]) {
            s.store_add(132, 230, 125);
        }

        if ((!s.b[485]) && (!s.b[486])) {
            s.copy_ad(132, 231);
        }

        s.store_div_ad(133, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(134, 17, 1.0, A::powf(s.ad_value(133), ((-1.0) / p.p71)));

        s.store_div_scaled_inputs2_indices(259, 132, 1.0, 134, (-1.0), 126, 1.0);

        s.b[487] = (s.v[132] < s.v[134]);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_add_scaled_product_right_ad(135, 132, 1.0, 126, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[487]) {
            s.store_add_scaled_product_right_ad(135, 134, 1.0, 126, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_powf(136, 199, p.p75);

        s.store_add_ad(137, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(136), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(135), s.ad_value(17))), (1.0 - p.p71))), 1.0 / ((1.0 - p.p71))), A::mul3(s.ad_value(136), s.ad_value(133), A::sub(s.ad_value(132), s.ad_value(135))));

        s.store_add_scaled_product_value_ad(138, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(137)), 1.0, 25, 230, 1.0);

        s.store_scale(139, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(140, 139, 246);

        s.store_div_scaled_value_offset_denominator(142, s.ad_value(140), 1.0, A::sqrt(A::offset(s.ad_value(140), 1.0)), 1.0, 1.0);

        s.store_pow_ad(122, s.ad_value(121), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(141, 139, 122);

        s.store_div_scaled_value_offset_denominator(143, s.ad_value(141), 1.0, A::sqrt(A::offset(s.ad_value(141), 1.0)), 1.0, 1.0);

        s.b[488] = (p.p91 == 0.0);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_add_ad(144, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));
        }

        if (!s.b[488]) {
            s.store_offset_scaled_div(269, 131, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_div_scaled_inputs_indices(270, 138, (-(s.v[99] * s.v[8])), 40, 1.0);
            s.store_scaled_sub_ad(144, A::exp(s.ad_value(269)), A::exp(s.ad_value(270)), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 144);

        s.b[489] = (s.v[144] < 0.0);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_div_from_scalar_sub_ad(145, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(144));
        }

        if (!s.b[489]) {
            s.store_scaled_add_ad_lhs(145, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 144, 0.5);
        }

        s.store_mul_offset_ad_rhs(146, 145, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_scaled_mul(147, 35, 122, p.p14);

        s.store_mul(148, 35, 246);

        s.store_div_scaled_inputs2_indices(149, 148, 1.0, 147, (-1.0), 146, 1.0);

        s.store_scale(259, 232, 10000.0);

        s.b[490] = (s.v[232] < 0.0);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.store_scaled_ln_one_plus_exp(276, 259, 0.0001);
        }

        if (!s.b[490]) {
            s.store_add_scaled_inputs_ad_rhs(276, 232, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 0.0001);
        }

        s.store_scale(278, 276, 1.0 / (p.p139));

        s.b[491] = (s.v[278] < p.p134);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if (!s.b[491]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_scaled_offset(259, 232, (-p.p141), 1000.0);

        s.b[493] = (((s.v[232] * s.v[8]) / p.p16) < p.p134);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if s.b[493] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p16)));
        }

        if (!s.b[493]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 232, (s.v[8] * 1.0 / (p.p16)), (((-p.p134)) + (1.0)));
        }

        s.b[494] = (p.p23 == 1.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        s.b[495] = (((s.v[232] - s.v[55]) * s.v[8]) < p.p134);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[495]) {
            s.store_exp_scaled_input_ad(278, A::sub(s.ad_value(232), s.ad_value(55)), s.v[8]);
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 275, A::sub_scaled_inputs(s.ad_value(232), s.v[8], s.ad_value(55), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[496] = (((s.v[149] / s.v[35]) - 1000.0) < 40.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if (s.b[494] && (!s.b[496])) {
            s.store_scalar(275, ((40.0) as f64).exp());
        }

        s.b[498] = (((s.v[233] * s.v[8]) / p.p18) < p.p134);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p18)));
        }

        if (!s.b[498]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 233, (s.v[8] * 1.0 / (p.p18)), (((-p.p134)) + (1.0)));
        }

        s.b[499] = (p.p23 == 1.0);
        s.v[499] = if s.b[499] { 1.0 } else { 0.0 };

        s.b[500] = (((s.v[233] - s.v[55]) * s.v[8]) < p.p134);
        s.v[500] = if s.b[500] { 1.0 } else { 0.0 };

        if (s.b[499] && s.b[500]) {
            s.store_exp_scaled_input_ad(278, A::sub(s.ad_value(233), s.ad_value(55)), s.v[8]);
        }

        if (s.b[499] && (!s.b[500])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(278, 275, A::sub_scaled_inputs(s.ad_value(233), s.v[8], s.ad_value(55), s.v[8]), (((-p.p134)) + (1.0)));
        }

        s.b[501] = (((s.v[232] * s.v[8]) / p.p20) < p.p134);
        s.v[501] = if s.b[501] { 1.0 } else { 0.0 };

        if s.b[501] {
            s.store_exp_scaled_input(276, 232, (s.v[8] * 1.0 / (p.p20)));
        }

        if (!s.b[501]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 232, (s.v[8] * 1.0 / (p.p20)), (((-p.p134)) + (1.0)));
        }

        s.b[502] = (((s.v[233] * s.v[8]) / p.p22) < p.p134);
        s.v[502] = if s.b[502] { 1.0 } else { 0.0 };

        if s.b[502] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p22)));
        }

        if (!s.b[502]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 233, (s.v[8] * 1.0 / (p.p22)), (((-p.p134)) + (1.0)));
        }

        s.b[503] = (((s.v[235] * s.v[8]) / p.p31) < p.p134);
        s.v[503] = if s.b[503] { 1.0 } else { 0.0 };

        if s.b[503] {
            s.store_exp_scaled_input(276, 235, (s.v[8] * 1.0 / (p.p31)));
        }

        if (!s.b[503]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[503]) {
            s.store_mul_scale_offset_rhs(276, 275, 235, (s.v[8] * 1.0 / (p.p31)), (((-p.p134)) + (1.0)));
        }

        s.b[504] = (((s.v[233] * s.v[8]) / p.p133) < p.p134);
        s.v[504] = if s.b[504] { 1.0 } else { 0.0 };

        if s.b[504] {
            s.store_exp_scaled_input(276, 233, (s.v[8] * 1.0 / (p.p133)));
        }

        if (!s.b[504]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 233, (s.v[8] * 1.0 / (p.p133)), (((-p.p134)) + (1.0)));
        }

        s.b[505] = (((p.p33 > 0.0) && (p.p34 > 0.0)) && (s.v[232] < 0.0));
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        s.b[506] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p134);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if (s.b[505] && (!s.b[506])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if s.b[505] {
            s.store_mul(255, 232, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt_square_offset(s.ad_value(255), 1e-30), ((-2.0) - p.p66)), A::sub(A::scale_offset(A::scale(s.ad_value(255), (3.0 * (p.p66 - 1.0))), (-p.p66), (((1.0 - (p.p66 * p.p66))) * (p.p66))), A::mul3_scaled_output(s.ad_value(255), s.ad_value(255), A::offset(s.ad_value(255), (p.p66 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(255, s.ad_value(232), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0);
        }

        s.b[507] = (s.v[255] < (-0.001));
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        s.b[508] = (s.v[255] < p.p134);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if ((s.b[505] && s.b[507]) && (!s.b[508])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.b[509] = (((p.p35 > 0.0) && (p.p36 > 0.0)) && (s.v[230] < 0.0));
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(230), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[510] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p134);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (s.b[509] && (!s.b[510])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        if s.b[509] {
            s.store_mul(257, 230, 67);
            s.store_scaled_mul_ad(80, A::powf(A::sqrt_square_offset(s.ad_value(257), 1e-30), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(257), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(257), s.ad_value(257), A::offset(s.ad_value(257), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(257, s.ad_value(230), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0);
        }

        s.b[511] = (s.v[257] < (-0.001));
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        s.b[512] = (s.v[257] < p.p134);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if ((s.b[509] && s.b[511]) && (!s.b[512])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
        }

        s.store_mul(158, 139, 248);

        s.store_scale(159, 250, 4.0);

        s.store_div_scaled_inputs2_mixed_iia(161, 158, 1.0, 139, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(158), 1.0)), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(160, s.ad_value(159), 1.0, A::sqrt(A::offset(s.ad_value(159), 1.0)), 1.0, 1.0);

        s.b[513] = ((p.p5 > 0.0) && (p.p32 > 0.0));
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_div_scaled_offset_numerator(164, s.ad_value(249), ((p.p32 * 2.0) * s.v[43]), ((-1.0) * ((p.p32 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(249), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);
            s.store_scalar(165, 0.0);
        }

        s.b[514] = (p.p5 == 1.0);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if (s.b[513] && s.b[514]) {
            s.store_scalar(271, ((p.p32 * s.v[43]) * s.v[32]));
            s.store_offset_scaled_ad(166, A::ln_scaled_input(s.ad_value(271), s.v[8]), (-s.v[6]), ((2.0) * (s.v[6])));
            s.store_sub(264, 241, 166);
            s.store_scalar(261, (0.11 * 0.11));
            s.store_square(262, 264);
        }

        s.b[515] = (s.v[264] < 0.0);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if ((s.b[513] && s.b[514]) && s.b[515]) {
            s.store_div_scaled_inputs_mixed_ia(167, 261, 0.5, A::sub(A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), s.ad_value(264)), 1.0);
        }

        if ((s.b[513] && s.b[514]) && (!s.b[515])) {
            s.store_scaled_add_ad_lhs(167, A::sqrt(A::add(s.ad_value(262), s.ad_value(261))), 264, 0.5);
        }

        if (s.b[513] && s.b[514]) {
            s.store_div_ad_rhs(168, 167, A::add_scaled_inputs4(s.ad_value(271), 1.0, s.ad_value(164), s.v[32], s.ad_value(165), s.v[32], s.ad_value(167), 1.0));
        }

        if (s.b[513] && (!s.b[514])) {
            s.store_scalar(166, 0.0);
            s.store_scalar(264, 0.0);
            s.store_scalar(167, 0.0);
            s.store_scalar(168, 1.0);
        }

        s.b[516] = (p.p83 == 1.0);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_add(322, 234, 230);
            s.store_scalar(261, (1e-6 * 1e-6));
            s.store_scaled_mul(262, 322, 322, ((-1.0) * (-1.0)));
        }

        s.store_add_ad(172, A::offset(A::div(s.ad_value(131), s.ad_value(41)), 1.0), A::div(s.ad_value(138), s.ad_value(40)));

        s.v[261] = (0.1 * 0.1);

        s.store_square(262, 172);

        s.b[519] = (s.v[172] < 0.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if s.b[519] {
            s.store_div_from_scalar_sub_ad(173, (0.5 * s.v[261]), A::sqrt(A::offset(s.ad_value(262), s.v[261])), s.ad_value(172));
        }

        if (!s.b[519]) {
            s.store_scaled_add_ad_lhs(173, A::sqrt(A::offset(s.ad_value(262), s.v[261])), 172, 0.5);
        }

        s.store_mul_offset_ad_rhs(174, 173, A::add_scaled_inputs(s.ad_value(142), 0.5, s.ad_value(143), 0.5), 1.0);

        s.store_div_from_scalar(176, s.v[29], 174);

        s.b[520] = (s.v[176] < s.v[316]);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if s.b[520] {
            s.copy_ad(176, 316);
        }

        s.store_scale(175, 176, 3.0);

        s.b[521] = (s.v[149] > 0.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        s.b[522] = (p.p38 == 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        s.b[523] = (s.v[230] < p.p43);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        s.b[524] = (((-s.v[149]) / p.p41) < p.p134);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[524]) {
            s.store_exp_scaled_input(308, 149, (-1.0 / (p.p41)));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[524])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(308, 275, 149, (-1.0 / (p.p41)), (((-p.p134)) + (1.0)));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_sub_from_scalar_lhs(309, p.p43, 230, 308);
        }

        s.b[525] = (((-s.v[310]) * ((s.v[309]) as f64).powf(p.p40)) < p.p134);
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[522]) && s.b[523]) && s.b[525]) {
            s.store_exp_mul_scaled_lhs_mixed_ia(313, 310, -1.0, A::powf(s.ad_value(309), p.p40));
        }

        if (((s.b[521] && s.b[522]) && s.b[523]) && (!s.b[525])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, A::powf(s.ad_value(309), p.p40)), (((-p.p134)) + (1.0)));
        }

        if ((s.b[521] && s.b[522]) && s.b[523]) {
            s.store_mul_ad_product_lhs(196, A::div_from_scalar(p.p39, s.ad_value(310)), s.ad_value(309), 313);
        }

        s.b[526] = (p.p38 == 2.0);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        s.b[527] = (s.v[230] < s.v[16]);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_scalar(185, ((2.0 * p.p45) / (p.p44 * p.p44)));
            s.store_div_scaled_inputs2_indices(260, 16, 1.0, 230, (-1.0), 199, 1.0);
            s.store_sqrt_ad(186, A::div_scaled_inputs(s.ad_value(260), 2.0, s.ad_value(185), 1.0));
        }

        s.b[528] = (p.p7 == 0.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[528]) {
            s.store_scalar(187, p.p44);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[528])) {
            s.store_sub_from_scalar_scaled_input(116, 1.0, 115, 0.5);
            s.store_scaled_mul(187, 116, 116, p.p44);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_product_sqrt_square_sum_denominator(188, 186, 187, 1.0, 186, 187, 1.0);
            s.store_div_scaled_inputs2_indices(189, 16, 1.0, 230, (-1.0), 188, 1.0);
            s.store_add_ad_rhs(190, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), s.ad_value(199), 0.5));
        }

        s.b[529] = (p.p7 == 0.0);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[529]) {
            s.copy_ad(191, 190);
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[529])) {
            s.store_offset_scaled(192, 115, ((2.0) * ((2.0 * p.p46))), (((2.0 * p.p46)) + (1.0)));
            s.store_scalar(193, ((1.0 + p.p46) / (1.0 + (2.0 * p.p46))));
            s.store_sub_ad_rhs(194, 189, A::mul3_scaled_output(s.ad_value(188), s.ad_value(185), A::sub(s.ad_value(193), A::div_scaled_inputs(s.ad_value(149), 1.0, s.ad_value(192), p.p61)), 0.5));
            s.store_add_scaled_product(260, A::mul3_scaled_output(s.ad_value(189), s.ad_value(189), s.ad_value(127), (0.1 * 1.0 / (p.p61))), 1.0, A::sub(s.ad_value(194), s.ad_value(190)), A::sub(s.ad_value(194), s.ad_value(190)), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_indices(191, 194, 0.5, 190, 0.5, 260, 0.5);
        }

        if (((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) {
            s.store_div_scaled_inputs2_indices(267, 191, 1.0, 189, (-1.0), 191, 1.0);
        }

        s.b[530] = (((s.v[267]) as f64).abs() > 1e-7);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && s.b[530]) {
            s.store_div_scaled_inputs_indices(195, 188, 0.5, 267, 1.0);
            s.store_mul_ad(196, A::mul3(A::div(s.ad_value(0), s.ad_value(98)), s.ad_value(191), s.ad_value(195)), A::sub(A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0), A::div(s.ad_value(187), s.ad_value(195)), 1.0))));
        }

        if ((((s.b[521] && (!s.b[522])) && s.b[526]) && s.b[527]) && (!s.b[530])) {
            s.store_mul_ad_product_rhs(196, 0, s.ad_value(187), A::exp(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(191), 1.0)));
        }

        s.b[531] = (p.p38 == 3.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        s.b[532] = (s.v[230] < p.p43);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad(200, A::powf(A::sub_from_scalar(p.p43, s.ad_value(230)), p.p40), A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(149), 1.0, s.ad_value(149), p.p47, 1.0)), p.p48));
        }

        s.b[533] = (p.p7 == 0.0);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[533]) {
            s.copy_ad(201, 200);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_scaled_offset(202, 149, (-p.p51), 1.0 / (p.p47));
            s.store_scaled_offset(259, 202, (-1.0), 1.0 / (p.p50));
        }

        s.b[534] = (s.v[202] < 1.0);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && s.b[534]) {
            s.store_offset_scaled_ad(203, A::ln_one_plus_exp(s.ad_value(259)), p.p50, 1.0);
        }

        if ((((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) && (!s.b[534])) {
            s.store_add_scaled_inputs_ad_rhs(203, 202, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), p.p50);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[533])) {
            s.store_mul_powf_ad_rhs(201, 200, s.ad_value(203), p.p49);
        }

        s.b[535] = (((-s.v[310]) * s.v[201]) < p.p134);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && s.b[535]) {
            s.store_exp_mul_scaled_lhs_indices(313, 310, -1.0, 201);
        }

        if (((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) && (!s.b[535])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(313, 275, A::mul_scaled_lhs(s.ad_value(310), -1.0, s.ad_value(201)), (((-p.p134)) + (1.0)));
        }

        if ((((s.b[521] && (!s.b[522])) && (!s.b[526])) && s.b[531]) && s.b[532]) {
            s.store_mul_ad_lhs(196, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p39, s.ad_value(310)), p.p43, s.ad_value(230)), 313);
        }

        s.b[536] = (s.v[196] > 0.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (p.p52 == 1.0);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if ((s.b[521] && s.b[536]) && s.b[537]) {
            s.store_add_scaled_inputs3(197, A::div_from_scalar(s.v[6], A::mul(s.ad_value(149), A::add(s.ad_value(30), s.ad_value(175)))), 1.0, A::div(s.ad_value(146), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(175))), 1.0);
        }

        s.b[538] = (p.p38 == 3.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if (((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) {
            s.store_scaled_sub(259, 196, 197, 1000000.0);
        }

        s.b[539] = (s.v[196] < s.v[197]);
        s.v[539] = if s.b[539] { 1.0 } else { 0.0 };

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && s.b[539]) {
            s.store_sub_scaled_inputs_ad_rhs(196, 196, 1.0, A::ln_one_plus_exp(s.ad_value(259)), 1e-6);
        }

        if ((((s.b[521] && s.b[536]) && s.b[537]) && s.b[538]) && (!s.b[539])) {
            s.store_sub_scaled_inputs_ad_rhs(196, 197, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(259))), 1e-6);
        }

        s.store_scaled_mul(204, 23, 131, (1.0 - p.p67));

        s.store_div_scaled_inputs2_indices(259, 233, 1.0, 129, (-1.0), 273, 1.0);

        s.b[541] = (s.v[233] < s.v[129]);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        if s.b[541] {
            s.store_add_scaled_product_right_ad(205, 233, 1.0, 273, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[541]) {
            s.store_add_scaled_product_right_ad(205, 129, 1.0, 273, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(206, 23, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(205), s.ad_value(65))), (1.0 - p.p66)), 1.0 / ((1.0 - p.p66))), p.p67, s.ad_value(233), ((3.0) * (p.p67)), s.ad_value(205), (((-3.0)) * (p.p67)), 0.0);

        s.store_scaled_mul(207, 24, 138, p.p76);

        s.v[208] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(212, 142, 173, (0.5 * s.v[208]));

        s.store_scaled_mul(213, 143, 173, (0.5 * s.v[208]));

        s.store_scale(274, 17, 0.1);

        s.store_div_scaled_inputs2_indices(259, 235, 1.0, 134, (-1.0), 274, 1.0);

        s.b[542] = (s.v[235] < s.v[134]);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if s.b[542] {
            s.store_add_scaled_product_right_ad(214, 235, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[542]) {
            s.store_add_scaled_product_right_ad(214, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(215, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(214), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(235), s.ad_value(214)), 1.0);

        s.store_mul_add_scaled_product_rhs(216, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(215)), ((1.0 - p.p76) * (1.0 - p.p32)), s.ad_value(25), s.ad_value(235), ((1.0 - p.p76) * (1.0 - p.p32)));

        s.store_div_scaled_inputs2_indices(259, 241, 1.0, 134, (-1.0), 274, 1.0);

        s.b[543] = (s.v[241] < s.v[134]);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_add_scaled_product_right_ad(217, 241, 1.0, 274, A::ln_one_plus_exp(s.ad_value(259)), (-1.0));
        }

        if (!s.b[543]) {
            s.store_add_scaled_product_right_ad(217, 134, 1.0, 274, A::ln_one_plus_exp(A::neg(s.ad_value(259))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(218, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(217), s.ad_value(17))), (1.0 - p.p71)), 1.0 / ((1.0 - p.p71))), 1.0, 133, A::sub(s.ad_value(241), s.ad_value(217)), 1.0);

        s.store_mul_add_scaled_product_rhs(219, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(218)), ((1.0 - p.p76) * p.p32), s.ad_value(25), s.ad_value(241), ((1.0 - p.p76) * p.p32));

        s.store_scaled_powf_ad(220, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p84), (s.v[93] * s.v[36]));

        s.b[544] = ((s.v[232] / (p.p84 * s.v[6])) < p.p134);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(276, 232, 1.0 / ((p.p84 * s.v[6])));
        }

        if (!s.b[544]) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_scale_offset_rhs(276, 275, 232, 1.0 / ((p.p84 * s.v[6])), (((-p.p134)) + (1.0)));
        }

        s.store_mul(222, 220, 276);

        s.v[223] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scaled_offset_ad_rhs(224, 115, (0.5 * s.v[223]), A::add(s.ad_value(119), s.ad_value(106)), 2.0);

        s.b[545] = (p.p78 == 0.0);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_add_scaled_inputs(229, 161, (s.v[208] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 160, (s.v[223] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[546] = ((((s.v[235] - s.v[22]) / p.p90) * s.v[8]) < p.p134);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if ((!s.b[545]) && s.b[546]) {
            s.store_exp_scaled_input_ad(170, A::sub(s.ad_value(235), s.ad_value(22)), (1.0 / (p.p90) * s.v[8]));
        }

        if ((!s.b[545]) && (!s.b[546])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(170, 275, A::sub_scaled_inputs(s.ad_value(235), (1.0 / (p.p90) * s.v[8]), s.ad_value(22), (1.0 / (p.p90) * s.v[8])), (((-p.p134)) + (1.0)));
        }

        if (!s.b[545]) {
            s.store_div_scaled_value_offset_denominator(229, s.ad_value(248), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(170), 4.0, 1.0)), 1.0, 1.0);
        }

        s.b[547] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0));
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scale(229, 229, s.v[150]);
        }

        s.b[548] = (p.p78 == 0.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[547] && s.b[548]) {
            s.store_mul(162, 139, 249);
            s.store_div_scaled_inputs2_mixed_iia(163, 162, 1.0, 139, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(162), 1.0)), 1.0), 1.0);
            s.store_scale(225, 252, 4.0);
            s.store_div_scaled_value_offset_denominator(226, s.ad_value(225), 1.0, A::sqrt(A::offset(s.ad_value(225), 1.0)), 1.0, 1.0);
            s.store_add_scaled_inputs(227, 163, (s.v[208] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 226, (s.v[223] * (((0.5 * p.p32) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[549] = (((s.v[241] - s.v[22]) * s.v[8]) < p.p134);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if ((s.b[547] && (!s.b[548])) && s.b[549]) {
            s.store_exp_scaled_input_ad(171, A::sub(s.ad_value(241), s.ad_value(22)), s.v[8]);
        }

        if ((s.b[547] && (!s.b[548])) && (!s.b[549])) {
            s.store_scalar(275, ((p.p134) as f64).exp());
            s.store_mul_offset_ad_rhs(171, 275, A::sub_scaled_inputs(s.ad_value(241), s.v[8], s.ad_value(22), s.v[8]), (((-p.p134)) + (1.0)));
        }

        if (s.b[547] && (!s.b[548])) {
            s.store_div_scaled_value_offset_denominator(227, s.ad_value(249), (((2.0 * p.p32) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(171), 4.0, 1.0)), 1.0, 1.0);
        }

        if s.b[547] {
            s.store_mul(228, 168, 227);
        }

        s.b[550] = (p.p6 == 1.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if s.b[550] {
            s.store_offset_powf_ad(179, A::sub_from_scalar(1.0, A::mul(s.ad_value(130), s.ad_value(65))), (-p.p66), (-3.0));
            s.store_div_scaled_inputs2_indices(268, 232, 1.0, 129, (-1.0), 273, 1.0);
        }

        s.b[551] = (s.v[268] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (s.b[550] && s.b[551]) {
            s.store_div_from_scalar_offset_ad(180, 1.0, A::exp(s.ad_value(268)), 1.0);
        }

        if (s.b[550] && (!s.b[551])) {
            s.store_div_ad(180, A::exp_scaled_input(s.ad_value(268), -1.0), A::offset(A::exp_scaled_input(s.ad_value(268), -1.0), 1.0));
        }

        if s.b[550] {
            s.store_offset_mul(178, 179, 180, 3.0);
            s.store_scaled_mul(181, 23, 178, (1.0 - p.p67));
            s.store_mul_ad(184, A::div_scaled_product(s.ad_value(139), s.ad_value(246), s.v[8], s.ad_value(48), 1.0), A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(140), 1.0))));
            s.store_scaled_mul(182, 173, 184, (0.5 * s.v[208]));
            s.store_scale(183, 222, 1.0 / ((p.p84 * s.v[6])));
            s.store_mul_add_scaled_inputs3_offset_rhs(211, 234, s.ad_value(181), 0.2, s.ad_value(182), 0.2, s.ad_value(183), 0.2, 0.0);
            s.store_scale(221, 222, (1.0 - p.p94));
            s.store_add_scaled_inputs(307, 212, 1.0, 222, p.p94);
            s.store_add_scaled_inputs(210, 307, p.p93, 213, 1.0);
            s.store_scale(209, 307, (1.0 - p.p93));
        }

        if (!s.b[550]) {
            s.copy_ad(209, 212);
            s.copy_ad(210, 213);
            s.copy_ad(221, 222);
        }

        s.store_div_scaled_inputs2_indices(303, 148, 1.0, 147, 1.0, 146, 1.0);

        s.b[556] = (s.v[303] > 0.0);
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_div_scaled_inputs2_indices(305, 209, 1.0, 210, 1.0, 303, 1.0);
        }

        if (!s.b[556]) {
            s.store_scaled_mul(305, 173, 146, s.v[94]);
        }

        s.b[557] = (p.p130 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if s.b[557] {
            s.store_scale(306, 305, p.p93);
        }

        s.b[558] = (p.p130 == 2.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if ((!s.b[557]) && s.b[558]) {
            s.store_scale(306, 305, p.p131);
        }

        if ((!s.b[557]) && (!s.b[558])) {
            s.store_scalar(306, 0.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
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
        let eq10_e233: f64 = (s.v[204] + s.v[209]);
        let eq10_e233_d_n0: f64 = (s.dn[204][0] + s.dn[209][0]);
        let eq10_e233_d_n1: f64 = (s.dn[204][1] + s.dn[209][1]);
        let eq10_e233_d_n2: f64 = (s.dn[204][2] + s.dn[209][2]);
        let eq10_e233_d_n3: f64 = (s.dn[204][3] + s.dn[209][3]);
        let eq10_e233_d_n4: f64 = (s.dn[204][4] + s.dn[209][4]);
        let eq10_e233_d_n5: f64 = (s.dn[204][5] + s.dn[209][5]);
        let eq10_e233_d_n6: f64 = (s.dn[204][6] + s.dn[209][6]);
        let eq10_e233_d_n7: f64 = (s.dn[204][7] + s.dn[209][7]);
        let eq10_e233_d_n8: f64 = (s.dn[204][8] + s.dn[209][8]);
        let eq10_e233_d_n9: f64 = (s.dn[204][9] + s.dn[209][9]);
        let eq10_e233_d_n10: f64 = (s.dn[204][10] + s.dn[209][10]);
        let eq10_e233_d_b0: f64 = (s.db[204][0] + s.db[209][0]);
        let eq10_e233_d_b1: f64 = (s.db[204][1] + s.db[209][1]);
        let eq10_e235: f64 = (eq10_e233 + s.v[221]);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + s.dn[221][0]);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + s.dn[221][1]);
        let eq10_e235_d_n2: f64 = (eq10_e233_d_n2 + s.dn[221][2]);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + s.dn[221][3]);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + s.dn[221][4]);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + s.dn[221][5]);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + s.dn[221][6]);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + s.dn[221][7]);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + s.dn[221][8]);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + s.dn[221][9]);
        let eq10_e235_d_n10: f64 = (eq10_e233_d_n10 + s.dn[221][10]);
        let eq10_e235_d_b0: f64 = (eq10_e233_d_b0 + s.db[221][0]);
        let eq10_e235_d_b1: f64 = (eq10_e233_d_b1 + s.db[221][1]);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n2: f64 = (p.p3 * eq10_e235_d_n2);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e236_d_n10: f64 = (p.p3 * eq10_e235_d_n10);
        let eq10_e236_d_b0: f64 = (p.p3 * eq10_e235_d_b0);
        let eq10_e236_d_b1: f64 = (p.p3 * eq10_e235_d_b1);
        let eq10_e237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq10_e236);
        let eq10_e237_d_n0: f64 = (eq10_e236_d_n0 * ddt_scale);
        let eq10_e237_d_n1: f64 = (eq10_e236_d_n1 * ddt_scale);
        let eq10_e237_d_n2: f64 = (eq10_e236_d_n2 * ddt_scale);
        let eq10_e237_d_n3: f64 = (eq10_e236_d_n3 * ddt_scale);
        let eq10_e237_d_n4: f64 = (eq10_e236_d_n4 * ddt_scale);
        let eq10_e237_d_n5: f64 = (eq10_e236_d_n5 * ddt_scale);
        let eq10_e237_d_n6: f64 = (eq10_e236_d_n6 * ddt_scale);
        let eq10_e237_d_n7: f64 = (eq10_e236_d_n7 * ddt_scale);
        let eq10_e237_d_n8: f64 = (eq10_e236_d_n8 * ddt_scale);
        let eq10_e237_d_n9: f64 = (eq10_e236_d_n9 * ddt_scale);
        let eq10_e237_d_n10: f64 = (eq10_e236_d_n10 * ddt_scale);
        let eq10_e237_d_b0: f64 = (eq10_e236_d_b0 * ddt_scale);
        let eq10_e237_d_b1: f64 = (eq10_e236_d_b1 * ddt_scale);
        let eq10_e239: f64 = (eq10_e237 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e237_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e237_d_n1 * p.p1);
        let eq10_e239_d_n2: f64 = (eq10_e237_d_n2 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e237_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e237_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e237_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e237_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e237_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e237_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e237_d_n9 * p.p1);
        let eq10_e239_d_n10: f64 = (eq10_e237_d_n10 * p.p1);
        let eq10_e239_d_b0: f64 = (eq10_e237_d_b0 * p.p1);
        let eq10_e239_d_b1: f64 = (eq10_e237_d_b1 * p.p1);
        let eq10_value: f64 = eq10_e239;
        let eq10_node_derivatives: [f64; 11] = [eq10_e239_d_n0, eq10_e239_d_n1, eq10_e239_d_n2, eq10_e239_d_n3, eq10_e239_d_n4, eq10_e239_d_n5, eq10_e239_d_n6, eq10_e239_d_n7, eq10_e239_d_n8, eq10_e239_d_n9, eq10_e239_d_n10];
        let eq10_branch_derivatives: [f64; 2] = [eq10_e239_d_b0, eq10_e239_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e242: f64 = (p.p3 * s.v[206]);
        let eq11_e242_d_n0: f64 = (p.p3 * s.dn[206][0]);
        let eq11_e242_d_n1: f64 = (p.p3 * s.dn[206][1]);
        let eq11_e242_d_n2: f64 = (p.p3 * s.dn[206][2]);
        let eq11_e242_d_n3: f64 = (p.p3 * s.dn[206][3]);
        let eq11_e242_d_n4: f64 = (p.p3 * s.dn[206][4]);
        let eq11_e242_d_n5: f64 = (p.p3 * s.dn[206][5]);
        let eq11_e242_d_n6: f64 = (p.p3 * s.dn[206][6]);
        let eq11_e242_d_n7: f64 = (p.p3 * s.dn[206][7]);
        let eq11_e242_d_n8: f64 = (p.p3 * s.dn[206][8]);
        let eq11_e242_d_n9: f64 = (p.p3 * s.dn[206][9]);
        let eq11_e242_d_n10: f64 = (p.p3 * s.dn[206][10]);
        let eq11_e242_d_b0: f64 = (p.p3 * s.db[206][0]);
        let eq11_e242_d_b1: f64 = (p.p3 * s.db[206][1]);
        let eq11_e243: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq11_e242);
        let eq11_e243_d_n0: f64 = (eq11_e242_d_n0 * ddt_scale);
        let eq11_e243_d_n1: f64 = (eq11_e242_d_n1 * ddt_scale);
        let eq11_e243_d_n2: f64 = (eq11_e242_d_n2 * ddt_scale);
        let eq11_e243_d_n3: f64 = (eq11_e242_d_n3 * ddt_scale);
        let eq11_e243_d_n4: f64 = (eq11_e242_d_n4 * ddt_scale);
        let eq11_e243_d_n5: f64 = (eq11_e242_d_n5 * ddt_scale);
        let eq11_e243_d_n6: f64 = (eq11_e242_d_n6 * ddt_scale);
        let eq11_e243_d_n7: f64 = (eq11_e242_d_n7 * ddt_scale);
        let eq11_e243_d_n8: f64 = (eq11_e242_d_n8 * ddt_scale);
        let eq11_e243_d_n9: f64 = (eq11_e242_d_n9 * ddt_scale);
        let eq11_e243_d_n10: f64 = (eq11_e242_d_n10 * ddt_scale);
        let eq11_e243_d_b0: f64 = (eq11_e242_d_b0 * ddt_scale);
        let eq11_e243_d_b1: f64 = (eq11_e242_d_b1 * ddt_scale);
        let eq11_e245: f64 = (eq11_e243 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e243_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e243_d_n1 * p.p1);
        let eq11_e245_d_n2: f64 = (eq11_e243_d_n2 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e243_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e243_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e243_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e243_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e243_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e243_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e243_d_n9 * p.p1);
        let eq11_e245_d_n10: f64 = (eq11_e243_d_n10 * p.p1);
        let eq11_e245_d_b0: f64 = (eq11_e243_d_b0 * p.p1);
        let eq11_e245_d_b1: f64 = (eq11_e243_d_b1 * p.p1);
        let eq11_value: f64 = eq11_e245;
        let eq11_node_derivatives: [f64; 11] = [eq11_e245_d_n0, eq11_e245_d_n1, eq11_e245_d_n2, eq11_e245_d_n3, eq11_e245_d_n4, eq11_e245_d_n5, eq11_e245_d_n6, eq11_e245_d_n7, eq11_e245_d_n8, eq11_e245_d_n9, eq11_e245_d_n10];
        let eq11_branch_derivatives: [f64; 2] = [eq11_e245_d_b0, eq11_e245_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(3),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e249: f64 = (s.v[207] + s.v[210]);
        let eq12_e249_d_n0: f64 = (s.dn[207][0] + s.dn[210][0]);
        let eq12_e249_d_n1: f64 = (s.dn[207][1] + s.dn[210][1]);
        let eq12_e249_d_n2: f64 = (s.dn[207][2] + s.dn[210][2]);
        let eq12_e249_d_n3: f64 = (s.dn[207][3] + s.dn[210][3]);
        let eq12_e249_d_n4: f64 = (s.dn[207][4] + s.dn[210][4]);
        let eq12_e249_d_n5: f64 = (s.dn[207][5] + s.dn[210][5]);
        let eq12_e249_d_n6: f64 = (s.dn[207][6] + s.dn[210][6]);
        let eq12_e249_d_n7: f64 = (s.dn[207][7] + s.dn[210][7]);
        let eq12_e249_d_n8: f64 = (s.dn[207][8] + s.dn[210][8]);
        let eq12_e249_d_n9: f64 = (s.dn[207][9] + s.dn[210][9]);
        let eq12_e249_d_n10: f64 = (s.dn[207][10] + s.dn[210][10]);
        let eq12_e249_d_b0: f64 = (s.db[207][0] + s.db[210][0]);
        let eq12_e249_d_b1: f64 = (s.db[207][1] + s.db[210][1]);
        let eq12_e251: f64 = (eq12_e249 + s.v[224]);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + s.dn[224][0]);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + s.dn[224][1]);
        let eq12_e251_d_n2: f64 = (eq12_e249_d_n2 + s.dn[224][2]);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + s.dn[224][3]);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + s.dn[224][4]);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + s.dn[224][5]);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + s.dn[224][6]);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + s.dn[224][7]);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + s.dn[224][8]);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + s.dn[224][9]);
        let eq12_e251_d_n10: f64 = (eq12_e249_d_n10 + s.dn[224][10]);
        let eq12_e251_d_b0: f64 = (eq12_e249_d_b0 + s.db[224][0]);
        let eq12_e251_d_b1: f64 = (eq12_e249_d_b1 + s.db[224][1]);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n2: f64 = (p.p3 * eq12_e251_d_n2);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e252_d_n10: f64 = (p.p3 * eq12_e251_d_n10);
        let eq12_e252_d_b0: f64 = (p.p3 * eq12_e251_d_b0);
        let eq12_e252_d_b1: f64 = (p.p3 * eq12_e251_d_b1);
        let eq12_e253: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq12_e252);
        let eq12_e253_d_n0: f64 = (eq12_e252_d_n0 * ddt_scale);
        let eq12_e253_d_n1: f64 = (eq12_e252_d_n1 * ddt_scale);
        let eq12_e253_d_n2: f64 = (eq12_e252_d_n2 * ddt_scale);
        let eq12_e253_d_n3: f64 = (eq12_e252_d_n3 * ddt_scale);
        let eq12_e253_d_n4: f64 = (eq12_e252_d_n4 * ddt_scale);
        let eq12_e253_d_n5: f64 = (eq12_e252_d_n5 * ddt_scale);
        let eq12_e253_d_n6: f64 = (eq12_e252_d_n6 * ddt_scale);
        let eq12_e253_d_n7: f64 = (eq12_e252_d_n7 * ddt_scale);
        let eq12_e253_d_n8: f64 = (eq12_e252_d_n8 * ddt_scale);
        let eq12_e253_d_n9: f64 = (eq12_e252_d_n9 * ddt_scale);
        let eq12_e253_d_n10: f64 = (eq12_e252_d_n10 * ddt_scale);
        let eq12_e253_d_b0: f64 = (eq12_e252_d_b0 * ddt_scale);
        let eq12_e253_d_b1: f64 = (eq12_e252_d_b1 * ddt_scale);
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
        let eq12_e255_d_b0: f64 = (eq12_e253_d_b0 * p.p1);
        let eq12_e255_d_b1: f64 = (eq12_e253_d_b1 * p.p1);
        let eq12_value: f64 = eq12_e255;
        let eq12_node_derivatives: [f64; 11] = [eq12_e255_d_n0, eq12_e255_d_n1, eq12_e255_d_n2, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, eq12_e255_d_n10];
        let eq12_branch_derivatives: [f64; 2] = [eq12_e255_d_b0, eq12_e255_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e258: f64 = (p.p3 * s.v[211]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[211][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[211][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[211][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[211][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[211][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[211][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[211][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[211][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[211][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[211][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[211][10]);
        let eq13_e258_d_b0: f64 = (p.p3 * s.db[211][0]);
        let eq13_e258_d_b1: f64 = (p.p3 * s.db[211][1]);
        let eq13_e259: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq13_e258);
        let eq13_e259_d_n0: f64 = (eq13_e258_d_n0 * ddt_scale);
        let eq13_e259_d_n1: f64 = (eq13_e258_d_n1 * ddt_scale);
        let eq13_e259_d_n2: f64 = (eq13_e258_d_n2 * ddt_scale);
        let eq13_e259_d_n3: f64 = (eq13_e258_d_n3 * ddt_scale);
        let eq13_e259_d_n4: f64 = (eq13_e258_d_n4 * ddt_scale);
        let eq13_e259_d_n5: f64 = (eq13_e258_d_n5 * ddt_scale);
        let eq13_e259_d_n6: f64 = (eq13_e258_d_n6 * ddt_scale);
        let eq13_e259_d_n7: f64 = (eq13_e258_d_n7 * ddt_scale);
        let eq13_e259_d_n8: f64 = (eq13_e258_d_n8 * ddt_scale);
        let eq13_e259_d_n9: f64 = (eq13_e258_d_n9 * ddt_scale);
        let eq13_e259_d_n10: f64 = (eq13_e258_d_n10 * ddt_scale);
        let eq13_e259_d_b0: f64 = (eq13_e258_d_b0 * ddt_scale);
        let eq13_e259_d_b1: f64 = (eq13_e258_d_b1 * ddt_scale);
        let eq13_e261: f64 = (eq13_e259 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e259_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e259_d_n1 * p.p1);
        let eq13_e261_d_n2: f64 = (eq13_e259_d_n2 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e259_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e259_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e259_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e259_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e259_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e259_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e259_d_n9 * p.p1);
        let eq13_e261_d_n10: f64 = (eq13_e259_d_n10 * p.p1);
        let eq13_e261_d_b0: f64 = (eq13_e259_d_b0 * p.p1);
        let eq13_e261_d_b1: f64 = (eq13_e259_d_b1 * p.p1);
        let eq13_value: f64 = eq13_e261;
        let eq13_node_derivatives: [f64; 11] = [eq13_e261_d_n0, eq13_e261_d_n1, eq13_e261_d_n2, eq13_e261_d_n3, eq13_e261_d_n4, eq13_e261_d_n5, eq13_e261_d_n6, eq13_e261_d_n7, eq13_e261_d_n8, eq13_e261_d_n9, eq13_e261_d_n10];
        let eq13_branch_derivatives: [f64; 2] = [eq13_e261_d_b0, eq13_e261_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(5),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * s.v[243]);
        let eq14_e266_d_n0: f64 = (eq14_e264 * s.dn[243][0]);
        let eq14_e266_d_n1: f64 = (eq14_e264 * s.dn[243][1]);
        let eq14_e266_d_n2: f64 = (eq14_e264 * s.dn[243][2]);
        let eq14_e266_d_n3: f64 = (eq14_e264 * s.dn[243][3]);
        let eq14_e266_d_n4: f64 = (eq14_e264 * s.dn[243][4]);
        let eq14_e266_d_n5: f64 = (eq14_e264 * s.dn[243][5]);
        let eq14_e266_d_n6: f64 = (eq14_e264 * s.dn[243][6]);
        let eq14_e266_d_n7: f64 = (eq14_e264 * s.dn[243][7]);
        let eq14_e266_d_n8: f64 = (eq14_e264 * s.dn[243][8]);
        let eq14_e266_d_n9: f64 = (eq14_e264 * s.dn[243][9]);
        let eq14_e266_d_n10: f64 = (eq14_e264 * s.dn[243][10]);
        let eq14_e266_d_b0: f64 = (eq14_e264 * s.db[243][0]);
        let eq14_e266_d_b1: f64 = (eq14_e264 * s.db[243][1]);
        let eq14_e267: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq14_e266);
        let eq14_e267_d_n0: f64 = (eq14_e266_d_n0 * ddt_scale);
        let eq14_e267_d_n1: f64 = (eq14_e266_d_n1 * ddt_scale);
        let eq14_e267_d_n2: f64 = (eq14_e266_d_n2 * ddt_scale);
        let eq14_e267_d_n3: f64 = (eq14_e266_d_n3 * ddt_scale);
        let eq14_e267_d_n4: f64 = (eq14_e266_d_n4 * ddt_scale);
        let eq14_e267_d_n5: f64 = (eq14_e266_d_n5 * ddt_scale);
        let eq14_e267_d_n6: f64 = (eq14_e266_d_n6 * ddt_scale);
        let eq14_e267_d_n7: f64 = (eq14_e266_d_n7 * ddt_scale);
        let eq14_e267_d_n8: f64 = (eq14_e266_d_n8 * ddt_scale);
        let eq14_e267_d_n9: f64 = (eq14_e266_d_n9 * ddt_scale);
        let eq14_e267_d_n10: f64 = (eq14_e266_d_n10 * ddt_scale);
        let eq14_e267_d_b0: f64 = (eq14_e266_d_b0 * ddt_scale);
        let eq14_e267_d_b1: f64 = (eq14_e266_d_b1 * ddt_scale);
        let eq14_e269: f64 = (eq14_e267 * p.p1);
        let eq14_e269_d_n0: f64 = (eq14_e267_d_n0 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e267_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e267_d_n2 * p.p1);
        let eq14_e269_d_n3: f64 = (eq14_e267_d_n3 * p.p1);
        let eq14_e269_d_n4: f64 = (eq14_e267_d_n4 * p.p1);
        let eq14_e269_d_n5: f64 = (eq14_e267_d_n5 * p.p1);
        let eq14_e269_d_n6: f64 = (eq14_e267_d_n6 * p.p1);
        let eq14_e269_d_n7: f64 = (eq14_e267_d_n7 * p.p1);
        let eq14_e269_d_n8: f64 = (eq14_e267_d_n8 * p.p1);
        let eq14_e269_d_n9: f64 = (eq14_e267_d_n9 * p.p1);
        let eq14_e269_d_n10: f64 = (eq14_e267_d_n10 * p.p1);
        let eq14_e269_d_b0: f64 = (eq14_e267_d_b0 * p.p1);
        let eq14_e269_d_b1: f64 = (eq14_e267_d_b1 * p.p1);
        let eq14_value: f64 = eq14_e269;
        let eq14_node_derivatives: [f64; 11] = [eq14_e269_d_n0, eq14_e269_d_n1, eq14_e269_d_n2, eq14_e269_d_n3, eq14_e269_d_n4, eq14_e269_d_n5, eq14_e269_d_n6, eq14_e269_d_n7, eq14_e269_d_n8, eq14_e269_d_n9, eq14_e269_d_n10];
        let eq14_branch_derivatives: [f64; 2] = [eq14_e269_d_b0, eq14_e269_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * s.v[244]);
        let eq15_e274_d_n0: f64 = (eq15_e272 * s.dn[244][0]);
        let eq15_e274_d_n1: f64 = (eq15_e272 * s.dn[244][1]);
        let eq15_e274_d_n2: f64 = (eq15_e272 * s.dn[244][2]);
        let eq15_e274_d_n3: f64 = (eq15_e272 * s.dn[244][3]);
        let eq15_e274_d_n4: f64 = (eq15_e272 * s.dn[244][4]);
        let eq15_e274_d_n5: f64 = (eq15_e272 * s.dn[244][5]);
        let eq15_e274_d_n6: f64 = (eq15_e272 * s.dn[244][6]);
        let eq15_e274_d_n7: f64 = (eq15_e272 * s.dn[244][7]);
        let eq15_e274_d_n8: f64 = (eq15_e272 * s.dn[244][8]);
        let eq15_e274_d_n9: f64 = (eq15_e272 * s.dn[244][9]);
        let eq15_e274_d_n10: f64 = (eq15_e272 * s.dn[244][10]);
        let eq15_e274_d_b0: f64 = (eq15_e272 * s.db[244][0]);
        let eq15_e274_d_b1: f64 = (eq15_e272 * s.db[244][1]);
        let eq15_e275: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq15_e274);
        let eq15_e275_d_n0: f64 = (eq15_e274_d_n0 * ddt_scale);
        let eq15_e275_d_n1: f64 = (eq15_e274_d_n1 * ddt_scale);
        let eq15_e275_d_n2: f64 = (eq15_e274_d_n2 * ddt_scale);
        let eq15_e275_d_n3: f64 = (eq15_e274_d_n3 * ddt_scale);
        let eq15_e275_d_n4: f64 = (eq15_e274_d_n4 * ddt_scale);
        let eq15_e275_d_n5: f64 = (eq15_e274_d_n5 * ddt_scale);
        let eq15_e275_d_n6: f64 = (eq15_e274_d_n6 * ddt_scale);
        let eq15_e275_d_n7: f64 = (eq15_e274_d_n7 * ddt_scale);
        let eq15_e275_d_n8: f64 = (eq15_e274_d_n8 * ddt_scale);
        let eq15_e275_d_n9: f64 = (eq15_e274_d_n9 * ddt_scale);
        let eq15_e275_d_n10: f64 = (eq15_e274_d_n10 * ddt_scale);
        let eq15_e275_d_b0: f64 = (eq15_e274_d_b0 * ddt_scale);
        let eq15_e275_d_b1: f64 = (eq15_e274_d_b1 * ddt_scale);
        let eq15_e277: f64 = (eq15_e275 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e277_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e277_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e277_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e277_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e277_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e277_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e277_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e277_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e277_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e277_d_b0: f64 = (eq15_e275_d_b0 * p.p1);
        let eq15_e277_d_b1: f64 = (eq15_e275_d_b1 * p.p1);
        let eq15_value: f64 = eq15_e277;
        let eq15_node_derivatives: [f64; 11] = [eq15_e277_d_n0, eq15_e277_d_n1, eq15_e277_d_n2, eq15_e277_d_n3, eq15_e277_d_n4, eq15_e277_d_n5, eq15_e277_d_n6, eq15_e277_d_n7, eq15_e277_d_n8, eq15_e277_d_n9, eq15_e277_d_n10];
        let eq15_branch_derivatives: [f64; 2] = [eq15_e277_d_b0, eq15_e277_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq18_e293: f64 = (s.v[219] + s.v[228]);
        let eq18_e293_d_n0: f64 = (s.dn[219][0] + s.dn[228][0]);
        let eq18_e293_d_n1: f64 = (s.dn[219][1] + s.dn[228][1]);
        let eq18_e293_d_n2: f64 = (s.dn[219][2] + s.dn[228][2]);
        let eq18_e293_d_n3: f64 = (s.dn[219][3] + s.dn[228][3]);
        let eq18_e293_d_n4: f64 = (s.dn[219][4] + s.dn[228][4]);
        let eq18_e293_d_n5: f64 = (s.dn[219][5] + s.dn[228][5]);
        let eq18_e293_d_n6: f64 = (s.dn[219][6] + s.dn[228][6]);
        let eq18_e293_d_n7: f64 = (s.dn[219][7] + s.dn[228][7]);
        let eq18_e293_d_n8: f64 = (s.dn[219][8] + s.dn[228][8]);
        let eq18_e293_d_n9: f64 = (s.dn[219][9] + s.dn[228][9]);
        let eq18_e293_d_n10: f64 = (s.dn[219][10] + s.dn[228][10]);
        let eq18_e293_d_b0: f64 = (s.db[219][0] + s.db[228][0]);
        let eq18_e293_d_b1: f64 = (s.db[219][1] + s.db[228][1]);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n2: f64 = (p.p3 * eq18_e293_d_n2);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e294_d_n10: f64 = (p.p3 * eq18_e293_d_n10);
        let eq18_e294_d_b0: f64 = (p.p3 * eq18_e293_d_b0);
        let eq18_e294_d_b1: f64 = (p.p3 * eq18_e293_d_b1);
        let eq18_e295: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq18_e294);
        let eq18_e295_d_n0: f64 = (eq18_e294_d_n0 * ddt_scale);
        let eq18_e295_d_n1: f64 = (eq18_e294_d_n1 * ddt_scale);
        let eq18_e295_d_n2: f64 = (eq18_e294_d_n2 * ddt_scale);
        let eq18_e295_d_n3: f64 = (eq18_e294_d_n3 * ddt_scale);
        let eq18_e295_d_n4: f64 = (eq18_e294_d_n4 * ddt_scale);
        let eq18_e295_d_n5: f64 = (eq18_e294_d_n5 * ddt_scale);
        let eq18_e295_d_n6: f64 = (eq18_e294_d_n6 * ddt_scale);
        let eq18_e295_d_n7: f64 = (eq18_e294_d_n7 * ddt_scale);
        let eq18_e295_d_n8: f64 = (eq18_e294_d_n8 * ddt_scale);
        let eq18_e295_d_n9: f64 = (eq18_e294_d_n9 * ddt_scale);
        let eq18_e295_d_n10: f64 = (eq18_e294_d_n10 * ddt_scale);
        let eq18_e295_d_b0: f64 = (eq18_e294_d_b0 * ddt_scale);
        let eq18_e295_d_b1: f64 = (eq18_e294_d_b1 * ddt_scale);
        let eq18_e297: f64 = (eq18_e295 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e295_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e295_d_n1 * p.p1);
        let eq18_e297_d_n2: f64 = (eq18_e295_d_n2 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e295_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e295_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e295_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e295_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e295_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e295_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e295_d_n9 * p.p1);
        let eq18_e297_d_n10: f64 = (eq18_e295_d_n10 * p.p1);
        let eq18_e297_d_b0: f64 = (eq18_e295_d_b0 * p.p1);
        let eq18_e297_d_b1: f64 = (eq18_e295_d_b1 * p.p1);
        let eq18_value: f64 = eq18_e297;
        let eq18_node_derivatives: [f64; 11] = [eq18_e297_d_n0, eq18_e297_d_n1, eq18_e297_d_n2, eq18_e297_d_n3, eq18_e297_d_n4, eq18_e297_d_n5, eq18_e297_d_n6, eq18_e297_d_n7, eq18_e297_d_n8, eq18_e297_d_n9, eq18_e297_d_n10];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e297_d_b0, eq18_e297_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(8),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq20_e312: f64 = (s.v[216] + s.v[229]);
        let eq20_e312_d_n0: f64 = (s.dn[216][0] + s.dn[229][0]);
        let eq20_e312_d_n1: f64 = (s.dn[216][1] + s.dn[229][1]);
        let eq20_e312_d_n2: f64 = (s.dn[216][2] + s.dn[229][2]);
        let eq20_e312_d_n3: f64 = (s.dn[216][3] + s.dn[229][3]);
        let eq20_e312_d_n4: f64 = (s.dn[216][4] + s.dn[229][4]);
        let eq20_e312_d_n5: f64 = (s.dn[216][5] + s.dn[229][5]);
        let eq20_e312_d_n6: f64 = (s.dn[216][6] + s.dn[229][6]);
        let eq20_e312_d_n7: f64 = (s.dn[216][7] + s.dn[229][7]);
        let eq20_e312_d_n8: f64 = (s.dn[216][8] + s.dn[229][8]);
        let eq20_e312_d_n9: f64 = (s.dn[216][9] + s.dn[229][9]);
        let eq20_e312_d_n10: f64 = (s.dn[216][10] + s.dn[229][10]);
        let eq20_e312_d_b0: f64 = (s.db[216][0] + s.db[229][0]);
        let eq20_e312_d_b1: f64 = (s.db[216][1] + s.db[229][1]);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n2: f64 = (p.p3 * eq20_e312_d_n2);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e313_d_n10: f64 = (p.p3 * eq20_e312_d_n10);
        let eq20_e313_d_b0: f64 = (p.p3 * eq20_e312_d_b0);
        let eq20_e313_d_b1: f64 = (p.p3 * eq20_e312_d_b1);
        let eq20_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq20_e313);
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
        let eq20_e316_d_b0: f64 = (eq20_e314_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e314_d_b1 * p.p1);
        let eq20_value: f64 = eq20_e316;
        let eq20_node_derivatives: [f64; 11] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(9),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq27_e355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, (nv10 - 0.0));
        let eq27_e356: f64 = (s.v[306] * eq27_e355);
        let eq27_e356_d_n0: f64 = (s.dn[306][0] * eq27_e355);
        let eq27_e356_d_n1: f64 = (s.dn[306][1] * eq27_e355);
        let eq27_e356_d_n2: f64 = (s.dn[306][2] * eq27_e355);
        let eq27_e356_d_n3: f64 = (s.dn[306][3] * eq27_e355);
        let eq27_e356_d_n4: f64 = (s.dn[306][4] * eq27_e355);
        let eq27_e356_d_n5: f64 = (s.dn[306][5] * eq27_e355);
        let eq27_e356_d_n6: f64 = (s.dn[306][6] * eq27_e355);
        let eq27_e356_d_n7: f64 = (s.dn[306][7] * eq27_e355);
        let eq27_e356_d_n8: f64 = (s.dn[306][8] * eq27_e355);
        let eq27_e356_d_n9: f64 = (s.dn[306][9] * eq27_e355);
        let eq27_e356_d_n10: f64 = ((s.dn[306][10] * eq27_e355) + (s.v[306] * ddt_scale));
        let eq27_e356_d_b0: f64 = (s.db[306][0] * eq27_e355);
        let eq27_e356_d_b1: f64 = (s.db[306][1] * eq27_e355);
        let eq27_value: f64 = eq27_e356;
        let eq27_node_derivatives: [f64; 11] = [eq27_e356_d_n0, eq27_e356_d_n1, eq27_e356_d_n2, eq27_e356_d_n3, eq27_e356_d_n4, eq27_e356_d_n5, eq27_e356_d_n6, eq27_e356_d_n7, eq27_e356_d_n8, eq27_e356_d_n9, eq27_e356_d_n10];
        let eq27_branch_derivatives: [f64; 2] = [eq27_e356_d_b0, eq27_e356_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e233: f64 = (s.v[204] + s.v[209]);
        let eq10_e233_d_n0: f64 = (s.dn[204][0] + s.dn[209][0]);
        let eq10_e233_d_n1: f64 = (s.dn[204][1] + s.dn[209][1]);
        let eq10_e233_d_n2: f64 = (s.dn[204][2] + s.dn[209][2]);
        let eq10_e233_d_n3: f64 = (s.dn[204][3] + s.dn[209][3]);
        let eq10_e233_d_n4: f64 = (s.dn[204][4] + s.dn[209][4]);
        let eq10_e233_d_n5: f64 = (s.dn[204][5] + s.dn[209][5]);
        let eq10_e233_d_n6: f64 = (s.dn[204][6] + s.dn[209][6]);
        let eq10_e233_d_n7: f64 = (s.dn[204][7] + s.dn[209][7]);
        let eq10_e233_d_n8: f64 = (s.dn[204][8] + s.dn[209][8]);
        let eq10_e233_d_n9: f64 = (s.dn[204][9] + s.dn[209][9]);
        let eq10_e233_d_n10: f64 = (s.dn[204][10] + s.dn[209][10]);
        let eq10_e233_d_b0: f64 = (s.db[204][0] + s.db[209][0]);
        let eq10_e233_d_b1: f64 = (s.db[204][1] + s.db[209][1]);
        let eq10_e235: f64 = (eq10_e233 + s.v[221]);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + s.dn[221][0]);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + s.dn[221][1]);
        let eq10_e235_d_n2: f64 = (eq10_e233_d_n2 + s.dn[221][2]);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + s.dn[221][3]);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + s.dn[221][4]);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + s.dn[221][5]);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + s.dn[221][6]);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + s.dn[221][7]);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + s.dn[221][8]);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + s.dn[221][9]);
        let eq10_e235_d_n10: f64 = (eq10_e233_d_n10 + s.dn[221][10]);
        let eq10_e235_d_b0: f64 = (eq10_e233_d_b0 + s.db[221][0]);
        let eq10_e235_d_b1: f64 = (eq10_e233_d_b1 + s.db[221][1]);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n2: f64 = (p.p3 * eq10_e235_d_n2);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e236_d_n10: f64 = (p.p3 * eq10_e235_d_n10);
        let eq10_e236_d_b0: f64 = (p.p3 * eq10_e235_d_b0);
        let eq10_e236_d_b1: f64 = (p.p3 * eq10_e235_d_b1);
        let eq10_e237_q: f64 = eq10_e236;
        let eq10_e239: f64 = (eq10_e236 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e236_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e236_d_n1 * p.p1);
        let eq10_e239_d_n2: f64 = (eq10_e236_d_n2 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e236_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e236_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e236_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e236_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e236_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e236_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e236_d_n9 * p.p1);
        let eq10_e239_d_n10: f64 = (eq10_e236_d_n10 * p.p1);
        let eq10_e239_d_b0: f64 = (eq10_e236_d_b0 * p.p1);
        let eq10_e239_d_b1: f64 = (eq10_e236_d_b1 * p.p1);
        let eq10_e239_q: f64 = (eq10_e237_q * p.p1);
        let eq10_reactive_node_derivatives: [f64; 11] = [eq10_e239_d_n0, eq10_e239_d_n1, eq10_e239_d_n2, eq10_e239_d_n3, eq10_e239_d_n4, eq10_e239_d_n5, eq10_e239_d_n6, eq10_e239_d_n7, eq10_e239_d_n8, eq10_e239_d_n9, eq10_e239_d_n10];
        let eq10_reactive_branch_derivatives: [f64; 2] = [eq10_e239_d_b0, eq10_e239_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e242: f64 = (p.p3 * s.v[206]);
        let eq11_e242_d_n0: f64 = (p.p3 * s.dn[206][0]);
        let eq11_e242_d_n1: f64 = (p.p3 * s.dn[206][1]);
        let eq11_e242_d_n2: f64 = (p.p3 * s.dn[206][2]);
        let eq11_e242_d_n3: f64 = (p.p3 * s.dn[206][3]);
        let eq11_e242_d_n4: f64 = (p.p3 * s.dn[206][4]);
        let eq11_e242_d_n5: f64 = (p.p3 * s.dn[206][5]);
        let eq11_e242_d_n6: f64 = (p.p3 * s.dn[206][6]);
        let eq11_e242_d_n7: f64 = (p.p3 * s.dn[206][7]);
        let eq11_e242_d_n8: f64 = (p.p3 * s.dn[206][8]);
        let eq11_e242_d_n9: f64 = (p.p3 * s.dn[206][9]);
        let eq11_e242_d_n10: f64 = (p.p3 * s.dn[206][10]);
        let eq11_e242_d_b0: f64 = (p.p3 * s.db[206][0]);
        let eq11_e242_d_b1: f64 = (p.p3 * s.db[206][1]);
        let eq11_e243_q: f64 = eq11_e242;
        let eq11_e245: f64 = (eq11_e242 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e242_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e242_d_n1 * p.p1);
        let eq11_e245_d_n2: f64 = (eq11_e242_d_n2 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e242_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e242_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e242_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e242_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e242_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e242_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e242_d_n9 * p.p1);
        let eq11_e245_d_n10: f64 = (eq11_e242_d_n10 * p.p1);
        let eq11_e245_d_b0: f64 = (eq11_e242_d_b0 * p.p1);
        let eq11_e245_d_b1: f64 = (eq11_e242_d_b1 * p.p1);
        let eq11_e245_q: f64 = (eq11_e243_q * p.p1);
        let eq11_reactive_node_derivatives: [f64; 11] = [eq11_e245_d_n0, eq11_e245_d_n1, eq11_e245_d_n2, eq11_e245_d_n3, eq11_e245_d_n4, eq11_e245_d_n5, eq11_e245_d_n6, eq11_e245_d_n7, eq11_e245_d_n8, eq11_e245_d_n9, eq11_e245_d_n10];
        let eq11_reactive_branch_derivatives: [f64; 2] = [eq11_e245_d_b0, eq11_e245_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e249: f64 = (s.v[207] + s.v[210]);
        let eq12_e249_d_n0: f64 = (s.dn[207][0] + s.dn[210][0]);
        let eq12_e249_d_n1: f64 = (s.dn[207][1] + s.dn[210][1]);
        let eq12_e249_d_n2: f64 = (s.dn[207][2] + s.dn[210][2]);
        let eq12_e249_d_n3: f64 = (s.dn[207][3] + s.dn[210][3]);
        let eq12_e249_d_n4: f64 = (s.dn[207][4] + s.dn[210][4]);
        let eq12_e249_d_n5: f64 = (s.dn[207][5] + s.dn[210][5]);
        let eq12_e249_d_n6: f64 = (s.dn[207][6] + s.dn[210][6]);
        let eq12_e249_d_n7: f64 = (s.dn[207][7] + s.dn[210][7]);
        let eq12_e249_d_n8: f64 = (s.dn[207][8] + s.dn[210][8]);
        let eq12_e249_d_n9: f64 = (s.dn[207][9] + s.dn[210][9]);
        let eq12_e249_d_n10: f64 = (s.dn[207][10] + s.dn[210][10]);
        let eq12_e249_d_b0: f64 = (s.db[207][0] + s.db[210][0]);
        let eq12_e249_d_b1: f64 = (s.db[207][1] + s.db[210][1]);
        let eq12_e251: f64 = (eq12_e249 + s.v[224]);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + s.dn[224][0]);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + s.dn[224][1]);
        let eq12_e251_d_n2: f64 = (eq12_e249_d_n2 + s.dn[224][2]);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + s.dn[224][3]);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + s.dn[224][4]);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + s.dn[224][5]);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + s.dn[224][6]);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + s.dn[224][7]);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + s.dn[224][8]);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + s.dn[224][9]);
        let eq12_e251_d_n10: f64 = (eq12_e249_d_n10 + s.dn[224][10]);
        let eq12_e251_d_b0: f64 = (eq12_e249_d_b0 + s.db[224][0]);
        let eq12_e251_d_b1: f64 = (eq12_e249_d_b1 + s.db[224][1]);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n2: f64 = (p.p3 * eq12_e251_d_n2);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e252_d_n10: f64 = (p.p3 * eq12_e251_d_n10);
        let eq12_e252_d_b0: f64 = (p.p3 * eq12_e251_d_b0);
        let eq12_e252_d_b1: f64 = (p.p3 * eq12_e251_d_b1);
        let eq12_e253_q: f64 = eq12_e252;
        let eq12_e255: f64 = (eq12_e252 * p.p1);
        let eq12_e255_d_n0: f64 = (eq12_e252_d_n0 * p.p1);
        let eq12_e255_d_n1: f64 = (eq12_e252_d_n1 * p.p1);
        let eq12_e255_d_n2: f64 = (eq12_e252_d_n2 * p.p1);
        let eq12_e255_d_n3: f64 = (eq12_e252_d_n3 * p.p1);
        let eq12_e255_d_n4: f64 = (eq12_e252_d_n4 * p.p1);
        let eq12_e255_d_n5: f64 = (eq12_e252_d_n5 * p.p1);
        let eq12_e255_d_n6: f64 = (eq12_e252_d_n6 * p.p1);
        let eq12_e255_d_n7: f64 = (eq12_e252_d_n7 * p.p1);
        let eq12_e255_d_n8: f64 = (eq12_e252_d_n8 * p.p1);
        let eq12_e255_d_n9: f64 = (eq12_e252_d_n9 * p.p1);
        let eq12_e255_d_n10: f64 = (eq12_e252_d_n10 * p.p1);
        let eq12_e255_d_b0: f64 = (eq12_e252_d_b0 * p.p1);
        let eq12_e255_d_b1: f64 = (eq12_e252_d_b1 * p.p1);
        let eq12_e255_q: f64 = (eq12_e253_q * p.p1);
        let eq12_reactive_node_derivatives: [f64; 11] = [eq12_e255_d_n0, eq12_e255_d_n1, eq12_e255_d_n2, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, eq12_e255_d_n10];
        let eq12_reactive_branch_derivatives: [f64; 2] = [eq12_e255_d_b0, eq12_e255_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e258: f64 = (p.p3 * s.v[211]);
        let eq13_e258_d_n0: f64 = (p.p3 * s.dn[211][0]);
        let eq13_e258_d_n1: f64 = (p.p3 * s.dn[211][1]);
        let eq13_e258_d_n2: f64 = (p.p3 * s.dn[211][2]);
        let eq13_e258_d_n3: f64 = (p.p3 * s.dn[211][3]);
        let eq13_e258_d_n4: f64 = (p.p3 * s.dn[211][4]);
        let eq13_e258_d_n5: f64 = (p.p3 * s.dn[211][5]);
        let eq13_e258_d_n6: f64 = (p.p3 * s.dn[211][6]);
        let eq13_e258_d_n7: f64 = (p.p3 * s.dn[211][7]);
        let eq13_e258_d_n8: f64 = (p.p3 * s.dn[211][8]);
        let eq13_e258_d_n9: f64 = (p.p3 * s.dn[211][9]);
        let eq13_e258_d_n10: f64 = (p.p3 * s.dn[211][10]);
        let eq13_e258_d_b0: f64 = (p.p3 * s.db[211][0]);
        let eq13_e258_d_b1: f64 = (p.p3 * s.db[211][1]);
        let eq13_e259_q: f64 = eq13_e258;
        let eq13_e261: f64 = (eq13_e258 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e258_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e258_d_n1 * p.p1);
        let eq13_e261_d_n2: f64 = (eq13_e258_d_n2 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e258_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e258_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e258_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e258_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e258_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e258_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e258_d_n9 * p.p1);
        let eq13_e261_d_n10: f64 = (eq13_e258_d_n10 * p.p1);
        let eq13_e261_d_b0: f64 = (eq13_e258_d_b0 * p.p1);
        let eq13_e261_d_b1: f64 = (eq13_e258_d_b1 * p.p1);
        let eq13_e261_q: f64 = (eq13_e259_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 11] = [eq13_e261_d_n0, eq13_e261_d_n1, eq13_e261_d_n2, eq13_e261_d_n3, eq13_e261_d_n4, eq13_e261_d_n5, eq13_e261_d_n6, eq13_e261_d_n7, eq13_e261_d_n8, eq13_e261_d_n9, eq13_e261_d_n10];
        let eq13_reactive_branch_derivatives: [f64; 2] = [eq13_e261_d_b0, eq13_e261_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * s.v[243]);
        let eq14_e266_d_n0: f64 = (eq14_e264 * s.dn[243][0]);
        let eq14_e266_d_n1: f64 = (eq14_e264 * s.dn[243][1]);
        let eq14_e266_d_n2: f64 = (eq14_e264 * s.dn[243][2]);
        let eq14_e266_d_n3: f64 = (eq14_e264 * s.dn[243][3]);
        let eq14_e266_d_n4: f64 = (eq14_e264 * s.dn[243][4]);
        let eq14_e266_d_n5: f64 = (eq14_e264 * s.dn[243][5]);
        let eq14_e266_d_n6: f64 = (eq14_e264 * s.dn[243][6]);
        let eq14_e266_d_n7: f64 = (eq14_e264 * s.dn[243][7]);
        let eq14_e266_d_n8: f64 = (eq14_e264 * s.dn[243][8]);
        let eq14_e266_d_n9: f64 = (eq14_e264 * s.dn[243][9]);
        let eq14_e266_d_n10: f64 = (eq14_e264 * s.dn[243][10]);
        let eq14_e266_d_b0: f64 = (eq14_e264 * s.db[243][0]);
        let eq14_e266_d_b1: f64 = (eq14_e264 * s.db[243][1]);
        let eq14_e267_q: f64 = eq14_e266;
        let eq14_e269: f64 = (eq14_e266 * p.p1);
        let eq14_e269_d_n0: f64 = (eq14_e266_d_n0 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e266_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e266_d_n2 * p.p1);
        let eq14_e269_d_n3: f64 = (eq14_e266_d_n3 * p.p1);
        let eq14_e269_d_n4: f64 = (eq14_e266_d_n4 * p.p1);
        let eq14_e269_d_n5: f64 = (eq14_e266_d_n5 * p.p1);
        let eq14_e269_d_n6: f64 = (eq14_e266_d_n6 * p.p1);
        let eq14_e269_d_n7: f64 = (eq14_e266_d_n7 * p.p1);
        let eq14_e269_d_n8: f64 = (eq14_e266_d_n8 * p.p1);
        let eq14_e269_d_n9: f64 = (eq14_e266_d_n9 * p.p1);
        let eq14_e269_d_n10: f64 = (eq14_e266_d_n10 * p.p1);
        let eq14_e269_d_b0: f64 = (eq14_e266_d_b0 * p.p1);
        let eq14_e269_d_b1: f64 = (eq14_e266_d_b1 * p.p1);
        let eq14_e269_q: f64 = (eq14_e267_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 11] = [eq14_e269_d_n0, eq14_e269_d_n1, eq14_e269_d_n2, eq14_e269_d_n3, eq14_e269_d_n4, eq14_e269_d_n5, eq14_e269_d_n6, eq14_e269_d_n7, eq14_e269_d_n8, eq14_e269_d_n9, eq14_e269_d_n10];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e269_d_b0, eq14_e269_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * s.v[244]);
        let eq15_e274_d_n0: f64 = (eq15_e272 * s.dn[244][0]);
        let eq15_e274_d_n1: f64 = (eq15_e272 * s.dn[244][1]);
        let eq15_e274_d_n2: f64 = (eq15_e272 * s.dn[244][2]);
        let eq15_e274_d_n3: f64 = (eq15_e272 * s.dn[244][3]);
        let eq15_e274_d_n4: f64 = (eq15_e272 * s.dn[244][4]);
        let eq15_e274_d_n5: f64 = (eq15_e272 * s.dn[244][5]);
        let eq15_e274_d_n6: f64 = (eq15_e272 * s.dn[244][6]);
        let eq15_e274_d_n7: f64 = (eq15_e272 * s.dn[244][7]);
        let eq15_e274_d_n8: f64 = (eq15_e272 * s.dn[244][8]);
        let eq15_e274_d_n9: f64 = (eq15_e272 * s.dn[244][9]);
        let eq15_e274_d_n10: f64 = (eq15_e272 * s.dn[244][10]);
        let eq15_e274_d_b0: f64 = (eq15_e272 * s.db[244][0]);
        let eq15_e274_d_b1: f64 = (eq15_e272 * s.db[244][1]);
        let eq15_e275_q: f64 = eq15_e274;
        let eq15_e277: f64 = (eq15_e274 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e274_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e274_d_n1 * p.p1);
        let eq15_e277_d_n2: f64 = (eq15_e274_d_n2 * p.p1);
        let eq15_e277_d_n3: f64 = (eq15_e274_d_n3 * p.p1);
        let eq15_e277_d_n4: f64 = (eq15_e274_d_n4 * p.p1);
        let eq15_e277_d_n5: f64 = (eq15_e274_d_n5 * p.p1);
        let eq15_e277_d_n6: f64 = (eq15_e274_d_n6 * p.p1);
        let eq15_e277_d_n7: f64 = (eq15_e274_d_n7 * p.p1);
        let eq15_e277_d_n8: f64 = (eq15_e274_d_n8 * p.p1);
        let eq15_e277_d_n9: f64 = (eq15_e274_d_n9 * p.p1);
        let eq15_e277_d_n10: f64 = (eq15_e274_d_n10 * p.p1);
        let eq15_e277_d_b0: f64 = (eq15_e274_d_b0 * p.p1);
        let eq15_e277_d_b1: f64 = (eq15_e274_d_b1 * p.p1);
        let eq15_e277_q: f64 = (eq15_e275_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 11] = [eq15_e277_d_n0, eq15_e277_d_n1, eq15_e277_d_n2, eq15_e277_d_n3, eq15_e277_d_n4, eq15_e277_d_n5, eq15_e277_d_n6, eq15_e277_d_n7, eq15_e277_d_n8, eq15_e277_d_n9, eq15_e277_d_n10];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e277_d_b0, eq15_e277_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e293: f64 = (s.v[219] + s.v[228]);
        let eq18_e293_d_n0: f64 = (s.dn[219][0] + s.dn[228][0]);
        let eq18_e293_d_n1: f64 = (s.dn[219][1] + s.dn[228][1]);
        let eq18_e293_d_n2: f64 = (s.dn[219][2] + s.dn[228][2]);
        let eq18_e293_d_n3: f64 = (s.dn[219][3] + s.dn[228][3]);
        let eq18_e293_d_n4: f64 = (s.dn[219][4] + s.dn[228][4]);
        let eq18_e293_d_n5: f64 = (s.dn[219][5] + s.dn[228][5]);
        let eq18_e293_d_n6: f64 = (s.dn[219][6] + s.dn[228][6]);
        let eq18_e293_d_n7: f64 = (s.dn[219][7] + s.dn[228][7]);
        let eq18_e293_d_n8: f64 = (s.dn[219][8] + s.dn[228][8]);
        let eq18_e293_d_n9: f64 = (s.dn[219][9] + s.dn[228][9]);
        let eq18_e293_d_n10: f64 = (s.dn[219][10] + s.dn[228][10]);
        let eq18_e293_d_b0: f64 = (s.db[219][0] + s.db[228][0]);
        let eq18_e293_d_b1: f64 = (s.db[219][1] + s.db[228][1]);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n2: f64 = (p.p3 * eq18_e293_d_n2);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e294_d_n10: f64 = (p.p3 * eq18_e293_d_n10);
        let eq18_e294_d_b0: f64 = (p.p3 * eq18_e293_d_b0);
        let eq18_e294_d_b1: f64 = (p.p3 * eq18_e293_d_b1);
        let eq18_e295_q: f64 = eq18_e294;
        let eq18_e297: f64 = (eq18_e294 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e294_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e294_d_n1 * p.p1);
        let eq18_e297_d_n2: f64 = (eq18_e294_d_n2 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e294_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e294_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e294_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e294_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e294_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e294_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e294_d_n9 * p.p1);
        let eq18_e297_d_n10: f64 = (eq18_e294_d_n10 * p.p1);
        let eq18_e297_d_b0: f64 = (eq18_e294_d_b0 * p.p1);
        let eq18_e297_d_b1: f64 = (eq18_e294_d_b1 * p.p1);
        let eq18_e297_q: f64 = (eq18_e295_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 11] = [eq18_e297_d_n0, eq18_e297_d_n1, eq18_e297_d_n2, eq18_e297_d_n3, eq18_e297_d_n4, eq18_e297_d_n5, eq18_e297_d_n6, eq18_e297_d_n7, eq18_e297_d_n8, eq18_e297_d_n9, eq18_e297_d_n10];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e297_d_b0, eq18_e297_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e312: f64 = (s.v[216] + s.v[229]);
        let eq20_e312_d_n0: f64 = (s.dn[216][0] + s.dn[229][0]);
        let eq20_e312_d_n1: f64 = (s.dn[216][1] + s.dn[229][1]);
        let eq20_e312_d_n2: f64 = (s.dn[216][2] + s.dn[229][2]);
        let eq20_e312_d_n3: f64 = (s.dn[216][3] + s.dn[229][3]);
        let eq20_e312_d_n4: f64 = (s.dn[216][4] + s.dn[229][4]);
        let eq20_e312_d_n5: f64 = (s.dn[216][5] + s.dn[229][5]);
        let eq20_e312_d_n6: f64 = (s.dn[216][6] + s.dn[229][6]);
        let eq20_e312_d_n7: f64 = (s.dn[216][7] + s.dn[229][7]);
        let eq20_e312_d_n8: f64 = (s.dn[216][8] + s.dn[229][8]);
        let eq20_e312_d_n9: f64 = (s.dn[216][9] + s.dn[229][9]);
        let eq20_e312_d_n10: f64 = (s.dn[216][10] + s.dn[229][10]);
        let eq20_e312_d_b0: f64 = (s.db[216][0] + s.db[229][0]);
        let eq20_e312_d_b1: f64 = (s.db[216][1] + s.db[229][1]);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n2: f64 = (p.p3 * eq20_e312_d_n2);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e313_d_n10: f64 = (p.p3 * eq20_e312_d_n10);
        let eq20_e313_d_b0: f64 = (p.p3 * eq20_e312_d_b0);
        let eq20_e313_d_b1: f64 = (p.p3 * eq20_e312_d_b1);
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
        let eq20_e316_d_b0: f64 = (eq20_e313_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e313_d_b1 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 11] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e355_q: f64 = (nv10 - 0.0);
        let eq27_e356: f64 = (s.v[306] * (nv10 - 0.0));
        let eq27_e356_d_n0: f64 = (s.dn[306][0] * (nv10 - 0.0));
        let eq27_e356_d_n1: f64 = (s.dn[306][1] * (nv10 - 0.0));
        let eq27_e356_d_n2: f64 = (s.dn[306][2] * (nv10 - 0.0));
        let eq27_e356_d_n3: f64 = (s.dn[306][3] * (nv10 - 0.0));
        let eq27_e356_d_n4: f64 = (s.dn[306][4] * (nv10 - 0.0));
        let eq27_e356_d_n5: f64 = (s.dn[306][5] * (nv10 - 0.0));
        let eq27_e356_d_n6: f64 = (s.dn[306][6] * (nv10 - 0.0));
        let eq27_e356_d_n7: f64 = (s.dn[306][7] * (nv10 - 0.0));
        let eq27_e356_d_n8: f64 = (s.dn[306][8] * (nv10 - 0.0));
        let eq27_e356_d_n9: f64 = (s.dn[306][9] * (nv10 - 0.0));
        let eq27_e356_d_n10: f64 = ((s.dn[306][10] * (nv10 - 0.0)) + s.v[306]);
        let eq27_e356_d_b0: f64 = (s.db[306][0] * (nv10 - 0.0));
        let eq27_e356_d_b1: f64 = (s.db[306][1] * (nv10 - 0.0));
        let eq27_e356_q: f64 = (s.v[306] * eq27_e355_q);
        let eq27_e356_q_d_n0: f64 = (s.dn[306][0] * eq27_e355_q);
        let eq27_e356_q_d_n1: f64 = (s.dn[306][1] * eq27_e355_q);
        let eq27_e356_q_d_n2: f64 = (s.dn[306][2] * eq27_e355_q);
        let eq27_e356_q_d_n3: f64 = (s.dn[306][3] * eq27_e355_q);
        let eq27_e356_q_d_n4: f64 = (s.dn[306][4] * eq27_e355_q);
        let eq27_e356_q_d_n5: f64 = (s.dn[306][5] * eq27_e355_q);
        let eq27_e356_q_d_n6: f64 = (s.dn[306][6] * eq27_e355_q);
        let eq27_e356_q_d_n7: f64 = (s.dn[306][7] * eq27_e355_q);
        let eq27_e356_q_d_n8: f64 = (s.dn[306][8] * eq27_e355_q);
        let eq27_e356_q_d_n9: f64 = (s.dn[306][9] * eq27_e355_q);
        let eq27_e356_q_d_n10: f64 = ((s.dn[306][10] * eq27_e355_q) + s.v[306]);
        let eq27_e356_q_d_b0: f64 = (s.db[306][0] * eq27_e355_q);
        let eq27_e356_q_d_b1: f64 = (s.db[306][1] * eq27_e355_q);
        let eq27_reactive_node_derivatives: [f64; 11] = [eq27_e356_q_d_n0, eq27_e356_q_d_n1, eq27_e356_q_d_n2, eq27_e356_q_d_n3, eq27_e356_q_d_n4, eq27_e356_q_d_n5, eq27_e356_q_d_n6, eq27_e356_q_d_n7, eq27_e356_q_d_n8, eq27_e356_q_d_n9, eq27_e356_q_d_n10];
        let eq27_reactive_branch_derivatives: [f64; 2] = [eq27_e356_q_d_b0, eq27_e356_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
