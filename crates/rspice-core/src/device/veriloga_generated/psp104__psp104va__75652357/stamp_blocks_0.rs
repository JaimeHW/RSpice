#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
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
        s.v[984] = if (p.p37 >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[984] != 0.0) {
            s.store_scalar(0, 1.0);
        }

        if (!(s.v[984] != 0.0)) {
            s.store_scalar(0, (-1.0));
        }

        s.v[761] = (8.8541878176e-12 * 11.8);

        s.v[344] = (273.15 + p.p38);

        s.v[468] = 0.0;

        s.v[985] = if (p.p920 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[985] != 0.0) {
            s.store_scalar(468, 1.0);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(468, 0.0);
        }

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

        s.v[986] = if ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[986] != 0.0) {
            s.store_scalar(467, 1.0);
        }

        if (!(s.v[986] != 0.0)) {
            s.store_scalar(467, 0.0);
        }

        s.v[987] = if (s.v[467] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[987] != 0.0) {
            s.store_scalar(451, (if ((p.p820 * p.p859) > 1e-18) { (p.p820 * p.p859) } else { 1e-18 }));
        }

        if (s.v[987] != 0.0) {
            s.store_scalar(452, (if ((p.p823 * p.p860) > 0.05) { (p.p823 * p.p860) } else { 0.05 }));
        }

        if (s.v[987] != 0.0) {
            s.store_scalar(453, (if ((if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) < 0.95) { (if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) } else { 0.95 }));
        }

        if (s.v[987] != 0.0) {
            s.store_scalar(454, (p.p829 * p.p862));
        }

        if (s.v[987] != 0.0) {
            s.store_offset(456, 454, s.v[369]);
        }

        if (s.v[987] != 0.0) {
            s.store_sub_from_scalar(461, 1.0, 453);
        }

        if (s.v[987] != 0.0) {
            s.store_div_from_scalar(462, 1.0, 461);
        }

        s.v[988] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[988] != 0.0) {
            s.store_scalar(499, p.p818);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(500, p.p819);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(501, p.p820);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(502, p.p821);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(503, p.p822);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(504, p.p823);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(505, p.p824);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(506, p.p825);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(507, p.p826);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(508, p.p827);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(509, p.p828);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(510, p.p829);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(511, p.p830);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(512, p.p831);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(513, p.p832);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(516, p.p833);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(517, p.p834);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(518, p.p835);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(514, p.p836);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(515, p.p837);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(519, p.p838);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(520, p.p839);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(521, p.p840);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(522, p.p841);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(523, p.p842);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(524, p.p843);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(525, p.p844);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(526, p.p845);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(527, p.p846);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(528, p.p847);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(529, p.p848);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(530, p.p849);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(531, p.p850);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(532, p.p851);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(533, p.p852);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(534, p.p853);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(535, p.p854);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(536, p.p855);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(537, p.p856);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(538, p.p857);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(539, p.p858);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(546, p.p921);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(547, p.p922);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(630, p.p865);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(631, p.p866);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(632, p.p867);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(633, p.p868);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(540, p.p859);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(541, p.p860);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(542, p.p861);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(543, p.p862);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(544, p.p863);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(545, p.p864);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(499, p.p869);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(500, p.p870);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(501, p.p871);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(502, p.p872);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(503, p.p873);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(504, p.p874);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(505, p.p875);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(506, p.p876);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(507, p.p877);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(508, p.p878);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(509, p.p879);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(510, p.p880);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(511, p.p881);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(512, p.p882);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(513, p.p883);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(516, p.p884);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(517, p.p885);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(518, p.p886);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(514, p.p887);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(515, p.p888);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(519, p.p889);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(520, p.p890);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(521, p.p891);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(522, p.p892);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(523, p.p893);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(524, p.p894);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(525, p.p895);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(526, p.p896);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(527, p.p897);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(528, p.p898);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(529, p.p899);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(530, p.p900);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(531, p.p901);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(532, p.p902);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(533, p.p903);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(534, p.p904);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(535, p.p905);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(536, p.p906);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(537, p.p907);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(538, p.p908);
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
        if (!(s.v[988] != 0.0)) {
            s.store_scalar(539, p.p909);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(546, p.p923);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(547, p.p924);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(630, p.p916);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(631, p.p917);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(632, p.p918);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(633, p.p919);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(540, p.p910);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(541, p.p911);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(542, p.p912);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(543, p.p913);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(544, p.p914);
        }

        if (!(s.v[988] != 0.0)) {
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

        s.store_div_ad_lhs(585, A::scale(s.ad_value(514), s.v[761]), 500);

        s.store_div_ad_lhs(586, A::scale(s.ad_value(515), s.v[761]), 501);

        s.store_div_from_scalar(587, 1.0, 584);

        s.store_div_from_scalar(588, 1.0, 585);

        s.store_div_from_scalar(589, 1.0, 586);

        s.store_div_from_scalar(590, 1.0, 502);

        s.store_div_from_scalar(591, 1.0, 503);

        s.store_div_from_scalar(592, 1.0, 504);

        s.store_div_from_scalar_ad(605, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[438], s.ad_value(537))));

        s.store_div_from_scalar_ad(606, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[438], s.ad_value(538))));

        s.store_div_from_scalar_ad(607, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[438], s.ad_value(539))));

        s.store_div_from_scalar(608, 1.0, 534);

        s.store_div_from_scalar(609, 1.0, 535);

        s.store_div_from_scalar(610, 1.0, 536);

        s.store_mul_ad_lhs(611, A::mul(A::neg(A::mul(A::square(s.ad_value(605)), A::pow_from_scalar(s.v[438], A::offset(s.ad_value(537), (-1.0))))), s.ad_value(537)), 608);

        s.store_mul_ad_lhs(612, A::mul(A::neg(A::mul(A::square(s.ad_value(606)), A::pow_from_scalar(s.v[438], A::offset(s.ad_value(538), (-1.0))))), s.ad_value(538)), 609);

        s.store_mul_ad_lhs(613, A::mul(A::neg(A::mul(A::square(s.ad_value(607)), A::pow_from_scalar(s.v[438], A::offset(s.ad_value(539), (-1.0))))), s.ad_value(539)), 610);

        s.v[989] = if ((((s.v[540] != 1.0) || (s.v[541] != 1.0)) || (s.v[542] != 1.0)) || (s.v[543] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[989] != 0.0) {
            s.store_scalar(629, 1.0);
        }

        if (!(s.v[989] != 0.0)) {
            s.store_scalar(629, 0.0);
        }

        s.v[990] = if (s.v[629] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[990] != 0.0) {
            s.store_ad(614, &{
                if ((s.v[501] * s.v[540]) > 1e-18) {
                    A::mul(s.ad_value(501), s.ad_value(540))
                } else {
                    A::constant(1e-18)
                }
            });
        }

        if (s.v[990] != 0.0) {
            s.store_ad(615, &{
                if ((s.v[504] * s.v[541]) > 0.05) {
                    A::mul(s.ad_value(504), s.ad_value(541))
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[990] != 0.0) {
            s.store_ad(616, &{
                if ((if ((s.v[507] * s.v[542]) > 0.05) { (s.v[507] * s.v[542]) } else { 0.05 }) < 0.95) {
                    {
                        if ((s.v[507] * s.v[542]) > 0.05) {
                            A::mul(s.ad_value(507), s.ad_value(542))
                        } else {
                            A::constant(0.05)
                        }
                    }
                } else {
                    A::constant(0.95)
                }
            });
        }

        if (s.v[990] != 0.0) {
            s.store_mul(617, 510, 543);
        }

        if (s.v[990] != 0.0) {
            s.store_offset(619, 617, s.v[369]);
        }

        if (s.v[990] != 0.0) {
            s.store_sub_from_scalar(624, 1.0, 616);
        }

        if (s.v[990] != 0.0) {
            s.store_div_from_scalar(625, 1.0, 624);
        }

        s.v[872] = 0.0;

        s.v[345] = ((ctx.temperature() + p.p55) + p.p35);

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

        if !(s.v[357] > 0.001) {
            s.store_scalar(357, 0.001);
        }

        s.v[712] = ((4.0 * 1.3806505e-23) * s.v[350]);

        s.v[359] = (((ctx.temperature() + p.p55) + p.p35)).max((273.15 + (-250.0)));

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

        if !(s.v[435] > 0.0) {
            s.store_scalar(435, 0.0);
        }

        if !(s.v[436] > 0.0) {
            s.store_scalar(436, 0.0);
        }

        if !(s.v[437] > 0.0) {
            s.store_scalar(437, 0.0);
        }

        s.v[1010] = if (s.v[467] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1010] != 0.0) {
            s.store_offset(455, 454, s.v[370]);
        }

        if (s.v[1010] != 0.0) {
            s.store_scale_ad(457, A::exp(A::scale(A::sub(A::scale(s.ad_value(456), s.v[363]), A::scale(s.ad_value(455), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));
        }

        if (s.v[1010] != 0.0) {
            s.store_sub_ad(458, A::scale(s.ad_value(452), s.v[360]), A::scale(A::ln(s.ad_value(457)), (2.0 * s.v[364])));
        }

        if (s.v[1010] != 0.0) {
            s.store_add_ad_rhs(459, 458, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(458)), s.v[365])), 1.0)), s.v[364]));
        }

        if (s.v[1010] != 0.0) {
            s.store_div_from_scalar(460, 1.0, 459);
        }

        if (s.v[1010] != 0.0) {
            s.store_mul_ad_rhs(463, 451, A::pow(A::mul(s.ad_value(452), s.ad_value(460)), s.ad_value(453)));
        }

        if (s.v[1010] != 0.0) {
            s.store_mul_ad_lhs(464, A::mul(s.ad_value(463), s.ad_value(459)), 462);
        }

        if (s.v[1010] != 0.0) {
            s.store_scale(465, 463, 2.0);
        }

        s.store_offset(551, 508, s.v[370]);

        s.store_offset(552, 509, s.v[370]);

        s.store_offset(553, 510, s.v[370]);

        s.store_scale_ad(554, A::exp(A::scale(A::sub(A::scale(s.ad_value(548), s.v[363]), A::scale(s.ad_value(551), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(555, A::exp(A::scale(A::sub(A::scale(s.ad_value(549), s.v[363]), A::scale(s.ad_value(552), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(556, A::exp(A::scale(A::sub(A::scale(s.ad_value(550), s.v[363]), A::scale(s.ad_value(553), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));

        s.store_mul_ad_lhs(557, A::mul(s.ad_value(511), s.ad_value(554)), 554);

        s.store_mul_ad_lhs(558, A::mul(s.ad_value(512), s.ad_value(555)), 555);

        s.store_mul_ad_lhs(559, A::mul(s.ad_value(513), s.ad_value(556)), 556);

        s.store_sub_ad(560, A::scale(s.ad_value(502), s.v[360]), A::scale(A::ln(s.ad_value(554)), (2.0 * s.v[364])));

        s.store_sub_ad(561, A::scale(s.ad_value(503), s.v[360]), A::scale(A::ln(s.ad_value(555)), (2.0 * s.v[364])));

        s.store_sub_ad(562, A::scale(s.ad_value(504), s.v[360]), A::scale(A::ln(s.ad_value(556)), (2.0 * s.v[364])));

        s.store_add_ad_rhs(563, 560, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(560)), s.v[365])), 1.0)), s.v[364]));

        s.store_add_ad_rhs(564, 561, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(561)), s.v[365])), 1.0)), s.v[364]));

        s.store_add_ad_rhs(565, 562, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(562)), s.v[365])), 1.0)), s.v[364]));

        s.store_div_from_scalar(566, 1.0, 563);

        s.store_div_from_scalar(567, 1.0, 564);

        s.store_div_from_scalar(568, 1.0, 565);

        s.store_mul_ad_rhs(575, 499, A::pow(A::mul(s.ad_value(502), s.ad_value(566)), s.ad_value(505)));

        s.store_mul_ad_rhs(576, 500, A::pow(A::mul(s.ad_value(503), s.ad_value(567)), s.ad_value(506)));

        s.store_mul_ad_rhs(577, 501, A::pow(A::mul(s.ad_value(504), s.ad_value(568)), s.ad_value(507)));

        s.store_mul_ad_lhs(578, A::mul(s.ad_value(575), s.ad_value(563)), 572);

        s.store_mul_ad_lhs(579, A::mul(s.ad_value(576), s.ad_value(564)), 573);

        s.store_mul_ad_lhs(580, A::mul(s.ad_value(577), s.ad_value(565)), 574);

        s.store_scale(581, 575, 2.0);

        s.store_scale(582, 576, 2.0);

        s.store_scale(583, 577, 2.0);

        s.store_max_with_scalar_ad(593, A::scale(s.ad_value(551), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(594, A::scale(s.ad_value(552), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(595, A::scale(s.ad_value(553), 0.5), s.v[364]);

        s.store_scale(596, 593, s.v[365]);

        s.store_scale(597, 594, s.v[365]);

        s.store_scale(598, 595, s.v[365]);

        s.store_scale_ad(599, A::sqrt(A::mul(A::scale(s.ad_value(522), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(593)), s.ad_value(593)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(600, A::sqrt(A::mul(A::scale(s.ad_value(523), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(594)), s.ad_value(594)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(601, A::sqrt(A::mul(A::scale(s.ad_value(524), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(595)), s.ad_value(595)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_ad_rhs(602, 528, A::offset(A::scale(s.ad_value(531), (s.v[359] - s.v[358])), 1.0));

        s.store_mul_ad_rhs(603, 529, A::offset(A::scale(s.ad_value(532), (s.v[359] - s.v[358])), 1.0));

        s.store_mul_ad_rhs(604, 530, A::offset(A::scale(s.ad_value(533), (s.v[359] - s.v[358])), 1.0));

        if !(s.v[602] > 0.0) {
            s.store_scalar(602, 0.0);
        }

        if !(s.v[603] > 0.0) {
            s.store_scalar(603, 0.0);
        }

        if !(s.v[604] > 0.0) {
            s.store_scalar(604, 0.0);
        }

        s.v[1011] = if (s.v[629] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1011] != 0.0) {
            s.store_offset(618, 617, s.v[370]);
        }

        if (s.v[1011] != 0.0) {
            s.store_scale_ad(620, A::exp(A::scale(A::sub(A::scale(s.ad_value(619), s.v[363]), A::scale(s.ad_value(618), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));
        }

        if (s.v[1011] != 0.0) {
            s.store_sub_ad(621, A::scale(s.ad_value(615), s.v[360]), A::scale(A::ln(s.ad_value(620)), (2.0 * s.v[364])));
        }

        if (s.v[1011] != 0.0) {
            s.store_add_ad_rhs(622, 621, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(621)), s.v[365])), 1.0)), s.v[364]));
        }

        if (s.v[1011] != 0.0) {
            s.store_div_from_scalar(623, 1.0, 622);
        }

        if (s.v[1011] != 0.0) {
            s.store_mul_ad_rhs(626, 614, A::pow(A::mul(s.ad_value(615), s.ad_value(623)), s.ad_value(616)));
        }

        if (s.v[1011] != 0.0) {
            s.store_mul_ad_lhs(627, A::mul(s.ad_value(626), s.ad_value(622)), 625);
        }

        if (s.v[1011] != 0.0) {
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

        s.v[1012] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

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
        if (s.v[1012] != 0.0) {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if (s.v[1012] != 0.0) {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
        }

        if (s.v[1012] != 0.0) {
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

        s.store_scale_ad(304, A::offset(A::scale(s.ad_value(303), p.p188), 1.0), (p.p186 * (1.0 + (p.p187 * s.v[302]))));

        s.store_scale_ad(305, A::offset(A::scale(s.ad_value(303), p.p192), 1.0), (p.p190 * (1.0 + (p.p191 * s.v[302]))));

        if (((s.v[3] + s.v[304]) - (2.0 * p.p189)) > 1e-9) {
            s.store_offset(306, 304, ((s.v[3]) + ((-(2.0 * p.p189)))));
        } else {
            s.store_scalar(306, 1e-9);
        }

        if (((s.v[4] + s.v[305]) - (2.0 * p.p193)) > 1e-9) {
            s.store_offset_ad(307, A::add(s.ad_value(4), s.ad_value(305)), (-(2.0 * p.p193)));
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
            s.store_offset_ad(314, A::offset(A::offset(s.ad_value(304), s.v[3]), (-(2.0 * p.p189))), p.p194);
        } else {
            s.store_scalar(314, 1e-9);
        }

        if ((((s.v[4] + s.v[305]) - (2.0 * p.p193)) + p.p195) > 1e-9) {
            s.store_offset_ad(315, A::offset(A::add(s.ad_value(4), s.ad_value(305)), (-(2.0 * p.p193))), p.p195);
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
            s.store_offset_ad(318, A::add(s.ad_value(4), s.ad_value(305)), p.p195);
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
            s.store_sub_from_scalar_ad(324, s.v[9], A::scale(s.ad_value(305), 0.5));
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

        s.v[1013] = if (if self.param_given[121] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1013] != 0.0) {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.v[1014] = if (if self.param_given[122] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1014] != 0.0) {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.v[1015] = if (if self.param_given[123] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1015] != 0.0) {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.v[1016] = if (if self.param_given[124] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1016] != 0.0) {
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

        s.v[1017] = if (if self.param_given[137] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1017] != 0.0) {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.v[1018] = if (if self.param_given[138] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1018] != 0.0) {
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

        s.v[141] = p.p157;

        s.v[142] = p.p158;

        s.v[143] = p.p159;

        s.v[144] = p.p160;

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

        s.v[158] = p.p174;

        s.v[159] = p.p175;

        s.v[160] = p.p176;

        s.v[161] = p.p177;

        s.v[162] = p.p178;

        s.v[163] = p.p179;

        s.v[166] = p.p180;

        s.v[167] = p.p181;

        s.v[168] = p.p183;

        s.v[169] = p.p182;

        s.v[170] = p.p184;

        s.v[171] = p.p185;

        s.v[1019] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1019] != 0.0) {
            s.store_add_ad(40, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p198), p.p197), p.p196), A::scale(s.ad_value(310), p.p199)), A::scale(s.ad_value(312), p.p200));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(308), p.p202), p.p201), A::scale(s.ad_value(310), p.p203)), A::scale(s.ad_value(312), p.p204));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(42, p.p205);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(43, p.p206);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(44, p.p207);
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(325, &A::scale({
                if ((1.0 + ((p.p209 * s.v[310]) * (((1.0 + (s.v[307] / p.p210))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(310), p.p209), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p210)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p208));
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(326, &A::scale({
                if ((1.0 + ((p.p212 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(310), p.p212), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p213)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211));
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(327, &A::scale({
                if ((1.0 + ((p.p215 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(310), p.p215), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p213)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214));
        }

        s.v[1020] = if (s.v[306] > (2.0 * s.v[327])) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_scalar(328, 75000000000.0);
        }

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_sub_ad(329, A::sqrt(A::add(s.ad_value(325), A::scale(s.ad_value(326), 0.5))), A::sqrt(s.ad_value(325)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_add_ad(330, A::sqrt(s.ad_value(325)), A::mul(s.ad_value(328), A::ln(A::offset(A::mul(A::div(A::scale(s.ad_value(327), 2.0), s.ad_value(306)), A::offset(A::exp(A::div(s.ad_value(329), s.ad_value(328))), (-1.0))), 1.0))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_square(330, 330);
        }

        s.v[1021] = if (s.v[306] >= s.v[327]) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (s.v[1021] != 0.0)) {
            s.store_add_ad_rhs(330, 325, A::div(A::mul(s.ad_value(326), s.ad_value(327)), s.ad_value(306)));
        }

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (!(s.v[1021] != 0.0))) {
            s.store_add_ad_rhs(330, 325, A::mul(s.ad_value(326), A::sub_from_scalar(2.0, A::div(s.ad_value(306), s.ad_value(327)))));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_rhs(45, 330, A::sub(A::sub_from_scalar(1.0, A::scale(s.ad_value(308), p.p216)), A::scale(s.ad_value(309), p.p217)));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(46, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p220), p.p219), p.p218), A::scale(s.ad_value(310), p.p221)), A::scale(s.ad_value(312), p.p222));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(47, p.p223);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(48, p.p224);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(49, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p227), p.p226), p.p225), A::scale(s.ad_value(310), p.p228)), A::scale(s.ad_value(312), p.p229));
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
        if (s.v[1019] != 0.0) {
            s.store_ad(50, &A::scale({
                if (1e-6 > (1.0 + (p.p231 * s.v[308]))) {
                    A::constant(1e-6)
                } else {
                    A::offset(A::scale(s.ad_value(308), p.p231), 1.0)
                }
            }, p.p230));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(55, p.p232);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(56, p.p233);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(57, p.p236);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(58, p.p237);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(51, A::mul(A::offset(A::scale(A::powf(s.ad_value(308), p.p240), p.p239), p.p238), A::offset(A::scale(s.ad_value(310), p.p241), 1.0)), A::offset(A::scale(s.ad_value(312), p.p242), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(52, p.p244);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(53, p.p243);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(54, p.p245);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(62, A::scale(A::powf(s.ad_value(308), p.p247), p.p246), A::offset(A::scale(s.ad_value(310), p.p248), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(63, p.p250);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(64, p.p249);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(59, A::scale(A::powf(s.ad_value(308), p.p252), p.p251), A::offset(A::scale(s.ad_value(310), p.p253), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(60, p.p255);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(61, p.p254);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale_ad(331, A::offset(A::scale(s.ad_value(310), p.p258), 1.0), p.p257);
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(332, &A::scale({
                if ((1.0 + (p.p260 * s.v[310])) > 0.001) {
                    A::offset(A::scale(s.ad_value(310), p.p260), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p259));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(333, A::offset(A::mul(A::div(A::mul(s.ad_value(331), s.ad_value(332)), s.ad_value(306)), A::sub_from_scalar(1.0, A::exp(A::div(A::neg(s.ad_value(306)), s.ad_value(332))))), 1.0), A::mul(A::div_from_scalar((p.p261 * p.p262), s.ad_value(306)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(306)), 1.0 / (p.p262))))));
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(333, &{
                if (s.v[333] > 1e-15) {
                    s.ad_value(333)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(334, A::offset(A::scale(s.ad_value(310), p.p263), 1.0), A::mul(A::scale(s.ad_value(310), p.p264), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p265)), 1.0))));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_lhs(65, A::div(A::scale(s.ad_value(307), p.p256), A::mul(s.ad_value(333), s.ad_value(306))), 334);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(308), p.p267), p.p266), A::scale(s.ad_value(310), p.p268)), A::scale(s.ad_value(312), p.p269));
        }

        if (s.v[1019] != 0.0) {
            s.store_scale_ad(67, A::offset(A::scale(s.ad_value(310), p.p271), 1.0), p.p270);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(68, p.p272);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(69, p.p273);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(70, p.p274);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(71, A::mul(A::offset(A::scale(A::powf(s.ad_value(308), p.p277), p.p276), p.p275), A::offset(A::scale(s.ad_value(310), p.p278), 1.0)), A::offset(A::scale(s.ad_value(312), p.p279), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(72, p.p280);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(73, p.p281);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(74, p.p282);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(75, A::mul(A::scale(A::offset(A::scale(s.ad_value(308), p.p284), 1.0), p.p283), A::offset(A::scale(s.ad_value(310), p.p285), 1.0)), A::offset(A::scale(s.ad_value(312), p.p286), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(76, p.p287);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(77, p.p288);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(78, A::scale(s.ad_value(310), p.p289), A::offset(A::scale(s.ad_value(310), p.p290), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(79, p.p291);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(80, p.p292);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(81, p.p293);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(82, A::mul(A::offset(A::mul(A::div(A::scale(s.ad_value(334), p.p295), s.ad_value(333)), A::powf(s.ad_value(308), p.p296)), p.p294), A::offset(A::scale(s.ad_value(310), p.p297), 1.0)), A::offset(A::scale(s.ad_value(312), p.p298), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(308), p.p300), p.p299), A::scale(s.ad_value(310), p.p301)), A::scale(s.ad_value(312), p.p302));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(84, p.p303);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(85, p.p304);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(86, p.p305);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(87, p.p306, A::offset(A::scale(s.ad_value(308), p.p307), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(88, A::scale(A::powf(s.ad_value(308), p.p309), p.p308), A::offset(A::scale(s.ad_value(310), p.p310), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(335, 308, p.p312);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad(89, A::mul(A::scale(s.ad_value(335), p.p311), A::offset(A::scale(s.ad_value(310), p.p314), 1.0)), A::offset(A::mul(A::scale(s.ad_value(308), p.p313), s.ad_value(335)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(335, 308, p.p316);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad(90, A::mul(A::scale(s.ad_value(335), p.p315), A::offset(A::scale(s.ad_value(310), p.p318), 1.0)), A::offset(A::mul(A::scale(s.ad_value(308), p.p317), s.ad_value(335)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(91, p.p319);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(92, A::scale(A::offset(A::scale(s.ad_value(308), p.p321), 1.0), p.p320), A::offset(A::scale(s.ad_value(310), p.p322), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(93, p.p323);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(94, p.p324);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(95, A::scale(A::offset(A::scale(s.ad_value(308), p.p326), 1.0), p.p325), A::offset(A::scale(s.ad_value(310), p.p327), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(96, A::scale(A::offset(A::scale(s.ad_value(308), p.p329), 1.0), p.p328), A::offset(A::scale(s.ad_value(310), p.p330), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(97, p.p331);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(98, p.p332);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar(99, p.p333, 312);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(100, (p.p334 * p.p234), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(101, (p.p335 * p.p235), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(102, p.p336);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(103, p.p337);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(104, p.p338);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(105, p.p337);
        }

        s.v[1022] = if (if self.param_given[339] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1022] != 0.0)) {
            s.store_scalar(105, p.p339);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(106, p.p338);
        }

        s.v[1023] = if (if self.param_given[340] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1023] != 0.0)) {
            s.store_scalar(106, p.p340);
        }

        if (s.v[1019] != 0.0) {
            s.copy_ad(107, 105);
        }

        s.v[1024] = if (if self.param_given[341] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1024] != 0.0)) {
            s.store_scalar(107, p.p341);
        }

        if (s.v[1019] != 0.0) {
            s.copy_ad(108, 106);
        }

        s.v[1025] = if (if self.param_given[342] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1025] != 0.0)) {
            s.store_scalar(108, p.p342);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(109, p.p343);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(110, (p.p344 * p.p234), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(111, (p.p345 * p.p235), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(112, p.p346);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(113, p.p347);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(114, p.p348);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(115, p.p349);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(116, p.p350);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(117, p.p351);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale_ad(118, A::mul(A::scale(s.ad_value(315), (8.8541878176e-12 * p.p207)), s.ad_value(314)), 1.0 / (p.p206));
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(125, 315, ((8.8541878176e-12 * p.p207) * (p.p234 * 1.0 / (p.p232))));
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(126, 315, ((8.8541878176e-12 * p.p207) * (p.p235 * 1.0 / (p.p233))));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(119, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p354), p.p353), p.p352), A::scale(s.ad_value(310), p.p355)), A::scale(s.ad_value(312), p.p356));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(308), p.p358), p.p357), A::scale(s.ad_value(310), p.p359)), A::scale(s.ad_value(312), p.p360));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(32, p.p294);
        }

        s.v[1026] = if (if self.param_given[361] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1026] != 0.0)) {
            s.store_scalar(32, p.p361);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(33, p.p295);
        }

        s.v[1027] = if (if self.param_given[362] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1027] != 0.0)) {
            s.store_scalar(33, p.p362);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(34, p.p296);
        }

        s.v[1028] = if (if self.param_given[363] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1028] != 0.0)) {
            s.store_scalar(34, p.p363);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(35, p.p297);
        }

        s.v[1029] = if (if self.param_given[364] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1029] != 0.0)) {
            s.store_scalar(35, p.p364);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(36, p.p298);
        }

        s.v[1030] = if (if self.param_given[365] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_scalar(36, p.p365);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(121, A::mul(A::add(s.ad_value(32), A::mul(A::div(A::mul(s.ad_value(33), s.ad_value(334)), s.ad_value(333)), A::pow(s.ad_value(308), s.ad_value(34)))), A::offset(A::mul(s.ad_value(35), s.ad_value(310)), 1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(312)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(37, p.p306);
        }

        s.v[1031] = if (if self.param_given[366] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_scalar(37, p.p366);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(38, p.p307);
        }

        s.v[1032] = if (if self.param_given[367] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1032] != 0.0)) {
            s.store_scalar(38, p.p367);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad_rhs(122, 37, A::offset(A::mul(s.ad_value(38), s.ad_value(308)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(123, A::scale(A::powf(s.ad_value(308), p.p369), p.p368), A::offset(A::scale(s.ad_value(310), p.p370), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(335, 308, p.p372);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad(124, A::mul(A::scale(s.ad_value(335), p.p371), A::offset(A::scale(s.ad_value(310), p.p374), 1.0)), A::offset(A::mul(A::scale(s.ad_value(308), p.p373), s.ad_value(335)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(127, p.p375);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(128, p.p376);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(129, p.p377);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(130, 319, p.p378);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(131, 316, p.p379);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(132, 316, p.p380);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(133, p.p381);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(134, p.p382);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(135, p.p383);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(136, p.p384);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(137, 320, p.p385);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(138, 320, p.p386);
        }

        if (s.v[1019] != 0.0) {
            s.store_sub_from_scalar_ad(1001, 1.0, A::div_from_scalar((2.0 * p.p393), s.ad_value(306)));
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
        if (s.v[1019] != 0.0) {
            s.store_ad(336, &{
                if (s.v[1001] > 0.001) {
                    s.ad_value(1001)
                } else {
                    A::constant(0.001)
                }
            });
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(337, 1.0, A::powf(s.ad_value(336), p.p394));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(139, p.p387);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_lhs(140, A::mul(A::mul(A::scale(s.ad_value(65), p.p388), s.ad_value(65)), s.ad_value(310)), 310);
        }

        if (s.v[1019] != 0.0) {
            s.store_scaled_mul(141, 337, 312, p.p389);
        }

        if (s.v[1019] != 0.0) {
            s.store_scaled_mul(142, 337, 312, p.p390);
        }

        if (s.v[1019] != 0.0) {
            s.store_scaled_mul(143, 337, 312, p.p391);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(144, p.p392);
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_scaled(338, 307, p.p396, (2.0 * p.p395));
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar(339, 1e-6, 338);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul(340, 308, 339);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(145, p.p397);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(308), p.p399), p.p398), A::scale(s.ad_value(310), p.p400)), A::scale(s.ad_value(312), p.p401));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(147, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p404), p.p403), p.p402), A::scale(s.ad_value(310), p.p405)), A::scale(s.ad_value(312), p.p406));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(148, A::mul(A::scale(A::offset(A::scale(A::powf(s.ad_value(308), p.p409), p.p408), 1.0), p.p407), A::offset(A::scale(s.ad_value(310), p.p410), 1.0)), A::offset(A::scale(s.ad_value(312), p.p411), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_ad(149, A::scale(A::powf(s.ad_value(308), p.p414), p.p413), p.p412);
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_ad(341, A::mul(A::div_from_scalar((p.p415 * p.p416), s.ad_value(306)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(306)), 1.0 / (p.p416))))), 1.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(341, &{
                if (s.v[341] > 1e-15) {
                    s.ad_value(341)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(150, A::div(A::scale(s.ad_value(338), p.p256), A::mul(s.ad_value(341), s.ad_value(306))), A::offset(A::scale(s.ad_value(310), p.p417), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(308), p.p419), p.p418), A::scale(s.ad_value(310), p.p420)), A::scale(s.ad_value(312), p.p421));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(152, A::scale(A::powf(s.ad_value(308), p.p423), p.p422), A::offset(A::scale(s.ad_value(310), p.p424), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(153, p.p425);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(154, p.p426);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(155, A::scale(A::powf(s.ad_value(308), p.p428), p.p427), A::offset(A::scale(s.ad_value(310), p.p429), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(156, p.p431);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(157, p.p430);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(158, p.p432);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(159, 340, p.p433);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(160, 340, p.p434);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(161, 340, p.p435);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(162, p.p436);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(342, A::add(A::offset(A::scale(s.ad_value(308), p.p808), p.p807), A::scale(s.ad_value(310), p.p809)), A::scale(s.ad_value(312), p.p810));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(343, A::add(A::offset(A::scale(s.ad_value(308), p.p812), p.p811), A::scale(s.ad_value(310), p.p813)), A::scale(s.ad_value(312), p.p814));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(163, A::add(A::div(A::scale(A::add(A::scale(s.ad_value(323), (0.3333333333333333 * 1.0 / (s.v[14]))), s.ad_value(324)), p.p440), A::scale(s.ad_value(322), s.v[14])), A::div_from_scalar((p.p438 + p.p439), A::mul(s.ad_value(323), s.ad_value(321)))), A::scale(s.ad_value(1), p.p437));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(164, (if (p.p442 > 0.0) { p.p442 } else { 0.0 }));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(165, (if (p.p443 > 0.0) { p.p443 } else { 0.0 }));
        }

        s.v[1033] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1033] != 0.0)) {
            s.copy_ad(165, 164);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_lhs(166, A::scale(s.ad_value(1), p.p12), 164);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_lhs(167, A::scale(s.ad_value(1), p.p13), 165);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(168, 1, p.p445);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(169, 1, p.p444);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(170, 1, p.p446);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(171, 1, p.p447);
        }

        s.v[1034] = if ((((if self.param_given[448] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[449] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[450] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[451] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1034] != 0.0)) {
            s.store_add_ad(40, A::add(A::offset(A::scale(s.ad_value(308), p.p449), p.p448), A::scale(s.ad_value(310), p.p450)), A::scale(s.ad_value(312), p.p451));
        }

        s.v[1035] = if ((((if self.param_given[452] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[453] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[454] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[455] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1035] != 0.0)) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(308), p.p453), p.p452), A::scale(s.ad_value(310), p.p454)), A::scale(s.ad_value(312), p.p455));
        }

        s.v[1036] = if ((((if self.param_given[456] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[457] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[458] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[459] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1036] != 0.0)) {
            s.store_add_ad(45, A::add(A::offset(A::scale(s.ad_value(308), p.p457), p.p456), A::scale(s.ad_value(310), p.p458)), A::scale(s.ad_value(312), p.p459));
        }

        s.v[1037] = if ((((if self.param_given[460] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[461] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[462] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[463] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1037] != 0.0)) {
            s.store_add_ad(46, A::add(A::offset(A::scale(s.ad_value(308), p.p461), p.p460), A::scale(s.ad_value(310), p.p462)), A::scale(s.ad_value(312), p.p463));
        }

        s.v[1038] = if ((((if self.param_given[464] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[465] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[466] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[467] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1038] != 0.0)) {
            s.store_add_ad(47, A::add(A::offset(A::scale(s.ad_value(308), p.p465), p.p464), A::scale(s.ad_value(310), p.p466)), A::scale(s.ad_value(312), p.p467));
        }

        s.v[1039] = if ((((if self.param_given[468] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[469] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[470] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[471] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1039] != 0.0)) {
            s.store_add_ad(49, A::add(A::offset(A::scale(s.ad_value(308), p.p469), p.p468), A::scale(s.ad_value(310), p.p470)), A::scale(s.ad_value(312), p.p471));
        }

        s.v[1040] = if ((((if self.param_given[472] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[473] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[474] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[475] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1040] != 0.0)) {
            s.store_add_ad(50, A::add(A::offset(A::scale(s.ad_value(308), p.p473), p.p472), A::scale(s.ad_value(310), p.p474)), A::scale(s.ad_value(312), p.p475));
        }

        s.v[1041] = if ((((if self.param_given[476] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[477] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[478] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[479] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1041] != 0.0)) {
            s.store_add_ad(57, A::add(A::offset(A::scale(s.ad_value(308), p.p477), p.p476), A::scale(s.ad_value(310), p.p478)), A::scale(s.ad_value(312), p.p479));
        }

        s.v[1042] = if ((((if self.param_given[480] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[481] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[482] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[483] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1042] != 0.0)) {
            s.store_add_ad(58, A::add(A::offset(A::scale(s.ad_value(308), p.p481), p.p480), A::scale(s.ad_value(310), p.p482)), A::scale(s.ad_value(312), p.p483));
        }

        s.v[1043] = if ((((if self.param_given[484] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[485] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[486] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[487] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1043] != 0.0)) {
            s.store_add_ad(51, A::add(A::offset(A::scale(s.ad_value(308), p.p485), p.p484), A::scale(s.ad_value(310), p.p486)), A::scale(s.ad_value(312), p.p487));
        }

        s.v[1044] = if ((((if self.param_given[492] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[493] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[494] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[495] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1044] != 0.0)) {
            s.store_add_ad(52, A::add(A::offset(A::scale(s.ad_value(308), p.p493), p.p492), A::scale(s.ad_value(310), p.p494)), A::scale(s.ad_value(312), p.p495));
        }

        s.v[1045] = if ((((if self.param_given[488] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[489] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[490] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[491] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1045] != 0.0)) {
            s.store_add_ad(53, A::add(A::offset(A::scale(s.ad_value(308), p.p489), p.p488), A::scale(s.ad_value(310), p.p490)), A::scale(s.ad_value(312), p.p491));
        }

        s.v[1046] = if ((((if self.param_given[496] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[497] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[498] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[499] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_add_ad(54, A::add(A::offset(A::scale(s.ad_value(308), p.p497), p.p496), A::scale(s.ad_value(310), p.p498)), A::scale(s.ad_value(312), p.p499));
        }

        s.v[1047] = if ((((if self.param_given[500] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[501] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[502] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[503] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1047] != 0.0)) {
            s.store_mul_ad_rhs(62, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p501), p.p500), A::scale(s.ad_value(310), p.p502)), A::scale(s.ad_value(312), p.p503)));
        }

        s.v[1048] = if ((((if self.param_given[508] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[509] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[510] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[511] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1048] != 0.0)) {
            s.store_add_ad(63, A::add(A::offset(A::scale(s.ad_value(308), p.p509), p.p508), A::scale(s.ad_value(310), p.p510)), A::scale(s.ad_value(312), p.p511));
        }

        s.v[1049] = if ((((if self.param_given[504] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[505] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[506] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[507] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1049] != 0.0)) {
            s.store_add_ad(64, A::add(A::offset(A::scale(s.ad_value(308), p.p505), p.p504), A::scale(s.ad_value(310), p.p506)), A::scale(s.ad_value(312), p.p507));
        }

        s.v[1050] = if ((((if self.param_given[512] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[513] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[514] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[515] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1050] != 0.0)) {
            s.store_mul_ad_rhs(59, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p513), p.p512), A::scale(s.ad_value(310), p.p514)), A::scale(s.ad_value(312), p.p515)));
        }

        s.v[1051] = if ((((if self.param_given[520] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[521] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[522] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[523] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_add_ad(60, A::add(A::offset(A::scale(s.ad_value(308), p.p521), p.p520), A::scale(s.ad_value(310), p.p522)), A::scale(s.ad_value(312), p.p523));
        }

        s.v[1052] = if ((((if self.param_given[516] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[517] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[518] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[519] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1052] != 0.0)) {
            s.store_add_ad(61, A::add(A::offset(A::scale(s.ad_value(308), p.p517), p.p516), A::scale(s.ad_value(310), p.p518)), A::scale(s.ad_value(312), p.p519));
        }

        s.v[1053] = if ((((if self.param_given[524] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[525] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[526] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[527] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1053] != 0.0)) {
            s.store_mul_ad(65, A::div(s.ad_value(307), s.ad_value(306)), A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p525), p.p524), A::scale(s.ad_value(310), p.p526)), A::scale(s.ad_value(312), p.p527)));
        }

        s.v[1054] = if ((((if self.param_given[528] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[529] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[530] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[531] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1054] != 0.0)) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(308), p.p529), p.p528), A::scale(s.ad_value(310), p.p530)), A::scale(s.ad_value(312), p.p531));
        }

        s.v[1055] = if ((((if self.param_given[532] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[533] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[534] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[535] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1055] != 0.0)) {
            s.store_add_ad(67, A::add(A::offset(A::scale(s.ad_value(308), p.p533), p.p532), A::scale(s.ad_value(310), p.p534)), A::scale(s.ad_value(312), p.p535));
        }

        s.v[1056] = if ((((if self.param_given[536] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[537] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[538] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[539] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1056] != 0.0)) {
            s.store_add_ad(69, A::add(A::offset(A::scale(s.ad_value(308), p.p537), p.p536), A::scale(s.ad_value(310), p.p538)), A::scale(s.ad_value(312), p.p539));
        }

        s.v[1057] = if ((((if self.param_given[540] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[541] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[542] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[543] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1057] != 0.0)) {
            s.store_add_ad(71, A::add(A::offset(A::scale(s.ad_value(308), p.p541), p.p540), A::scale(s.ad_value(310), p.p542)), A::scale(s.ad_value(312), p.p543));
        }

        s.v[1058] = if ((((if self.param_given[544] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[545] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[546] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[547] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1058] != 0.0)) {
            s.store_add_ad(73, A::add(A::offset(A::scale(s.ad_value(308), p.p545), p.p544), A::scale(s.ad_value(310), p.p546)), A::scale(s.ad_value(312), p.p547));
        }

        s.v[1059] = if ((((if self.param_given[548] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[549] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[550] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[551] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1059] != 0.0)) {
            s.store_add_ad(75, A::add(A::offset(A::scale(s.ad_value(308), p.p549), p.p548), A::scale(s.ad_value(310), p.p550)), A::scale(s.ad_value(312), p.p551));
        }

        s.v[1060] = if ((((if self.param_given[552] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[553] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[554] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[555] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1060] != 0.0)) {
            s.store_mul_ad_rhs(78, 310, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p553), p.p552), A::scale(s.ad_value(310), p.p554)), A::scale(s.ad_value(312), p.p555)));
        }

        s.v[1061] = if ((((if self.param_given[556] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[557] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[558] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[559] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1061] != 0.0)) {
            s.store_add_ad(79, A::add(A::offset(A::scale(s.ad_value(308), p.p557), p.p556), A::scale(s.ad_value(310), p.p558)), A::scale(s.ad_value(312), p.p559));
        }

        s.v[1062] = if ((((if self.param_given[560] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[561] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[562] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[563] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1062] != 0.0)) {
            s.store_add_ad(80, A::add(A::offset(A::scale(s.ad_value(308), p.p561), p.p560), A::scale(s.ad_value(310), p.p562)), A::scale(s.ad_value(312), p.p563));
        }

        s.v[1063] = if ((((if self.param_given[564] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[565] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[566] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[567] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1063] != 0.0)) {
            s.store_add_ad(81, A::add(A::offset(A::scale(s.ad_value(308), p.p565), p.p564), A::scale(s.ad_value(310), p.p566)), A::scale(s.ad_value(312), p.p567));
        }

        s.v[1064] = if ((((if self.param_given[568] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[569] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[570] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[571] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1064] != 0.0)) {
            s.store_mul_ad_rhs(82, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p569), p.p568), A::scale(s.ad_value(310), p.p570)), A::scale(s.ad_value(312), p.p571)));
        }

        s.v[1065] = if ((((if self.param_given[572] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[573] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[574] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[575] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1065] != 0.0)) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(308), p.p573), p.p572), A::scale(s.ad_value(310), p.p574)), A::scale(s.ad_value(312), p.p575));
        }

        s.v[1066] = if ((((if self.param_given[576] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[577] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[578] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[579] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1066] != 0.0)) {
            s.store_add_ad(84, A::add(A::offset(A::scale(s.ad_value(308), p.p577), p.p576), A::scale(s.ad_value(310), p.p578)), A::scale(s.ad_value(312), p.p579));
        }

        s.v[1067] = if ((((if self.param_given[580] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[581] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[582] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[583] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1067] != 0.0)) {
            s.store_add_ad(85, A::add(A::offset(A::scale(s.ad_value(308), p.p581), p.p580), A::scale(s.ad_value(310), p.p582)), A::scale(s.ad_value(312), p.p583));
        }

        s.v[1068] = if ((((if self.param_given[584] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[585] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[586] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[587] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1068] != 0.0)) {
            s.store_add_ad(87, A::add(A::offset(A::scale(s.ad_value(308), p.p585), p.p584), A::scale(s.ad_value(310), p.p586)), A::scale(s.ad_value(312), p.p587));
        }

        s.v[1069] = if ((((if self.param_given[588] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[589] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[590] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[591] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1069] != 0.0)) {
            s.store_mul_ad_rhs(88, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p589), p.p588), A::scale(s.ad_value(310), p.p590)), A::scale(s.ad_value(312), p.p591)));
        }

        s.v[1070] = if ((((if self.param_given[592] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[593] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[594] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[595] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1070] != 0.0)) {
            s.store_add_ad(89, A::add(A::offset(A::scale(s.ad_value(308), p.p593), p.p592), A::scale(s.ad_value(310), p.p594)), A::scale(s.ad_value(312), p.p595));
        }

        s.v[1071] = if ((((if self.param_given[596] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[597] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[598] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[599] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1071] != 0.0)) {
            s.store_add_ad(90, A::add(A::offset(A::scale(s.ad_value(308), p.p597), p.p596), A::scale(s.ad_value(310), p.p598)), A::scale(s.ad_value(312), p.p599));
        }

        s.v[1072] = if ((((if self.param_given[600] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[601] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[602] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[603] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1072] != 0.0)) {
            s.store_add_ad(92, A::add(A::offset(A::scale(s.ad_value(308), p.p601), p.p600), A::scale(s.ad_value(310), p.p602)), A::scale(s.ad_value(312), p.p603));
        }

        s.v[1073] = if ((((if self.param_given[604] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[605] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[606] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[607] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1073] != 0.0)) {
            s.store_add_ad(94, A::add(A::offset(A::scale(s.ad_value(308), p.p605), p.p604), A::scale(s.ad_value(310), p.p606)), A::scale(s.ad_value(312), p.p607));
        }

        s.v[1074] = if ((((if self.param_given[608] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[609] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[610] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[611] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1074] != 0.0)) {
            s.store_add_ad(95, A::add(A::offset(A::scale(s.ad_value(308), p.p609), p.p608), A::scale(s.ad_value(310), p.p610)), A::scale(s.ad_value(312), p.p611));
        }

        s.v[1075] = if ((((if self.param_given[612] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[613] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[614] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[615] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1075] != 0.0)) {
            s.store_add_ad(96, A::add(A::offset(A::scale(s.ad_value(308), p.p613), p.p612), A::scale(s.ad_value(310), p.p614)), A::scale(s.ad_value(312), p.p615));
        }

        s.v[1076] = if ((((if self.param_given[616] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[617] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[618] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[619] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1076] != 0.0)) {
            s.store_mul_ad_rhs(99, 313, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p617), p.p616), A::scale(s.ad_value(310), p.p618)), A::scale(s.ad_value(312), p.p619)));
        }

        s.v[1077] = if ((((if self.param_given[620] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[621] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[622] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[623] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1077] != 0.0)) {
            s.store_mul_ad_rhs(100, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p621), p.p620), A::scale(s.ad_value(310), p.p622)), A::scale(s.ad_value(312), p.p623)));
        }

        s.v[1078] = if ((((if self.param_given[624] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[625] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[626] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[627] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1078] != 0.0)) {
            s.store_mul_ad_rhs(101, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p625), p.p624), A::scale(s.ad_value(310), p.p626)), A::scale(s.ad_value(312), p.p627)));
        }

        s.v[1079] = if ((((if self.param_given[628] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[629] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[630] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[631] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1079] != 0.0)) {
            s.store_add_ad(102, A::add(A::offset(A::scale(s.ad_value(308), p.p629), p.p628), A::scale(s.ad_value(310), p.p630)), A::scale(s.ad_value(312), p.p631));
        }

        s.v[1080] = if ((((if self.param_given[632] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[633] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[634] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[635] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1080] != 0.0)) {
            s.store_mul_ad_rhs(110, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p633), p.p632), A::scale(s.ad_value(310), p.p634)), A::scale(s.ad_value(312), p.p635)));
        }

        s.v[1081] = if ((((if self.param_given[636] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[637] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[638] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[639] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1081] != 0.0)) {
            s.store_mul_ad_rhs(111, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p637), p.p636), A::scale(s.ad_value(310), p.p638)), A::scale(s.ad_value(312), p.p639)));
        }

        s.v[1082] = if ((((if self.param_given[640] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[641] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[642] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[643] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_add_ad(114, A::add(A::offset(A::scale(s.ad_value(308), p.p641), p.p640), A::scale(s.ad_value(310), p.p642)), A::scale(s.ad_value(312), p.p643));
        }

        s.v[1083] = if ((((if self.param_given[644] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[645] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[646] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[647] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1083] != 0.0)) {
            s.store_add_ad(115, A::add(A::offset(A::scale(s.ad_value(308), p.p645), p.p644), A::scale(s.ad_value(310), p.p646)), A::scale(s.ad_value(312), p.p647));
        }

        s.v[1084] = if ((((if self.param_given[648] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[649] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[650] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[651] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_mul_ad(118, A::scale(A::mul(s.ad_value(316), s.ad_value(314)), 1000000.0), A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p649), p.p648), A::scale(s.ad_value(310), p.p650)), A::scale(s.ad_value(312), p.p651)));
        }

        s.v[1085] = if ((((if self.param_given[652] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[653] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[654] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[655] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1085] != 0.0)) {
            s.store_add_ad(119, A::add(A::offset(A::scale(s.ad_value(308), p.p653), p.p652), A::scale(s.ad_value(310), p.p654)), A::scale(s.ad_value(312), p.p655));
        }

        s.v[1086] = if ((((if self.param_given[656] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[657] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[658] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[659] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1086] != 0.0)) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(308), p.p657), p.p656), A::scale(s.ad_value(310), p.p658)), A::scale(s.ad_value(312), p.p659));
        }

        s.v[1087] = if ((((((((if self.param_given[660] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[661] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[662] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[663] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[568] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[569] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[570] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[571] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(28, p.p568);
        }

        s.v[1088] = if (if self.param_given[660] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

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
        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1088] != 0.0)) {
            s.store_scalar(28, p.p660);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(29, p.p569);
        }

        s.v[1089] = if (if self.param_given[661] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1089] != 0.0)) {
            s.store_scalar(29, p.p661);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(30, p.p570);
        }

        s.v[1090] = if (if self.param_given[662] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_scalar(30, p.p662);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(31, p.p571);
        }

        s.v[1091] = if (if self.param_given[663] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1091] != 0.0)) {
            s.store_scalar(31, p.p663);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_mul_ad_rhs(121, 308, A::add(A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(308))), A::mul(s.ad_value(30), s.ad_value(310))), A::mul(s.ad_value(31), s.ad_value(312))));
        }

        s.v[1092] = if ((((((((if self.param_given[664] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[667] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[584] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[585] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[586] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[587] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(28, p.p584);
        }

        s.v[1093] = if (if self.param_given[664] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1093] != 0.0)) {
            s.store_scalar(28, p.p664);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(29, p.p585);
        }

        s.v[1094] = if (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1094] != 0.0)) {
            s.store_scalar(29, p.p665);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(30, p.p586);
        }

        s.v[1095] = if (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(30, p.p666);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(31, p.p587);
        }

        s.v[1096] = if (if self.param_given[667] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1096] != 0.0)) {
            s.store_scalar(31, p.p667);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_add_ad(122, A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(308))), A::mul(s.ad_value(30), s.ad_value(310))), A::mul(s.ad_value(31), s.ad_value(312)));
        }

        s.v[1097] = if ((((if self.param_given[668] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[669] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[670] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[671] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_mul_ad_rhs(123, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p669), p.p668), A::scale(s.ad_value(310), p.p670)), A::scale(s.ad_value(312), p.p671)));
        }

        s.v[1098] = if ((((if self.param_given[672] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[673] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[674] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[675] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_mul_ad_rhs(124, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p673), p.p672), A::scale(s.ad_value(310), p.p674)), A::scale(s.ad_value(312), p.p675)));
        }

        s.v[1099] = if ((((if self.param_given[676] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[677] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[678] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[679] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1099] != 0.0)) {
            s.store_mul_ad_rhs(125, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p677), p.p676), A::scale(s.ad_value(310), p.p678)), A::scale(s.ad_value(312), p.p679)));
        }

        s.v[1100] = if ((((if self.param_given[680] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[681] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[682] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[683] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_mul_ad_rhs(126, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p681), p.p680), A::scale(s.ad_value(310), p.p682)), A::scale(s.ad_value(312), p.p683)));
        }

        s.v[1101] = if ((((if self.param_given[684] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[685] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[686] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[687] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1101] != 0.0)) {
            s.store_mul_ad_rhs(130, 319, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p685), p.p684), A::scale(s.ad_value(310), p.p686)), A::scale(s.ad_value(312), p.p687)));
        }

        s.v[1102] = if ((((if self.param_given[688] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[689] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[690] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[691] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1102] != 0.0)) {
            s.store_mul_ad_rhs(131, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p689), p.p688), A::scale(s.ad_value(310), p.p690)), A::scale(s.ad_value(312), p.p691)));
        }

        s.v[1103] = if ((((if self.param_given[692] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[693] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[694] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[695] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_mul_ad_rhs(132, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p693), p.p692), A::scale(s.ad_value(310), p.p694)), A::scale(s.ad_value(312), p.p695)));
        }

        s.v[1104] = if ((((if self.param_given[696] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[697] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[698] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[699] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1104] != 0.0)) {
            s.store_mul_ad_rhs(137, 320, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p697), p.p696), A::scale(s.ad_value(310), p.p698)), A::scale(s.ad_value(312), p.p699)));
        }

        s.v[1105] = if ((((if self.param_given[700] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[701] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[702] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[703] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1105] != 0.0)) {
            s.store_mul_ad_rhs(138, 320, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p701), p.p700), A::scale(s.ad_value(310), p.p702)), A::scale(s.ad_value(312), p.p703)));
        }

        s.v[1106] = if ((((if self.param_given[704] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[705] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[706] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[707] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1106] != 0.0)) {
            s.store_mul_ad_rhs(140, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p705), p.p704), A::scale(s.ad_value(310), p.p706)), A::scale(s.ad_value(312), p.p707)));
        }

        s.v[1107] = if ((((if self.param_given[708] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[709] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[710] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[711] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1107] != 0.0)) {
            s.store_mul_ad_rhs(141, 312, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p709), p.p708), A::scale(s.ad_value(310), p.p710)), A::scale(s.ad_value(312), p.p711)));
        }

        s.v[1108] = if ((((if self.param_given[712] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[713] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[714] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[715] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_mul_ad_rhs(142, 312, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p713), p.p712), A::scale(s.ad_value(310), p.p714)), A::scale(s.ad_value(312), p.p715)));
        }

        s.v[1109] = if ((((if self.param_given[716] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[717] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[718] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[719] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1109] != 0.0)) {
            s.store_mul_ad_rhs(143, 312, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p717), p.p716), A::scale(s.ad_value(310), p.p718)), A::scale(s.ad_value(312), p.p719)));
        }

        s.v[1110] = if ((((if self.param_given[720] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[721] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[722] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[723] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1110] != 0.0)) {
            s.store_add_ad(145, A::add(A::offset(A::scale(s.ad_value(308), p.p721), p.p720), A::scale(s.ad_value(310), p.p722)), A::scale(s.ad_value(312), p.p723));
        }

        s.v[1111] = if ((((if self.param_given[724] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[725] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[726] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[727] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1111] != 0.0)) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(308), p.p725), p.p724), A::scale(s.ad_value(310), p.p726)), A::scale(s.ad_value(312), p.p727));
        }

        s.v[1112] = if ((((if self.param_given[728] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[729] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[730] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[731] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1112] != 0.0)) {
            s.store_add_ad(147, A::add(A::offset(A::scale(s.ad_value(308), p.p729), p.p728), A::scale(s.ad_value(310), p.p730)), A::scale(s.ad_value(312), p.p731));
        }

        s.v[1113] = if ((((if self.param_given[732] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[733] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[734] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[735] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1113] != 0.0)) {
            s.store_add_ad(148, A::add(A::offset(A::scale(s.ad_value(308), p.p733), p.p732), A::scale(s.ad_value(310), p.p734)), A::scale(s.ad_value(312), p.p735));
        }

        s.v[1114] = if ((((if self.param_given[736] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[737] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[738] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[739] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1114] != 0.0)) {
            s.store_add_ad(149, A::add(A::offset(A::scale(s.ad_value(308), p.p737), p.p736), A::scale(s.ad_value(310), p.p738)), A::scale(s.ad_value(312), p.p739));
        }

        s.v[1115] = if ((((if self.param_given[740] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[741] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[742] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[743] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1115] != 0.0)) {
            s.store_mul_ad(150, A::div(s.ad_value(338), s.ad_value(306)), A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p741), p.p740), A::scale(s.ad_value(310), p.p742)), A::scale(s.ad_value(312), p.p743)));
        }

        s.v[1116] = if ((((if self.param_given[744] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[745] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[746] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[747] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1116] != 0.0)) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(308), p.p745), p.p744), A::scale(s.ad_value(310), p.p746)), A::scale(s.ad_value(312), p.p747));
        }

        s.v[1117] = if ((((if self.param_given[748] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[749] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[750] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[751] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1117] != 0.0)) {
            s.store_mul_ad_rhs(152, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p749), p.p748), A::scale(s.ad_value(310), p.p750)), A::scale(s.ad_value(312), p.p751)));
        }

        s.v[1118] = if ((((if self.param_given[752] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[753] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[754] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[755] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1118] != 0.0)) {
            s.store_add_ad(153, A::add(A::offset(A::scale(s.ad_value(308), p.p753), p.p752), A::scale(s.ad_value(310), p.p754)), A::scale(s.ad_value(312), p.p755));
        }

        s.v[1119] = if ((((if self.param_given[756] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[757] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[758] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[759] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1119] != 0.0)) {
            s.store_add_ad(154, A::add(A::offset(A::scale(s.ad_value(308), p.p757), p.p756), A::scale(s.ad_value(310), p.p758)), A::scale(s.ad_value(312), p.p759));
        }

        s.v[1120] = if ((((if self.param_given[760] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[761] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[762] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[763] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1120] != 0.0)) {
            s.store_mul_ad_rhs(155, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p761), p.p760), A::scale(s.ad_value(310), p.p762)), A::scale(s.ad_value(312), p.p763)));
        }

        s.v[1121] = if ((((if self.param_given[768] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[769] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[770] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[771] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1121] != 0.0)) {
            s.store_add_ad(156, A::add(A::offset(A::scale(s.ad_value(308), p.p769), p.p768), A::scale(s.ad_value(310), p.p770)), A::scale(s.ad_value(312), p.p771));
        }

        s.v[1122] = if ((((if self.param_given[764] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[765] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[766] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[767] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1122] != 0.0)) {
            s.store_add_ad(157, A::add(A::offset(A::scale(s.ad_value(308), p.p765), p.p764), A::scale(s.ad_value(310), p.p766)), A::scale(s.ad_value(312), p.p767));
        }

        s.v[1123] = if ((((if self.param_given[772] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[773] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[774] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[775] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1123] != 0.0)) {
            s.store_mul_ad_rhs(159, 340, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p773), p.p772), A::scale(s.ad_value(310), p.p774)), A::scale(s.ad_value(312), p.p775)));
        }

        s.v[1124] = if ((((if self.param_given[776] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[777] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[778] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[779] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1124] != 0.0)) {
            s.store_mul_ad_rhs(160, 340, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p777), p.p776), A::scale(s.ad_value(310), p.p778)), A::scale(s.ad_value(312), p.p779)));
        }

        s.v[1125] = if ((((if self.param_given[780] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[781] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[782] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[783] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1125] != 0.0)) {
            s.store_mul_ad_rhs(161, 340, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p781), p.p780), A::scale(s.ad_value(310), p.p782)), A::scale(s.ad_value(312), p.p783)));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(1008, 0.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(1009, 0.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(1007, 0.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(39, p.p788);
        }

        s.v[1126] = if (if self.param_given[789] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1126] != 0.0)) {
            s.store_scalar(39, p.p789);
        }

        s.v[1127] = if (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0)))) { 1.0 } else { 0.0 };

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (s.v[1] - 0.5);
            let assign9160_cond_e8971: f64 = if (((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) && (s.v[1007] < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1008, 1008, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1007), (s.v[7] + s.v[3])), (s.v[5] + (0.5 * s.v[3])))));
            }
            if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1009, 1009, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1007), (s.v[7] + s.v[3])), (s.v[6] + (0.5 * s.v[3])))));
            }
            if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_offset(1007, 1007, 1.0);
            }
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(992, 1008, 2);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(993, 1009, 2);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(994, (1.0 / (p.p784 + (0.5 * s.v[3]))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(995, (1.0 / (p.p785 + (0.5 * s.v[3]))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1005, &{
                if ((s.v[3] + s.v[304]) > 1e-9) {
                    A::offset(s.ad_value(304), s.v[3])
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1006, &{
                if (((s.v[4] + s.v[305]) + p.p786) > 1e-9) {
                    A::offset(A::add(s.ad_value(4), s.ad_value(305)), p.p786)
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1003, 1.0, A::powf(s.ad_value(1005), p.p794));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1004, 1.0, A::powf(s.ad_value(1006), p.p795));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scale_ad(996, A::add(A::add(A::offset(A::scale(s.ad_value(1003), p.p791), 1.0), A::scale(s.ad_value(1004), p.p792)), A::mul(A::scale(s.ad_value(1003), p.p793), s.ad_value(1004))), (1.0 + (p.p790 * (s.v[346] - 1.0))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(997, A::scale(A::add(s.ad_value(992), s.ad_value(993)), p.p787), 996);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(998, A::scale(A::add(s.ad_value(994), s.ad_value(995)), p.p787), 996);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1003, 1.0, A::powf(s.ad_value(1005), p.p800));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1004, 1.0, A::powf(s.ad_value(1006), p.p801));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add_ad(999, A::add(A::offset(A::scale(s.ad_value(1003), p.p797), 1.0), A::scale(s.ad_value(1004), p.p798)), A::mul(A::scale(s.ad_value(1003), p.p799), s.ad_value(1004)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_sub_ad_lhs(1001, A::sub(A::add(s.ad_value(992), s.ad_value(993)), s.ad_value(994)), 995);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(1002, A::offset(s.ad_value(997), 1.0), A::offset(s.ad_value(998), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(65, 65, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(82, A::mul(A::mul(s.ad_value(82), s.ad_value(1002)), A::offset(A::scale(s.ad_value(998), p.p788), 1.0)), A::offset(A::scale(s.ad_value(997), p.p788), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(121, A::mul(A::mul(s.ad_value(121), s.ad_value(1002)), A::offset(A::mul(s.ad_value(39), s.ad_value(998)), 1.0)), A::offset(A::mul(s.ad_value(39), s.ad_value(997)), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(150, 150, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(1002, A::scale(s.ad_value(1001), p.p796), 999);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(40, 40, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(145, 145, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(1002, A::scale(s.ad_value(1001), p.p802), A::powf(s.ad_value(999), p.p803));
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(62, 62, 1002);
        }

        if ((s.v[1019] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(155, 155, 1002);
        }

        s.v[1128] = if ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1129] = if (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_offset(1001, 4, s.v[8]);
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_scalar(1002, (1.0 / p.p804));
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_from_scalar_ad(11, (p.p804 * p.p804), A::scale(s.ad_value(1001), s.v[8]));
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(12, A::sub(A::scale(A::exp(A::scale(s.ad_value(1002), ((-10.0) * s.v[8]))), ((0.1 * s.v[8]) + (0.01 * p.p804))), A::mul(A::offset(A::scale(s.ad_value(1001), 0.1), (0.01 * p.p804)), A::exp(A::mul(A::scale(s.ad_value(1001), (-10.0)), s.ad_value(1002))))), 4);
        }

        if (((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(13, A::sub(A::scale(A::exp(A::scale(s.ad_value(1002), ((-20.0) * s.v[8]))), ((0.05 * s.v[8]) + (0.0025 * p.p804))), A::mul(A::offset(A::scale(s.ad_value(1001), 0.05), (0.0025 * p.p804)), A::exp(A::mul(A::scale(s.ad_value(1001), (-20.0)), s.ad_value(1002))))), 4);
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad(1001, A::add(s.ad_value(11), A::scale(s.ad_value(12), p.p805)), A::scale(s.ad_value(13), p.p806));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(40, 40, A::mul(s.ad_value(342), s.ad_value(1001)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(65, 65, A::offset(A::mul(s.ad_value(343), s.ad_value(1001)), 1.0));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(145, 145, A::mul(s.ad_value(342), s.ad_value(1001)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(150, 150, A::offset(A::mul(s.ad_value(343), s.ad_value(1001)), 1.0));
        }

        s.copy_ad(172, 40);

        s.copy_ad(173, 41);

        s.copy_ad(174, 42);

        s.copy_ad(176, 43);

        s.copy_ad(177, 44);

        if (s.v[45] > 1e20) {
            s.store_ad(178, &{
                if (s.v[45] < 1e26) {
                    s.ad_value(45)
                } else {
                    A::constant(1e26)
                }
            });
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
        if (s.v[57] > 1e23) {
            s.store_ad(189, &{
                if (s.v[57] < 1e27) {
                    s.ad_value(57)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(189, 1e23);
        }

        if (s.v[58] > 1e23) {
            s.store_ad(190, &{
                if (s.v[58] < 1e27) {
                    s.ad_value(58)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(190, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(184, 51);
        } else {
            s.store_scalar(184, 0.0);
        }

        if (s.v[53] > 0.0) {
            s.store_ad(186, &{
                if (s.v[53] < 0.5) {
                    s.ad_value(53)
                } else {
                    A::constant(0.5)
                }
            });
        } else {
            s.store_scalar(186, 0.0);
        }

        if (s.v[52] > 0.0) {
            s.store_ad(185, &{
                if (s.v[52] < 1.0) {
                    s.ad_value(52)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(193, &{
                if (s.v[64] < 1.0) {
                    s.ad_value(64)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(195, &{
                if (s.v[61] < 1.0) {
                    s.ad_value(61)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(212, &{
                if (s.v[80] < 1.0) {
                    s.ad_value(80)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(216, &{
                if (s.v[84] < 1.0) {
                    s.ad_value(84)
                } else {
                    A::constant(1.0)
                }
            });
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

        if (s.v[141] > 0.0) {
            s.copy_ad(273, 141);
        } else {
            s.store_scalar(273, 0.0);
        }

        if (s.v[142] > 0.0) {
            s.copy_ad(274, 142);
        } else {
            s.store_scalar(274, 0.0);
        }

        if (s.v[143] > 0.0) {
            s.copy_ad(275, 143);
        } else {
            s.store_scalar(275, 0.0);
        }

        s.copy_ad(276, 144);

        s.copy_ad(277, 145);

        s.copy_ad(278, 146);

        s.copy_ad(279, 147);

        if (s.v[148] > 1e20) {
            s.store_ad(280, &{
                if (s.v[148] < 1e26) {
                    s.ad_value(148)
                } else {
                    A::constant(1e26)
                }
            });
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
            s.store_ad(285, &{
                if (s.v[153] < 1.0) {
                    s.ad_value(153)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(285, 0.0);
        }

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
        if (s.v[157] > 0.0) {
            s.store_ad(289, &{
                if (s.v[157] < 1.0) {
                    s.ad_value(157)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(288, 156);
        } else {
            s.store_scalar(288, 0.0);
        }

        s.copy_ad(290, 158);

        if (s.v[159] > 0.0) {
            s.copy_ad(291, 159);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[160] > 0.0) {
            s.copy_ad(292, 160);
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[161] > 0.0) {
            s.copy_ad(293, 161);
        } else {
            s.store_scalar(293, 0.0);
        }

        s.copy_ad(294, 162);

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

        s.v[1130] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1130] != 0.0) {
            s.copy_ad(188, 187);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(190, 189);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(243, 242);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(245, 244);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(247, 246);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(249, 248);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(233, 232);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(239, 237);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(240, 238);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(258, 257);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(260, 259);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(264, 263);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(270, 269);
        }

        s.store_scale(762, 177, 8.8541878176e-12);

        s.store_div(763, 762, 176);

        s.store_square(764, 176);

        s.store_scale(765, 763, 6.241449993689894e18);

        s.store_mul(766, 252, 178);

        if (s.v[766] > 1e20) {
            s.store_ad(766, &{
                if (s.v[766] < 1e26) {
                    s.ad_value(766)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(766, 1e20);
        }

        s.v[767] = 0.0;

        s.v[1131] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1131] != 0.0) {
            s.store_scale_ad(767, A::powf(s.ad_value(763), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.v[1132] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1132] != 0.0)) {
            s.store_scale(767, 767, (7.448711 / 5.951993));
        }

        s.store_scale(768, 763, (1e-8 * 1.0 / (s.v[761])));

        s.store_scale(769, 209, 0.5);

        s.v[770] = 0.5;

        s.v[1133] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[1133] != 0.0) {
            s.store_scale(769, 209, 0.3333333333333333);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(770, 0.3333333333333333);
        }

        s.store_offset_ad(1000, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(219)), 1.0)), (-1.0));

        s.store_ad(771, &A::div(A::mul(A::offset(s.ad_value(1000), (-1.0)), A::offset(s.ad_value(1000), (-1.0))), {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_offset_ad(1000, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(254)), 1.0)), (-1.0));

        s.store_ad(772, &A::div(A::mul(A::offset(s.ad_value(1000), (-1.0)), A::offset(s.ad_value(1000), (-1.0))), {
            if ((4.0 * s.v[1000]) > 0.0001) {
                A::scale(s.ad_value(1000), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_div_from_scalar(773, 1.0, 223);

        s.store_div(774, 762, 187);

        s.store_div(775, 762, 188);

        s.store_div_ad_lhs(776, A::sqrt(A::scale(s.ad_value(189), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[349])))), 774);

        s.store_div_ad_lhs(777, A::sqrt(A::scale(s.ad_value(190), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[349])))), 775);

        s.store_square(778, 776);

        s.store_square(779, 777);

        s.store_offset_ad(780, A::div(A::ln(A::offset(A::exp(A::scale(s.ad_value(261), (0.005 * s.v[349]))), (-1.0))), s.ad_value(261)), (-((((((0.005 * s.v[349])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(781, A::ln(A::scale(s.ad_value(776), 0.5)), 780);

        s.store_add_ad_lhs(782, A::ln(A::scale(s.ad_value(777), 0.5)), 780);

        s.store_div_from_scalar(814, 1.0, 776);

        s.store_offset_scaled(815, 776, 3.1, 8.5);

        s.store_square(783, 815);

        s.store_scale(816, 815, 0.5);

        s.v[1134] = if (s.v[814] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1134] != 0.0) {
            s.store_scale(784, 814, 64.0);
        }

        s.v[1135] = if (s.v[814] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1134] != 0.0)) && (s.v[1135] != 0.0)) {
            s.store_offset_scaled(784, 814, 22.0, 3.0);
        }

        s.v[1136] = if (s.v[814] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (s.v[1136] != 0.0)) {
            s.store_offset_scaled(784, 814, (-7.2), 15.5);
        }

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (!(s.v[1136] != 0.0))) {
            s.copy_ad(784, 776);
        }

        s.store_sub_ad(785, A::add(s.ad_value(816), A::scale(s.ad_value(778), 0.5)), A::mul(s.ad_value(776), A::sqrt(A::add(A::add(s.ad_value(816), A::scale(s.ad_value(778), 0.25)), s.ad_value(784)))));

        s.store_div_from_scalar(814, 1.0, 777);

        s.store_offset_scaled(815, 777, 3.1, 8.5);

        s.store_square(786, 815);

        s.store_scale(816, 815, 0.5);

        s.v[1137] = if (s.v[814] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1137] != 0.0) {
            s.store_scale(787, 814, 64.0);
        }

        s.v[1138] = if (s.v[814] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1137] != 0.0)) && (s.v[1138] != 0.0)) {
            s.store_offset_scaled(787, 814, 22.0, 3.0);
        }

        s.v[1139] = if (s.v[814] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (s.v[1139] != 0.0)) {
            s.store_offset_scaled(787, 814, (-7.2), 15.5);
        }

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (!(s.v[1139] != 0.0))) {
            s.copy_ad(787, 777);
        }

        s.store_sub_ad(788, A::add(s.ad_value(816), A::scale(s.ad_value(779), 0.5)), A::mul(s.ad_value(777), A::sqrt(A::add(A::add(s.ad_value(816), A::scale(s.ad_value(779), 0.25)), s.ad_value(787)))));

        s.store_add_ad(722, A::offset(s.ad_value(182), s.v[356]), A::scale(A::ln(A::scale(A::mul(s.ad_value(178), A::powf(s.ad_value(357), (-0.75))), 4e-26)), (2.0 * s.v[709])));

        if !(s.v[722] > 0.05) {
            s.store_scalar(722, 0.05);
        }

        s.store_div_ad_lhs(723, A::sqrt(A::scale(s.ad_value(178), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[355])))), 763);

        s.v[724] = 0.0;

        s.v[725] = 0.0;

        s.v[1140] = if (s.v[183] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1140] != 0.0) {
            s.store_div_from_scalar(726, 80000000.0, 764);
        }

        if (s.v[1140] != 0.0) {
            s.store_ad(725, &{
                if (s.v[183] > s.v[726]) {
                    s.ad_value(183)
                } else {
                    s.ad_value(726)
                }
            });
        }

        if (s.v[1140] != 0.0) {
            s.store_ad(725, &{
                if (5e24 > s.v[725]) {
                    A::constant(5e24)
                } else {
                    s.ad_value(725)
                }
            });
        }

        if (s.v[1140] != 0.0) {
            s.store_div_ad(724, A::scale(A::mul(A::scale(s.ad_value(763), 2.0), s.ad_value(763)), s.v[709]), A::scale(s.ad_value(725), (1.6021918e-19 * s.v[761])));
        }

        s.v[727] = ((100.0 * s.v[709]) * s.v[709]);

        s.v[1141] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1141] != 0.0) {
            s.store_sqrt_ad(728, A::mul(A::mul(A::scale(s.ad_value(723), s.v[709]), s.ad_value(723)), s.ad_value(722)));
        }

        if (s.v[1141] != 0.0) {
            s.store_mul_ad(729, A::scale(s.ad_value(767), 0.75), A::powf(s.ad_value(728), 0.6666666666666666));
        }

        if (s.v[1141] != 0.0) {
            s.store_add(722, 722, 729);
        }

        if (s.v[1141] != 0.0) {
            s.store_mul_ad_rhs(723, 723, A::offset(A::div(A::scale(s.ad_value(729), (2.0 * 0.6666666666666666)), s.ad_value(728)), 1.0));
        }

        s.store_sqrt(730, 722);

        s.store_scale(731, 722, 0.95);

        s.store_mul_ad_lhs(732, A::scale(s.ad_value(722), 0.0025), 722);

        s.copy_ad(733, 732);

        s.store_scaled_sqrt(734, 733, 0.5);

        s.store_scale_ad(735, A::sub(A::sub(s.ad_value(731), s.ad_value(734)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(731), s.ad_value(734)), A::sub(s.ad_value(731), s.ad_value(734))), s.ad_value(732)))), 0.5);

        s.store_scaled_offset(736, 722, s.v[356], 0.5);

        s.store_sub_ad_lhs(737, A::sqrt(A::add(s.ad_value(180), s.ad_value(722))), 730);

        s.store_sub_ad_lhs(738, A::sub(A::sqrt(A::add(A::add(s.ad_value(180), s.ad_value(181)), s.ad_value(722))), s.ad_value(730)), 737);

        s.store_add_ad(739, A::add(A::offset(s.ad_value(182), s.v[356]), s.ad_value(251)), A::scale(A::ln(A::scale(A::mul(s.ad_value(766), A::powf(s.ad_value(357), (-0.75))), 4e-26)), (2.0 * s.v[709])));

        if !(s.v[739] > 0.05) {
            s.store_scalar(739, 0.05);
        }

        s.store_div_ad_lhs(740, A::sqrt(A::scale(s.ad_value(766), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[355])))), 763);

        s.v[1142] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1142] != 0.0) {
            s.store_sqrt_ad(728, A::mul(A::mul(A::scale(s.ad_value(740), s.v[709]), s.ad_value(740)), s.ad_value(739)));
        }

        if (s.v[1142] != 0.0) {
            s.store_mul_ad(729, A::scale(s.ad_value(767), 0.75), A::powf(s.ad_value(728), 0.6666666666666666));
        }

        if (s.v[1142] != 0.0) {
            s.store_add(739, 739, 729);
        }

        if (s.v[1142] != 0.0) {
            s.store_mul_ad_rhs(740, 740, A::offset(A::div(A::scale(s.ad_value(729), (2.0 * 0.6666666666666666)), s.ad_value(728)), 1.0));
        }

        s.store_scale(741, 739, 0.95);

        s.store_mul_ad_lhs(742, A::scale(s.ad_value(739), 0.0025), 739);

        s.copy_ad(743, 742);

        s.store_scaled_sqrt(734, 743, 0.5);

        s.store_scale_ad(744, A::sub(A::sub(s.ad_value(741), s.ad_value(734)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(741), s.ad_value(734)), A::sub(s.ad_value(741), s.ad_value(734))), s.ad_value(742)))), 0.5);

        s.store_offset_ad(694, A::add(s.ad_value(172), A::mul(A::scale(s.ad_value(173), s.v[352]), A::offset(A::scale(s.ad_value(174), s.v[352]), 1.0))), s.v[17]);

        s.store_exp_ad(745, A::scale(s.ad_value(175), s.v[354]));

        s.store_mul(695, 184, 745);

        s.store_scale(696, 185, 1.0 / (s.v[353]));

        s.store_exp_ad(746, A::scale(s.ad_value(198), s.v[354]));

        s.store_mul(697, 197, 746);

        s.store_mul_ad_lhs(710, A::scale(s.ad_value(697), s.v[16]), 763);

        s.store_mul_ad_rhs(699, 201, A::exp(A::scale(s.ad_value(202), s.v[354])));

        s.store_exp_ad(747, A::scale(s.ad_value(200), s.v[354]));

        s.store_mul(698, 199, 747);

        s.store_mul_ad_rhs(701, 205, A::exp(A::scale(s.ad_value(206), s.v[354])));

        s.store_exp_ad(748, A::scale(s.ad_value(204), s.v[354]));

        s.store_mul(700, 203, 748);

        s.store_exp_ad(749, A::scale(s.ad_value(208), s.v[354]));

        s.store_mul(702, 207, 749);

        s.store_exp_ad(750, A::scale(s.ad_value(211), s.v[354]));

        s.store_mul(703, 210, 750);

        s.store_mul_ad_lhs(751, A::scale(s.ad_value(710), 2.0), 703);

        s.store_exp_ad(752, A::scale(s.ad_value(215), s.v[354]));

        s.store_mul(714, 214, 752);

        s.store_mul(715, 253, 752);

        s.store_mul_ad_rhs(706, 225, A::exp(A::scale(A::neg(s.ad_value(226)), s.v[354])));

        s.store_scale(713, 271, (4.0 * (1.3806505e-23 * s.v[350])));

        s.store_div_ad_lhs(716, A::scale(s.ad_value(710), (s.v[709] * s.v[709])), 765);

        s.v[1143] = if ((p.p46 != 0.0) && (s.v[282] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1143] != 0.0) {
            s.store_offset_ad(707, A::add(s.ad_value(277), A::scale(s.ad_value(278), s.v[352])), s.v[19]);
        }

        if (s.v[1143] != 0.0) {
            s.store_exp_ad(753, A::scale(s.ad_value(283), s.v[354]));
        }

        if (s.v[1143] != 0.0) {
            s.store_mul(708, 282, 753);
        }

        if (s.v[1143] != 0.0) {
            s.store_mul_ad_lhs(711, A::scale(s.ad_value(708), s.v[18]), 763);
        }

        if (s.v[1143] != 0.0) {
            s.store_scale_ad(717, A::offset(A::scale(s.ad_value(281), s.v[353]), 1.0), s.v[709]);
        }

        if (s.v[1143] != 0.0) {
            s.store_add_ad(754, A::offset(s.ad_value(279), s.v[356]), A::mul(A::scale(s.ad_value(717), 2.0), A::ln(A::scale(A::mul(s.ad_value(280), A::powf(s.ad_value(357), (-0.75))), 4e-26))));
        }

        if (s.v[1143] != 0.0) {
            s.store_ad(754, &{
                if (s.v[754] > 0.05) {
                    s.ad_value(754)
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[1143] != 0.0) {
            s.store_div_ad_lhs(755, A::sqrt(A::scale(s.ad_value(280), ((2.0 * 1.6021918e-19) * (s.v[761] * s.v[355])))), 763);
        }

        if (s.v[1143] != 0.0) {
            s.store_square(718, 755);
        }

        if (s.v[1143] != 0.0) {
            s.store_ln(719, 718);
        }

        if (s.v[1143] != 0.0) {
            s.store_scale(756, 754, 0.95);
        }

        if (s.v[1143] != 0.0) {
            s.store_mul_ad_lhs(757, A::scale(s.ad_value(754), 0.0025), 754);
        }

        if (s.v[1143] != 0.0) {
            s.copy_ad(758, 757);
        }

        if (s.v[1143] != 0.0) {
            s.store_scaled_sqrt(759, 758, 0.5);
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
        if (s.v[1143] != 0.0) {
            s.store_scale_ad(760, A::sub(A::sub(s.ad_value(756), s.ad_value(759)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(756), s.ad_value(759)), A::sub(s.ad_value(756), s.ad_value(759))), s.ad_value(757)))), 0.5);
        }

        if (s.v[1143] != 0.0) {
            s.store_div_ad_lhs(720, A::scale(s.ad_value(711), (s.v[709] * s.v[709])), 765);
        }

        if (s.v[1143] != 0.0) {
            s.store_scale(721, 290, (4.0 * (1.3806505e-23 * s.v[350])));
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(707, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(753, 1.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(708, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(711, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(717, s.v[709]);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(754, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(755, 1.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(718, 1.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(756, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(757, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(758, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(759, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(760, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if (!(s.v[1143] != 0.0)) {
            s.store_scalar(721, 1.0);
        }

        s.store_div_from_scalar(789, 1.0, 241);

        s.store_scale_ad(790, A::sqrt(A::scale(s.ad_value(241), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(791, 790, 176);

        s.store_mul(792, 790, 187);

        s.store_mul(793, 790, 188);

        s.v[794] = 0.0;

        s.v[1144] = if (s.v[236] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1144] != 0.0) {
            s.store_div_ad_lhs(794, A::scale(s.ad_value(235), (-0.495)), 236);
        }

        s.v[795] = 0.0;

        s.v[1145] = if (s.v[238] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1145] != 0.0) {
            s.store_div_ad_lhs(795, A::scale(s.ad_value(237), (-0.495)), 238);
        }

        s.v[1146] = if (s.v[240] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1146] != 0.0) {
            s.store_div_ad_lhs(796, A::scale(s.ad_value(239), (-0.495)), 240);
        }

        s.store_ad(797, &A::pow_from_scalar(s.v[346], s.ad_value(234)));

        s.store_mul(231, 231, 797);

        s.store_mul(232, 232, 797);

        s.store_mul(233, 233, 797);

        s.store_div_ad(798, A::scale(s.ad_value(242), 4e-18), A::square(s.ad_value(187)));

        s.store_div_ad(799, A::scale(s.ad_value(243), 4e-18), A::square(s.ad_value(188)));

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

        s.v[1147] = if (s.v[267] > 1e-10) { 1.0 } else { 0.0 };

        if (s.v[1147] != 0.0) {
            s.store_div_from_scalar(802, 0.75, 267);
        }

        s.store_square(803, 268);

        s.store_scale(804, 272, (9.1093826e-31 * 1000000000.0));

        s.v[1148] = if (s.v[295] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1148] != 0.0) {
            s.store_div_from_scalar(805, 1.0, 295);
        }

        if (!(s.v[1148] != 0.0)) {
            s.store_scalar(805, 0.0);
        }

        s.v[1149] = if (s.v[296] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1149] != 0.0) {
            s.store_div_from_scalar(806, 1.0, 296);
        }

        if (!(s.v[1149] != 0.0)) {
            s.store_scalar(806, 0.0);
        }

        s.v[1150] = if (s.v[297] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1150] != 0.0) {
            s.store_div_from_scalar(807, 1.0, 297);
        }

        if (!(s.v[1150] != 0.0)) {
            s.store_scalar(807, 0.0);
        }

        s.v[1151] = if (s.v[298] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1151] != 0.0) {
            s.store_div_from_scalar(808, 1.0, 298);
        }

        if (!(s.v[1151] != 0.0)) {
            s.store_scalar(808, 0.0);
        }

        s.v[1152] = if (s.v[299] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1152] != 0.0) {
            s.store_div_from_scalar(809, 1.0, 299);
        }

        if (!(s.v[1152] != 0.0)) {
            s.store_scalar(809, 0.0);
        }

        s.v[1153] = if (s.v[300] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1153] != 0.0) {
            s.store_div_from_scalar(810, 1.0, 300);
        }

        if (!(s.v[1153] != 0.0)) {
            s.store_scalar(810, 0.0);
        }

        s.v[1154] = if (s.v[301] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1154] != 0.0) {
            s.store_div_from_scalar(811, 1.0, 301);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(811, 0.0);
        }

        s.store_scale(20, 2, s.v[640]);

        s.store_scale(21, 2, s.v[641]);

        s.store_scale(22, 2, s.v[642]);

        s.store_scale(23, 2, s.v[667]);

        s.store_scale(24, 2, s.v[668]);

        s.store_scale(25, 2, s.v[669]);

        s.v[26] = 0.0;

        s.v[1155] = if (p.p43 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1155] != 0.0) {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 307);

        s.v[1156] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1156] != 0.0) {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.v[1157] = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1157] != 0.0) {
            s.store_scale(20, 2, s.v[643]);
        }

        if (s.v[1157] != 0.0) {
            s.store_sub_ad(21, A::scale(s.ad_value(2), s.v[644]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1157] != 0.0) {
            s.copy_ad(22, 27);
        }

        if (s.v[1157] != 0.0) {
            s.store_scale(23, 2, s.v[670]);
        }

        if (s.v[1157] != 0.0) {
            s.store_sub_ad(24, A::scale(s.ad_value(2), s.v[671]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1157] != 0.0) {
            s.copy_ad(25, 27);
        }

        s.v[1158] = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1158] != 0.0) {
            s.store_ad(640, &{
                if (s.v[20] > 0.0) {
                    s.ad_value(20)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(641, &{
                if (s.v[21] > 0.0) {
                    s.ad_value(21)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(642, &{
                if (s.v[22] > 0.0) {
                    s.ad_value(22)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(667, &{
                if (s.v[23] > 0.0) {
                    s.ad_value(23)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(668, &{
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1158] != 0.0) {
            s.store_ad(669, &{
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(640, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(641, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(642, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(667, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
            s.store_scalar(668, 0.0);
        }

        if (!(s.v[1158] != 0.0)) {
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

        s.v[1159] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[1160] = if ((s.v[381] * s.v[640]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1160] != 0.0)) {
            s.store_scale_ad(448, A::ln(A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(640), s.v[381])), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1160] != 0.0))) {
            s.store_scalar(448, 100000000.0);
        }

        s.v[1161] = if ((s.v[382] * s.v[641]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scale_ad(449, A::ln(A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(641), s.v[382])), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1161] != 0.0))) {
            s.store_scalar(449, 100000000.0);
        }

        s.v[1162] = if ((s.v[383] * s.v[642]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scale_ad(450, A::ln(A::offset(A::div_from_scalar(p.p815, A::scale(s.ad_value(642), s.v[383])), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1162] != 0.0))) {
            s.store_scalar(450, 100000000.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(648, &A::min(A::min(s.ad_value(448), s.ad_value(449)), s.ad_value(450)));
        }

        s.v[1163] = if ((((s.v[648] * s.v[365])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

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
        if ((s.v[1159] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_exp_ad(649, A::scale(s.ad_value(648), s.v[365]));
        }

        s.v[1164] = if ((s.v[648] * s.v[365]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1163] != 0.0))) && (s.v[1164] != 0.0)) {
            s.store_div_from_scalar_ad(649, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(648), s.v[365])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(648), s.v[365])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(648), s.v[365])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1163] != 0.0))) && (!(s.v[1164] != 0.0))) {
            s.store_scale_ad(649, A::offset(A::mul(A::offset(A::scale(s.ad_value(648), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(648), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(648), s.v[365]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(390, s.v[387]);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(391, s.v[388]);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(392, s.v[389]);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(393, p.p824);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(394, p.p825);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(395, p.p826);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(396, p.p821);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(397, p.p822);
        }

        if (s.v[1159] != 0.0) {
            s.store_scalar(398, p.p823);
        }

        s.v[1165] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scalar(390, (s.v[388] + s.v[389]));
        }

        if ((s.v[1159] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scalar(393, (0.9 * (p.p825).min(p.p826)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scalar(396, (p.p822 + p.p823));
        }

        s.v[1166] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scalar(391, (s.v[387] + s.v[389]));
        }

        if ((s.v[1159] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scalar(394, (0.9 * (p.p824).min(p.p826)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scalar(397, (p.p821 + p.p823));
        }

        s.v[1167] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_scalar(392, (s.v[387] + s.v[388]));
        }

        if ((s.v[1159] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_scalar(395, (0.9 * (p.p824).min(p.p825)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_scalar(398, (p.p821 + p.p822));
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(650, &A::min(A::min(s.ad_value(390), s.ad_value(391)), s.ad_value(392)));
        }

        if (s.v[1159] != 0.0) {
            s.store_scale(651, 650, 0.1);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(371, &A::max(A::max(s.ad_value(393), s.ad_value(394)), s.ad_value(395)));
        }

        if (s.v[1159] != 0.0) {
            s.store_mul_ad_rhs(652, 650, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371)))));
        }

        if (s.v[1159] != 0.0) {
            s.store_offset_ad(653, A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)), (-0.05));
        }

        s.v[1168] = if ((s.v[557] * s.v[667]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1168] != 0.0)) {
            s.store_scale_ad(448, A::ln(A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(557), s.ad_value(667))), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1168] != 0.0))) {
            s.store_scalar(448, 100000000.0);
        }

        s.v[1169] = if ((s.v[558] * s.v[668]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_scale_ad(449, A::ln(A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(558), s.ad_value(668))), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1169] != 0.0))) {
            s.store_scalar(449, 100000000.0);
        }

        s.v[1170] = if ((s.v[559] * s.v[669]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_scale_ad(450, A::ln(A::offset(A::div_from_scalar(p.p815, A::mul(s.ad_value(559), s.ad_value(669))), 1.0)), s.v[364]);
        }

        if ((s.v[1159] != 0.0) && (!(s.v[1170] != 0.0))) {
            s.store_scalar(450, 100000000.0);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(675, &A::min(A::min(s.ad_value(448), s.ad_value(449)), s.ad_value(450)));
        }

        s.v[1171] = if ((((s.v[675] * s.v[365])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_exp_ad(676, A::scale(s.ad_value(675), s.v[365]));
        }

        s.v[1172] = if ((s.v[675] * s.v[365]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (!(s.v[1171] != 0.0))) && (s.v[1172] != 0.0)) {
            s.store_div_from_scalar_ad(676, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(675), s.v[365])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(675), s.v[365])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(675), s.v[365])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1159] != 0.0) && (!(s.v[1171] != 0.0))) && (!(s.v[1172] != 0.0))) {
            s.store_scale_ad(676, A::offset(A::mul(A::offset(A::scale(s.ad_value(675), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(675), s.v[365]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(675), s.v[365]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(390, 563);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(391, 564);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(392, 565);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(393, 505);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(394, 506);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(395, 507);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(396, 502);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(397, 503);
        }

        if (s.v[1159] != 0.0) {
            s.copy_ad(398, 504);
        }

        s.v[1173] = if (s.v[667] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_add(390, 564, 565);
        }

        if ((s.v[1159] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_scale_ad(393, A::min(s.ad_value(506), s.ad_value(507)), 0.9);
        }

        if ((s.v[1159] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_add(396, 503, 504);
        }

        s.v[1174] = if (s.v[668] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_add(391, 563, 565);
        }

        if ((s.v[1159] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_scale_ad(394, A::min(s.ad_value(505), s.ad_value(507)), 0.9);
        }

        if ((s.v[1159] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_add(397, 502, 504);
        }

        s.v[1175] = if (s.v[669] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add(392, 563, 564);
        }

        if ((s.v[1159] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_scale_ad(395, A::min(s.ad_value(505), s.ad_value(506)), 0.9);
        }

        if ((s.v[1159] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_add(398, 502, 503);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(677, &A::min(A::min(s.ad_value(390), s.ad_value(391)), s.ad_value(392)));
        }

        if (s.v[1159] != 0.0) {
            s.store_scale(678, 677, 0.1);
        }

        if (s.v[1159] != 0.0) {
            s.store_ad(371, &A::max(A::max(s.ad_value(393), s.ad_value(394)), s.ad_value(395)));
        }

        if (s.v[1159] != 0.0) {
            s.store_mul_ad_rhs(679, 677, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(371)))));
        }

        if (s.v[1159] != 0.0) {
            s.store_offset_ad(680, A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)), (-0.05));
        }

        s.v[1176] = if (s.v[468] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1177, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1178, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1179, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1186, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1188, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1190, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1191, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1192, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1194, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1195, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1198, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1199, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1201, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1202, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1203, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1205, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1206, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1207, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1208, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1210, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1211, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1213, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1214, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1217, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1221, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(492, 0.4);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(493, 0.65);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(494, 0.8);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scale_ad(479, A::neg(s.ad_value(492)), p.p921);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scale_ad(480, A::neg(s.ad_value(493)), p.p921);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scale_ad(481, A::neg(s.ad_value(494)), p.p921);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(482, 0.1);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(483, 0.2);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1190, 0.0);
        }

        s.v[1225] = if !(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1226] = if (s.v[479] < s.v[648]) { 1.0 } else { 0.0 };

        s.v[1227] = if (((((-0.5) * (s.v[479] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) {
            s.store_exp_ad(1188, A::scale(s.ad_value(479), (s.v[365] * (-0.5))));
        }

        s.v[1228] = if (((-0.5) * (s.v[479] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && (!(s.v[1227] != 0.0))) && (s.v[1228] != 0.0)) {
            let assign15450_ad_e12918: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(479), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(479), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(479), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1188, &assign15450_ad_e12918);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && (!(s.v[1227] != 0.0))) && (!(s.v[1228] != 0.0))) {
            s.store_scale_ad(1188, A::offset(A::mul(A::offset(A::scale(s.ad_value(479), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(479), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(479), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) {
            s.store_div_from_scalar(1189, 1.0, 1188);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) {
            s.store_square(1186, 1189);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) {
            s.store_mul_ad_lhs(1186, A::offset(A::scale(A::sub(s.ad_value(479), s.ad_value(648)), s.v[365]), 1.0), 649);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) {
            s.store_sqrt(1189, 1186);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) {
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.v[1229] = if (s.v[479] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1229] != 0.0)) {
            s.store_scale_ad(1190, A::ln(A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1188), 1.0), A::offset(s.ad_value(1188), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1229] != 0.0))) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1189), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1189), 1.0), A::offset(A::scale(s.ad_value(1189), 3.0), 1.0))))), (s.v[364] * 2.0)), 479);
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
        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) {
            s.store_sub(1191, 650, 1190);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) {
            s.store_scale_ad(1192, A::sub(A::add(s.ad_value(479), s.ad_value(1191)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(479), s.ad_value(1191)), A::sub(s.ad_value(479), s.ad_value(1191))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) {
            s.store_scale_ad(1193, A::sub(A::add(s.ad_value(479), s.ad_value(653)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(479), s.ad_value(653)), A::sub(s.ad_value(479), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1225] != 0.0)) {
            s.store_scale_ad(1194, A::sub(s.ad_value(479), A::sqrt(A::offset(A::mul(s.ad_value(479), s.ad_value(479)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1230] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1230] != 0.0)) {
            s.store_scalar(1222, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.v[1231] = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (s.v[1231] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1232] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) && (s.v[1232] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) && (!(s.v[1232] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p824)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1233] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[423]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) && (!(s.v[1233] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_scale(1202, 1195, s.v[417]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[378]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.v[1234] = if (p.p838 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (s.v[1234] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[402]), s.ad_value(1198)), s.v[432]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1235] = if (((-p.p824) * s.v[405]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (s.v[1235] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1235] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[429]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[429])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1236] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (s.v[1236] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1236] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1237] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (s.v[1237] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1237] != 0.0))) {
            let assign15990_ad_e13814: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign15990_ad_e13814);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1238] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (s.v[1238] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1239] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1238] != 0.0))) && (s.v[1239] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1238] != 0.0))) && (!(s.v[1239] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[429]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p838);
        }

        s.v[1240] = if (p.p844 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (s.v[1240] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1241] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1240] != 0.0))) && (s.v[1241] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1240] != 0.0))) && (!(s.v[1241] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1240] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[420]), s.ad_value(1195)), s.v[405]);
        }

        s.v[1242] = if (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1240] != 0.0))) && (s.v[1242] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(435)), s.ad_value(1220)));
        }

        s.v[1243] = if (((-s.v[435]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1240] != 0.0))) && (!(s.v[1242] != 0.0))) && (s.v[1243] != 0.0)) {
            let assign16180_ad_e14141: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign16180_ad_e14141);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1240] != 0.0))) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) {
            let assign16190_ad_e14191: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign16190_ad_e14191);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1240] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(479), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p844);
        }

        s.v[1244] = if (p.p853 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1245] = if (s.v[1194] > ((-s.v[438]) * p.p853)) { 1.0 } else { 0.0 };

        s.v[1246] = if (p.p856 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1244] != 0.0))) && (s.v[1245] != 0.0)) && (s.v[1246] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[442]), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1244] != 0.0))) && (s.v[1245] != 0.0)) && (!(s.v[1246] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[442])), p.p856);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1244] != 0.0))) && (s.v[1245] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) && (!(s.v[1244] != 0.0))) && (!(s.v[1245] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p853)), s.v[445]), s.v[439]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1230] != 0.0))) {
            s.store_mul_ad_lhs(1222, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1247] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1247] != 0.0)) {
            s.store_scalar(1223, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.v[1248] = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (s.v[1248] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1249] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) && (s.v[1249] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) && (!(s.v[1249] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p825)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1250] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) && (s.v[1250] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) && (!(s.v[1250] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_scale(1202, 1195, s.v[418]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[379]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.v[1251] = if (p.p839 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (s.v[1251] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[403]), s.ad_value(1198)), s.v[433]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1252] = if (((-p.p825) * s.v[406]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[430]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[430])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1253] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (s.v[1253] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (!(s.v[1253] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1254] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (s.v[1254] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (!(s.v[1254] != 0.0))) {
            let assign16690_ad_e14957: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign16690_ad_e14957);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1255] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (s.v[1255] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1256] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (!(s.v[1255] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[430]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p839);
        }

        s.v[1257] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (s.v[1257] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1258] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1257] != 0.0))) && (!(s.v[1258] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1257] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[421]), s.ad_value(1195)), s.v[406]);
        }

        s.v[1259] = if (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1257] != 0.0))) && (s.v[1259] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(436)), s.ad_value(1220)));
        }

        s.v[1260] = if (((-s.v[436]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1257] != 0.0))) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            let assign16880_ad_e15284: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign16880_ad_e15284);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1257] != 0.0))) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            let assign16890_ad_e15334: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign16890_ad_e15334);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1257] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(479), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p845);
        }

        s.v[1261] = if (p.p854 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (s.v[1261] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1262] = if (s.v[1194] > ((-s.v[438]) * p.p854)) { 1.0 } else { 0.0 };

        s.v[1263] = if (p.p857 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) && (s.v[1263] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[443]), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) && (!(s.v[1263] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[443])), p.p857);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p854)), s.v[446]), s.v[440]);
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
        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1247] != 0.0))) {
            s.store_mul_ad_lhs(1223, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1264] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1264] != 0.0)) {
            s.store_scalar(1224, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.v[1265] = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (s.v[1265] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1266] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) && (s.v[1266] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) && (!(s.v[1266] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p826)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1267] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) && (s.v[1267] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[425]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) && (!(s.v[1267] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[425]), p.p826);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_scale(1202, 1195, s.v[419]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[380]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.v[1268] = if (p.p840 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (s.v[1268] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[404]), s.ad_value(1198)), s.v[434]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1269] = if (((-p.p826) * s.v[407]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (s.v[1269] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (!(s.v[1269] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[431]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[431])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1270] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (s.v[1270] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (!(s.v[1270] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1271] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (s.v[1271] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (!(s.v[1271] != 0.0))) {
            let assign17390_ad_e16100: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign17390_ad_e16100);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1272] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (s.v[1272] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1273] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1273] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (!(s.v[1272] != 0.0))) && (!(s.v[1273] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (!(s.v[1272] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[431]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p840);
        }

        s.v[1274] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (s.v[1274] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1275] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1275] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1275] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]), p.p826);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1274] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[422]), s.ad_value(1195)), s.v[407]);
        }

        s.v[1276] = if (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1276] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(437)), s.ad_value(1220)));
        }

        s.v[1277] = if (((-s.v[437]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1276] != 0.0))) && (s.v[1277] != 0.0)) {
            let assign17580_ad_e16427: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign17580_ad_e16427);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            let assign17590_ad_e16477: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign17590_ad_e16477);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1274] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(479), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p846);
        }

        s.v[1278] = if (p.p855 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (s.v[1278] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1279] = if (s.v[1194] > ((-s.v[438]) * p.p855)) { 1.0 } else { 0.0 };

        s.v[1280] = if (p.p858 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1278] != 0.0))) && (s.v[1279] != 0.0)) && (s.v[1280] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[444]), A::scale(s.ad_value(1194), s.v[444])), A::scale(s.ad_value(1194), s.v[444])), A::scale(s.ad_value(1194), s.v[444]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1278] != 0.0))) && (s.v[1279] != 0.0)) && (!(s.v[1280] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[444])), p.p858);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1278] != 0.0))) && (s.v[1279] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) && (!(s.v[1278] != 0.0))) && (!(s.v[1279] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p855)), s.v[447]), s.v[441]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1264] != 0.0))) {
            s.store_mul_ad_lhs(1224, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_add_ad(469, A::add(A::mul(s.ad_value(640), s.ad_value(1222)), A::mul(s.ad_value(641), s.ad_value(1223))), A::mul(s.ad_value(642), s.ad_value(1224)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1190, 0.0);
        }

        s.v[1281] = if !(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1282] = if (s.v[480] < s.v[648]) { 1.0 } else { 0.0 };

        s.v[1283] = if (((((-0.5) * (s.v[480] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (s.v[1282] != 0.0)) && (s.v[1283] != 0.0)) {
            s.store_exp_ad(1188, A::scale(s.ad_value(480), (s.v[365] * (-0.5))));
        }

        s.v[1284] = if (((-0.5) * (s.v[480] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (s.v[1282] != 0.0)) && (!(s.v[1283] != 0.0))) && (s.v[1284] != 0.0)) {
            let assign17850_ad_e16848: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(480), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(480), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(480), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1188, &assign17850_ad_e16848);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (s.v[1282] != 0.0)) && (!(s.v[1283] != 0.0))) && (!(s.v[1284] != 0.0))) {
            s.store_scale_ad(1188, A::offset(A::mul(A::offset(A::scale(s.ad_value(480), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(480), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(480), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (s.v[1282] != 0.0)) {
            s.store_div_from_scalar(1189, 1.0, 1188);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (s.v[1282] != 0.0)) {
            s.store_square(1186, 1189);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (!(s.v[1282] != 0.0))) {
            s.store_mul_ad_lhs(1186, A::offset(A::scale(A::sub(s.ad_value(480), s.ad_value(648)), s.v[365]), 1.0), 649);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (!(s.v[1282] != 0.0))) {
            s.store_sqrt(1189, 1186);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (!(s.v[1282] != 0.0))) {
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.v[1285] = if (s.v[480] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (s.v[1285] != 0.0)) {
            s.store_scale_ad(1190, A::ln(A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1188), 1.0), A::offset(s.ad_value(1188), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) && (!(s.v[1285] != 0.0))) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1189), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1189), 1.0), A::offset(A::scale(s.ad_value(1189), 3.0), 1.0))))), (s.v[364] * 2.0)), 480);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_sub(1191, 650, 1190);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_scale_ad(1192, A::sub(A::add(s.ad_value(480), s.ad_value(1191)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(480), s.ad_value(1191)), A::sub(s.ad_value(480), s.ad_value(1191))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_scale_ad(1193, A::sub(A::add(s.ad_value(480), s.ad_value(653)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(480), s.ad_value(653)), A::sub(s.ad_value(480), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_scale_ad(1194, A::sub(s.ad_value(480), A::sqrt(A::offset(A::mul(s.ad_value(480), s.ad_value(480)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1286] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1286] != 0.0)) {
            s.store_scalar(1222, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.v[1287] = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (s.v[1287] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1288] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) && (s.v[1288] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) && (!(s.v[1288] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p824)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1289] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) && (s.v[1289] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[423]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) && (!(s.v[1289] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_scale(1202, 1195, s.v[417]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[378]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.v[1290] = if (p.p838 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (s.v[1290] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[402]), s.ad_value(1198)), s.v[432]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1291] = if (((-p.p824) * s.v[405]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1291] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[429]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[429])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1292] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1292] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1293] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1293] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1293] != 0.0))) {
            let assign18390_ad_e17744: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign18390_ad_e17744);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1294] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1294] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1295] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1294] != 0.0))) && (s.v[1295] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1294] != 0.0))) && (!(s.v[1295] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1294] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[429]), s.ad_value(1213)), (1.772453850905516 * 0.5));
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
        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p838);
        }

        s.v[1296] = if (p.p844 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (s.v[1296] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1297] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1296] != 0.0))) && (s.v[1297] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1296] != 0.0))) && (!(s.v[1297] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[420]), s.ad_value(1195)), s.v[405]);
        }

        s.v[1298] = if (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1296] != 0.0))) && (s.v[1298] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(435)), s.ad_value(1220)));
        }

        s.v[1299] = if (((-s.v[435]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1296] != 0.0))) && (!(s.v[1298] != 0.0))) && (s.v[1299] != 0.0)) {
            let assign18580_ad_e18071: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign18580_ad_e18071);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1296] != 0.0))) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            let assign18590_ad_e18121: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign18590_ad_e18121);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(480), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p844);
        }

        s.v[1300] = if (p.p853 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (s.v[1300] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1301] = if (s.v[1194] > ((-s.v[438]) * p.p853)) { 1.0 } else { 0.0 };

        s.v[1302] = if (p.p856 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1300] != 0.0))) && (s.v[1301] != 0.0)) && (s.v[1302] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[442]), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1300] != 0.0))) && (s.v[1301] != 0.0)) && (!(s.v[1302] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[442])), p.p856);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1300] != 0.0))) && (s.v[1301] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) && (!(s.v[1300] != 0.0))) && (!(s.v[1301] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p853)), s.v[445]), s.v[439]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1286] != 0.0))) {
            s.store_mul_ad_lhs(1222, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1303] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1303] != 0.0)) {
            s.store_scalar(1223, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.v[1304] = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (s.v[1304] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1305] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p825)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1306] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_scale(1202, 1195, s.v[418]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[379]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.v[1307] = if (p.p839 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (s.v[1307] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[403]), s.ad_value(1198)), s.v[433]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1308] = if (((-p.p825) * s.v[406]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[430]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[430])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1309] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1310] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1310] != 0.0))) {
            let assign19090_ad_e18887: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign19090_ad_e18887);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1311] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1311] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1312] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1311] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1311] != 0.0))) && (!(s.v[1312] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1311] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[430]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p839);
        }

        s.v[1313] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1314] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1313] != 0.0))) && (!(s.v[1314] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[421]), s.ad_value(1195)), s.v[406]);
        }

        s.v[1315] = if (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1315] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(436)), s.ad_value(1220)));
        }

        s.v[1316] = if (((-s.v[436]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1313] != 0.0))) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            let assign19280_ad_e19214: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign19280_ad_e19214);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1313] != 0.0))) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            let assign19290_ad_e19264: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign19290_ad_e19264);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(480), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p845);
        }

        s.v[1317] = if (p.p854 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (s.v[1317] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1318] = if (s.v[1194] > ((-s.v[438]) * p.p854)) { 1.0 } else { 0.0 };

        s.v[1319] = if (p.p857 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1317] != 0.0))) && (s.v[1318] != 0.0)) && (s.v[1319] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[443]), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1317] != 0.0))) && (s.v[1318] != 0.0)) && (!(s.v[1319] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[443])), p.p857);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1317] != 0.0))) && (s.v[1318] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) && (!(s.v[1317] != 0.0))) && (!(s.v[1318] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p854)), s.v[446]), s.v[440]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1303] != 0.0))) {
            s.store_mul_ad_lhs(1223, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1320] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1320] != 0.0)) {
            s.store_scalar(1224, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.v[1321] = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1322] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) && (s.v[1322] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) && (!(s.v[1322] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p826)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1323] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) && (s.v[1323] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[425]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) && (!(s.v[1323] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[425]), p.p826);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_scale(1202, 1195, s.v[419]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[380]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.v[1324] = if (p.p840 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (s.v[1324] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[404]), s.ad_value(1198)), s.v[434]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1325] = if (((-p.p826) * s.v[407]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (s.v[1325] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1325] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[431]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[431])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1326] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (s.v[1326] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1326] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1327] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (s.v[1327] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1327] != 0.0))) {
            let assign19790_ad_e20030: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign19790_ad_e20030);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1328] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (s.v[1328] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1329] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1328] != 0.0))) && (s.v[1329] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1328] != 0.0))) && (!(s.v[1329] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1328] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[431]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p840);
        }

        s.v[1330] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (s.v[1330] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1331] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1330] != 0.0))) && (s.v[1331] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1330] != 0.0))) && (!(s.v[1331] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]), p.p826);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1330] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[422]), s.ad_value(1195)), s.v[407]);
        }

        s.v[1332] = if (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1330] != 0.0))) && (s.v[1332] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(437)), s.ad_value(1220)));
        }

        s.v[1333] = if (((-s.v[437]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1330] != 0.0))) && (!(s.v[1332] != 0.0))) && (s.v[1333] != 0.0)) {
            let assign19980_ad_e20357: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign19980_ad_e20357);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1330] != 0.0))) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            let assign19990_ad_e20407: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign19990_ad_e20407);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1330] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(480), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p846);
        }

        s.v[1334] = if (p.p855 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (s.v[1334] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1335] = if (s.v[1194] > ((-s.v[438]) * p.p855)) { 1.0 } else { 0.0 };

        s.v[1336] = if (p.p858 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1334] != 0.0))) && (s.v[1335] != 0.0)) && (s.v[1336] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[444]), A::scale(s.ad_value(1194), s.v[444])), A::scale(s.ad_value(1194), s.v[444])), A::scale(s.ad_value(1194), s.v[444]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1334] != 0.0))) && (s.v[1335] != 0.0)) && (!(s.v[1336] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[444])), p.p858);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1334] != 0.0))) && (s.v[1335] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) && (!(s.v[1334] != 0.0))) && (!(s.v[1335] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p855)), s.v[447]), s.v[441]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1320] != 0.0))) {
            s.store_mul_ad_lhs(1224, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_add_ad(470, A::add(A::mul(s.ad_value(640), s.ad_value(1222)), A::mul(s.ad_value(641), s.ad_value(1223))), A::mul(s.ad_value(642), s.ad_value(1224)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1190, 0.0);
        }

        s.v[1337] = if !(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1338] = if (s.v[481] < s.v[648]) { 1.0 } else { 0.0 };

        s.v[1339] = if (((((-0.5) * (s.v[481] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) && (s.v[1339] != 0.0)) {
            s.store_exp_ad(1188, A::scale(s.ad_value(481), (s.v[365] * (-0.5))));
        }

        s.v[1340] = if (((-0.5) * (s.v[481] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) && (!(s.v[1339] != 0.0))) && (s.v[1340] != 0.0)) {
            let assign20250_ad_e20778: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(481), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(481), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(481), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1188, &assign20250_ad_e20778);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) && (!(s.v[1339] != 0.0))) && (!(s.v[1340] != 0.0))) {
            s.store_scale_ad(1188, A::offset(A::mul(A::offset(A::scale(s.ad_value(481), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(481), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(481), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) {
            s.store_div_from_scalar(1189, 1.0, 1188);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (s.v[1338] != 0.0)) {
            s.store_square(1186, 1189);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_mul_ad_lhs(1186, A::offset(A::scale(A::sub(s.ad_value(481), s.ad_value(648)), s.v[365]), 1.0), 649);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_sqrt(1189, 1186);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.v[1341] = if (s.v[481] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (s.v[1341] != 0.0)) {
            s.store_scale_ad(1190, A::ln(A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1188), 1.0), A::offset(s.ad_value(1188), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) && (!(s.v[1341] != 0.0))) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1189), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1189), 1.0), A::offset(A::scale(s.ad_value(1189), 3.0), 1.0))))), (s.v[364] * 2.0)), 481);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) {
            s.store_sub(1191, 650, 1190);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) {
            s.store_scale_ad(1192, A::sub(A::add(s.ad_value(481), s.ad_value(1191)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(481), s.ad_value(1191)), A::sub(s.ad_value(481), s.ad_value(1191))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) {
            s.store_scale_ad(1193, A::sub(A::add(s.ad_value(481), s.ad_value(653)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(481), s.ad_value(653)), A::sub(s.ad_value(481), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1337] != 0.0)) {
            s.store_scale_ad(1194, A::sub(s.ad_value(481), A::sqrt(A::offset(A::mul(s.ad_value(481), s.ad_value(481)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1342] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1342] != 0.0)) {
            s.store_scalar(1222, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.v[1343] = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1344] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && (s.v[1344] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && (!(s.v[1344] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p824)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1345] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && (s.v[1345] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[423]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && (!(s.v[1345] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_scale(1202, 1195, s.v[417]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[378]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.v[1346] = if (p.p838 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (s.v[1346] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[402]), s.ad_value(1198)), s.v[432]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1347] = if (((-p.p824) * s.v[405]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1347] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[429]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[429])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1348] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1348] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1348] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1349] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1349] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1349] != 0.0))) {
            let assign20790_ad_e21674: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign20790_ad_e21674);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1350] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1350] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1351] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1350] != 0.0))) && (s.v[1351] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1350] != 0.0))) && (!(s.v[1351] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1350] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[429]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p838);
        }

        s.v[1352] = if (p.p844 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (s.v[1352] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1353] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1352] != 0.0))) && (s.v[1353] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1352] != 0.0))) && (!(s.v[1353] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1352] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[420]), s.ad_value(1195)), s.v[405]);
        }

        s.v[1354] = if (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1352] != 0.0))) && (s.v[1354] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(435)), s.ad_value(1220)));
        }

        s.v[1355] = if (((-s.v[435]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1352] != 0.0))) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) {
            let assign20980_ad_e22001: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign20980_ad_e22001);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1352] != 0.0))) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) {
            let assign20990_ad_e22051: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign20990_ad_e22051);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1352] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(481), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p844);
        }

        s.v[1356] = if (p.p853 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (s.v[1356] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1357] = if (s.v[1194] > ((-s.v[438]) * p.p853)) { 1.0 } else { 0.0 };

        s.v[1358] = if (p.p856 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1356] != 0.0))) && (s.v[1357] != 0.0)) && (s.v[1358] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[442]), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1356] != 0.0))) && (s.v[1357] != 0.0)) && (!(s.v[1358] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[442])), p.p856);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1356] != 0.0))) && (s.v[1357] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) && (!(s.v[1356] != 0.0))) && (!(s.v[1357] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p853)), s.v[445]), s.v[439]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1342] != 0.0))) {
            s.store_mul_ad_lhs(1222, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1359] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1359] != 0.0)) {
            s.store_scalar(1223, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.v[1360] = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (s.v[1360] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1361] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) && (!(s.v[1361] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p825)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1362] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) && (s.v[1362] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) && (!(s.v[1362] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_scale(1202, 1195, s.v[418]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[379]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.v[1363] = if (p.p839 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (s.v[1363] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[403]), s.ad_value(1198)), s.v[433]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1364] = if (((-p.p825) * s.v[406]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (s.v[1364] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (!(s.v[1364] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[430]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[430])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1365] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (s.v[1365] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1366] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (s.v[1366] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (!(s.v[1366] != 0.0))) {
            let assign21490_ad_e22817: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign21490_ad_e22817);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1367] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (s.v[1367] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1368] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (!(s.v[1367] != 0.0))) && (s.v[1368] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (!(s.v[1367] != 0.0))) && (!(s.v[1368] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (!(s.v[1367] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[430]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p839);
        }

        s.v[1369] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (s.v[1369] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1370] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1369] != 0.0))) && (s.v[1370] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1369] != 0.0))) && (!(s.v[1370] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1369] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[421]), s.ad_value(1195)), s.v[406]);
        }

        s.v[1371] = if (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1369] != 0.0))) && (s.v[1371] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(436)), s.ad_value(1220)));
        }

        s.v[1372] = if (((-s.v[436]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1369] != 0.0))) && (!(s.v[1371] != 0.0))) && (s.v[1372] != 0.0)) {
            let assign21680_ad_e23144: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign21680_ad_e23144);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1369] != 0.0))) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) {
            let assign21690_ad_e23194: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign21690_ad_e23194);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1369] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(481), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p845);
        }

        s.v[1373] = if (p.p854 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (s.v[1373] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1374] = if (s.v[1194] > ((-s.v[438]) * p.p854)) { 1.0 } else { 0.0 };

        s.v[1375] = if (p.p857 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1373] != 0.0))) && (s.v[1374] != 0.0)) && (s.v[1375] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[443]), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1373] != 0.0))) && (s.v[1374] != 0.0)) && (!(s.v[1375] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[443])), p.p857);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1373] != 0.0))) && (s.v[1374] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) && (!(s.v[1373] != 0.0))) && (!(s.v[1374] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p854)), s.v[446]), s.v[440]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1359] != 0.0))) {
            s.store_mul_ad_lhs(1223, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1376] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1376] != 0.0)) {
            s.store_scalar(1224, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.v[1377] = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (s.v[1377] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1378] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) && (s.v[1378] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) && (!(s.v[1378] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p826)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1379] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) && (s.v[1379] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[425]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) && (!(s.v[1379] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[425]), p.p826);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_scale(1202, 1195, s.v[419]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[380]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.v[1380] = if (p.p840 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (s.v[1380] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[404]), s.ad_value(1198)), s.v[434]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1381] = if (((-p.p826) * s.v[407]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (s.v[1381] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (!(s.v[1381] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[431]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[431])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1382] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (s.v[1382] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (!(s.v[1382] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1383] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (s.v[1383] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (!(s.v[1383] != 0.0))) {
            let assign22190_ad_e23960: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign22190_ad_e23960);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1384] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (s.v[1384] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1385] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (!(s.v[1384] != 0.0))) && (s.v[1385] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (!(s.v[1384] != 0.0))) && (!(s.v[1385] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (!(s.v[1384] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[431]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p840);
        }

        s.v[1386] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (s.v[1386] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1387] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1386] != 0.0))) && (s.v[1387] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1386] != 0.0))) && (!(s.v[1387] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]), p.p826);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1386] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[422]), s.ad_value(1195)), s.v[407]);
        }

        s.v[1388] = if (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1386] != 0.0))) && (s.v[1388] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(437)), s.ad_value(1220)));
        }

        s.v[1389] = if (((-s.v[437]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1386] != 0.0))) && (!(s.v[1388] != 0.0))) && (s.v[1389] != 0.0)) {
            let assign22380_ad_e24287: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign22380_ad_e24287);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1386] != 0.0))) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) {
            let assign22390_ad_e24337: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign22390_ad_e24337);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1386] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(481), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p846);
        }

        s.v[1390] = if (p.p855 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (s.v[1390] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1391] = if (s.v[1194] > ((-s.v[438]) * p.p855)) { 1.0 } else { 0.0 };

        s.v[1392] = if (p.p858 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1390] != 0.0))) && (s.v[1391] != 0.0)) && (s.v[1392] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[444]), A::scale(s.ad_value(1194), s.v[444])), A::scale(s.ad_value(1194), s.v[444])), A::scale(s.ad_value(1194), s.v[444]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1390] != 0.0))) && (s.v[1391] != 0.0)) && (!(s.v[1392] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[444])), p.p858);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1390] != 0.0))) && (s.v[1391] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) && (!(s.v[1390] != 0.0))) && (!(s.v[1391] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p855)), s.v[447]), s.v[441]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1376] != 0.0))) {
            s.store_mul_ad_lhs(1224, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_add_ad(471, A::add(A::mul(s.ad_value(640), s.ad_value(1222)), A::mul(s.ad_value(641), s.ad_value(1223))), A::mul(s.ad_value(642), s.ad_value(1224)));
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) {
            s.store_scalar(1190, 0.0);
        }

        s.v[1393] = if !(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1394] = if (s.v[482] < s.v[648]) { 1.0 } else { 0.0 };

        s.v[1395] = if (((((-0.5) * (s.v[482] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (s.v[1394] != 0.0)) && (s.v[1395] != 0.0)) {
            s.store_exp_ad(1188, A::scale(s.ad_value(482), (s.v[365] * (-0.5))));
        }

        s.v[1396] = if (((-0.5) * (s.v[482] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (s.v[1394] != 0.0)) && (!(s.v[1395] != 0.0))) && (s.v[1396] != 0.0)) {
            let assign22650_ad_e24708: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(482), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(482), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(482), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1188, &assign22650_ad_e24708);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (s.v[1394] != 0.0)) && (!(s.v[1395] != 0.0))) && (!(s.v[1396] != 0.0))) {
            s.store_scale_ad(1188, A::offset(A::mul(A::offset(A::scale(s.ad_value(482), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(482), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(482), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (s.v[1394] != 0.0)) {
            s.store_div_from_scalar(1189, 1.0, 1188);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (s.v[1394] != 0.0)) {
            s.store_square(1186, 1189);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_mul_ad_lhs(1186, A::offset(A::scale(A::sub(s.ad_value(482), s.ad_value(648)), s.v[365]), 1.0), 649);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_sqrt(1189, 1186);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.v[1397] = if (s.v[482] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (s.v[1397] != 0.0)) {
            s.store_scale_ad(1190, A::ln(A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1188), 1.0), A::offset(s.ad_value(1188), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) && (!(s.v[1397] != 0.0))) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1189), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1189), 1.0), A::offset(A::scale(s.ad_value(1189), 3.0), 1.0))))), (s.v[364] * 2.0)), 482);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_sub(1191, 650, 1190);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_scale_ad(1192, A::sub(A::add(s.ad_value(482), s.ad_value(1191)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(482), s.ad_value(1191)), A::sub(s.ad_value(482), s.ad_value(1191))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_scale_ad(1193, A::sub(A::add(s.ad_value(482), s.ad_value(653)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(482), s.ad_value(653)), A::sub(s.ad_value(482), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_scale_ad(1194, A::sub(s.ad_value(482), A::sqrt(A::offset(A::mul(s.ad_value(482), s.ad_value(482)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1398] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1398] != 0.0)) {
            s.store_scalar(1222, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.v[1399] = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (s.v[1399] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1400] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) && (s.v[1400] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) && (!(s.v[1400] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p824)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1401] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[423]));
        }

    }

    pub(super) fn stamp_transient_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) && (!(s.v[1401] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_scale(1202, 1195, s.v[417]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[378]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.v[1402] = if (p.p838 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (s.v[1402] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[402]), s.ad_value(1198)), s.v[432]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1403] = if (((-p.p824) * s.v[405]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1403] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1403] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[429]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[429])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1404] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1404] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1404] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1405] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1405] != 0.0))) {
            let assign23190_ad_e25604: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign23190_ad_e25604);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1406] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1406] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1407] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1406] != 0.0))) && (s.v[1407] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1406] != 0.0))) && (!(s.v[1407] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1406] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[429]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p838);
        }

        s.v[1408] = if (p.p844 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (s.v[1408] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1409] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1408] != 0.0))) && (s.v[1409] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1408] != 0.0))) && (!(s.v[1409] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]), p.p824);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1408] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[420]), s.ad_value(1195)), s.v[405]);
        }

        s.v[1410] = if (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1408] != 0.0))) && (s.v[1410] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(435)), s.ad_value(1220)));
        }

        s.v[1411] = if (((-s.v[435]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1408] != 0.0))) && (!(s.v[1410] != 0.0))) && (s.v[1411] != 0.0)) {
            let assign23380_ad_e25931: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign23380_ad_e25931);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1408] != 0.0))) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) {
            let assign23390_ad_e25981: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign23390_ad_e25981);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1408] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(482), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p844);
        }

        s.v[1412] = if (p.p853 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (s.v[1412] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1413] = if (s.v[1194] > ((-s.v[438]) * p.p853)) { 1.0 } else { 0.0 };

        s.v[1414] = if (p.p856 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) && (s.v[1414] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[442]), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442])), A::scale(s.ad_value(1194), s.v[442]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) && (!(s.v[1414] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[442])), p.p856);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) && (!(s.v[1412] != 0.0))) && (!(s.v[1413] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p853)), s.v[445]), s.v[439]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1398] != 0.0))) {
            s.store_mul_ad_lhs(1222, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1415] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1415] != 0.0)) {
            s.store_scalar(1223, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.v[1416] = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (s.v[1416] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1417] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) && (s.v[1417] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) && (!(s.v[1417] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p825)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1418] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) && (s.v[1418] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) && (!(s.v[1418] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_scale(1202, 1195, s.v[418]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[379]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.v[1419] = if (p.p839 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (s.v[1419] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_scale_ad(1205, A::div(A::scale(s.ad_value(1202), s.v[403]), s.ad_value(1198)), s.v[433]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_square(1207, 1206);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_sqrt_ad(1208, A::div(A::square(s.ad_value(1207)), A::offset(A::square(s.ad_value(1207)), 1.0)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_sqrt(1209, 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_mul(1210, 1208, 1209);
        }

        s.v[1420] = if (((-p.p825) * s.v[406]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_div_from_scalar_ad(1211, 1.0, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (!(s.v[1420] != 0.0))) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_div_ad(1212, A::mul(s.ad_value(1201), s.ad_value(1211)), A::add(s.ad_value(1201), s.ad_value(1211)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_sqrt_ad(1213, A::scale(A::div(s.ad_value(1205), s.ad_value(1209)), 0.375));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_sub_ad_lhs(1214, A::scale(A::mul(s.ad_value(1206), s.ad_value(1209)), 2.0), 1208);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_add_ad(1215, A::sub(A::mul(A::scale(s.ad_value(1206), s.v[430]), s.ad_value(1209)), A::scale(s.ad_value(1208), s.v[430])), A::scale(A::mul(s.ad_value(1205), s.ad_value(1210)), 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_mul_ad_lhs(1216, A::offset(s.ad_value(1214), (-1.0)), 1213);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_square(1177, 1216);
        }

        s.v[1421] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (s.v[1421] != 0.0)) {
            s.store_div_from_scalar_ad(1178, 1.0, A::offset(A::scale(s.ad_value(1216), s.v[366]), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (!(s.v[1421] != 0.0))) {
            s.store_div_from_scalar_ad(1178, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1216), s.v[366])));
        }

        s.v[1422] = if (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_exp_ad(1195, A::sub(s.ad_value(1215), s.ad_value(1177)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (!(s.v[1422] != 0.0))) {
            let assign23890_ad_e26747: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1195, &assign23890_ad_e26747);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_mul_ad_lhs(1179, A::add(A::add(A::scale(s.ad_value(1178), 0.29214664), A::scale(A::square(s.ad_value(1178)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(1178)), s.ad_value(1178)), s.v[368])), 1195);
        }

        s.v[1423] = if (s.v[1216] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (s.v[1423] != 0.0)) {
            s.copy_ad(1217, 1179);
        }

        s.v[1424] = if (s.v[1215] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (!(s.v[1423] != 0.0))) && (s.v[1424] != 0.0)) {
            s.store_exp(1195, 1215);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (!(s.v[1423] != 0.0))) && (!(s.v[1424] != 0.0))) {
            s.store_div_from_scalar_ad(1195, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1215)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (!(s.v[1423] != 0.0))) {
            s.store_sub_ad_lhs(1217, A::scale(s.ad_value(1195), 2.0), 1179);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_scale_ad(1218, A::div(A::scale(s.ad_value(1217), s.v[430]), s.ad_value(1213)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_scale_ad(1204, A::mul(A::mul(s.ad_value(1203), s.ad_value(1218)), s.ad_value(1212)), p.p839);
        }

        s.v[1425] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (s.v[1425] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        s.v[1426] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1425] != 0.0))) && (s.v[1426] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1425] != 0.0))) && (!(s.v[1426] != 0.0))) {
            s.store_powf_ad(1195, A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]), p.p825);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1425] != 0.0))) {
            s.store_scale_ad(1220, A::div(A::scale(A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[421]), s.ad_value(1195)), s.v[406]);
        }

        s.v[1427] = if (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1425] != 0.0))) && (s.v[1427] != 0.0)) {
            s.store_exp_ad(1195, A::div(A::neg(s.ad_value(436)), s.ad_value(1220)));
        }

        s.v[1428] = if (((-s.v[436]) / s.v[1220]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1425] != 0.0))) && (!(s.v[1427] != 0.0))) && (s.v[1428] != 0.0)) {
            let assign24080_ad_e27074: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(1220))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1195, 1e-100, assign24080_ad_e27074);
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1425] != 0.0))) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            let assign24090_ad_e27124: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(1220)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1195, &assign24090_ad_e27124);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1425] != 0.0))) {
            s.store_scale_ad(1219, A::mul(A::mul(A::mul(s.ad_value(482), s.ad_value(1220)), s.ad_value(1220)), s.ad_value(1195)), p.p845);
        }

        s.v[1429] = if (p.p854 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (s.v[1429] != 0.0)) {
            s.store_scalar(1221, 1.0);
        }

        s.v[1430] = if (s.v[1194] > ((-s.v[438]) * p.p854)) { 1.0 } else { 0.0 };

        s.v[1431] = if (p.p857 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1429] != 0.0))) && (s.v[1430] != 0.0)) && (s.v[1431] != 0.0)) {
            s.store_mul_ad(1195, A::mul(A::mul(A::scale(s.ad_value(1194), s.v[443]), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443])), A::scale(s.ad_value(1194), s.v[443]));
        }

        if ((((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1429] != 0.0))) && (s.v[1430] != 0.0)) && (!(s.v[1431] != 0.0))) {
            s.store_powf_ad(1195, A::abs(A::scale(s.ad_value(1194), s.v[443])), p.p857);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1429] != 0.0))) && (s.v[1430] != 0.0)) {
            s.store_div_from_scalar_ad(1221, 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) && (!(s.v[1429] != 0.0))) && (!(s.v[1430] != 0.0))) {
            s.store_offset_ad(1221, A::scale(A::offset(s.ad_value(1194), (s.v[438] * p.p854)), s.v[446]), s.v[440]);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1415] != 0.0))) {
            s.store_mul_ad_lhs(1223, A::scale(A::add(A::add(A::add(s.ad_value(1196), s.ad_value(1197)), s.ad_value(1204)), s.ad_value(1219)), p.p29), 1221);
        }

        s.v[1432] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (s.v[1432] != 0.0)) {
            s.store_scalar(1224, 0.0);
        }

        if (((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.v[1433] = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (s.v[1433] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.v[1434] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) && (s.v[1434] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) && (!(s.v[1434] != 0.0))) {
            s.store_scale_ad(1200, A::add(A::div(A::mul(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199))), A::sub_from_scalar(1.0, s.ad_value(1199))), s.ad_value(1199)), (1.0 - (2.0 * p.p826)));
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_add(1201, 1199, 1200);
        }

        s.v[1435] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) && (s.v[1435] != 0.0)) {
            s.store_sqrt_ad(1195, A::scale(s.ad_value(1198), s.v[425]));
        }

        if (((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) && (!(s.v[1435] != 0.0))) {
            s.store_powf_ad(1195, A::scale(s.ad_value(1198), s.v[425]), p.p826);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_scale(1202, 1195, s.v[419]);
        }

        if ((((s.v[1159] != 0.0) && (s.v[1176] != 0.0)) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_scale_ad(1203, A::mul(A::offset(s.ad_value(1189), (-1.0)), s.ad_value(1202)), s.v[380]);
        }

    }
}
