#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[990] = (p.p37 >= 0.0);
        s.v[990] = if s.b[990] { 1.0 } else { 0.0 };

        if s.b[990] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[990]) {
            s.store_scalar(0, (-1.0));
        }

        s.v[767] = (8.8541878176e-12 * 11.8);

        s.b[991] = (p.p51 < 0.5);
        s.v[991] = if s.b[991] { 1.0 } else { 0.0 };

        if s.b[991] {
            s.store_scalar(1, 0.0);
        }

        s.b[992] = (p.p51 < 1.5);
        s.v[992] = if s.b[992] { 1.0 } else { 0.0 };

        if ((!s.b[991]) && s.b[992]) {
            s.store_scalar(1, 1.0);
        }

        s.b[993] = (p.p51 < 2.5);
        s.v[993] = if s.b[993] { 1.0 } else { 0.0 };

        if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {
            s.store_scalar(1, 2.0);
        }

        s.b[994] = (p.p51 < 4.0);
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

        if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {
            s.store_scalar(1, 3.0);
        }

        s.b[995] = (p.p51 < 7.0);
        s.v[995] = if s.b[995] { 1.0 } else { 0.0 };

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && s.b[995]) {
            s.store_scalar(1, 5.0);
        }

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && (!s.b[995])) {
            s.store_scalar(1, 9.0);
        }

        s.v[2] = 1000.0;

        s.v[3] = 10.0;

        s.v[4] = (1.0 / s.v[3]);

        s.v[350] = (273.15 + p.p38);

        s.v[474] = 0.0;

        s.b[996] = (p.p927 > 0.5);
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        if s.b[996] {
            s.store_scalar(474, 1.0);
        }

        if (!s.b[996]) {
            s.store_scalar(474, 0.0);
        }

        s.v[364] = (273.15 + p.p823);

        s.v[367] = (1.3806505e-23 / 1.6021918e-19);

        s.v[368] = (s.v[367] * s.v[364]);

        s.v[369] = (1.0 / s.v[368]);

        s.v[375] = ((-((0.000702 * s.v[364]) * s.v[364])) / (1108.0 + s.v[364]));

        s.v[378] = (p.p834 + s.v[375]);

        s.v[379] = (p.p835 + s.v[375]);

        s.v[380] = (p.p836 + s.v[375]);

        s.v[408] = (1.0 - p.p831);

        s.v[409] = (1.0 - p.p832);

        s.v[410] = (1.0 - p.p833);

        s.v[411] = (1.0 / s.v[408]);

        s.v[412] = (1.0 / s.v[409]);

        s.v[413] = (1.0 / s.v[410]);

        s.v[423] = (s.v[767] / p.p825);

        s.v[424] = ((p.p843 * s.v[767]) / p.p826);

        s.v[425] = ((p.p844 * s.v[767]) / p.p827);

        s.v[426] = (1.0 / s.v[423]);

        s.v[427] = (1.0 / s.v[424]);

        s.v[428] = (1.0 / s.v[425]);

        s.v[429] = (1.0 / p.p828);

        s.v[430] = (1.0 / p.p829);

        s.v[431] = (1.0 / p.p830);

        s.v[372] = (1.772453850905516 * 0.29214664);

        s.v[373] = (((((-5.0) * 0.29214664) + 6.0) - ((s.v[372]) as f64).powf((-2.0))) / 3.0);

        s.v[374] = ((1.0 - 0.29214664) - s.v[373]);

        s.v[444] = (1.0 - (1.0 / p.p824));

        s.v[445] = (1.0 / (1.0 - ((s.v[444]) as f64).powf(p.p863)));

        s.v[446] = (1.0 / (1.0 - ((s.v[444]) as f64).powf(p.p864)));

        s.v[447] = (1.0 / (1.0 - ((s.v[444]) as f64).powf(p.p865)));

        s.v[448] = (1.0 / p.p860);

        s.v[449] = (1.0 / p.p861);

        s.v[450] = (1.0 / p.p862);

        s.v[451] = (((-((s.v[445] * s.v[445]) * ((s.v[444]) as f64).powf((p.p863 - 1.0)))) * p.p863) * s.v[448]);

        s.v[452] = (((-((s.v[446] * s.v[446]) * ((s.v[444]) as f64).powf((p.p864 - 1.0)))) * p.p864) * s.v[449]);

        s.v[453] = (((-((s.v[447] * s.v[447]) * ((s.v[444]) as f64).powf((p.p865 - 1.0)))) * p.p865) * s.v[450]);

        s.b[997] = ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0));
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if s.b[997] {
            s.store_scalar(473, 1.0);
        }

        if (!s.b[997]) {
            s.store_scalar(473, 0.0);
        }

        s.b[998] = (s.v[473] == 1.0);
        s.v[998] = if s.b[998] { 1.0 } else { 0.0 };

        if s.b[998] {
            s.store_scalar(457, (if ((p.p827 * p.p866) > 1e-18) { (p.p827 * p.p866) } else { 1e-18 }));
        }

        if s.b[998] {
            s.store_scalar(458, (if ((p.p830 * p.p867) > 0.05) { (p.p830 * p.p867) } else { 0.05 }));
        }

        if s.b[998] {
            s.store_scalar(459, (if ((if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) < 0.95) { (if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[998] {
            s.store_scalar(460, (p.p836 * p.p869));
            s.store_offset(462, 460, s.v[375]);
            s.store_sub_from_scalar(467, 1.0, 459);
            s.store_div_from_scalar(468, 1.0, 467);
        }

        s.b[999] = (p.p44 == 0.0);
        s.v[999] = if s.b[999] { 1.0 } else { 0.0 };

        if s.b[999] {
            s.store_scalar(505, p.p825);
            s.store_scalar(506, p.p826);
            s.store_scalar(507, p.p827);
            s.store_scalar(508, p.p828);
            s.store_scalar(509, p.p829);
            s.store_scalar(510, p.p830);
            s.store_scalar(511, p.p831);
            s.store_scalar(512, p.p832);
            s.store_scalar(513, p.p833);
            s.store_scalar(514, p.p834);
            s.store_scalar(515, p.p835);
            s.store_scalar(516, p.p836);
            s.store_scalar(517, p.p837);
            s.store_scalar(518, p.p838);
            s.store_scalar(519, p.p839);
            s.store_scalar(522, p.p840);
            s.store_scalar(523, p.p841);
            s.store_scalar(524, p.p842);
            s.store_scalar(520, p.p843);
            s.store_scalar(521, p.p844);
            s.store_scalar(525, p.p845);
            s.store_scalar(526, p.p846);
            s.store_scalar(527, p.p847);
            s.store_scalar(528, p.p848);
            s.store_scalar(529, p.p849);
            s.store_scalar(530, p.p850);
            s.store_scalar(531, p.p851);
            s.store_scalar(532, p.p852);
            s.store_scalar(533, p.p853);
            s.store_scalar(534, p.p854);
            s.store_scalar(535, p.p855);
            s.store_scalar(536, p.p856);
            s.store_scalar(537, p.p857);
            s.store_scalar(538, p.p858);
            s.store_scalar(539, p.p859);
            s.store_scalar(540, p.p860);
            s.store_scalar(541, p.p861);
            s.store_scalar(542, p.p862);
            s.store_scalar(543, p.p863);
            s.store_scalar(544, p.p864);
            s.store_scalar(545, p.p865);
            s.store_scalar(552, p.p928);
            s.store_scalar(553, p.p929);
            s.store_scalar(636, p.p872);
            s.store_scalar(637, p.p873);
            s.store_scalar(638, p.p874);
            s.store_scalar(639, p.p875);
            s.store_scalar(546, p.p866);
            s.store_scalar(547, p.p867);
            s.store_scalar(548, p.p868);
            s.store_scalar(549, p.p869);
            s.store_scalar(550, p.p870);
            s.store_scalar(551, p.p871);
        }

        if (!s.b[999]) {
            s.store_scalar(505, p.p876);
            s.store_scalar(506, p.p877);
            s.store_scalar(507, p.p878);
            s.store_scalar(508, p.p879);
            s.store_scalar(509, p.p880);
            s.store_scalar(510, p.p881);
            s.store_scalar(511, p.p882);
            s.store_scalar(512, p.p883);
            s.store_scalar(513, p.p884);
            s.store_scalar(514, p.p885);
            s.store_scalar(515, p.p886);
            s.store_scalar(516, p.p887);
            s.store_scalar(517, p.p888);
            s.store_scalar(518, p.p889);
            s.store_scalar(519, p.p890);
            s.store_scalar(522, p.p891);
            s.store_scalar(523, p.p892);
            s.store_scalar(524, p.p893);
            s.store_scalar(520, p.p894);
            s.store_scalar(521, p.p895);
            s.store_scalar(525, p.p896);
            s.store_scalar(526, p.p897);
            s.store_scalar(527, p.p898);
            s.store_scalar(528, p.p899);
            s.store_scalar(529, p.p900);
            s.store_scalar(530, p.p901);
            s.store_scalar(531, p.p902);
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[999]) {
            s.store_scalar(532, p.p903);
            s.store_scalar(533, p.p904);
            s.store_scalar(534, p.p905);
            s.store_scalar(535, p.p906);
            s.store_scalar(536, p.p907);
            s.store_scalar(537, p.p908);
            s.store_scalar(538, p.p909);
            s.store_scalar(539, p.p910);
            s.store_scalar(540, p.p911);
            s.store_scalar(541, p.p912);
            s.store_scalar(542, p.p913);
            s.store_scalar(543, p.p914);
            s.store_scalar(544, p.p915);
            s.store_scalar(545, p.p916);
            s.store_scalar(552, p.p930);
            s.store_scalar(553, p.p931);
            s.store_scalar(636, p.p923);
            s.store_scalar(637, p.p924);
            s.store_scalar(638, p.p925);
            s.store_scalar(639, p.p926);
            s.store_scalar(546, p.p917);
            s.store_scalar(547, p.p918);
            s.store_scalar(548, p.p919);
            s.store_scalar(549, p.p920);
            s.store_scalar(550, p.p921);
            s.store_scalar(551, p.p922);
        }

        s.store_offset(554, 514, s.v[375]);

        s.store_offset(555, 515, s.v[375]);

        s.store_offset(556, 516, s.v[375]);

        s.store_sub_from_scalar(575, 1.0, 511);

        s.store_sub_from_scalar(576, 1.0, 512);

        s.store_sub_from_scalar(577, 1.0, 513);

        s.store_div_from_scalar(578, 1.0, 575);

        s.store_div_from_scalar(579, 1.0, 576);

        s.store_div_from_scalar(580, 1.0, 577);

        s.store_div_from_scalar(590, s.v[767], 505);

        s.store_div_scaled_inputs(591, s.ad_value(520), s.v[767], s.ad_value(506), 1.0);

        s.store_div_scaled_inputs(592, s.ad_value(521), s.v[767], s.ad_value(507), 1.0);

        s.store_div_from_scalar(593, 1.0, 590);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 508);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar_sub_from_scalar_ad(611, 1.0, 1.0, A::pow_from_scalar(s.v[444], s.ad_value(543)));

        s.store_div_from_scalar_sub_from_scalar_ad(612, 1.0, 1.0, A::pow_from_scalar(s.v[444], s.ad_value(544)));

        s.store_div_from_scalar_sub_from_scalar_ad(613, 1.0, 1.0, A::pow_from_scalar(s.v[444], s.ad_value(545)));

        s.store_div_from_scalar(614, 1.0, 540);

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.store_mul_product3_rhs(617, 614, A::square(s.ad_value(611)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(543), (-1.0))), s.ad_value(543), -1.0);

        s.store_mul_product3_rhs(618, 615, A::square(s.ad_value(612)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(544), (-1.0))), s.ad_value(544), -1.0);

        s.store_mul_product3_rhs(619, 616, A::square(s.ad_value(613)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(545), (-1.0))), s.ad_value(545), -1.0);

        s.b[1000] = ((((s.v[546] != 1.0) || (s.v[547] != 1.0)) || (s.v[548] != 1.0)) || (s.v[549] != 1.0));
        s.v[1000] = if s.b[1000] { 1.0 } else { 0.0 };

        if s.b[1000] {
            s.store_scalar(635, 1.0);
        }

        if (!s.b[1000]) {
            s.store_scalar(635, 0.0);
        }

        s.b[1001] = (s.v[635] == 1.0);
        s.v[1001] = if s.b[1001] { 1.0 } else { 0.0 };

        if s.b[1001] {
            if ((s.v[507] * s.v[546]) > 1e-18) {
                s.store_mul(620, 507, 546);
            } else {
                s.store_scalar(620, 1e-18);
            }
        }

        if s.b[1001] {
            if ((s.v[510] * s.v[547]) > 0.05) {
                s.store_mul(621, 510, 547);
            } else {
                s.store_scalar(621, 0.05);
            }
        }

        if s.b[1001] {
            if ((if ((s.v[513] * s.v[548]) > 0.05) { (s.v[513] * s.v[548]) } else { 0.05 }) < 0.95) {
                if ((s.v[513] * s.v[548]) > 0.05) {
                    s.store_mul(622, 513, 548);
                } else {
                    s.store_scalar(622, 0.05);
                }
            } else {
                s.store_scalar(622, 0.95);
            }
        }

        if s.b[1001] {
            s.store_mul(623, 516, 549);
            s.store_offset(625, 623, s.v[375]);
            s.store_sub_from_scalar(630, 1.0, 622);
            s.store_div_from_scalar(631, 1.0, 630);
        }

        s.v[878] = 0.0;

        s.v[351] = ((ctx_temp + p.p56) + p.p35);

        s.v[352] = (s.v[351] / s.v[350]);

        s.v[353] = (s.v[351] - s.v[350]);

        s.v[354] = ((s.v[351] * 1.3806505e-23) / 1.6021918e-19);

        s.v[355] = (1.0 / s.v[354]);

        s.v[356] = s.v[351];

        s.v[357] = (s.v[356] * s.v[356]);

        s.v[358] = (s.v[356] - s.v[350]);

        s.v[359] = (s.v[350] / s.v[356]);

        s.v[360] = ((s.v[359]) as f64).ln();

        s.v[715] = ((s.v[356] * 1.3806505e-23) / 1.6021918e-19);

        s.v[361] = (1.0 / s.v[715]);

        s.v[362] = ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357]));

        s.v[363] = ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0);

        if (!(s.v[363] > 0.001)) {
            s.store_scalar(363, 0.001);
        }

        s.v[718] = ((4.0 * 1.3806505e-23) * s.v[356]);

        s.v[365] = (((ctx_temp + p.p56) + p.p35)).max((273.15 + (-250.0)));

        s.v[366] = (s.v[365] / s.v[364]);

        s.v[370] = (s.v[367] * s.v[365]);

        s.v[371] = (1.0 / s.v[370]);

        s.v[376] = ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365]));

        s.v[381] = (p.p834 + s.v[376]);

        s.v[382] = (p.p835 + s.v[376]);

        s.v[383] = (p.p836 + s.v[376]);

        s.v[384] = (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[378] * s.v[369]) - (s.v[381] * s.v[371])))) as f64).exp());

        s.v[385] = (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[369]) - (s.v[382] * s.v[371])))) as f64).exp());

        s.v[386] = (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[369]) - (s.v[383] * s.v[371])))) as f64).exp());

        s.v[387] = ((p.p837 * s.v[384]) * s.v[384]);

        s.v[388] = ((p.p838 * s.v[385]) * s.v[385]);

        s.v[389] = ((p.p839 * s.v[386]) * s.v[386]);

        s.v[390] = ((p.p828 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[384]) as f64).ln()));

        s.v[391] = ((p.p829 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[385]) as f64).ln()));

        s.v[392] = ((p.p830 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[386]) as f64).ln()));

        s.v[393] = (s.v[390] + (s.v[370] * (((1.0 + ((((0.05 - s.v[390]) * s.v[371])) as f64).exp())) as f64).ln()));

        s.v[394] = (s.v[391] + (s.v[370] * (((1.0 + ((((0.05 - s.v[391]) * s.v[371])) as f64).exp())) as f64).ln()));

        s.v[395] = (s.v[392] + (s.v[370] * (((1.0 + ((((0.05 - s.v[392]) * s.v[371])) as f64).exp())) as f64).ln()));

        s.v[405] = (1.0 / s.v[393]);

        s.v[406] = (1.0 / s.v[394]);

        s.v[407] = (1.0 / s.v[395]);

        s.v[414] = (p.p825 * (((p.p828 * s.v[405])) as f64).powf(p.p831));

        s.v[415] = (p.p826 * (((p.p829 * s.v[406])) as f64).powf(p.p832));

        s.v[416] = (p.p827 * (((p.p830 * s.v[407])) as f64).powf(p.p833));

        s.v[417] = ((s.v[414] * s.v[393]) * s.v[411]);

        s.v[418] = ((s.v[415] * s.v[394]) * s.v[412]);

        s.v[419] = ((s.v[416] * s.v[395]) * s.v[413]);

        s.v[420] = (2.0 * s.v[414]);

        s.v[421] = (2.0 * s.v[415]);

        s.v[422] = (2.0 * s.v[416]);

        s.v[432] = ((0.5 * s.v[381])).max(s.v[370]);

        s.v[433] = ((0.5 * s.v[382])).max(s.v[370]);

        s.v[434] = ((0.5 * s.v[383])).max(s.v[370]);

        s.v[435] = (s.v[432] * s.v[371]);

        s.v[436] = (s.v[433] * s.v[371]);

        s.v[437] = (s.v[434] * s.v[371]);

        s.v[438] = (((((((32.0 * p.p848) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[432] * s.v[432]) * s.v[432]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[439] = (((((((32.0 * p.p849) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[440] = (((((((32.0 * p.p850) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[441] = (p.p854 * (1.0 + (p.p857 * (s.v[365] - s.v[364]))));

        s.v[442] = (p.p855 * (1.0 + (p.p858 * (s.v[365] - s.v[364]))));

        s.v[443] = (p.p856 * (1.0 + (p.p859 * (s.v[365] - s.v[364]))));

        if (!(s.v[441] > 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if (!(s.v[442] > 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (!(s.v[443] > 0.0)) {
            s.store_scalar(443, 0.0);
        }

        s.b[1021] = (s.v[473] == 1.0);
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if s.b[1021] {
            s.store_offset(461, 460, s.v[376]);
            s.store_scale_ad(463, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(462), s.v[369], s.ad_value(461), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(464, 458, s.v[366], A::ln(s.ad_value(463)), (2.0 * s.v[370]));
            s.store_add_scaled_inputs_ad_rhs(465, 464, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(464), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);
            s.store_div_from_scalar(466, 1.0, 465);
            s.store_mul_pow_ad_rhs(469, 457, A::mul(s.ad_value(458), s.ad_value(466)), s.ad_value(459));
            s.store_mul3_lhs(470, 469, 465, 468);
            s.store_scale(471, 469, 2.0);
        }

        s.store_offset(557, 514, s.v[376]);

        s.store_offset(558, 515, s.v[376]);

        s.store_offset(559, 516, s.v[376]);

        s.store_scale_ad(560, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(554), s.v[369], s.ad_value(557), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[369], s.ad_value(558), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[369], s.ad_value(559), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_mul3_lhs(563, 517, 560, 560);

        s.store_mul3_lhs(564, 518, 561, 561);

        s.store_mul3_lhs(565, 519, 562, 562);

        s.store_sub_scaled_inputs_ad_rhs(566, 508, s.v[366], A::ln(s.ad_value(560)), (2.0 * s.v[370]));

        s.store_sub_scaled_inputs_ad_rhs(567, 509, s.v[366], A::ln(s.ad_value(561)), (2.0 * s.v[370]));

        s.store_sub_scaled_inputs_ad_rhs(568, 510, s.v[366], A::ln(s.ad_value(562)), (2.0 * s.v[370]));

        s.store_add_scaled_inputs_ad_rhs(569, 566, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(566), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_add_scaled_inputs_ad_rhs(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_add_scaled_inputs_ad_rhs(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_mul_pow_ad_rhs(581, 505, A::mul(s.ad_value(508), s.ad_value(572)), s.ad_value(511));

        s.store_mul_pow_ad_rhs(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512));

        s.store_mul_pow_ad_rhs(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513));

        s.store_mul3_lhs(584, 581, 569, 578);

        s.store_mul3_lhs(585, 582, 570, 579);

        s.store_mul3_lhs(586, 583, 571, 580);

        s.store_scale(587, 581, 2.0);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_max_with_scalar_ad(599, A::scale(s.ad_value(557), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[370]);

        s.store_scale(602, 599, s.v[371]);

        s.store_scale(603, 600, s.v[371]);

        s.store_scale(604, 601, s.v[371]);

        s.store_scaled_sqrt_ad(605, A::mul3_scaled_output(s.ad_value(528), A::square(s.ad_value(599)), s.ad_value(599), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_scale_offset_rhs(608, 534, 537, (s.v[365] - s.v[364]), 1.0);

        s.store_mul_scale_offset_rhs(609, 535, 538, (s.v[365] - s.v[364]), 1.0);

        s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[365] - s.v[364]), 1.0);

        if (!(s.v[608] > 0.0)) {
            s.store_scalar(608, 0.0);
        }

        if (!(s.v[609] > 0.0)) {
            s.store_scalar(609, 0.0);
        }

        if (!(s.v[610] > 0.0)) {
            s.store_scalar(610, 0.0);
        }

        s.b[1022] = (s.v[635] == 1.0);
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if s.b[1022] {
            s.store_offset(624, 623, s.v[376]);
            s.store_scale_ad(626, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(625), s.v[369], s.ad_value(624), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(627, 621, s.v[366], A::ln(s.ad_value(626)), (2.0 * s.v[370]));
            s.store_add_scaled_inputs_ad_rhs(628, 627, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(627), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);
            s.store_div_from_scalar(629, 1.0, 628);
            s.store_mul_pow_ad_rhs(632, 620, A::mul(s.ad_value(621), s.ad_value(629)), s.ad_value(622));
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1022] {
            s.store_mul3_lhs(633, 632, 628, 631);
            s.store_scale(634, 632, 2.0);
        }

        s.v[5] = 1.0;

        s.v[6] = 1.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

        s.v[7] = p.p0;

        s.v[8] = p.p1;

        s.v[9] = p.p2;

        s.v[10] = p.p3;

        s.v[11] = p.p4;

        s.v[12] = p.p8;

        s.v[13] = p.p11;

        s.v[646] = p.p19;

        s.v[647] = p.p20;

        s.v[648] = p.p21;

        s.v[673] = p.p22;

        s.v[674] = p.p23;

        s.v[675] = p.p24;

        s.v[649] = p.p25;

        s.v[650] = p.p26;

        s.v[676] = p.p27;

        s.v[677] = p.p28;

        s.v[14] = p.p14;

        s.b[1023] = (p.p39 > 0.0);
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if s.b[1023] {
            s.store_scalar(5, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1023] {
            s.store_floor_ad(5, A::offset(s.ad_value(5), 0.5));
            s.store_div_from_scalar(6, 1.0, 5);
        }

        if ((s.v[8] * s.v[6]) > 1e-9) {
            s.store_scale(8, 6, s.v[8]);
        } else {
            s.store_scalar(8, 1e-9);
        }

        s.v[15] = p.p5;

        s.v[16] = p.p6;

        s.v[17] = p.p7;

        s.v[18] = (if (p.p10 < 1.5) { 1.0 } else { 2.0 });

        s.v[308] = (1e-6 / s.v[7]);

        s.store_div_from_scalar(309, 1e-6, 8);

        s.store_offset_scaled(310, 309, ((p.p190) * ((p.p188 * (1.0 + (p.p189 * s.v[308]))))), (p.p188 * (1.0 + (p.p189 * s.v[308]))));

        s.store_offset_scaled(311, 309, ((p.p194) * ((p.p192 * (1.0 + (p.p193 * s.v[308]))))), (p.p192 * (1.0 + (p.p193 * s.v[308]))));

        if (((s.v[7] + s.v[310]) - (2.0 * p.p191)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[7]) + ((-(2.0 * p.p191)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[8] + s.v[311]) - (2.0 * p.p195)) > 1e-9) {
            s.store_offset_add(313, 8, 311, (-(2.0 * p.p195)));
        } else {
            s.store_scalar(313, 1e-9);
        }

        s.store_div_from_scalar(314, 1e-6, 312);

        s.store_square(315, 314);

        s.store_div_from_scalar(316, 1e-6, 313);

        s.store_div_from_scalar(317, 1.0, 316);

        s.store_mul(318, 314, 316);

        s.store_div_from_scalar(319, 1.0, 318);

        if ((((s.v[7] + s.v[310]) - (2.0 * p.p191)) + p.p196) > 1e-9) {
            s.store_offset(320, 310, ((((s.v[7]) + ((-(2.0 * p.p191))))) + (p.p196)));
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[8] + s.v[311]) - (2.0 * p.p195)) + p.p197) > 1e-9) {
            s.store_offset_add(321, 8, 311, (((-(2.0 * p.p195))) + (p.p197)));
        } else {
            s.store_scalar(321, 1e-9);
        }

        s.store_scale(322, 321, 1000000.0);

        if (((s.v[7] + s.v[310]) + p.p196) > 1e-9) {
            s.store_offset(323, 310, ((s.v[7]) + (p.p196)));
        } else {
            s.store_scalar(323, 1e-9);
        }

        if (((s.v[8] + s.v[311]) + p.p197) > 1e-9) {
            s.store_offset_add(324, 8, 311, p.p197);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.store_scale(325, 323, 1000000.0);

        s.store_scale(326, 324, 1000000.0);

        if ((s.v[7] + s.v[310]) > 1e-9) {
            s.store_offset(327, 310, s.v[7]);
        } else {
            s.store_scalar(327, 1e-9);
        }

        if ((s.v[327] + p.p443) > 1e-9) {
            s.store_offset(328, 327, p.p443);
        } else {
            s.store_scalar(328, 1e-9);
        }

        if ((s.v[8] + s.v[311]) > 1e-9) {
            s.store_add(329, 8, 311);
        } else {
            s.store_scalar(329, 1e-9);
        }

        if ((s.v[13] - (0.5 * s.v[311])) > 1e-9) {
            s.store_sub_from_scalar_scaled_input(330, s.v[13], 311, 0.5);
        } else {
            s.store_scalar(330, 1e-9);
        }

        s.v[44] = p.p57;

        s.v[45] = p.p58;

        s.v[46] = p.p59;

        s.v[47] = p.p60;

        s.v[48] = p.p61;

        s.v[49] = p.p62;

        s.v[50] = p.p63;

        s.v[51] = p.p64;

        s.v[52] = p.p65;

        s.v[53] = p.p66;

        s.v[54] = p.p67;

        s.v[59] = p.p68;

        s.v[60] = p.p69;

        s.v[61] = p.p70;

        s.v[62] = p.p71;

        s.v[55] = p.p72;

        s.v[56] = p.p74;

        s.v[57] = p.p73;

        s.v[58] = p.p75;

        s.v[63] = p.p79;

        s.v[64] = p.p81;

        s.v[65] = p.p80;

        s.v[66] = p.p76;

        s.v[67] = p.p78;

        s.v[68] = p.p77;

        s.v[69] = p.p82;

        s.v[70] = p.p83;

        s.v[71] = p.p84;

        s.v[72] = p.p85;

        s.v[73] = p.p86;

        s.v[74] = p.p87;

        s.v[75] = p.p88;

        s.v[76] = p.p89;

        s.v[77] = p.p90;

        s.v[78] = p.p91;

        s.v[79] = p.p92;

        s.v[80] = p.p93;

        s.v[81] = p.p94;

        s.v[82] = p.p95;

        s.v[83] = p.p96;

        s.v[84] = p.p97;

        s.v[85] = p.p98;

        s.v[86] = p.p99;

        s.v[87] = p.p100;

        s.v[88] = p.p101;

        s.v[89] = p.p102;

        s.v[90] = p.p103;

        s.v[91] = p.p104;

        s.v[92] = p.p105;

        s.v[93] = p.p106;

        s.v[94] = p.p107;

        s.v[95] = p.p108;

        s.v[96] = p.p109;

        s.v[97] = p.p110;

        s.v[98] = p.p111;

        s.v[99] = p.p112;

        s.v[100] = p.p113;

        s.v[101] = p.p114;

        s.v[102] = p.p115;

        s.v[103] = p.p116;

        s.v[104] = p.p117;

        s.v[105] = p.p118;

        s.v[106] = p.p119;

        s.v[107] = p.p120;

        s.v[108] = p.p121;

        s.v[109] = p.p120;

        s.b[1024] = param_given[122];
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if s.b[1024] {
            s.store_scalar(109, p.p122);
        }

        s.v[110] = p.p121;

        s.b[1025] = param_given[123];
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if s.b[1025] {
            s.store_scalar(110, p.p123);
        }

        s.copy_ad(111, 109);

        s.b[1026] = param_given[124];
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if s.b[1026] {
            s.store_scalar(111, p.p124);
        }

        s.copy_ad(112, 110);

        s.b[1027] = param_given[125];
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if s.b[1027] {
            s.store_scalar(112, p.p125);
        }

        s.v[113] = p.p126;

        s.v[114] = p.p127;

        s.v[115] = p.p128;

        s.v[116] = p.p129;

        s.v[117] = p.p130;

        s.v[118] = p.p131;

        s.v[119] = p.p132;

        s.v[120] = p.p133;

        s.v[121] = p.p134;

        s.v[122] = p.p135;

        s.v[123] = p.p136;

        s.v[124] = p.p137;

        s.v[125] = p.p99;

        s.b[1028] = param_given[138];
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if s.b[1028] {
            s.store_scalar(125, p.p138);
        }

        s.v[126] = p.p104;

        s.b[1029] = param_given[139];
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if s.b[1029] {
            s.store_scalar(126, p.p139);
        }

        s.v[127] = p.p140;

        s.v[128] = p.p141;

        s.v[129] = p.p142;

        s.v[130] = p.p143;

        s.v[131] = p.p144;

        s.v[132] = p.p145;

        s.v[133] = p.p146;

        s.v[134] = p.p147;

        s.v[135] = p.p148;

        s.v[136] = p.p149;

        s.v[137] = p.p150;

        s.v[138] = p.p151;

        s.v[139] = p.p152;

        s.v[140] = p.p153;

        s.v[141] = p.p154;

        s.v[142] = p.p155;

        s.v[143] = p.p156;

        s.v[144] = p.p157;

        s.v[145] = p.p158;

        s.v[146] = p.p159;

        s.v[147] = p.p160;

        s.v[148] = p.p161;

        s.v[149] = p.p162;

        s.v[150] = p.p163;

        s.v[151] = p.p164;

        s.v[152] = p.p165;

        s.v[153] = p.p166;

        s.v[154] = p.p167;

        s.v[155] = p.p168;

        s.v[156] = p.p169;

        s.v[157] = p.p170;

        s.v[158] = p.p171;

        s.v[159] = p.p172;

        s.v[160] = p.p174;

        s.v[161] = p.p173;

        s.v[162] = p.p175;

        s.v[163] = p.p176;

        s.v[164] = p.p177;

        s.v[165] = p.p178;

        s.v[166] = p.p179;

        s.v[167] = p.p180;

        s.v[170] = p.p181;

        s.v[171] = p.p182;

        s.v[172] = p.p184;

        s.v[173] = p.p183;

        s.v[174] = p.p185;

        s.v[175] = p.p186;

        s.v[176] = p.p187;

        s.b[1030] = (p.p39 > 0.0);
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if s.b[1030] {
            s.store_add_scaled_inputs3_offset(44, A::powf(s.ad_value(314), p.p200), p.p199, s.ad_value(316), p.p201, s.ad_value(318), p.p202, p.p198);
            s.store_add_scaled_inputs3_offset(45, s.ad_value(314), p.p204, s.ad_value(316), p.p205, s.ad_value(318), p.p206, p.p203);
            s.store_scalar(46, p.p207);
            s.store_scalar(47, p.p208);
            s.store_scalar(48, p.p209);
        }

        if s.b[1030] {
            s.store_scale_ad(331, {
                if ((1.0 + ((p.p211 * s.v[316]) * (((1.0 + (s.v[313] / p.p212))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p211, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p212), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p210);
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {
            s.store_scale_ad(332, {
                if ((1.0 + ((p.p214 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p214, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p213);
        }

        if s.b[1030] {
            s.store_scale_ad(333, {
                if ((1.0 + ((p.p217 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p217, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p216);
        }

        s.b[1031] = (s.v[312] > (2.0 * s.v[333]));
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1031]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);
            s.store_square(336, 336);
        }

        s.b[1032] = (s.v[312] >= s.v[333]);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if ((s.b[1030] && (!s.b[1031])) && s.b[1032]) {
            s.store_add_ad_rhs(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));
        }

        if ((s.b[1030] && (!s.b[1031])) && (!s.b[1032])) {
            s.store_add_ad_rhs(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));
        }

        if s.b[1030] {
            s.store_mul_sub_scaled_inputs_rhs(49, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p218)), 1.0, s.ad_value(315), p.p219);
            s.store_add_scaled_inputs3_offset(50, A::powf(s.ad_value(314), p.p222), p.p221, s.ad_value(316), p.p223, s.ad_value(318), p.p224, p.p220);
            s.store_scalar(51, p.p225);
            s.store_scalar(52, p.p226);
            s.store_add_scaled_inputs3_offset(53, A::powf(s.ad_value(314), p.p229), p.p228, s.ad_value(316), p.p230, s.ad_value(318), p.p231, p.p227);
        }

        if s.b[1030] {
            s.store_scale_ad(54, {
                if (1e-6 > (1.0 + (p.p233 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p233, 1.0)
                }
            }, p.p232);
        }

        if s.b[1030] {
            s.store_scalar(59, p.p234);
            s.store_scalar(60, p.p235);
            s.store_scalar(61, p.p238);
            s.store_scalar(62, p.p239);
            s.store_mul3_ad(55, A::scale_offset(A::powf(s.ad_value(314), p.p242), p.p241, p.p240), A::scale_offset(s.ad_value(316), p.p243, 1.0), A::scale_offset(s.ad_value(318), p.p244, 1.0));
            s.store_scalar(56, p.p246);
            s.store_scalar(57, p.p245);
            s.store_scalar(58, p.p247);
            s.store_scaled_mul_scale_offset_rhs_ad(66, A::powf(s.ad_value(314), p.p249), 316, p.p250, 1.0, p.p248);
            s.store_scalar(67, p.p252);
            s.store_scalar(68, p.p251);
            s.store_scaled_mul_scale_offset_rhs_ad(63, A::powf(s.ad_value(314), p.p254), 316, p.p255, 1.0, p.p253);
            s.store_scalar(64, p.p257);
            s.store_scalar(65, p.p256);
            s.store_offset_scaled(337, 316, ((p.p260) * (p.p259)), p.p259);
        }

        if s.b[1030] {
            s.store_scale_ad(338, {
                if ((1.0 + (p.p262 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p262, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p261);
        }

        if s.b[1030] {
            s.store_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp(A::div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0))), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p263 * p.p264), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p264)))));
        }

        if s.b[1030] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p265, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p267), 1.0)), p.p266);
            s.store_mul_div_scaled_inputs_rhs(69, 340, s.ad_value(313), p.p258, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset(70, s.ad_value(314), p.p269, s.ad_value(316), p.p270, s.ad_value(318), p.p271, p.p268);
            s.store_offset_scaled(71, 316, ((p.p273) * (p.p272)), p.p272);
            s.store_scalar(72, p.p274);
            s.store_scalar(73, p.p275);
            s.store_scalar(74, p.p276);
            s.store_mul3_ad(75, A::scale_offset(A::powf(s.ad_value(314), p.p279), p.p278, p.p277), A::scale_offset(s.ad_value(316), p.p280, 1.0), A::scale_offset(s.ad_value(318), p.p281, 1.0));
            s.store_scalar(76, p.p282);
            s.store_scalar(77, p.p283);
            s.store_scalar(78, p.p284);
            s.store_mul3_ad_scaled_output(79, A::scale_offset(s.ad_value(314), p.p286, 1.0), A::scale_offset(s.ad_value(316), p.p287, 1.0), A::scale_offset(s.ad_value(318), p.p288, 1.0), p.p285);
            s.store_scalar(80, p.p289);
            s.store_scalar(81, p.p290);
            s.store_mul_scaled_ad_rhs(82, 316, p.p291, A::scale_offset(s.ad_value(316), p.p292, 1.0));
            s.store_scalar(83, p.p293);
            s.store_scalar(84, p.p294);
            s.store_scalar(85, p.p295);
            s.store_mul3_ad(86, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p297, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p298)), p.p296), A::scale_offset(s.ad_value(316), p.p299, 1.0), A::scale_offset(s.ad_value(318), p.p300, 1.0));
            s.store_add_scaled_inputs3_offset(87, s.ad_value(314), p.p302, s.ad_value(316), p.p303, s.ad_value(318), p.p304, p.p301);
            s.store_scalar(88, p.p305);
            s.store_scalar(89, p.p306);
            s.store_scalar(90, p.p307);
            s.store_div_from_scalar_offset_scaled_input(91, p.p308, 314, p.p309, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(92, A::powf(s.ad_value(314), p.p311), 316, p.p312, 1.0, p.p310);
            s.store_powf(341, 314, p.p314);
            s.store_div_scaled_product_offset_denominator(93, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p316, 1.0), p.p313, A::mul_scaled_lhs(s.ad_value(314), p.p315, s.ad_value(341)), 1.0, 1.0);
            s.store_powf(341, 314, p.p318);
            s.store_div_scaled_product_offset_denominator(94, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p320, 1.0), p.p317, A::mul_scaled_lhs(s.ad_value(314), p.p319, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(95, p.p321);
            s.store_scaled_mul_scale_offset_inputs(96, 314, p.p323, 1.0, 316, p.p324, 1.0, p.p322);
            s.store_scalar(97, p.p325);
            s.store_scalar(98, p.p326);
            s.store_scaled_mul_scale_offset_inputs(99, 314, p.p328, 1.0, 316, p.p329, 1.0, p.p327);
            s.store_scaled_mul_scale_offset_inputs(100, 314, p.p331, 1.0, 316, p.p332, 1.0, p.p330);
            s.store_scalar(101, p.p333);
            s.store_scalar(102, p.p334);
            s.store_div_from_scalar(103, p.p335, 318);
            s.store_div_from_scalar_scaled_input(104, (p.p336 * p.p236), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(105, (p.p337 * p.p237), 316, 1e-6);
            s.store_scalar(106, p.p338);
            s.store_scalar(107, p.p339);
            s.store_scalar(108, p.p340);
            s.store_scalar(109, p.p339);
        }

        s.b[1033] = param_given[341];
        s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1033]) {
            s.store_scalar(109, p.p341);
        }

        if s.b[1030] {
            s.store_scalar(110, p.p340);
        }

        s.b[1034] = param_given[342];
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1034]) {
            s.store_scalar(110, p.p342);
        }

        if s.b[1030] {
            s.copy_ad(111, 109);
        }

        s.b[1035] = param_given[343];
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1035]) {
            s.store_scalar(111, p.p343);
        }

        if s.b[1030] {
            s.copy_ad(112, 110);
        }

        s.b[1036] = param_given[344];
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1036]) {
            s.store_scalar(112, p.p344);
        }

        if s.b[1030] {
            s.store_scalar(113, p.p345);
            s.store_div_from_scalar_scaled_input(114, (p.p346 * p.p236), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(115, (p.p347 * p.p237), 316, 1e-6);
            s.store_scalar(116, p.p348);
            s.store_scalar(117, p.p349);
            s.store_scalar(118, p.p350);
            s.store_scalar(119, p.p351);
            s.store_scalar(120, p.p352);
            s.store_scalar(121, p.p353);
            s.store_scaled_mul(122, 321, 320, ((8.8541878176e-12 * p.p209) * 1.0 / (p.p208)));
            s.store_scale(129, 321, ((8.8541878176e-12 * p.p209) * (p.p236 * 1.0 / (p.p234))));
            s.store_scale(130, 321, ((8.8541878176e-12 * p.p209) * (p.p237 * 1.0 / (p.p235))));
            s.store_add_scaled_inputs3_offset(123, A::powf(s.ad_value(314), p.p356), p.p355, s.ad_value(316), p.p357, s.ad_value(318), p.p358, p.p354);
            s.store_add_scaled_inputs3_offset(124, s.ad_value(314), p.p360, s.ad_value(316), p.p361, s.ad_value(318), p.p362, p.p359);
            s.store_scalar(36, p.p296);
        }

        s.b[1037] = param_given[363];
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1037]) {
            s.store_scalar(36, p.p363);
        }

        if s.b[1030] {
            s.store_scalar(37, p.p297);
        }

        s.b[1038] = param_given[364];
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1038]) {
            s.store_scalar(37, p.p364);
        }

        if s.b[1030] {
            s.store_scalar(38, p.p298);
        }

        s.b[1039] = param_given[365];
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1039]) {
            s.store_scalar(38, p.p365);
        }

        if s.b[1030] {
            s.store_scalar(39, p.p299);
        }

        s.b[1040] = param_given[366];
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1040]) {
            s.store_scalar(39, p.p366);
        }

        if s.b[1030] {
            s.store_scalar(40, p.p300);
        }

        s.b[1041] = param_given[367];
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1041]) {
            s.store_scalar(40, p.p367);
        }

        if s.b[1030] {
            s.store_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(38)), 1.0), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));
            s.store_scalar(41, p.p308);
        }

        s.b[1042] = param_given[368];
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1042]) {
            s.store_scalar(41, p.p368);
        }

        if s.b[1030] {
            s.store_scalar(42, p.p309);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1043] = param_given[369];
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1043]) {
            s.store_scalar(42, p.p369);
        }

        if s.b[1030] {
            s.store_div_scaled_value_offset_denominator(126, s.ad_value(41), 1.0, A::mul(s.ad_value(42), s.ad_value(314)), 1.0, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(127, A::powf(s.ad_value(314), p.p371), 316, p.p372, 1.0, p.p370);
            s.store_powf(341, 314, p.p374);
            s.store_div_scaled_product_offset_denominator(128, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p376, 1.0), p.p373, A::mul_scaled_lhs(s.ad_value(314), p.p375, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(131, p.p377);
            s.store_scalar(132, p.p378);
            s.store_scalar(133, p.p379);
            s.store_scale(134, 325, p.p380);
            s.store_scale(135, 322, p.p381);
            s.store_scale(136, 322, p.p382);
            s.store_scalar(137, p.p383);
            s.store_scalar(138, p.p384);
            s.store_scalar(139, p.p385);
            s.store_scalar(140, p.p386);
            s.store_scale(141, 326, p.p387);
            s.store_scale(142, 326, p.p388);
            s.store_sub_from_scalar_ad(1012, 1.0, A::div_from_scalar((2.0 * p.p395), s.ad_value(312)));
        }

        if s.b[1030] {
            if (s.v[1012] > 0.001) {
                s.copy_ad(342, 1012);
            } else {
                s.store_scalar(342, 0.001);
            }
        }

        if s.b[1030] {
            s.store_div_from_scalar_powf_ad(343, 1.0, s.ad_value(342), p.p396);
            s.store_scalar(143, p.p389);
            s.store_mul_product3_rhs(144, 316, s.ad_value(69), s.ad_value(69), s.ad_value(316), p.p390);
            s.store_scaled_mul(145, 343, 318, p.p391);
            s.store_scaled_mul(146, 343, 318, p.p392);
            s.store_scaled_mul(147, 343, 318, p.p393);
            s.store_scalar(148, p.p394);
            s.store_offset_scaled(344, 313, p.p398, (2.0 * p.p397));
            s.store_div_from_scalar(345, 1e-6, 344);
            s.store_mul(346, 314, 345);
            s.store_scalar(149, p.p399);
            s.store_add_scaled_inputs3_offset(150, s.ad_value(314), p.p401, s.ad_value(316), p.p402, s.ad_value(318), p.p403, p.p400);
            s.store_add_scaled_inputs3_offset(151, A::powf(s.ad_value(314), p.p406), p.p405, s.ad_value(316), p.p407, s.ad_value(318), p.p408, p.p404);
            s.store_mul3_ad_scaled_output(152, A::scale_offset(A::powf(s.ad_value(314), p.p411), p.p410, 1.0), A::scale_offset(s.ad_value(316), p.p412, 1.0), A::scale_offset(s.ad_value(318), p.p413, 1.0), p.p409);
            s.store_offset_scaled_ad(153, A::powf(s.ad_value(314), p.p416), p.p415, p.p414);
            s.store_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p417 * p.p418), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p418)))), 1.0);
        }

        if s.b[1030] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_mul_ad(154, A::div_scaled_inputs(s.ad_value(344), p.p258, A::mul(s.ad_value(347), s.ad_value(312)), 1.0), A::scale_offset(s.ad_value(316), p.p419, 1.0));
            s.store_add_scaled_inputs3_offset(155, s.ad_value(314), p.p421, s.ad_value(316), p.p422, s.ad_value(318), p.p423, p.p420);
            s.store_scaled_mul_scale_offset_rhs_ad(156, A::powf(s.ad_value(314), p.p425), 316, p.p426, 1.0, p.p424);
            s.store_scalar(157, p.p427);
            s.store_scalar(158, p.p428);
            s.store_scaled_mul_scale_offset_rhs_ad(159, A::powf(s.ad_value(314), p.p430), 316, p.p431, 1.0, p.p429);
            s.store_scalar(160, p.p433);
            s.store_scalar(161, p.p432);
            s.store_scalar(162, p.p434);
            s.store_scale(163, 346, p.p435);
            s.store_scale(164, 346, p.p436);
            s.store_scale(165, 346, p.p437);
            s.store_scalar(166, p.p438);
            s.store_add_scaled_inputs3_offset(348, s.ad_value(314), p.p815, s.ad_value(316), p.p816, s.ad_value(318), p.p817, p.p814);
            s.store_add_scaled_inputs3_offset(349, s.ad_value(314), p.p819, s.ad_value(316), p.p820, s.ad_value(318), p.p821, p.p818);
            s.store_add_scaled_inputs3(167, A::div_scaled_inputs2(s.ad_value(329), ((0.3333333333333333 * 1.0 / (s.v[18])) * p.p442), s.ad_value(330), p.p442, s.ad_value(328), s.v[18]), 1.0, A::div_from_scalar((p.p440 + p.p441), A::mul(s.ad_value(329), s.ad_value(327))), 1.0, s.ad_value(5), p.p439);
        }

        if s.b[1030] {
            s.store_scalar(168, (if (p.p444 > 0.0) { p.p444 } else { 0.0 }));
        }

        if s.b[1030] {
            s.store_scalar(169, (if (p.p445 > 0.0) { p.p445 } else { 0.0 }));
        }

        s.b[1044] = (p.p44 == 0.0);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1044]) {
            s.copy_ad(169, 168);
        }

        if s.b[1030] {
            s.store_scaled_mul(170, 5, 168, p.p12);
            s.store_scaled_mul(171, 5, 169, p.p13);
            s.store_scale(172, 5, p.p447);
            s.store_scale(173, 5, p.p446);
            s.store_scale(174, 5, p.p448);
            s.store_scale(175, 5, p.p449);
            s.store_scalar(176, p.p450);
        }

        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset(44, s.ad_value(314), p.p452, s.ad_value(316), p.p453, s.ad_value(318), p.p454, p.p451);
        }

        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset(45, s.ad_value(314), p.p456, s.ad_value(316), p.p457, s.ad_value(318), p.p458, p.p455);
        }

        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1047]) {
            s.store_add_scaled_inputs3_offset(49, s.ad_value(314), p.p460, s.ad_value(316), p.p461, s.ad_value(318), p.p462, p.p459);
        }

        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset(50, s.ad_value(314), p.p464, s.ad_value(316), p.p465, s.ad_value(318), p.p466, p.p463);
        }

        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset(51, s.ad_value(314), p.p468, s.ad_value(316), p.p469, s.ad_value(318), p.p470, p.p467);
        }

        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1050]) {
            s.store_add_scaled_inputs3_offset(53, s.ad_value(314), p.p472, s.ad_value(316), p.p473, s.ad_value(318), p.p474, p.p471);
        }

        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset(54, s.ad_value(314), p.p476, s.ad_value(316), p.p477, s.ad_value(318), p.p478, p.p475);
        }

        s.b[1052] = (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset(61, s.ad_value(314), p.p480, s.ad_value(316), p.p481, s.ad_value(318), p.p482, p.p479);
        }

        s.b[1053] = (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1053]) {
            s.store_add_scaled_inputs3_offset(62, s.ad_value(314), p.p484, s.ad_value(316), p.p485, s.ad_value(318), p.p486, p.p483);
        }

        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset(55, s.ad_value(314), p.p488, s.ad_value(316), p.p489, s.ad_value(318), p.p490, p.p487);
        }

        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset(56, s.ad_value(314), p.p496, s.ad_value(316), p.p497, s.ad_value(318), p.p498, p.p495);
        }

        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset(57, s.ad_value(314), p.p492, s.ad_value(316), p.p493, s.ad_value(318), p.p494, p.p491);
        }

        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1057]) {
            s.store_add_scaled_inputs3_offset(58, s.ad_value(314), p.p500, s.ad_value(316), p.p501, s.ad_value(318), p.p502, p.p499);
        }

        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1058]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(66, 315, s.ad_value(314), p.p504, s.ad_value(316), p.p505, s.ad_value(318), p.p506, p.p503);
        }

        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset(67, s.ad_value(314), p.p512, s.ad_value(316), p.p513, s.ad_value(318), p.p514, p.p511);
        }

        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1060]) {
            s.store_add_scaled_inputs3_offset(68, s.ad_value(314), p.p508, s.ad_value(316), p.p509, s.ad_value(318), p.p510, p.p507);
        }

        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1061]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(63, 315, s.ad_value(314), p.p516, s.ad_value(316), p.p517, s.ad_value(318), p.p518, p.p515);
        }

        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset(64, s.ad_value(314), p.p524, s.ad_value(316), p.p525, s.ad_value(318), p.p526, p.p523);
        }

        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset(65, s.ad_value(314), p.p520, s.ad_value(316), p.p521, s.ad_value(318), p.p522, p.p519);
        }

        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1064]) {
            s.store_mul_ad(69, A::div(s.ad_value(313), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p528, s.ad_value(316), p.p529, s.ad_value(318), p.p530, p.p527));
        }

        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset(70, s.ad_value(314), p.p532, s.ad_value(316), p.p533, s.ad_value(318), p.p534, p.p531);
        }

        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1066]) {
            s.store_add_scaled_inputs3_offset(71, s.ad_value(314), p.p536, s.ad_value(316), p.p537, s.ad_value(318), p.p538, p.p535);
        }

        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset(73, s.ad_value(314), p.p540, s.ad_value(316), p.p541, s.ad_value(318), p.p542, p.p539);
        }

        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset(75, s.ad_value(314), p.p544, s.ad_value(316), p.p545, s.ad_value(318), p.p546, p.p543);
        }

        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1069]) {
            s.store_add_scaled_inputs3_offset(77, s.ad_value(314), p.p548, s.ad_value(316), p.p549, s.ad_value(318), p.p550, p.p547);
        }

        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset(79, s.ad_value(314), p.p552, s.ad_value(316), p.p553, s.ad_value(318), p.p554, p.p551);
        }

        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1071]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 316, s.ad_value(314), p.p556, s.ad_value(316), p.p557, s.ad_value(318), p.p558, p.p555);
        }

        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset(83, s.ad_value(314), p.p560, s.ad_value(316), p.p561, s.ad_value(318), p.p562, p.p559);
        }

        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1073]) {
            s.store_add_scaled_inputs3_offset(84, s.ad_value(314), p.p564, s.ad_value(316), p.p565, s.ad_value(318), p.p566, p.p563);
        }

        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1074]) {
            s.store_add_scaled_inputs3_offset(85, s.ad_value(314), p.p568, s.ad_value(316), p.p569, s.ad_value(318), p.p570, p.p567);
        }

        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1075]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(86, 314, s.ad_value(314), p.p572, s.ad_value(316), p.p573, s.ad_value(318), p.p574, p.p571);
        }

        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1076]) {
            s.store_add_scaled_inputs3_offset(87, s.ad_value(314), p.p576, s.ad_value(316), p.p577, s.ad_value(318), p.p578, p.p575);
        }

        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1077]) {
            s.store_add_scaled_inputs3_offset(88, s.ad_value(314), p.p580, s.ad_value(316), p.p581, s.ad_value(318), p.p582, p.p579);
        }

        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1078]) {
            s.store_add_scaled_inputs3_offset(89, s.ad_value(314), p.p584, s.ad_value(316), p.p585, s.ad_value(318), p.p586, p.p583);
        }

        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset(91, s.ad_value(314), p.p588, s.ad_value(316), p.p589, s.ad_value(318), p.p590, p.p587);
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1080]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(92, 314, s.ad_value(314), p.p592, s.ad_value(316), p.p593, s.ad_value(318), p.p594, p.p591);
        }

        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1081]) {
            s.store_add_scaled_inputs3_offset(93, s.ad_value(314), p.p596, s.ad_value(316), p.p597, s.ad_value(318), p.p598, p.p595);
        }

        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset(94, s.ad_value(314), p.p600, s.ad_value(316), p.p601, s.ad_value(318), p.p602, p.p599);
        }

        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset(96, s.ad_value(314), p.p604, s.ad_value(316), p.p605, s.ad_value(318), p.p606, p.p603);
        }

        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1084]) {
            s.store_add_scaled_inputs3_offset(98, s.ad_value(314), p.p608, s.ad_value(316), p.p609, s.ad_value(318), p.p610, p.p607);
        }

        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1085]) {
            s.store_add_scaled_inputs3_offset(99, s.ad_value(314), p.p612, s.ad_value(316), p.p613, s.ad_value(318), p.p614, p.p611);
        }

        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1086]) {
            s.store_add_scaled_inputs3_offset(100, s.ad_value(314), p.p616, s.ad_value(316), p.p617, s.ad_value(318), p.p618, p.p615);
        }

        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1087]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(103, 319, s.ad_value(314), p.p620, s.ad_value(316), p.p621, s.ad_value(318), p.p622, p.p619);
        }

        s.b[1088] = (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]);
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1088]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(104, 317, s.ad_value(314), p.p624, s.ad_value(316), p.p625, s.ad_value(318), p.p626, p.p623);
        }

        s.b[1089] = (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1089]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 317, s.ad_value(314), p.p628, s.ad_value(316), p.p629, s.ad_value(318), p.p630, p.p627);
        }

        s.b[1090] = (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]);
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1090]) {
            s.store_add_scaled_inputs3_offset(106, s.ad_value(314), p.p632, s.ad_value(316), p.p633, s.ad_value(318), p.p634, p.p631);
        }

        s.b[1091] = (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]);
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1091]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(114, 317, s.ad_value(314), p.p636, s.ad_value(316), p.p637, s.ad_value(318), p.p638, p.p635);
        }

        s.b[1092] = (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]);
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1092]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(115, 317, s.ad_value(314), p.p640, s.ad_value(316), p.p641, s.ad_value(318), p.p642, p.p639);
        }

        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1093]) {
            s.store_add_scaled_inputs3_offset(118, s.ad_value(314), p.p644, s.ad_value(316), p.p645, s.ad_value(318), p.p646, p.p643);
        }

        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1094]) {
            s.store_add_scaled_inputs3_offset(119, s.ad_value(314), p.p648, s.ad_value(316), p.p649, s.ad_value(318), p.p650, p.p647);
        }

        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1095]) {
            s.store_mul_ad_affine_product_rhs(122, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p652, s.ad_value(316), p.p653, s.ad_value(318), p.p654, p.p651), 1.0 / (1e-6), 0.0);
        }

        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1096]) {
            s.store_add_scaled_inputs3_offset(123, s.ad_value(314), p.p656, s.ad_value(316), p.p657, s.ad_value(318), p.p658, p.p655);
        }

        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1097]) {
            s.store_add_scaled_inputs3_offset(124, s.ad_value(314), p.p660, s.ad_value(316), p.p661, s.ad_value(318), p.p662, p.p659);
        }

        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(32, p.p571);
        }

        s.b[1099] = param_given[663];
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1099]) {
            s.store_scalar(32, p.p663);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(33, p.p572);
        }

        s.b[1100] = param_given[664];
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1100]) {
            s.store_scalar(33, p.p664);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(34, p.p573);
        }

        s.b[1101] = param_given[665];
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1101]) {
            s.store_scalar(34, p.p665);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(35, p.p574);
        }

        s.b[1102] = param_given[666];
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1098]) && s.b[1102]) {
            s.store_scalar(35, p.p666);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_mul_ad_rhs(125, 314, A::add_scaled_value_products3(s.ad_value(32), 1.0, s.ad_value(33), s.ad_value(314), 1.0, s.ad_value(34), s.ad_value(316), 1.0, s.ad_value(35), s.ad_value(318), 1.0));
        }

        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(32, p.p587);
        }

        s.b[1104] = param_given[667];
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1104]) {
            s.store_scalar(32, p.p667);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(33, p.p588);
        }

        s.b[1105] = param_given[668];
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1105]) {
            s.store_scalar(33, p.p668);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(34, p.p589);
        }

        s.b[1106] = param_given[669];
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1106]) {
            s.store_scalar(34, p.p669);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(35, p.p590);
        }

        s.b[1107] = param_given[670];
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1103]) && s.b[1107]) {
            s.store_scalar(35, p.p670);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_add_scaled_value_products3_indices(126, 32, 1.0, 33, 314, 1.0, 34, 316, 1.0, 35, 318, 1.0);
        }

        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1108]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(127, 314, s.ad_value(314), p.p672, s.ad_value(316), p.p673, s.ad_value(318), p.p674, p.p671);
        }

        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1109]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(128, 314, s.ad_value(314), p.p676, s.ad_value(316), p.p677, s.ad_value(318), p.p678, p.p675);
        }

        s.b[1110] = (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1110]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(129, 322, s.ad_value(314), p.p680, s.ad_value(316), p.p681, s.ad_value(318), p.p682, p.p679);
        }

        s.b[1111] = (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1111]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 322, s.ad_value(314), p.p684, s.ad_value(316), p.p685, s.ad_value(318), p.p686, p.p683);
        }

        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1112]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(134, 325, s.ad_value(314), p.p688, s.ad_value(316), p.p689, s.ad_value(318), p.p690, p.p687);
        }

        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1113]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 322, s.ad_value(314), p.p692, s.ad_value(316), p.p693, s.ad_value(318), p.p694, p.p691);
        }

        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1114]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(136, 322, s.ad_value(314), p.p696, s.ad_value(316), p.p697, s.ad_value(318), p.p698, p.p695);
        }

        s.b[1115] = (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1115]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(141, 326, s.ad_value(314), p.p700, s.ad_value(316), p.p701, s.ad_value(318), p.p702, p.p699);
        }

        s.b[1116] = (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1116]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(142, 326, s.ad_value(314), p.p704, s.ad_value(316), p.p705, s.ad_value(318), p.p706, p.p703);
        }

        s.b[1117] = (((param_given[707] || param_given[708]) || param_given[709]) || param_given[710]);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1117]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(144, 315, s.ad_value(314), p.p708, s.ad_value(316), p.p709, s.ad_value(318), p.p710, p.p707);
        }

        s.b[1118] = (((param_given[711] || param_given[712]) || param_given[713]) || param_given[714]);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1118]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(145, 318, s.ad_value(314), p.p712, s.ad_value(316), p.p713, s.ad_value(318), p.p714, p.p711);
        }

        s.b[1119] = (((param_given[715] || param_given[716]) || param_given[717]) || param_given[718]);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1119]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(146, 318, s.ad_value(314), p.p716, s.ad_value(316), p.p717, s.ad_value(318), p.p718, p.p715);
        }

        s.b[1120] = (((param_given[719] || param_given[720]) || param_given[721]) || param_given[722]);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1120]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(147, 318, s.ad_value(314), p.p720, s.ad_value(316), p.p721, s.ad_value(318), p.p722, p.p719);
        }

        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1121]) {
            s.store_add_scaled_inputs3_offset(149, s.ad_value(314), p.p724, s.ad_value(316), p.p725, s.ad_value(318), p.p726, p.p723);
        }

        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1122]) {
            s.store_add_scaled_inputs3_offset(150, s.ad_value(314), p.p728, s.ad_value(316), p.p729, s.ad_value(318), p.p730, p.p727);
        }

        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1123]) {
            s.store_add_scaled_inputs3_offset(151, s.ad_value(314), p.p732, s.ad_value(316), p.p733, s.ad_value(318), p.p734, p.p731);
        }

        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1124]) {
            s.store_add_scaled_inputs3_offset(152, s.ad_value(314), p.p736, s.ad_value(316), p.p737, s.ad_value(318), p.p738, p.p735);
        }

        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1125]) {
            s.store_add_scaled_inputs3_offset(153, s.ad_value(314), p.p740, s.ad_value(316), p.p741, s.ad_value(318), p.p742, p.p739);
        }

        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1126]) {
            s.store_mul_ad(154, A::div(s.ad_value(344), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p744, s.ad_value(316), p.p745, s.ad_value(318), p.p746, p.p743));
        }

        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1127]) {
            s.store_add_scaled_inputs3_offset(155, s.ad_value(314), p.p748, s.ad_value(316), p.p749, s.ad_value(318), p.p750, p.p747);
        }

        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1128]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(156, 315, s.ad_value(314), p.p752, s.ad_value(316), p.p753, s.ad_value(318), p.p754, p.p751);
        }

        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1129]) {
            s.store_add_scaled_inputs3_offset(157, s.ad_value(314), p.p756, s.ad_value(316), p.p757, s.ad_value(318), p.p758, p.p755);
        }

        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1130]) {
            s.store_add_scaled_inputs3_offset(158, s.ad_value(314), p.p760, s.ad_value(316), p.p761, s.ad_value(318), p.p762, p.p759);
        }

        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1131]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(159, 315, s.ad_value(314), p.p764, s.ad_value(316), p.p765, s.ad_value(318), p.p766, p.p763);
        }

        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1132]) {
            s.store_add_scaled_inputs3_offset(160, s.ad_value(314), p.p772, s.ad_value(316), p.p773, s.ad_value(318), p.p774, p.p771);
        }

        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1133]) {
            s.store_add_scaled_inputs3_offset(161, s.ad_value(314), p.p768, s.ad_value(316), p.p769, s.ad_value(318), p.p770, p.p767);
        }

        s.b[1134] = (((param_given[775] || param_given[776]) || param_given[777]) || param_given[778]);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1134]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(163, 346, s.ad_value(314), p.p776, s.ad_value(316), p.p777, s.ad_value(318), p.p778, p.p775);
        }

        s.b[1135] = (((param_given[779] || param_given[780]) || param_given[781]) || param_given[782]);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1135]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(164, 346, s.ad_value(314), p.p780, s.ad_value(316), p.p781, s.ad_value(318), p.p782, p.p779);
        }

        s.b[1136] = (((param_given[783] || param_given[784]) || param_given[785]) || param_given[786]);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1136]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(165, 346, s.ad_value(314), p.p784, s.ad_value(316), p.p785, s.ad_value(318), p.p786, p.p783);
        }

        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1137]) {
            s.store_add_scaled_inputs3_offset(176, s.ad_value(314), p.p788, s.ad_value(316), p.p789, s.ad_value(318), p.p790, p.p787);
        }

        if s.b[1030] {
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(43, p.p795);
        }

        s.b[1138] = param_given[796];
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1138]) {
            s.store_scalar(43, p.p796);
        }

        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (s.v[5] - 0.5);
            let assign9340_cond_e9224: f64 = if ((s.b[1030] && s.b[1139]) && (s.v[1018] < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1030] && s.b[1139]) {
                s.store_add_ad_rhs(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[9] + (0.5 * s.v[7])))));
                s.store_add_ad_rhs(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[10] + (0.5 * s.v[7])))));
                s.store_offset(1018, 1018, 1.0);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_mul(1003, 1019, 6);
            s.store_mul(1004, 1020, 6);
            s.store_scalar(1005, (1.0 / (p.p791 + (0.5 * s.v[7]))));
            s.store_scalar(1006, (1.0 / (p.p792 + (0.5 * s.v[7]))));
        }

        if (s.b[1030] && s.b[1139]) {
            if ((s.v[7] + s.v[310]) > 1e-9) {
                s.store_offset(1016, 310, s.v[7]);
            } else {
                s.store_scalar(1016, 1e-9);
            }
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1030] && s.b[1139]) {
            if (((s.v[8] + s.v[311]) + p.p793) > 1e-9) {
                s.store_offset_add(1017, 8, 311, p.p793);
            } else {
                s.store_scalar(1017, 1e-9);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p801);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p802);
            s.store_add_scaled_inputs_product_first_ad(1007, A::scale_offset(s.ad_value(1014), p.p798, 1.0), (1.0 + (p.p797 * (s.v[352] - 1.0))), 1015, (p.p799 * (1.0 + (p.p797 * (s.v[352] - 1.0)))), 1014, 1015, (p.p800 * (1.0 + (p.p797 * (s.v[352] - 1.0)))));
            s.store_div_scaled_inputs2(1008, s.ad_value(1003), p.p794, s.ad_value(1004), p.p794, s.ad_value(1007), 1.0);
            s.store_div_scaled_inputs2(1009, s.ad_value(1005), p.p794, s.ad_value(1006), p.p794, s.ad_value(1007), 1.0);
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p807);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p808);
            s.store_add_scaled_inputs_product_first_ad(1010, A::scale_offset(s.ad_value(1014), p.p804, 1.0), 1.0, 1015, p.p805, 1014, 1015, p.p806);
            s.store_add_scaled_inputs4(1012, s.ad_value(1003), 1.0, s.ad_value(1004), 1.0, s.ad_value(1005), -1.0, s.ad_value(1006), -1.0);
            s.store_div_scaled_offset_numerator(1013, s.ad_value(1008), 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);
            s.store_mul(69, 69, 1013);
            s.store_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p.p795, 1.0), 1.0, A::scale_offset(s.ad_value(1008), p.p795, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);
            s.store_mul(154, 154, 1013);
            s.store_div_scaled_inputs(1013, s.ad_value(1012), p.p803, s.ad_value(1010), 1.0);
            s.store_add(44, 44, 1013);
            s.store_add(149, 149, 1013);
            s.store_div_scaled_inputs(1013, s.ad_value(1012), p.p809, A::powf(s.ad_value(1010), p.p810), 1.0);
            s.store_add(66, 66, 1013);
            s.store_add(159, 159, 1013);
        }

        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {
            s.store_offset(1012, 8, s.v[12]);
            s.store_scalar(1013, (1.0 / p.p811));
            s.store_div_from_scalar_scaled_input(15, (p.p811 * p.p811), 1012, s.v[12]);
            s.store_div_scaled_add_product(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p.p811)), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
            s.store_div_scaled_add_product(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p.p811)), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
        }

        if (s.b[1030] && s.b[1140]) {
            s.store_add_scaled_inputs3(1012, s.ad_value(15), 1.0, s.ad_value(16), p.p812, s.ad_value(17), p.p813);
            s.store_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
            s.store_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
        }

        s.copy_ad(177, 44);

        s.copy_ad(178, 45);

        s.copy_ad(179, 46);

        s.copy_ad(181, 47);

        s.copy_ad(182, 48);

        if (s.v[49] > 1e20) {
            if (s.v[49] < 1e26) {
                s.copy_ad(183, 49);
            } else {
                s.store_scalar(183, 1e26);
            }
        } else {
            s.store_scalar(183, 1e20);
        }

        if (s.v[50] > 0.01) {
            s.copy_ad(184, 50);
        } else {
            s.store_scalar(184, 0.01);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(185, 51);
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(186, 52);

        s.copy_ad(187, 53);

        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(192, 59);

        s.copy_ad(193, 60);

        if (s.v[61] > 1e23) {
            if (s.v[61] < 1e27) {
                s.copy_ad(194, 61);
            } else {
                s.store_scalar(194, 1e27);
            }
        } else {
            s.store_scalar(194, 1e23);
        }

        if (s.v[62] > 1e23) {
            if (s.v[62] < 1e27) {
                s.copy_ad(195, 62);
            } else {
                s.store_scalar(195, 1e27);
            }
        } else {
            s.store_scalar(195, 1e23);
        }

        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[57] > 0.0) {
            if (s.v[57] < 0.5) {
                s.copy_ad(191, 57);
            } else {
                s.store_scalar(191, 0.5);
            }
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[56] > 0.0) {
            if (s.v[56] < 1.0) {
                s.copy_ad(190, 56);
            } else {
                s.store_scalar(190, 1.0);
            }
        } else {
            s.store_scalar(190, 0.0);
        }

        s.copy_ad(180, 58);

        if (s.v[66] > 0.0) {
            s.copy_ad(196, 66);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[68] > 0.0) {
            if (s.v[68] < 1.0) {
                s.copy_ad(198, 68);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[67] > 0.0) {
            s.copy_ad(197, 67);
        } else {
            s.store_scalar(197, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(199, 63);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            if (s.v[65] < 1.0) {
                s.copy_ad(200, 65);
            } else {
                s.store_scalar(200, 1.0);
            }
        } else {
            s.store_scalar(200, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.copy_ad(201, 64);
        } else {
            s.store_scalar(201, 0.0);
        }

        if (s.v[69] > 0.0) {
            s.copy_ad(202, 69);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(204, 71);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(206, 73);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(208, 75);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 76);

        if (s.v[77] > 0.0) {
            s.copy_ad(210, 77);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 78);

        if (s.v[79] > 0.0) {
            s.copy_ad(212, 79);
        } else {
            s.store_scalar(212, 0.0);
        }

        s.copy_ad(213, 80);

        s.copy_ad(214, 81);

        if (s.v[82] > 0.0) {
            s.copy_ad(215, 82);
        } else {
            s.store_scalar(215, 0.0);
        }

        s.copy_ad(216, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(217, 84);
            } else {
                s.store_scalar(217, 1.0);
            }
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(218, 85);
        } else {
            s.store_scalar(218, (-0.5));
        }

        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }

        s.copy_ad(220, 87);

        if (s.v[88] > (-0.5)) {
            if (s.v[88] < 1.0) {
                s.copy_ad(221, 88);
            } else {
                s.store_scalar(221, 1.0);
            }
        } else {
            s.store_scalar(221, (-0.5));
        }

        if (s.v[89] > (-0.5)) {
            s.copy_ad(222, 89);
        } else {
            s.store_scalar(222, (-0.5));
        }

        if (s.v[90] > 0.01) {
            s.copy_ad(223, 90);
        } else {
            s.store_scalar(223, 0.01);
        }

        if (s.v[91] > 2.0) {
            s.copy_ad(224, 91);
        } else {
            s.store_scalar(224, 2.0);
        }

        if (s.v[92] > 0.0) {
            s.copy_ad(225, 92);
        } else {
            s.store_scalar(225, 0.0);
        }

        if (s.v[93] > 0.0) {
            s.copy_ad(226, 93);
        } else {
            s.store_scalar(226, 0.0);
        }

        if (s.v[94] > 0.0) {
            s.copy_ad(227, 94);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 95);

        if (s.v[96] > 0.0) {
            s.copy_ad(229, 96);
        } else {
            s.store_scalar(229, 0.0);
        }

        s.copy_ad(230, 97);

        s.copy_ad(231, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(232, 99);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(233, 100);
        } else {
            s.store_scalar(233, 0.0);
        }

        if (s.v[101] > 1e-12) {
            s.copy_ad(234, 101);
        } else {
            s.store_scalar(234, 1e-12);
        }

        s.copy_ad(235, 102);

        if (s.v[103] > 0.0) {
            s.copy_ad(236, 103);
        } else {
            s.store_scalar(236, 0.0);
        }

        if (s.v[104] > 0.0) {
            s.copy_ad(237, 104);
        } else {
            s.store_scalar(237, 0.0);
        }

        if (s.v[105] > 0.0) {
            s.copy_ad(238, 105);
        } else {
            s.store_scalar(238, 0.0);
        }

        s.copy_ad(239, 106);

        s.copy_ad(240, 107);

        s.copy_ad(241, 108);

        s.copy_ad(242, 109);

        s.copy_ad(243, 110);

        s.copy_ad(244, 111);

        s.copy_ad(245, 112);

        s.copy_ad(246, 113);

        if (s.v[114] > 0.0) {
            s.copy_ad(247, 114);
        } else {
            s.store_scalar(247, 0.0);
        }

        if (s.v[115] > 0.0) {
            s.copy_ad(248, 115);
        } else {
            s.store_scalar(248, 0.0);
        }

        s.copy_ad(249, 116);

        s.copy_ad(250, 117);

        s.copy_ad(251, 118);

        s.copy_ad(252, 119);

        s.copy_ad(253, 120);

        s.copy_ad(254, 121);

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[122] > 0.0) {
            s.copy_ad(255, 122);
        } else {
            s.store_scalar(255, 0.0);
        }

        s.copy_ad(256, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(257, 124);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }

        if (s.v[126] > 2.0) {
            s.copy_ad(259, 126);
        } else {
            s.store_scalar(259, 2.0);
        }

        s.copy_ad(260, 127);

        if (s.v[128] > 0.0) {
            s.copy_ad(261, 128);
        } else {
            s.store_scalar(261, 0.0);
        }

        if (s.v[129] > 0.0) {
            s.copy_ad(262, 129);
        } else {
            s.store_scalar(262, 0.0);
        }

        if (s.v[130] > 0.0) {
            s.copy_ad(263, 130);
        } else {
            s.store_scalar(263, 0.0);
        }

        s.copy_ad(264, 131);

        s.copy_ad(265, 132);

        s.copy_ad(266, 133);

        if (s.v[134] > 0.0) {
            s.copy_ad(267, 134);
        } else {
            s.store_scalar(267, 0.0);
        }

        if (s.v[135] > 0.0) {
            s.copy_ad(268, 135);
        } else {
            s.store_scalar(268, 0.0);
        }

        if (s.v[136] > 0.0) {
            s.copy_ad(269, 136);
        } else {
            s.store_scalar(269, 0.0);
        }

        s.copy_ad(270, 137);

        s.copy_ad(271, 138);

        s.copy_ad(272, 139);

        s.copy_ad(273, 140);

        if (s.v[141] > 0.0) {
            s.copy_ad(274, 141);
        } else {
            s.store_scalar(274, 0.0);
        }

        if (s.v[142] > 0.0) {
            s.copy_ad(275, 142);
        } else {
            s.store_scalar(275, 0.0);
        }

        s.copy_ad(276, 143);

        if (s.v[144] > 0.0) {
            s.copy_ad(277, 144);
        } else {
            s.store_scalar(277, 0.0);
        }

        if (s.v[145] > 0.0) {
            s.copy_ad(278, 145);
        } else {
            s.store_scalar(278, 0.0);
        }

        if (s.v[146] > 0.0) {
            s.copy_ad(279, 146);
        } else {
            s.store_scalar(279, 0.0);
        }

        if (s.v[147] > 0.0) {
            s.copy_ad(280, 147);
        } else {
            s.store_scalar(280, 0.0);
        }

        s.copy_ad(281, 148);

        s.copy_ad(282, 149);

        s.copy_ad(283, 150);

        s.copy_ad(284, 151);

        if (s.v[152] > 1e20) {
            if (s.v[152] < 1e26) {
                s.copy_ad(285, 152);
            } else {
                s.store_scalar(285, 1e26);
            }
        } else {
            s.store_scalar(285, 1e20);
        }

        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(287, 154);
        } else {
            s.store_scalar(287, 0.0);
        }

        s.copy_ad(288, 155);

        if (s.v[156] > 0.0) {
            s.copy_ad(289, 156);
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(290, 157);
            } else {
                s.store_scalar(290, 1.0);
            }
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[158] > 0.0) {
            s.copy_ad(291, 158);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[159] > 0.0) {
            s.copy_ad(292, 159);
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[161] > 0.0) {
            if (s.v[161] < 1.0) {
                s.copy_ad(294, 161);
            } else {
                s.store_scalar(294, 1.0);
            }
        } else {
            s.store_scalar(294, 0.0);
        }

        if (s.v[160] > 0.0) {
            s.copy_ad(293, 160);
        } else {
            s.store_scalar(293, 0.0);
        }

        s.copy_ad(295, 162);

        if (s.v[163] > 0.0) {
            s.copy_ad(296, 163);
        } else {
            s.store_scalar(296, 0.0);
        }

        if (s.v[164] > 0.0) {
            s.copy_ad(297, 164);
        } else {
            s.store_scalar(297, 0.0);
        }

        if (s.v[165] > 0.0) {
            s.copy_ad(298, 165);
        } else {
            s.store_scalar(298, 0.0);
        }

        s.copy_ad(299, 166);

        if (s.v[167] > 0.0) {
            s.copy_ad(300, 167);
        } else {
            s.store_scalar(300, 0.0);
        }

        s.copy_ad(301, 170);

        s.copy_ad(302, 171);

        s.copy_ad(303, 173);

        s.copy_ad(304, 174);

        s.copy_ad(305, 175);

        s.copy_ad(306, 172);

        if ((p.p31 * s.v[5]) > 0.0) {
            s.store_scale(19, 5, p.p31);
        } else {
            s.store_scalar(19, 0.0);
        }

        s.v[20] = p.p16;

        s.v[21] = p.p15;

        s.v[22] = p.p18;

        s.v[23] = p.p17;

        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }

        s.b[1142] = (p.p44 == 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.copy_ad(193, 192);
            s.copy_ad(195, 194);
            s.copy_ad(248, 247);
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(254, 253);
            s.copy_ad(238, 237);
            s.copy_ad(244, 242);
            s.copy_ad(245, 243);
            s.copy_ad(263, 262);
            s.copy_ad(265, 264);
            s.copy_ad(269, 268);
            s.copy_ad(275, 274);
        }

        s.store_scale(768, 182, 8.8541878176e-12);

        s.store_div(769, 768, 181);

        s.store_square(770, 181);

        s.store_scale(771, 769, 6.241449993689894e18);

        s.store_mul(772, 257, 183);

        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }

        s.v[773] = 0.0;

        s.b[1143] = (p.p52 > 0.0);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p.p52));
        }

        s.b[1144] = (s.v[0] == (-1.0));
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if (s.b[1143] && s.b[1144]) {
            s.store_scale(773, 773, (7.448711 / 5.951993));
        }

        s.store_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));

        s.store_scale(775, 214, 0.5);

        s.v[776] = 0.5;

        s.b[1145] = (s.v[0] == (-1.0));
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_scale(775, 214, 0.3333333333333333);
            s.store_scalar(776, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(777, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(778, s.ad_value(1011), (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(779, 1.0, 228);

        s.store_div(780, 768, 192);

        s.store_div(781, 768, 193);

        s.store_div_ad_lhs(782, A::sqrt_scaled_input(s.ad_value(194), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 780);

        s.store_div_ad_lhs(783, A::sqrt_scaled_input(s.ad_value(195), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 781);

        s.store_square(784, 782);

        s.store_square(785, 783);

        s.store_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * s.v[355])), (-1.0))), s.ad_value(266), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(787, A::ln_scaled_input(s.ad_value(782), 0.5), 786);

        s.store_add_ad_lhs(788, A::ln_scaled_input(s.ad_value(783), 0.5), 786);

        s.store_div_from_scalar(820, 1.0, 782);

        s.store_offset_scaled(821, 782, 3.1, 8.5);

        s.store_square(789, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1146] = (s.v[820] < 0.06);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_scale(790, 820, 64.0);
        }

        s.b[1147] = (s.v[820] <= 0.45);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if ((!s.b[1146]) && s.b[1147]) {
            s.store_offset_scaled(790, 820, 22.0, 3.0);
        }

        s.b[1148] = (s.v[820] <= 1.6);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if (((!s.b[1146]) && (!s.b[1147])) && s.b[1148]) {
            s.store_offset_scaled(790, 820, (-7.2), 15.5);
        }

        if (((!s.b[1146]) && (!s.b[1147])) && (!s.b[1148])) {
            s.copy_ad(790, 782);
        }

        s.store_add_scaled_inputs_product_right_ad(791, 822, 1.0, 784, 0.5, 782, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), (-1.0));

        s.store_div_from_scalar(820, 1.0, 783);

        s.store_offset_scaled(821, 783, 3.1, 8.5);

        s.store_square(792, 821);

        s.store_scale(822, 821, 0.5);

        s.b[1149] = (s.v[820] < 0.06);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if s.b[1149] {
            s.store_scale(793, 820, 64.0);
        }

        s.b[1150] = (s.v[820] <= 0.45);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if ((!s.b[1149]) && s.b[1150]) {
            s.store_offset_scaled(793, 820, 22.0, 3.0);
        }

        s.b[1151] = (s.v[820] <= 1.6);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {
            s.store_offset_scaled(793, 820, (-7.2), 15.5);
        }

        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {
            s.copy_ad(793, 783);
        }

        s.store_add_scaled_inputs_product_right_ad(794, 822, 1.0, 785, 0.5, 783, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), (-1.0));

        s.store_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));

        if (!(s.v[728] > 0.05)) {
            s.store_scalar(728, 0.05);
        }

        s.store_div_ad_lhs(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.v[730] = 0.0;

        s.v[731] = 0.0;

        s.b[1152] = (s.v[188] > 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_div_from_scalar(732, 80000000.0, 770);
        }

        if s.b[1152] {
            if (s.v[188] > s.v[732]) {
                s.copy_ad(731, 188);
            } else {
                s.copy_ad(731, 732);
            }
        }

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1152] {
            if (5e24 > s.v[731]) {
                s.store_scalar(731, 5e24);
            } else {
            }
        }

        if s.b[1152] {
            s.store_div_scaled_product_indices(730, 769, 769, (2.0 * s.v[715]), 731, (1.6021918e-19 * s.v[767]));
        }

        s.v[733] = ((100.0 * s.v[715]) * s.v[715]);

        s.b[1153] = (p.p52 > 0.0);
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));
            s.store_mul_scaled_ad_rhs(735, 773, 0.75, A::powf(s.ad_value(734), 0.6666666666666666));
            s.store_add(728, 728, 735);
            s.store_mul_offset_ad_rhs(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_sqrt(736, 728);

        s.store_scale(737, 728, 0.95);

        s.store_scaled_mul(738, 728, 728, 0.0025);

        s.copy_ad(739, 738);

        s.store_scaled_sqrt(740, 739, 0.5);

        s.store_add_scaled_inputs3(741, s.ad_value(737), 0.5, s.ad_value(740), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(738), 1.0, A::sub(s.ad_value(737), s.ad_value(740)), A::sub(s.ad_value(737), s.ad_value(740)), 1.0)), (-0.5));

        s.store_scaled_offset(742, 728, s.v[362], 0.5);

        s.store_sub_ad_lhs(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);

        s.store_add_scaled_inputs3(744, A::sqrt(A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0)), 1.0, s.ad_value(736), (-1.0), s.ad_value(743), -1.0);

        s.store_add_scaled_inputs3_offset(745, s.ad_value(187), 1.0, s.ad_value(256), 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);

        if (!(s.v[745] > 0.05)) {
            s.store_scalar(745, 0.05);
        }

        s.store_div_ad_lhs(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);

        s.b[1154] = (p.p52 > 0.0);
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));
            s.store_mul_scaled_ad_rhs(735, 773, 0.75, A::powf(s.ad_value(734), 0.6666666666666666));
            s.store_add(745, 745, 735);
            s.store_mul_offset_ad_rhs(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0);
        }

        s.store_scale(747, 745, 0.95);

        s.store_scaled_mul(748, 745, 745, 0.0025);

        s.copy_ad(749, 748);

        s.store_scaled_sqrt(740, 749, 0.5);

        s.store_add_scaled_inputs3(750, s.ad_value(747), 0.5, s.ad_value(740), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(748), 1.0, A::sub(s.ad_value(747), s.ad_value(740)), A::sub(s.ad_value(747), s.ad_value(740)), 1.0)), (-0.5));

        s.store_offset_add_scaled_product(700, s.ad_value(177), 1.0, s.ad_value(178), A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);

        s.store_exp_scaled_input(751, 180, s.v[360]);

        s.store_mul(701, 189, 751);

        s.store_scale(702, 190, 1.0 / (s.v[359]));

        s.store_exp_scaled_input(752, 203, s.v[360]);

        s.store_mul(703, 202, 752);

        s.store_scaled_mul(716, 703, 769, s.v[20]);

        s.store_mul_ad_rhs(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));

        s.store_exp_scaled_input(753, 205, s.v[360]);

        s.store_mul(704, 204, 753);

        s.store_mul_ad_rhs(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));

        s.store_exp_scaled_input(754, 209, s.v[360]);

        s.store_mul(706, 208, 754);

        s.store_exp_scaled_input(755, 213, s.v[360]);

        s.store_mul(708, 212, 755);

        s.store_exp_scaled_input(756, 216, s.v[360]);

        s.store_mul(709, 215, 756);

        s.store_scaled_mul(757, 716, 709, 2.0);

        s.store_exp_scaled_input(758, 220, s.v[360]);

        s.store_mul(720, 219, 758);

        s.store_mul(721, 258, 758);

        s.store_mul_ad_rhs(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));

        s.store_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));

        s.store_div_scaled_inputs(722, s.ad_value(716), (s.v[715] * s.v[715]), s.ad_value(771), 1.0);

        s.b[1155] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if s.b[1155] {
            s.store_offset_add_scaled_inputs(713, s.ad_value(282), 1.0, s.ad_value(283), s.v[358], s.v[23]);
            s.store_exp_scaled_input(759, 288, s.v[360]);
            s.store_mul(714, 287, 759);
            s.store_scaled_mul(717, 714, 769, s.v[22]);
            s.store_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);
            s.store_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);
        }

        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }

        if s.b[1155] {
            s.store_div_ad_lhs(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);
            s.store_square(724, 761);
            s.store_ln(725, 724);
            s.store_scale(762, 760, 0.95);
            s.store_scaled_mul(763, 760, 760, 0.0025);
            s.copy_ad(764, 763);
            s.store_scaled_sqrt(765, 764, 0.5);
            s.store_add_scaled_inputs3(766, s.ad_value(762), 0.5, s.ad_value(765), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(763), 1.0, A::sub(s.ad_value(762), s.ad_value(765)), A::sub(s.ad_value(762), s.ad_value(765)), 1.0)), (-0.5));
            s.store_div_scaled_inputs(726, s.ad_value(717), (s.v[715] * s.v[715]), s.ad_value(771), 1.0);
            s.store_scale(727, 295, (4.0 * (1.3806505e-23 * s.v[356])));
        }

        if (!s.b[1155]) {
            s.store_scalar(713, 0.0);
            s.store_scalar(759, 1.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(723, s.v[715]);
            s.store_scalar(760, 0.0);
            s.store_scalar(761, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(762, 0.0);
            s.store_scalar(763, 0.0);
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(727, 1.0);
        }

        s.store_div_from_scalar(795, 1.0, 246);

        s.store_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(797, 796, 181);

        s.store_mul(798, 796, 192);

        s.store_mul(799, 796, 193);

        s.v[800] = 0.0;

        s.b[1156] = (s.v[241] < 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if s.b[1156] {
            s.store_div_scaled_inputs(800, s.ad_value(240), (-0.495), s.ad_value(241), 1.0);
        }

        s.v[801] = 0.0;

        s.b[1157] = (s.v[243] < 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if s.b[1157] {
            s.store_div_scaled_inputs(801, s.ad_value(242), (-0.495), s.ad_value(243), 1.0);
        }

        s.b[1158] = (s.v[245] < 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if s.b[1158] {
            s.store_div_scaled_inputs(802, s.ad_value(244), (-0.495), s.ad_value(245), 1.0);
        }

        s.store_pow_from_scalar_ad(803, s.v[352], s.ad_value(239));

        s.store_mul(236, 236, 803);

        s.store_mul(237, 237, 803);

        s.store_mul(238, 238, 803);

        s.store_div_scaled_inputs(804, s.ad_value(247), 4e-18, A::square(s.ad_value(192)), 1.0);

        s.store_div_scaled_inputs(805, s.ad_value(248), 4e-18, A::square(s.ad_value(193)), 1.0);

        if ((1.0 + (s.v[251] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 251, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(710, 249, 796);

        s.store_scaled_mul(806, 710, 192, 500000000.0);

        if ((1.0 + (s.v[252] * s.v[353])) > 0.0) {
            s.store_offset_scaled(796, 252, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }

        s.store_mul(711, 250, 796);

        s.store_scaled_mul(807, 711, 193, 500000000.0);

        s.v[808] = 0.0;

        s.b[1159] = (s.v[272] > 1e-10);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if s.b[1159] {
            s.store_div_from_scalar(808, 0.75, 272);
        }

        s.store_square(809, 273);

        s.store_scale(810, 277, (9.1093826e-31 * 1000000000.0));

        s.b[1160] = (s.v[300] > 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if s.b[1160] {
            s.store_div_from_scalar(811, 1.0, 300);
        }

        if (!s.b[1160]) {
            s.store_scalar(811, 0.0);
        }

        s.b[1161] = (s.v[301] > 0.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if s.b[1161] {
            s.store_div_from_scalar(812, 1.0, 301);
        }

        if (!s.b[1161]) {
            s.store_scalar(812, 0.0);
        }

        s.b[1162] = (s.v[302] > 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if s.b[1162] {
            s.store_div_from_scalar(813, 1.0, 302);
        }

        if (!s.b[1162]) {
            s.store_scalar(813, 0.0);
        }

        s.b[1163] = (s.v[303] > 0.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if s.b[1163] {
            s.store_div_from_scalar(814, 1.0, 303);
        }

        if (!s.b[1163]) {
            s.store_scalar(814, 0.0);
        }

        s.b[1164] = (s.v[304] > 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if s.b[1164] {
            s.store_div_from_scalar(815, 1.0, 304);
        }

        if (!s.b[1164]) {
            s.store_scalar(815, 0.0);
        }

        s.b[1165] = (s.v[305] > 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if s.b[1165] {
            s.store_div_from_scalar(816, 1.0, 305);
        }

        if (!s.b[1165]) {
            s.store_scalar(816, 0.0);
        }

        s.b[1166] = (s.v[306] > 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if s.b[1166] {
            s.store_div_from_scalar(817, 1.0, 306);
        }

        if (!s.b[1166]) {
            s.store_scalar(817, 0.0);
        }

        s.store_scale(24, 6, s.v[646]);

        s.store_scale(25, 6, s.v[647]);

        s.store_scale(26, 6, s.v[648]);

        s.store_scale(27, 6, s.v[673]);

        s.store_scale(28, 6, s.v[674]);

        s.store_scale(29, 6, s.v[675]);

        s.v[30] = 0.0;

        s.b[1167] = (p.p43 == 3.0);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if s.b[1167] {
            s.store_scalar(30, 1.0);
        }

        s.copy_ad(31, 313);

        s.b[1168] = (p.p39 == 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if s.b[1168] {
            s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));
        }

        s.b[1169] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if s.b[1169] {
            s.store_scale(24, 6, s.v[649]);
            s.store_add_scaled_product_indices(25, 6, s.v[650], 30, 31, (-1.0));
            s.copy_ad(26, 31);
            s.store_scale(27, 6, s.v[676]);
            s.store_add_scaled_product_indices(28, 6, s.v[677], 30, 31, (-1.0));
            s.copy_ad(29, 31);
        }

        s.b[1170] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if s.b[1170] {
            if (s.v[24] > 0.0) {
                s.copy_ad(646, 24);
            } else {
                s.store_scalar(646, 0.0);
            }
        }

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1170] {
            if (s.v[25] > 0.0) {
                s.copy_ad(647, 25);
            } else {
                s.store_scalar(647, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[26] > 0.0) {
                s.copy_ad(648, 26);
            } else {
                s.store_scalar(648, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[27] > 0.0) {
                s.copy_ad(673, 27);
            } else {
                s.store_scalar(673, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[28] > 0.0) {
                s.copy_ad(674, 28);
            } else {
                s.store_scalar(674, 0.0);
            }
        }

        if s.b[1170] {
            if (s.v[29] > 0.0) {
                s.copy_ad(675, 29);
            } else {
                s.store_scalar(675, 0.0);
            }
        }

        if (!s.b[1170]) {
            s.store_scalar(646, 0.0);
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
        }

        s.v[656] = 0.0;

        s.v[683] = 0.0;

        s.v[658] = 0.0;

        s.v[685] = 0.0;

        s.v[657] = 0.0;

        s.v[684] = 0.0;

        s.v[659] = 0.0;

        s.v[686] = 0.0;

        s.v[654] = 0.0;

        s.v[681] = 0.0;

        s.v[655] = 0.0;

        s.v[682] = 0.0;

        s.v[667] = 0.0;

        s.v[694] = 0.0;

        s.v[668] = 1.0;

        s.v[695] = 1.0;

        s.v[669] = 0.0;

        s.v[696] = 0.0;

        s.v[670] = 1.0;

        s.v[697] = 1.0;

        s.v[671] = 0.0;

        s.v[698] = 0.0;

        s.v[672] = 1.0;

        s.v[699] = 1.0;

        s.v[666] = 0.0;

        s.v[693] = 0.0;

        s.v[660] = 0.0;

        s.v[687] = 0.0;

        s.v[661] = 0.0;

        s.v[688] = 0.0;

        s.v[662] = 0.0;

        s.v[689] = 0.0;

        s.v[663] = 0.0;

        s.v[690] = 0.0;

        s.v[664] = 0.0;

        s.v[691] = 0.0;

        s.v[665] = 0.0;

        s.v[692] = 0.0;

        s.v[651] = 1.0;

        s.v[678] = 1.0;

        s.v[652] = 1.0;

        s.v[679] = 1.0;

        s.v[653] = 1.0;

        s.v[680] = 1.0;

        s.v[491] = 0.0;

        s.v[492] = 0.0;

        s.v[480] = 0.0;

        s.v[481] = 0.0;

        s.v[482] = 0.0;

        s.v[483] = 0.0;

        s.v[484] = 0.0;

        s.v[493] = 0.0;

        s.v[494] = 0.0;

        s.v[495] = 0.0;

        s.v[501] = 0.0;

        s.v[490] = 0.0;

        s.b[1171] = (p.p43 > 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1172]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1172])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1173]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1173])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1174]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1174])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(654, 454, 455, 456);
        }

        s.b[1175] = ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1175]) {
            s.store_exp_scaled_input(655, 654, s.v[371]);
        }

        s.b[1176] = ((s.v[654] * s.v[371]) < 0.0);
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if ((s.b[1171] && (!s.b[1175])) && s.b[1176]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(655, 1e-100, (-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((s.b[1171] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_scaled_offset_ad(655, A::mul_offset_rhs(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(654), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.store_scalar(396, s.v[393]);
            s.store_scalar(397, s.v[394]);
            s.store_scalar(398, s.v[395]);
            s.store_scalar(399, p.p831);
            s.store_scalar(400, p.p832);
            s.store_scalar(401, p.p833);
            s.store_scalar(402, p.p828);
            s.store_scalar(403, p.p829);
            s.store_scalar(404, p.p830);
        }

        s.b[1177] = (s.v[646] == 0.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1177]) {
            s.store_scalar(396, (s.v[394] + s.v[395]));
            s.store_scalar(399, (0.9 * (p.p832).min(p.p833)));
            s.store_scalar(402, (p.p829 + p.p830));
        }

        s.b[1178] = (s.v[647] == 0.0);
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1178]) {
            s.store_scalar(397, (s.v[393] + s.v[395]));
            s.store_scalar(400, (0.9 * (p.p831).min(p.p833)));
            s.store_scalar(403, (p.p828 + p.p830));
        }

        s.b[1179] = (s.v[648] == 0.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1179]) {
            s.store_scalar(398, (s.v[393] + s.v[394]));
            s.store_scalar(401, (0.9 * (p.p831).min(p.p832)));
            s.store_scalar(404, (p.p828 + p.p829));
        }

        if s.b[1171] {
            s.store_min3(656, 396, 397, 398);
            s.store_scale(657, 656, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(658, 656, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(659, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1180] = ((s.v[563] * s.v[673]) > 0.0);
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1180]) {
            s.store_scaled_ln_ad(454, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(563), s.ad_value(673))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1180])) {
            s.store_scalar(454, 100000000.0);
        }

        s.b[1181] = ((s.v[564] * s.v[674]) > 0.0);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1181]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1181])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1182] = ((s.v[565] * s.v[675]) > 0.0);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1182]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);
        }

        if (s.b[1171] && (!s.b[1182])) {
            s.store_scalar(456, 100000000.0);
        }

        if s.b[1171] {
            s.store_min3(681, 454, 455, 456);
        }

        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1183]) {
            s.store_exp_scaled_input(682, 681, s.v[371]);
        }

        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((s.b[1171] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_scaled_offset_ad(682, A::mul_offset_rhs(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(681), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1171] {
            s.copy_ad(396, 569);
            s.copy_ad(397, 570);
            s.copy_ad(398, 571);
            s.copy_ad(399, 511);
            s.copy_ad(400, 512);
            s.copy_ad(401, 513);
            s.copy_ad(402, 508);
            s.copy_ad(403, 509);
            s.copy_ad(404, 510);
        }

        s.b[1185] = (s.v[673] == 0.0);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1185]) {
            s.store_add(396, 570, 571);
            s.store_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
            s.store_add(402, 509, 510);
        }

        s.b[1186] = (s.v[674] == 0.0);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1186]) {
            s.store_add(397, 569, 571);
            s.store_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);
            s.store_add(403, 508, 510);
        }

        s.b[1187] = (s.v[675] == 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1187]) {
            s.store_add(398, 569, 570);
            s.store_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);
            s.store_add(404, 508, 509);
        }

        if s.b[1171] {
            s.store_min3(683, 396, 397, 398);
            s.store_scale(684, 683, 0.1);
            s.store_max3(377, 399, 400, 401);
            s.store_mul_sub_from_scalar_ad_rhs(685, 683, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))));
            s.store_offset_min_ad(686, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));
        }

        s.b[1188] = (s.v[474] == 1.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if (s.b[1171] && s.b[1188]) {
            s.store_scalar(1189, 0.0);
            s.store_scalar(1190, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1171] && s.b[1188]) {
            s.store_scalar(1191, 0.0);
            s.store_scalar(1198, 0.0);
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
            s.store_scalar(1222, 0.0);
            s.store_scalar(1223, 0.0);
            s.store_scalar(1224, 0.0);
            s.store_scalar(1225, 0.0);
            s.store_scalar(1226, 0.0);
            s.store_scalar(1227, 0.0);
            s.store_scalar(1228, 0.0);
            s.store_scalar(1229, 0.0);
            s.store_scalar(1230, 0.0);
            s.store_scalar(1231, 0.0);
            s.store_scalar(1232, 0.0);
            s.store_scalar(1233, 0.0);
            s.store_scalar(498, 0.4);
            s.store_scalar(499, 0.65);
            s.store_scalar(500, 0.8);
            s.store_scale(485, 498, (-p.p928));
            s.store_scale(486, 499, (-p.p928));
            s.store_scale(487, 500, (-p.p928));
            s.store_scalar(488, 0.1);
            s.store_scalar(489, 0.2);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1237] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        s.b[1238] = (s.v[485] < s.v[654]);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        s.b[1239] = (((((-0.5) * (s.v[485] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && s.b[1239]) {
            s.store_exp_scaled_input(1200, 485, (s.v[371] * (-0.5)));
        }

        s.b[1240] = (((-0.5) * (s.v[485] * s.v[371])) < 0.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && (!s.b[1239])) && s.b[1240]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && (!s.b[1239])) && (!s.b[1240])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(485), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && (!s.b[1238])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(485), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1237]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1241] = (s.v[485] > 0.0);
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1241]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && (!s.b[1241])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 485);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1237]) {
            s.store_sub(1203, 656, 1202);
            s.store_add_scaled_inputs3(1204, s.ad_value(485), 0.5, s.ad_value(1203), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(485), s.ad_value(1203)), A::sub(s.ad_value(485), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370]))), (-0.5));
            s.store_add_scaled_inputs3(1205, s.ad_value(485), 0.5, s.ad_value(659), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(485), s.ad_value(659)), A::sub(s.ad_value(485), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368]))), (-0.5));
            s.store_scaled_sub_ad_rhs(1206, 485, A::sqrt(A::offset(A::mul(s.ad_value(485), s.ad_value(485)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1242] = (s.v[646] == 0.0);
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1242]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1242])) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.b[1243] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1243]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1244] = (p.p831 == 0.5);
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && s.b[1244]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1245] = (p.p831 == 0.5);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && s.b[1245]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && (!s.b[1245])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {
            s.store_scale(1214, 1207, s.v[423]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[384]);
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.b[1246] = (p.p845 == 0.0);
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1246]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[408] * s.v[438]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1247] = (((-p.p831) * s.v[411]) == (-1.0));
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1247]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1247])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[435]), s.ad_value(1218), s.ad_value(1221), s.v[435], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1248] = (s.v[1228] > 0.0);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1248]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1248])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1249] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1249]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1249])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1250] = (s.v[1228] > 0.0);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1250]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1251] = (s.v[1227] > (-230.25850929940458));
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) && s.b[1251]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) && (!s.b[1251])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[435] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);
        }

        s.b[1252] = (p.p851 == 0.0);
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1252]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1253] = (p.p831 == 0.5);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && s.b[1253]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1253])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[429]), ((p.p828) * (s.v[429]))), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), s.ad_value(1207), 1.0);
        }

        s.b[1254] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && s.b[1254]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1255] = (((-s.v[441]) / s.v[1232]) < 0.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1254])) && s.b[1255]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1254])) && (!s.b[1255])) {
            let assign16380_ad_e14449: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign16380_ad_e14449, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851);
        }

        s.b[1256] = (p.p860 > 1000.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1256]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1257] = (s.v[1206] > ((-s.v[444]) * p.p860));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        s.b[1258] = (p.p863 == 4.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) && s.b[1258]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) && (!s.b[1258])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && (!s.b[1257])) {
            s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1242])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1259] = (s.v[647] == 0.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1259]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1259])) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.b[1260] = ((p.p841 == 0.0) && (p.p846 == 0.0));
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1260]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1261] = (p.p832 == 0.5);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1261]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1261])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1262] = (p.p832 == 0.5);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1262]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1262])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {
            s.store_scale(1214, 1207, s.v[424]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[385]);
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.b[1263] = (p.p846 == 0.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1263]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[409] * s.v[439]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1264] = (((-p.p832) * s.v[412]) == (-1.0));
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1264]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1264])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[436]), s.ad_value(1218), s.ad_value(1221), s.v[436], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1265] = (s.v[1228] > 0.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1265]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1265])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1266] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1266]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1266])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1267] = (s.v[1228] > 0.0);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1267]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1268] = (s.v[1227] > (-230.25850929940458));
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) && s.b[1268]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) && (!s.b[1268])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[436] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);
        }

        s.b[1269] = (p.p852 == 0.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1269]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1270] = (p.p832 == 0.5);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && s.b[1270]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1270])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[430]), ((p.p829) * (s.v[430]))), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), s.ad_value(1207), 1.0);
        }

        s.b[1271] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && s.b[1271]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1272] = (((-s.v[442]) / s.v[1232]) < 0.0);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1271])) && s.b[1272]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1271])) && (!s.b[1272])) {
            let assign17080_ad_e15592: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign17080_ad_e15592, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852);
        }

        s.b[1273] = (p.p861 > 1000.0);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1273]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1274] = (s.v[1206] > ((-s.v[444]) * p.p861));
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        s.b[1275] = (p.p864 == 4.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) && s.b[1275]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) && (!s.b[1275])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && (!s.b[1274])) {
            s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1259])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1276] = (s.v[648] == 0.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1276]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1276])) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.b[1277] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1277]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1278] = (p.p833 == 0.5);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && s.b[1278]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && (!s.b[1278])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1279] = (p.p833 == 0.5);
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && s.b[1279]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && (!s.b[1279])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {
            s.store_scale(1214, 1207, s.v[425]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[386]);
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.b[1280] = (p.p847 == 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1280]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[410] * s.v[440]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1281] = (((-p.p833) * s.v[413]) == (-1.0));
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1281]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1281])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[437]), s.ad_value(1218), s.ad_value(1221), s.v[437], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1282] = (s.v[1228] > 0.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1282]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1282])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1283] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1283]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1283])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1284] = (s.v[1228] > 0.0);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1284]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1285] = (s.v[1227] > (-230.25850929940458));
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) && s.b[1285]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) && (!s.b[1285])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[437] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);
        }

        s.b[1286] = (p.p853 == 0.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1286]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1287] = (p.p833 == 0.5);
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && s.b[1287]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[431]), ((p.p830) * (s.v[431]))), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), s.ad_value(1207), 1.0);
        }

        s.b[1288] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && s.b[1288]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1289] = (((-s.v[443]) / s.v[1232]) < 0.0);
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1288])) && s.b[1289]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1288])) && (!s.b[1289])) {
            let assign17780_ad_e16735: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign17780_ad_e16735, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853);
        }

        s.b[1290] = (p.p862 > 1000.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1290]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1291] = (s.v[1206] > ((-s.v[444]) * p.p862));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        s.b[1292] = (p.p865 == 4.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) && s.b[1292]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) && (!s.b[1292])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && (!s.b[1291])) {
            s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1276])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(475, s.ad_value(646), s.ad_value(1234), 1.0, s.ad_value(647), s.ad_value(1235), 1.0, s.ad_value(648), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1293] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        s.b[1294] = (s.v[486] < s.v[654]);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        s.b[1295] = (((((-0.5) * (s.v[486] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && s.b[1295]) {
            s.store_exp_scaled_input(1200, 486, (s.v[371] * (-0.5)));
        }

        s.b[1296] = (((-0.5) * (s.v[486] * s.v[371])) < 0.0);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && (!s.b[1295])) && s.b[1296]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && (!s.b[1294])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(486), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1293]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1297] = (s.v[486] > 0.0);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1297]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && (!s.b[1297])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 486);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1293]) {
            s.store_sub(1203, 656, 1202);
            s.store_add_scaled_inputs3(1204, s.ad_value(486), 0.5, s.ad_value(1203), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(1203)), A::sub(s.ad_value(486), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370]))), (-0.5));
            s.store_add_scaled_inputs3(1205, s.ad_value(486), 0.5, s.ad_value(659), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(659)), A::sub(s.ad_value(486), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368]))), (-0.5));
            s.store_scaled_sub_ad_rhs(1206, 486, A::sqrt(A::offset(A::mul(s.ad_value(486), s.ad_value(486)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1298] = (s.v[646] == 0.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1298]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1298])) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.b[1299] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1299]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1300] = (p.p831 == 0.5);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && s.b[1300]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && (!s.b[1300])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1301] = (p.p831 == 0.5);
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && s.b[1301]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && (!s.b[1301])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {
            s.store_scale(1214, 1207, s.v[423]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[384]);
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.b[1302] = (p.p845 == 0.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1302]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[408] * s.v[438]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1303] = (((-p.p831) * s.v[411]) == (-1.0));
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1303]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1303])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[435]), s.ad_value(1218), s.ad_value(1221), s.v[435], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1304] = (s.v[1228] > 0.0);
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1304]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1304])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1305] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1305]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1305])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1306] = (s.v[1228] > 0.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1306]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1307] = (s.v[1227] > (-230.25850929940458));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) && s.b[1307]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) && (!s.b[1307])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[435] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);
        }

        s.b[1308] = (p.p851 == 0.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1308]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1309] = (p.p831 == 0.5);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && s.b[1309]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1309])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[429]), ((p.p828) * (s.v[429]))), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), s.ad_value(1207), 1.0);
        }

        s.b[1310] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && s.b[1310]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1311] = (((-s.v[441]) / s.v[1232]) < 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1310])) && s.b[1311]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1310])) && (!s.b[1311])) {
            let assign18780_ad_e18379: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign18780_ad_e18379, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851);
        }

        s.b[1312] = (p.p860 > 1000.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1312]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1313] = (s.v[1206] > ((-s.v[444]) * p.p860));
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        s.b[1314] = (p.p863 == 4.0);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) && s.b[1314]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) && (!s.b[1314])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && (!s.b[1313])) {
            s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1298])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1315] = (s.v[647] == 0.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1315]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1315])) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.b[1316] = ((p.p841 == 0.0) && (p.p846 == 0.0));
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1316]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1317] = (p.p832 == 0.5);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1318] = (p.p832 == 0.5);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1318]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1318])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {
            s.store_scale(1214, 1207, s.v[424]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[385]);
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.b[1319] = (p.p846 == 0.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1319]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[409] * s.v[439]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1320] = (((-p.p832) * s.v[412]) == (-1.0));
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1320]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1320])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[436]), s.ad_value(1218), s.ad_value(1221), s.v[436], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1321] = (s.v[1228] > 0.0);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1321]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1321])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1322] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1322]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1322])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1323] = (s.v[1228] > 0.0);
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1323]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1324] = (s.v[1227] > (-230.25850929940458));
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) && s.b[1324]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) && (!s.b[1324])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[436] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);
        }

        s.b[1325] = (p.p852 == 0.0);
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1325]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1326] = (p.p832 == 0.5);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && s.b[1326]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1326])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[430]), ((p.p829) * (s.v[430]))), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), s.ad_value(1207), 1.0);
        }

        s.b[1327] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && s.b[1327]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1328] = (((-s.v[442]) / s.v[1232]) < 0.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1327])) && s.b[1328]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1327])) && (!s.b[1328])) {
            let assign19480_ad_e19522: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign19480_ad_e19522, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852);
        }

        s.b[1329] = (p.p861 > 1000.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1329]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1330] = (s.v[1206] > ((-s.v[444]) * p.p861));
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        s.b[1331] = (p.p864 == 4.0);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) && s.b[1331]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) && (!s.b[1331])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && (!s.b[1330])) {
            s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1315])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1332] = (s.v[648] == 0.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1332]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1332])) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.b[1333] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1333]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1334] = (p.p833 == 0.5);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && s.b[1334]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && (!s.b[1334])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1335] = (p.p833 == 0.5);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && s.b[1335]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && (!s.b[1335])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {
            s.store_scale(1214, 1207, s.v[425]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[386]);
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.b[1336] = (p.p847 == 0.0);
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1336]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[410] * s.v[440]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1337] = (((-p.p833) * s.v[413]) == (-1.0));
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1337]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1337])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[437]), s.ad_value(1218), s.ad_value(1221), s.v[437], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1338] = (s.v[1228] > 0.0);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1338]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1338])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1339] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1339]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1339])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1340] = (s.v[1228] > 0.0);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1340]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1341] = (s.v[1227] > (-230.25850929940458));
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1340])) && s.b[1341]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1340])) && (!s.b[1341])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1340])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[437] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);
        }

        s.b[1342] = (p.p853 == 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1342]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1343] = (p.p833 == 0.5);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && s.b[1343]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1343])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[431]), ((p.p830) * (s.v[431]))), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), s.ad_value(1207), 1.0);
        }

        s.b[1344] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && s.b[1344]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1345] = (((-s.v[443]) / s.v[1232]) < 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1344])) && s.b[1345]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1344])) && (!s.b[1345])) {
            let assign20180_ad_e20665: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign20180_ad_e20665, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853);
        }

        s.b[1346] = (p.p862 > 1000.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1346]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1347] = (s.v[1206] > ((-s.v[444]) * p.p862));
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        s.b[1348] = (p.p865 == 4.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) && s.b[1348]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) && (!s.b[1348])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1332])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(476, s.ad_value(646), s.ad_value(1234), 1.0, s.ad_value(647), s.ad_value(1235), 1.0, s.ad_value(648), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1349] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        s.b[1350] = (s.v[487] < s.v[654]);
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        s.b[1351] = (((((-0.5) * (s.v[487] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) && s.b[1351]) {
            s.store_exp_scaled_input(1200, 487, (s.v[371] * (-0.5)));
        }

        s.b[1352] = (((-0.5) * (s.v[487] * s.v[371])) < 0.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) && (!s.b[1351])) && s.b[1352]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) && (!s.b[1351])) && (!s.b[1352])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && (!s.b[1350])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(487), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1349]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1353] = (s.v[487] > 0.0);
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1353]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && (!s.b[1353])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 487);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1349]) {
            s.store_sub(1203, 656, 1202);
            s.store_add_scaled_inputs3(1204, s.ad_value(487), 0.5, s.ad_value(1203), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(1203)), A::sub(s.ad_value(487), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370]))), (-0.5));
            s.store_add_scaled_inputs3(1205, s.ad_value(487), 0.5, s.ad_value(659), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(659)), A::sub(s.ad_value(487), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368]))), (-0.5));
            s.store_scaled_sub_ad_rhs(1206, 487, A::sqrt(A::offset(A::mul(s.ad_value(487), s.ad_value(487)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1354] = (s.v[646] == 0.0);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1354]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1354])) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.b[1355] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1355]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1356] = (p.p831 == 0.5);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && s.b[1356]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1356])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1357] = (p.p831 == 0.5);
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && s.b[1357]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1357])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {
            s.store_scale(1214, 1207, s.v[423]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[384]);
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.b[1358] = (p.p845 == 0.0);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1358]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[408] * s.v[438]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1359] = (((-p.p831) * s.v[411]) == (-1.0));
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1359]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1359])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[435]), s.ad_value(1218), s.ad_value(1221), s.v[435], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1360] = (s.v[1228] > 0.0);
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1360]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1360])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1361] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1361]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1361])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1362] = (s.v[1228] > 0.0);
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1362]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1363] = (s.v[1227] > (-230.25850929940458));
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1362])) && s.b[1363]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1362])) && (!s.b[1363])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1362])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[435] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);
        }

        s.b[1364] = (p.p851 == 0.0);
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1364]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1365] = (p.p831 == 0.5);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && s.b[1365]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && (!s.b[1365])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[429]), ((p.p828) * (s.v[429]))), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), s.ad_value(1207), 1.0);
        }

        s.b[1366] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && s.b[1366]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1367] = (((-s.v[441]) / s.v[1232]) < 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && (!s.b[1366])) && s.b[1367]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && (!s.b[1366])) && (!s.b[1367])) {
            let assign21180_ad_e22309: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign21180_ad_e22309, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851);
        }

        s.b[1368] = (p.p860 > 1000.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1368]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1369] = (s.v[1206] > ((-s.v[444]) * p.p860));
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        s.b[1370] = (p.p863 == 4.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && s.b[1369]) && s.b[1370]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && s.b[1369]) && (!s.b[1370])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && s.b[1369]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && (!s.b[1369])) {
            s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1354])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1371] = (s.v[647] == 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1371]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1371])) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.b[1372] = ((p.p841 == 0.0) && (p.p846 == 0.0));
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1372]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1373] = (p.p832 == 0.5);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && s.b[1373]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && (!s.b[1373])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1374] = (p.p832 == 0.5);
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && s.b[1374]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && (!s.b[1374])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) {
            s.store_scale(1214, 1207, s.v[424]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[385]);
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.b[1375] = (p.p846 == 0.0);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1375]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[409] * s.v[439]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1376] = (((-p.p832) * s.v[412]) == (-1.0));
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1376]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1376])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[436]), s.ad_value(1218), s.ad_value(1221), s.v[436], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1377] = (s.v[1228] > 0.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1377]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1377])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1378] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1378]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1378])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1379] = (s.v[1228] > 0.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1379]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1380] = (s.v[1227] > (-230.25850929940458));
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1379])) && s.b[1380]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1379])) && (!s.b[1380])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1379])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[436] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);
        }

        s.b[1381] = (p.p852 == 0.0);
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1381]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1382] = (p.p832 == 0.5);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && s.b[1382]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && (!s.b[1382])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[430]), ((p.p829) * (s.v[430]))), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), s.ad_value(1207), 1.0);
        }

        s.b[1383] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && s.b[1383]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1384] = (((-s.v[442]) / s.v[1232]) < 0.0);
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && (!s.b[1383])) && s.b[1384]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && (!s.b[1383])) && (!s.b[1384])) {
            let assign21880_ad_e23452: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign21880_ad_e23452, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852);
        }

        s.b[1385] = (p.p861 > 1000.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1385]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1386] = (s.v[1206] > ((-s.v[444]) * p.p861));
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        s.b[1387] = (p.p864 == 4.0);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && s.b[1386]) && s.b[1387]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && s.b[1386]) && (!s.b[1387])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && s.b[1386]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && (!s.b[1386])) {
            s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1371])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1388] = (s.v[648] == 0.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1388]) {
            s.store_scalar(1236, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1171] && s.b[1188]) && (!s.b[1388])) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.b[1389] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1389]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1390] = (p.p833 == 0.5);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && s.b[1390]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && (!s.b[1390])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1391] = (p.p833 == 0.5);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && s.b[1391]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && (!s.b[1391])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) {
            s.store_scale(1214, 1207, s.v[425]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[386]);
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.b[1392] = (p.p847 == 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1392]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[410] * s.v[440]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1393] = (((-p.p833) * s.v[413]) == (-1.0));
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1393]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1393])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[437]), s.ad_value(1218), s.ad_value(1221), s.v[437], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1394] = (s.v[1228] > 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1394]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1394])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1395] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1395]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1395])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1396] = (s.v[1228] > 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1396]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1397] = (s.v[1227] > (-230.25850929940458));
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1396])) && s.b[1397]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1396])) && (!s.b[1397])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_rhs(1207, 1e-100, (-230.25850929940458), 1227, A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1227), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1396])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {
            s.store_div_scaled_inputs(1230, s.ad_value(1229), (s.v[437] * (1.772453850905516 * 0.5)), s.ad_value(1225), 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);
        }

        s.b[1398] = (p.p853 == 0.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1398]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1399] = (p.p833 == 0.5);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && s.b[1399]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && (!s.b[1399])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[431]), ((p.p830) * (s.v[431]))), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), s.ad_value(1207), 1.0);
        }

        s.b[1400] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && s.b[1400]) {
            s.store_exp_ad(1207, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));
        }

        s.b[1401] = (((-s.v[443]) / s.v[1232]) < 0.0);
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && (!s.b[1400])) && s.b[1401]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && (!s.b[1400])) && (!s.b[1401])) {
            let assign22580_ad_e24595: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign22580_ad_e24595, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853);
        }

        s.b[1402] = (p.p862 > 1000.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1402]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1403] = (s.v[1206] > ((-s.v[444]) * p.p862));
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        s.b[1404] = (p.p865 == 4.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && s.b[1403]) && s.b[1404]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && s.b[1403]) && (!s.b[1404])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && s.b[1403]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && (!s.b[1403])) {
            s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1388])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(477, s.ad_value(646), s.ad_value(1234), 1.0, s.ad_value(647), s.ad_value(1235), 1.0, s.ad_value(648), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1405] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        s.b[1406] = (s.v[488] < s.v[654]);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        s.b[1407] = (((((-0.5) * (s.v[488] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) && s.b[1407]) {
            s.store_exp_scaled_input(1200, 488, (s.v[371] * (-0.5)));
        }

        s.b[1408] = (((-0.5) * (s.v[488] * s.v[371])) < 0.0);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) && (!s.b[1407])) && s.b[1408]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) && (!s.b[1407])) && (!s.b[1408])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(488), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(488), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(488), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && (!s.b[1406])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(488), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1405]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1409] = (s.v[488] > 0.0);
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1409]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && (!s.b[1409])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 488);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1405]) {
            s.store_sub(1203, 656, 1202);
            s.store_add_scaled_inputs3(1204, s.ad_value(488), 0.5, s.ad_value(1203), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(1203)), A::sub(s.ad_value(488), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370]))), (-0.5));
            s.store_add_scaled_inputs3(1205, s.ad_value(488), 0.5, s.ad_value(659), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(659)), A::sub(s.ad_value(488), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368]))), (-0.5));
            s.store_scaled_sub_ad_rhs(1206, 488, A::sqrt(A::offset(A::mul(s.ad_value(488), s.ad_value(488)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1410] = (s.v[646] == 0.0);
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1410]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1410])) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.b[1411] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && s.b[1411]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1412] = (p.p831 == 0.5);
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && s.b[1412]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && (!s.b[1412])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1413] = (p.p831 == 0.5);
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && s.b[1413]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && (!s.b[1413])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) {
            s.store_scale(1214, 1207, s.v[423]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[384]);
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.b[1414] = (p.p845 == 0.0);
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && s.b[1414]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) {
            s.store_div_scaled_inputs(1217, s.ad_value(1214), (s.v[408] * s.v[438]), s.ad_value(1210), 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1415] = (((-p.p831) * s.v[411]) == (-1.0));
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1415]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1415])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) {
            s.store_div_scaled_product_denominator_ad(1224, 1213, 1223, 1.0, A::add(s.ad_value(1213), s.ad_value(1223)), 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[435]), s.ad_value(1218), s.ad_value(1221), s.v[435], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1416] = (s.v[1228] > 0.0);
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1416]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1416])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1417] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1417]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1417])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1418] = (s.v[1228] > 0.0);
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1418]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1419] = (s.v[1227] > (-230.25850929940458));
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

    }
}
