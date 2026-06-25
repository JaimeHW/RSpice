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
        s.v[990] = if (p.p37 >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[990] != 0.0) {
            s.store_scalar(0, 1.0);
        }

        if (!(s.v[990] != 0.0)) {
            s.store_scalar(0, (-1.0));
        }

        s.v[767] = (8.8541878176e-12 * 11.8);

        s.v[991] = if (p.p51 < 0.5) { 1.0 } else { 0.0 };

        if (s.v[991] != 0.0) {
            s.store_scalar(1, 0.0);
        }

        s.v[992] = if (p.p51 < 1.5) { 1.0 } else { 0.0 };

        if ((!(s.v[991] != 0.0)) && (s.v[992] != 0.0)) {
            s.store_scalar(1, 1.0);
        }

        s.v[993] = if (p.p51 < 2.5) { 1.0 } else { 0.0 };

        if (((!(s.v[991] != 0.0)) && (!(s.v[992] != 0.0))) && (s.v[993] != 0.0)) {
            s.store_scalar(1, 2.0);
        }

        s.v[994] = if (p.p51 < 4.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[991] != 0.0)) && (!(s.v[992] != 0.0))) && (!(s.v[993] != 0.0))) && (s.v[994] != 0.0)) {
            s.store_scalar(1, 3.0);
        }

        s.v[995] = if (p.p51 < 7.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[991] != 0.0)) && (!(s.v[992] != 0.0))) && (!(s.v[993] != 0.0))) && (!(s.v[994] != 0.0))) && (s.v[995] != 0.0)) {
            s.store_scalar(1, 5.0);
        }

        if (((((!(s.v[991] != 0.0)) && (!(s.v[992] != 0.0))) && (!(s.v[993] != 0.0))) && (!(s.v[994] != 0.0))) && (!(s.v[995] != 0.0))) {
            s.store_scalar(1, 9.0);
        }

        s.v[2] = 1000.0;

        s.v[3] = 10.0;

        s.v[4] = (1.0 / s.v[3]);

        s.v[350] = (273.15 + p.p38);

        s.v[474] = 0.0;

        s.v[996] = if (p.p927 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[996] != 0.0) {
            s.store_scalar(474, 1.0);
        }

        if (!(s.v[996] != 0.0)) {
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

        s.v[997] = if ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[997] != 0.0) {
            s.store_scalar(473, 1.0);
        }

        if (!(s.v[997] != 0.0)) {
            s.store_scalar(473, 0.0);
        }

        s.v[998] = if (s.v[473] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[998] != 0.0) {
            s.store_scalar(457, (if ((p.p827 * p.p866) > 1e-18) { (p.p827 * p.p866) } else { 1e-18 }));
        }

        if (s.v[998] != 0.0) {
            s.store_scalar(458, (if ((p.p830 * p.p867) > 0.05) { (p.p830 * p.p867) } else { 0.05 }));
        }

        if (s.v[998] != 0.0) {
            s.store_scalar(459, (if ((if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) < 0.95) { (if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) } else { 0.95 }));
        }

        if (s.v[998] != 0.0) {
            s.store_scalar(460, (p.p836 * p.p869));
        }

        if (s.v[998] != 0.0) {
            s.store_offset(462, 460, s.v[375]);
        }

        if (s.v[998] != 0.0) {
            s.store_sub_from_scalar(467, 1.0, 459);
        }

        if (s.v[998] != 0.0) {
            s.store_div_from_scalar(468, 1.0, 467);
        }

        s.v[999] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[999] != 0.0) {
            s.store_scalar(505, p.p825);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(506, p.p826);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(507, p.p827);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(508, p.p828);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(509, p.p829);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(510, p.p830);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(511, p.p831);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(512, p.p832);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(513, p.p833);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(514, p.p834);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(515, p.p835);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(516, p.p836);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(517, p.p837);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(518, p.p838);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(519, p.p839);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(522, p.p840);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(523, p.p841);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(524, p.p842);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(520, p.p843);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(521, p.p844);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(525, p.p845);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(526, p.p846);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(527, p.p847);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(528, p.p848);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(529, p.p849);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(530, p.p850);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(531, p.p851);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(532, p.p852);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(533, p.p853);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(534, p.p854);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(535, p.p855);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(536, p.p856);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(537, p.p857);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(538, p.p858);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(539, p.p859);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(540, p.p860);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(541, p.p861);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(542, p.p862);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(543, p.p863);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(544, p.p864);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(545, p.p865);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(552, p.p928);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(553, p.p929);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(636, p.p872);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(637, p.p873);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(638, p.p874);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(639, p.p875);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(546, p.p866);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(547, p.p867);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(548, p.p868);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(549, p.p869);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(550, p.p870);
        }

        if (s.v[999] != 0.0) {
            s.store_scalar(551, p.p871);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(505, p.p876);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(506, p.p877);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(507, p.p878);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(508, p.p879);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(509, p.p880);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(510, p.p881);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(511, p.p882);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(512, p.p883);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(513, p.p884);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(514, p.p885);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(515, p.p886);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(516, p.p887);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(517, p.p888);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(518, p.p889);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(519, p.p890);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(522, p.p891);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(523, p.p892);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(524, p.p893);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(520, p.p894);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(521, p.p895);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(525, p.p896);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(526, p.p897);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(527, p.p898);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(528, p.p899);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(529, p.p900);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(530, p.p901);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(531, p.p902);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(532, p.p903);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(533, p.p904);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(534, p.p905);
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
        if (!(s.v[999] != 0.0)) {
            s.store_scalar(535, p.p906);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(536, p.p907);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(537, p.p908);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(538, p.p909);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(539, p.p910);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(540, p.p911);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(541, p.p912);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(542, p.p913);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(543, p.p914);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(544, p.p915);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(545, p.p916);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(552, p.p930);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(553, p.p931);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(636, p.p923);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(637, p.p924);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(638, p.p925);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(639, p.p926);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(546, p.p917);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(547, p.p918);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(548, p.p919);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(549, p.p920);
        }

        if (!(s.v[999] != 0.0)) {
            s.store_scalar(550, p.p921);
        }

        if (!(s.v[999] != 0.0)) {
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

        s.store_div_ad_lhs(591, A::scale(s.ad_value(520), s.v[767]), 506);

        s.store_div_ad_lhs(592, A::scale(s.ad_value(521), s.v[767]), 507);

        s.store_div_from_scalar(593, 1.0, 590);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 508);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar_ad(611, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[444], s.ad_value(543))));

        s.store_div_from_scalar_ad(612, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[444], s.ad_value(544))));

        s.store_div_from_scalar_ad(613, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[444], s.ad_value(545))));

        s.store_div_from_scalar(614, 1.0, 540);

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.store_mul_ad_lhs(617, A::mul(A::neg(A::mul(A::square(s.ad_value(611)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(543), (-1.0))))), s.ad_value(543)), 614);

        s.store_mul_ad_lhs(618, A::mul(A::neg(A::mul(A::square(s.ad_value(612)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(544), (-1.0))))), s.ad_value(544)), 615);

        s.store_mul_ad_lhs(619, A::mul(A::neg(A::mul(A::square(s.ad_value(613)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(545), (-1.0))))), s.ad_value(545)), 616);

        s.v[1000] = if ((((s.v[546] != 1.0) || (s.v[547] != 1.0)) || (s.v[548] != 1.0)) || (s.v[549] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1000] != 0.0) {
            s.store_scalar(635, 1.0);
        }

        if (!(s.v[1000] != 0.0)) {
            s.store_scalar(635, 0.0);
        }

        s.v[1001] = if (s.v[635] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1001] != 0.0) {
            s.store_ad(620, &{
                if ((s.v[507] * s.v[546]) > 1e-18) {
                    A::mul(s.ad_value(507), s.ad_value(546))
                } else {
                    A::constant(1e-18)
                }
            });
        }

        if (s.v[1001] != 0.0) {
            s.store_ad(621, &{
                if ((s.v[510] * s.v[547]) > 0.05) {
                    A::mul(s.ad_value(510), s.ad_value(547))
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[1001] != 0.0) {
            s.store_ad(622, &{
                if ((if ((s.v[513] * s.v[548]) > 0.05) { (s.v[513] * s.v[548]) } else { 0.05 }) < 0.95) {
                    {
                        if ((s.v[513] * s.v[548]) > 0.05) {
                            A::mul(s.ad_value(513), s.ad_value(548))
                        } else {
                            A::constant(0.05)
                        }
                    }
                } else {
                    A::constant(0.95)
                }
            });
        }

        if (s.v[1001] != 0.0) {
            s.store_mul(623, 516, 549);
        }

        if (s.v[1001] != 0.0) {
            s.store_offset(625, 623, s.v[375]);
        }

        if (s.v[1001] != 0.0) {
            s.store_sub_from_scalar(630, 1.0, 622);
        }

        if (s.v[1001] != 0.0) {
            s.store_div_from_scalar(631, 1.0, 630);
        }

        s.v[878] = 0.0;

        s.v[351] = ((ctx.temperature() + p.p56) + p.p35);

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

        if !(s.v[363] > 0.001) {
            s.store_scalar(363, 0.001);
        }

        s.v[718] = ((4.0 * 1.3806505e-23) * s.v[356]);

        s.v[365] = (((ctx.temperature() + p.p56) + p.p35)).max((273.15 + (-250.0)));

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

        if !(s.v[441] > 0.0) {
            s.store_scalar(441, 0.0);
        }

        if !(s.v[442] > 0.0) {
            s.store_scalar(442, 0.0);
        }

        if !(s.v[443] > 0.0) {
            s.store_scalar(443, 0.0);
        }

        s.v[1021] = if (s.v[473] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1021] != 0.0) {
            s.store_offset(461, 460, s.v[376]);
        }

        if (s.v[1021] != 0.0) {
            s.store_scale_ad(463, A::exp(A::scale(A::sub(A::scale(s.ad_value(462), s.v[369]), A::scale(s.ad_value(461), s.v[371])), 0.5)), ((s.v[366]) as f64).powf(1.5));
        }

        if (s.v[1021] != 0.0) {
            s.store_sub_ad(464, A::scale(s.ad_value(458), s.v[366]), A::scale(A::ln(s.ad_value(463)), (2.0 * s.v[370])));
        }

        if (s.v[1021] != 0.0) {
            s.store_add_ad_rhs(465, 464, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(464)), s.v[371])), 1.0)), s.v[370]));
        }

        if (s.v[1021] != 0.0) {
            s.store_div_from_scalar(466, 1.0, 465);
        }

        if (s.v[1021] != 0.0) {
            s.store_mul_ad_rhs(469, 457, A::pow(A::mul(s.ad_value(458), s.ad_value(466)), s.ad_value(459)));
        }

        if (s.v[1021] != 0.0) {
            s.store_mul_ad_lhs(470, A::mul(s.ad_value(469), s.ad_value(465)), 468);
        }

        if (s.v[1021] != 0.0) {
            s.store_scale(471, 469, 2.0);
        }

        s.store_offset(557, 514, s.v[376]);

        s.store_offset(558, 515, s.v[376]);

        s.store_offset(559, 516, s.v[376]);

        s.store_scale_ad(560, A::exp(A::scale(A::sub(A::scale(s.ad_value(554), s.v[369]), A::scale(s.ad_value(557), s.v[371])), 0.5)), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(561, A::exp(A::scale(A::sub(A::scale(s.ad_value(555), s.v[369]), A::scale(s.ad_value(558), s.v[371])), 0.5)), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp(A::scale(A::sub(A::scale(s.ad_value(556), s.v[369]), A::scale(s.ad_value(559), s.v[371])), 0.5)), ((s.v[366]) as f64).powf(1.5));

        s.store_mul_ad_lhs(563, A::mul(s.ad_value(517), s.ad_value(560)), 560);

        s.store_mul_ad_lhs(564, A::mul(s.ad_value(518), s.ad_value(561)), 561);

        s.store_mul_ad_lhs(565, A::mul(s.ad_value(519), s.ad_value(562)), 562);

        s.store_sub_ad(566, A::scale(s.ad_value(508), s.v[366]), A::scale(A::ln(s.ad_value(560)), (2.0 * s.v[370])));

        s.store_sub_ad(567, A::scale(s.ad_value(509), s.v[366]), A::scale(A::ln(s.ad_value(561)), (2.0 * s.v[370])));

        s.store_sub_ad(568, A::scale(s.ad_value(510), s.v[366]), A::scale(A::ln(s.ad_value(562)), (2.0 * s.v[370])));

        s.store_add_ad_rhs(569, 566, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(566)), s.v[371])), 1.0)), s.v[370]));

        s.store_add_ad_rhs(570, 567, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(567)), s.v[371])), 1.0)), s.v[370]));

        s.store_add_ad_rhs(571, 568, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(568)), s.v[371])), 1.0)), s.v[370]));

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_mul_ad_rhs(581, 505, A::pow(A::mul(s.ad_value(508), s.ad_value(572)), s.ad_value(511)));

        s.store_mul_ad_rhs(582, 506, A::pow(A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512)));

        s.store_mul_ad_rhs(583, 507, A::pow(A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513)));

        s.store_mul_ad_lhs(584, A::mul(s.ad_value(581), s.ad_value(569)), 578);

        s.store_mul_ad_lhs(585, A::mul(s.ad_value(582), s.ad_value(570)), 579);

        s.store_mul_ad_lhs(586, A::mul(s.ad_value(583), s.ad_value(571)), 580);

        s.store_scale(587, 581, 2.0);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_max_with_scalar_ad(599, A::scale(s.ad_value(557), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[370]);

        s.store_scale(602, 599, s.v[371]);

        s.store_scale(603, 600, s.v[371]);

        s.store_scale(604, 601, s.v[371]);

        s.store_scale_ad(605, A::sqrt(A::mul(A::scale(s.ad_value(528), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(599)), s.ad_value(599)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(606, A::sqrt(A::mul(A::scale(s.ad_value(529), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(600)), s.ad_value(600)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(607, A::sqrt(A::mul(A::scale(s.ad_value(530), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(601)), s.ad_value(601)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_ad_rhs(608, 534, A::offset(A::scale(s.ad_value(537), (s.v[365] - s.v[364])), 1.0));

        s.store_mul_ad_rhs(609, 535, A::offset(A::scale(s.ad_value(538), (s.v[365] - s.v[364])), 1.0));

        s.store_mul_ad_rhs(610, 536, A::offset(A::scale(s.ad_value(539), (s.v[365] - s.v[364])), 1.0));

        if !(s.v[608] > 0.0) {
            s.store_scalar(608, 0.0);
        }

        if !(s.v[609] > 0.0) {
            s.store_scalar(609, 0.0);
        }

        if !(s.v[610] > 0.0) {
            s.store_scalar(610, 0.0);
        }

        s.v[1022] = if (s.v[635] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1022] != 0.0) {
            s.store_offset(624, 623, s.v[376]);
        }

        if (s.v[1022] != 0.0) {
            s.store_scale_ad(626, A::exp(A::scale(A::sub(A::scale(s.ad_value(625), s.v[369]), A::scale(s.ad_value(624), s.v[371])), 0.5)), ((s.v[366]) as f64).powf(1.5));
        }

        if (s.v[1022] != 0.0) {
            s.store_sub_ad(627, A::scale(s.ad_value(621), s.v[366]), A::scale(A::ln(s.ad_value(626)), (2.0 * s.v[370])));
        }

        if (s.v[1022] != 0.0) {
            s.store_add_ad_rhs(628, 627, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(627)), s.v[371])), 1.0)), s.v[370]));
        }

        if (s.v[1022] != 0.0) {
            s.store_div_from_scalar(629, 1.0, 628);
        }

        if (s.v[1022] != 0.0) {
            s.store_mul_ad_rhs(632, 620, A::pow(A::mul(s.ad_value(621), s.ad_value(629)), s.ad_value(622)));
        }

        if (s.v[1022] != 0.0) {
            s.store_mul_ad_lhs(633, A::mul(s.ad_value(632), s.ad_value(628)), 631);
        }

        if (s.v[1022] != 0.0) {
            s.store_scale(634, 632, 2.0);
        }

        s.v[5] = 1.0;

        s.v[6] = 1.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

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

        s.v[1023] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1023] != 0.0) {
            s.store_scalar(5, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if (s.v[1023] != 0.0) {
            s.store_floor_ad(5, A::offset(s.ad_value(5), 0.5));
        }

        if (s.v[1023] != 0.0) {
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

        s.store_scale_ad(310, A::offset(A::scale(s.ad_value(309), p.p190), 1.0), (p.p188 * (1.0 + (p.p189 * s.v[308]))));

        s.store_scale_ad(311, A::offset(A::scale(s.ad_value(309), p.p194), 1.0), (p.p192 * (1.0 + (p.p193 * s.v[308]))));

        if (((s.v[7] + s.v[310]) - (2.0 * p.p191)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[7]) + ((-(2.0 * p.p191)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[8] + s.v[311]) - (2.0 * p.p195)) > 1e-9) {
            s.store_offset_ad(313, A::add(s.ad_value(8), s.ad_value(311)), (-(2.0 * p.p195)));
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
            s.store_offset_ad(320, A::offset(A::offset(s.ad_value(310), s.v[7]), (-(2.0 * p.p191))), p.p196);
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[8] + s.v[311]) - (2.0 * p.p195)) + p.p197) > 1e-9) {
            s.store_offset_ad(321, A::offset(A::add(s.ad_value(8), s.ad_value(311)), (-(2.0 * p.p195))), p.p197);
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
            s.store_offset_ad(324, A::add(s.ad_value(8), s.ad_value(311)), p.p197);
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
            s.store_sub_from_scalar_ad(330, s.v[13], A::scale(s.ad_value(311), 0.5));
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

        s.v[1024] = if (if self.param_given[122] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1024] != 0.0) {
            s.store_scalar(109, p.p122);
        }

        s.v[110] = p.p121;

        s.v[1025] = if (if self.param_given[123] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1025] != 0.0) {
            s.store_scalar(110, p.p123);
        }

        s.copy_ad(111, 109);

        s.v[1026] = if (if self.param_given[124] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1026] != 0.0) {
            s.store_scalar(111, p.p124);
        }

        s.copy_ad(112, 110);

        s.v[1027] = if (if self.param_given[125] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1027] != 0.0) {
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

        s.v[1028] = if (if self.param_given[138] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1028] != 0.0) {
            s.store_scalar(125, p.p138);
        }

        s.v[126] = p.p104;

        s.v[1029] = if (if self.param_given[139] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1029] != 0.0) {
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

        s.v[1030] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1030] != 0.0) {
            s.store_add_ad(44, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p200), p.p199), p.p198), A::scale(s.ad_value(316), p.p201)), A::scale(s.ad_value(318), p.p202));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(45, A::add(A::offset(A::scale(s.ad_value(314), p.p204), p.p203), A::scale(s.ad_value(316), p.p205)), A::scale(s.ad_value(318), p.p206));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(46, p.p207);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(47, p.p208);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(48, p.p209);
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(331, &A::scale({
                if ((1.0 + ((p.p211 * s.v[316]) * (((1.0 + (s.v[313] / p.p212))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p211), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p212)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p210));
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(332, &A::scale({
                if ((1.0 + ((p.p214 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p214), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p215)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p213));
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(333, &A::scale({
                if ((1.0 + ((p.p217 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p217), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p215)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p216));
        }

        s.v[1031] = if (s.v[312] > (2.0 * s.v[333])) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_scalar(334, 75000000000.0);
        }

        if ((s.v[1030] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_sub_ad(335, A::sqrt(A::add(s.ad_value(331), A::scale(s.ad_value(332), 0.5))), A::sqrt(s.ad_value(331)));
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
        if ((s.v[1030] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_add_ad(336, A::sqrt(s.ad_value(331)), A::mul(s.ad_value(334), A::ln(A::offset(A::mul(A::div(A::scale(s.ad_value(333), 2.0), s.ad_value(312)), A::offset(A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0))), 1.0))));
        }

        if ((s.v[1030] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_square(336, 336);
        }

        s.v[1032] = if (s.v[312] >= s.v[333]) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (!(s.v[1031] != 0.0))) && (s.v[1032] != 0.0)) {
            s.store_add_ad_rhs(336, 331, A::div(A::mul(s.ad_value(332), s.ad_value(333)), s.ad_value(312)));
        }

        if (((s.v[1030] != 0.0) && (!(s.v[1031] != 0.0))) && (!(s.v[1032] != 0.0))) {
            s.store_add_ad_rhs(336, 331, A::mul(s.ad_value(332), A::sub_from_scalar(2.0, A::div(s.ad_value(312), s.ad_value(333)))));
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad_rhs(49, 336, A::sub(A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p218)), A::scale(s.ad_value(315), p.p219)));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(50, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p222), p.p221), p.p220), A::scale(s.ad_value(316), p.p223)), A::scale(s.ad_value(318), p.p224));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(51, p.p225);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(52, p.p226);
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(53, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p229), p.p228), p.p227), A::scale(s.ad_value(316), p.p230)), A::scale(s.ad_value(318), p.p231));
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(54, &A::scale({
                if (1e-6 > (1.0 + (p.p233 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::offset(A::scale(s.ad_value(314), p.p233), 1.0)
                }
            }, p.p232));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(59, p.p234);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(60, p.p235);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(61, p.p238);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(62, p.p239);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(55, A::mul(A::offset(A::scale(A::powf(s.ad_value(314), p.p242), p.p241), p.p240), A::offset(A::scale(s.ad_value(316), p.p243), 1.0)), A::offset(A::scale(s.ad_value(318), p.p244), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(56, p.p246);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(57, p.p245);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(58, p.p247);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(66, A::scale(A::powf(s.ad_value(314), p.p249), p.p248), A::offset(A::scale(s.ad_value(316), p.p250), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(67, p.p252);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(68, p.p251);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(63, A::scale(A::powf(s.ad_value(314), p.p254), p.p253), A::offset(A::scale(s.ad_value(316), p.p255), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(64, p.p257);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(65, p.p256);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale_ad(337, A::offset(A::scale(s.ad_value(316), p.p260), 1.0), p.p259);
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(338, &A::scale({
                if ((1.0 + (p.p262 * s.v[316])) > 0.001) {
                    A::offset(A::scale(s.ad_value(316), p.p262), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p261));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(339, A::offset(A::mul(A::div(A::mul(s.ad_value(337), s.ad_value(338)), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::div(A::neg(s.ad_value(312)), s.ad_value(338))))), 1.0), A::mul(A::div_from_scalar((p.p263 * p.p264), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(312)), 1.0 / (p.p264))))));
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(339, &{
                if (s.v[339] > 1e-15) {
                    s.ad_value(339)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(340, A::offset(A::scale(s.ad_value(316), p.p265), 1.0), A::mul(A::scale(s.ad_value(316), p.p266), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p267)), 1.0))));
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad_lhs(69, A::div(A::scale(s.ad_value(313), p.p258), A::mul(s.ad_value(339), s.ad_value(312))), 340);
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(70, A::add(A::offset(A::scale(s.ad_value(314), p.p269), p.p268), A::scale(s.ad_value(316), p.p270)), A::scale(s.ad_value(318), p.p271));
        }

        if (s.v[1030] != 0.0) {
            s.store_scale_ad(71, A::offset(A::scale(s.ad_value(316), p.p273), 1.0), p.p272);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(72, p.p274);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(73, p.p275);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(74, p.p276);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(75, A::mul(A::offset(A::scale(A::powf(s.ad_value(314), p.p279), p.p278), p.p277), A::offset(A::scale(s.ad_value(316), p.p280), 1.0)), A::offset(A::scale(s.ad_value(318), p.p281), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(76, p.p282);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(77, p.p283);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(78, p.p284);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(79, A::mul(A::scale(A::offset(A::scale(s.ad_value(314), p.p286), 1.0), p.p285), A::offset(A::scale(s.ad_value(316), p.p287), 1.0)), A::offset(A::scale(s.ad_value(318), p.p288), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(80, p.p289);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(81, p.p290);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(82, A::scale(s.ad_value(316), p.p291), A::offset(A::scale(s.ad_value(316), p.p292), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(83, p.p293);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(84, p.p294);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(85, p.p295);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(86, A::mul(A::offset(A::mul(A::div(A::scale(s.ad_value(340), p.p297), s.ad_value(339)), A::powf(s.ad_value(314), p.p298)), p.p296), A::offset(A::scale(s.ad_value(316), p.p299), 1.0)), A::offset(A::scale(s.ad_value(318), p.p300), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(87, A::add(A::offset(A::scale(s.ad_value(314), p.p302), p.p301), A::scale(s.ad_value(316), p.p303)), A::scale(s.ad_value(318), p.p304));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(88, p.p305);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(89, p.p306);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(90, p.p307);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar_ad(91, p.p308, A::offset(A::scale(s.ad_value(314), p.p309), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(92, A::scale(A::powf(s.ad_value(314), p.p311), p.p310), A::offset(A::scale(s.ad_value(316), p.p312), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_powf(341, 314, p.p314);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_ad(93, A::mul(A::scale(s.ad_value(341), p.p313), A::offset(A::scale(s.ad_value(316), p.p316), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p315), s.ad_value(341)), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_powf(341, 314, p.p318);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_ad(94, A::mul(A::scale(s.ad_value(341), p.p317), A::offset(A::scale(s.ad_value(316), p.p320), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p319), s.ad_value(341)), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(95, p.p321);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(96, A::scale(A::offset(A::scale(s.ad_value(314), p.p323), 1.0), p.p322), A::offset(A::scale(s.ad_value(316), p.p324), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(97, p.p325);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(98, p.p326);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(99, A::scale(A::offset(A::scale(s.ad_value(314), p.p328), 1.0), p.p327), A::offset(A::scale(s.ad_value(316), p.p329), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(100, A::scale(A::offset(A::scale(s.ad_value(314), p.p331), 1.0), p.p330), A::offset(A::scale(s.ad_value(316), p.p332), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(101, p.p333);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(102, p.p334);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar(103, p.p335, 318);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar_ad(104, (p.p336 * p.p236), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar_ad(105, (p.p337 * p.p237), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(106, p.p338);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(107, p.p339);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(108, p.p340);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(109, p.p339);
        }

        s.v[1033] = if (if self.param_given[341] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1033] != 0.0)) {
            s.store_scalar(109, p.p341);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(110, p.p340);
        }

        s.v[1034] = if (if self.param_given[342] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1034] != 0.0)) {
            s.store_scalar(110, p.p342);
        }

        if (s.v[1030] != 0.0) {
            s.copy_ad(111, 109);
        }

        s.v[1035] = if (if self.param_given[343] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1035] != 0.0)) {
            s.store_scalar(111, p.p343);
        }

        if (s.v[1030] != 0.0) {
            s.copy_ad(112, 110);
        }

        s.v[1036] = if (if self.param_given[344] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1036] != 0.0)) {
            s.store_scalar(112, p.p344);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(113, p.p345);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar_ad(114, (p.p346 * p.p236), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar_ad(115, (p.p347 * p.p237), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(116, p.p348);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(117, p.p349);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(118, p.p350);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(119, p.p351);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(120, p.p352);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(121, p.p353);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale_ad(122, A::mul(A::scale(s.ad_value(321), (8.8541878176e-12 * p.p209)), s.ad_value(320)), 1.0 / (p.p208));
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(129, 321, ((8.8541878176e-12 * p.p209) * (p.p236 * 1.0 / (p.p234))));
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(130, 321, ((8.8541878176e-12 * p.p209) * (p.p237 * 1.0 / (p.p235))));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(123, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p356), p.p355), p.p354), A::scale(s.ad_value(316), p.p357)), A::scale(s.ad_value(318), p.p358));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(124, A::add(A::offset(A::scale(s.ad_value(314), p.p360), p.p359), A::scale(s.ad_value(316), p.p361)), A::scale(s.ad_value(318), p.p362));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(36, p.p296);
        }

        s.v[1037] = if (if self.param_given[363] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1037] != 0.0)) {
            s.store_scalar(36, p.p363);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(37, p.p297);
        }

        s.v[1038] = if (if self.param_given[364] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1038] != 0.0)) {
            s.store_scalar(37, p.p364);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(38, p.p298);
        }

        s.v[1039] = if (if self.param_given[365] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1039] != 0.0)) {
            s.store_scalar(38, p.p365);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(39, p.p299);
        }

        s.v[1040] = if (if self.param_given[366] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1040] != 0.0)) {
            s.store_scalar(39, p.p366);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(40, p.p300);
        }

        s.v[1041] = if (if self.param_given[367] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1041] != 0.0)) {
            s.store_scalar(40, p.p367);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(125, A::mul(A::add(s.ad_value(36), A::mul(A::div(A::mul(s.ad_value(37), s.ad_value(340)), s.ad_value(339)), A::pow(s.ad_value(314), s.ad_value(38)))), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0)), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(41, p.p308);
        }

        s.v[1042] = if (if self.param_given[368] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1042] != 0.0)) {
            s.store_scalar(41, p.p368);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(42, p.p309);
        }

        s.v[1043] = if (if self.param_given[369] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1043] != 0.0)) {
            s.store_scalar(42, p.p369);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_ad_rhs(126, 41, A::offset(A::mul(s.ad_value(42), s.ad_value(314)), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(127, A::scale(A::powf(s.ad_value(314), p.p371), p.p370), A::offset(A::scale(s.ad_value(316), p.p372), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_powf(341, 314, p.p374);
        }

        if (s.v[1030] != 0.0) {
            s.store_div_ad(128, A::mul(A::scale(s.ad_value(341), p.p373), A::offset(A::scale(s.ad_value(316), p.p376), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p375), s.ad_value(341)), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(131, p.p377);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(132, p.p378);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(133, p.p379);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(134, 325, p.p380);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(135, 322, p.p381);
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
        if (s.v[1030] != 0.0) {
            s.store_scale(136, 322, p.p382);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(137, p.p383);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(138, p.p384);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(139, p.p385);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(140, p.p386);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(141, 326, p.p387);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(142, 326, p.p388);
        }

        if (s.v[1030] != 0.0) {
            s.store_sub_from_scalar_ad(1012, 1.0, A::div_from_scalar((2.0 * p.p395), s.ad_value(312)));
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(342, &{
                if (s.v[1012] > 0.001) {
                    s.ad_value(1012)
                } else {
                    A::constant(0.001)
                }
            });
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar_ad(343, 1.0, A::powf(s.ad_value(342), p.p396));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(143, p.p389);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad_lhs(144, A::mul(A::mul(A::scale(s.ad_value(69), p.p390), s.ad_value(69)), s.ad_value(316)), 316);
        }

        if (s.v[1030] != 0.0) {
            s.store_scaled_mul(145, 343, 318, p.p391);
        }

        if (s.v[1030] != 0.0) {
            s.store_scaled_mul(146, 343, 318, p.p392);
        }

        if (s.v[1030] != 0.0) {
            s.store_scaled_mul(147, 343, 318, p.p393);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(148, p.p394);
        }

        if (s.v[1030] != 0.0) {
            s.store_offset_scaled(344, 313, p.p398, (2.0 * p.p397));
        }

        if (s.v[1030] != 0.0) {
            s.store_div_from_scalar(345, 1e-6, 344);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul(346, 314, 345);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(149, p.p399);
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(150, A::add(A::offset(A::scale(s.ad_value(314), p.p401), p.p400), A::scale(s.ad_value(316), p.p402)), A::scale(s.ad_value(318), p.p403));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(151, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p406), p.p405), p.p404), A::scale(s.ad_value(316), p.p407)), A::scale(s.ad_value(318), p.p408));
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(152, A::mul(A::scale(A::offset(A::scale(A::powf(s.ad_value(314), p.p411), p.p410), 1.0), p.p409), A::offset(A::scale(s.ad_value(316), p.p412), 1.0)), A::offset(A::scale(s.ad_value(318), p.p413), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_offset_ad(153, A::scale(A::powf(s.ad_value(314), p.p416), p.p415), p.p414);
        }

        if (s.v[1030] != 0.0) {
            s.store_offset_ad(347, A::mul(A::div_from_scalar((p.p417 * p.p418), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(312)), 1.0 / (p.p418))))), 1.0);
        }

        if (s.v[1030] != 0.0) {
            s.store_ad(347, &{
                if (s.v[347] > 1e-15) {
                    s.ad_value(347)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(154, A::div(A::scale(s.ad_value(344), p.p258), A::mul(s.ad_value(347), s.ad_value(312))), A::offset(A::scale(s.ad_value(316), p.p419), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(155, A::add(A::offset(A::scale(s.ad_value(314), p.p421), p.p420), A::scale(s.ad_value(316), p.p422)), A::scale(s.ad_value(318), p.p423));
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(156, A::scale(A::powf(s.ad_value(314), p.p425), p.p424), A::offset(A::scale(s.ad_value(316), p.p426), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(157, p.p427);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(158, p.p428);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad(159, A::scale(A::powf(s.ad_value(314), p.p430), p.p429), A::offset(A::scale(s.ad_value(316), p.p431), 1.0));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(160, p.p433);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(161, p.p432);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(162, p.p434);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(163, 346, p.p435);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(164, 346, p.p436);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(165, 346, p.p437);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(166, p.p438);
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(348, A::add(A::offset(A::scale(s.ad_value(314), p.p815), p.p814), A::scale(s.ad_value(316), p.p816)), A::scale(s.ad_value(318), p.p817));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(349, A::add(A::offset(A::scale(s.ad_value(314), p.p819), p.p818), A::scale(s.ad_value(316), p.p820)), A::scale(s.ad_value(318), p.p821));
        }

        if (s.v[1030] != 0.0) {
            s.store_add_ad(167, A::add(A::div(A::scale(A::add(A::scale(s.ad_value(329), (0.3333333333333333 * 1.0 / (s.v[18]))), s.ad_value(330)), p.p442), A::scale(s.ad_value(328), s.v[18])), A::div_from_scalar((p.p440 + p.p441), A::mul(s.ad_value(329), s.ad_value(327)))), A::scale(s.ad_value(5), p.p439));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(168, (if (p.p444 > 0.0) { p.p444 } else { 0.0 }));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(169, (if (p.p445 > 0.0) { p.p445 } else { 0.0 }));
        }

        s.v[1044] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1044] != 0.0)) {
            s.copy_ad(169, 168);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(5), p.p12), 168);
        }

        if (s.v[1030] != 0.0) {
            s.store_mul_ad_lhs(171, A::scale(s.ad_value(5), p.p13), 169);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(172, 5, p.p447);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(173, 5, p.p446);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(174, 5, p.p448);
        }

        if (s.v[1030] != 0.0) {
            s.store_scale(175, 5, p.p449);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(176, p.p450);
        }

        s.v[1045] = if ((((if self.param_given[451] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[452] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[453] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[454] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1045] != 0.0)) {
            s.store_add_ad(44, A::add(A::offset(A::scale(s.ad_value(314), p.p452), p.p451), A::scale(s.ad_value(316), p.p453)), A::scale(s.ad_value(318), p.p454));
        }

        s.v[1046] = if ((((if self.param_given[455] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[456] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[457] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[458] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_add_ad(45, A::add(A::offset(A::scale(s.ad_value(314), p.p456), p.p455), A::scale(s.ad_value(316), p.p457)), A::scale(s.ad_value(318), p.p458));
        }

        s.v[1047] = if ((((if self.param_given[459] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[460] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[461] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[462] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1047] != 0.0)) {
            s.store_add_ad(49, A::add(A::offset(A::scale(s.ad_value(314), p.p460), p.p459), A::scale(s.ad_value(316), p.p461)), A::scale(s.ad_value(318), p.p462));
        }

        s.v[1048] = if ((((if self.param_given[463] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[464] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[465] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[466] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1048] != 0.0)) {
            s.store_add_ad(50, A::add(A::offset(A::scale(s.ad_value(314), p.p464), p.p463), A::scale(s.ad_value(316), p.p465)), A::scale(s.ad_value(318), p.p466));
        }

        s.v[1049] = if ((((if self.param_given[467] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[468] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[469] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[470] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1049] != 0.0)) {
            s.store_add_ad(51, A::add(A::offset(A::scale(s.ad_value(314), p.p468), p.p467), A::scale(s.ad_value(316), p.p469)), A::scale(s.ad_value(318), p.p470));
        }

        s.v[1050] = if ((((if self.param_given[471] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[472] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[473] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[474] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1050] != 0.0)) {
            s.store_add_ad(53, A::add(A::offset(A::scale(s.ad_value(314), p.p472), p.p471), A::scale(s.ad_value(316), p.p473)), A::scale(s.ad_value(318), p.p474));
        }

        s.v[1051] = if ((((if self.param_given[475] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[476] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[477] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[478] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_add_ad(54, A::add(A::offset(A::scale(s.ad_value(314), p.p476), p.p475), A::scale(s.ad_value(316), p.p477)), A::scale(s.ad_value(318), p.p478));
        }

        s.v[1052] = if ((((if self.param_given[479] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[480] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[481] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[482] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1052] != 0.0)) {
            s.store_add_ad(61, A::add(A::offset(A::scale(s.ad_value(314), p.p480), p.p479), A::scale(s.ad_value(316), p.p481)), A::scale(s.ad_value(318), p.p482));
        }

        s.v[1053] = if ((((if self.param_given[483] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[484] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[485] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[486] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1053] != 0.0)) {
            s.store_add_ad(62, A::add(A::offset(A::scale(s.ad_value(314), p.p484), p.p483), A::scale(s.ad_value(316), p.p485)), A::scale(s.ad_value(318), p.p486));
        }

        s.v[1054] = if ((((if self.param_given[487] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[488] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[489] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[490] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1054] != 0.0)) {
            s.store_add_ad(55, A::add(A::offset(A::scale(s.ad_value(314), p.p488), p.p487), A::scale(s.ad_value(316), p.p489)), A::scale(s.ad_value(318), p.p490));
        }

        s.v[1055] = if ((((if self.param_given[495] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[496] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[497] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[498] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1055] != 0.0)) {
            s.store_add_ad(56, A::add(A::offset(A::scale(s.ad_value(314), p.p496), p.p495), A::scale(s.ad_value(316), p.p497)), A::scale(s.ad_value(318), p.p498));
        }

        s.v[1056] = if ((((if self.param_given[491] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[492] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[493] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[494] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1056] != 0.0)) {
            s.store_add_ad(57, A::add(A::offset(A::scale(s.ad_value(314), p.p492), p.p491), A::scale(s.ad_value(316), p.p493)), A::scale(s.ad_value(318), p.p494));
        }

        s.v[1057] = if ((((if self.param_given[499] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[500] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[501] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[502] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1057] != 0.0)) {
            s.store_add_ad(58, A::add(A::offset(A::scale(s.ad_value(314), p.p500), p.p499), A::scale(s.ad_value(316), p.p501)), A::scale(s.ad_value(318), p.p502));
        }

        s.v[1058] = if ((((if self.param_given[503] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[504] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[505] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[506] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1058] != 0.0)) {
            s.store_mul_ad_rhs(66, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p504), p.p503), A::scale(s.ad_value(316), p.p505)), A::scale(s.ad_value(318), p.p506)));
        }

        s.v[1059] = if ((((if self.param_given[511] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[512] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[513] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[514] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1059] != 0.0)) {
            s.store_add_ad(67, A::add(A::offset(A::scale(s.ad_value(314), p.p512), p.p511), A::scale(s.ad_value(316), p.p513)), A::scale(s.ad_value(318), p.p514));
        }

        s.v[1060] = if ((((if self.param_given[507] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[508] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[509] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[510] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1060] != 0.0)) {
            s.store_add_ad(68, A::add(A::offset(A::scale(s.ad_value(314), p.p508), p.p507), A::scale(s.ad_value(316), p.p509)), A::scale(s.ad_value(318), p.p510));
        }

        s.v[1061] = if ((((if self.param_given[515] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[516] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[517] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[518] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1061] != 0.0)) {
            s.store_mul_ad_rhs(63, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p516), p.p515), A::scale(s.ad_value(316), p.p517)), A::scale(s.ad_value(318), p.p518)));
        }

        s.v[1062] = if ((((if self.param_given[523] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[524] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[525] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[526] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1062] != 0.0)) {
            s.store_add_ad(64, A::add(A::offset(A::scale(s.ad_value(314), p.p524), p.p523), A::scale(s.ad_value(316), p.p525)), A::scale(s.ad_value(318), p.p526));
        }

        s.v[1063] = if ((((if self.param_given[519] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[520] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[521] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[522] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1063] != 0.0)) {
            s.store_add_ad(65, A::add(A::offset(A::scale(s.ad_value(314), p.p520), p.p519), A::scale(s.ad_value(316), p.p521)), A::scale(s.ad_value(318), p.p522));
        }

        s.v[1064] = if ((((if self.param_given[527] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[528] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[529] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[530] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1064] != 0.0)) {
            s.store_mul_ad(69, A::div(s.ad_value(313), s.ad_value(312)), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p528), p.p527), A::scale(s.ad_value(316), p.p529)), A::scale(s.ad_value(318), p.p530)));
        }

        s.v[1065] = if ((((if self.param_given[531] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[532] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[533] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[534] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1065] != 0.0)) {
            s.store_add_ad(70, A::add(A::offset(A::scale(s.ad_value(314), p.p532), p.p531), A::scale(s.ad_value(316), p.p533)), A::scale(s.ad_value(318), p.p534));
        }

        s.v[1066] = if ((((if self.param_given[535] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[536] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[537] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[538] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1066] != 0.0)) {
            s.store_add_ad(71, A::add(A::offset(A::scale(s.ad_value(314), p.p536), p.p535), A::scale(s.ad_value(316), p.p537)), A::scale(s.ad_value(318), p.p538));
        }

        s.v[1067] = if ((((if self.param_given[539] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[540] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[541] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[542] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1067] != 0.0)) {
            s.store_add_ad(73, A::add(A::offset(A::scale(s.ad_value(314), p.p540), p.p539), A::scale(s.ad_value(316), p.p541)), A::scale(s.ad_value(318), p.p542));
        }

        s.v[1068] = if ((((if self.param_given[543] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[544] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[545] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[546] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1068] != 0.0)) {
            s.store_add_ad(75, A::add(A::offset(A::scale(s.ad_value(314), p.p544), p.p543), A::scale(s.ad_value(316), p.p545)), A::scale(s.ad_value(318), p.p546));
        }

        s.v[1069] = if ((((if self.param_given[547] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[548] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[549] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[550] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1069] != 0.0)) {
            s.store_add_ad(77, A::add(A::offset(A::scale(s.ad_value(314), p.p548), p.p547), A::scale(s.ad_value(316), p.p549)), A::scale(s.ad_value(318), p.p550));
        }

        s.v[1070] = if ((((if self.param_given[551] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[552] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[553] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[554] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1070] != 0.0)) {
            s.store_add_ad(79, A::add(A::offset(A::scale(s.ad_value(314), p.p552), p.p551), A::scale(s.ad_value(316), p.p553)), A::scale(s.ad_value(318), p.p554));
        }

        s.v[1071] = if ((((if self.param_given[555] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[556] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[557] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[558] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1071] != 0.0)) {
            s.store_mul_ad_rhs(82, 316, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p556), p.p555), A::scale(s.ad_value(316), p.p557)), A::scale(s.ad_value(318), p.p558)));
        }

        s.v[1072] = if ((((if self.param_given[559] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[560] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[561] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[562] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1072] != 0.0)) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(314), p.p560), p.p559), A::scale(s.ad_value(316), p.p561)), A::scale(s.ad_value(318), p.p562));
        }

        s.v[1073] = if ((((if self.param_given[563] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[564] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[565] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[566] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1073] != 0.0)) {
            s.store_add_ad(84, A::add(A::offset(A::scale(s.ad_value(314), p.p564), p.p563), A::scale(s.ad_value(316), p.p565)), A::scale(s.ad_value(318), p.p566));
        }

        s.v[1074] = if ((((if self.param_given[567] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[568] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[569] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[570] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1074] != 0.0)) {
            s.store_add_ad(85, A::add(A::offset(A::scale(s.ad_value(314), p.p568), p.p567), A::scale(s.ad_value(316), p.p569)), A::scale(s.ad_value(318), p.p570));
        }

        s.v[1075] = if ((((if self.param_given[571] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[572] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[573] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[574] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1075] != 0.0)) {
            s.store_mul_ad_rhs(86, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p572), p.p571), A::scale(s.ad_value(316), p.p573)), A::scale(s.ad_value(318), p.p574)));
        }

        s.v[1076] = if ((((if self.param_given[575] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[576] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[577] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[578] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1076] != 0.0)) {
            s.store_add_ad(87, A::add(A::offset(A::scale(s.ad_value(314), p.p576), p.p575), A::scale(s.ad_value(316), p.p577)), A::scale(s.ad_value(318), p.p578));
        }

        s.v[1077] = if ((((if self.param_given[579] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[580] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[581] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[582] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1077] != 0.0)) {
            s.store_add_ad(88, A::add(A::offset(A::scale(s.ad_value(314), p.p580), p.p579), A::scale(s.ad_value(316), p.p581)), A::scale(s.ad_value(318), p.p582));
        }

        s.v[1078] = if ((((if self.param_given[583] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[584] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[585] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[586] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1078] != 0.0)) {
            s.store_add_ad(89, A::add(A::offset(A::scale(s.ad_value(314), p.p584), p.p583), A::scale(s.ad_value(316), p.p585)), A::scale(s.ad_value(318), p.p586));
        }

        s.v[1079] = if ((((if self.param_given[587] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[588] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[589] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[590] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1079] != 0.0)) {
            s.store_add_ad(91, A::add(A::offset(A::scale(s.ad_value(314), p.p588), p.p587), A::scale(s.ad_value(316), p.p589)), A::scale(s.ad_value(318), p.p590));
        }

        s.v[1080] = if ((((if self.param_given[591] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[592] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[593] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[594] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1080] != 0.0)) {
            s.store_mul_ad_rhs(92, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p592), p.p591), A::scale(s.ad_value(316), p.p593)), A::scale(s.ad_value(318), p.p594)));
        }

        s.v[1081] = if ((((if self.param_given[595] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[596] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[597] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[598] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1081] != 0.0)) {
            s.store_add_ad(93, A::add(A::offset(A::scale(s.ad_value(314), p.p596), p.p595), A::scale(s.ad_value(316), p.p597)), A::scale(s.ad_value(318), p.p598));
        }

        s.v[1082] = if ((((if self.param_given[599] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[600] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[601] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[602] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_add_ad(94, A::add(A::offset(A::scale(s.ad_value(314), p.p600), p.p599), A::scale(s.ad_value(316), p.p601)), A::scale(s.ad_value(318), p.p602));
        }

        s.v[1083] = if ((((if self.param_given[603] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[604] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[605] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[606] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1083] != 0.0)) {
            s.store_add_ad(96, A::add(A::offset(A::scale(s.ad_value(314), p.p604), p.p603), A::scale(s.ad_value(316), p.p605)), A::scale(s.ad_value(318), p.p606));
        }

        s.v[1084] = if ((((if self.param_given[607] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[608] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[609] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[610] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_add_ad(98, A::add(A::offset(A::scale(s.ad_value(314), p.p608), p.p607), A::scale(s.ad_value(316), p.p609)), A::scale(s.ad_value(318), p.p610));
        }

        s.v[1085] = if ((((if self.param_given[611] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[612] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[613] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[614] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1085] != 0.0)) {
            s.store_add_ad(99, A::add(A::offset(A::scale(s.ad_value(314), p.p612), p.p611), A::scale(s.ad_value(316), p.p613)), A::scale(s.ad_value(318), p.p614));
        }

        s.v[1086] = if ((((if self.param_given[615] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[616] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[617] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[618] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1086] != 0.0)) {
            s.store_add_ad(100, A::add(A::offset(A::scale(s.ad_value(314), p.p616), p.p615), A::scale(s.ad_value(316), p.p617)), A::scale(s.ad_value(318), p.p618));
        }

        s.v[1087] = if ((((if self.param_given[619] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[620] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[621] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[622] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_mul_ad_rhs(103, 319, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p620), p.p619), A::scale(s.ad_value(316), p.p621)), A::scale(s.ad_value(318), p.p622)));
        }

        s.v[1088] = if ((((if self.param_given[623] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[624] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[625] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[626] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1088] != 0.0)) {
            s.store_mul_ad_rhs(104, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p624), p.p623), A::scale(s.ad_value(316), p.p625)), A::scale(s.ad_value(318), p.p626)));
        }

        s.v[1089] = if ((((if self.param_given[627] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[628] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[629] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[630] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_mul_ad_rhs(105, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p628), p.p627), A::scale(s.ad_value(316), p.p629)), A::scale(s.ad_value(318), p.p630)));
        }

        s.v[1090] = if ((((if self.param_given[631] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[632] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[633] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[634] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1090] != 0.0)) {
            s.store_add_ad(106, A::add(A::offset(A::scale(s.ad_value(314), p.p632), p.p631), A::scale(s.ad_value(316), p.p633)), A::scale(s.ad_value(318), p.p634));
        }

        s.v[1091] = if ((((if self.param_given[635] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[636] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[637] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[638] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1091] != 0.0)) {
            s.store_mul_ad_rhs(114, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p636), p.p635), A::scale(s.ad_value(316), p.p637)), A::scale(s.ad_value(318), p.p638)));
        }

        s.v[1092] = if ((((if self.param_given[639] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[640] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[641] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[642] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_mul_ad_rhs(115, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p640), p.p639), A::scale(s.ad_value(316), p.p641)), A::scale(s.ad_value(318), p.p642)));
        }

        s.v[1093] = if ((((if self.param_given[643] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[644] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[645] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[646] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

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
        if ((s.v[1030] != 0.0) && (s.v[1093] != 0.0)) {
            s.store_add_ad(118, A::add(A::offset(A::scale(s.ad_value(314), p.p644), p.p643), A::scale(s.ad_value(316), p.p645)), A::scale(s.ad_value(318), p.p646));
        }

        s.v[1094] = if ((((if self.param_given[647] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[648] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[649] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[650] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1094] != 0.0)) {
            s.store_add_ad(119, A::add(A::offset(A::scale(s.ad_value(314), p.p648), p.p647), A::scale(s.ad_value(316), p.p649)), A::scale(s.ad_value(318), p.p650));
        }

        s.v[1095] = if ((((if self.param_given[651] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[652] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[653] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[654] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1095] != 0.0)) {
            s.store_mul_ad(122, A::scale(A::mul(s.ad_value(322), s.ad_value(320)), 1000000.0), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p652), p.p651), A::scale(s.ad_value(316), p.p653)), A::scale(s.ad_value(318), p.p654)));
        }

        s.v[1096] = if ((((if self.param_given[655] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[656] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[657] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[658] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1096] != 0.0)) {
            s.store_add_ad(123, A::add(A::offset(A::scale(s.ad_value(314), p.p656), p.p655), A::scale(s.ad_value(316), p.p657)), A::scale(s.ad_value(318), p.p658));
        }

        s.v[1097] = if ((((if self.param_given[659] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[660] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[661] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[662] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_add_ad(124, A::add(A::offset(A::scale(s.ad_value(314), p.p660), p.p659), A::scale(s.ad_value(316), p.p661)), A::scale(s.ad_value(318), p.p662));
        }

        s.v[1098] = if ((((((((if self.param_given[663] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[664] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[571] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[572] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[573] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[574] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_scalar(32, p.p571);
        }

        s.v[1099] = if (if self.param_given[663] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) && (s.v[1099] != 0.0)) {
            s.store_scalar(32, p.p663);
        }

        if ((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_scalar(33, p.p572);
        }

        s.v[1100] = if (if self.param_given[664] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) && (s.v[1100] != 0.0)) {
            s.store_scalar(33, p.p664);
        }

        if ((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_scalar(34, p.p573);
        }

        s.v[1101] = if (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) && (s.v[1101] != 0.0)) {
            s.store_scalar(34, p.p665);
        }

        if ((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_scalar(35, p.p574);
        }

        s.v[1102] = if (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) && (s.v[1102] != 0.0)) {
            s.store_scalar(35, p.p666);
        }

        if ((s.v[1030] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_mul_ad_rhs(125, 314, A::add(A::add(A::add(s.ad_value(32), A::mul(s.ad_value(33), s.ad_value(314))), A::mul(s.ad_value(34), s.ad_value(316))), A::mul(s.ad_value(35), s.ad_value(318))));
        }

        s.v[1103] = if ((((((((if self.param_given[667] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[668] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[669] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[670] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[587] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[588] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[589] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[590] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_scalar(32, p.p587);
        }

        s.v[1104] = if (if self.param_given[667] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) {
            s.store_scalar(32, p.p667);
        }

        if ((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_scalar(33, p.p588);
        }

        s.v[1105] = if (if self.param_given[668] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) && (s.v[1105] != 0.0)) {
            s.store_scalar(33, p.p668);
        }

        if ((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_scalar(34, p.p589);
        }

        s.v[1106] = if (if self.param_given[669] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) && (s.v[1106] != 0.0)) {
            s.store_scalar(34, p.p669);
        }

        if ((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_scalar(35, p.p590);
        }

        s.v[1107] = if (if self.param_given[670] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) && (s.v[1107] != 0.0)) {
            s.store_scalar(35, p.p670);
        }

        if ((s.v[1030] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_add_ad(126, A::add(A::add(s.ad_value(32), A::mul(s.ad_value(33), s.ad_value(314))), A::mul(s.ad_value(34), s.ad_value(316))), A::mul(s.ad_value(35), s.ad_value(318)));
        }

        s.v[1108] = if ((((if self.param_given[671] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[672] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[673] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[674] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_mul_ad_rhs(127, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p672), p.p671), A::scale(s.ad_value(316), p.p673)), A::scale(s.ad_value(318), p.p674)));
        }

        s.v[1109] = if ((((if self.param_given[675] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[676] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[677] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[678] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1109] != 0.0)) {
            s.store_mul_ad_rhs(128, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p676), p.p675), A::scale(s.ad_value(316), p.p677)), A::scale(s.ad_value(318), p.p678)));
        }

        s.v[1110] = if ((((if self.param_given[679] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[680] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[681] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[682] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1110] != 0.0)) {
            s.store_mul_ad_rhs(129, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p680), p.p679), A::scale(s.ad_value(316), p.p681)), A::scale(s.ad_value(318), p.p682)));
        }

        s.v[1111] = if ((((if self.param_given[683] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[684] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[685] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[686] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1111] != 0.0)) {
            s.store_mul_ad_rhs(130, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p684), p.p683), A::scale(s.ad_value(316), p.p685)), A::scale(s.ad_value(318), p.p686)));
        }

        s.v[1112] = if ((((if self.param_given[687] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[688] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[689] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[690] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1112] != 0.0)) {
            s.store_mul_ad_rhs(134, 325, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p688), p.p687), A::scale(s.ad_value(316), p.p689)), A::scale(s.ad_value(318), p.p690)));
        }

        s.v[1113] = if ((((if self.param_given[691] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[692] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[693] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[694] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1113] != 0.0)) {
            s.store_mul_ad_rhs(135, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p692), p.p691), A::scale(s.ad_value(316), p.p693)), A::scale(s.ad_value(318), p.p694)));
        }

        s.v[1114] = if ((((if self.param_given[695] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[696] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[697] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[698] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1114] != 0.0)) {
            s.store_mul_ad_rhs(136, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p696), p.p695), A::scale(s.ad_value(316), p.p697)), A::scale(s.ad_value(318), p.p698)));
        }

        s.v[1115] = if ((((if self.param_given[699] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[700] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[701] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[702] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1115] != 0.0)) {
            s.store_mul_ad_rhs(141, 326, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p700), p.p699), A::scale(s.ad_value(316), p.p701)), A::scale(s.ad_value(318), p.p702)));
        }

        s.v[1116] = if ((((if self.param_given[703] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[704] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[705] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[706] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1116] != 0.0)) {
            s.store_mul_ad_rhs(142, 326, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p704), p.p703), A::scale(s.ad_value(316), p.p705)), A::scale(s.ad_value(318), p.p706)));
        }

        s.v[1117] = if ((((if self.param_given[707] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[708] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[709] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[710] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1117] != 0.0)) {
            s.store_mul_ad_rhs(144, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p708), p.p707), A::scale(s.ad_value(316), p.p709)), A::scale(s.ad_value(318), p.p710)));
        }

        s.v[1118] = if ((((if self.param_given[711] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[712] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[713] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[714] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1118] != 0.0)) {
            s.store_mul_ad_rhs(145, 318, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p712), p.p711), A::scale(s.ad_value(316), p.p713)), A::scale(s.ad_value(318), p.p714)));
        }

        s.v[1119] = if ((((if self.param_given[715] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[716] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[717] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[718] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1119] != 0.0)) {
            s.store_mul_ad_rhs(146, 318, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p716), p.p715), A::scale(s.ad_value(316), p.p717)), A::scale(s.ad_value(318), p.p718)));
        }

        s.v[1120] = if ((((if self.param_given[719] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[720] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[721] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[722] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1120] != 0.0)) {
            s.store_mul_ad_rhs(147, 318, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p720), p.p719), A::scale(s.ad_value(316), p.p721)), A::scale(s.ad_value(318), p.p722)));
        }

        s.v[1121] = if ((((if self.param_given[723] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[724] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[725] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[726] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1121] != 0.0)) {
            s.store_add_ad(149, A::add(A::offset(A::scale(s.ad_value(314), p.p724), p.p723), A::scale(s.ad_value(316), p.p725)), A::scale(s.ad_value(318), p.p726));
        }

        s.v[1122] = if ((((if self.param_given[727] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[728] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[729] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[730] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1122] != 0.0)) {
            s.store_add_ad(150, A::add(A::offset(A::scale(s.ad_value(314), p.p728), p.p727), A::scale(s.ad_value(316), p.p729)), A::scale(s.ad_value(318), p.p730));
        }

        s.v[1123] = if ((((if self.param_given[731] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[732] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[733] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[734] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1123] != 0.0)) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(314), p.p732), p.p731), A::scale(s.ad_value(316), p.p733)), A::scale(s.ad_value(318), p.p734));
        }

        s.v[1124] = if ((((if self.param_given[735] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[736] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[737] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[738] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1124] != 0.0)) {
            s.store_add_ad(152, A::add(A::offset(A::scale(s.ad_value(314), p.p736), p.p735), A::scale(s.ad_value(316), p.p737)), A::scale(s.ad_value(318), p.p738));
        }

        s.v[1125] = if ((((if self.param_given[739] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[740] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[741] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[742] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1125] != 0.0)) {
            s.store_add_ad(153, A::add(A::offset(A::scale(s.ad_value(314), p.p740), p.p739), A::scale(s.ad_value(316), p.p741)), A::scale(s.ad_value(318), p.p742));
        }

        s.v[1126] = if ((((if self.param_given[743] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[744] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[745] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[746] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1126] != 0.0)) {
            s.store_mul_ad(154, A::div(s.ad_value(344), s.ad_value(312)), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p744), p.p743), A::scale(s.ad_value(316), p.p745)), A::scale(s.ad_value(318), p.p746)));
        }

        s.v[1127] = if ((((if self.param_given[747] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[748] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[749] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[750] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add_ad(155, A::add(A::offset(A::scale(s.ad_value(314), p.p748), p.p747), A::scale(s.ad_value(316), p.p749)), A::scale(s.ad_value(318), p.p750));
        }

        s.v[1128] = if ((((if self.param_given[751] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[752] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[753] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[754] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(156, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p752), p.p751), A::scale(s.ad_value(316), p.p753)), A::scale(s.ad_value(318), p.p754)));
        }

        s.v[1129] = if ((((if self.param_given[755] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[756] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[757] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[758] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1129] != 0.0)) {
            s.store_add_ad(157, A::add(A::offset(A::scale(s.ad_value(314), p.p756), p.p755), A::scale(s.ad_value(316), p.p757)), A::scale(s.ad_value(318), p.p758));
        }

        s.v[1130] = if ((((if self.param_given[759] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[760] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[761] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[762] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1130] != 0.0)) {
            s.store_add_ad(158, A::add(A::offset(A::scale(s.ad_value(314), p.p760), p.p759), A::scale(s.ad_value(316), p.p761)), A::scale(s.ad_value(318), p.p762));
        }

        s.v[1131] = if ((((if self.param_given[763] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[764] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[765] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[766] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1131] != 0.0)) {
            s.store_mul_ad_rhs(159, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p764), p.p763), A::scale(s.ad_value(316), p.p765)), A::scale(s.ad_value(318), p.p766)));
        }

        s.v[1132] = if ((((if self.param_given[771] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[772] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[773] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[774] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1132] != 0.0)) {
            s.store_add_ad(160, A::add(A::offset(A::scale(s.ad_value(314), p.p772), p.p771), A::scale(s.ad_value(316), p.p773)), A::scale(s.ad_value(318), p.p774));
        }

        s.v[1133] = if ((((if self.param_given[767] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[768] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[769] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[770] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1133] != 0.0)) {
            s.store_add_ad(161, A::add(A::offset(A::scale(s.ad_value(314), p.p768), p.p767), A::scale(s.ad_value(316), p.p769)), A::scale(s.ad_value(318), p.p770));
        }

        s.v[1134] = if ((((if self.param_given[775] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[776] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[777] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[778] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1134] != 0.0)) {
            s.store_mul_ad_rhs(163, 346, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p776), p.p775), A::scale(s.ad_value(316), p.p777)), A::scale(s.ad_value(318), p.p778)));
        }

        s.v[1135] = if ((((if self.param_given[779] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[780] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[781] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[782] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1135] != 0.0)) {
            s.store_mul_ad_rhs(164, 346, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p780), p.p779), A::scale(s.ad_value(316), p.p781)), A::scale(s.ad_value(318), p.p782)));
        }

        s.v[1136] = if ((((if self.param_given[783] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[784] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[785] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[786] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1136] != 0.0)) {
            s.store_mul_ad_rhs(165, 346, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p784), p.p783), A::scale(s.ad_value(316), p.p785)), A::scale(s.ad_value(318), p.p786)));
        }

        s.v[1137] = if ((((if self.param_given[787] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[788] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[789] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[790] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1137] != 0.0)) {
            s.store_add_ad(176, A::add(A::offset(A::scale(s.ad_value(314), p.p788), p.p787), A::scale(s.ad_value(316), p.p789)), A::scale(s.ad_value(318), p.p790));
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(1019, 0.0);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(1020, 0.0);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(1018, 0.0);
        }

        if (s.v[1030] != 0.0) {
            s.store_scalar(43, p.p795);
        }

        s.v[1138] = if (if self.param_given[796] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1030] != 0.0) && (s.v[1138] != 0.0)) {
            s.store_scalar(43, p.p796);
        }

        s.v[1139] = if (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0)))) { 1.0 } else { 0.0 };

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (s.v[5] - 0.5);
            let assign9340_cond_e9224: f64 = if (((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) && (s.v[1018] < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
                s.store_add_ad_rhs(1019, 1019, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1018), (s.v[11] + s.v[7])), (s.v[9] + (0.5 * s.v[7])))));
            }
            if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
                s.store_add_ad_rhs(1020, 1020, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1018), (s.v[11] + s.v[7])), (s.v[10] + (0.5 * s.v[7])))));
            }
            if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
                s.store_offset(1018, 1018, 1.0);
            }
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_mul(1003, 1019, 6);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_mul(1004, 1020, 6);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_scalar(1005, (1.0 / (p.p791 + (0.5 * s.v[7]))));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_scalar(1006, (1.0 / (p.p792 + (0.5 * s.v[7]))));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_ad(1016, &{
                if ((s.v[7] + s.v[310]) > 1e-9) {
                    A::offset(s.ad_value(310), s.v[7])
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_ad(1017, &{
                if (((s.v[8] + s.v[311]) + p.p793) > 1e-9) {
                    A::offset(A::add(s.ad_value(8), s.ad_value(311)), p.p793)
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_from_scalar_ad(1014, 1.0, A::powf(s.ad_value(1016), p.p801));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_from_scalar_ad(1015, 1.0, A::powf(s.ad_value(1017), p.p802));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_scale_ad(1007, A::add(A::add(A::offset(A::scale(s.ad_value(1014), p.p798), 1.0), A::scale(s.ad_value(1015), p.p799)), A::mul(A::scale(s.ad_value(1014), p.p800), s.ad_value(1015))), (1.0 + (p.p797 * (s.v[352] - 1.0))));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_ad_lhs(1008, A::scale(A::add(s.ad_value(1003), s.ad_value(1004)), p.p794), 1007);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_ad_lhs(1009, A::scale(A::add(s.ad_value(1005), s.ad_value(1006)), p.p794), 1007);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_from_scalar_ad(1014, 1.0, A::powf(s.ad_value(1016), p.p807));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_from_scalar_ad(1015, 1.0, A::powf(s.ad_value(1017), p.p808));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_add_ad(1010, A::add(A::offset(A::scale(s.ad_value(1014), p.p804), 1.0), A::scale(s.ad_value(1015), p.p805)), A::mul(A::scale(s.ad_value(1014), p.p806), s.ad_value(1015)));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_sub_ad_lhs(1012, A::sub(A::add(s.ad_value(1003), s.ad_value(1004)), s.ad_value(1005)), 1006);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_ad(1013, A::offset(s.ad_value(1008), 1.0), A::offset(s.ad_value(1009), 1.0));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_mul(69, 69, 1013);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_ad(86, A::mul(A::mul(s.ad_value(86), s.ad_value(1013)), A::offset(A::scale(s.ad_value(1009), p.p795), 1.0)), A::offset(A::scale(s.ad_value(1008), p.p795), 1.0));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_ad(125, A::mul(A::mul(s.ad_value(125), s.ad_value(1013)), A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0)), A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_mul(154, 154, 1013);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_ad_lhs(1013, A::scale(s.ad_value(1012), p.p803), 1010);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_add(44, 44, 1013);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_add(149, 149, 1013);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_div_ad(1013, A::scale(s.ad_value(1012), p.p809), A::powf(s.ad_value(1010), p.p810));
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_add(66, 66, 1013);
        }

        if ((s.v[1030] != 0.0) && (s.v[1139] != 0.0)) {
            s.store_add(159, 159, 1013);
        }

        s.v[1140] = if ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1141] = if (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) && (s.v[1141] != 0.0)) {
            s.store_offset(1012, 8, s.v[12]);
        }

        if (((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) && (s.v[1141] != 0.0)) {
            s.store_scalar(1013, (1.0 / p.p811));
        }

        if (((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) && (s.v[1141] != 0.0)) {
            s.store_div_from_scalar_ad(15, (p.p811 * p.p811), A::scale(s.ad_value(1012), s.v[12]));
        }

        if (((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) && (s.v[1141] != 0.0)) {
            s.store_div_ad_lhs(16, A::sub(A::scale(A::exp(A::scale(s.ad_value(1013), ((-10.0) * s.v[12]))), ((0.1 * s.v[12]) + (0.01 * p.p811))), A::mul(A::offset(A::scale(s.ad_value(1012), 0.1), (0.01 * p.p811)), A::exp(A::mul(A::scale(s.ad_value(1012), (-10.0)), s.ad_value(1013))))), 8);
        }

        if (((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) && (s.v[1141] != 0.0)) {
            s.store_div_ad_lhs(17, A::sub(A::scale(A::exp(A::scale(s.ad_value(1013), ((-20.0) * s.v[12]))), ((0.05 * s.v[12]) + (0.0025 * p.p811))), A::mul(A::offset(A::scale(s.ad_value(1012), 0.05), (0.0025 * p.p811)), A::exp(A::mul(A::scale(s.ad_value(1012), (-20.0)), s.ad_value(1013))))), 8);
        }

        if ((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) {
            s.store_add_ad(1012, A::add(s.ad_value(15), A::scale(s.ad_value(16), p.p812)), A::scale(s.ad_value(17), p.p813));
        }

        if ((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) {
            s.store_add_ad_rhs(44, 44, A::mul(s.ad_value(348), s.ad_value(1012)));
        }

        if ((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) {
            s.store_mul_ad_rhs(69, 69, A::offset(A::mul(s.ad_value(349), s.ad_value(1012)), 1.0));
        }

        if ((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) {
            s.store_add_ad_rhs(149, 149, A::mul(s.ad_value(348), s.ad_value(1012)));
        }

        if ((s.v[1030] != 0.0) && (s.v[1140] != 0.0)) {
            s.store_mul_ad_rhs(154, 154, A::offset(A::mul(s.ad_value(349), s.ad_value(1012)), 1.0));
        }

        s.copy_ad(177, 44);

        s.copy_ad(178, 45);

        s.copy_ad(179, 46);

        s.copy_ad(181, 47);

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
        s.copy_ad(182, 48);

        if (s.v[49] > 1e20) {
            s.store_ad(183, &{
                if (s.v[49] < 1e26) {
                    s.ad_value(49)
                } else {
                    A::constant(1e26)
                }
            });
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
            s.store_ad(194, &{
                if (s.v[61] < 1e27) {
                    s.ad_value(61)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(194, 1e23);
        }

        if (s.v[62] > 1e23) {
            s.store_ad(195, &{
                if (s.v[62] < 1e27) {
                    s.ad_value(62)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(195, 1e23);
        }

        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[57] > 0.0) {
            s.store_ad(191, &{
                if (s.v[57] < 0.5) {
                    s.ad_value(57)
                } else {
                    A::constant(0.5)
                }
            });
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[56] > 0.0) {
            s.store_ad(190, &{
                if (s.v[56] < 1.0) {
                    s.ad_value(56)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(198, &{
                if (s.v[68] < 1.0) {
                    s.ad_value(68)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(200, &{
                if (s.v[65] < 1.0) {
                    s.ad_value(65)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(217, &{
                if (s.v[84] < 1.0) {
                    s.ad_value(84)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(221, &{
                if (s.v[88] < 1.0) {
                    s.ad_value(88)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(285, &{
                if (s.v[152] < 1e26) {
                    s.ad_value(152)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(285, 1e20);
        }

        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
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
            s.store_ad(290, &{
                if (s.v[157] < 1.0) {
                    s.ad_value(157)
                } else {
                    A::constant(1.0)
                }
            });
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
            s.store_ad(294, &{
                if (s.v[161] < 1.0) {
                    s.ad_value(161)
                } else {
                    A::constant(1.0)
                }
            });
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

        s.v[1142] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1142] != 0.0) {
            s.copy_ad(193, 192);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(195, 194);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(248, 247);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(250, 249);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(252, 251);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(254, 253);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(238, 237);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(244, 242);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(245, 243);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(263, 262);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(265, 264);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(269, 268);
        }

        if (s.v[1142] != 0.0) {
            s.copy_ad(275, 274);
        }

        s.store_scale(768, 182, 8.8541878176e-12);

        s.store_div(769, 768, 181);

        s.store_square(770, 181);

        s.store_scale(771, 769, 6.241449993689894e18);

        s.store_mul(772, 257, 183);

        if (s.v[772] > 1e20) {
            s.store_ad(772, &{
                if (s.v[772] < 1e26) {
                    s.ad_value(772)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(772, 1e20);
        }

        s.v[773] = 0.0;

        s.v[1143] = if (p.p52 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1143] != 0.0) {
            s.store_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p.p52));
        }

        s.v[1144] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1143] != 0.0) && (s.v[1144] != 0.0)) {
            s.store_scale(773, 773, (7.448711 / 5.951993));
        }

        s.store_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));

        s.store_scale(775, 214, 0.5);

        s.v[776] = 0.5;

        s.v[1145] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[1145] != 0.0) {
            s.store_scale(775, 214, 0.3333333333333333);
        }

        if (s.v[1145] != 0.0) {
            s.store_scalar(776, 0.3333333333333333);
        }

        s.store_offset_ad(1011, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0)), (-1.0));

        s.store_ad(777, &A::div(A::mul(A::offset(s.ad_value(1011), (-1.0)), A::offset(s.ad_value(1011), (-1.0))), {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_offset_ad(1011, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0)), (-1.0));

        s.store_ad(778, &A::div(A::mul(A::offset(s.ad_value(1011), (-1.0)), A::offset(s.ad_value(1011), (-1.0))), {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_div_from_scalar(779, 1.0, 228);

        s.store_div(780, 768, 192);

        s.store_div(781, 768, 193);

        s.store_div_ad_lhs(782, A::sqrt(A::scale(s.ad_value(194), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[355])))), 780);

        s.store_div_ad_lhs(783, A::sqrt(A::scale(s.ad_value(195), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[355])))), 781);

        s.store_square(784, 782);

        s.store_square(785, 783);

        s.store_offset_ad(786, A::div(A::ln(A::offset(A::exp(A::scale(s.ad_value(266), (0.005 * s.v[355]))), (-1.0))), s.ad_value(266)), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(787, A::ln(A::scale(s.ad_value(782), 0.5)), 786);

        s.store_add_ad_lhs(788, A::ln(A::scale(s.ad_value(783), 0.5)), 786);

        s.store_div_from_scalar(820, 1.0, 782);

        s.store_offset_scaled(821, 782, 3.1, 8.5);

        s.store_square(789, 821);

        s.store_scale(822, 821, 0.5);

        s.v[1146] = if (s.v[820] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1146] != 0.0) {
            s.store_scale(790, 820, 64.0);
        }

        s.v[1147] = if (s.v[820] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1146] != 0.0)) && (s.v[1147] != 0.0)) {
            s.store_offset_scaled(790, 820, 22.0, 3.0);
        }

        s.v[1148] = if (s.v[820] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1146] != 0.0)) && (!(s.v[1147] != 0.0))) && (s.v[1148] != 0.0)) {
            s.store_offset_scaled(790, 820, (-7.2), 15.5);
        }

        if (((!(s.v[1146] != 0.0)) && (!(s.v[1147] != 0.0))) && (!(s.v[1148] != 0.0))) {
            s.copy_ad(790, 782);
        }

        s.store_sub_ad(791, A::add(s.ad_value(822), A::scale(s.ad_value(784), 0.5)), A::mul(s.ad_value(782), A::sqrt(A::add(A::add(s.ad_value(822), A::scale(s.ad_value(784), 0.25)), s.ad_value(790)))));

        s.store_div_from_scalar(820, 1.0, 783);

        s.store_offset_scaled(821, 783, 3.1, 8.5);

        s.store_square(792, 821);

        s.store_scale(822, 821, 0.5);

        s.v[1149] = if (s.v[820] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1149] != 0.0) {
            s.store_scale(793, 820, 64.0);
        }

        s.v[1150] = if (s.v[820] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1149] != 0.0)) && (s.v[1150] != 0.0)) {
            s.store_offset_scaled(793, 820, 22.0, 3.0);
        }

        s.v[1151] = if (s.v[820] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1149] != 0.0)) && (!(s.v[1150] != 0.0))) && (s.v[1151] != 0.0)) {
            s.store_offset_scaled(793, 820, (-7.2), 15.5);
        }

        if (((!(s.v[1149] != 0.0)) && (!(s.v[1150] != 0.0))) && (!(s.v[1151] != 0.0))) {
            s.copy_ad(793, 783);
        }

        s.store_sub_ad(794, A::add(s.ad_value(822), A::scale(s.ad_value(785), 0.5)), A::mul(s.ad_value(783), A::sqrt(A::add(A::add(s.ad_value(822), A::scale(s.ad_value(785), 0.25)), s.ad_value(793)))));

        s.store_add_ad(728, A::offset(s.ad_value(187), s.v[362]), A::scale(A::ln(A::scale(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26)), (2.0 * s.v[715])));

        if !(s.v[728] > 0.05) {
            s.store_scalar(728, 0.05);
        }

        s.store_div_ad_lhs(729, A::sqrt(A::scale(s.ad_value(183), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[361])))), 769);

        s.v[730] = 0.0;

        s.v[731] = 0.0;

        s.v[1152] = if (s.v[188] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1152] != 0.0) {
            s.store_div_from_scalar(732, 80000000.0, 770);
        }

        if (s.v[1152] != 0.0) {
            s.store_ad(731, &{
                if (s.v[188] > s.v[732]) {
                    s.ad_value(188)
                } else {
                    s.ad_value(732)
                }
            });
        }

        if (s.v[1152] != 0.0) {
            s.store_ad(731, &{
                if (5e24 > s.v[731]) {
                    A::constant(5e24)
                } else {
                    s.ad_value(731)
                }
            });
        }

        if (s.v[1152] != 0.0) {
            s.store_div_ad(730, A::scale(A::mul(A::scale(s.ad_value(769), 2.0), s.ad_value(769)), s.v[715]), A::scale(s.ad_value(731), (1.6021918e-19 * s.v[767])));
        }

        s.v[733] = ((100.0 * s.v[715]) * s.v[715]);

        s.v[1153] = if (p.p52 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1153] != 0.0) {
            s.store_sqrt_ad(734, A::mul(A::mul(A::scale(s.ad_value(729), s.v[715]), s.ad_value(729)), s.ad_value(728)));
        }

        if (s.v[1153] != 0.0) {
            s.store_mul_ad(735, A::scale(s.ad_value(773), 0.75), A::powf(s.ad_value(734), 0.6666666666666666));
        }

        if (s.v[1153] != 0.0) {
            s.store_add(728, 728, 735);
        }

        if (s.v[1153] != 0.0) {
            s.store_mul_ad_rhs(729, 729, A::offset(A::div(A::scale(s.ad_value(735), (2.0 * 0.6666666666666666)), s.ad_value(734)), 1.0));
        }

        s.store_sqrt(736, 728);

        s.store_scale(737, 728, 0.95);

        s.store_mul_ad_lhs(738, A::scale(s.ad_value(728), 0.0025), 728);

        s.copy_ad(739, 738);

        s.store_scaled_sqrt(740, 739, 0.5);

        s.store_scale_ad(741, A::sub(A::sub(s.ad_value(737), s.ad_value(740)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(737), s.ad_value(740)), A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)))), 0.5);

        s.store_scaled_offset(742, 728, s.v[362], 0.5);

        s.store_sub_ad_lhs(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);

        s.store_sub_ad_lhs(744, A::sub(A::sqrt(A::add(A::add(s.ad_value(185), s.ad_value(186)), s.ad_value(728))), s.ad_value(736)), 743);

        s.store_add_ad(745, A::add(A::offset(s.ad_value(187), s.v[362]), s.ad_value(256)), A::scale(A::ln(A::scale(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26)), (2.0 * s.v[715])));

        if !(s.v[745] > 0.05) {
            s.store_scalar(745, 0.05);
        }

        s.store_div_ad_lhs(746, A::sqrt(A::scale(s.ad_value(772), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[361])))), 769);

        s.v[1154] = if (p.p52 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1154] != 0.0) {
            s.store_sqrt_ad(734, A::mul(A::mul(A::scale(s.ad_value(746), s.v[715]), s.ad_value(746)), s.ad_value(745)));
        }

        if (s.v[1154] != 0.0) {
            s.store_mul_ad(735, A::scale(s.ad_value(773), 0.75), A::powf(s.ad_value(734), 0.6666666666666666));
        }

        if (s.v[1154] != 0.0) {
            s.store_add(745, 745, 735);
        }

        if (s.v[1154] != 0.0) {
            s.store_mul_ad_rhs(746, 746, A::offset(A::div(A::scale(s.ad_value(735), (2.0 * 0.6666666666666666)), s.ad_value(734)), 1.0));
        }

        s.store_scale(747, 745, 0.95);

        s.store_mul_ad_lhs(748, A::scale(s.ad_value(745), 0.0025), 745);

        s.copy_ad(749, 748);

        s.store_scaled_sqrt(740, 749, 0.5);

        s.store_scale_ad(750, A::sub(A::sub(s.ad_value(747), s.ad_value(740)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(747), s.ad_value(740)), A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)))), 0.5);

        s.store_offset_ad(700, A::add(s.ad_value(177), A::mul(A::scale(s.ad_value(178), s.v[358]), A::offset(A::scale(s.ad_value(179), s.v[358]), 1.0))), s.v[21]);

        s.store_exp_ad(751, A::scale(s.ad_value(180), s.v[360]));

        s.store_mul(701, 189, 751);

        s.store_scale(702, 190, 1.0 / (s.v[359]));

        s.store_exp_ad(752, A::scale(s.ad_value(203), s.v[360]));

        s.store_mul(703, 202, 752);

        s.store_mul_ad_lhs(716, A::scale(s.ad_value(703), s.v[20]), 769);

        s.store_mul_ad_rhs(705, 206, A::exp(A::scale(s.ad_value(207), s.v[360])));

        s.store_exp_ad(753, A::scale(s.ad_value(205), s.v[360]));

        s.store_mul(704, 204, 753);

        s.store_mul_ad_rhs(707, 210, A::exp(A::scale(s.ad_value(211), s.v[360])));

        s.store_exp_ad(754, A::scale(s.ad_value(209), s.v[360]));

        s.store_mul(706, 208, 754);

        s.store_exp_ad(755, A::scale(s.ad_value(213), s.v[360]));

        s.store_mul(708, 212, 755);

        s.store_exp_ad(756, A::scale(s.ad_value(216), s.v[360]));

        s.store_mul(709, 215, 756);

        s.store_mul_ad_lhs(757, A::scale(s.ad_value(716), 2.0), 709);

        s.store_exp_ad(758, A::scale(s.ad_value(220), s.v[360]));

        s.store_mul(720, 219, 758);

        s.store_mul(721, 258, 758);

        s.store_mul_ad_rhs(712, 230, A::exp(A::scale(A::neg(s.ad_value(231)), s.v[360])));

        s.store_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));

        s.store_div_ad_lhs(722, A::scale(s.ad_value(716), (s.v[715] * s.v[715])), 771);

        s.v[1155] = if ((p.p46 != 0.0) && (s.v[287] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1155] != 0.0) {
            s.store_offset_ad(713, A::add(s.ad_value(282), A::scale(s.ad_value(283), s.v[358])), s.v[23]);
        }

        if (s.v[1155] != 0.0) {
            s.store_exp_ad(759, A::scale(s.ad_value(288), s.v[360]));
        }

        if (s.v[1155] != 0.0) {
            s.store_mul(714, 287, 759);
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_lhs(717, A::scale(s.ad_value(714), s.v[22]), 769);
        }

        if (s.v[1155] != 0.0) {
            s.store_scale_ad(723, A::offset(A::scale(s.ad_value(286), s.v[359]), 1.0), s.v[715]);
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
        if (s.v[1155] != 0.0) {
            s.store_add_ad(760, A::offset(s.ad_value(284), s.v[362]), A::mul(A::scale(s.ad_value(723), 2.0), A::ln(A::scale(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26))));
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(760, &{
                if (s.v[760] > 0.05) {
                    s.ad_value(760)
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[1155] != 0.0) {
            s.store_div_ad_lhs(761, A::sqrt(A::scale(s.ad_value(285), ((2.0 * 1.6021918e-19) * (s.v[767] * s.v[361])))), 769);
        }

        if (s.v[1155] != 0.0) {
            s.store_square(724, 761);
        }

        if (s.v[1155] != 0.0) {
            s.store_ln(725, 724);
        }

        if (s.v[1155] != 0.0) {
            s.store_scale(762, 760, 0.95);
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_lhs(763, A::scale(s.ad_value(760), 0.0025), 760);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(764, 763);
        }

        if (s.v[1155] != 0.0) {
            s.store_scaled_sqrt(765, 764, 0.5);
        }

        if (s.v[1155] != 0.0) {
            s.store_scale_ad(766, A::sub(A::sub(s.ad_value(762), s.ad_value(765)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(762), s.ad_value(765)), A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)))), 0.5);
        }

        if (s.v[1155] != 0.0) {
            s.store_div_ad_lhs(726, A::scale(s.ad_value(717), (s.v[715] * s.v[715])), 771);
        }

        if (s.v[1155] != 0.0) {
            s.store_scale(727, 295, (4.0 * (1.3806505e-23 * s.v[356])));
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(713, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(759, 1.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(714, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(717, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(723, s.v[715]);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(760, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(761, 1.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(725, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(762, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(763, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(764, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(765, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(766, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if (!(s.v[1155] != 0.0)) {
            s.store_scalar(727, 1.0);
        }

        s.store_div_from_scalar(795, 1.0, 246);

        s.store_scale_ad(796, A::sqrt(A::scale(s.ad_value(246), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(797, 796, 181);

        s.store_mul(798, 796, 192);

        s.store_mul(799, 796, 193);

        s.v[800] = 0.0;

        s.v[1156] = if (s.v[241] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1156] != 0.0) {
            s.store_div_ad_lhs(800, A::scale(s.ad_value(240), (-0.495)), 241);
        }

        s.v[801] = 0.0;

        s.v[1157] = if (s.v[243] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1157] != 0.0) {
            s.store_div_ad_lhs(801, A::scale(s.ad_value(242), (-0.495)), 243);
        }

        s.v[1158] = if (s.v[245] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1158] != 0.0) {
            s.store_div_ad_lhs(802, A::scale(s.ad_value(244), (-0.495)), 245);
        }

        s.store_ad(803, &A::pow_from_scalar(s.v[352], s.ad_value(239)));

        s.store_mul(236, 236, 803);

        s.store_mul(237, 237, 803);

        s.store_mul(238, 238, 803);

        s.store_div_ad(804, A::scale(s.ad_value(247), 4e-18), A::square(s.ad_value(192)));

        s.store_div_ad(805, A::scale(s.ad_value(248), 4e-18), A::square(s.ad_value(193)));

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

        s.v[1159] = if (s.v[272] > 1e-10) { 1.0 } else { 0.0 };

        if (s.v[1159] != 0.0) {
            s.store_div_from_scalar(808, 0.75, 272);
        }

        s.store_square(809, 273);

        s.store_scale(810, 277, (9.1093826e-31 * 1000000000.0));

        s.v[1160] = if (s.v[300] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1160] != 0.0) {
            s.store_div_from_scalar(811, 1.0, 300);
        }

        if (!(s.v[1160] != 0.0)) {
            s.store_scalar(811, 0.0);
        }

        s.v[1161] = if (s.v[301] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1161] != 0.0) {
            s.store_div_from_scalar(812, 1.0, 301);
        }

        if (!(s.v[1161] != 0.0)) {
            s.store_scalar(812, 0.0);
        }

        s.v[1162] = if (s.v[302] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1162] != 0.0) {
            s.store_div_from_scalar(813, 1.0, 302);
        }

        if (!(s.v[1162] != 0.0)) {
            s.store_scalar(813, 0.0);
        }

        s.v[1163] = if (s.v[303] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1163] != 0.0) {
            s.store_div_from_scalar(814, 1.0, 303);
        }

        if (!(s.v[1163] != 0.0)) {
            s.store_scalar(814, 0.0);
        }

        s.v[1164] = if (s.v[304] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1164] != 0.0) {
            s.store_div_from_scalar(815, 1.0, 304);
        }

        if (!(s.v[1164] != 0.0)) {
            s.store_scalar(815, 0.0);
        }

        s.v[1165] = if (s.v[305] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1165] != 0.0) {
            s.store_div_from_scalar(816, 1.0, 305);
        }

        if (!(s.v[1165] != 0.0)) {
            s.store_scalar(816, 0.0);
        }

        s.v[1166] = if (s.v[306] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1166] != 0.0) {
            s.store_div_from_scalar(817, 1.0, 306);
        }

        if (!(s.v[1166] != 0.0)) {
            s.store_scalar(817, 0.0);
        }

        s.store_scale(24, 6, s.v[646]);

        s.store_scale(25, 6, s.v[647]);

        s.store_scale(26, 6, s.v[648]);

        s.store_scale(27, 6, s.v[673]);

        s.store_scale(28, 6, s.v[674]);

        s.store_scale(29, 6, s.v[675]);

        s.v[30] = 0.0;

        s.v[1167] = if (p.p43 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1167] != 0.0) {
            s.store_scalar(30, 1.0);
        }

        s.copy_ad(31, 313);

        s.v[1168] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1168] != 0.0) {
            s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));
        }

        s.v[1169] = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1169] != 0.0) {
            s.store_scale(24, 6, s.v[649]);
        }

        if (s.v[1169] != 0.0) {
            s.store_sub_ad(25, A::scale(s.ad_value(6), s.v[650]), A::mul(s.ad_value(30), s.ad_value(31)));
        }

        if (s.v[1169] != 0.0) {
            s.copy_ad(26, 31);
        }

        if (s.v[1169] != 0.0) {
            s.store_scale(27, 6, s.v[676]);
        }

        if (s.v[1169] != 0.0) {
            s.store_sub_ad(28, A::scale(s.ad_value(6), s.v[677]), A::mul(s.ad_value(30), s.ad_value(31)));
        }

        if (s.v[1169] != 0.0) {
            s.copy_ad(29, 31);
        }

        s.v[1170] = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1170] != 0.0) {
            s.store_ad(646, &{
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(647, &{
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(648, &{
                if (s.v[26] > 0.0) {
                    s.ad_value(26)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(673, &{
                if (s.v[27] > 0.0) {
                    s.ad_value(27)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(674, &{
                if (s.v[28] > 0.0) {
                    s.ad_value(28)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1170] != 0.0) {
            s.store_ad(675, &{
                if (s.v[29] > 0.0) {
                    s.ad_value(29)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(673, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
            s.store_scalar(674, 0.0);
        }

        if (!(s.v[1170] != 0.0)) {
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
        s.v[501] = 0.0;

        s.v[490] = 0.0;

        s.v[1171] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[1172] = if ((s.v[387] * s.v[646]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scale_ad(454, A::ln(A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(646), s.v[387])), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1172] != 0.0))) {
            s.store_scalar(454, 100000000.0);
        }

        s.v[1173] = if ((s.v[388] * s.v[647]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1173] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(647), s.v[388])), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1173] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1174] = if ((s.v[389] * s.v[648]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1174] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p822, A::scale(s.ad_value(648), s.v[389])), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1174] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(654, &A::min(A::min(s.ad_value(454), s.ad_value(455)), s.ad_value(456)));
        }

        s.v[1175] = if ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1175] != 0.0)) {
            s.store_exp_ad(655, A::scale(s.ad_value(654), s.v[371]));
        }

        s.v[1176] = if ((s.v[654] * s.v[371]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (!(s.v[1175] != 0.0))) && (s.v[1176] != 0.0)) {
            s.store_div_from_scalar_ad(655, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(654), s.v[371])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(654), s.v[371])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(654), s.v[371])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1171] != 0.0) && (!(s.v[1175] != 0.0))) && (!(s.v[1176] != 0.0))) {
            s.store_scale_ad(655, A::offset(A::mul(A::offset(A::scale(s.ad_value(654), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(654), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(654), s.v[371]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(396, s.v[393]);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(397, s.v[394]);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(398, s.v[395]);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(399, p.p831);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(400, p.p832);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(401, p.p833);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(402, p.p828);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(403, p.p829);
        }

        if (s.v[1171] != 0.0) {
            s.store_scalar(404, p.p830);
        }

        s.v[1177] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scalar(396, (s.v[394] + s.v[395]));
        }

        if ((s.v[1171] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scalar(399, (0.9 * (p.p832).min(p.p833)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1177] != 0.0)) {
            s.store_scalar(402, (p.p829 + p.p830));
        }

        s.v[1178] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(397, (s.v[393] + s.v[395]));
        }

        if ((s.v[1171] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(400, (0.9 * (p.p831).min(p.p833)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1178] != 0.0)) {
            s.store_scalar(403, (p.p828 + p.p830));
        }

        s.v[1179] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(398, (s.v[393] + s.v[394]));
        }

        if ((s.v[1171] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(401, (0.9 * (p.p831).min(p.p832)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1179] != 0.0)) {
            s.store_scalar(404, (p.p828 + p.p829));
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(656, &A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)));
        }

        if (s.v[1171] != 0.0) {
            s.store_scale(657, 656, 0.1);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(377, &A::max(A::max(s.ad_value(399), s.ad_value(400)), s.ad_value(401)));
        }

        if (s.v[1171] != 0.0) {
            s.store_mul_ad_rhs(658, 656, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377)))));
        }

        if (s.v[1171] != 0.0) {
            s.store_offset_ad(659, A::min(A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404)), (-0.05));
        }

        s.v[1180] = if ((s.v[563] * s.v[673]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1180] != 0.0)) {
            s.store_scale_ad(454, A::ln(A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(563), s.ad_value(673))), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1180] != 0.0))) {
            s.store_scalar(454, 100000000.0);
        }

        s.v[1181] = if ((s.v[564] * s.v[674]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1181] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(564), s.ad_value(674))), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1181] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1182] = if ((s.v[565] * s.v[675]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1182] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p822, A::mul(s.ad_value(565), s.ad_value(675))), 1.0)), s.v[370]);
        }

        if ((s.v[1171] != 0.0) && (!(s.v[1182] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(681, &A::min(A::min(s.ad_value(454), s.ad_value(455)), s.ad_value(456)));
        }

        s.v[1183] = if ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1183] != 0.0)) {
            s.store_exp_ad(682, A::scale(s.ad_value(681), s.v[371]));
        }

        s.v[1184] = if ((s.v[681] * s.v[371]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (!(s.v[1183] != 0.0))) && (s.v[1184] != 0.0)) {
            s.store_div_from_scalar_ad(682, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(681), s.v[371])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(681), s.v[371])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(681), s.v[371])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1171] != 0.0) && (!(s.v[1183] != 0.0))) && (!(s.v[1184] != 0.0))) {
            s.store_scale_ad(682, A::offset(A::mul(A::offset(A::scale(s.ad_value(681), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(681), s.v[371]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(681), s.v[371]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(396, 569);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(397, 570);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(398, 571);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(399, 511);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(400, 512);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(401, 513);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(402, 508);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(403, 509);
        }

        if (s.v[1171] != 0.0) {
            s.copy_ad(404, 510);
        }

        s.v[1185] = if (s.v[673] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_add(396, 570, 571);
        }

        if ((s.v[1171] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
        }

        if ((s.v[1171] != 0.0) && (s.v[1185] != 0.0)) {
            s.store_add(402, 509, 510);
        }

        s.v[1186] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1186] != 0.0)) {
            s.store_add(397, 569, 571);
        }

        if ((s.v[1171] != 0.0) && (s.v[1186] != 0.0)) {
            s.store_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);
        }

        if ((s.v[1171] != 0.0) && (s.v[1186] != 0.0)) {
            s.store_add(403, 508, 510);
        }

        s.v[1187] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1187] != 0.0)) {
            s.store_add(398, 569, 570);
        }

        if ((s.v[1171] != 0.0) && (s.v[1187] != 0.0)) {
            s.store_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);
        }

        if ((s.v[1171] != 0.0) && (s.v[1187] != 0.0)) {
            s.store_add(404, 508, 509);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(683, &A::min(A::min(s.ad_value(396), s.ad_value(397)), s.ad_value(398)));
        }

        if (s.v[1171] != 0.0) {
            s.store_scale(684, 683, 0.1);
        }

        if (s.v[1171] != 0.0) {
            s.store_ad(377, &A::max(A::max(s.ad_value(399), s.ad_value(400)), s.ad_value(401)));
        }

        if (s.v[1171] != 0.0) {
            s.store_mul_ad_rhs(685, 683, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377)))));
        }

        if (s.v[1171] != 0.0) {
            s.store_offset_ad(686, A::min(A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404)), (-0.05));
        }

        s.v[1188] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1190, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1191, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1198, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1201, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1202, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1203, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1205, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1206, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1207, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1208, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1210, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1211, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1213, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1214, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1217, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1221, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1225, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1226, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1227, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1228, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1229, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1230, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1232, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1233, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(498, 0.4);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(499, 0.65);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(500, 0.8);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scale_ad(485, A::neg(s.ad_value(498)), p.p928);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scale_ad(486, A::neg(s.ad_value(499)), p.p928);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scale_ad(487, A::neg(s.ad_value(500)), p.p928);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(488, 0.1);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(489, 0.2);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1205, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1202, 0.0);
        }

        s.v[1237] = if !(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1238] = if (s.v[485] < s.v[654]) { 1.0 } else { 0.0 };

        s.v[1239] = if (((((-0.5) * (s.v[485] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) && (s.v[1239] != 0.0)) {
            s.store_exp_ad(1200, A::scale(s.ad_value(485), (s.v[371] * (-0.5))));
        }

        s.v[1240] = if (((-0.5) * (s.v[485] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

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
        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) && (!(s.v[1239] != 0.0))) && (s.v[1240] != 0.0)) {
            let assign15640_ad_e13177: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1200, &assign15640_ad_e13177);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) && (!(s.v[1239] != 0.0))) && (!(s.v[1240] != 0.0))) {
            s.store_scale_ad(1200, A::offset(A::mul(A::offset(A::scale(s.ad_value(485), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(485), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(485), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) {
            s.store_div_from_scalar(1201, 1.0, 1200);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) {
            s.store_square(1198, 1201);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (!(s.v[1238] != 0.0))) {
            s.store_mul_ad_lhs(1198, A::offset(A::scale(A::sub(s.ad_value(485), s.ad_value(654)), s.v[371]), 1.0), 655);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (!(s.v[1238] != 0.0))) {
            s.store_sqrt(1201, 1198);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (!(s.v[1238] != 0.0))) {
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.v[1241] = if (s.v[485] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (s.v[1241] != 0.0)) {
            s.store_scale_ad(1202, A::ln(A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1200), 1.0), A::offset(s.ad_value(1200), 3.0))))), (s.v[370] * 2.0));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) && (!(s.v[1241] != 0.0))) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1201), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1201), 1.0), A::offset(A::scale(s.ad_value(1201), 3.0), 1.0))))), (s.v[370] * 2.0)), 485);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) {
            s.store_sub(1203, 656, 1202);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) {
            s.store_scale_ad(1204, A::sub(A::add(s.ad_value(485), s.ad_value(1203)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(485), s.ad_value(1203)), A::sub(s.ad_value(485), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) {
            s.store_scale_ad(1205, A::sub(A::add(s.ad_value(485), s.ad_value(659)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(485), s.ad_value(659)), A::sub(s.ad_value(485), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1237] != 0.0)) {
            s.store_scale_ad(1206, A::sub(s.ad_value(485), A::sqrt(A::offset(A::mul(s.ad_value(485), s.ad_value(485)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1242] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1242] != 0.0)) {
            s.store_scalar(1234, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.v[1243] = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (s.v[1243] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1244] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p831)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1245] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) && (s.v[1245] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) && (!(s.v[1245] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) {
            s.store_scale(1214, 1207, s.v[423]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[384]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1243] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.v[1246] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (s.v[1246] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[408]), s.ad_value(1210)), s.v[438]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1247] = if (((-p.p831) * s.v[411]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (s.v[1247] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[435]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[435])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1248] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (s.v[1248] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1249] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (s.v[1249] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (!(s.v[1249] != 0.0))) {
            let assign16180_ad_e14073: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign16180_ad_e14073);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1250] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (s.v[1250] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1251] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (!(s.v[1250] != 0.0))) && (s.v[1251] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (!(s.v[1250] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) && (!(s.v[1250] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[435]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p845);
        }

        s.v[1252] = if (p.p851 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (s.v[1252] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1253] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1252] != 0.0))) && (s.v[1253] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1252] != 0.0))) && (!(s.v[1253] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[426]), s.ad_value(1207)), s.v[411]);
        }

        s.v[1254] = if (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1252] != 0.0))) && (s.v[1254] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(441)), s.ad_value(1232)));
        }

        s.v[1255] = if (((-s.v[441]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1252] != 0.0))) && (!(s.v[1254] != 0.0))) && (s.v[1255] != 0.0)) {
            let assign16370_ad_e14400: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign16370_ad_e14400);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1252] != 0.0))) && (!(s.v[1254] != 0.0))) && (!(s.v[1255] != 0.0))) {
            let assign16380_ad_e14450: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign16380_ad_e14450);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(485), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p851);
        }

        s.v[1256] = if (p.p860 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (s.v[1256] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1257] = if (s.v[1206] > ((-s.v[444]) * p.p860)) { 1.0 } else { 0.0 };

        s.v[1258] = if (p.p863 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1256] != 0.0))) && (s.v[1257] != 0.0)) && (s.v[1258] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[448]), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1256] != 0.0))) && (s.v[1257] != 0.0)) && (!(s.v[1258] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[448])), p.p863);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1256] != 0.0))) && (s.v[1257] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) && (!(s.v[1256] != 0.0))) && (!(s.v[1257] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p860)), s.v[451]), s.v[445]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1242] != 0.0))) {
            s.store_mul_ad_lhs(1234, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1259] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_scalar(1235, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.v[1260] = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1260] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1261] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) && (s.v[1261] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p832)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1262] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            s.store_scale(1214, 1207, s.v[424]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[385]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1260] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.v[1263] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1263] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[409]), s.ad_value(1210)), s.v[439]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1264] = if (((-p.p832) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (s.v[1264] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[436]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[436])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1265] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (s.v[1265] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1266] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (s.v[1266] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (!(s.v[1266] != 0.0))) {
            let assign16880_ad_e15216: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign16880_ad_e15216);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1267] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (s.v[1267] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1268] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (!(s.v[1267] != 0.0))) && (s.v[1268] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (!(s.v[1267] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) && (!(s.v[1267] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[436]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p846);
        }

        s.v[1269] = if (p.p852 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1269] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1270] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1269] != 0.0))) && (s.v[1270] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1269] != 0.0))) && (!(s.v[1270] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1269] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[427]), s.ad_value(1207)), s.v[412]);
        }

        s.v[1271] = if (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1269] != 0.0))) && (s.v[1271] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(442)), s.ad_value(1232)));
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
        s.v[1272] = if (((-s.v[442]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1269] != 0.0))) && (!(s.v[1271] != 0.0))) && (s.v[1272] != 0.0)) {
            let assign17070_ad_e15543: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign17070_ad_e15543);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1269] != 0.0))) && (!(s.v[1271] != 0.0))) && (!(s.v[1272] != 0.0))) {
            let assign17080_ad_e15593: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign17080_ad_e15593);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1269] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(485), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p852);
        }

        s.v[1273] = if (p.p861 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (s.v[1273] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1274] = if (s.v[1206] > ((-s.v[444]) * p.p861)) { 1.0 } else { 0.0 };

        s.v[1275] = if (p.p864 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1273] != 0.0))) && (s.v[1274] != 0.0)) && (s.v[1275] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[449]), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1273] != 0.0))) && (s.v[1274] != 0.0)) && (!(s.v[1275] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[449])), p.p864);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1273] != 0.0))) && (s.v[1274] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) && (!(s.v[1273] != 0.0))) && (!(s.v[1274] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p861)), s.v[452]), s.v[446]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_mul_ad_lhs(1235, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1276] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1276] != 0.0)) {
            s.store_scalar(1236, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.v[1277] = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (s.v[1277] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1278] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) && (s.v[1278] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) && (!(s.v[1278] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p833)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1279] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) && (s.v[1279] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[431]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) && (!(s.v[1279] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_scale(1214, 1207, s.v[425]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[386]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1277] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.v[1280] = if (p.p847 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (s.v[1280] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[410]), s.ad_value(1210)), s.v[440]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1281] = if (((-p.p833) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1281] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1281] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[437]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[437])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1282] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1282] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1282] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1283] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1283] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1283] != 0.0))) {
            let assign17580_ad_e16359: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign17580_ad_e16359);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1284] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (s.v[1284] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1285] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1284] != 0.0))) && (s.v[1285] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1284] != 0.0))) && (!(s.v[1285] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) && (!(s.v[1284] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[437]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p847);
        }

        s.v[1286] = if (p.p853 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (s.v[1286] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1287] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1286] != 0.0))) && (s.v[1287] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]), p.p833);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[428]), s.ad_value(1207)), s.v[413]);
        }

        s.v[1288] = if (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1286] != 0.0))) && (s.v[1288] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(443)), s.ad_value(1232)));
        }

        s.v[1289] = if (((-s.v[443]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1288] != 0.0))) && (s.v[1289] != 0.0)) {
            let assign17770_ad_e16686: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign17770_ad_e16686);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1288] != 0.0))) && (!(s.v[1289] != 0.0))) {
            let assign17780_ad_e16736: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign17780_ad_e16736);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(485), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p853);
        }

        s.v[1290] = if (p.p862 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (s.v[1290] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1291] = if (s.v[1206] > ((-s.v[444]) * p.p862)) { 1.0 } else { 0.0 };

        s.v[1292] = if (p.p865 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) && (s.v[1292] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[450]), A::scale(s.ad_value(1206), s.v[450])), A::scale(s.ad_value(1206), s.v[450])), A::scale(s.ad_value(1206), s.v[450]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) && (!(s.v[1292] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[450])), p.p865);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1291] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p862)), s.v[453]), s.v[447]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1276] != 0.0))) {
            s.store_mul_ad_lhs(1236, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_add_ad(475, A::add(A::mul(s.ad_value(646), s.ad_value(1234)), A::mul(s.ad_value(647), s.ad_value(1235))), A::mul(s.ad_value(648), s.ad_value(1236)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1205, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1202, 0.0);
        }

        s.v[1293] = if !(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1294] = if (s.v[486] < s.v[654]) { 1.0 } else { 0.0 };

        s.v[1295] = if (((((-0.5) * (s.v[486] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (s.v[1294] != 0.0)) && (s.v[1295] != 0.0)) {
            s.store_exp_ad(1200, A::scale(s.ad_value(486), (s.v[371] * (-0.5))));
        }

        s.v[1296] = if (((-0.5) * (s.v[486] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (s.v[1294] != 0.0)) && (!(s.v[1295] != 0.0))) && (s.v[1296] != 0.0)) {
            let assign18040_ad_e17107: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1200, &assign18040_ad_e17107);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (s.v[1294] != 0.0)) && (!(s.v[1295] != 0.0))) && (!(s.v[1296] != 0.0))) {
            s.store_scale_ad(1200, A::offset(A::mul(A::offset(A::scale(s.ad_value(486), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(486), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(486), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (s.v[1294] != 0.0)) {
            s.store_div_from_scalar(1201, 1.0, 1200);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (s.v[1294] != 0.0)) {
            s.store_square(1198, 1201);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (!(s.v[1294] != 0.0))) {
            s.store_mul_ad_lhs(1198, A::offset(A::scale(A::sub(s.ad_value(486), s.ad_value(654)), s.v[371]), 1.0), 655);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (!(s.v[1294] != 0.0))) {
            s.store_sqrt(1201, 1198);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (!(s.v[1294] != 0.0))) {
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.v[1297] = if (s.v[486] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (s.v[1297] != 0.0)) {
            s.store_scale_ad(1202, A::ln(A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1200), 1.0), A::offset(s.ad_value(1200), 3.0))))), (s.v[370] * 2.0));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) && (!(s.v[1297] != 0.0))) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1201), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1201), 1.0), A::offset(A::scale(s.ad_value(1201), 3.0), 1.0))))), (s.v[370] * 2.0)), 486);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) {
            s.store_sub(1203, 656, 1202);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) {
            s.store_scale_ad(1204, A::sub(A::add(s.ad_value(486), s.ad_value(1203)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(1203)), A::sub(s.ad_value(486), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) {
            s.store_scale_ad(1205, A::sub(A::add(s.ad_value(486), s.ad_value(659)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(659)), A::sub(s.ad_value(486), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1293] != 0.0)) {
            s.store_scale_ad(1206, A::sub(s.ad_value(486), A::sqrt(A::offset(A::mul(s.ad_value(486), s.ad_value(486)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1298] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1298] != 0.0)) {
            s.store_scalar(1234, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.v[1299] = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (s.v[1299] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1300] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) && (s.v[1300] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p831)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1301] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) && (s.v[1301] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) && (!(s.v[1301] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            s.store_scale(1214, 1207, s.v[423]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[384]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1299] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.v[1302] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (s.v[1302] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[408]), s.ad_value(1210)), s.v[438]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1303] = if (((-p.p831) * s.v[411]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (s.v[1303] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[435]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[435])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1304] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (s.v[1304] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
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
        s.v[1305] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (!(s.v[1305] != 0.0))) {
            let assign18580_ad_e18003: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign18580_ad_e18003);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1306] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (s.v[1306] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1307] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1307] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (!(s.v[1306] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) && (!(s.v[1306] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[435]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p845);
        }

        s.v[1308] = if (p.p851 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1309] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1308] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[426]), s.ad_value(1207)), s.v[411]);
        }

        s.v[1310] = if (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1308] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(441)), s.ad_value(1232)));
        }

        s.v[1311] = if (((-s.v[441]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1308] != 0.0))) && (!(s.v[1310] != 0.0))) && (s.v[1311] != 0.0)) {
            let assign18770_ad_e18330: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign18770_ad_e18330);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1308] != 0.0))) && (!(s.v[1310] != 0.0))) && (!(s.v[1311] != 0.0))) {
            let assign18780_ad_e18380: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign18780_ad_e18380);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p851);
        }

        s.v[1312] = if (p.p860 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (s.v[1312] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1313] = if (s.v[1206] > ((-s.v[444]) * p.p860)) { 1.0 } else { 0.0 };

        s.v[1314] = if (p.p863 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1312] != 0.0))) && (s.v[1313] != 0.0)) && (s.v[1314] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[448]), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1312] != 0.0))) && (s.v[1313] != 0.0)) && (!(s.v[1314] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[448])), p.p863);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1312] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) && (!(s.v[1312] != 0.0))) && (!(s.v[1313] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p860)), s.v[451]), s.v[445]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1298] != 0.0))) {
            s.store_mul_ad_lhs(1234, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1315] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1315] != 0.0)) {
            s.store_scalar(1235, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.v[1316] = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (s.v[1316] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1317] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (s.v[1317] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p832)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1318] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (s.v[1318] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) && (!(s.v[1318] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_scale(1214, 1207, s.v[424]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[385]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1316] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.v[1319] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (s.v[1319] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[409]), s.ad_value(1210)), s.v[439]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1320] = if (((-p.p832) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (s.v[1320] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[436]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[436])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1321] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1322] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (s.v[1322] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1322] != 0.0))) {
            let assign19280_ad_e19146: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign19280_ad_e19146);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1323] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (s.v[1323] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1324] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1323] != 0.0))) && (s.v[1324] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1323] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) && (!(s.v[1323] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[436]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p846);
        }

        s.v[1325] = if (p.p852 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (s.v[1325] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1326] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1325] != 0.0))) && (s.v[1326] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1325] != 0.0))) && (!(s.v[1326] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1325] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[427]), s.ad_value(1207)), s.v[412]);
        }

        s.v[1327] = if (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1325] != 0.0))) && (s.v[1327] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(442)), s.ad_value(1232)));
        }

        s.v[1328] = if (((-s.v[442]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1325] != 0.0))) && (!(s.v[1327] != 0.0))) && (s.v[1328] != 0.0)) {
            let assign19470_ad_e19473: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign19470_ad_e19473);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1325] != 0.0))) && (!(s.v[1327] != 0.0))) && (!(s.v[1328] != 0.0))) {
            let assign19480_ad_e19523: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign19480_ad_e19523);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1325] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p852);
        }

        s.v[1329] = if (p.p861 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (s.v[1329] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1330] = if (s.v[1206] > ((-s.v[444]) * p.p861)) { 1.0 } else { 0.0 };

        s.v[1331] = if (p.p864 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1329] != 0.0))) && (s.v[1330] != 0.0)) && (s.v[1331] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[449]), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1329] != 0.0))) && (s.v[1330] != 0.0)) && (!(s.v[1331] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[449])), p.p864);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1329] != 0.0))) && (s.v[1330] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) && (!(s.v[1329] != 0.0))) && (!(s.v[1330] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p861)), s.v[452]), s.v[446]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1315] != 0.0))) {
            s.store_mul_ad_lhs(1235, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1332] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1332] != 0.0)) {
            s.store_scalar(1236, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.v[1333] = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (s.v[1333] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1334] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) && (s.v[1334] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) && (!(s.v[1334] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p833)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1335] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) && (s.v[1335] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[431]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) && (!(s.v[1335] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            s.store_scale(1214, 1207, s.v[425]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[386]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1333] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.v[1336] = if (p.p847 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (s.v[1336] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[410]), s.ad_value(1210)), s.v[440]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1337] = if (((-p.p833) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (s.v[1337] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (!(s.v[1337] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[437]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[437])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1338] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (s.v[1338] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (!(s.v[1338] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1339] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (s.v[1339] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (!(s.v[1339] != 0.0))) {
            let assign19980_ad_e20289: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign19980_ad_e20289);
        }

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
        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1340] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (s.v[1340] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1341] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (!(s.v[1340] != 0.0))) && (s.v[1341] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (!(s.v[1340] != 0.0))) && (!(s.v[1341] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) && (!(s.v[1340] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[437]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p847);
        }

        s.v[1342] = if (p.p853 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (s.v[1342] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1343] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]), p.p833);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[428]), s.ad_value(1207)), s.v[413]);
        }

        s.v[1344] = if (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1344] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(443)), s.ad_value(1232)));
        }

        s.v[1345] = if (((-s.v[443]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1344] != 0.0))) && (s.v[1345] != 0.0)) {
            let assign20170_ad_e20616: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign20170_ad_e20616);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1344] != 0.0))) && (!(s.v[1345] != 0.0))) {
            let assign20180_ad_e20666: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign20180_ad_e20666);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p853);
        }

        s.v[1346] = if (p.p862 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (s.v[1346] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1347] = if (s.v[1206] > ((-s.v[444]) * p.p862)) { 1.0 } else { 0.0 };

        s.v[1348] = if (p.p865 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) && (s.v[1348] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[450]), A::scale(s.ad_value(1206), s.v[450])), A::scale(s.ad_value(1206), s.v[450])), A::scale(s.ad_value(1206), s.v[450]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) && (!(s.v[1348] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[450])), p.p865);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1347] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p862)), s.v[453]), s.v[447]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1332] != 0.0))) {
            s.store_mul_ad_lhs(1236, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_add_ad(476, A::add(A::mul(s.ad_value(646), s.ad_value(1234)), A::mul(s.ad_value(647), s.ad_value(1235))), A::mul(s.ad_value(648), s.ad_value(1236)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1205, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1202, 0.0);
        }

        s.v[1349] = if !(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1350] = if (s.v[487] < s.v[654]) { 1.0 } else { 0.0 };

        s.v[1351] = if (((((-0.5) * (s.v[487] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) && (s.v[1351] != 0.0)) {
            s.store_exp_ad(1200, A::scale(s.ad_value(487), (s.v[371] * (-0.5))));
        }

        s.v[1352] = if (((-0.5) * (s.v[487] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) && (!(s.v[1351] != 0.0))) && (s.v[1352] != 0.0)) {
            let assign20440_ad_e21037: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1200, &assign20440_ad_e21037);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) && (!(s.v[1351] != 0.0))) && (!(s.v[1352] != 0.0))) {
            s.store_scale_ad(1200, A::offset(A::mul(A::offset(A::scale(s.ad_value(487), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(487), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(487), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_div_from_scalar(1201, 1.0, 1200);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_square(1198, 1201);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (!(s.v[1350] != 0.0))) {
            s.store_mul_ad_lhs(1198, A::offset(A::scale(A::sub(s.ad_value(487), s.ad_value(654)), s.v[371]), 1.0), 655);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (!(s.v[1350] != 0.0))) {
            s.store_sqrt(1201, 1198);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (!(s.v[1350] != 0.0))) {
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.v[1353] = if (s.v[487] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (s.v[1353] != 0.0)) {
            s.store_scale_ad(1202, A::ln(A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1200), 1.0), A::offset(s.ad_value(1200), 3.0))))), (s.v[370] * 2.0));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) && (!(s.v[1353] != 0.0))) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1201), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1201), 1.0), A::offset(A::scale(s.ad_value(1201), 3.0), 1.0))))), (s.v[370] * 2.0)), 487);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) {
            s.store_sub(1203, 656, 1202);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) {
            s.store_scale_ad(1204, A::sub(A::add(s.ad_value(487), s.ad_value(1203)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(1203)), A::sub(s.ad_value(487), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) {
            s.store_scale_ad(1205, A::sub(A::add(s.ad_value(487), s.ad_value(659)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(659)), A::sub(s.ad_value(487), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1349] != 0.0)) {
            s.store_scale_ad(1206, A::sub(s.ad_value(487), A::sqrt(A::offset(A::mul(s.ad_value(487), s.ad_value(487)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1354] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1354] != 0.0)) {
            s.store_scalar(1234, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.v[1355] = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (s.v[1355] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1356] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (s.v[1356] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p831)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1357] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (s.v[1357] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) && (!(s.v[1357] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) {
            s.store_scale(1214, 1207, s.v[423]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[384]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1355] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.v[1358] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (s.v[1358] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[408]), s.ad_value(1210)), s.v[438]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1359] = if (((-p.p831) * s.v[411]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1359] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[435]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[435])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1360] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1360] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1361] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1361] != 0.0))) {
            let assign20980_ad_e21933: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign20980_ad_e21933);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1362] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (s.v[1362] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1363] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1362] != 0.0))) && (s.v[1363] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1362] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) && (!(s.v[1362] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[435]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p845);
        }

        s.v[1364] = if (p.p851 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (s.v[1364] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1365] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1364] != 0.0))) && (s.v[1365] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1364] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1364] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[426]), s.ad_value(1207)), s.v[411]);
        }

        s.v[1366] = if (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1364] != 0.0))) && (s.v[1366] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(441)), s.ad_value(1232)));
        }

        s.v[1367] = if (((-s.v[441]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1364] != 0.0))) && (!(s.v[1366] != 0.0))) && (s.v[1367] != 0.0)) {
            let assign21170_ad_e22260: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign21170_ad_e22260);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1364] != 0.0))) && (!(s.v[1366] != 0.0))) && (!(s.v[1367] != 0.0))) {
            let assign21180_ad_e22310: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign21180_ad_e22310);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1364] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(487), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p851);
        }

        s.v[1368] = if (p.p860 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (s.v[1368] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1369] = if (s.v[1206] > ((-s.v[444]) * p.p860)) { 1.0 } else { 0.0 };

        s.v[1370] = if (p.p863 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1368] != 0.0))) && (s.v[1369] != 0.0)) && (s.v[1370] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[448]), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1368] != 0.0))) && (s.v[1369] != 0.0)) && (!(s.v[1370] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[448])), p.p863);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1368] != 0.0))) && (s.v[1369] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) && (!(s.v[1368] != 0.0))) && (!(s.v[1369] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p860)), s.v[451]), s.v[445]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1354] != 0.0))) {
            s.store_mul_ad_lhs(1234, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1371] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1371] != 0.0)) {
            s.store_scalar(1235, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.v[1372] = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (s.v[1372] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1373] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) && (s.v[1373] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p832)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1374] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) && (s.v[1374] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) && (!(s.v[1374] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) {
            s.store_scale(1214, 1207, s.v[424]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[385]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1372] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.v[1375] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (s.v[1375] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[409]), s.ad_value(1210)), s.v[439]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
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
        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1376] = if (((-p.p832) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (s.v[1376] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[436]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[436])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1377] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (s.v[1377] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1378] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (s.v[1378] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (!(s.v[1378] != 0.0))) {
            let assign21680_ad_e23076: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign21680_ad_e23076);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1379] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (s.v[1379] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1380] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (!(s.v[1379] != 0.0))) && (s.v[1380] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (!(s.v[1379] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) && (!(s.v[1379] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[436]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p846);
        }

        s.v[1381] = if (p.p852 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (s.v[1381] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1382] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1381] != 0.0))) && (s.v[1382] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1381] != 0.0))) && (!(s.v[1382] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1381] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[427]), s.ad_value(1207)), s.v[412]);
        }

        s.v[1383] = if (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1381] != 0.0))) && (s.v[1383] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(442)), s.ad_value(1232)));
        }

        s.v[1384] = if (((-s.v[442]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1381] != 0.0))) && (!(s.v[1383] != 0.0))) && (s.v[1384] != 0.0)) {
            let assign21870_ad_e23403: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign21870_ad_e23403);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1381] != 0.0))) && (!(s.v[1383] != 0.0))) && (!(s.v[1384] != 0.0))) {
            let assign21880_ad_e23453: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign21880_ad_e23453);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1381] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(487), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p852);
        }

        s.v[1385] = if (p.p861 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (s.v[1385] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1386] = if (s.v[1206] > ((-s.v[444]) * p.p861)) { 1.0 } else { 0.0 };

        s.v[1387] = if (p.p864 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1385] != 0.0))) && (s.v[1386] != 0.0)) && (s.v[1387] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[449]), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1385] != 0.0))) && (s.v[1386] != 0.0)) && (!(s.v[1387] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[449])), p.p864);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1385] != 0.0))) && (s.v[1386] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) && (!(s.v[1385] != 0.0))) && (!(s.v[1386] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p861)), s.v[452]), s.v[446]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1371] != 0.0))) {
            s.store_mul_ad_lhs(1235, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1388] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1388] != 0.0)) {
            s.store_scalar(1236, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.v[1389] = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (s.v[1389] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1390] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) && (s.v[1390] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) && (!(s.v[1390] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p833)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1391] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) && (s.v[1391] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[431]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) && (!(s.v[1391] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) {
            s.store_scale(1214, 1207, s.v[425]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[386]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1389] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.v[1392] = if (p.p847 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (s.v[1392] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[410]), s.ad_value(1210)), s.v[440]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1393] = if (((-p.p833) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (s.v[1393] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (!(s.v[1393] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[437]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[437])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1394] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (s.v[1394] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (!(s.v[1394] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1395] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (s.v[1395] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (!(s.v[1395] != 0.0))) {
            let assign22380_ad_e24219: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign22380_ad_e24219);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1396] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (s.v[1396] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1397] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (!(s.v[1396] != 0.0))) && (s.v[1397] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (!(s.v[1396] != 0.0))) && (!(s.v[1397] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) && (!(s.v[1396] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[437]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p847);
        }

        s.v[1398] = if (p.p853 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1399] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1398] != 0.0))) && (s.v[1399] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]), p.p833);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[428]), s.ad_value(1207)), s.v[413]);
        }

        s.v[1400] = if (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1398] != 0.0))) && (s.v[1400] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(443)), s.ad_value(1232)));
        }

        s.v[1401] = if (((-s.v[443]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1400] != 0.0))) && (s.v[1401] != 0.0)) {
            let assign22570_ad_e24546: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign22570_ad_e24546);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1400] != 0.0))) && (!(s.v[1401] != 0.0))) {
            let assign22580_ad_e24596: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign22580_ad_e24596);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(487), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p853);
        }

        s.v[1402] = if (p.p862 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (s.v[1402] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1403] = if (s.v[1206] > ((-s.v[444]) * p.p862)) { 1.0 } else { 0.0 };

        s.v[1404] = if (p.p865 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1403] != 0.0)) && (s.v[1404] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[450]), A::scale(s.ad_value(1206), s.v[450])), A::scale(s.ad_value(1206), s.v[450])), A::scale(s.ad_value(1206), s.v[450]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1403] != 0.0)) && (!(s.v[1404] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[450])), p.p865);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1403] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1403] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p862)), s.v[453]), s.v[447]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1388] != 0.0))) {
            s.store_mul_ad_lhs(1236, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_add_ad(477, A::add(A::mul(s.ad_value(646), s.ad_value(1234)), A::mul(s.ad_value(647), s.ad_value(1235))), A::mul(s.ad_value(648), s.ad_value(1236)));
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1205, 0.0);
        }

        if ((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) {
            s.store_scalar(1202, 0.0);
        }

        s.v[1405] = if !(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1406] = if (s.v[488] < s.v[654]) { 1.0 } else { 0.0 };

        s.v[1407] = if (((((-0.5) * (s.v[488] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (s.v[1406] != 0.0)) && (s.v[1407] != 0.0)) {
            s.store_exp_ad(1200, A::scale(s.ad_value(488), (s.v[371] * (-0.5))));
        }

        s.v[1408] = if (((-0.5) * (s.v[488] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (s.v[1406] != 0.0)) && (!(s.v[1407] != 0.0))) && (s.v[1408] != 0.0)) {
            let assign22840_ad_e24967: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1200, &assign22840_ad_e24967);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (s.v[1406] != 0.0)) && (!(s.v[1407] != 0.0))) && (!(s.v[1408] != 0.0))) {
            s.store_scale_ad(1200, A::offset(A::mul(A::offset(A::scale(s.ad_value(488), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(488), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(488), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_div_from_scalar(1201, 1.0, 1200);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (s.v[1406] != 0.0)) {
            s.store_square(1198, 1201);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (!(s.v[1406] != 0.0))) {
            s.store_mul_ad_lhs(1198, A::offset(A::scale(A::sub(s.ad_value(488), s.ad_value(654)), s.v[371]), 1.0), 655);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (!(s.v[1406] != 0.0))) {
            s.store_sqrt(1201, 1198);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (!(s.v[1406] != 0.0))) {
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.v[1409] = if (s.v[488] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (s.v[1409] != 0.0)) {
            s.store_scale_ad(1202, A::ln(A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1200), 1.0), A::offset(s.ad_value(1200), 3.0))))), (s.v[370] * 2.0));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) && (!(s.v[1409] != 0.0))) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1201), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1201), 1.0), A::offset(A::scale(s.ad_value(1201), 3.0), 1.0))))), (s.v[370] * 2.0)), 488);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_sub(1203, 656, 1202);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_scale_ad(1204, A::sub(A::add(s.ad_value(488), s.ad_value(1203)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(1203)), A::sub(s.ad_value(488), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_scale_ad(1205, A::sub(A::add(s.ad_value(488), s.ad_value(659)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(659)), A::sub(s.ad_value(488), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1405] != 0.0)) {
            s.store_scale_ad(1206, A::sub(s.ad_value(488), A::sqrt(A::offset(A::mul(s.ad_value(488), s.ad_value(488)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1410] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

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
        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_scalar(1234, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.v[1411] = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (s.v[1411] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1412] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) && (s.v[1412] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p831)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1413] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) && (!(s.v[1413] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) {
            s.store_scale(1214, 1207, s.v[423]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[384]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1411] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.v[1414] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (s.v[1414] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[408]), s.ad_value(1210)), s.v[438]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1415] = if (((-p.p831) * s.v[411]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (s.v[1415] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[435]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[435])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1416] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (s.v[1416] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1417] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (s.v[1417] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (!(s.v[1417] != 0.0))) {
            let assign23380_ad_e25863: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign23380_ad_e25863);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1418] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (s.v[1418] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1419] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (!(s.v[1418] != 0.0))) && (s.v[1419] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (!(s.v[1418] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) && (!(s.v[1418] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[435]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p845);
        }

        s.v[1420] = if (p.p851 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1421] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1420] != 0.0))) && (s.v[1421] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1420] != 0.0))) && (!(s.v[1421] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]), p.p831);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1420] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[426]), s.ad_value(1207)), s.v[411]);
        }

        s.v[1422] = if (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1420] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(441)), s.ad_value(1232)));
        }

        s.v[1423] = if (((-s.v[441]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1420] != 0.0))) && (!(s.v[1422] != 0.0))) && (s.v[1423] != 0.0)) {
            let assign23570_ad_e26190: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign23570_ad_e26190);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1420] != 0.0))) && (!(s.v[1422] != 0.0))) && (!(s.v[1423] != 0.0))) {
            let assign23580_ad_e26240: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign23580_ad_e26240);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1420] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(488), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p851);
        }

        s.v[1424] = if (p.p860 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (s.v[1424] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1425] = if (s.v[1206] > ((-s.v[444]) * p.p860)) { 1.0 } else { 0.0 };

        s.v[1426] = if (p.p863 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1424] != 0.0))) && (s.v[1425] != 0.0)) && (s.v[1426] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[448]), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448])), A::scale(s.ad_value(1206), s.v[448]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1424] != 0.0))) && (s.v[1425] != 0.0)) && (!(s.v[1426] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[448])), p.p863);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1424] != 0.0))) && (s.v[1425] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) && (!(s.v[1424] != 0.0))) && (!(s.v[1425] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p860)), s.v[451]), s.v[445]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1410] != 0.0))) {
            s.store_mul_ad_lhs(1234, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1427] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1427] != 0.0)) {
            s.store_scalar(1235, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.v[1428] = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (s.v[1428] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.v[1429] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) && (s.v[1429] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_scale_ad(1212, A::add(A::div(A::mul(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211))), A::sub_from_scalar(1.0, s.ad_value(1211))), s.ad_value(1211)), (1.0 - (2.0 * p.p832)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_add(1213, 1211, 1212);
        }

        s.v[1430] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) && (s.v[1430] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(s.ad_value(1210), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) && (!(s.v[1430] != 0.0))) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_scale(1214, 1207, s.v[424]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::offset(s.ad_value(1201), (-1.0)), s.ad_value(1214)), s.v[385]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1428] != 0.0))) {
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.v[1431] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (s.v[1431] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_scale_ad(1217, A::div(A::scale(s.ad_value(1214), s.v[409]), s.ad_value(1210)), s.v[439]);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_square(1219, 1218);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_sqrt_ad(1220, A::div(A::square(s.ad_value(1219)), A::offset(A::square(s.ad_value(1219)), 1.0)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_sqrt(1221, 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_mul(1222, 1220, 1221);
        }

        s.v[1432] = if (((-p.p832) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (s.v[1432] != 0.0)) {
            s.store_div_from_scalar_ad(1223, 1.0, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_div_ad(1224, A::mul(s.ad_value(1213), s.ad_value(1223)), A::add(s.ad_value(1213), s.ad_value(1223)));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_sqrt_ad(1225, A::scale(A::div(s.ad_value(1217), s.ad_value(1221)), 0.375));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_sub_ad_lhs(1226, A::scale(A::mul(s.ad_value(1218), s.ad_value(1221)), 2.0), 1220);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_add_ad(1227, A::sub(A::mul(A::scale(s.ad_value(1218), s.v[436]), s.ad_value(1221)), A::scale(s.ad_value(1220), s.v[436])), A::scale(A::mul(s.ad_value(1217), s.ad_value(1222)), 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_mul_ad_lhs(1228, A::offset(s.ad_value(1226), (-1.0)), 1225);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_square(1189, 1228);
        }

        s.v[1433] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (s.v[1433] != 0.0)) {
            s.store_div_from_scalar_ad(1190, 1.0, A::offset(A::scale(s.ad_value(1228), s.v[372]), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_div_from_scalar_ad(1190, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1228), s.v[372])));
        }

        s.v[1434] = if (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (s.v[1434] != 0.0)) {
            s.store_exp_ad(1207, A::sub(s.ad_value(1227), s.ad_value(1189)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (!(s.v[1434] != 0.0))) {
            let assign24080_ad_e27006: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1207, &assign24080_ad_e27006);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_mul_ad_lhs(1191, A::add(A::add(A::scale(s.ad_value(1190), 0.29214664), A::scale(A::square(s.ad_value(1190)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(1190)), s.ad_value(1190)), s.v[374])), 1207);
        }

        s.v[1435] = if (s.v[1228] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (s.v[1435] != 0.0)) {
            s.copy_ad(1229, 1191);
        }

        s.v[1436] = if (s.v[1227] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (!(s.v[1435] != 0.0))) && (s.v[1436] != 0.0)) {
            s.store_exp(1207, 1227);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (!(s.v[1435] != 0.0))) && (!(s.v[1436] != 0.0))) {
            s.store_div_from_scalar_ad(1207, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1227)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) && (!(s.v[1435] != 0.0))) {
            s.store_sub_ad_lhs(1229, A::scale(s.ad_value(1207), 2.0), 1191);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_scale_ad(1230, A::div(A::scale(s.ad_value(1229), s.v[436]), s.ad_value(1225)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_scale_ad(1216, A::mul(A::mul(s.ad_value(1215), s.ad_value(1230)), s.ad_value(1224)), p.p846);
        }

        s.v[1437] = if (p.p852 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (s.v[1437] != 0.0)) {
            s.store_scalar(1231, 0.0);
        }

        s.v[1438] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1437] != 0.0))) && (s.v[1438] != 0.0)) {
            s.store_sqrt_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1437] != 0.0))) && (!(s.v[1438] != 0.0))) {
            s.store_powf_ad(1207, A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]), p.p832);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1437] != 0.0))) {
            s.store_scale_ad(1232, A::div(A::scale(A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[427]), s.ad_value(1207)), s.v[412]);
        }

        s.v[1439] = if (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1437] != 0.0))) && (s.v[1439] != 0.0)) {
            s.store_exp_ad(1207, A::div(A::neg(s.ad_value(442)), s.ad_value(1232)));
        }

        s.v[1440] = if (((-s.v[442]) / s.v[1232]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1437] != 0.0))) && (!(s.v[1439] != 0.0))) && (s.v[1440] != 0.0)) {
            let assign24270_ad_e27333: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1232))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1207, 1e-100, assign24270_ad_e27333);
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1437] != 0.0))) && (!(s.v[1439] != 0.0))) && (!(s.v[1440] != 0.0))) {
            let assign24280_ad_e27383: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1232)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1207, &assign24280_ad_e27383);
        }

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1437] != 0.0))) {
            s.store_scale_ad(1231, A::mul(A::mul(A::mul(s.ad_value(488), s.ad_value(1232)), s.ad_value(1232)), s.ad_value(1207)), p.p852);
        }

        s.v[1441] = if (p.p861 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (s.v[1441] != 0.0)) {
            s.store_scalar(1233, 1.0);
        }

        s.v[1442] = if (s.v[1206] > ((-s.v[444]) * p.p861)) { 1.0 } else { 0.0 };

        s.v[1443] = if (p.p864 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1441] != 0.0))) && (s.v[1442] != 0.0)) && (s.v[1443] != 0.0)) {
            s.store_mul_ad(1207, A::mul(A::mul(A::scale(s.ad_value(1206), s.v[449]), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449])), A::scale(s.ad_value(1206), s.v[449]));
        }

        if ((((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1441] != 0.0))) && (s.v[1442] != 0.0)) && (!(s.v[1443] != 0.0))) {
            s.store_powf_ad(1207, A::abs(A::scale(s.ad_value(1206), s.v[449])), p.p864);
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1441] != 0.0))) && (s.v[1442] != 0.0)) {
            s.store_div_from_scalar_ad(1233, 1.0, A::sub_from_scalar(1.0, s.ad_value(1207)));
        }

        if (((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) && (!(s.v[1441] != 0.0))) && (!(s.v[1442] != 0.0))) {
            s.store_offset_ad(1233, A::scale(A::offset(s.ad_value(1206), (s.v[444] * p.p861)), s.v[452]), s.v[446]);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1427] != 0.0))) {
            s.store_mul_ad_lhs(1235, A::scale(A::add(A::add(A::add(s.ad_value(1208), s.ad_value(1209)), s.ad_value(1216)), s.ad_value(1231)), p.p29), 1233);
        }

        s.v[1444] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (s.v[1444] != 0.0)) {
            s.store_scalar(1236, 0.0);
        }

        if (((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1444] != 0.0))) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.v[1445] = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1171] != 0.0) && (s.v[1188] != 0.0)) && (!(s.v[1444] != 0.0))) && (s.v[1445] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

    }
}
