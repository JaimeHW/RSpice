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

        let assign300_e742: f64 = (2.0 - s.v[76]);
        let assign300_e743: f64 = (2.0_f64).powf(assign300_e742);
        s.v[79] = assign300_e743;

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

        let assign370_e814: f64 = (1.0 / s.v[87]);
        s.v[86] = assign370_e814;

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
            s.store_add_scaled_inputs_ad(70, A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1);
        }

        s.store_scaled_offset(279, 88, (((-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))) + ((-0.05))), 10.0);

        s.b[482] = ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        let (assign690_e966,) = {
    if s.b[482] {
        let assign690_e960: f64 = (s.v[279]).exp();
        let assign690_e961: f64 = (1.0 + assign690_e960);
        let assign690_e962: f64 = (assign690_e961).ln();
        let assign690_e963: f64 = (0.1 * assign690_e962);
        let assign690_e964: f64 = (0.05 + assign690_e963);
        (assign690_e964,)
    } else {
        (s.v[85],)
    }
};
        s.v[85] = assign690_e966;

        let (assign700_e990,) = {
    if (!s.b[482]) {
        let assign700_e972: f64 = (p.p118 * s.v[2]);
        let assign700_e974: f64 = (assign700_e972 * s.v[2]);
        let assign700_e977: f64 = (s.v[2] + p.p119);
        let assign700_e978: f64 = (assign700_e974 / assign700_e977);
        let assign700_e979: f64 = (s.v[88] - assign700_e978);
        let assign700_e983: f64 = (-s.v[279]);
        let assign700_e984: f64 = (assign700_e983).exp();
        let assign700_e985: f64 = (1.0 + assign700_e984);
        let assign700_e986: f64 = (assign700_e985).ln();
        let assign700_e987: f64 = (0.1 * assign700_e986);
        let assign700_e988: f64 = (assign700_e979 + assign700_e987);
        (assign700_e988,)
    } else {
        (s.v[85],)
    }
};
        s.v[85] = assign700_e990;

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

        let (assign940_e1226,) = {
    if s.b[487] {
        let assign940_e1220: f64 = (s.v[279]).exp();
        let assign940_e1221: f64 = (1.0 + assign940_e1220);
        let assign940_e1222: f64 = (assign940_e1221).ln();
        let assign940_e1223: f64 = (s.v[6] * assign940_e1222);
        let assign940_e1224: f64 = (s.v[20] + assign940_e1223);
        (assign940_e1224,)
    } else {
        (s.v[19],)
    }
};
        s.v[19] = assign940_e1226;

        let (assign950_e1240,) = {
    if (!s.b[487]) {
        let assign950_e1233: f64 = (-s.v[279]);
        let assign950_e1234: f64 = (assign950_e1233).exp();
        let assign950_e1235: f64 = (1.0 + assign950_e1234);
        let assign950_e1236: f64 = (assign950_e1235).ln();
        let assign950_e1237: f64 = (s.v[6] * assign950_e1236);
        let assign950_e1238: f64 = (0.05 + assign950_e1237);
        (assign950_e1238,)
    } else {
        (s.v[19],)
    }
};
        s.v[19] = assign950_e1240;

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p27 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[279] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[488] = (0.05 < s.v[56]);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        let (assign990_e1276,) = {
    if s.b[488] {
        let assign990_e1270: f64 = (s.v[279]).exp();
        let assign990_e1271: f64 = (1.0 + assign990_e1270);
        let assign990_e1272: f64 = (assign990_e1271).ln();
        let assign990_e1273: f64 = (s.v[6] * assign990_e1272);
        let assign990_e1274: f64 = (s.v[56] + assign990_e1273);
        (assign990_e1274,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign990_e1276;

        let (assign1000_e1290,) = {
    if (!s.b[488]) {
        let assign1000_e1283: f64 = (-s.v[279]);
        let assign1000_e1284: f64 = (assign1000_e1283).exp();
        let assign1000_e1285: f64 = (1.0 + assign1000_e1284);
        let assign1000_e1286: f64 = (assign1000_e1285).ln();
        let assign1000_e1287: f64 = (s.v[6] * assign1000_e1286);
        let assign1000_e1288: f64 = (0.05 + assign1000_e1287);
        (assign1000_e1288,)
    } else {
        (s.v[55],)
    }
};
        s.v[55] = assign1000_e1290;

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

        let assign1070_e1346: f64 = (1.0 / s.v[19]);
        s.v[67] = assign1070_e1346;

        s.store_powf_scaled_input(73, 65, p.p66, p.p67);

        let assign1090_e1354: f64 = (s.v[75] * s.v[67]);
        let assign1090_e1356: f64 = (assign1090_e1354).powf(s.v[76]);
        s.v[90] = assign1090_e1356;

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
            s.store_add_scaled_inputs_ad_rhs(50, 50, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]);
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
            s.store_add_scaled_inputs_ad_rhs(51, 51, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]);
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

        s.store_powf_scaled_input(275, 70, s.v[72], (-0.5));

        s.store_div_from_scalar(276, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(275), p.p35), s.ad_value(276), (p.p66 * (s.v[72] * s.v[72])), 0.0, 65);

        let assign1680_e1876: f64 = (1.0 / s.v[19]);
        s.v[67] = assign1680_e1876;

        let assign1690_e1879: f64 = (s.v[85] * s.v[86]);
        let assign1690_e1881: f64 = (-0.5);
        let assign1690_e1882: f64 = (assign1690_e1879).powf(assign1690_e1881);
        s.v[277] = assign1690_e1882;

        let assign1700_e1885: f64 = (1.0 / s.v[90]);
        s.v[278] = assign1700_e1885;

        let assign1710_e1888: f64 = (p.p37 * s.v[85]);
        let assign1710_e1890: f64 = (assign1710_e1888 * s.v[85]);
        let assign1710_e1892: f64 = (assign1710_e1890 * s.v[277]);
        let assign1710_e1894: f64 = (assign1710_e1892 * s.v[278]);
        let assign1710_e1896: f64 = (assign1710_e1894 * s.v[75]);
        let assign1710_e1898: f64 = (assign1710_e1896 * s.v[67]);
        let assign1710_e1900: f64 = (assign1710_e1898 * s.v[86]);
        let assign1710_e1902: f64 = (assign1710_e1900 * s.v[86]);
        s.v[83] = assign1710_e1902;

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

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
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

        s.store_add_scaled_inputs4_indices(249, 248, 1.0, 245, 1.0, 250, -1.0, 252, -1.0);

        s.store_add_scaled_inputs4_indices(262, 260, 1.0, 264, (-1.0), 249, 1.0, 251, -1.0);

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
            s.store_mul_scale_offset_rhs(265, 295, 245, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[506] = (((s.v[246] * s.v[8]) / s.v[48]) < p.p147);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if s.b[506] {
            s.store_ad_value(266, A::exp_div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0));
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
            s.store_mul_scale_offset_rhs(268, 295, 249, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[508] = ((s.v[248] * s.v[8]) < p.p147);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if (!s.b[508]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[509] = ((s.v[261] * s.v[8]) < p.p147);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_exp_scaled_input(269, 261, s.v[8]);
        }

        if (!s.b[509]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(269, 295, 261, s.v[8], (((-p.p147)) + (1.0)));
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
            s.store_mul_scale_offset_rhs(257, 295, 254, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[512] = ((s.v[255] * s.v[8]) < p.p147);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if (!s.b[512]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[513] = (((s.v[261] - s.v[16]) * s.v[8]) < p.p147);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_exp_scaled_input_ad(272, A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[513]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(272, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[514] = (((s.v[249] - s.v[16]) * s.v[8]) < p.p147);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if s.b[514] {
            s.store_exp_scaled_input_ad(270, A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[514]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(270, 295, A::sub_scaled_inputs(s.ad_value(249), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[515] = (((s.v[245] - s.v[16]) * s.v[8]) < p.p147);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if s.b[515] {
            s.store_exp_scaled_input_ad(271, A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[515]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(271, 295, A::sub_scaled_inputs(s.ad_value(245), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[516] = (((s.v[244] - s.v[16]) * s.v[8]) < p.p147);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_exp_scaled_input_ad(273, A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[516]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(273, 295, A::sub_scaled_inputs(s.ad_value(244), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(111, 271, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(112, 273, 4.0, 1.0);

        s.store_div_scaled_value_offset_denominator(113, s.ad_value(273), 2.0, s.ad_value(112), 1.0, 1.0);

        s.b[517] = (s.v[113] < p.p149);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if s.b[517] {
            s.store_scalar(113, p.p149);
        }

        s.store_add_scaled_inputs3_mixed_iia(114, 111, s.v[6], 112, ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(111), 1.0, 1.0, A::offset(s.ad_value(112), 1.0), 1.0)), (-s.v[6]));

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
            s.store_add_scaled_inputs3_mixed_iai(116, 16, 1.0, A::ln(A::scale_offset(s.ad_value(115), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), 297, -1.0);
            s.store_scale(292, 16, 0.2);
            s.store_square(281, 292);
            s.store_square(282, 116);
        }

        s.b[520] = (s.v[116] < 0.0);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[520]) {
            s.store_div_scaled_inputs_mixed_ia(117, 281, 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)), 1.0);
        }

        if (s.b[518] && (!s.b[520])) {
            s.store_scaled_add_ad_lhs(117, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 116, 0.5);
        }

        if s.b[518] {
            s.store_div_scaled_product_offset_rhs(118, s.ad_value(117), s.ad_value(117), (p.p62 * p.p61), 1.0, A::scaled_offset(s.ad_value(117), (p.p62 * s.v[31]), p.p61), 1.0);
            s.store_div(285, 115, 118);
            s.store_scaled_offset(279, 285, (-1.0), 1.0 / (p.p63));
        }

        s.b[521] = (s.v[285] < 1.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[521]) {
            s.store_offset_scaled_ad(283, A::ln_one_plus_exp(s.ad_value(279)), p.p63, 1.0);
        }

        if (s.b[518] && (!s.b[521])) {
            s.store_add_scaled_inputs_ad_rhs(283, 285, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p63);
        }

        if s.b[518] {
            s.store_scale(119, 283, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
            s.store_scale(120, 117, 1.0 / ((p.p62 * p.p61)));
            s.store_div_scaled_offset_numerator(121, A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(119), s.ad_value(120), A::offset(s.ad_value(120), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(119), 2.0, A::offset(s.ad_value(120), 1.0)), 1.0);
            s.store_div_ad(122, A::add_scaled_sub_value_product(1.0, s.ad_value(121), 1.0, s.ad_value(113), s.ad_value(121), 1.0), A::offset(A::mul(s.ad_value(113), s.ad_value(121)), 1.0));
            s.store_scaled_mul(124, 115, 122, ((0.5 * s.v[31]) * s.v[8]));
            s.store_add_scaled_offset_product_rhs_mixed_iia(286, 124, 2.0, 113, A::add(s.ad_value(113), s.ad_value(124)), 1.0, 1.0);
            s.store_scaled_offset(125, 124, (-1.0), 0.5);
            s.store_add_ad_lhs(280, A::square(s.ad_value(125)), 286);
        }

        s.b[522] = (s.v[124] >= 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[522]) {
            s.store_add_ad_rhs(126, 125, A::sqrt(s.ad_value(280)));
        }

        if (s.b[518] && (!s.b[522])) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(126, 286, A::sqrt(s.ad_value(280)), 1.0, 125, -1.0);
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
            s.store_div_scaled_value_offset_denominator(134, s.ad_value(115), p.p62, s.ad_value(115), p.p62, 1.0);
            s.store_div_from_scalar_offset_input(210, p.p62, 115, p.p62);
        }

        if (!s.b[518]) {
            s.store_scalar(118, 0.0);
            s.store_div_scaled_value_offset_denominator(126, s.ad_value(271), 2.0, s.ad_value(111), 1.0, 1.0);
            s.copy_ad(128, 265);
        }

        s.b[525] = ((((s.v[250]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[114]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[111] + s.v[112]))));
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((!s.b[518]) && s.b[525]) {
            s.store_scaled_add(135, 126, 113, 0.5);
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(135), 1.0, s.ad_value(135), 1.0, 1.0);
        }

        if ((!s.b[518]) && (!s.b[525])) {
            s.store_div_ad_rhs(122, 114, A::add_scaled_inputs3(s.ad_value(114), 1.0, s.ad_value(245), 1.0, s.ad_value(244), -1.0));
        }

        if (!s.b[518]) {
            s.copy_ad(132, 250);
            s.store_scale(133, 17, 0.1);
            s.copy_ad(134, 115);
            s.store_sub_from_scalar_scaled_input(210, 1.0, 134, 1.0 / (p.p62));
        }

        s.store_scale(136, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(293, 14, 0.1);

        s.store_div_scaled_inputs2_indices(279, 246, 1.0, 136, (-1.0), 293, 1.0);

        s.b[526] = (s.v[246] < s.v[136]);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_add_scaled_product_right_ad(137, 246, 1.0, 293, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[526]) {
            s.store_add_scaled_product_right_ad(137, 136, 1.0, 293, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (1.0 - p.p67));

        s.store_add_scaled_inputs3_mixed_aii(138, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p67))), 1.0, 246, 3.0, 137, (-3.0));

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

        s.store_div_scaled_inputs2_indices(279, 139, 1.0, 141, (-1.0), 133, 1.0);

        s.b[529] = (s.v[139] < s.v[141]);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_add_scaled_product_right_ad(142, 139, 1.0, 133, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[529]) {
            s.store_add_scaled_product_right_ad(142, 141, 1.0, 133, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf(143, 210, p.p76);

        s.store_add_ad(144, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(143), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(142), s.ad_value(17))), (1.0 - p.p72))), 1.0 / ((1.0 - p.p72))), A::mul3(s.ad_value(143), s.ad_value(140), A::sub(s.ad_value(139), s.ad_value(142))));

        s.store_add_scaled_product_value_ad(145, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(144)), 1.0, 25, 244, 1.0);

        s.store_scale(146, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(147, 146, 266);

        s.store_div_scaled_value_offset_denominator(149, s.ad_value(147), 1.0, A::sqrt(A::offset(s.ad_value(147), 1.0)), 1.0, 1.0);

        s.store_pow_ad(129, s.ad_value(128), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(148, 146, 129);

        s.store_div_scaled_value_offset_denominator(150, s.ad_value(148), 1.0, A::sqrt(A::offset(s.ad_value(148), 1.0)), 1.0, 1.0);

        s.b[530] = (p.p92 == 0.0);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if s.b[530] {
            s.store_add_ad(151, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));
        }

        if (!s.b[530]) {
            s.store_offset_scaled_div(289, 138, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_div_scaled_inputs_indices(290, 145, (-(s.v[99] * s.v[8])), 40, 1.0);
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

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul(155, 35, 266);

        s.store_div_scaled_inputs2_indices(156, 155, 1.0, 154, (-1.0), 153, 1.0);

        s.store_scale(279, 246, 10000.0);

        s.b[532] = (s.v[246] < 0.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if s.b[532] {
            s.store_scaled_ln_one_plus_exp(296, 279, 0.0001);
        }

        if (!s.b[532]) {
            s.store_add_scaled_inputs_ad_rhs(296, 246, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.0001);
        }

        let assign3740_e3505: f64 = (s.v[296] / p.p152);
        s.v[298] = assign3740_e3505;

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
            s.store_mul_scale_offset_rhs(296, 295, 246, (s.v[8] * 1.0 / (p.p17)), (((-p.p147)) + (1.0)));
        }

        s.b[536] = (p.p24 == 1.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (((s.v[246] - s.v[55]) * s.v[8]) < p.p147);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        let (assign3910_e3636,) = {
    if (s.b[536] && s.b[537]) {
        let assign3910_e3631: f64 = (s.v[246] - s.v[55]);
        let assign3910_e3633: f64 = (assign3910_e3631 * s.v[8]);
        let assign3910_e3634: f64 = (assign3910_e3633).exp();
        (assign3910_e3634,)
    } else {
        (s.v[298],)
    }
};
        s.v[298] = assign3910_e3636;

        if (s.b[536] && (!s.b[537])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        let (assign3930_e3661,) = {
    if (s.b[536] && (!s.b[537])) {
        let assign3930_e3653: f64 = (s.v[246] - s.v[55]);
        let assign3930_e3655: f64 = (assign3930_e3653 * s.v[8]);
        let assign3930_e3657: f64 = (assign3930_e3655 - p.p147);
        let assign3930_e3658: f64 = (1.0 + assign3930_e3657);
        let assign3930_e3659: f64 = (s.v[295] * assign3930_e3658);
        (assign3930_e3659,)
    } else {
        (s.v[298],)
    }
};
        s.v[298] = assign3930_e3661;

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
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p19)), (((-p.p147)) + (1.0)));
        }

        s.b[541] = (p.p24 == 1.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

        s.b[542] = (((s.v[247] - s.v[55]) * s.v[8]) < p.p147);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        let (assign4080_e3849,) = {
    if (s.b[541] && s.b[542]) {
        let assign4080_e3844: f64 = (s.v[247] - s.v[55]);
        let assign4080_e3846: f64 = (assign4080_e3844 * s.v[8]);
        let assign4080_e3847: f64 = (assign4080_e3846).exp();
        (assign4080_e3847,)
    } else {
        (s.v[298],)
    }
};
        s.v[298] = assign4080_e3849;

        if (s.b[541] && (!s.b[542])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        let (assign4100_e3874,) = {
    if (s.b[541] && (!s.b[542])) {
        let assign4100_e3866: f64 = (s.v[247] - s.v[55]);
        let assign4100_e3868: f64 = (assign4100_e3866 * s.v[8]);
        let assign4100_e3870: f64 = (assign4100_e3868 - p.p147);
        let assign4100_e3871: f64 = (1.0 + assign4100_e3870);
        let assign4100_e3872: f64 = (s.v[295] * assign4100_e3871);
        (assign4100_e3872,)
    } else {
        (s.v[298],)
    }
};
        s.v[298] = assign4100_e3874;

        s.b[543] = (((s.v[246] * s.v[8]) / p.p21) < p.p147);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_exp_scaled_input(296, 246, (s.v[8] * 1.0 / (p.p21)));
        }

        if (!s.b[543]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 246, (s.v[8] * 1.0 / (p.p21)), (((-p.p147)) + (1.0)));
        }

        s.b[544] = (((s.v[247] * s.v[8]) / p.p23) < p.p147);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p23)));
        }

        if (!s.b[544]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p23)), (((-p.p147)) + (1.0)));
        }

        s.b[545] = (((s.v[249] * s.v[8]) / p.p32) < p.p147);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_exp_scaled_input(296, 249, (s.v[8] * 1.0 / (p.p32)));
        }

        if (!s.b[545]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 249, (s.v[8] * 1.0 / (p.p32)), (((-p.p147)) + (1.0)));
        }

        s.b[546] = (((s.v[247] * s.v[8]) / p.p146) < p.p147);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if s.b[546] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p146)));
        }

        if (!s.b[546]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p146)), (((-p.p147)) + (1.0)));
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
            s.store_scaled_mul_ad(60, A::powf(A::sqrt_square_offset(s.ad_value(275), 1e-30), ((-2.0) - p.p67)), A::sub(A::scale_offset(A::scale(s.ad_value(275), (3.0 * (p.p67 - 1.0))), (-p.p67), (((1.0 - (p.p67 * p.p67))) * (p.p67))), A::mul3_scaled_output(s.ad_value(275), s.ad_value(275), A::offset(s.ad_value(275), (p.p67 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(275, s.ad_value(246), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0);
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

        let (assign4520_e4332,) = {
    if s.b[551] {
        let assign4520_e4325: f64 = (s.v[244] * s.v[67]);
        let assign4520_e4326: f64 = (1.0 - assign4520_e4325);
        let assign4520_e4329: f64 = (1.0 - s.v[76]);
        let assign4520_e4330: f64 = (assign4520_e4326).powf(assign4520_e4329);
        (assign4520_e4330,)
    } else {
        (s.v[77],)
    }
};
        s.v[77] = assign4520_e4332;

        s.b[552] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p147);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[551] && (!s.b[552])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        let (assign4570_e4393,) = {
    if s.b[551] {
        let assign4570_e4391: f64 = (s.v[244] * s.v[67]);
        (assign4570_e4391,)
    } else {
        (s.v[277],)
    }
};
        s.v[277] = assign4570_e4393;

        let (assign4580_e4437,) = {
    if s.b[551] {
        let assign4580_e4397: f64 = (s.v[277] * s.v[277]);
        let assign4580_e4399: f64 = (assign4580_e4397 + 1e-30);
        let assign4580_e4400: f64 = (assign4580_e4399).sqrt();
        let assign4580_e4402: f64 = (-2.0);
        let assign4580_e4404: f64 = (assign4580_e4402 - s.v[76]);
        let assign4580_e4405: f64 = (assign4580_e4400).powf(assign4580_e4404);
        let assign4580_e4410: f64 = (s.v[76] * s.v[76]);
        let assign4580_e4411: f64 = (1.0 - assign4580_e4410);
        let assign4580_e4414: f64 = (3.0 * s.v[277]);
        let assign4580_e4417: f64 = (s.v[76] - 1.0);
        let assign4580_e4418: f64 = (assign4580_e4414 * assign4580_e4417);
        let assign4580_e4419: f64 = (assign4580_e4411 - assign4580_e4418);
        let assign4580_e4420: f64 = (s.v[76] * assign4580_e4419);
        let assign4580_e4423: f64 = (6.0 * s.v[277]);
        let assign4580_e4425: f64 = (assign4580_e4423 * s.v[277]);
        let assign4580_e4428: f64 = (s.v[76] - 1.0);
        let assign4580_e4430: f64 = (assign4580_e4428 + s.v[277]);
        let assign4580_e4431: f64 = (assign4580_e4425 * assign4580_e4430);
        let assign4580_e4432: f64 = (assign4580_e4420 - assign4580_e4431);
        let assign4580_e4433: f64 = (assign4580_e4405 * assign4580_e4432);
        let assign4580_e4435: f64 = (assign4580_e4433 * 0.16666666666666666);
        (assign4580_e4435,)
    } else {
        (s.v[80],)
    }
};
        s.v[80] = assign4580_e4437;

        let (assign4590_e4449,) = {
    if s.b[551] {
        let assign4590_e4441: f64 = (s.v[244] * s.v[79]);
        let assign4590_e4443: f64 = (assign4590_e4441 * s.v[83]);
        let assign4590_e4446: f64 = (s.v[85] * s.v[80]);
        let assign4590_e4447: f64 = (assign4590_e4443 / assign4590_e4446);
        (assign4590_e4447,)
    } else {
        (s.v[277],)
    }
};
        s.v[277] = assign4590_e4449;

        s.b[553] = (s.v[277] < (-0.001));
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        s.b[554] = (s.v[277] < p.p147);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if ((s.b[551] && s.b[553]) && (!s.b[554])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.store_mul(165, 146, 268);

        s.store_scale(166, 270, 4.0);

        s.store_div_scaled_inputs2_mixed_iia(168, 165, 1.0, 146, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(167, s.ad_value(166), 1.0, A::sqrt(A::offset(s.ad_value(166), 1.0)), 1.0, 1.0);

        s.b[556] = ((p.p5 > 0.0) && (p.p33 > 0.0));
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_div_scaled_offset_numerator(171, s.ad_value(269), ((p.p33 * 2.0) * s.v[43]), ((-1.0) * ((p.p33 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);
        }

        s.b[557] = (p.p8 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[557]) {
            s.store_div_scaled_inputs2_mixed_iia(172, 269, ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), 257, (-((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::add_scaled_inputs(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), s.ad_value(257), (p.p144 * ((4.0 * s.v[104]) / s.v[106]))), 1.0)), 1.0), 1.0);
        }

        if (s.b[556] && (!s.b[557])) {
            s.store_div_scaled_offset_numerator(172, s.ad_value(269), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), ((-1.0) * ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), 1.0)), 1.0), 1.0);
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
            s.store_div_scaled_inputs_mixed_ia(174, 281, 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)), 1.0);
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
            s.store_mul_scale_offset_rhs(332, 295, 156, (-1.0 / (p.p42)), (((-p.p147)) + (1.0)));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_sub_from_scalar_lhs(333, p.p44, 244, 332);
        }

        s.b[569] = (((-s.v[334]) * ((s.v[333]) as f64).powf(p.p41)) < p.p147);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[569]) {
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 334, -1.0, A::powf(s.ad_value(333), p.p41));
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
            s.store_div_scaled_inputs2_indices(280, 16, 1.0, 244, (-1.0), 210, 1.0);
            s.store_sqrt_div_scaled_inputs(197, 280, 2.0, 196, 1.0);
        }

        s.b[572] = (p.p7 == 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[572]) {
            s.store_scalar(198, p.p45);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[572])) {
            s.store_sub_from_scalar_scaled_input(123, 1.0, 122, 0.5);
            s.store_scaled_mul(198, 123, 123, p.p45);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_product_sqrt_square_sum_denominator(199, 197, 198, 1.0, 197, 198, 1.0);
            s.store_div_scaled_inputs2_indices(200, 16, 1.0, 244, (-1.0), 199, 1.0);
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
            s.store_add_ad(280, A::square(A::sub(s.ad_value(205), s.ad_value(201))), A::mul3_scaled_output(s.ad_value(200), s.ad_value(200), s.ad_value(134), (0.1 * 1.0 / (p.p62))));
            s.store_add_scaled_inputs3_sqrt_third_indices(202, 205, 0.5, 201, 0.5, 280, 0.5);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_inputs2_indices(287, 202, 1.0, 200, (-1.0), 202, 1.0);
        }

        s.b[574] = (((s.v[287]) as f64).abs() > 1e-7);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[574]) {
            s.store_div_scaled_inputs_indices(206, 199, 0.5, 287, 1.0);
            s.store_mul_product3_mixed_aaii(207, A::sub(A::exp_div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::div(s.ad_value(198), s.ad_value(206)), 1.0))), A::div(s.ad_value(0), s.ad_value(98)), 202, 206, 1.0);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[574])) {
            s.store_mul_ad_product_rhs(207, 0, s.ad_value(198), A::exp_div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0));
        }

        s.b[575] = (p.p39 == 3.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (s.v[244] < p.p44);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_powf(211, A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(156), 1.0, s.ad_value(156), p.p48, 1.0)), p.p49), A::sub_from_scalar(p.p44, s.ad_value(244)), p.p41);
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
            s.store_add_scaled_inputs_ad_rhs(214, 213, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p51);
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_mul_powf_ad_rhs(212, 211, s.ad_value(214), p.p50);
        }

        s.b[579] = (((-s.v[334]) * s.v[212]) < p.p147);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[579]) {
            s.store_exp_mul_scaled_lhs_indices(337, 334, -1.0, 212);
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
            s.store_add_scaled_inputs3(208, A::div_from_scalar(s.v[6], A::mul(s.ad_value(156), A::add(s.ad_value(30), s.ad_value(186)))), 1.0, A::div(s.ad_value(153), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(186))), 1.0);
        }

        s.b[582] = (p.p39 == 3.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_scaled_sub(279, 207, 208, 1000000.0);
        }

        s.b[583] = (s.v[207] < s.v[208]);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && s.b[583]) {
            s.store_sub_scaled_inputs_ad_rhs(207, 207, 1.0, A::ln_one_plus_exp(s.ad_value(279)), 1e-6);
        }

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && (!s.b[583])) {
            s.store_sub_scaled_inputs_ad_rhs(207, 208, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 1e-6);
        }

        s.store_scaled_mul(215, 23, 138, (1.0 - p.p68));

        s.store_div_scaled_inputs2_indices(279, 247, 1.0, 136, (-1.0), 293, 1.0);

        s.b[585] = (s.v[247] < s.v[136]);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if s.b[585] {
            s.store_add_scaled_product_right_ad(216, 247, 1.0, 293, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[585]) {
            s.store_add_scaled_product_right_ad(216, 136, 1.0, 293, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(217, 23, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(65))), (1.0 - p.p67)), 1.0 / ((1.0 - p.p67))), p.p68, s.ad_value(247), ((3.0) * (p.p68)), s.ad_value(216), (((-3.0)) * (p.p68)), 0.0);

        s.store_scaled_mul(218, 24, 145, p.p77);

        s.v[219] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(223, 149, 184, (0.5 * s.v[219]));

        s.store_scaled_mul(224, 150, 184, (0.5 * s.v[219]));

        s.store_scale(294, 17, 0.1);

        s.store_div_scaled_inputs2_indices(279, 249, 1.0, 141, (-1.0), 294, 1.0);

        s.b[586] = (s.v[249] < s.v[141]);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if s.b[586] {
            s.store_add_scaled_product_right_ad(225, 249, 1.0, 294, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[586]) {
            s.store_add_scaled_product_right_ad(225, 141, 1.0, 294, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(226, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(225), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, 140, A::sub(s.ad_value(249), s.ad_value(225)), 1.0);

        s.store_mul_add_scaled_product_rhs(227, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(226)), ((1.0 - p.p77) * (1.0 - p.p33)), s.ad_value(25), s.ad_value(249), ((1.0 - p.p77) * (1.0 - p.p33)));

        s.store_div_scaled_inputs2_indices(279, 261, 1.0, 141, (-1.0), 294, 1.0);

        s.b[587] = (s.v[261] < s.v[141]);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if s.b[587] {
            s.store_add_scaled_product_right_ad(228, 261, 1.0, 294, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[587]) {
            s.store_add_scaled_product_right_ad(228, 141, 1.0, 294, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(229, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(228), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, 140, A::sub(s.ad_value(261), s.ad_value(228)), 1.0);

        s.store_mul_add_scaled_product_rhs(230, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(229)), ((1.0 - p.p77) * p.p33), s.ad_value(25), s.ad_value(261), ((1.0 - p.p77) * p.p33));

        s.store_scale(301, 102, 0.1);

        s.store_scale(231, 102, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_scaled_inputs2_indices(279, 253, 1.0, 231, (-1.0), 301, 1.0);

        s.b[588] = (s.v[253] < s.v[231]);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if s.b[588] {
            s.store_add_scaled_product_right_ad(232, 253, 1.0, 301, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[588]) {
            s.store_add_scaled_product_right_ad(232, 231, 1.0, 301, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(233, 103, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(102), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(102))), (1.0 - p.p139)), 1.0 / ((1.0 - p.p139))), 1.0, s.ad_value(253), 2.0, s.ad_value(232), (-2.0), 0.0);

        s.store_scaled_powf_ad(234, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p85), (s.v[93] * s.v[36]));

        s.b[589] = ((s.v[246] / (p.p85 * s.v[6])) < p.p147);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if s.b[589] {
            s.store_exp_scaled_input(296, 246, 1.0 / ((p.p85 * s.v[6])));
        }

        if (!s.b[589]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 246, 1.0 / ((p.p85 * s.v[6])), (((-p.p147)) + (1.0)));
        }

        s.store_mul(236, 234, 296);

        s.v[237] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_offset_ad_rhs(238, 122, (0.5 * s.v[237]), A::add(s.ad_value(126), s.ad_value(113)), 2.0);

        s.b[590] = (p.p79 == 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_add_scaled_inputs(243, 168, (s.v[219] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 167, (s.v[237] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[591] = ((((s.v[249] - s.v[22]) / p.p91) * s.v[8]) < p.p147);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((!s.b[590]) && s.b[591]) {
            s.store_exp_scaled_input_ad(177, A::sub(s.ad_value(249), s.ad_value(22)), (1.0 / (p.p91) * s.v[8]));
        }

        if ((!s.b[590]) && (!s.b[591])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(177, 295, A::sub_scaled_inputs(s.ad_value(249), (1.0 / (p.p91) * s.v[8]), s.ad_value(22), (1.0 / (p.p91) * s.v[8])), (((-p.p147)) + (1.0)));
        }

        if (!s.b[590]) {
            s.store_div_scaled_value_offset_denominator(243, s.ad_value(268), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(177), 4.0, 1.0)), 1.0, 1.0);
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
            s.store_div_scaled_inputs2_mixed_iia(170, 169, 1.0, 146, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0), 1.0);
            s.store_scale(239, 272, 4.0);
            s.store_div_scaled_value_offset_denominator(240, s.ad_value(239), 1.0, A::sqrt(A::offset(s.ad_value(239), 1.0)), 1.0, 1.0);
            s.store_add_scaled_inputs(241, 170, (s.v[219] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 240, (s.v[237] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[594] = (((s.v[261] - s.v[22]) * s.v[8]) < p.p147);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((s.b[592] && (!s.b[593])) && s.b[594]) {
            s.store_exp_scaled_input_ad(178, A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]);
        }

        if ((s.b[592] && (!s.b[593])) && (!s.b[594])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(178, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(22), s.v[8]), (((-p.p147)) + (1.0)));
        }

        if (s.b[592] && (!s.b[593])) {
            s.store_div_scaled_value_offset_denominator(241, s.ad_value(269), (((2.0 * p.p33) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(178), 4.0, 1.0)), 1.0, 1.0);
        }

        if s.b[592] {
            s.store_mul(242, 175, 241);
        }

        s.b[595] = (p.p6 == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_offset_powf_ad(190, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (-p.p67), (-3.0));
            s.store_div_scaled_inputs2_indices(288, 246, 1.0, 136, (-1.0), 293, 1.0);
        }

        s.b[596] = (s.v[288] < 0.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_div_from_scalar_offset_ad(191, 1.0, A::exp(s.ad_value(288)), 1.0);
        }

        if (s.b[595] && (!s.b[596])) {
            let assign6660_ad_e6874: A = A::exp_scaled_input(s.ad_value(288), -1.0);
            s.store_div_ad(191, assign6660_ad_e6874, A::offset(assign6660_ad_e6874, 1.0));
        }

        if s.b[595] {
            s.store_offset_mul(189, 190, 191, 3.0);
            s.store_scaled_mul(192, 23, 189, (1.0 - p.p68));
            s.store_mul_div_scaled_product_mixed_aiii(195, A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(147), 1.0))), 146, 266, s.v[8], 48, 1.0);
            s.store_scaled_mul(193, 184, 195, (0.5 * s.v[219]));
            s.store_scale(194, 236, 1.0 / ((p.p85 * s.v[6])));
            s.store_mul_add_scaled_inputs3_offset_rhs(222, 248, s.ad_value(192), 0.2, s.ad_value(193), 0.2, s.ad_value(194), 0.2, 0.0);
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

        s.store_div_scaled_inputs2_indices(327, 155, 1.0, 154, 1.0, 153, 1.0);

        s.b[601] = (s.v[327] > 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_div_scaled_inputs2_indices(329, 220, 1.0, 221, 1.0, 327, 1.0);
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
            s.store_add_scaled_inputs_ad(70, A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1);
        }

        s.store_scaled_offset(279, 88, (((-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))) + ((-0.05))), 10.0);

        s.b[482] = ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if s.b[482] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(279)), 0.1, 0.05);
        }

        if (!s.b[482]) {
            s.store_add_scaled_inputs_ad(85, A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1);
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

        s.store_powf_scaled_input(73, 65, p.p66, p.p67);

        s.store_powf_scaled_input(90, 67, s.v[75], s.v[76]);

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
            s.store_add_scaled_inputs_ad_rhs(50, 50, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]);
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
            s.store_add_scaled_inputs_ad_rhs(51, 51, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]);
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

        s.store_powf_scaled_input(275, 70, s.v[72], (-0.5));

        s.store_div_from_scalar(276, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(275), p.p35), s.ad_value(276), (p.p66 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_scaled_input(277, 85, s.v[86], (-0.5));

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

        s.store_add_scaled_inputs4_indices(249, 248, 1.0, 245, 1.0, 250, -1.0, 252, -1.0);

        s.store_add_scaled_inputs4_indices(262, 260, 1.0, 264, (-1.0), 249, 1.0, 251, -1.0);

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
            s.store_mul_scale_offset_rhs(265, 295, 245, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[506] = (((s.v[246] * s.v[8]) / s.v[48]) < p.p147);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if s.b[506] {
            s.store_ad_value(266, A::exp_div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0));
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
            s.store_mul_scale_offset_rhs(268, 295, 249, s.v[8], (((-p.p147)) + (1.0)));
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
            s.store_mul_scale_offset_rhs(269, 295, 261, s.v[8], (((-p.p147)) + (1.0)));
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
            s.store_mul_scale_offset_rhs(257, 295, 254, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[512] = ((s.v[255] * s.v[8]) < p.p147);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if (!s.b[512]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[513] = (((s.v[261] - s.v[16]) * s.v[8]) < p.p147);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_exp_scaled_input_ad(272, A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[513]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(272, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[514] = (((s.v[249] - s.v[16]) * s.v[8]) < p.p147);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if s.b[514] {
            s.store_exp_scaled_input_ad(270, A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[514]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(270, 295, A::sub_scaled_inputs(s.ad_value(249), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[515] = (((s.v[245] - s.v[16]) * s.v[8]) < p.p147);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if s.b[515] {
            s.store_exp_scaled_input_ad(271, A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[515]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(271, 295, A::sub_scaled_inputs(s.ad_value(245), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[516] = (((s.v[244] - s.v[16]) * s.v[8]) < p.p147);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_exp_scaled_input_ad(273, A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[516]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(273, 295, A::sub_scaled_inputs(s.ad_value(244), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(111, 271, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(112, 273, 4.0, 1.0);

        s.store_div_scaled_value_offset_denominator(113, s.ad_value(273), 2.0, s.ad_value(112), 1.0, 1.0);

        s.b[517] = (s.v[113] < p.p149);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if s.b[517] {
            s.store_scalar(113, p.p149);
        }

        s.store_add_scaled_inputs3_mixed_iia(114, 111, s.v[6], 112, ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(111), 1.0, 1.0, A::offset(s.ad_value(112), 1.0), 1.0)), (-s.v[6]));

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
            s.store_add_scaled_inputs3_mixed_iai(116, 16, 1.0, A::ln(A::scale_offset(s.ad_value(115), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), 297, -1.0);
            s.store_scale(292, 16, 0.2);
            s.store_square(281, 292);
            s.store_square(282, 116);
        }

        s.b[520] = (s.v[116] < 0.0);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[520]) {
            s.store_div_scaled_inputs_mixed_ia(117, 281, 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)), 1.0);
        }

        if (s.b[518] && (!s.b[520])) {
            s.store_scaled_add_ad_lhs(117, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 116, 0.5);
        }

        if s.b[518] {
            s.store_div_scaled_product_offset_rhs(118, s.ad_value(117), s.ad_value(117), (p.p62 * p.p61), 1.0, A::scaled_offset(s.ad_value(117), (p.p62 * s.v[31]), p.p61), 1.0);
            s.store_div(285, 115, 118);
            s.store_scaled_offset(279, 285, (-1.0), 1.0 / (p.p63));
        }

        s.b[521] = (s.v[285] < 1.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[521]) {
            s.store_offset_scaled_ad(283, A::ln_one_plus_exp(s.ad_value(279)), p.p63, 1.0);
        }

        if (s.b[518] && (!s.b[521])) {
            s.store_add_scaled_inputs_ad_rhs(283, 285, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p63);
        }

        if s.b[518] {
            s.store_scale(119, 283, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
            s.store_scale(120, 117, 1.0 / ((p.p62 * p.p61)));
            s.store_div_scaled_offset_numerator(121, A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(119), s.ad_value(120), A::offset(s.ad_value(120), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(119), 2.0, A::offset(s.ad_value(120), 1.0)), 1.0);
            s.store_div_ad(122, A::add_scaled_sub_value_product(1.0, s.ad_value(121), 1.0, s.ad_value(113), s.ad_value(121), 1.0), A::offset(A::mul(s.ad_value(113), s.ad_value(121)), 1.0));
            s.store_scaled_mul(124, 115, 122, ((0.5 * s.v[31]) * s.v[8]));
            s.store_add_scaled_offset_product_rhs_mixed_iia(286, 124, 2.0, 113, A::add(s.ad_value(113), s.ad_value(124)), 1.0, 1.0);
            s.store_scaled_offset(125, 124, (-1.0), 0.5);
            s.store_add_ad_lhs(280, A::square(s.ad_value(125)), 286);
        }

        s.b[522] = (s.v[124] >= 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[522]) {
            s.store_add_ad_rhs(126, 125, A::sqrt(s.ad_value(280)));
        }

        if (s.b[518] && (!s.b[522])) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(126, 286, A::sqrt(s.ad_value(280)), 1.0, 125, -1.0);
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
            s.store_div_scaled_value_offset_denominator(134, s.ad_value(115), p.p62, s.ad_value(115), p.p62, 1.0);
            s.store_div_from_scalar_offset_input(210, p.p62, 115, p.p62);
        }

        if (!s.b[518]) {
            s.store_scalar(118, 0.0);
            s.store_div_scaled_value_offset_denominator(126, s.ad_value(271), 2.0, s.ad_value(111), 1.0, 1.0);
            s.copy_ad(128, 265);
        }

        s.b[525] = ((((s.v[250]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[114]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[111] + s.v[112]))));
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((!s.b[518]) && s.b[525]) {
            s.store_scaled_add(135, 126, 113, 0.5);
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(135), 1.0, s.ad_value(135), 1.0, 1.0);
        }

        if ((!s.b[518]) && (!s.b[525])) {
            s.store_div_ad_rhs(122, 114, A::add_scaled_inputs3(s.ad_value(114), 1.0, s.ad_value(245), 1.0, s.ad_value(244), -1.0));
        }

        if (!s.b[518]) {
            s.copy_ad(132, 250);
            s.store_scale(133, 17, 0.1);
            s.copy_ad(134, 115);
            s.store_sub_from_scalar_scaled_input(210, 1.0, 134, 1.0 / (p.p62));
        }

        s.store_scale(136, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(293, 14, 0.1);

        s.store_div_scaled_inputs2_indices(279, 246, 1.0, 136, (-1.0), 293, 1.0);

        s.b[526] = (s.v[246] < s.v[136]);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_add_scaled_product_right_ad(137, 246, 1.0, 293, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[526]) {
            s.store_add_scaled_product_right_ad(137, 136, 1.0, 293, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (1.0 - p.p67));

        s.store_add_scaled_inputs3_mixed_aii(138, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p67))), 1.0, 246, 3.0, 137, (-3.0));

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

        s.store_div_scaled_inputs2_indices(279, 139, 1.0, 141, (-1.0), 133, 1.0);

        s.b[529] = (s.v[139] < s.v[141]);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_add_scaled_product_right_ad(142, 139, 1.0, 133, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[529]) {
            s.store_add_scaled_product_right_ad(142, 141, 1.0, 133, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_powf(143, 210, p.p76);

        s.store_add_ad(144, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(143), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(142), s.ad_value(17))), (1.0 - p.p72))), 1.0 / ((1.0 - p.p72))), A::mul3(s.ad_value(143), s.ad_value(140), A::sub(s.ad_value(139), s.ad_value(142))));

        s.store_add_scaled_product_value_ad(145, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(144)), 1.0, 25, 244, 1.0);

        s.store_scale(146, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(147, 146, 266);

        s.store_div_scaled_value_offset_denominator(149, s.ad_value(147), 1.0, A::sqrt(A::offset(s.ad_value(147), 1.0)), 1.0, 1.0);

        s.store_pow_ad(129, s.ad_value(128), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(148, 146, 129);

        s.store_div_scaled_value_offset_denominator(150, s.ad_value(148), 1.0, A::sqrt(A::offset(s.ad_value(148), 1.0)), 1.0, 1.0);

        s.b[530] = (p.p92 == 0.0);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if s.b[530] {
            s.store_add_ad(151, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));
        }

        if (!s.b[530]) {
            s.store_offset_scaled_div(289, 138, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_div_scaled_inputs_indices(290, 145, (-(s.v[99] * s.v[8])), 40, 1.0);
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

        s.store_div_scaled_inputs2_indices(156, 155, 1.0, 154, (-1.0), 153, 1.0);

        s.store_scale(279, 246, 10000.0);

        s.b[532] = (s.v[246] < 0.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if s.b[532] {
            s.store_scaled_ln_one_plus_exp(296, 279, 0.0001);
        }

        if (!s.b[532]) {
            s.store_add_scaled_inputs_ad_rhs(296, 246, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.0001);
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
            s.store_mul_scale_offset_rhs(296, 295, 246, (s.v[8] * 1.0 / (p.p17)), (((-p.p147)) + (1.0)));
        }

        s.b[536] = (p.p24 == 1.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (((s.v[246] - s.v[55]) * s.v[8]) < p.p147);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (s.b[536] && s.b[537]) {
            s.store_exp_scaled_input_ad(298, A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]);
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
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p19)), (((-p.p147)) + (1.0)));
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
            s.store_exp_scaled_input_ad(298, A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]);
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
            s.store_mul_scale_offset_rhs(296, 295, 246, (s.v[8] * 1.0 / (p.p21)), (((-p.p147)) + (1.0)));
        }

        s.b[544] = (((s.v[247] * s.v[8]) / p.p23) < p.p147);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p23)));
        }

        if (!s.b[544]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p23)), (((-p.p147)) + (1.0)));
        }

        s.b[545] = (((s.v[249] * s.v[8]) / p.p32) < p.p147);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_exp_scaled_input(296, 249, (s.v[8] * 1.0 / (p.p32)));
        }

        if (!s.b[545]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 249, (s.v[8] * 1.0 / (p.p32)), (((-p.p147)) + (1.0)));
        }

        s.b[546] = (((s.v[247] * s.v[8]) / p.p146) < p.p147);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if s.b[546] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p146)));
        }

        if (!s.b[546]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p146)), (((-p.p147)) + (1.0)));
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
            s.store_scaled_mul_ad(60, A::powf(A::sqrt_square_offset(s.ad_value(275), 1e-30), ((-2.0) - p.p67)), A::sub(A::scale_offset(A::scale(s.ad_value(275), (3.0 * (p.p67 - 1.0))), (-p.p67), (((1.0 - (p.p67 * p.p67))) * (p.p67))), A::mul3_scaled_output(s.ad_value(275), s.ad_value(275), A::offset(s.ad_value(275), (p.p67 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(275, s.ad_value(246), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0);
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
            s.store_scaled_mul_ad(80, A::powf(A::sqrt_square_offset(s.ad_value(277), 1e-30), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(277), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(277), s.ad_value(277), A::offset(s.ad_value(277), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(277, s.ad_value(244), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0);
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

        s.store_div_scaled_inputs2_mixed_iia(168, 165, 1.0, 146, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(167, s.ad_value(166), 1.0, A::sqrt(A::offset(s.ad_value(166), 1.0)), 1.0, 1.0);

        s.b[556] = ((p.p5 > 0.0) && (p.p33 > 0.0));
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_div_scaled_offset_numerator(171, s.ad_value(269), ((p.p33 * 2.0) * s.v[43]), ((-1.0) * ((p.p33 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);
        }

        s.b[557] = (p.p8 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[557]) {
            s.store_div_scaled_inputs2_mixed_iia(172, 269, ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), 257, (-((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::add_scaled_inputs(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), s.ad_value(257), (p.p144 * ((4.0 * s.v[104]) / s.v[106]))), 1.0)), 1.0), 1.0);
        }

        if (s.b[556] && (!s.b[557])) {
            s.store_div_scaled_offset_numerator(172, s.ad_value(269), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), ((-1.0) * ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), 1.0)), 1.0), 1.0);
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
            s.store_div_scaled_inputs_mixed_ia(174, 281, 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)), 1.0);
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
            s.store_mul_scale_offset_rhs(332, 295, 156, (-1.0 / (p.p42)), (((-p.p147)) + (1.0)));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_sub_from_scalar_lhs(333, p.p44, 244, 332);
        }

        s.b[569] = (((-s.v[334]) * ((s.v[333]) as f64).powf(p.p41)) < p.p147);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[569]) {
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 334, -1.0, A::powf(s.ad_value(333), p.p41));
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
            s.store_div_scaled_inputs2_indices(280, 16, 1.0, 244, (-1.0), 210, 1.0);
            s.store_sqrt_div_scaled_inputs(197, 280, 2.0, 196, 1.0);
        }

        s.b[572] = (p.p7 == 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[572]) {
            s.store_scalar(198, p.p45);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[572])) {
            s.store_sub_from_scalar_scaled_input(123, 1.0, 122, 0.5);
            s.store_scaled_mul(198, 123, 123, p.p45);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_product_sqrt_square_sum_denominator(199, 197, 198, 1.0, 197, 198, 1.0);
            s.store_div_scaled_inputs2_indices(200, 16, 1.0, 244, (-1.0), 199, 1.0);
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
            s.store_add_ad(280, A::square(A::sub(s.ad_value(205), s.ad_value(201))), A::mul3_scaled_output(s.ad_value(200), s.ad_value(200), s.ad_value(134), (0.1 * 1.0 / (p.p62))));
            s.store_add_scaled_inputs3_sqrt_third_indices(202, 205, 0.5, 201, 0.5, 280, 0.5);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_inputs2_indices(287, 202, 1.0, 200, (-1.0), 202, 1.0);
        }

        s.b[574] = (((s.v[287]) as f64).abs() > 1e-7);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[574]) {
            s.store_div_scaled_inputs_indices(206, 199, 0.5, 287, 1.0);
            s.store_mul_product3_mixed_aaii(207, A::sub(A::exp_div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::div(s.ad_value(198), s.ad_value(206)), 1.0))), A::div(s.ad_value(0), s.ad_value(98)), 202, 206, 1.0);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[574])) {
            s.store_mul_ad_product_rhs(207, 0, s.ad_value(198), A::exp_div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0));
        }

        s.b[575] = (p.p39 == 3.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (s.v[244] < p.p44);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_powf(211, A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(156), 1.0, s.ad_value(156), p.p48, 1.0)), p.p49), A::sub_from_scalar(p.p44, s.ad_value(244)), p.p41);
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
            s.store_add_scaled_inputs_ad_rhs(214, 213, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p51);
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_mul_powf_ad_rhs(212, 211, s.ad_value(214), p.p50);
        }

        s.b[579] = (((-s.v[334]) * s.v[212]) < p.p147);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[579]) {
            s.store_exp_mul_scaled_lhs_indices(337, 334, -1.0, 212);
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
            s.store_add_scaled_inputs3(208, A::div_from_scalar(s.v[6], A::mul(s.ad_value(156), A::add(s.ad_value(30), s.ad_value(186)))), 1.0, A::div(s.ad_value(153), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(186))), 1.0);
        }

        s.b[582] = (p.p39 == 3.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_scaled_sub(279, 207, 208, 1000000.0);
        }

        s.b[583] = (s.v[207] < s.v[208]);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && s.b[583]) {
            s.store_sub_scaled_inputs_ad_rhs(207, 207, 1.0, A::ln_one_plus_exp(s.ad_value(279)), 1e-6);
        }

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && (!s.b[583])) {
            s.store_sub_scaled_inputs_ad_rhs(207, 208, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 1e-6);
        }

        s.store_scaled_mul(215, 23, 138, (1.0 - p.p68));

        s.store_div_scaled_inputs2_indices(279, 247, 1.0, 136, (-1.0), 293, 1.0);

        s.b[585] = (s.v[247] < s.v[136]);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if s.b[585] {
            s.store_add_scaled_product_right_ad(216, 247, 1.0, 293, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[585]) {
            s.store_add_scaled_product_right_ad(216, 136, 1.0, 293, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(217, 23, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(65))), (1.0 - p.p67)), 1.0 / ((1.0 - p.p67))), p.p68, s.ad_value(247), ((3.0) * (p.p68)), s.ad_value(216), (((-3.0)) * (p.p68)), 0.0);

        s.store_scaled_mul(218, 24, 145, p.p77);

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[219] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(223, 149, 184, (0.5 * s.v[219]));

        s.store_scaled_mul(224, 150, 184, (0.5 * s.v[219]));

        s.store_scale(294, 17, 0.1);

        s.store_div_scaled_inputs2_indices(279, 249, 1.0, 141, (-1.0), 294, 1.0);

        s.b[586] = (s.v[249] < s.v[141]);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if s.b[586] {
            s.store_add_scaled_product_right_ad(225, 249, 1.0, 294, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[586]) {
            s.store_add_scaled_product_right_ad(225, 141, 1.0, 294, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(226, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(225), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, 140, A::sub(s.ad_value(249), s.ad_value(225)), 1.0);

        s.store_mul_add_scaled_product_rhs(227, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(226)), ((1.0 - p.p77) * (1.0 - p.p33)), s.ad_value(25), s.ad_value(249), ((1.0 - p.p77) * (1.0 - p.p33)));

        s.store_div_scaled_inputs2_indices(279, 261, 1.0, 141, (-1.0), 294, 1.0);

        s.b[587] = (s.v[261] < s.v[141]);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if s.b[587] {
            s.store_add_scaled_product_right_ad(228, 261, 1.0, 294, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[587]) {
            s.store_add_scaled_product_right_ad(228, 141, 1.0, 294, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(229, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(228), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, 140, A::sub(s.ad_value(261), s.ad_value(228)), 1.0);

        s.store_mul_add_scaled_product_rhs(230, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(229)), ((1.0 - p.p77) * p.p33), s.ad_value(25), s.ad_value(261), ((1.0 - p.p77) * p.p33));

        s.store_scale(301, 102, 0.1);

        s.store_scale(231, 102, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_scaled_inputs2_indices(279, 253, 1.0, 231, (-1.0), 301, 1.0);

        s.b[588] = (s.v[253] < s.v[231]);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if s.b[588] {
            s.store_add_scaled_product_right_ad(232, 253, 1.0, 301, A::ln_one_plus_exp(s.ad_value(279)), (-1.0));
        }

        if (!s.b[588]) {
            s.store_add_scaled_product_right_ad(232, 231, 1.0, 301, A::ln_one_plus_exp(A::neg(s.ad_value(279))), (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(233, 103, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(102), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(102))), (1.0 - p.p139)), 1.0 / ((1.0 - p.p139))), 1.0, s.ad_value(253), 2.0, s.ad_value(232), (-2.0), 0.0);

        s.store_scaled_powf_ad(234, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p85), (s.v[93] * s.v[36]));

        s.b[589] = ((s.v[246] / (p.p85 * s.v[6])) < p.p147);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if s.b[589] {
            s.store_exp_scaled_input(296, 246, 1.0 / ((p.p85 * s.v[6])));
        }

        if (!s.b[589]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 246, 1.0 / ((p.p85 * s.v[6])), (((-p.p147)) + (1.0)));
        }

        s.store_mul(236, 234, 296);

        s.v[237] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_offset_ad_rhs(238, 122, (0.5 * s.v[237]), A::add(s.ad_value(126), s.ad_value(113)), 2.0);

        s.b[590] = (p.p79 == 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_add_scaled_inputs(243, 168, (s.v[219] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 167, (s.v[237] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[591] = ((((s.v[249] - s.v[22]) / p.p91) * s.v[8]) < p.p147);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((!s.b[590]) && s.b[591]) {
            s.store_exp_scaled_input_ad(177, A::sub(s.ad_value(249), s.ad_value(22)), (1.0 / (p.p91) * s.v[8]));
        }

        if ((!s.b[590]) && (!s.b[591])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(177, 295, A::sub_scaled_inputs(s.ad_value(249), (1.0 / (p.p91) * s.v[8]), s.ad_value(22), (1.0 / (p.p91) * s.v[8])), (((-p.p147)) + (1.0)));
        }

        if (!s.b[590]) {
            s.store_div_scaled_value_offset_denominator(243, s.ad_value(268), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(177), 4.0, 1.0)), 1.0, 1.0);
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
            s.store_div_scaled_inputs2_mixed_iia(170, 169, 1.0, 146, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0), 1.0);
            s.store_scale(239, 272, 4.0);
            s.store_div_scaled_value_offset_denominator(240, s.ad_value(239), 1.0, A::sqrt(A::offset(s.ad_value(239), 1.0)), 1.0, 1.0);
            s.store_add_scaled_inputs(241, 170, (s.v[219] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 240, (s.v[237] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[594] = (((s.v[261] - s.v[22]) * s.v[8]) < p.p147);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((s.b[592] && (!s.b[593])) && s.b[594]) {
            s.store_exp_scaled_input_ad(178, A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]);
        }

        if ((s.b[592] && (!s.b[593])) && (!s.b[594])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(178, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(22), s.v[8]), (((-p.p147)) + (1.0)));
        }

        if (s.b[592] && (!s.b[593])) {
            s.store_div_scaled_value_offset_denominator(241, s.ad_value(269), (((2.0 * p.p33) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(178), 4.0, 1.0)), 1.0, 1.0);
        }

        if s.b[592] {
            s.store_mul(242, 175, 241);
        }

        s.b[595] = (p.p6 == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_offset_powf_ad(190, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (-p.p67), (-3.0));
            s.store_div_scaled_inputs2_indices(288, 246, 1.0, 136, (-1.0), 293, 1.0);
        }

        s.b[596] = (s.v[288] < 0.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_div_from_scalar_offset_ad(191, 1.0, A::exp(s.ad_value(288)), 1.0);
        }

        if (s.b[595] && (!s.b[596])) {
            let assign6660_ad_e6874: A = A::exp_scaled_input(s.ad_value(288), -1.0);
            s.store_div_ad(191, assign6660_ad_e6874, A::offset(assign6660_ad_e6874, 1.0));
        }

        if s.b[595] {
            s.store_offset_mul(189, 190, 191, 3.0);
            s.store_scaled_mul(192, 23, 189, (1.0 - p.p68));
            s.store_mul_div_scaled_product_mixed_aiii(195, A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(147), 1.0))), 146, 266, s.v[8], 48, 1.0);
            s.store_scaled_mul(193, 184, 195, (0.5 * s.v[219]));
            s.store_scale(194, 236, 1.0 / ((p.p85 * s.v[6])));
            s.store_mul_add_scaled_inputs3_offset_rhs(222, 248, s.ad_value(192), 0.2, s.ad_value(193), 0.2, s.ad_value(194), 0.2, 0.0);
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

        s.store_div_scaled_inputs2_indices(327, 155, 1.0, 154, 1.0, 153, 1.0);

        s.b[601] = (s.v[327] > 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_div_scaled_inputs2_indices(329, 220, 1.0, 221, 1.0, 327, 1.0);
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
        let eq14_e272: f64 = (eq14_e270 * p.p1);
        let eq14_e272_d_n0: f64 = ((eq14_e269_d_n0 * ddt_scale) * p.p1);
        let eq14_e272_d_n1: f64 = ((eq14_e269_d_n1 * ddt_scale) * p.p1);
        let eq14_e272_d_n2: f64 = ((eq14_e269_d_n2 * ddt_scale) * p.p1);
        let eq14_e272_d_n3: f64 = ((eq14_e269_d_n3 * ddt_scale) * p.p1);
        let eq14_e272_d_n4: f64 = ((eq14_e269_d_n4 * ddt_scale) * p.p1);
        let eq14_e272_d_n5: f64 = ((eq14_e269_d_n5 * ddt_scale) * p.p1);
        let eq14_e272_d_n6: f64 = ((eq14_e269_d_n6 * ddt_scale) * p.p1);
        let eq14_e272_d_n7: f64 = ((eq14_e269_d_n7 * ddt_scale) * p.p1);
        let eq14_e272_d_n8: f64 = ((eq14_e269_d_n8 * ddt_scale) * p.p1);
        let eq14_e272_d_n9: f64 = ((eq14_e269_d_n9 * ddt_scale) * p.p1);
        let eq14_e272_d_n10: f64 = ((eq14_e269_d_n10 * ddt_scale) * p.p1);
        let eq14_e272_d_n11: f64 = ((eq14_e269_d_n11 * ddt_scale) * p.p1);
        let eq14_e272_d_b0: f64 = ((eq14_e269_d_b0 * ddt_scale) * p.p1);
        let eq14_e272_d_b1: f64 = ((eq14_e269_d_b1 * ddt_scale) * p.p1);
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
        let eq15_e278: f64 = (eq15_e276 * p.p1);
        let eq15_e278_d_n0: f64 = ((eq15_e275_d_n0 * ddt_scale) * p.p1);
        let eq15_e278_d_n1: f64 = ((eq15_e275_d_n1 * ddt_scale) * p.p1);
        let eq15_e278_d_n2: f64 = ((eq15_e275_d_n2 * ddt_scale) * p.p1);
        let eq15_e278_d_n3: f64 = ((eq15_e275_d_n3 * ddt_scale) * p.p1);
        let eq15_e278_d_n4: f64 = ((eq15_e275_d_n4 * ddt_scale) * p.p1);
        let eq15_e278_d_n5: f64 = ((eq15_e275_d_n5 * ddt_scale) * p.p1);
        let eq15_e278_d_n6: f64 = ((eq15_e275_d_n6 * ddt_scale) * p.p1);
        let eq15_e278_d_n7: f64 = ((eq15_e275_d_n7 * ddt_scale) * p.p1);
        let eq15_e278_d_n8: f64 = ((eq15_e275_d_n8 * ddt_scale) * p.p1);
        let eq15_e278_d_n9: f64 = ((eq15_e275_d_n9 * ddt_scale) * p.p1);
        let eq15_e278_d_n10: f64 = ((eq15_e275_d_n10 * ddt_scale) * p.p1);
        let eq15_e278_d_n11: f64 = ((eq15_e275_d_n11 * ddt_scale) * p.p1);
        let eq15_e278_d_b0: f64 = ((eq15_e275_d_b0 * ddt_scale) * p.p1);
        let eq15_e278_d_b1: f64 = ((eq15_e275_d_b1 * ddt_scale) * p.p1);
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
        let eq16_e288: f64 = (eq16_e286 * p.p1);
        let eq16_e288_d_n0: f64 = ((eq16_e285_d_n0 * ddt_scale) * p.p1);
        let eq16_e288_d_n1: f64 = ((eq16_e285_d_n1 * ddt_scale) * p.p1);
        let eq16_e288_d_n2: f64 = ((eq16_e285_d_n2 * ddt_scale) * p.p1);
        let eq16_e288_d_n3: f64 = ((eq16_e285_d_n3 * ddt_scale) * p.p1);
        let eq16_e288_d_n4: f64 = ((eq16_e285_d_n4 * ddt_scale) * p.p1);
        let eq16_e288_d_n5: f64 = ((eq16_e285_d_n5 * ddt_scale) * p.p1);
        let eq16_e288_d_n6: f64 = ((eq16_e285_d_n6 * ddt_scale) * p.p1);
        let eq16_e288_d_n7: f64 = ((eq16_e285_d_n7 * ddt_scale) * p.p1);
        let eq16_e288_d_n8: f64 = ((eq16_e285_d_n8 * ddt_scale) * p.p1);
        let eq16_e288_d_n9: f64 = ((eq16_e285_d_n9 * ddt_scale) * p.p1);
        let eq16_e288_d_n10: f64 = ((eq16_e285_d_n10 * ddt_scale) * p.p1);
        let eq16_e288_d_n11: f64 = ((eq16_e285_d_n11 * ddt_scale) * p.p1);
        let eq16_e288_d_b0: f64 = ((eq16_e285_d_b0 * ddt_scale) * p.p1);
        let eq16_e288_d_b1: f64 = ((eq16_e285_d_b1 * ddt_scale) * p.p1);
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
        let eq17_e294: f64 = (eq17_e292 * p.p1);
        let eq17_e294_d_n0: f64 = ((eq17_e291_d_n0 * ddt_scale) * p.p1);
        let eq17_e294_d_n1: f64 = ((eq17_e291_d_n1 * ddt_scale) * p.p1);
        let eq17_e294_d_n2: f64 = ((eq17_e291_d_n2 * ddt_scale) * p.p1);
        let eq17_e294_d_n3: f64 = ((eq17_e291_d_n3 * ddt_scale) * p.p1);
        let eq17_e294_d_n4: f64 = ((eq17_e291_d_n4 * ddt_scale) * p.p1);
        let eq17_e294_d_n5: f64 = ((eq17_e291_d_n5 * ddt_scale) * p.p1);
        let eq17_e294_d_n6: f64 = ((eq17_e291_d_n6 * ddt_scale) * p.p1);
        let eq17_e294_d_n7: f64 = ((eq17_e291_d_n7 * ddt_scale) * p.p1);
        let eq17_e294_d_n8: f64 = ((eq17_e291_d_n8 * ddt_scale) * p.p1);
        let eq17_e294_d_n9: f64 = ((eq17_e291_d_n9 * ddt_scale) * p.p1);
        let eq17_e294_d_n10: f64 = ((eq17_e291_d_n10 * ddt_scale) * p.p1);
        let eq17_e294_d_n11: f64 = ((eq17_e291_d_n11 * ddt_scale) * p.p1);
        let eq17_e294_d_b0: f64 = ((eq17_e291_d_b0 * ddt_scale) * p.p1);
        let eq17_e294_d_b1: f64 = ((eq17_e291_d_b1 * ddt_scale) * p.p1);
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
        let eq18_e300: f64 = (eq18_e298 * p.p1);
        let eq18_e300_d_n0: f64 = ((eq18_e297_d_n0 * ddt_scale) * p.p1);
        let eq18_e300_d_n1: f64 = ((eq18_e297_d_n1 * ddt_scale) * p.p1);
        let eq18_e300_d_n2: f64 = ((eq18_e297_d_n2 * ddt_scale) * p.p1);
        let eq18_e300_d_n3: f64 = ((eq18_e297_d_n3 * ddt_scale) * p.p1);
        let eq18_e300_d_n4: f64 = ((eq18_e297_d_n4 * ddt_scale) * p.p1);
        let eq18_e300_d_n5: f64 = ((eq18_e297_d_n5 * ddt_scale) * p.p1);
        let eq18_e300_d_n6: f64 = ((eq18_e297_d_n6 * ddt_scale) * p.p1);
        let eq18_e300_d_n7: f64 = ((eq18_e297_d_n7 * ddt_scale) * p.p1);
        let eq18_e300_d_n8: f64 = ((eq18_e297_d_n8 * ddt_scale) * p.p1);
        let eq18_e300_d_n9: f64 = ((eq18_e297_d_n9 * ddt_scale) * p.p1);
        let eq18_e300_d_n10: f64 = ((eq18_e297_d_n10 * ddt_scale) * p.p1);
        let eq18_e300_d_n11: f64 = ((eq18_e297_d_n11 * ddt_scale) * p.p1);
        let eq18_e300_d_b0: f64 = ((eq18_e297_d_b0 * ddt_scale) * p.p1);
        let eq18_e300_d_b1: f64 = ((eq18_e297_d_b1 * ddt_scale) * p.p1);
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
        let eq19_e308: f64 = (eq19_e306 * p.p1);
        let eq19_e308_d_n0: f64 = ((eq19_e305_d_n0 * ddt_scale) * p.p1);
        let eq19_e308_d_n1: f64 = ((eq19_e305_d_n1 * ddt_scale) * p.p1);
        let eq19_e308_d_n2: f64 = ((eq19_e305_d_n2 * ddt_scale) * p.p1);
        let eq19_e308_d_n3: f64 = ((eq19_e305_d_n3 * ddt_scale) * p.p1);
        let eq19_e308_d_n4: f64 = ((eq19_e305_d_n4 * ddt_scale) * p.p1);
        let eq19_e308_d_n5: f64 = ((eq19_e305_d_n5 * ddt_scale) * p.p1);
        let eq19_e308_d_n6: f64 = ((eq19_e305_d_n6 * ddt_scale) * p.p1);
        let eq19_e308_d_n7: f64 = ((eq19_e305_d_n7 * ddt_scale) * p.p1);
        let eq19_e308_d_n8: f64 = ((eq19_e305_d_n8 * ddt_scale) * p.p1);
        let eq19_e308_d_n9: f64 = ((eq19_e305_d_n9 * ddt_scale) * p.p1);
        let eq19_e308_d_n10: f64 = ((eq19_e305_d_n10 * ddt_scale) * p.p1);
        let eq19_e308_d_n11: f64 = ((eq19_e305_d_n11 * ddt_scale) * p.p1);
        let eq19_e308_d_b0: f64 = ((eq19_e305_d_b0 * ddt_scale) * p.p1);
        let eq19_e308_d_b1: f64 = ((eq19_e305_d_b1 * ddt_scale) * p.p1);
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
        let eq20_e316: f64 = (eq20_e314 * p.p1);
        let eq20_e316_d_n0: f64 = ((eq20_e313_d_n0 * ddt_scale) * p.p1);
        let eq20_e316_d_n1: f64 = ((eq20_e313_d_n1 * ddt_scale) * p.p1);
        let eq20_e316_d_n2: f64 = ((eq20_e313_d_n2 * ddt_scale) * p.p1);
        let eq20_e316_d_n3: f64 = ((eq20_e313_d_n3 * ddt_scale) * p.p1);
        let eq20_e316_d_n4: f64 = ((eq20_e313_d_n4 * ddt_scale) * p.p1);
        let eq20_e316_d_n5: f64 = ((eq20_e313_d_n5 * ddt_scale) * p.p1);
        let eq20_e316_d_n6: f64 = ((eq20_e313_d_n6 * ddt_scale) * p.p1);
        let eq20_e316_d_n7: f64 = ((eq20_e313_d_n7 * ddt_scale) * p.p1);
        let eq20_e316_d_n8: f64 = ((eq20_e313_d_n8 * ddt_scale) * p.p1);
        let eq20_e316_d_n9: f64 = ((eq20_e313_d_n9 * ddt_scale) * p.p1);
        let eq20_e316_d_n10: f64 = ((eq20_e313_d_n10 * ddt_scale) * p.p1);
        let eq20_e316_d_n11: f64 = ((eq20_e313_d_n11 * ddt_scale) * p.p1);
        let eq20_e316_d_b0: f64 = ((eq20_e313_d_b0 * ddt_scale) * p.p1);
        let eq20_e316_d_b1: f64 = ((eq20_e313_d_b1 * ddt_scale) * p.p1);
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
        let eq23_e336: f64 = (eq23_e334 * p.p1);
        let eq23_e336_d_n0: f64 = ((eq23_e333_d_n0 * ddt_scale) * p.p1);
        let eq23_e336_d_n1: f64 = ((eq23_e333_d_n1 * ddt_scale) * p.p1);
        let eq23_e336_d_n2: f64 = ((eq23_e333_d_n2 * ddt_scale) * p.p1);
        let eq23_e336_d_n3: f64 = ((eq23_e333_d_n3 * ddt_scale) * p.p1);
        let eq23_e336_d_n4: f64 = ((eq23_e333_d_n4 * ddt_scale) * p.p1);
        let eq23_e336_d_n5: f64 = ((eq23_e333_d_n5 * ddt_scale) * p.p1);
        let eq23_e336_d_n6: f64 = ((eq23_e333_d_n6 * ddt_scale) * p.p1);
        let eq23_e336_d_n7: f64 = ((eq23_e333_d_n7 * ddt_scale) * p.p1);
        let eq23_e336_d_n8: f64 = ((eq23_e333_d_n8 * ddt_scale) * p.p1);
        let eq23_e336_d_n9: f64 = ((eq23_e333_d_n9 * ddt_scale) * p.p1);
        let eq23_e336_d_n10: f64 = ((eq23_e333_d_n10 * ddt_scale) * p.p1);
        let eq23_e336_d_n11: f64 = ((eq23_e333_d_n11 * ddt_scale) * p.p1);
        let eq23_e336_d_b0: f64 = ((eq23_e333_d_b0 * ddt_scale) * p.p1);
        let eq23_e336_d_b1: f64 = ((eq23_e333_d_b1 * ddt_scale) * p.p1);
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
        let eq25_e355: f64 = (eq25_e353 * p.p1);
        let eq25_e355_d_n0: f64 = ((eq25_e352_d_n0 * ddt_scale) * p.p1);
        let eq25_e355_d_n1: f64 = ((eq25_e352_d_n1 * ddt_scale) * p.p1);
        let eq25_e355_d_n2: f64 = ((eq25_e352_d_n2 * ddt_scale) * p.p1);
        let eq25_e355_d_n3: f64 = ((eq25_e352_d_n3 * ddt_scale) * p.p1);
        let eq25_e355_d_n4: f64 = ((eq25_e352_d_n4 * ddt_scale) * p.p1);
        let eq25_e355_d_n5: f64 = ((eq25_e352_d_n5 * ddt_scale) * p.p1);
        let eq25_e355_d_n6: f64 = ((eq25_e352_d_n6 * ddt_scale) * p.p1);
        let eq25_e355_d_n7: f64 = ((eq25_e352_d_n7 * ddt_scale) * p.p1);
        let eq25_e355_d_n8: f64 = ((eq25_e352_d_n8 * ddt_scale) * p.p1);
        let eq25_e355_d_n9: f64 = ((eq25_e352_d_n9 * ddt_scale) * p.p1);
        let eq25_e355_d_n10: f64 = ((eq25_e352_d_n10 * ddt_scale) * p.p1);
        let eq25_e355_d_n11: f64 = ((eq25_e352_d_n11 * ddt_scale) * p.p1);
        let eq25_e355_d_b0: f64 = ((eq25_e352_d_b0 * ddt_scale) * p.p1);
        let eq25_e355_d_b1: f64 = ((eq25_e352_d_b1 * ddt_scale) * p.p1);
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
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e272_d_n0, eq14_e272_d_n1, eq14_e272_d_n2, eq14_e272_d_n3, eq14_e272_d_n4, eq14_e272_d_n5, eq14_e272_d_n6, eq14_e272_d_n7, eq14_e272_d_n8, eq14_e272_d_n9, eq14_e272_d_n10, eq14_e272_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e272_d_b0, eq14_e272_d_b1];
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
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e278_d_n0, eq15_e278_d_n1, eq15_e278_d_n2, eq15_e278_d_n3, eq15_e278_d_n4, eq15_e278_d_n5, eq15_e278_d_n6, eq15_e278_d_n7, eq15_e278_d_n8, eq15_e278_d_n9, eq15_e278_d_n10, eq15_e278_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e278_d_b0, eq15_e278_d_b1];
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
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e288_d_n0, eq16_e288_d_n1, eq16_e288_d_n2, eq16_e288_d_n3, eq16_e288_d_n4, eq16_e288_d_n5, eq16_e288_d_n6, eq16_e288_d_n7, eq16_e288_d_n8, eq16_e288_d_n9, eq16_e288_d_n10, eq16_e288_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 2] = [eq16_e288_d_b0, eq16_e288_d_b1];
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
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e294_d_n0, eq17_e294_d_n1, eq17_e294_d_n2, eq17_e294_d_n3, eq17_e294_d_n4, eq17_e294_d_n5, eq17_e294_d_n6, eq17_e294_d_n7, eq17_e294_d_n8, eq17_e294_d_n9, eq17_e294_d_n10, eq17_e294_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e294_d_b0, eq17_e294_d_b1];
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
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e300_d_n0, eq18_e300_d_n1, eq18_e300_d_n2, eq18_e300_d_n3, eq18_e300_d_n4, eq18_e300_d_n5, eq18_e300_d_n6, eq18_e300_d_n7, eq18_e300_d_n8, eq18_e300_d_n9, eq18_e300_d_n10, eq18_e300_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e300_d_b0, eq18_e300_d_b1];
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
        let eq19_reactive_node_derivatives: [f64; 12] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10, eq19_e308_d_n11];
        let eq19_reactive_branch_derivatives: [f64; 2] = [eq19_e308_d_b0, eq19_e308_d_b1];
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
        let eq20_reactive_node_derivatives: [f64; 12] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10, eq20_e316_d_n11];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
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
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e336_d_n0, eq23_e336_d_n1, eq23_e336_d_n2, eq23_e336_d_n3, eq23_e336_d_n4, eq23_e336_d_n5, eq23_e336_d_n6, eq23_e336_d_n7, eq23_e336_d_n8, eq23_e336_d_n9, eq23_e336_d_n10, eq23_e336_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e336_d_b0, eq23_e336_d_b1];
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
        let eq25_reactive_node_derivatives: [f64; 12] = [eq25_e355_d_n0, eq25_e355_d_n1, eq25_e355_d_n2, eq25_e355_d_n3, eq25_e355_d_n4, eq25_e355_d_n5, eq25_e355_d_n6, eq25_e355_d_n7, eq25_e355_d_n8, eq25_e355_d_n9, eq25_e355_d_n10, eq25_e355_d_n11];
        let eq25_reactive_branch_derivatives: [f64; 2] = [eq25_e355_d_b0, eq25_e355_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
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
