#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[984] = (p.p37 >= 0.0);
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        if s.b[984] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[984]) {
            s.store_scalar(0, (-1.0));
        }

        s.v[761] = (8.8541878176e-12 * 11.8);

        s.v[344] = (273.15 + p.p38);

        s.v[468] = 0.0;

        s.b[985] = (p.p920 > 0.5);
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

        let (assign70_e1470,) = {
    if s.b[985] {
        (1.0,)
    } else {
        (s.v[468],)
    }
};
        s.v[468] = assign70_e1470;

        let (assign80_e1475,) = {
    if (!s.b[985]) {
        (0.0,)
    } else {
        (s.v[468],)
    }
};
        s.v[468] = assign80_e1475;

        s.v[358] = (273.15 + p.p816);

        s.v[361] = (1.3806505e-23 / 1.6021918e-19);

        s.v[362] = (s.v[361] * s.v[358]);

        s.v[363] = (1.0 / s.v[362]);

        s.v[369] = ((-((0.000702 * s.v[358]) * s.v[358])) / (1108.0 + s.v[358]));

        s.v[372] = (p.p827 + s.v[369]);

        s.v[373] = (p.p828 + s.v[369]);

        s.v[374] = (p.p829 + s.v[369]);

        s.v[402] = (1.0 - p.p824);

        s.v[403] = (1.0 - p.p825);

        s.v[404] = (1.0 - p.p826);

        s.v[405] = (1.0 / s.v[402]);

        s.v[406] = (1.0 / s.v[403]);

        s.v[407] = (1.0 / s.v[404]);

        s.v[417] = (s.v[761] / p.p818);

        s.v[418] = ((p.p836 * s.v[761]) / p.p819);

        s.v[419] = ((p.p837 * s.v[761]) / p.p820);

        s.v[420] = (1.0 / s.v[417]);

        s.v[421] = (1.0 / s.v[418]);

        s.v[422] = (1.0 / s.v[419]);

        s.v[423] = (1.0 / p.p821);

        s.v[424] = (1.0 / p.p822);

        s.v[425] = (1.0 / p.p823);

        s.v[366] = (1.772453850905516 * 0.29214664);

        s.v[367] = (((((-5.0) * 0.29214664) + 6.0) - ((s.v[366]) as f64).powf((-2.0))) / 3.0);

        s.v[368] = ((1.0 - 0.29214664) - s.v[367]);

        s.v[438] = (1.0 - (1.0 / p.p817));

        s.v[439] = (1.0 / (1.0 - ((s.v[438]) as f64).powf(p.p856)));

        s.v[440] = (1.0 / (1.0 - ((s.v[438]) as f64).powf(p.p857)));

        s.v[441] = (1.0 / (1.0 - ((s.v[438]) as f64).powf(p.p858)));

        s.v[442] = (1.0 / p.p853);

        s.v[443] = (1.0 / p.p854);

        s.v[444] = (1.0 / p.p855);

        s.v[445] = (((-((s.v[439] * s.v[439]) * ((s.v[438]) as f64).powf((p.p856 - 1.0)))) * p.p856) * s.v[442]);

        s.v[446] = (((-((s.v[440] * s.v[440]) * ((s.v[438]) as f64).powf((p.p857 - 1.0)))) * p.p857) * s.v[443]);

        s.v[447] = (((-((s.v[441] * s.v[441]) * ((s.v[438]) as f64).powf((p.p858 - 1.0)))) * p.p858) * s.v[444]);

        s.b[986] = ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        let (assign460_e1672,) = {
    if s.b[986] {
        (1.0,)
    } else {
        (s.v[467],)
    }
};
        s.v[467] = assign460_e1672;

        let (assign470_e1677,) = {
    if (!s.b[986]) {
        (0.0,)
    } else {
        (s.v[467],)
    }
};
        s.v[467] = assign470_e1677;

        s.b[987] = (s.v[467] == 1.0);
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if s.b[987] {
            s.store_scalar(451, (if ((p.p820 * p.p859) > 1e-18) { (p.p820 * p.p859) } else { 1e-18 }));
        }

        if s.b[987] {
            s.store_scalar(452, (if ((p.p823 * p.p860) > 0.05) { (p.p823 * p.p860) } else { 0.05 }));
        }

        if s.b[987] {
            s.store_scalar(453, (if ((if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) < 0.95) { (if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[987] {
            s.store_scalar(454, (p.p829 * p.p862));
            s.store_offset(456, 454, s.v[369]);
            s.store_sub_from_scalar(461, 1.0, 453);
            s.store_div_from_scalar(462, 1.0, 461);
        }

        s.b[988] = (p.p44 == 0.0);
        s.v[988] = if s.b[988] { 1.0 } else { 0.0 };

        if s.b[988] {
            s.store_scalar(499, p.p818);
            s.store_scalar(500, p.p819);
            s.store_scalar(501, p.p820);
            s.store_scalar(502, p.p821);
            s.store_scalar(503, p.p822);
            s.store_scalar(504, p.p823);
            s.store_scalar(505, p.p824);
            s.store_scalar(506, p.p825);
            s.store_scalar(507, p.p826);
            s.store_scalar(508, p.p827);
            s.store_scalar(509, p.p828);
            s.store_scalar(510, p.p829);
            s.store_scalar(511, p.p830);
            s.store_scalar(512, p.p831);
            s.store_scalar(513, p.p832);
            s.store_scalar(516, p.p833);
            s.store_scalar(517, p.p834);
            s.store_scalar(518, p.p835);
            s.store_scalar(514, p.p836);
            s.store_scalar(515, p.p837);
            s.store_scalar(519, p.p838);
            s.store_scalar(520, p.p839);
            s.store_scalar(521, p.p840);
            s.store_scalar(522, p.p841);
            s.store_scalar(523, p.p842);
            s.store_scalar(524, p.p843);
            s.store_scalar(525, p.p844);
            s.store_scalar(526, p.p845);
            s.store_scalar(527, p.p846);
            s.store_scalar(528, p.p847);
            s.store_scalar(529, p.p848);
            s.store_scalar(530, p.p849);
            s.store_scalar(531, p.p850);
            s.store_scalar(532, p.p851);
            s.store_scalar(533, p.p852);
            s.store_scalar(534, p.p853);
            s.store_scalar(535, p.p854);
            s.store_scalar(536, p.p855);
            s.store_scalar(537, p.p856);
            s.store_scalar(538, p.p857);
            s.store_scalar(539, p.p858);
            s.store_scalar(546, p.p921);
        }

        let (assign990_e1932,) = {
    if s.b[988] {
        (p.p922,)
    } else {
        (s.v[547],)
    }
};
        s.v[547] = assign990_e1932;

        if s.b[988] {
            s.store_scalar(630, p.p865);
            s.store_scalar(631, p.p866);
            s.store_scalar(632, p.p867);
            s.store_scalar(633, p.p868);
            s.store_scalar(540, p.p859);
            s.store_scalar(541, p.p860);
            s.store_scalar(542, p.p861);
            s.store_scalar(543, p.p862);
            s.store_scalar(544, p.p863);
            s.store_scalar(545, p.p864);
        }

        if (!s.b[988]) {
            s.store_scalar(499, p.p869);
            s.store_scalar(500, p.p870);
            s.store_scalar(501, p.p871);
            s.store_scalar(502, p.p872);
            s.store_scalar(503, p.p873);
            s.store_scalar(504, p.p874);
            s.store_scalar(505, p.p875);
            s.store_scalar(506, p.p876);
            s.store_scalar(507, p.p877);
            s.store_scalar(508, p.p878);
            s.store_scalar(509, p.p879);
            s.store_scalar(510, p.p880);
            s.store_scalar(511, p.p881);
            s.store_scalar(512, p.p882);
            s.store_scalar(513, p.p883);
            s.store_scalar(516, p.p884);
            s.store_scalar(517, p.p885);
            s.store_scalar(518, p.p886);
            s.store_scalar(514, p.p887);
            s.store_scalar(515, p.p888);
            s.store_scalar(519, p.p889);
            s.store_scalar(520, p.p890);
            s.store_scalar(521, p.p891);
            s.store_scalar(522, p.p892);
            s.store_scalar(523, p.p893);
            s.store_scalar(524, p.p894);
            s.store_scalar(525, p.p895);
            s.store_scalar(526, p.p896);
            s.store_scalar(527, p.p897);
            s.store_scalar(528, p.p898);
            s.store_scalar(529, p.p899);
            s.store_scalar(530, p.p900);
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[988]) {
            s.store_scalar(531, p.p901);
            s.store_scalar(532, p.p902);
            s.store_scalar(533, p.p903);
            s.store_scalar(534, p.p904);
            s.store_scalar(535, p.p905);
            s.store_scalar(536, p.p906);
            s.store_scalar(537, p.p907);
            s.store_scalar(538, p.p908);
            s.store_scalar(539, p.p909);
            s.store_scalar(546, p.p923);
        }

        let (assign1520_e2187,) = {
    if (!s.b[988]) {
        (p.p924,)
    } else {
        (s.v[547],)
    }
};
        s.v[547] = assign1520_e2187;

        if (!s.b[988]) {
            s.store_scalar(630, p.p916);
            s.store_scalar(631, p.p917);
            s.store_scalar(632, p.p918);
            s.store_scalar(633, p.p919);
            s.store_scalar(540, p.p910);
            s.store_scalar(541, p.p911);
            s.store_scalar(542, p.p912);
            s.store_scalar(543, p.p913);
            s.store_scalar(544, p.p914);
            s.store_scalar(545, p.p915);
        }

        s.store_offset(548, 508, s.v[369]);

        s.store_offset(549, 509, s.v[369]);

        s.store_offset(550, 510, s.v[369]);

        s.store_sub_from_scalar(569, 1.0, 505);

        s.store_sub_from_scalar(570, 1.0, 506);

        s.store_sub_from_scalar(571, 1.0, 507);

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(584, s.v[761], 499);

        s.store_div_scaled_inputs_indices(585, 514, s.v[761], 500, 1.0);

        s.store_div_scaled_inputs_indices(586, 515, s.v[761], 501, 1.0);

        s.store_div_from_scalar(587, 1.0, 584);

        s.store_div_from_scalar(588, 1.0, 585);

        s.store_div_from_scalar(589, 1.0, 586);

        s.store_div_from_scalar(590, 1.0, 502);

        s.store_div_from_scalar(591, 1.0, 503);

        s.store_div_from_scalar(592, 1.0, 504);

        s.store_div_from_scalar_sub_from_scalar_ad(605, 1.0, 1.0, A::pow_from_scalar(s.v[438], s.ad_value(537)));

        s.store_div_from_scalar_sub_from_scalar_ad(606, 1.0, 1.0, A::pow_from_scalar(s.v[438], s.ad_value(538)));

        s.store_div_from_scalar_sub_from_scalar_ad(607, 1.0, 1.0, A::pow_from_scalar(s.v[438], s.ad_value(539)));

        s.store_div_from_scalar(608, 1.0, 534);

        s.store_div_from_scalar(609, 1.0, 535);

        s.store_div_from_scalar(610, 1.0, 536);

        s.store_mul_product3_rhs(611, 608, A::square(s.ad_value(605)), A::pow_from_scalar(s.v[438], A::offset(s.ad_value(537), (-1.0))), s.ad_value(537), -1.0);

        s.store_mul_product3_rhs(612, 609, A::square(s.ad_value(606)), A::pow_from_scalar(s.v[438], A::offset(s.ad_value(538), (-1.0))), s.ad_value(538), -1.0);

        s.store_mul_product3_rhs(613, 610, A::square(s.ad_value(607)), A::pow_from_scalar(s.v[438], A::offset(s.ad_value(539), (-1.0))), s.ad_value(539), -1.0);

        s.b[989] = ((((s.v[540] != 1.0) || (s.v[541] != 1.0)) || (s.v[542] != 1.0)) || (s.v[543] != 1.0));
        s.v[989] = if s.b[989] { 1.0 } else { 0.0 };

        let (assign1910_e2386,) = {
    if s.b[989] {
        (1.0,)
    } else {
        (s.v[629],)
    }
};
        s.v[629] = assign1910_e2386;

        let (assign1920_e2391,) = {
    if (!s.b[989]) {
        (0.0,)
    } else {
        (s.v[629],)
    }
};
        s.v[629] = assign1920_e2391;

        s.b[990] = (s.v[629] == 1.0);
        s.v[990] = if s.b[990] { 1.0 } else { 0.0 };

        if s.b[990] {
            if ((s.v[501] * s.v[540]) > 1e-18) {
                s.store_mul(614, 501, 540);
            } else {
                s.store_scalar(614, 1e-18);
            }
        }

        if s.b[990] {
            if ((s.v[504] * s.v[541]) > 0.05) {
                s.store_mul(615, 504, 541);
            } else {
                s.store_scalar(615, 0.05);
            }
        }

        if s.b[990] {
            if ((if ((s.v[507] * s.v[542]) > 0.05) { (s.v[507] * s.v[542]) } else { 0.05 }) < 0.95) {
                if ((s.v[507] * s.v[542]) > 0.05) {
                    s.store_mul(616, 507, 542);
                } else {
                    s.store_scalar(616, 0.05);
                }
            } else {
                s.store_scalar(616, 0.95);
            }
        }

        if s.b[990] {
            s.store_mul(617, 510, 543);
            s.store_offset(619, 617, s.v[369]);
            s.store_sub_from_scalar(624, 1.0, 616);
            s.store_div_from_scalar(625, 1.0, 624);
        }

        s.v[345] = ((ctx_temp + p.p55) + p.p35);

        s.v[346] = (s.v[345] / s.v[344]);

        s.v[347] = (s.v[345] - s.v[344]);

        s.v[348] = ((s.v[345] * 1.3806505e-23) / 1.6021918e-19);

        s.v[349] = (1.0 / s.v[348]);

        s.v[350] = s.v[345];

        s.v[351] = (s.v[350] * s.v[350]);

        s.v[352] = (s.v[350] - s.v[344]);

        s.v[353] = (s.v[344] / s.v[350]);

        s.v[354] = ((s.v[353]) as f64).ln();

        s.v[709] = ((s.v[350] * 1.3806505e-23) / 1.6021918e-19);

        s.v[355] = (1.0 / s.v[709]);

        s.v[356] = ((1.179 - (9.025e-5 * s.v[350])) - (3.05e-7 * s.v[351]));

        s.v[357] = ((((1.045 + (0.00045 * s.v[350])) * ((0.523 + (0.0014 * s.v[350])) - (1.48e-6 * s.v[351]))) * s.v[351]) / 90000.0);

        if (!(s.v[357] > 0.001)) {
            s.store_scalar(357, 0.001);
        }

        s.v[712] = ((4.0 * 1.3806505e-23) * s.v[350]);

        s.v[359] = (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0)));

        s.v[360] = (s.v[359] / s.v[358]);

        s.v[364] = (s.v[361] * s.v[359]);

        s.v[365] = (1.0 / s.v[364]);

        s.v[370] = ((-((0.000702 * s.v[359]) * s.v[359])) / (1108.0 + s.v[359]));

        s.v[375] = (p.p827 + s.v[370]);

        s.v[376] = (p.p828 + s.v[370]);

        s.v[377] = (p.p829 + s.v[370]);

        s.v[378] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[372] * s.v[363]) - (s.v[375] * s.v[365])))) as f64).exp());

        s.v[379] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[373] * s.v[363]) - (s.v[376] * s.v[365])))) as f64).exp());

        s.v[380] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[374] * s.v[363]) - (s.v[377] * s.v[365])))) as f64).exp());

        s.v[381] = ((p.p830 * s.v[378]) * s.v[378]);

        s.v[382] = ((p.p831 * s.v[379]) * s.v[379]);

        s.v[383] = ((p.p832 * s.v[380]) * s.v[380]);

        s.v[384] = ((p.p821 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[378]) as f64).ln()));

        s.v[385] = ((p.p822 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[379]) as f64).ln()));

        s.v[386] = ((p.p823 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[380]) as f64).ln()));

        s.v[387] = (s.v[384] + (s.v[364] * (((1.0 + ((((0.05 - s.v[384]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[388] = (s.v[385] + (s.v[364] * (((1.0 + ((((0.05 - s.v[385]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[389] = (s.v[386] + (s.v[364] * (((1.0 + ((((0.05 - s.v[386]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[399] = (1.0 / s.v[387]);

        s.v[400] = (1.0 / s.v[388]);

        s.v[401] = (1.0 / s.v[389]);

        s.v[408] = (p.p818 * (((p.p821 * s.v[399])) as f64).powf(p.p824));

        s.v[409] = (p.p819 * (((p.p822 * s.v[400])) as f64).powf(p.p825));

        s.v[410] = (p.p820 * (((p.p823 * s.v[401])) as f64).powf(p.p826));

        s.v[411] = ((s.v[408] * s.v[387]) * s.v[405]);

        s.v[412] = ((s.v[409] * s.v[388]) * s.v[406]);

        s.v[413] = ((s.v[410] * s.v[389]) * s.v[407]);

        s.v[414] = (2.0 * s.v[408]);

        s.v[415] = (2.0 * s.v[409]);

        s.v[416] = (2.0 * s.v[410]);

        s.v[426] = ((0.5 * s.v[375])).max(s.v[364]);

        s.v[427] = ((0.5 * s.v[376])).max(s.v[364]);

        s.v[428] = ((0.5 * s.v[377])).max(s.v[364]);

        s.v[429] = (s.v[426] * s.v[365]);

        s.v[430] = (s.v[427] * s.v[365]);

        s.v[431] = (s.v[428] * s.v[365]);

        s.v[432] = (((((((32.0 * p.p841) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[426] * s.v[426]) * s.v[426]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[433] = (((((((32.0 * p.p842) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[427] * s.v[427]) * s.v[427]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[434] = (((((((32.0 * p.p843) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[428] * s.v[428]) * s.v[428]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[435] = (p.p847 * (1.0 + (p.p850 * (s.v[359] - s.v[358]))));

        s.v[436] = (p.p848 * (1.0 + (p.p851 * (s.v[359] - s.v[358]))));

        s.v[437] = (p.p849 * (1.0 + (p.p852 * (s.v[359] - s.v[358]))));

        if (!(s.v[435] > 0.0)) {
            s.store_scalar(435, 0.0);
        }

        if (!(s.v[436] > 0.0)) {
            s.store_scalar(436, 0.0);
        }

        if (!(s.v[437] > 0.0)) {
            s.store_scalar(437, 0.0);
        }

        s.b[1010] = (s.v[467] == 1.0);
        s.v[1010] = if s.b[1010] { 1.0 } else { 0.0 };

        if s.b[1010] {
            s.store_offset(455, 454, s.v[370]);
            s.store_scale_ad(457, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(456), s.v[363], s.ad_value(455), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(458, 452, s.v[360], A::ln(s.ad_value(457)), (2.0 * s.v[364]));
            s.store_add_scaled_inputs_ad_rhs(459, 458, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(458), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);
            s.store_div_from_scalar(460, 1.0, 459);
            s.store_mul_pow_ad_rhs(463, 451, A::mul(s.ad_value(452), s.ad_value(460)), s.ad_value(453));
            s.store_mul3_lhs(464, 463, 459, 462);
            s.store_scale(465, 463, 2.0);
        }

        s.store_offset(551, 508, s.v[370]);

        s.store_offset(552, 509, s.v[370]);

        s.store_offset(553, 510, s.v[370]);

        s.store_scale_ad(554, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(548), s.v[363], s.ad_value(551), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(555, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(549), s.v[363], s.ad_value(552), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(556, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(550), s.v[363], s.ad_value(553), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_mul3_lhs(557, 511, 554, 554);

        s.store_mul3_lhs(558, 512, 555, 555);

        s.store_mul3_lhs(559, 513, 556, 556);

        s.store_sub_scaled_inputs_ad_rhs(560, 502, s.v[360], A::ln(s.ad_value(554)), (2.0 * s.v[364]));

        s.store_sub_scaled_inputs_ad_rhs(561, 503, s.v[360], A::ln(s.ad_value(555)), (2.0 * s.v[364]));

        s.store_sub_scaled_inputs_ad_rhs(562, 504, s.v[360], A::ln(s.ad_value(556)), (2.0 * s.v[364]));

        s.store_add_scaled_inputs_ad_rhs(563, 560, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(560), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);

        s.store_add_scaled_inputs_ad_rhs(564, 561, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(561), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);

        s.store_add_scaled_inputs_ad_rhs(565, 562, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(562), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);

        s.store_div_from_scalar(566, 1.0, 563);

        s.store_div_from_scalar(567, 1.0, 564);

        s.store_div_from_scalar(568, 1.0, 565);

        s.store_mul_pow_ad_rhs(575, 499, A::mul(s.ad_value(502), s.ad_value(566)), s.ad_value(505));

        s.store_mul_pow_ad_rhs(576, 500, A::mul(s.ad_value(503), s.ad_value(567)), s.ad_value(506));

        s.store_mul_pow_ad_rhs(577, 501, A::mul(s.ad_value(504), s.ad_value(568)), s.ad_value(507));

        s.store_mul3_lhs(578, 575, 563, 572);

        s.store_mul3_lhs(579, 576, 564, 573);

        s.store_mul3_lhs(580, 577, 565, 574);

        s.store_scale(581, 575, 2.0);

        s.store_scale(582, 576, 2.0);

        s.store_scale(583, 577, 2.0);

        s.store_max_with_scalar_ad(593, A::scale(s.ad_value(551), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(594, A::scale(s.ad_value(552), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(595, A::scale(s.ad_value(553), 0.5), s.v[364]);

        s.store_scale(596, 593, s.v[365]);

        s.store_scale(597, 594, s.v[365]);

        s.store_scale(598, 595, s.v[365]);

        s.store_scaled_sqrt_ad(599, A::mul3_scaled_output(s.ad_value(522), A::square(s.ad_value(593)), s.ad_value(593), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(600, A::mul3_scaled_output(s.ad_value(523), A::square(s.ad_value(594)), s.ad_value(594), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(601, A::mul3_scaled_output(s.ad_value(524), A::square(s.ad_value(595)), s.ad_value(595), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_scale_offset_rhs(602, 528, 531, (s.v[359] - s.v[358]), 1.0);

        s.store_mul_scale_offset_rhs(603, 529, 532, (s.v[359] - s.v[358]), 1.0);

        s.store_mul_scale_offset_rhs(604, 530, 533, (s.v[359] - s.v[358]), 1.0);

        if (!(s.v[602] > 0.0)) {
            s.store_scalar(602, 0.0);
        }

        if (!(s.v[603] > 0.0)) {
            s.store_scalar(603, 0.0);
        }

        if (!(s.v[604] > 0.0)) {
            s.store_scalar(604, 0.0);
        }

        s.b[1011] = (s.v[629] == 1.0);
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if s.b[1011] {
            s.store_offset(618, 617, s.v[370]);
            s.store_scale_ad(620, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(619), s.v[363], s.ad_value(618), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(621, 615, s.v[360], A::ln(s.ad_value(620)), (2.0 * s.v[364]));
            s.store_add_scaled_inputs_ad_rhs(622, 621, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(621), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);
            s.store_div_from_scalar(623, 1.0, 622);
            s.store_mul_pow_ad_rhs(626, 614, A::mul(s.ad_value(615), s.ad_value(623)), s.ad_value(616));
            s.store_mul3_lhs(627, 626, 622, 625);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1011] {
            s.store_scale(628, 626, 2.0);
        }

        s.v[1] = 1.0;

        s.v[2] = 1.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[3] = p.p0;

        s.v[4] = p.p1;

        s.v[5] = p.p2;

        s.v[6] = p.p3;

        s.v[7] = p.p4;

        s.v[8] = p.p8;

        s.v[9] = p.p11;

        s.v[640] = p.p19;

        s.v[641] = p.p20;

        s.v[642] = p.p21;

        s.v[667] = p.p22;

        s.v[668] = p.p23;

        s.v[669] = p.p24;

        s.v[643] = p.p25;

        s.v[644] = p.p26;

        s.v[670] = p.p27;

        s.v[671] = p.p28;

        s.v[10] = p.p14;

        s.b[1012] = (p.p39 > 0.0);
        s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };

        if s.b[1012] {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1012] {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
            s.store_div_from_scalar(2, 1.0, 1);
        }

        if ((s.v[4] * s.v[2]) > 1e-9) {
            s.store_scale(4, 2, s.v[4]);
        } else {
            s.store_scalar(4, 1e-9);
        }

        s.v[11] = p.p5;

        s.v[12] = p.p6;

        s.v[13] = p.p7;

        s.v[14] = (if (p.p10 < 1.5) { 1.0 } else { 2.0 });

        s.v[302] = (1e-6 / s.v[3]);

        s.store_div_from_scalar(303, 1e-6, 4);

        s.store_offset_scaled(304, 303, ((p.p188) * ((p.p186 * (1.0 + (p.p187 * s.v[302]))))), (p.p186 * (1.0 + (p.p187 * s.v[302]))));

        s.store_offset_scaled(305, 303, ((p.p192) * ((p.p190 * (1.0 + (p.p191 * s.v[302]))))), (p.p190 * (1.0 + (p.p191 * s.v[302]))));

        if (((s.v[3] + s.v[304]) - (2.0 * p.p189)) > 1e-9) {
            s.store_offset(306, 304, ((s.v[3]) + ((-(2.0 * p.p189)))));
        } else {
            s.store_scalar(306, 1e-9);
        }

        if (((s.v[4] + s.v[305]) - (2.0 * p.p193)) > 1e-9) {
            s.store_offset_add(307, 4, 305, (-(2.0 * p.p193)));
        } else {
            s.store_scalar(307, 1e-9);
        }

        s.store_div_from_scalar(308, 1e-6, 306);

        s.store_square(309, 308);

        s.store_div_from_scalar(310, 1e-6, 307);

        s.store_div_from_scalar(311, 1.0, 310);

        s.store_mul(312, 308, 310);

        s.store_div_from_scalar(313, 1.0, 312);

        if ((((s.v[3] + s.v[304]) - (2.0 * p.p189)) + p.p194) > 1e-9) {
            s.store_offset(314, 304, ((((s.v[3]) + ((-(2.0 * p.p189))))) + (p.p194)));
        } else {
            s.store_scalar(314, 1e-9);
        }

        if ((((s.v[4] + s.v[305]) - (2.0 * p.p193)) + p.p195) > 1e-9) {
            s.store_offset_add(315, 4, 305, (((-(2.0 * p.p193))) + (p.p195)));
        } else {
            s.store_scalar(315, 1e-9);
        }

        s.store_scale(316, 315, 1000000.0);

        if (((s.v[3] + s.v[304]) + p.p194) > 1e-9) {
            s.store_offset(317, 304, ((s.v[3]) + (p.p194)));
        } else {
            s.store_scalar(317, 1e-9);
        }

        if (((s.v[4] + s.v[305]) + p.p195) > 1e-9) {
            s.store_offset_add(318, 4, 305, p.p195);
        } else {
            s.store_scalar(318, 1e-9);
        }

        s.store_scale(319, 317, 1000000.0);

        s.store_scale(320, 318, 1000000.0);

        if ((s.v[3] + s.v[304]) > 1e-9) {
            s.store_offset(321, 304, s.v[3]);
        } else {
            s.store_scalar(321, 1e-9);
        }

        if ((s.v[321] + p.p441) > 1e-9) {
            s.store_offset(322, 321, p.p441);
        } else {
            s.store_scalar(322, 1e-9);
        }

        if ((s.v[4] + s.v[305]) > 1e-9) {
            s.store_add(323, 4, 305);
        } else {
            s.store_scalar(323, 1e-9);
        }

        if ((s.v[9] - (0.5 * s.v[305])) > 1e-9) {
            s.store_sub_from_scalar_scaled_input(324, s.v[9], 305, 0.5);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.v[40] = p.p56;

        s.v[41] = p.p57;

        s.v[42] = p.p58;

        s.v[43] = p.p59;

        s.v[44] = p.p60;

        s.v[45] = p.p61;

        s.v[46] = p.p62;

        s.v[47] = p.p63;

        s.v[48] = p.p64;

        s.v[49] = p.p65;

        s.v[50] = p.p66;

        s.v[55] = p.p67;

        s.v[56] = p.p68;

        s.v[57] = p.p69;

        s.v[58] = p.p70;

        s.v[51] = p.p71;

        s.v[52] = p.p73;

        s.v[53] = p.p72;

        s.v[54] = p.p74;

        s.v[59] = p.p78;

        s.v[60] = p.p80;

        s.v[61] = p.p79;

        s.v[62] = p.p75;

        s.v[63] = p.p77;

        s.v[64] = p.p76;

        s.v[65] = p.p81;

        s.v[66] = p.p82;

        s.v[67] = p.p83;

        s.v[68] = p.p84;

        s.v[69] = p.p85;

        s.v[70] = p.p86;

        s.v[71] = p.p87;

        s.v[72] = p.p88;

        s.v[73] = p.p89;

        s.v[74] = p.p90;

        s.v[75] = p.p91;

        s.v[76] = p.p92;

        s.v[77] = p.p93;

        s.v[78] = p.p94;

        s.v[79] = p.p95;

        s.v[80] = p.p96;

        s.v[81] = p.p97;

        s.v[82] = p.p98;

        s.v[83] = p.p99;

        s.v[84] = p.p100;

        s.v[85] = p.p101;

        s.v[86] = p.p102;

        s.v[87] = p.p103;

        s.v[88] = p.p104;

        s.v[89] = p.p105;

        s.v[90] = p.p106;

        s.v[91] = p.p107;

        s.v[92] = p.p108;

        s.v[93] = p.p109;

        s.v[94] = p.p110;

        s.v[95] = p.p111;

        s.v[96] = p.p112;

        s.v[97] = p.p113;

        s.v[98] = p.p114;

        s.v[99] = p.p115;

        s.v[100] = p.p116;

        s.v[101] = p.p117;

        s.v[102] = p.p118;

        s.v[103] = p.p119;

        s.v[104] = p.p120;

        s.v[105] = p.p119;

        s.b[1013] = param_given[121];
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        if s.b[1013] {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.b[1014] = param_given[122];
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        if s.b[1014] {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.b[1015] = param_given[123];
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if s.b[1015] {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.b[1016] = param_given[124];
        s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };

        if s.b[1016] {
            s.store_scalar(108, p.p124);
        }

        s.v[109] = p.p125;

        s.v[110] = p.p126;

        s.v[111] = p.p127;

        s.v[112] = p.p128;

        s.v[113] = p.p129;

        s.v[114] = p.p130;

        s.v[115] = p.p131;

        s.v[116] = p.p132;

        s.v[117] = p.p133;

        s.v[118] = p.p134;

        s.v[119] = p.p135;

        s.v[120] = p.p136;

        s.v[121] = p.p98;

        s.b[1017] = param_given[137];
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if s.b[1017] {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.b[1018] = param_given[138];
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        if s.b[1018] {
            s.store_scalar(122, p.p138);
        }

        s.v[123] = p.p139;

        s.v[124] = p.p140;

        s.v[125] = p.p141;

        s.v[126] = p.p142;

        s.v[127] = p.p143;

        s.v[128] = p.p144;

        s.v[129] = p.p145;

        s.v[130] = p.p146;

        s.v[131] = p.p147;

        s.v[132] = p.p148;

        s.v[133] = p.p149;

        s.v[134] = p.p150;

        s.v[135] = p.p151;

        s.v[136] = p.p152;

        s.v[137] = p.p153;

        s.v[138] = p.p154;

        s.v[139] = p.p155;

        s.v[140] = p.p156;

        s.v[145] = p.p161;

        s.v[146] = p.p162;

        s.v[147] = p.p163;

        s.v[148] = p.p164;

        s.v[149] = p.p165;

        s.v[150] = p.p166;

        s.v[151] = p.p167;

        s.v[152] = p.p168;

        s.v[153] = p.p169;

        s.v[154] = p.p170;

        s.v[155] = p.p171;

        s.v[156] = p.p173;

        s.v[157] = p.p172;

        s.v[163] = p.p179;

        s.v[166] = p.p180;

        s.v[167] = p.p181;

        s.v[168] = p.p183;

        s.v[169] = p.p182;

        s.v[170] = p.p184;

        s.v[171] = p.p185;

        s.b[1019] = (p.p39 > 0.0);
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        if s.b[1019] {
            s.store_add_scaled_inputs3_offset_mixed_aii(40, A::powf(s.ad_value(308), p.p198), p.p197, 310, p.p199, 312, p.p200, p.p196);
            s.store_add_scaled_inputs3_offset_indices(41, 308, p.p202, 310, p.p203, 312, p.p204, p.p201);
            s.store_scalar(42, p.p205);
            s.store_scalar(43, p.p206);
            s.store_scalar(44, p.p207);
        }

        if s.b[1019] {
            s.store_scale_ad(325, {
                if ((1.0 + ((p.p209 * s.v[310]) * (((1.0 + (s.v[307] / p.p210))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p209, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p210), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p208);
        }

        if s.b[1019] {
            s.store_scale_ad(326, {
                if ((1.0 + ((p.p212 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p212, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }

        if s.b[1019] {
            s.store_scale_ad(327, {
                if ((1.0 + ((p.p215 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p215, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }

        s.b[1020] = (s.v[306] > (2.0 * s.v[327]));
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1020]) {
            s.store_scalar(328, 75000000000.0);
            s.store_sub_ad(329, A::sqrt(A::add_scaled_inputs(s.ad_value(325), 1.0, s.ad_value(326), 0.5)), A::sqrt(s.ad_value(325)));
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1019] && s.b[1020]) {
            s.store_add_scaled_product_mixed_aia(330, A::sqrt(s.ad_value(325)), 1.0, 328, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(327), 2.0, s.ad_value(306), 1.0), A::exp(A::div(s.ad_value(329), s.ad_value(328))), (-1.0)), 1.0)), 1.0);
            s.store_square(330, 330);
        }

        s.b[1021] = (s.v[306] >= s.v[327]);
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {
            s.store_add_ad_rhs(330, 325, A::div_scaled_product(s.ad_value(326), s.ad_value(327), 1.0, s.ad_value(306), 1.0));
        }

        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {
            s.store_add_ad_rhs(330, 325, A::mul_sub_from_scalar_rhs(s.ad_value(326), 2.0, A::div(s.ad_value(306), s.ad_value(327))));
        }

        if s.b[1019] {
            s.store_mul_sub_scaled_inputs_rhs(45, 330, A::sub_from_scalar(1.0, A::scale(s.ad_value(308), p.p216)), 1.0, s.ad_value(309), p.p217);
            s.store_add_scaled_inputs3_offset_mixed_aii(46, A::powf(s.ad_value(308), p.p220), p.p219, 310, p.p221, 312, p.p222, p.p218);
            s.store_scalar(47, p.p223);
            s.store_scalar(48, p.p224);
            s.store_add_scaled_inputs3_offset_mixed_aii(49, A::powf(s.ad_value(308), p.p227), p.p226, 310, p.p228, 312, p.p229, p.p225);
        }

        if s.b[1019] {
            s.store_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p231 * s.v[308]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(308), p.p231, 1.0)
                }
            }, p.p230);
        }

        if s.b[1019] {
            s.store_scalar(55, p.p232);
            s.store_scalar(56, p.p233);
            s.store_scalar(57, p.p236);
            s.store_scalar(58, p.p237);
            s.store_mul3_ad(51, A::scale_offset(A::powf(s.ad_value(308), p.p240), p.p239, p.p238), A::scale_offset(s.ad_value(310), p.p241, 1.0), A::scale_offset(s.ad_value(312), p.p242, 1.0));
            s.store_scalar(52, p.p244);
            s.store_scalar(53, p.p243);
            s.store_scalar(54, p.p245);
            s.store_scaled_mul_scale_offset_rhs_ad(62, A::powf(s.ad_value(308), p.p247), 310, p.p248, 1.0, p.p246);
            s.store_scalar(63, p.p250);
            s.store_scalar(64, p.p249);
            s.store_scaled_mul_scale_offset_rhs_ad(59, A::powf(s.ad_value(308), p.p252), 310, p.p253, 1.0, p.p251);
            s.store_scalar(60, p.p255);
            s.store_scalar(61, p.p254);
            s.store_offset_scaled(331, 310, ((p.p258) * (p.p257)), p.p257);
        }

        if s.b[1019] {
            s.store_scale_ad(332, {
                if ((1.0 + (p.p260 * s.v[310])) > 0.001) {
                    A::scale_offset(s.ad_value(310), p.p260, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p259);
        }

        if s.b[1019] {
            s.store_add_ad(333, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(331), s.ad_value(332), 1.0, s.ad_value(306), 1.0), 1.0, A::exp(A::div_scaled_inputs(s.ad_value(306), -1.0, s.ad_value(332), 1.0))), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p261 * p.p262), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p262)))));
        }

        if s.b[1019] {
            if (s.v[333] > 1e-15) {
            } else {
                s.store_scalar(333, 1e-15);
            }
        }

        if s.b[1019] {
            s.store_add_scaled_product_mixed_aia(334, A::scale_offset(s.ad_value(310), p.p263, 1.0), 1.0, 310, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p265), 1.0)), p.p264);
            s.store_mul_div_scaled_inputs_rhs(65, 334, s.ad_value(307), p.p256, A::mul(s.ad_value(333), s.ad_value(306)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(66, 308, p.p267, 310, p.p268, 312, p.p269, p.p266);
            s.store_offset_scaled(67, 310, ((p.p271) * (p.p270)), p.p270);
            s.store_scalar(68, p.p272);
            s.store_scalar(69, p.p273);
            s.store_scalar(70, p.p274);
            s.store_mul3_ad(71, A::scale_offset(A::powf(s.ad_value(308), p.p277), p.p276, p.p275), A::scale_offset(s.ad_value(310), p.p278, 1.0), A::scale_offset(s.ad_value(312), p.p279, 1.0));
            s.store_scalar(72, p.p280);
            s.store_scalar(73, p.p281);
            s.store_scalar(74, p.p282);
            s.store_mul3_ad_scaled_output(75, A::scale_offset(s.ad_value(308), p.p284, 1.0), A::scale_offset(s.ad_value(310), p.p285, 1.0), A::scale_offset(s.ad_value(312), p.p286, 1.0), p.p283);
            s.store_scalar(76, p.p287);
            s.store_scalar(77, p.p288);
            s.store_mul_scale_offset_rhs(78, 310, 310, ((p.p290) * (p.p289)), p.p289);
            s.store_scalar(79, p.p291);
            s.store_scalar(80, p.p292);
            s.store_scalar(81, p.p293);
            s.store_mul3_ad(82, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(334), p.p295, s.ad_value(333), 1.0), A::powf(s.ad_value(308), p.p296)), p.p294), A::scale_offset(s.ad_value(310), p.p297, 1.0), A::scale_offset(s.ad_value(312), p.p298, 1.0));
            s.store_add_scaled_inputs3_offset_indices(83, 308, p.p300, 310, p.p301, 312, p.p302, p.p299);
            s.store_scalar(84, p.p303);
            s.store_scalar(85, p.p304);
            s.store_scalar(86, p.p305);
            s.store_div_from_scalar_offset_scaled_input(87, p.p306, 308, p.p307, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(88, A::powf(s.ad_value(308), p.p309), 310, p.p310, 1.0, p.p308);
            s.store_powf(335, 308, p.p312);
            s.store_div_scaled_product_offset_denominator(89, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p314, 1.0), p.p311, A::mul_scaled_lhs(s.ad_value(308), p.p313, s.ad_value(335)), 1.0, 1.0);
            s.store_powf(335, 308, p.p316);
            s.store_div_scaled_product_offset_denominator(90, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p318, 1.0), p.p315, A::mul_scaled_lhs(s.ad_value(308), p.p317, s.ad_value(335)), 1.0, 1.0);
            s.store_scalar(91, p.p319);
            s.store_scaled_mul_scale_offset_inputs(92, 308, p.p321, 1.0, 310, p.p322, 1.0, p.p320);
            s.store_scalar(93, p.p323);
            s.store_scalar(94, p.p324);
            s.store_scaled_mul_scale_offset_inputs(95, 308, p.p326, 1.0, 310, p.p327, 1.0, p.p325);
            s.store_scaled_mul_scale_offset_inputs(96, 308, p.p329, 1.0, 310, p.p330, 1.0, p.p328);
            s.store_scalar(97, p.p331);
            s.store_scalar(98, p.p332);
            s.store_div_from_scalar(99, p.p333, 312);
            s.store_div_from_scalar_scaled_input(100, (p.p334 * p.p234), 310, 1e-6);
            s.store_div_from_scalar_scaled_input(101, (p.p335 * p.p235), 310, 1e-6);
            s.store_scalar(102, p.p336);
            s.store_scalar(103, p.p337);
            s.store_scalar(104, p.p338);
            s.store_scalar(105, p.p337);
        }

        s.b[1022] = param_given[339];
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1022]) {
            s.store_scalar(105, p.p339);
        }

        if s.b[1019] {
            s.store_scalar(106, p.p338);
        }

        s.b[1023] = param_given[340];
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1023]) {
            s.store_scalar(106, p.p340);
        }

        if s.b[1019] {
            s.copy_ad(107, 105);
        }

        s.b[1024] = param_given[341];
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1024]) {
            s.store_scalar(107, p.p341);
        }

        if s.b[1019] {
            s.copy_ad(108, 106);
        }

        s.b[1025] = param_given[342];
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1025]) {
            s.store_scalar(108, p.p342);
        }

        if s.b[1019] {
            s.store_scalar(109, p.p343);
            s.store_div_from_scalar_scaled_input(110, (p.p344 * p.p234), 310, 1e-6);
            s.store_div_from_scalar_scaled_input(111, (p.p345 * p.p235), 310, 1e-6);
            s.store_scalar(112, p.p346);
            s.store_scalar(113, p.p347);
            s.store_scalar(114, p.p348);
            s.store_scalar(115, p.p349);
            s.store_scalar(116, p.p350);
            s.store_scalar(117, p.p351);
            s.store_scaled_mul(118, 315, 314, ((8.8541878176e-12 * p.p207) * 1.0 / (p.p206)));
            s.store_scale(125, 315, ((8.8541878176e-12 * p.p207) * (p.p234 * 1.0 / (p.p232))));
            s.store_scale(126, 315, ((8.8541878176e-12 * p.p207) * (p.p235 * 1.0 / (p.p233))));
            s.store_add_scaled_inputs3_offset_mixed_aii(119, A::powf(s.ad_value(308), p.p354), p.p353, 310, p.p355, 312, p.p356, p.p352);
            s.store_add_scaled_inputs3_offset_indices(120, 308, p.p358, 310, p.p359, 312, p.p360, p.p357);
            s.store_scalar(32, p.p294);
        }

        s.b[1026] = param_given[361];
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1026]) {
            s.store_scalar(32, p.p361);
        }

        if s.b[1019] {
            s.store_scalar(33, p.p295);
        }

        s.b[1027] = param_given[362];
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1027]) {
            s.store_scalar(33, p.p362);
        }

        if s.b[1019] {
            s.store_scalar(34, p.p296);
        }

        s.b[1028] = param_given[363];
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1028]) {
            s.store_scalar(34, p.p363);
        }

        if s.b[1019] {
            s.store_scalar(35, p.p297);
        }

        s.b[1029] = param_given[364];
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1029]) {
            s.store_scalar(35, p.p364);
        }

        if s.b[1019] {
            s.store_scalar(36, p.p298);
        }

        s.b[1030] = param_given[365];
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1030]) {
            s.store_scalar(36, p.p365);
        }

        if s.b[1019] {
            s.store_mul3_ad(121, A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(334), 1.0, s.ad_value(333), 1.0), A::pow(s.ad_value(308), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(310)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(312)), 1.0));
            s.store_scalar(37, p.p306);
        }

        s.b[1031] = param_given[366];
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1031]) {
            s.store_scalar(37, p.p366);
        }

        if s.b[1019] {
            s.store_scalar(38, p.p307);
        }

        s.b[1032] = param_given[367];
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1032]) {
            s.store_scalar(38, p.p367);
        }

        if s.b[1019] {
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(37), 1.0, A::mul(s.ad_value(38), s.ad_value(308)), 1.0, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(123, A::powf(s.ad_value(308), p.p369), 310, p.p370, 1.0, p.p368);
            s.store_powf(335, 308, p.p372);
            s.store_div_scaled_product_offset_denominator(124, s.ad_value(335), A::scale_offset(s.ad_value(310), p.p374, 1.0), p.p371, A::mul_scaled_lhs(s.ad_value(308), p.p373, s.ad_value(335)), 1.0, 1.0);
            s.store_scalar(127, p.p375);
            s.store_scalar(128, p.p376);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1019] {
            s.store_scalar(129, p.p377);
            s.store_scale(130, 319, p.p378);
            s.store_scale(131, 316, p.p379);
            s.store_scale(132, 316, p.p380);
            s.store_scalar(133, p.p381);
            s.store_scalar(134, p.p382);
            s.store_scalar(135, p.p383);
            s.store_scalar(136, p.p384);
            s.store_scale(137, 320, p.p385);
            s.store_scale(138, 320, p.p386);
            s.store_sub_from_scalar_ad(1001, 1.0, A::div_from_scalar((2.0 * p.p393), s.ad_value(306)));
            s.store_scalar(139, p.p387);
            s.store_mul_product3_rhs(140, 310, s.ad_value(65), s.ad_value(65), s.ad_value(310), p.p388);
            s.store_offset_scaled(338, 307, p.p396, (2.0 * p.p395));
            s.store_scalar(145, p.p397);
            s.store_add_scaled_inputs3_offset_indices(146, 308, p.p399, 310, p.p400, 312, p.p401, p.p398);
            s.store_add_scaled_inputs3_offset_mixed_aii(147, A::powf(s.ad_value(308), p.p404), p.p403, 310, p.p405, 312, p.p406, p.p402);
            s.store_mul3_ad_scaled_output(148, A::scale_offset(A::powf(s.ad_value(308), p.p409), p.p408, 1.0), A::scale_offset(s.ad_value(310), p.p410, 1.0), A::scale_offset(s.ad_value(312), p.p411, 1.0), p.p407);
            s.store_offset_scaled_ad(149, A::powf(s.ad_value(308), p.p414), p.p413, p.p412);
            s.store_offset_ad(341, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p415 * p.p416), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p416)))), 1.0);
        }

        if s.b[1019] {
            if (s.v[341] > 1e-15) {
            } else {
                s.store_scalar(341, 1e-15);
            }
        }

        if s.b[1019] {
            s.store_mul_ad(150, A::div_scaled_inputs(s.ad_value(338), p.p256, A::mul(s.ad_value(341), s.ad_value(306)), 1.0), A::scale_offset(s.ad_value(310), p.p417, 1.0));
            s.store_add_scaled_inputs3_offset_indices(151, 308, p.p419, 310, p.p420, 312, p.p421, p.p418);
            s.store_scaled_mul_scale_offset_rhs_ad(152, A::powf(s.ad_value(308), p.p423), 310, p.p424, 1.0, p.p422);
            s.store_scalar(153, p.p425);
            s.store_scalar(154, p.p426);
            s.store_scaled_mul_scale_offset_rhs_ad(155, A::powf(s.ad_value(308), p.p428), 310, p.p429, 1.0, p.p427);
            s.store_scalar(156, p.p431);
            s.store_scalar(157, p.p430);
            s.store_add_scaled_inputs3_offset_indices(342, 308, p.p808, 310, p.p809, 312, p.p810, p.p807);
            s.store_add_scaled_inputs3_offset_indices(343, 308, p.p812, 310, p.p813, 312, p.p814, p.p811);
            s.store_add_scaled_inputs3_mixed_aai(163, A::div_scaled_inputs2(s.ad_value(323), ((0.3333333333333333 * 1.0 / (s.v[14])) * p.p440), s.ad_value(324), p.p440, s.ad_value(322), s.v[14]), 1.0, A::div_from_scalar((p.p438 + p.p439), A::mul(s.ad_value(323), s.ad_value(321))), 1.0, 1, p.p437);
        }

        if s.b[1019] {
            s.store_scalar(164, (if (p.p442 > 0.0) { p.p442 } else { 0.0 }));
        }

        if s.b[1019] {
            s.store_scalar(165, (if (p.p443 > 0.0) { p.p443 } else { 0.0 }));
        }

        s.b[1033] = (p.p44 == 0.0);
        s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1033]) {
            s.copy_ad(165, 164);
        }

        if s.b[1019] {
            s.store_scaled_mul(166, 1, 164, p.p12);
            s.store_scaled_mul(167, 1, 165, p.p13);
            s.store_scale(168, 1, p.p445);
            s.store_scale(169, 1, p.p444);
            s.store_scale(170, 1, p.p446);
            s.store_scale(171, 1, p.p447);
        }

        s.b[1034] = (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]);
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1034]) {
            s.store_add_scaled_inputs3_offset_indices(40, 308, p.p449, 310, p.p450, 312, p.p451, p.p448);
        }

        s.b[1035] = (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1035]) {
            s.store_add_scaled_inputs3_offset_indices(41, 308, p.p453, 310, p.p454, 312, p.p455, p.p452);
        }

        s.b[1036] = (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1036]) {
            s.store_add_scaled_inputs3_offset_indices(45, 308, p.p457, 310, p.p458, 312, p.p459, p.p456);
        }

        s.b[1037] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1037]) {
            s.store_add_scaled_inputs3_offset_indices(46, 308, p.p461, 310, p.p462, 312, p.p463, p.p460);
        }

        s.b[1038] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1038]) {
            s.store_add_scaled_inputs3_offset_indices(47, 308, p.p465, 310, p.p466, 312, p.p467, p.p464);
        }

        s.b[1039] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1039]) {
            s.store_add_scaled_inputs3_offset_indices(49, 308, p.p469, 310, p.p470, 312, p.p471, p.p468);
        }

        s.b[1040] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1040]) {
            s.store_add_scaled_inputs3_offset_indices(50, 308, p.p473, 310, p.p474, 312, p.p475, p.p472);
        }

        s.b[1041] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1041]) {
            s.store_add_scaled_inputs3_offset_indices(57, 308, p.p477, 310, p.p478, 312, p.p479, p.p476);
        }

        s.b[1042] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1042]) {
            s.store_add_scaled_inputs3_offset_indices(58, 308, p.p481, 310, p.p482, 312, p.p483, p.p480);
        }

        s.b[1043] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1043]) {
            s.store_add_scaled_inputs3_offset_indices(51, 308, p.p485, 310, p.p486, 312, p.p487, p.p484);
        }

        s.b[1044] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1044]) {
            s.store_add_scaled_inputs3_offset_indices(52, 308, p.p493, 310, p.p494, 312, p.p495, p.p492);
        }

        s.b[1045] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset_indices(53, 308, p.p489, 310, p.p490, 312, p.p491, p.p488);
        }

        s.b[1046] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset_indices(54, 308, p.p497, 310, p.p498, 312, p.p499, p.p496);
        }

        s.b[1047] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1047]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(62, 309, s.ad_value(308), p.p501, s.ad_value(310), p.p502, s.ad_value(312), p.p503, p.p500);
        }

        s.b[1048] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset_indices(63, 308, p.p509, 310, p.p510, 312, p.p511, p.p508);
        }

        s.b[1049] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset_indices(64, 308, p.p505, 310, p.p506, 312, p.p507, p.p504);
        }

        s.b[1050] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1050]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(59, 309, s.ad_value(308), p.p513, s.ad_value(310), p.p514, s.ad_value(312), p.p515, p.p512);
        }

        s.b[1051] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset_indices(60, 308, p.p521, 310, p.p522, 312, p.p523, p.p520);
        }

        s.b[1052] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset_indices(61, 308, p.p517, 310, p.p518, 312, p.p519, p.p516);
        }

        s.b[1053] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1053]) {
            s.store_mul_ad(65, A::div(s.ad_value(307), s.ad_value(306)), A::add_scaled_inputs3_offset(s.ad_value(308), p.p525, s.ad_value(310), p.p526, s.ad_value(312), p.p527, p.p524));
        }

        s.b[1054] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset_indices(66, 308, p.p529, 310, p.p530, 312, p.p531, p.p528);
        }

        s.b[1055] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset_indices(67, 308, p.p533, 310, p.p534, 312, p.p535, p.p532);
        }

        s.b[1056] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset_indices(69, 308, p.p537, 310, p.p538, 312, p.p539, p.p536);
        }

        s.b[1057] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1057]) {
            s.store_add_scaled_inputs3_offset_indices(71, 308, p.p541, 310, p.p542, 312, p.p543, p.p540);
        }

        s.b[1058] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1058]) {
            s.store_add_scaled_inputs3_offset_indices(73, 308, p.p545, 310, p.p546, 312, p.p547, p.p544);
        }

        s.b[1059] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset_indices(75, 308, p.p549, 310, p.p550, 312, p.p551, p.p548);
        }

        s.b[1060] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1060]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(78, 310, s.ad_value(308), p.p553, s.ad_value(310), p.p554, s.ad_value(312), p.p555, p.p552);
        }

        s.b[1061] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1061]) {
            s.store_add_scaled_inputs3_offset_indices(79, 308, p.p557, 310, p.p558, 312, p.p559, p.p556);
        }

        s.b[1062] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset_indices(80, 308, p.p561, 310, p.p562, 312, p.p563, p.p560);
        }

        s.b[1063] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset_indices(81, 308, p.p565, 310, p.p566, 312, p.p567, p.p564);
        }

        s.b[1064] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1064]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 308, s.ad_value(308), p.p569, s.ad_value(310), p.p570, s.ad_value(312), p.p571, p.p568);
        }

        s.b[1065] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset_indices(83, 308, p.p573, 310, p.p574, 312, p.p575, p.p572);
        }

        s.b[1066] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1066]) {
            s.store_add_scaled_inputs3_offset_indices(84, 308, p.p577, 310, p.p578, 312, p.p579, p.p576);
        }

        s.b[1067] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset_indices(85, 308, p.p581, 310, p.p582, 312, p.p583, p.p580);
        }

        s.b[1068] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset_indices(87, 308, p.p585, 310, p.p586, 312, p.p587, p.p584);
        }

        s.b[1069] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1069]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(88, 308, s.ad_value(308), p.p589, s.ad_value(310), p.p590, s.ad_value(312), p.p591, p.p588);
        }

        s.b[1070] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset_indices(89, 308, p.p593, 310, p.p594, 312, p.p595, p.p592);
        }

        s.b[1071] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1071]) {
            s.store_add_scaled_inputs3_offset_indices(90, 308, p.p597, 310, p.p598, 312, p.p599, p.p596);
        }

        s.b[1072] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset_indices(92, 308, p.p601, 310, p.p602, 312, p.p603, p.p600);
        }

        s.b[1073] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1073]) {
            s.store_add_scaled_inputs3_offset_indices(94, 308, p.p605, 310, p.p606, 312, p.p607, p.p604);
        }

        s.b[1074] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1074]) {
            s.store_add_scaled_inputs3_offset_indices(95, 308, p.p609, 310, p.p610, 312, p.p611, p.p608);
        }

        s.b[1075] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1075]) {
            s.store_add_scaled_inputs3_offset_indices(96, 308, p.p613, 310, p.p614, 312, p.p615, p.p612);
        }

        s.b[1076] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1076]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(99, 313, s.ad_value(308), p.p617, s.ad_value(310), p.p618, s.ad_value(312), p.p619, p.p616);
        }

        s.b[1077] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1077]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(100, 311, s.ad_value(308), p.p621, s.ad_value(310), p.p622, s.ad_value(312), p.p623, p.p620);
        }

        s.b[1078] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1078]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(101, 311, s.ad_value(308), p.p625, s.ad_value(310), p.p626, s.ad_value(312), p.p627, p.p624);
        }

        s.b[1079] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset_indices(102, 308, p.p629, 310, p.p630, 312, p.p631, p.p628);
        }

        s.b[1080] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1080]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(110, 311, s.ad_value(308), p.p633, s.ad_value(310), p.p634, s.ad_value(312), p.p635, p.p632);
        }

        s.b[1081] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1081]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(111, 311, s.ad_value(308), p.p637, s.ad_value(310), p.p638, s.ad_value(312), p.p639, p.p636);
        }

        s.b[1082] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1019] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset_indices(114, 308, p.p641, 310, p.p642, 312, p.p643, p.p640);
        }

        s.b[1083] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset_indices(115, 308, p.p645, 310, p.p646, 312, p.p647, p.p644);
        }

        s.b[1084] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1084]) {
            s.store_mul_ad_affine_product_rhs(118, 316, s.ad_value(314), A::add_scaled_inputs3_offset(s.ad_value(308), p.p649, s.ad_value(310), p.p650, s.ad_value(312), p.p651, p.p648), 1.0 / (1e-6), 0.0);
        }

        s.b[1085] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1085]) {
            s.store_add_scaled_inputs3_offset_indices(119, 308, p.p653, 310, p.p654, 312, p.p655, p.p652);
        }

        s.b[1086] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1086]) {
            s.store_add_scaled_inputs3_offset_indices(120, 308, p.p657, 310, p.p658, 312, p.p659, p.p656);
        }

        s.b[1087] = (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(28, p.p568);
        }

        s.b[1088] = param_given[660];
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1088]) {
            s.store_scalar(28, p.p660);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(29, p.p569);
        }

        s.b[1089] = param_given[661];
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1089]) {
            s.store_scalar(29, p.p661);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(30, p.p570);
        }

        s.b[1090] = param_given[662];
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1090]) {
            s.store_scalar(30, p.p662);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_scalar(31, p.p571);
        }

        s.b[1091] = param_given[663];
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1087]) && s.b[1091]) {
            s.store_scalar(31, p.p663);
        }

        if (s.b[1019] && s.b[1087]) {
            s.store_mul_ad_rhs(121, 308, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(308), 1.0, s.ad_value(30), s.ad_value(310), 1.0, s.ad_value(31), s.ad_value(312), 1.0));
        }

        s.b[1092] = (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]);
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(28, p.p584);
        }

        s.b[1093] = param_given[664];
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1093]) {
            s.store_scalar(28, p.p664);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(29, p.p585);
        }

        s.b[1094] = param_given[665];
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1094]) {
            s.store_scalar(29, p.p665);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(30, p.p586);
        }

        s.b[1095] = param_given[666];
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1095]) {
            s.store_scalar(30, p.p666);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_scalar(31, p.p587);
        }

        s.b[1096] = param_given[667];
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1092]) && s.b[1096]) {
            s.store_scalar(31, p.p667);
        }

        if (s.b[1019] && s.b[1092]) {
            s.store_add_scaled_value_products3_indices(122, 28, 1.0, 29, 308, 1.0, 30, 310, 1.0, 31, 312, 1.0);
        }

        s.b[1097] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1097]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(123, 308, s.ad_value(308), p.p669, s.ad_value(310), p.p670, s.ad_value(312), p.p671, p.p668);
        }

        s.b[1098] = (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1098]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(124, 308, s.ad_value(308), p.p673, s.ad_value(310), p.p674, s.ad_value(312), p.p675, p.p672);
        }

        s.b[1099] = (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1099]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(125, 316, s.ad_value(308), p.p677, s.ad_value(310), p.p678, s.ad_value(312), p.p679, p.p676);
        }

        s.b[1100] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1100]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(126, 316, s.ad_value(308), p.p681, s.ad_value(310), p.p682, s.ad_value(312), p.p683, p.p680);
        }

        s.b[1101] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1101]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 319, s.ad_value(308), p.p685, s.ad_value(310), p.p686, s.ad_value(312), p.p687, p.p684);
        }

        s.b[1102] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1102]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(131, 316, s.ad_value(308), p.p689, s.ad_value(310), p.p690, s.ad_value(312), p.p691, p.p688);
        }

        s.b[1103] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1103]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(132, 316, s.ad_value(308), p.p693, s.ad_value(310), p.p694, s.ad_value(312), p.p695, p.p692);
        }

        s.b[1104] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1104]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 320, s.ad_value(308), p.p697, s.ad_value(310), p.p698, s.ad_value(312), p.p699, p.p696);
        }

        s.b[1105] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1105]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(138, 320, s.ad_value(308), p.p701, s.ad_value(310), p.p702, s.ad_value(312), p.p703, p.p700);
        }

        s.b[1106] = (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1106]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(140, 309, s.ad_value(308), p.p705, s.ad_value(310), p.p706, s.ad_value(312), p.p707, p.p704);
        }

        s.b[1110] = (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1110]) {
            s.store_add_scaled_inputs3_offset_indices(145, 308, p.p721, 310, p.p722, 312, p.p723, p.p720);
        }

        s.b[1111] = (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1111]) {
            s.store_add_scaled_inputs3_offset_indices(146, 308, p.p725, 310, p.p726, 312, p.p727, p.p724);
        }

        s.b[1112] = (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1112]) {
            s.store_add_scaled_inputs3_offset_indices(147, 308, p.p729, 310, p.p730, 312, p.p731, p.p728);
        }

        s.b[1113] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1113]) {
            s.store_add_scaled_inputs3_offset_indices(148, 308, p.p733, 310, p.p734, 312, p.p735, p.p732);
        }

        s.b[1114] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1114]) {
            s.store_add_scaled_inputs3_offset_indices(149, 308, p.p737, 310, p.p738, 312, p.p739, p.p736);
        }

        s.b[1115] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1115]) {
            s.store_mul_ad(150, A::div(s.ad_value(338), s.ad_value(306)), A::add_scaled_inputs3_offset(s.ad_value(308), p.p741, s.ad_value(310), p.p742, s.ad_value(312), p.p743, p.p740));
        }

        s.b[1116] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1116]) {
            s.store_add_scaled_inputs3_offset_indices(151, 308, p.p745, 310, p.p746, 312, p.p747, p.p744);
        }

        s.b[1117] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1117]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(152, 309, s.ad_value(308), p.p749, s.ad_value(310), p.p750, s.ad_value(312), p.p751, p.p748);
        }

        s.b[1118] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1118]) {
            s.store_add_scaled_inputs3_offset_indices(153, 308, p.p753, 310, p.p754, 312, p.p755, p.p752);
        }

        s.b[1119] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1119]) {
            s.store_add_scaled_inputs3_offset_indices(154, 308, p.p757, 310, p.p758, 312, p.p759, p.p756);
        }

        s.b[1120] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1120]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(155, 309, s.ad_value(308), p.p761, s.ad_value(310), p.p762, s.ad_value(312), p.p763, p.p760);
        }

        s.b[1121] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1121]) {
            s.store_add_scaled_inputs3_offset_indices(156, 308, p.p769, 310, p.p770, 312, p.p771, p.p768);
        }

        s.b[1122] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1122]) {
            s.store_add_scaled_inputs3_offset_indices(157, 308, p.p765, 310, p.p766, 312, p.p767, p.p764);
        }

        if s.b[1019] {
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(39, p.p788);
        }

        s.b[1126] = param_given[789];
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1126]) {
            s.store_scalar(39, p.p789);
        }

        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (s.v[1] - 0.5);
            let assign9160_cond_e8971: f64 = if ((s.b[1019] && s.b[1127]) && (s.v[1007] < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1019] && s.b[1127]) {
                s.store_add_ad_rhs(1008, 1008, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));
                s.store_add_ad_rhs(1009, 1009, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1007), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));
                s.store_offset(1007, 1007, 1.0);
            }
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_mul(992, 1008, 2);
            s.store_mul(993, 1009, 2);
            s.store_scalar(994, (1.0 / (p.p784 + (0.5 * s.v[3]))));
            s.store_scalar(995, (1.0 / (p.p785 + (0.5 * s.v[3]))));
        }

        if (s.b[1019] && s.b[1127]) {
            if ((s.v[3] + s.v[304]) > 1e-9) {
                s.store_offset(1005, 304, s.v[3]);
            } else {
                s.store_scalar(1005, 1e-9);
            }
        }

        if (s.b[1019] && s.b[1127]) {
            if (((s.v[4] + s.v[305]) + p.p786) > 1e-9) {
                s.store_offset_add(1006, 4, 305, p.p786);
            } else {
                s.store_scalar(1006, 1e-9);
            }
        }

        if (s.b[1019] && s.b[1127]) {
            s.store_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p794);
            s.store_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p795);
            s.store_add_scaled_inputs_product_first_ad(996, A::scale_offset(s.ad_value(1003), p.p791, 1.0), (1.0 + (p.p790 * (s.v[346] - 1.0))), 1004, (p.p792 * (1.0 + (p.p790 * (s.v[346] - 1.0)))), 1003, 1004, (p.p793 * (1.0 + (p.p790 * (s.v[346] - 1.0)))));
            s.store_div_scaled_inputs2_indices(997, 992, p.p787, 993, p.p787, 996, 1.0);
            s.store_div_scaled_inputs2_indices(998, 994, p.p787, 995, p.p787, 996, 1.0);
            s.store_div_from_scalar_powf_ad(1003, 1.0, s.ad_value(1005), p.p800);
            s.store_div_from_scalar_powf_ad(1004, 1.0, s.ad_value(1006), p.p801);
            s.store_add_scaled_inputs_product_first_ad(999, A::scale_offset(s.ad_value(1003), p.p797, 1.0), 1.0, 1004, p.p798, 1003, 1004, p.p799);
            s.store_add_scaled_inputs4_indices(1001, 992, 1.0, 993, 1.0, 994, -1.0, 995, -1.0);
            s.store_div_scaled_offset_numerator(1002, s.ad_value(997), 1.0, 1.0, A::offset(s.ad_value(998), 1.0), 1.0);
            s.store_mul(65, 65, 1002);
            s.store_div_scaled_product3_mixed_iiaa(82, 82, 1002, A::scale_offset(s.ad_value(998), p.p788, 1.0), 1.0, A::scale_offset(s.ad_value(997), p.p788, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(121, 121, 1002, A::offset(A::mul(s.ad_value(39), s.ad_value(998)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(997)), 1.0), 1.0);
            s.store_mul(150, 150, 1002);
            s.store_div_scaled_inputs_indices(1002, 1001, p.p796, 999, 1.0);
            s.store_add(40, 40, 1002);
            s.store_add(145, 145, 1002);
            s.store_div_scaled_inputs_mixed_ia(1002, 1001, p.p802, A::powf(s.ad_value(999), p.p803), 1.0);
            s.store_add(62, 62, 1002);
            s.store_add(155, 155, 1002);
        }

        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if ((s.b[1019] && s.b[1128]) && s.b[1129]) {
            s.store_offset(1001, 4, s.v[8]);
            s.store_scalar(1002, (1.0 / p.p804));
            s.store_div_from_scalar_scaled_input(11, (p.p804 * p.p804), 1001, s.v[8]);
            s.store_div_scaled_add_product(12, A::exp_scaled_input(s.ad_value(1002), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p804)), A::scale_offset(s.ad_value(1001), 0.1, (0.01 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-10.0), s.ad_value(1002))), (-1.0), s.ad_value(4), 1.0);
            s.store_div_scaled_add_product(13, A::exp_scaled_input(s.ad_value(1002), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p804)), A::scale_offset(s.ad_value(1001), 0.05, (0.0025 * p.p804)), A::exp(A::mul_scaled_lhs(s.ad_value(1001), (-20.0), s.ad_value(1002))), (-1.0), s.ad_value(4), 1.0);
        }

        if (s.b[1019] && s.b[1128]) {
            s.store_add_scaled_inputs3_indices(1001, 11, 1.0, 12, p.p805, 13, p.p806);
            s.store_add_scaled_product_indices(40, 40, 1.0, 342, 1001, 1.0);
            s.store_mul_offset_ad_rhs(65, 65, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0);
            s.store_add_scaled_product_indices(145, 145, 1.0, 342, 1001, 1.0);
            s.store_mul_offset_ad_rhs(150, 150, A::mul(s.ad_value(343), s.ad_value(1001)), 1.0);
        }

        s.copy_ad(172, 40);

        s.copy_ad(173, 41);

        s.copy_ad(174, 42);

        s.copy_ad(176, 43);

        s.copy_ad(177, 44);

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
    ) {
        if (s.v[45] > 1e20) {
            if (s.v[45] < 1e26) {
                s.copy_ad(178, 45);
            } else {
                s.store_scalar(178, 1e26);
            }
        } else {
            s.store_scalar(178, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(179, 46);
        } else {
            s.store_scalar(179, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(180, 47);
        } else {
            s.store_scalar(180, 0.0);
        }

        s.copy_ad(181, 48);

        s.copy_ad(182, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(183, 50);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(187, 55);

        s.copy_ad(188, 56);

        if (s.v[57] > 1e23) {
            if (s.v[57] < 1e27) {
                s.copy_ad(189, 57);
            } else {
                s.store_scalar(189, 1e27);
            }
        } else {
            s.store_scalar(189, 1e23);
        }

        if (s.v[58] > 1e23) {
            if (s.v[58] < 1e27) {
                s.copy_ad(190, 58);
            } else {
                s.store_scalar(190, 1e27);
            }
        } else {
            s.store_scalar(190, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(184, 51);
        } else {
            s.store_scalar(184, 0.0);
        }

        if (s.v[53] > 0.0) {
            if (s.v[53] < 0.5) {
                s.copy_ad(186, 53);
            } else {
                s.store_scalar(186, 0.5);
            }
        } else {
            s.store_scalar(186, 0.0);
        }

        if (s.v[52] > 0.0) {
            if (s.v[52] < 1.0) {
                s.copy_ad(185, 52);
            } else {
                s.store_scalar(185, 1.0);
            }
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(175, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(191, 62);
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[64] > 0.0) {
            if (s.v[64] < 1.0) {
                s.copy_ad(193, 64);
            } else {
                s.store_scalar(193, 1.0);
            }
        } else {
            s.store_scalar(193, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(192, 63);
        } else {
            s.store_scalar(192, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(194, 59);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[61] > 0.0) {
            if (s.v[61] < 1.0) {
                s.copy_ad(195, 61);
            } else {
                s.store_scalar(195, 1.0);
            }
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(196, 60);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(197, 65);
        } else {
            s.store_scalar(197, 0.0);
        }

        s.copy_ad(198, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(199, 67);
        } else {
            s.store_scalar(199, 0.0);
        }

        s.copy_ad(200, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(201, 69);
        } else {
            s.store_scalar(201, 0.0);
        }

        s.copy_ad(202, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(203, 71);
        } else {
            s.store_scalar(203, 0.0);
        }

        s.copy_ad(204, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(205, 73);
        } else {
            s.store_scalar(205, 0.0);
        }

        s.copy_ad(206, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(207, 75);
        } else {
            s.store_scalar(207, 0.0);
        }

        s.copy_ad(208, 76);

        s.copy_ad(209, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(210, 78);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 79);

        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(212, 80);
            } else {
                s.store_scalar(212, 1.0);
            }
        } else {
            s.store_scalar(212, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(213, 81);
        } else {
            s.store_scalar(213, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(214, 82);
        } else {
            s.store_scalar(214, 0.0);
        }

        s.copy_ad(215, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(216, 84);
            } else {
                s.store_scalar(216, 1.0);
            }
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(217, 85);
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(218, 86);
        } else {
            s.store_scalar(218, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(219, 87);
        } else {
            s.store_scalar(219, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(220, 88);
        } else {
            s.store_scalar(220, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(221, 89);
        } else {
            s.store_scalar(221, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(222, 90);
        } else {
            s.store_scalar(222, 0.0);
        }

        s.copy_ad(223, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(224, 92);
        } else {
            s.store_scalar(224, 0.0);
        }

        s.copy_ad(225, 93);

        s.copy_ad(226, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(227, 95);
        } else {
            s.store_scalar(227, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(228, 96);
        } else {
            s.store_scalar(228, 0.0);
        }

        if (s.v[97] > 1e-12) {
            s.copy_ad(229, 97);
        } else {
            s.store_scalar(229, 1e-12);
        }

        s.copy_ad(230, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(231, 99);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(232, 100);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(233, 101);
        } else {
            s.store_scalar(233, 0.0);
        }

        s.copy_ad(234, 102);

        s.copy_ad(235, 103);

        s.copy_ad(236, 104);

        s.copy_ad(237, 105);

        s.copy_ad(238, 106);

        s.copy_ad(239, 107);

        s.copy_ad(240, 108);

        s.copy_ad(241, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(242, 110);
        } else {
            s.store_scalar(242, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(243, 111);
        } else {
            s.store_scalar(243, 0.0);
        }

        s.copy_ad(244, 112);

        s.copy_ad(245, 113);

        s.copy_ad(246, 114);

        s.copy_ad(247, 115);

        s.copy_ad(248, 116);

        s.copy_ad(249, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(250, 118);
        } else {
            s.store_scalar(250, 0.0);
        }

        s.copy_ad(251, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(252, 120);
        } else {
            s.store_scalar(252, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(253, 121);
        } else {
            s.store_scalar(253, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(254, 122);
        } else {
            s.store_scalar(254, 2.0);
        }

        s.copy_ad(255, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(256, 124);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(257, 125);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(258, 126);
        } else {
            s.store_scalar(258, 0.0);
        }

        s.copy_ad(259, 127);

        s.copy_ad(260, 128);

        s.copy_ad(261, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(262, 130);
        } else {
            s.store_scalar(262, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(263, 131);
        } else {
            s.store_scalar(263, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(264, 132);
        } else {
            s.store_scalar(264, 0.0);
        }

        s.copy_ad(265, 133);

        s.copy_ad(266, 134);

        s.copy_ad(267, 135);

        s.copy_ad(268, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(269, 137);
        } else {
            s.store_scalar(269, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(270, 138);
        } else {
            s.store_scalar(270, 0.0);
        }

        s.copy_ad(271, 139);

        if (s.v[140] > 0.0) {
            s.copy_ad(272, 140);
        } else {
            s.store_scalar(272, 0.0);
        }

        s.copy_ad(277, 145);

        s.copy_ad(278, 146);

        s.copy_ad(279, 147);

        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(280, 148);
            } else {
                s.store_scalar(280, 1e26);
            }
        } else {
            s.store_scalar(280, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(281, 149);
        } else {
            s.store_scalar(281, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(282, 150);
        } else {
            s.store_scalar(282, 0.0);
        }

        s.copy_ad(283, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(284, 152);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[153] > 0.0) {
            if (s.v[153] < 1.0) {
                s.copy_ad(285, 153);
            } else {
                s.store_scalar(285, 1.0);
            }
        } else {
            s.store_scalar(285, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[154] > 0.0) {
            s.copy_ad(286, 154);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[155] > 0.0) {
            s.copy_ad(287, 155);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(289, 157);
            } else {
                s.store_scalar(289, 1.0);
            }
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(288, 156);
        } else {
            s.store_scalar(288, 0.0);
        }

        if (s.v[163] > 0.0) {
            s.copy_ad(295, 163);
        } else {
            s.store_scalar(295, 0.0);
        }

        s.copy_ad(296, 166);

        s.copy_ad(297, 167);

        s.copy_ad(298, 169);

        s.copy_ad(299, 170);

        s.copy_ad(300, 171);

        s.copy_ad(301, 168);

        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }

        s.v[16] = p.p16;

        s.v[17] = p.p15;

        s.v[18] = p.p18;

        s.v[19] = p.p17;

        s.b[1130] = (p.p44 == 0.0);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.copy_ad(188, 187);
            s.copy_ad(190, 189);
            s.copy_ad(243, 242);
            s.copy_ad(245, 244);
            s.copy_ad(247, 246);
            s.copy_ad(249, 248);
            s.copy_ad(233, 232);
            s.copy_ad(239, 237);
            s.copy_ad(240, 238);
            s.copy_ad(258, 257);
            s.copy_ad(260, 259);
            s.copy_ad(264, 263);
            s.copy_ad(270, 269);
        }

        s.store_scale(762, 177, 8.8541878176e-12);

        s.store_div(763, 762, 176);

        s.store_square(764, 176);

        s.store_scale(765, 763, 6.241449993689894e18);

        s.store_mul(766, 252, 178);

        if (s.v[766] > 1e20) {
            if (s.v[766] < 1e26) {
            } else {
                s.store_scalar(766, 1e26);
            }
        } else {
            s.store_scalar(766, 1e20);
        }

        s.v[767] = 0.0;

        s.b[1131] = (p.p51 > 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if s.b[1131] {
            s.store_scale_ad(767, A::powf(s.ad_value(763), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.b[1132] = (s.v[0] == (-1.0));
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1132]) {
            s.store_scale(767, 767, (7.448711 / 5.951993));
        }

        s.store_scale(768, 763, (1e-8 * 1.0 / (s.v[761])));

        s.store_scale(769, 209, 0.5);

        s.v[770] = 0.5;

        s.b[1133] = (s.v[0] == (-1.0));
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if s.b[1133] {
            s.store_scale(769, 209, 0.3333333333333333);
            s.store_scalar(770, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(219)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(771, s.ad_value(1000), (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1000, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(254)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(772, s.ad_value(1000), (-1.0), A::offset(s.ad_value(1000), (-1.0)), 1.0, {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(773, 1.0, 223);

        s.store_div(774, 762, 187);

        s.store_div(775, 762, 188);

        s.store_div_ad_lhs(776, A::sqrt_scaled_input(s.ad_value(189), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 774);

        s.store_div_ad_lhs(777, A::sqrt_scaled_input(s.ad_value(190), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[349])), 775);

        s.store_square(778, 776);

        s.store_square(779, 777);

        s.store_offset_div_ad(780, A::ln(A::offset(A::exp_scaled_input(s.ad_value(261), (0.005 * s.v[349])), (-1.0))), s.ad_value(261), (-((((((0.005 * s.v[349])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(781, A::ln_scaled_input(s.ad_value(776), 0.5), 780);

        s.store_add_ad_lhs(782, A::ln_scaled_input(s.ad_value(777), 0.5), 780);

        s.store_div_from_scalar(814, 1.0, 776);

        s.store_offset_scaled(815, 776, 3.1, 8.5);

        s.store_square(783, 815);

        s.store_scale(816, 815, 0.5);

        s.b[1134] = (s.v[814] < 0.06);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if s.b[1134] {
            s.store_scale(784, 814, 64.0);
        }

        s.b[1135] = (s.v[814] <= 0.45);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if ((!s.b[1134]) && s.b[1135]) {
            s.store_offset_scaled(784, 814, 22.0, 3.0);
        }

        s.b[1136] = (s.v[814] <= 1.6);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {
            s.store_offset_scaled(784, 814, (-7.2), 15.5);
        }

        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {
            s.copy_ad(784, 776);
        }

        s.store_add_scaled_inputs_product_right_ad(785, 816, 1.0, 778, 0.5, 776, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), (-1.0));

        s.store_div_from_scalar(814, 1.0, 777);

        s.store_offset_scaled(815, 777, 3.1, 8.5);

        s.store_square(786, 815);

        s.store_scale(816, 815, 0.5);

        s.b[1137] = (s.v[814] < 0.06);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if s.b[1137] {
            s.store_scale(787, 814, 64.0);
        }

        s.b[1138] = (s.v[814] <= 0.45);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if ((!s.b[1137]) && s.b[1138]) {
            s.store_offset_scaled(787, 814, 22.0, 3.0);
        }

        s.b[1139] = (s.v[814] <= 1.6);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {
            s.store_offset_scaled(787, 814, (-7.2), 15.5);
        }

        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {
            s.copy_ad(787, 777);
        }

        s.store_add_scaled_inputs_product_right_ad(788, 816, 1.0, 779, 0.5, 777, A::sqrt(A::add_scaled_inputs3(s.ad_value(816), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), (-1.0));

        s.store_add_scaled_inputs_ad(722, A::offset(s.ad_value(182), s.v[356]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(178), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]));

        if (!(s.v[722] > 0.05)) {
            s.store_scalar(722, 0.05);
        }

        s.store_div_ad_lhs(723, A::sqrt_scaled_input(s.ad_value(178), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);

        s.v[724] = 0.0;

        s.v[725] = 0.0;

        s.b[1140] = (s.v[183] > 0.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_div_from_scalar(726, 80000000.0, 764);
        }

        if s.b[1140] {
            if (s.v[183] > s.v[726]) {
                s.copy_ad(725, 183);
            } else {
                s.copy_ad(725, 726);
            }
        }

        if s.b[1140] {
            if (5e24 > s.v[725]) {
                s.store_scalar(725, 5e24);
            } else {
            }
        }

        if s.b[1140] {
            s.store_div_scaled_product_indices(724, 763, 763, (2.0 * s.v[709]), 725, (1.6021918e-19 * s.v[761]));
        }

        s.v[727] = ((100.0 * s.v[709]) * s.v[709]);

        s.b[1141] = (p.p51 > 0.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(723), s.ad_value(723), s.ad_value(722), s.v[709]));
            s.store_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);
            s.store_add(722, 722, 729);
            s.store_mul_offset_ad_rhs(723, 723, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0);
        }

        s.store_sqrt(730, 722);

        s.store_scale(731, 722, 0.95);

        s.store_scaled_mul(732, 722, 722, 0.0025);

        s.copy_ad(733, 732);

        s.store_scaled_sqrt(734, 733, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(735, 731, 0.5, 734, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(732), 1.0, A::sub(s.ad_value(731), s.ad_value(734)), A::sub(s.ad_value(731), s.ad_value(734)), 1.0), (-0.5));

        s.store_scaled_offset(736, 722, s.v[356], 0.5);

        s.store_sub_ad_lhs(737, A::sqrt(A::add(s.ad_value(180), s.ad_value(722))), 730);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(738, A::add_scaled_inputs3(s.ad_value(180), 1.0, s.ad_value(181), 1.0, s.ad_value(722), 1.0), 1.0, 730, (-1.0), 737, -1.0);

        s.store_add_scaled_inputs3_offset_mixed_iia(739, 182, 1.0, 251, 1.0, A::ln_scaled_input(A::mul(s.ad_value(766), A::powf(s.ad_value(357), (-0.75))), 4e-26), (2.0 * s.v[709]), s.v[356]);

        if (!(s.v[739] > 0.05)) {
            s.store_scalar(739, 0.05);
        }

        s.store_div_ad_lhs(740, A::sqrt_scaled_input(s.ad_value(766), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);

        s.b[1142] = (p.p51 > 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_sqrt_ad(728, A::mul3_scaled_output(s.ad_value(740), s.ad_value(740), s.ad_value(739), s.v[709]));
            s.store_mul_scaled_powf_rhs(729, 767, 0.75, 728, 0.6666666666666666);
            s.store_add(739, 739, 729);
            s.store_mul_offset_ad_rhs(740, 740, A::div_scaled_inputs(s.ad_value(729), (2.0 * 0.6666666666666666), s.ad_value(728), 1.0), 1.0);
        }

        s.store_scale(741, 739, 0.95);

        s.store_scaled_mul(742, 739, 739, 0.0025);

        s.copy_ad(743, 742);

        s.store_scaled_sqrt(734, 743, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(744, 741, 0.5, 734, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(742), 1.0, A::sub(s.ad_value(741), s.ad_value(734)), A::sub(s.ad_value(741), s.ad_value(734)), 1.0), (-0.5));

        s.store_offset_add_scaled_product(694, s.ad_value(172), 1.0, s.ad_value(173), A::scale_offset(s.ad_value(174), s.v[352], 1.0), s.v[352], s.v[17]);

        s.store_exp_scaled_input(745, 175, s.v[354]);

        s.store_mul(695, 184, 745);

        s.store_scale(696, 185, 1.0 / (s.v[353]));

        s.store_exp_scaled_input(746, 198, s.v[354]);

        s.store_mul(697, 197, 746);

        s.store_scaled_mul(710, 697, 763, s.v[16]);

        s.store_mul_ad_rhs(699, 201, A::exp_scaled_input(s.ad_value(202), s.v[354]));

        s.store_exp_scaled_input(747, 200, s.v[354]);

        s.store_mul(698, 199, 747);

        s.store_mul_ad_rhs(701, 205, A::exp_scaled_input(s.ad_value(206), s.v[354]));

        s.store_exp_scaled_input(748, 204, s.v[354]);

        s.store_mul(700, 203, 748);

        s.store_exp_scaled_input(749, 208, s.v[354]);

        s.store_mul(702, 207, 749);

        s.store_exp_scaled_input(750, 211, s.v[354]);

        s.store_mul(703, 210, 750);

        s.store_scaled_mul(751, 710, 703, 2.0);

        s.store_exp_scaled_input(752, 215, s.v[354]);

        s.store_mul(714, 214, 752);

        s.store_mul(715, 253, 752);

        s.store_mul_ad_rhs(706, 225, A::exp_scaled_input(s.ad_value(226), (-s.v[354])));

        s.store_scale(713, 271, (4.0 * (1.3806505e-23 * s.v[350])));

        s.b[1143] = ((p.p46 != 0.0) && (s.v[282] > 0.0));
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_offset_add_scaled_inputs_indices(707, 277, 1.0, 278, s.v[352], s.v[19]);
            s.store_exp_scaled_input(753, 283, s.v[354]);
            s.store_mul(708, 282, 753);
            s.store_scaled_mul(711, 708, 763, s.v[18]);
            s.store_offset_scaled(717, 281, ((s.v[353]) * (s.v[709])), s.v[709]);
            s.store_add_scaled_product_mixed_aia(754, A::offset(s.ad_value(279), s.v[356]), 1.0, 717, A::ln_scaled_input(A::mul(s.ad_value(280), A::powf(s.ad_value(357), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1143] {
            if (s.v[754] > 0.05) {
            } else {
                s.store_scalar(754, 0.05);
            }
        }

        if s.b[1143] {
            s.store_div_ad_lhs(755, A::sqrt_scaled_input(s.ad_value(280), (((2.0 * 1.6021918e-19) * s.v[761]) * s.v[355])), 763);
            s.store_square(718, 755);
            s.store_ln(719, 718);
            s.store_scale(756, 754, 0.95);
            s.store_scaled_mul(757, 754, 754, 0.0025);
            s.copy_ad(758, 757);
            s.store_scaled_sqrt(759, 758, 0.5);
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1143] {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(760, 756, 0.5, 759, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(757), 1.0, A::sub(s.ad_value(756), s.ad_value(759)), A::sub(s.ad_value(756), s.ad_value(759)), 1.0), (-0.5));
        }

        if (!s.b[1143]) {
            s.store_scalar(707, 0.0);
            s.store_scalar(753, 1.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(717, s.v[709]);
            s.store_scalar(754, 0.0);
            s.store_scalar(755, 1.0);
            s.store_scalar(718, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(756, 0.0);
            s.store_scalar(757, 0.0);
            s.store_scalar(758, 0.0);
            s.store_scalar(759, 0.0);
            s.store_scalar(760, 0.0);
        }

        s.store_div_from_scalar(789, 1.0, 241);

        s.store_scaled_sqrt_scaled_input(790, 241, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(791, 790, 176);

        s.store_mul(792, 790, 187);

        s.store_mul(793, 790, 188);

        s.v[794] = 0.0;

        s.b[1144] = (s.v[236] < 0.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_div_scaled_inputs_indices(794, 235, (-0.495), 236, 1.0);
        }

        s.v[795] = 0.0;

        s.b[1145] = (s.v[238] < 0.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_div_scaled_inputs_indices(795, 237, (-0.495), 238, 1.0);
        }

        s.b[1146] = (s.v[240] < 0.0);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_div_scaled_inputs_indices(796, 239, (-0.495), 240, 1.0);
        }

        s.store_pow_from_scalar_ad(797, s.v[346], s.ad_value(234));

        s.store_mul(231, 231, 797);

        s.store_mul(232, 232, 797);

        s.store_mul(233, 233, 797);

        s.store_div_scaled_inputs_mixed_ia(798, 242, 4e-18, A::square(s.ad_value(187)), 1.0);

        s.store_div_scaled_inputs_mixed_ia(799, 243, 4e-18, A::square(s.ad_value(188)), 1.0);

        if ((1.0 + (s.v[246] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 246, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(704, 244, 790);

        s.store_scaled_mul(800, 704, 187, 500000000.0);

        if ((1.0 + (s.v[247] * s.v[347])) > 0.0) {
            s.store_offset_scaled(790, 247, s.v[347], 1.0);
        } else {
            s.store_scalar(790, 0.0);
        }

        s.store_mul(705, 245, 790);

        s.store_scaled_mul(801, 705, 188, 500000000.0);

        s.v[802] = 0.0;

        s.b[1147] = (s.v[267] > 1e-10);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if s.b[1147] {
            s.store_div_from_scalar(802, 0.75, 267);
        }

        s.store_square(803, 268);

        s.store_scale(804, 272, (9.1093826e-31 * 1000000000.0));

        s.b[1148] = (s.v[295] > 0.0);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_div_from_scalar(805, 1.0, 295);
        }

        if (!s.b[1148]) {
            s.store_scalar(805, 0.0);
        }

        s.b[1149] = (s.v[296] > 0.0);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if s.b[1149] {
            s.store_div_from_scalar(806, 1.0, 296);
        }

        if (!s.b[1149]) {
            s.store_scalar(806, 0.0);
        }

        s.b[1150] = (s.v[297] > 0.0);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if s.b[1150] {
            s.store_div_from_scalar(807, 1.0, 297);
        }

        if (!s.b[1150]) {
            s.store_scalar(807, 0.0);
        }

        s.b[1151] = (s.v[298] > 0.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if s.b[1151] {
            s.store_div_from_scalar(808, 1.0, 298);
        }

        if (!s.b[1151]) {
            s.store_scalar(808, 0.0);
        }

        s.b[1152] = (s.v[299] > 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_div_from_scalar(809, 1.0, 299);
        }

        if (!s.b[1152]) {
            s.store_scalar(809, 0.0);
        }

        s.b[1153] = (s.v[300] > 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_div_from_scalar(810, 1.0, 300);
        }

        if (!s.b[1153]) {
            s.store_scalar(810, 0.0);
        }

        s.b[1154] = (s.v[301] > 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_div_from_scalar(811, 1.0, 301);
        }

        if (!s.b[1154]) {
            s.store_scalar(811, 0.0);
        }

        s.store_scale(20, 2, s.v[640]);

        s.store_scale(21, 2, s.v[641]);

        s.store_scale(22, 2, s.v[642]);

        s.store_scale(23, 2, s.v[667]);

        s.store_scale(24, 2, s.v[668]);

        s.store_scale(25, 2, s.v[669]);

        s.v[26] = 0.0;

        s.b[1155] = (p.p43 == 3.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if s.b[1155] {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 307);

        s.b[1156] = (p.p39 == 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.b[1157] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_scale(20, 2, s.v[643]);
            s.store_add_scaled_product_indices(21, 2, s.v[644], 26, 27, (-1.0));
            s.copy_ad(22, 27);
            s.store_scale(23, 2, s.v[670]);
            s.store_add_scaled_product_indices(24, 2, s.v[671], 26, 27, (-1.0));
            s.copy_ad(25, 27);
        }

        s.b[1158] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            if (s.v[20] > 0.0) {
                s.copy_ad(640, 20);
            } else {
                s.store_scalar(640, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[21] > 0.0) {
                s.copy_ad(641, 21);
            } else {
                s.store_scalar(641, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[22] > 0.0) {
                s.copy_ad(642, 22);
            } else {
                s.store_scalar(642, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[23] > 0.0) {
                s.copy_ad(667, 23);
            } else {
                s.store_scalar(667, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[24] > 0.0) {
                s.copy_ad(668, 24);
            } else {
                s.store_scalar(668, 0.0);
            }
        }

        if s.b[1158] {
            if (s.v[25] > 0.0) {
                s.copy_ad(669, 25);
            } else {
                s.store_scalar(669, 0.0);
            }
        }

        if (!s.b[1158]) {
            s.store_scalar(640, 0.0);
            s.store_scalar(641, 0.0);
            s.store_scalar(642, 0.0);
            s.store_scalar(667, 0.0);
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
        }

        s.v[650] = 0.0;

        s.v[677] = 0.0;

        s.v[652] = 0.0;

        s.v[679] = 0.0;

        s.v[651] = 0.0;

        s.v[678] = 0.0;

        s.v[653] = 0.0;

        s.v[680] = 0.0;

        s.v[648] = 0.0;

        s.v[675] = 0.0;

        s.v[649] = 0.0;

        s.v[676] = 0.0;

        s.v[661] = 0.0;

        s.v[688] = 0.0;

        s.v[662] = 1.0;

        s.v[689] = 1.0;

        s.v[663] = 0.0;

        s.v[690] = 0.0;

        s.v[664] = 1.0;

        s.v[691] = 1.0;

        s.v[665] = 0.0;

        s.v[692] = 0.0;

        s.v[666] = 1.0;

        s.v[693] = 1.0;

        s.v[660] = 0.0;

        s.v[687] = 0.0;

        s.v[654] = 0.0;

        s.v[681] = 0.0;

        s.v[655] = 0.0;

        s.v[682] = 0.0;

        s.v[656] = 0.0;

        s.v[683] = 0.0;

        s.v[657] = 0.0;

        s.v[684] = 0.0;

        s.v[658] = 0.0;

        s.v[685] = 0.0;

        s.v[659] = 0.0;

        s.v[686] = 0.0;

        s.v[645] = 1.0;

        s.v[672] = 1.0;

        s.v[646] = 1.0;

        s.v[673] = 1.0;

        s.v[647] = 1.0;

        s.v[674] = 1.0;

        s.v[485] = 0.0;

        s.v[486] = 0.0;

        s.v[474] = 0.0;

        s.v[475] = 0.0;

        s.v[476] = 0.0;

        s.v[477] = 0.0;

        s.v[478] = 0.0;

        s.v[487] = 0.0;

        s.v[488] = 0.0;

        s.v[489] = 0.0;

        s.v[495] = 0.0;

        s.v[484] = 0.0;

        s.b[1159] = (p.p43 > 0.0);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        s.b[1160] = ((s.v[381] * s.v[640]) > 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1160]) {
            s.store_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(640), s.v[381])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_scalar(448, 100000000.0);
        }

        s.b[1161] = ((s.v[382] * s.v[641]) > 0.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1161]) {
            s.store_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(641), s.v[382])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1161])) {
            s.store_scalar(449, 100000000.0);
        }

        s.b[1162] = ((s.v[383] * s.v[642]) > 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1162]) {
            s.store_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(642), s.v[383])), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1162])) {
            s.store_scalar(450, 100000000.0);
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1159] {
            s.store_min3(648, 448, 449, 450);
        }

        s.b[1163] = ((((s.v[648] * s.v[365])) as f64).abs() < 230.25850929940458);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1163]) {
            s.store_exp_scaled_input(649, 648, s.v[365]);
        }

        s.b[1164] = ((s.v[648] * s.v[365]) < 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(649, 1e-100, (-230.25850929940458), A::scale(s.ad_value(648), s.v[365]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {
            s.store_scaled_offset_ad(649, A::mul_offset_rhs(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(648), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(648), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1159] {
            s.store_scalar(390, s.v[387]);
            s.store_scalar(391, s.v[388]);
            s.store_scalar(392, s.v[389]);
            s.store_scalar(393, p.p824);
            s.store_scalar(394, p.p825);
            s.store_scalar(395, p.p826);
            s.store_scalar(396, p.p821);
            s.store_scalar(397, p.p822);
            s.store_scalar(398, p.p823);
        }

        s.b[1165] = (s.v[640] == 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1165]) {
            s.store_scalar(390, (s.v[388] + s.v[389]));
            s.store_scalar(393, (0.9 * (p.p825).min(p.p826)));
            s.store_scalar(396, (p.p822 + p.p823));
        }

        s.b[1166] = (s.v[641] == 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1166]) {
            s.store_scalar(391, (s.v[387] + s.v[389]));
            s.store_scalar(394, (0.9 * (p.p824).min(p.p826)));
            s.store_scalar(397, (p.p821 + p.p823));
        }

        s.b[1167] = (s.v[642] == 0.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1167]) {
            s.store_scalar(392, (s.v[387] + s.v[388]));
            s.store_scalar(395, (0.9 * (p.p824).min(p.p825)));
            s.store_scalar(398, (p.p821 + p.p822));
        }

        if s.b[1159] {
            s.store_min3(650, 390, 391, 392);
            s.store_scale(651, 650, 0.1);
            s.store_max3(371, 393, 394, 395);
            s.store_mul_sub_from_scalar_ad_rhs(652, 650, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))));
            s.store_offset_min_ad(653, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));
        }

        s.b[1168] = ((s.v[557] * s.v[667]) > 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1168]) {
            s.store_scaled_ln_ad(448, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(557), s.ad_value(667))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1168])) {
            s.store_scalar(448, 100000000.0);
        }

        s.b[1169] = ((s.v[558] * s.v[668]) > 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1169]) {
            s.store_scaled_ln_ad(449, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(558), s.ad_value(668))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1169])) {
            s.store_scalar(449, 100000000.0);
        }

        s.b[1170] = ((s.v[559] * s.v[669]) > 0.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1170]) {
            s.store_scaled_ln_ad(450, A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(559), s.ad_value(669))), 1.0), s.v[364]);
        }

        if (s.b[1159] && (!s.b[1170])) {
            s.store_scalar(450, 100000000.0);
        }

        if s.b[1159] {
            s.store_min3(675, 448, 449, 450);
        }

        s.b[1171] = ((((s.v[675] * s.v[365])) as f64).abs() < 230.25850929940458);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1171]) {
            s.store_exp_scaled_input(676, 675, s.v[365]);
        }

        s.b[1172] = ((s.v[675] * s.v[365]) < 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1171])) && s.b[1172]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(676, 1e-100, (-230.25850929940458), A::scale(s.ad_value(675), s.v[365]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1159] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_scaled_offset_ad(676, A::mul_offset_rhs(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(675), s.v[365], (-230.25850929940458)), A::scale_offset(s.ad_value(675), ((s.v[365]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1159] {
            s.copy_ad(390, 563);
            s.copy_ad(391, 564);
            s.copy_ad(392, 565);
            s.copy_ad(393, 505);
            s.copy_ad(394, 506);
            s.copy_ad(395, 507);
            s.copy_ad(396, 502);
            s.copy_ad(397, 503);
            s.copy_ad(398, 504);
        }

        s.b[1173] = (s.v[667] == 0.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1173]) {
            s.store_add(390, 564, 565);
            s.store_scale_ad(393, A::min(s.ad_value(506), s.ad_value(507)), 0.9);
            s.store_add(396, 503, 504);
        }

        s.b[1174] = (s.v[668] == 0.0);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1174]) {
            s.store_add(391, 563, 565);
            s.store_scale_ad(394, A::min(s.ad_value(505), s.ad_value(507)), 0.9);
            s.store_add(397, 502, 504);
        }

        s.b[1175] = (s.v[669] == 0.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1175]) {
            s.store_add(392, 563, 564);
            s.store_scale_ad(395, A::min(s.ad_value(505), s.ad_value(506)), 0.9);
            s.store_add(398, 502, 503);
        }

        if s.b[1159] {
            s.store_min3(677, 390, 391, 392);
            s.store_scale(678, 677, 0.1);
            s.store_max3(371, 393, 394, 395);
            s.store_mul_sub_from_scalar_ad_rhs(679, 677, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371))));
            s.store_offset_min_ad(680, A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398), (-0.05));
        }

        s.b[1176] = (s.v[468] == 1.0);
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1176]) {
            s.store_scalar(1177, 0.0);
            s.store_scalar(1178, 0.0);
            s.store_scalar(1179, 0.0);
            s.store_scalar(1186, 0.0);
            s.store_scalar(1188, 0.0);
            s.store_scalar(1189, 0.0);
            s.store_scalar(1190, 0.0);
            s.store_scalar(1191, 0.0);
            s.store_scalar(1192, 0.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1194, 0.0);
            s.store_scalar(1195, 0.0);
            s.store_scalar(1196, 0.0);
            s.store_scalar(1197, 0.0);
            s.store_scalar(1198, 0.0);
            s.store_scalar(1199, 0.0);
            s.store_scalar(1200, 0.0);
            s.store_scalar(1201, 0.0);
            s.store_scalar(1202, 0.0);
            s.store_scalar(1203, 0.0);
            s.store_scalar(1204, 0.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1206, 0.0);
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
            s.store_scalar(1209, 0.0);
            s.store_scalar(1210, 0.0);
            s.store_scalar(1211, 0.0);
            s.store_scalar(1212, 0.0);
            s.store_scalar(1213, 0.0);
            s.store_scalar(1214, 0.0);
            s.store_scalar(1215, 0.0);
            s.store_scalar(1216, 0.0);
            s.store_scalar(1217, 0.0);
            s.store_scalar(1218, 0.0);
            s.store_scalar(1219, 0.0);
            s.store_scalar(1220, 0.0);
            s.store_scalar(1221, 0.0);
            s.store_scalar(492, 0.4);
            s.store_scalar(493, 0.65);
            s.store_scalar(494, 0.8);
            s.store_scale(479, 492, (-p.p921));
            s.store_scale(480, 493, (-p.p921));
            s.store_scale(481, 494, (-p.p921));
            s.store_scalar(482, 0.1);
            s.store_scalar(483, 0.2);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1225] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        s.b[1226] = (s.v[479] < s.v[648]);
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        s.b[1227] = (((((-0.5) * (s.v[479] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) && s.b[1227]) {
            s.store_exp_scaled_input(1188, 479, (s.v[365] * (-0.5)));
        }

        s.b[1228] = (((-0.5) * (s.v[479] * s.v[365])) < 0.0);
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) && (!s.b[1227])) && s.b[1228]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(479), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) && (!s.b[1227])) && (!s.b[1228])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(479), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1226]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && (!s.b[1226])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(479), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && (!s.b[1226])) {
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1225]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1229] = (s.v[479] > 0.0);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && s.b[1229]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1225]) && (!s.b[1229])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 479);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1225]) {
            s.store_sub(1191, 650, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 479, 0.5, 1191, 0.5, 479, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 479, 0.5, 653, 0.5, 479, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 479, 479, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1230] = (s.v[640] == 0.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1230]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1230])) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.b[1231] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1231]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1232] = (p.p824 == 0.5);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && s.b[1232]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && (!s.b[1232])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1233] = (p.p824 == 0.5);
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && s.b[1233]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) && (!s.b[1233])) {
            s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1231])) {
            s.store_scale(1202, 1195, s.v[417]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[378]);
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.b[1234] = (p.p838 == 0.0);
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1234]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1235] = (((-p.p824) * s.v[405]) == (-1.0));
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1235]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1235])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[429]), s.ad_value(1206), s.ad_value(1209), s.v[429], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1236] = (s.v[1216] > 0.0);
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1236]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1236])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1237] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1237]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1237])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1238] = (s.v[1216] > 0.0);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && s.b[1238]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1239] = (s.v[1215] > (-230.25850929940458));
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1238])) && s.b[1239]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1238])) && (!s.b[1239])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1238])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1234])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);
        }

        s.b[1240] = (p.p844 == 0.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1240]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1241] = (p.p824 == 0.5);
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && s.b[1241]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && (!s.b[1241])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), s.ad_value(1195), 1.0);
        }

        s.b[1242] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && s.b[1242]) {
            s.store_exp_div_scaled_inputs_indices(1195, 435, -1.0, 1220, 1.0);
        }

        s.b[1243] = (((-s.v[435]) / s.v[1220]) < 0.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && (!s.b[1242])) && s.b[1243]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) && (!s.b[1242])) && (!s.b[1243])) {
            let assign16190_ad_e14190: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign16190_ad_e14190, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1240])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844);
        }

        s.b[1244] = (p.p853 > 1000.0);
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1230])) && s.b[1244]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1245] = (s.v[1194] > ((-s.v[438]) * p.p853));
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        s.b[1246] = (p.p856 == 4.0);
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && s.b[1245]) && s.b[1246]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && s.b[1245]) && (!s.b[1246])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && s.b[1245]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1230])) && (!s.b[1244])) && (!s.b[1245])) {
            s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1230])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1247] = (s.v[641] == 0.0);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1247]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1247])) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.b[1248] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1248]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1249] = (p.p825 == 0.5);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && s.b[1249]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && (!s.b[1249])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1250] = (p.p825 == 0.5);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && s.b[1250]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) && (!s.b[1250])) {
            s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1248])) {
            s.store_scale(1202, 1195, s.v[418]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[379]);
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.b[1251] = (p.p839 == 0.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1251]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1252] = (((-p.p825) * s.v[406]) == (-1.0));
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1252]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1252])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[430]), s.ad_value(1206), s.ad_value(1209), s.v[430], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1253] = (s.v[1216] > 0.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1253]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1253])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1254] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1254]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1254])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1255] = (s.v[1216] > 0.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && s.b[1255]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1256] = (s.v[1215] > (-230.25850929940458));
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1255])) && s.b[1256]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1255])) && (!s.b[1256])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1255])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1251])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);
        }

        s.b[1257] = (p.p845 == 0.0);
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1257]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1258] = (p.p825 == 0.5);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && s.b[1258]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && (!s.b[1258])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), s.ad_value(1195), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1259] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && s.b[1259]) {
            s.store_exp_div_scaled_inputs_indices(1195, 436, -1.0, 1220, 1.0);
        }

        s.b[1260] = (((-s.v[436]) / s.v[1220]) < 0.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && (!s.b[1259])) && s.b[1260]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) && (!s.b[1259])) && (!s.b[1260])) {
            let assign16890_ad_e15333: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign16890_ad_e15333, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1257])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845);
        }

        s.b[1261] = (p.p854 > 1000.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1247])) && s.b[1261]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1262] = (s.v[1194] > ((-s.v[438]) * p.p854));
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        s.b[1263] = (p.p857 == 4.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && s.b[1262]) && s.b[1263]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && s.b[1262]) && (!s.b[1263])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && s.b[1262]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1247])) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1247])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1264] = (s.v[642] == 0.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1264]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1264])) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.b[1265] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1265]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1266] = (p.p826 == 0.5);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && s.b[1266]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && (!s.b[1266])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1267] = (p.p826 == 0.5);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && s.b[1267]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) && (!s.b[1267])) {
            s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1265])) {
            s.store_scale(1202, 1195, s.v[419]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[380]);
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.b[1268] = (p.p840 == 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1268]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1269] = (((-p.p826) * s.v[407]) == (-1.0));
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1269]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1269])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[431]), s.ad_value(1206), s.ad_value(1209), s.v[431], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1270] = (s.v[1216] > 0.0);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1270]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1270])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1271] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1271]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1271])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1272] = (s.v[1216] > 0.0);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && s.b[1272]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1273] = (s.v[1215] > (-230.25850929940458));
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1272])) && s.b[1273]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1272])) && (!s.b[1273])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1272])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1268])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);
        }

        s.b[1274] = (p.p846 == 0.0);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1274]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1275] = (p.p826 == 0.5);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && s.b[1275]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && (!s.b[1275])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), s.ad_value(1195), 1.0);
        }

        s.b[1276] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && s.b[1276]) {
            s.store_exp_div_scaled_inputs_indices(1195, 437, -1.0, 1220, 1.0);
        }

        s.b[1277] = (((-s.v[437]) / s.v[1220]) < 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && (!s.b[1276])) && s.b[1277]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) && (!s.b[1276])) && (!s.b[1277])) {
            let assign17590_ad_e16476: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign17590_ad_e16476, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1274])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846);
        }

        s.b[1278] = (p.p855 > 1000.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1264])) && s.b[1278]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1279] = (s.v[1194] > ((-s.v[438]) * p.p855));
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        s.b[1280] = (p.p858 == 4.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && s.b[1279]) && s.b[1280]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && s.b[1279]) && (!s.b[1280])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && s.b[1279]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1264])) && (!s.b[1278])) && (!s.b[1279])) {
            s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1264])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(469, s.ad_value(640), s.ad_value(1222), 1.0, s.ad_value(641), s.ad_value(1223), 1.0, s.ad_value(642), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1281] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        s.b[1282] = (s.v[480] < s.v[648]);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        s.b[1283] = (((((-0.5) * (s.v[480] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) && s.b[1283]) {
            s.store_exp_scaled_input(1188, 480, (s.v[365] * (-0.5)));
        }

        s.b[1284] = (((-0.5) * (s.v[480] * s.v[365])) < 0.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) && (!s.b[1283])) && s.b[1284]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(480), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) && (!s.b[1283])) && (!s.b[1284])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(480), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1282]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && (!s.b[1282])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(480), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1281]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1285] = (s.v[480] > 0.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && s.b[1285]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1281]) && (!s.b[1285])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 480);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1281]) {
            s.store_sub(1191, 650, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 480, 0.5, 1191, 0.5, 480, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 480, 0.5, 653, 0.5, 480, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 480, 480, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1286] = (s.v[640] == 0.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1286]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1286])) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.b[1287] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1287]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1288] = (p.p824 == 0.5);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && s.b[1288]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && (!s.b[1288])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1289] = (p.p824 == 0.5);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && s.b[1289]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) && (!s.b[1289])) {
            s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_scale(1202, 1195, s.v[417]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[378]);
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.b[1290] = (p.p838 == 0.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1290]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1291] = (((-p.p824) * s.v[405]) == (-1.0));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1291]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1291])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[429]), s.ad_value(1206), s.ad_value(1209), s.v[429], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1292] = (s.v[1216] > 0.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1292]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1292])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1293] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1293]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1293])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1294] = (s.v[1216] > 0.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && s.b[1294]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1295] = (s.v[1215] > (-230.25850929940458));
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1294])) && s.b[1295]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1294])) && (!s.b[1295])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1294])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1290])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);
        }

        s.b[1296] = (p.p844 == 0.0);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1296]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1297] = (p.p824 == 0.5);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && s.b[1297]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && (!s.b[1297])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), s.ad_value(1195), 1.0);
        }

        s.b[1298] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && s.b[1298]) {
            s.store_exp_div_scaled_inputs_indices(1195, 435, -1.0, 1220, 1.0);
        }

        s.b[1299] = (((-s.v[435]) / s.v[1220]) < 0.0);
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && (!s.b[1298])) && s.b[1299]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) && (!s.b[1298])) && (!s.b[1299])) {
            let assign18590_ad_e18120: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign18590_ad_e18120, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1296])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844);
        }

        s.b[1300] = (p.p853 > 1000.0);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1286])) && s.b[1300]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1301] = (s.v[1194] > ((-s.v[438]) * p.p853));
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        s.b[1302] = (p.p856 == 4.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && s.b[1301]) && s.b[1302]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && s.b[1301]) && (!s.b[1302])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && s.b[1301]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1286])) && (!s.b[1300])) && (!s.b[1301])) {
            s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1286])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1303] = (s.v[641] == 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1303]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1303])) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.b[1304] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1304]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1305] = (p.p825 == 0.5);
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && s.b[1305]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && (!s.b[1305])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1306] = (p.p825 == 0.5);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && s.b[1306]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && (!s.b[1306])) {
            s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) {
            s.store_scale(1202, 1195, s.v[418]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[379]);
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.b[1307] = (p.p839 == 0.0);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1307]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1308] = (((-p.p825) * s.v[406]) == (-1.0));
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1308]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1308])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[430]), s.ad_value(1206), s.ad_value(1209), s.v[430], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1309] = (s.v[1216] > 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1309]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1309])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1310] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1310]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1310])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1311] = (s.v[1216] > 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1311]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1312] = (s.v[1215] > (-230.25850929940458));
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1311])) && s.b[1312]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1311])) && (!s.b[1312])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1311])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);
        }

        s.b[1313] = (p.p845 == 0.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1313]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1314] = (p.p825 == 0.5);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && s.b[1314]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && (!s.b[1314])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), s.ad_value(1195), 1.0);
        }

        s.b[1315] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && s.b[1315]) {
            s.store_exp_div_scaled_inputs_indices(1195, 436, -1.0, 1220, 1.0);
        }

        s.b[1316] = (((-s.v[436]) / s.v[1220]) < 0.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && (!s.b[1315])) && s.b[1316]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && (!s.b[1315])) && (!s.b[1316])) {
            let assign19290_ad_e19263: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign19290_ad_e19263, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845);
        }

        s.b[1317] = (p.p854 > 1000.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1317]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1318] = (s.v[1194] > ((-s.v[438]) * p.p854));
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        s.b[1319] = (p.p857 == 4.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && s.b[1318]) && s.b[1319]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && s.b[1318]) && (!s.b[1319])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && s.b[1318]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && (!s.b[1318])) {
            s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1303])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1320] = (s.v[642] == 0.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1320]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1320])) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.b[1321] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1321]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1322] = (p.p826 == 0.5);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && s.b[1322]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && (!s.b[1322])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1323] = (p.p826 == 0.5);
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && s.b[1323]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && (!s.b[1323])) {
            s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) {
            s.store_scale(1202, 1195, s.v[419]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[380]);
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.b[1324] = (p.p840 == 0.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1324]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
            s.store_square(1207, 1206);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1325] = (((-p.p826) * s.v[407]) == (-1.0));
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1325]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1325])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[431]), s.ad_value(1206), s.ad_value(1209), s.v[431], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1326] = (s.v[1216] > 0.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1326]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1326])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1327] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1327]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1327])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1328] = (s.v[1216] > 0.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1328]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1329] = (s.v[1215] > (-230.25850929940458));
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1328])) && s.b[1329]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1328])) && (!s.b[1329])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1328])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);
        }

        s.b[1330] = (p.p846 == 0.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1330]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1331] = (p.p826 == 0.5);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && s.b[1331]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && (!s.b[1331])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), s.ad_value(1195), 1.0);
        }

        s.b[1332] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && s.b[1332]) {
            s.store_exp_div_scaled_inputs_indices(1195, 437, -1.0, 1220, 1.0);
        }

        s.b[1333] = (((-s.v[437]) / s.v[1220]) < 0.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && (!s.b[1332])) && s.b[1333]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && (!s.b[1332])) && (!s.b[1333])) {
            let assign19990_ad_e20406: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign19990_ad_e20406, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846);
        }

        s.b[1334] = (p.p855 > 1000.0);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1334]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1335] = (s.v[1194] > ((-s.v[438]) * p.p855));
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        s.b[1336] = (p.p858 == 4.0);
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && s.b[1335]) && s.b[1336]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && s.b[1335]) && (!s.b[1336])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && s.b[1335]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && (!s.b[1335])) {
            s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1320])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(470, s.ad_value(640), s.ad_value(1222), 1.0, s.ad_value(641), s.ad_value(1223), 1.0, s.ad_value(642), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1337] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        s.b[1338] = (s.v[481] < s.v[648]);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        s.b[1339] = (((((-0.5) * (s.v[481] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) && s.b[1339]) {
            s.store_exp_scaled_input(1188, 481, (s.v[365] * (-0.5)));
        }

        s.b[1340] = (((-0.5) * (s.v[481] * s.v[365])) < 0.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) && (!s.b[1339])) && s.b[1340]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(481), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) && (!s.b[1339])) && (!s.b[1340])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(481), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && (!s.b[1338])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(481), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1337]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1341] = (s.v[481] > 0.0);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1341]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && (!s.b[1341])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 481);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1337]) {
            s.store_sub(1191, 650, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 481, 0.5, 1191, 0.5, 481, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 481, 0.5, 653, 0.5, 481, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 481, 481, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1342] = (s.v[640] == 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1342]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1342])) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.b[1343] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1343]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1344] = (p.p824 == 0.5);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && s.b[1344]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && (!s.b[1344])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1345] = (p.p824 == 0.5);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && s.b[1345]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && (!s.b[1345])) {
            s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) {
            s.store_scale(1202, 1195, s.v[417]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[378]);
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.b[1346] = (p.p838 == 0.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1346]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1347] = (((-p.p824) * s.v[405]) == (-1.0));
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1347]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[429]), s.ad_value(1206), s.ad_value(1209), s.v[429], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1348] = (s.v[1216] > 0.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1348]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1348])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1349] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1349]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1349])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1350] = (s.v[1216] > 0.0);
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1350]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1351] = (s.v[1215] > (-230.25850929940458));
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1350])) && s.b[1351]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1350])) && (!s.b[1351])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1350])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);
        }

        s.b[1352] = (p.p844 == 0.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1352]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1353] = (p.p824 == 0.5);
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && s.b[1353]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && (!s.b[1353])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), s.ad_value(1195), 1.0);
        }

        s.b[1354] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && s.b[1354]) {
            s.store_exp_div_scaled_inputs_indices(1195, 435, -1.0, 1220, 1.0);
        }

        s.b[1355] = (((-s.v[435]) / s.v[1220]) < 0.0);
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && (!s.b[1354])) && s.b[1355]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && (!s.b[1354])) && (!s.b[1355])) {
            let assign20990_ad_e22050: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign20990_ad_e22050, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844);
        }

        s.b[1356] = (p.p853 > 1000.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1356]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1357] = (s.v[1194] > ((-s.v[438]) * p.p853));
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        s.b[1358] = (p.p856 == 4.0);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && s.b[1357]) && s.b[1358]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && s.b[1357]) && (!s.b[1358])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && s.b[1357]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && (!s.b[1357])) {
            s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1342])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1359] = (s.v[641] == 0.0);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1359]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1359])) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.b[1360] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1360]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1361] = (p.p825 == 0.5);
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && s.b[1361]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && (!s.b[1361])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1362] = (p.p825 == 0.5);
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && s.b[1362]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && (!s.b[1362])) {
            s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) {
            s.store_scale(1202, 1195, s.v[418]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[379]);
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.b[1363] = (p.p839 == 0.0);
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1363]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1364] = (((-p.p825) * s.v[406]) == (-1.0));
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1364]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1364])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[430]), s.ad_value(1206), s.ad_value(1209), s.v[430], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1365] = (s.v[1216] > 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1365]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1365])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1366] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1366]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1366])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1367] = (s.v[1216] > 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1367]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1368] = (s.v[1215] > (-230.25850929940458));
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1367])) && s.b[1368]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1367])) && (!s.b[1368])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1367])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);
        }

        s.b[1369] = (p.p845 == 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1369]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1370] = (p.p825 == 0.5);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && s.b[1370]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && (!s.b[1370])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), s.ad_value(1195), 1.0);
        }

        s.b[1371] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && s.b[1371]) {
            s.store_exp_div_scaled_inputs_indices(1195, 436, -1.0, 1220, 1.0);
        }

        s.b[1372] = (((-s.v[436]) / s.v[1220]) < 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && (!s.b[1371])) && s.b[1372]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && (!s.b[1371])) && (!s.b[1372])) {
            let assign21690_ad_e23193: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign21690_ad_e23193, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845);
        }

        s.b[1373] = (p.p854 > 1000.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1373]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1374] = (s.v[1194] > ((-s.v[438]) * p.p854));
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        s.b[1375] = (p.p857 == 4.0);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && s.b[1374]) && s.b[1375]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && s.b[1374]) && (!s.b[1375])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && s.b[1374]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && (!s.b[1374])) {
            s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1359])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1376] = (s.v[642] == 0.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1376]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1376])) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.b[1377] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1377]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1378] = (p.p826 == 0.5);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && s.b[1378]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && (!s.b[1378])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1379] = (p.p826 == 0.5);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && s.b[1379]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && (!s.b[1379])) {
            s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) {
            s.store_scale(1202, 1195, s.v[419]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[380]);
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.b[1380] = (p.p840 == 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1380]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1381] = (((-p.p826) * s.v[407]) == (-1.0));
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1381]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1381])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[431]), s.ad_value(1206), s.ad_value(1209), s.v[431], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1382] = (s.v[1216] > 0.0);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1382]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1382])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1383] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1383]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1383])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1384] = (s.v[1216] > 0.0);
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1384]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1385] = (s.v[1215] > (-230.25850929940458));
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1384])) && s.b[1385]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1384])) && (!s.b[1385])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1384])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);
        }

        s.b[1386] = (p.p846 == 0.0);
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1386]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1387] = (p.p826 == 0.5);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && s.b[1387]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && (!s.b[1387])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), s.ad_value(1195), 1.0);
        }

        s.b[1388] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && s.b[1388]) {
            s.store_exp_div_scaled_inputs_indices(1195, 437, -1.0, 1220, 1.0);
        }

        s.b[1389] = (((-s.v[437]) / s.v[1220]) < 0.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && (!s.b[1388])) && s.b[1389]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && (!s.b[1388])) && (!s.b[1389])) {
            let assign22390_ad_e24336: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign22390_ad_e24336, 1e100);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846);
        }

        s.b[1390] = (p.p855 > 1000.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1390]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1391] = (s.v[1194] > ((-s.v[438]) * p.p855));
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        s.b[1392] = (p.p858 == 4.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && s.b[1391]) && s.b[1392]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && s.b[1391]) && (!s.b[1392])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && s.b[1391]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && (!s.b[1391])) {
            s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1376])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(471, s.ad_value(640), s.ad_value(1222), 1.0, s.ad_value(641), s.ad_value(1223), 1.0, s.ad_value(642), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1393] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        s.b[1394] = (s.v[482] < s.v[648]);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        s.b[1395] = (((((-0.5) * (s.v[482] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) && s.b[1395]) {
            s.store_exp_scaled_input(1188, 482, (s.v[365] * (-0.5)));
        }

        s.b[1396] = (((-0.5) * (s.v[482] * s.v[365])) < 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) && (!s.b[1395])) && s.b[1396]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(482), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) && (!s.b[1395])) && (!s.b[1396])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(482), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && (!s.b[1394])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(482), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1393]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1397] = (s.v[482] > 0.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1397]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && (!s.b[1397])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 482);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1393]) {
            s.store_sub(1191, 650, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 482, 0.5, 1191, 0.5, 482, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 482, 0.5, 653, 0.5, 482, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 482, 482, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1398] = (s.v[640] == 0.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1398]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1398])) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.b[1399] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1399]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1400] = (p.p824 == 0.5);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && s.b[1400]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && (!s.b[1400])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1401] = (p.p824 == 0.5);
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && s.b[1401]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && (!s.b[1401])) {
            s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) {
            s.store_scale(1202, 1195, s.v[417]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[378]);
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.b[1402] = (p.p838 == 0.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1402]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1403] = (((-p.p824) * s.v[405]) == (-1.0));
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1403]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1403])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[429]), s.ad_value(1206), s.ad_value(1209), s.v[429], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1404] = (s.v[1216] > 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1404]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1404])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1405] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1405]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1405])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1406] = (s.v[1216] > 0.0);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1406]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1407] = (s.v[1215] > (-230.25850929940458));
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1406])) && s.b[1407]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1406])) && (!s.b[1407])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1406])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);
        }

        s.b[1408] = (p.p844 == 0.0);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1408]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1409] = (p.p824 == 0.5);
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && s.b[1409]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && (!s.b[1409])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), s.ad_value(1195), 1.0);
        }

        s.b[1410] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && s.b[1410]) {
            s.store_exp_div_scaled_inputs_indices(1195, 435, -1.0, 1220, 1.0);
        }

        s.b[1411] = (((-s.v[435]) / s.v[1220]) < 0.0);
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && (!s.b[1410])) && s.b[1411]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && (!s.b[1410])) && (!s.b[1411])) {
            let assign23390_ad_e25980: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1195, assign23390_ad_e25980, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844);
        }

        s.b[1412] = (p.p853 > 1000.0);
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1412]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1413] = (s.v[1194] > ((-s.v[438]) * p.p853));
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        s.b[1414] = (p.p856 == 4.0);
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && s.b[1413]) && s.b[1414]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && s.b[1413]) && (!s.b[1414])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && s.b[1413]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && (!s.b[1413])) {
            s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1398])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1415] = (s.v[641] == 0.0);
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1415]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1415])) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.b[1416] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1416]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1417] = (p.p825 == 0.5);
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && s.b[1417]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && (!s.b[1417])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1418] = (p.p825 == 0.5);
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && s.b[1418]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && (!s.b[1418])) {
            s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) {
            s.store_scale(1202, 1195, s.v[418]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[379]);
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.b[1419] = (p.p839 == 0.0);
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1419]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_ad(1208, A::div_scaled_product_offset_denominator(s.ad_value(1207), s.ad_value(1207), 1.0, A::square(s.ad_value(1207)), 1.0, 1.0));
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1420] = (((-p.p825) * s.v[406]) == (-1.0));
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1420]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1420])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[430]), s.ad_value(1206), s.ad_value(1209), s.v[430], s.ad_value(1205), s.ad_value(1210), 0.5);
        }

    }
}
