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
        s.v[981] = if (p.p37 >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[981] != 0.0) {
            s.store_scalar(0, 1.0);
        }

        if (!(s.v[981] != 0.0)) {
            s.store_scalar(0, (-1.0));
        }

        s.v[756] = (8.8541878176e-12 * 11.8);

        s.v[351] = (273.15 + p.p38);

        s.v[475] = 0.0;

        s.v[982] = if (p.p944 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[982] != 0.0) {
            s.store_scalar(475, 1.0);
        }

        if (!(s.v[982] != 0.0)) {
            s.store_scalar(475, 0.0);
        }

        s.v[365] = (273.15 + p.p840);

        s.v[368] = (1.3806505e-23 / 1.6021918e-19);

        s.v[369] = (s.v[368] * s.v[365]);

        s.v[370] = (1.0 / s.v[369]);

        s.v[376] = ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365]));

        s.v[379] = (p.p851 + s.v[376]);

        s.v[380] = (p.p852 + s.v[376]);

        s.v[381] = (p.p853 + s.v[376]);

        s.v[409] = (1.0 - p.p848);

        s.v[410] = (1.0 - p.p849);

        s.v[411] = (1.0 - p.p850);

        s.v[412] = (1.0 / s.v[409]);

        s.v[413] = (1.0 / s.v[410]);

        s.v[414] = (1.0 / s.v[411]);

        s.v[424] = (s.v[756] / p.p842);

        s.v[425] = ((p.p860 * s.v[756]) / p.p843);

        s.v[426] = ((p.p861 * s.v[756]) / p.p844);

        s.v[427] = (1.0 / s.v[424]);

        s.v[428] = (1.0 / s.v[425]);

        s.v[429] = (1.0 / s.v[426]);

        s.v[430] = (1.0 / p.p845);

        s.v[431] = (1.0 / p.p846);

        s.v[432] = (1.0 / p.p847);

        s.v[373] = (1.772453850905516 * 0.29214664);

        s.v[374] = (((((-5.0) * 0.29214664) + 6.0) - ((s.v[373]) as f64).powf((-2.0))) / 3.0);

        s.v[375] = ((1.0 - 0.29214664) - s.v[374]);

        s.v[445] = (1.0 - (1.0 / p.p841));

        s.v[446] = (1.0 / (1.0 - ((s.v[445]) as f64).powf(p.p880)));

        s.v[447] = (1.0 / (1.0 - ((s.v[445]) as f64).powf(p.p881)));

        s.v[448] = (1.0 / (1.0 - ((s.v[445]) as f64).powf(p.p882)));

        s.v[449] = (1.0 / p.p877);

        s.v[450] = (1.0 / p.p878);

        s.v[451] = (1.0 / p.p879);

        s.v[452] = (((-((s.v[446] * s.v[446]) * ((s.v[445]) as f64).powf((p.p880 - 1.0)))) * p.p880) * s.v[449]);

        s.v[453] = (((-((s.v[447] * s.v[447]) * ((s.v[445]) as f64).powf((p.p881 - 1.0)))) * p.p881) * s.v[450]);

        s.v[454] = (((-((s.v[448] * s.v[448]) * ((s.v[445]) as f64).powf((p.p882 - 1.0)))) * p.p882) * s.v[451]);

        s.v[983] = if ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[983] != 0.0) {
            s.store_scalar(474, 1.0);
        }

        if (!(s.v[983] != 0.0)) {
            s.store_scalar(474, 0.0);
        }

        s.v[984] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[984] != 0.0) {
            s.store_scalar(458, (if ((p.p844 * p.p883) > 1e-18) { (p.p844 * p.p883) } else { 1e-18 }));
        }

        if (s.v[984] != 0.0) {
            s.store_scalar(459, (if ((p.p847 * p.p884) > 0.05) { (p.p847 * p.p884) } else { 0.05 }));
        }

        if (s.v[984] != 0.0) {
            s.store_scalar(460, (if ((if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) < 0.95) { (if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) } else { 0.95 }));
        }

        if (s.v[984] != 0.0) {
            s.store_scalar(461, (p.p853 * p.p886));
        }

        if (s.v[984] != 0.0) {
            s.store_offset(463, 461, s.v[376]);
        }

        if (s.v[984] != 0.0) {
            s.store_sub_from_scalar(468, 1.0, 460);
        }

        if (s.v[984] != 0.0) {
            s.store_div_from_scalar(469, 1.0, 468);
        }

        s.v[985] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[985] != 0.0) {
            s.store_scalar(506, p.p842);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(507, p.p843);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(508, p.p844);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(509, p.p845);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(510, p.p846);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(511, p.p847);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(512, p.p848);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(513, p.p849);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(514, p.p850);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(515, p.p851);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(516, p.p852);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(517, p.p853);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(518, p.p854);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(519, p.p855);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(520, p.p856);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(523, p.p857);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(524, p.p858);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(525, p.p859);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(521, p.p860);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(522, p.p861);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(526, p.p862);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(527, p.p863);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(528, p.p864);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(529, p.p865);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(530, p.p866);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(531, p.p867);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(532, p.p868);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(533, p.p869);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(534, p.p870);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(535, p.p871);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(536, p.p872);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(537, p.p873);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(538, p.p874);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(539, p.p875);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(540, p.p876);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(541, p.p877);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(542, p.p878);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(543, p.p879);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(544, p.p880);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(545, p.p881);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(546, p.p882);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(553, p.p945);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(554, p.p946);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(637, p.p889);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(638, p.p890);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(639, p.p891);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(640, p.p892);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(547, p.p883);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(548, p.p884);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(549, p.p885);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(550, p.p886);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(551, p.p887);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(552, p.p888);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(506, p.p893);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(507, p.p894);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(508, p.p895);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(509, p.p896);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(510, p.p897);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(511, p.p898);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(512, p.p899);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(513, p.p900);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(514, p.p901);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(515, p.p902);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(516, p.p903);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(517, p.p904);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(518, p.p905);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(519, p.p906);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(520, p.p907);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(523, p.p908);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(524, p.p909);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(525, p.p910);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(521, p.p911);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(522, p.p912);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(526, p.p913);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(527, p.p914);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(528, p.p915);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(529, p.p916);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(530, p.p917);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(531, p.p918);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(532, p.p919);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(533, p.p920);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(534, p.p921);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(535, p.p922);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(536, p.p923);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(537, p.p924);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(538, p.p925);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(539, p.p926);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(540, p.p927);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(541, p.p928);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(542, p.p929);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(543, p.p930);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(544, p.p931);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(545, p.p932);
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
        if (!(s.v[985] != 0.0)) {
            s.store_scalar(546, p.p933);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(553, p.p947);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(554, p.p948);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(637, p.p940);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(638, p.p941);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(639, p.p942);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(640, p.p943);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(547, p.p934);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(548, p.p935);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(549, p.p936);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(550, p.p937);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(551, p.p938);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(552, p.p939);
        }

        s.store_offset(555, 515, s.v[376]);

        s.store_offset(556, 516, s.v[376]);

        s.store_offset(557, 517, s.v[376]);

        s.store_sub_from_scalar(576, 1.0, 512);

        s.store_sub_from_scalar(577, 1.0, 513);

        s.store_sub_from_scalar(578, 1.0, 514);

        s.store_div_from_scalar(579, 1.0, 576);

        s.store_div_from_scalar(580, 1.0, 577);

        s.store_div_from_scalar(581, 1.0, 578);

        s.store_div_from_scalar(591, s.v[756], 506);

        s.store_div_ad_lhs(592, A::scale(s.ad_value(521), s.v[756]), 507);

        s.store_div_ad_lhs(593, A::scale(s.ad_value(522), s.v[756]), 508);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 593);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar(599, 1.0, 511);

        s.store_div_from_scalar_ad(612, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[445], s.ad_value(544))));

        s.store_div_from_scalar_ad(613, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[445], s.ad_value(545))));

        s.store_div_from_scalar_ad(614, 1.0, A::sub_from_scalar(1.0, A::pow_from_scalar(s.v[445], s.ad_value(546))));

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.store_div_from_scalar(617, 1.0, 543);

        s.store_mul_ad_lhs(618, A::mul(A::neg(A::mul(A::square(s.ad_value(612)), A::pow_from_scalar(s.v[445], A::offset(s.ad_value(544), (-1.0))))), s.ad_value(544)), 615);

        s.store_mul_ad_lhs(619, A::mul(A::neg(A::mul(A::square(s.ad_value(613)), A::pow_from_scalar(s.v[445], A::offset(s.ad_value(545), (-1.0))))), s.ad_value(545)), 616);

        s.store_mul_ad_lhs(620, A::mul(A::neg(A::mul(A::square(s.ad_value(614)), A::pow_from_scalar(s.v[445], A::offset(s.ad_value(546), (-1.0))))), s.ad_value(546)), 617);

        s.v[986] = if ((((s.v[547] != 1.0) || (s.v[548] != 1.0)) || (s.v[549] != 1.0)) || (s.v[550] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[986] != 0.0) {
            s.store_scalar(636, 1.0);
        }

        if (!(s.v[986] != 0.0)) {
            s.store_scalar(636, 0.0);
        }

        s.v[987] = if (s.v[636] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[987] != 0.0) {
            s.store_ad(621, &{
                if ((s.v[508] * s.v[547]) > 1e-18) {
                    A::mul(s.ad_value(508), s.ad_value(547))
                } else {
                    A::constant(1e-18)
                }
            });
        }

        if (s.v[987] != 0.0) {
            s.store_ad(622, &{
                if ((s.v[511] * s.v[548]) > 0.05) {
                    A::mul(s.ad_value(511), s.ad_value(548))
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[987] != 0.0) {
            s.store_ad(623, &{
                if ((if ((s.v[514] * s.v[549]) > 0.05) { (s.v[514] * s.v[549]) } else { 0.05 }) < 0.95) {
                    {
                        if ((s.v[514] * s.v[549]) > 0.05) {
                            A::mul(s.ad_value(514), s.ad_value(549))
                        } else {
                            A::constant(0.05)
                        }
                    }
                } else {
                    A::constant(0.95)
                }
            });
        }

        if (s.v[987] != 0.0) {
            s.store_mul(624, 517, 550);
        }

        if (s.v[987] != 0.0) {
            s.store_offset(626, 624, s.v[376]);
        }

        if (s.v[987] != 0.0) {
            s.store_sub_from_scalar(631, 1.0, 623);
        }

        if (s.v[987] != 0.0) {
            s.store_div_from_scalar(632, 1.0, 631);
        }

        s.v[867] = 0.0;

        s.v[352] = ((ctx.temperature() + p.p55) + p.p35);

        s.v[353] = (s.v[352] / s.v[351]);

        s.v[354] = (s.v[352] - s.v[351]);

        s.v[355] = ((s.v[352] * 1.3806505e-23) / 1.6021918e-19);

        s.v[356] = (1.0 / s.v[355]);

        s.v[366] = (((ctx.temperature() + p.p55) + p.p35)).max((273.15 + (-250.0)));

        s.v[367] = (s.v[366] / s.v[365]);

        s.v[371] = (s.v[368] * s.v[366]);

        s.v[372] = (1.0 / s.v[371]);

        s.v[377] = ((-((0.000702 * s.v[366]) * s.v[366])) / (1108.0 + s.v[366]));

        s.v[382] = (p.p851 + s.v[377]);

        s.v[383] = (p.p852 + s.v[377]);

        s.v[384] = (p.p853 + s.v[377]);

        s.v[385] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[370]) - (s.v[382] * s.v[372])))) as f64).exp());

        s.v[386] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[370]) - (s.v[383] * s.v[372])))) as f64).exp());

        s.v[387] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[381] * s.v[370]) - (s.v[384] * s.v[372])))) as f64).exp());

        s.v[388] = ((p.p854 * s.v[385]) * s.v[385]);

        s.v[389] = ((p.p855 * s.v[386]) * s.v[386]);

        s.v[390] = ((p.p856 * s.v[387]) * s.v[387]);

        s.v[391] = ((p.p845 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[385]) as f64).ln()));

        s.v[392] = ((p.p846 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[386]) as f64).ln()));

        s.v[393] = ((p.p847 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[387]) as f64).ln()));

        s.v[394] = (s.v[391] + (s.v[371] * (((1.0 + ((((0.05 - s.v[391]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[395] = (s.v[392] + (s.v[371] * (((1.0 + ((((0.05 - s.v[392]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[396] = (s.v[393] + (s.v[371] * (((1.0 + ((((0.05 - s.v[393]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[406] = (1.0 / s.v[394]);

        s.v[407] = (1.0 / s.v[395]);

        s.v[408] = (1.0 / s.v[396]);

        s.v[415] = (p.p842 * (((p.p845 * s.v[406])) as f64).powf(p.p848));

        s.v[416] = (p.p843 * (((p.p846 * s.v[407])) as f64).powf(p.p849));

        s.v[417] = (p.p844 * (((p.p847 * s.v[408])) as f64).powf(p.p850));

        s.v[418] = ((s.v[415] * s.v[394]) * s.v[412]);

        s.v[419] = ((s.v[416] * s.v[395]) * s.v[413]);

        s.v[420] = ((s.v[417] * s.v[396]) * s.v[414]);

        s.v[421] = (2.0 * s.v[415]);

        s.v[422] = (2.0 * s.v[416]);

        s.v[423] = (2.0 * s.v[417]);

        s.v[433] = ((0.5 * s.v[382])).max(s.v[371]);

        s.v[434] = ((0.5 * s.v[383])).max(s.v[371]);

        s.v[435] = ((0.5 * s.v[384])).max(s.v[371]);

        s.v[436] = (s.v[433] * s.v[372]);

        s.v[437] = (s.v[434] * s.v[372]);

        s.v[438] = (s.v[435] * s.v[372]);

        s.v[439] = (((((((32.0 * p.p865) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[440] = (((((((32.0 * p.p866) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[441] = (((((((32.0 * p.p867) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[435] * s.v[435]) * s.v[435]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[442] = (p.p871 * (1.0 + (p.p874 * (s.v[366] - s.v[365]))));

        s.v[443] = (p.p872 * (1.0 + (p.p875 * (s.v[366] - s.v[365]))));

        s.v[444] = (p.p873 * (1.0 + (p.p876 * (s.v[366] - s.v[365]))));

        if !(s.v[442] > 0.0) {
            s.store_scalar(442, 0.0);
        }

        if !(s.v[443] > 0.0) {
            s.store_scalar(443, 0.0);
        }

        if !(s.v[444] > 0.0) {
            s.store_scalar(444, 0.0);
        }

        s.v[1007] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1007] != 0.0) {
            s.store_offset(462, 461, s.v[377]);
        }

        if (s.v[1007] != 0.0) {
            s.store_scale_ad(464, A::exp(A::scale(A::sub(A::scale(s.ad_value(463), s.v[370]), A::scale(s.ad_value(462), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));
        }

        if (s.v[1007] != 0.0) {
            s.store_sub_ad(465, A::scale(s.ad_value(459), s.v[367]), A::scale(A::ln(s.ad_value(464)), (2.0 * s.v[371])));
        }

        if (s.v[1007] != 0.0) {
            s.store_add_ad_rhs(466, 465, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(465)), s.v[372])), 1.0)), s.v[371]));
        }

        if (s.v[1007] != 0.0) {
            s.store_div_from_scalar(467, 1.0, 466);
        }

        if (s.v[1007] != 0.0) {
            s.store_mul_ad_rhs(470, 458, A::pow(A::mul(s.ad_value(459), s.ad_value(467)), s.ad_value(460)));
        }

        if (s.v[1007] != 0.0) {
            s.store_mul_ad_lhs(471, A::mul(s.ad_value(470), s.ad_value(466)), 469);
        }

        if (s.v[1007] != 0.0) {
            s.store_scale(472, 470, 2.0);
        }

        s.store_offset(558, 515, s.v[377]);

        s.store_offset(559, 516, s.v[377]);

        s.store_offset(560, 517, s.v[377]);

        s.store_scale_ad(561, A::exp(A::scale(A::sub(A::scale(s.ad_value(555), s.v[370]), A::scale(s.ad_value(558), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp(A::scale(A::sub(A::scale(s.ad_value(556), s.v[370]), A::scale(s.ad_value(559), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(563, A::exp(A::scale(A::sub(A::scale(s.ad_value(557), s.v[370]), A::scale(s.ad_value(560), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));

        s.store_mul_ad_lhs(564, A::mul(s.ad_value(518), s.ad_value(561)), 561);

        s.store_mul_ad_lhs(565, A::mul(s.ad_value(519), s.ad_value(562)), 562);

        s.store_mul_ad_lhs(566, A::mul(s.ad_value(520), s.ad_value(563)), 563);

        s.store_sub_ad(567, A::scale(s.ad_value(509), s.v[367]), A::scale(A::ln(s.ad_value(561)), (2.0 * s.v[371])));

        s.store_sub_ad(568, A::scale(s.ad_value(510), s.v[367]), A::scale(A::ln(s.ad_value(562)), (2.0 * s.v[371])));

        s.store_sub_ad(569, A::scale(s.ad_value(511), s.v[367]), A::scale(A::ln(s.ad_value(563)), (2.0 * s.v[371])));

        s.store_add_ad_rhs(570, 567, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(567)), s.v[372])), 1.0)), s.v[371]));

        s.store_add_ad_rhs(571, 568, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(568)), s.v[372])), 1.0)), s.v[371]));

        s.store_add_ad_rhs(572, 569, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(569)), s.v[372])), 1.0)), s.v[371]));

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(575, 1.0, 572);

        s.store_mul_ad_rhs(582, 506, A::pow(A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512)));

        s.store_mul_ad_rhs(583, 507, A::pow(A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513)));

        s.store_mul_ad_rhs(584, 508, A::pow(A::mul(s.ad_value(511), s.ad_value(575)), s.ad_value(514)));

        s.store_mul_ad_lhs(585, A::mul(s.ad_value(582), s.ad_value(570)), 579);

        s.store_mul_ad_lhs(586, A::mul(s.ad_value(583), s.ad_value(571)), 580);

        s.store_mul_ad_lhs(587, A::mul(s.ad_value(584), s.ad_value(572)), 581);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_scale(590, 584, 2.0);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(602, A::scale(s.ad_value(560), 0.5), s.v[371]);

        s.store_scale(603, 600, s.v[372]);

        s.store_scale(604, 601, s.v[372]);

        s.store_scale(605, 602, s.v[372]);

        s.store_scale_ad(606, A::sqrt(A::mul(A::scale(s.ad_value(529), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(600)), s.ad_value(600)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(607, A::sqrt(A::mul(A::scale(s.ad_value(530), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(601)), s.ad_value(601)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(608, A::sqrt(A::mul(A::scale(s.ad_value(531), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(602)), s.ad_value(602)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_ad_rhs(609, 535, A::offset(A::scale(s.ad_value(538), (s.v[366] - s.v[365])), 1.0));

        s.store_mul_ad_rhs(610, 536, A::offset(A::scale(s.ad_value(539), (s.v[366] - s.v[365])), 1.0));

        s.store_mul_ad_rhs(611, 537, A::offset(A::scale(s.ad_value(540), (s.v[366] - s.v[365])), 1.0));

        if !(s.v[609] > 0.0) {
            s.store_scalar(609, 0.0);
        }

        if !(s.v[610] > 0.0) {
            s.store_scalar(610, 0.0);
        }

        if !(s.v[611] > 0.0) {
            s.store_scalar(611, 0.0);
        }

        s.v[1008] = if (s.v[636] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1008] != 0.0) {
            s.store_offset(625, 624, s.v[377]);
        }

        if (s.v[1008] != 0.0) {
            s.store_scale_ad(627, A::exp(A::scale(A::sub(A::scale(s.ad_value(626), s.v[370]), A::scale(s.ad_value(625), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));
        }

        if (s.v[1008] != 0.0) {
            s.store_sub_ad(628, A::scale(s.ad_value(622), s.v[367]), A::scale(A::ln(s.ad_value(627)), (2.0 * s.v[371])));
        }

        if (s.v[1008] != 0.0) {
            s.store_add_ad_rhs(629, 628, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(628)), s.v[372])), 1.0)), s.v[371]));
        }

        if (s.v[1008] != 0.0) {
            s.store_div_from_scalar(630, 1.0, 629);
        }

        if (s.v[1008] != 0.0) {
            s.store_mul_ad_rhs(633, 621, A::pow(A::mul(s.ad_value(622), s.ad_value(630)), s.ad_value(623)));
        }

        if (s.v[1008] != 0.0) {
            s.store_mul_ad_lhs(634, A::mul(s.ad_value(633), s.ad_value(629)), 632);
        }

        if (s.v[1008] != 0.0) {
            s.store_scale(635, 633, 2.0);
        }

        s.v[1] = 1.0;

        s.v[2] = 1.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

        s.v[3] = p.p0;

        s.v[4] = p.p1;

        s.v[5] = p.p2;

        s.v[6] = p.p3;

        s.v[7] = p.p4;

        s.v[8] = p.p8;

        s.v[9] = p.p11;

        s.v[647] = p.p19;

        s.v[648] = p.p20;

        s.v[649] = p.p21;

        s.v[674] = p.p22;

        s.v[675] = p.p23;

        s.v[676] = p.p24;

        s.v[650] = p.p25;

        s.v[651] = p.p26;

        s.v[677] = p.p27;

        s.v[678] = p.p28;

        s.v[10] = p.p14;

        s.v[1009] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1009] != 0.0) {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if (s.v[1009] != 0.0) {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
        }

        if (s.v[1009] != 0.0) {
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
        s.v[308] = (1e-6 / s.v[3]);

        s.store_div_from_scalar(309, 1e-6, 4);

        s.store_scale_ad(310, A::offset(A::scale(s.ad_value(309), p.p191), 1.0), (p.p189 * (1.0 + (p.p190 * s.v[308]))));

        s.store_scale_ad(311, A::offset(A::scale(s.ad_value(309), p.p195), 1.0), (p.p193 * (1.0 + (p.p194 * s.v[308]))));

        if (((s.v[3] + s.v[310]) - (2.0 * p.p192)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[3]) + ((-(2.0 * p.p192)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[4] + s.v[311]) - (2.0 * p.p196)) > 1e-9) {
            s.store_offset_ad(313, A::add(s.ad_value(4), s.ad_value(311)), (-(2.0 * p.p196)));
        } else {
            s.store_scalar(313, 1e-9);
        }

        s.store_div_from_scalar(314, 1e-6, 312);

        s.store_square(315, 314);

        s.store_div_from_scalar(316, 1e-6, 313);

        s.store_div_from_scalar(317, 1.0, 316);

        s.store_mul(318, 314, 316);

        s.store_div_from_scalar(319, 1.0, 318);

        if ((((s.v[3] + s.v[310]) - (2.0 * p.p192)) + p.p197) > 1e-9) {
            s.store_offset_ad(320, A::offset(A::offset(s.ad_value(310), s.v[3]), (-(2.0 * p.p192))), p.p197);
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[4] + s.v[311]) - (2.0 * p.p196)) + p.p198) > 1e-9) {
            s.store_offset_ad(321, A::offset(A::add(s.ad_value(4), s.ad_value(311)), (-(2.0 * p.p196))), p.p198);
        } else {
            s.store_scalar(321, 1e-9);
        }

        s.store_scale(322, 321, 1000000.0);

        if (((s.v[3] + s.v[310]) + p.p197) > 1e-9) {
            s.store_offset(323, 310, ((s.v[3]) + (p.p197)));
        } else {
            s.store_scalar(323, 1e-9);
        }

        if (((s.v[4] + s.v[311]) + p.p198) > 1e-9) {
            s.store_offset_ad(324, A::add(s.ad_value(4), s.ad_value(311)), p.p198);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.store_scale(325, 323, 1000000.0);

        s.store_scale(326, 324, 1000000.0);

        if ((s.v[3] + s.v[310]) > 1e-9) {
            s.store_offset(327, 310, s.v[3]);
        } else {
            s.store_scalar(327, 1e-9);
        }

        if ((s.v[327] + p.p444) > 1e-9) {
            s.store_offset(328, 327, p.p444);
        } else {
            s.store_scalar(328, 1e-9);
        }

        if ((s.v[4] + s.v[311]) > 1e-9) {
            s.store_add(329, 4, 311);
        } else {
            s.store_scalar(329, 1e-9);
        }

        if ((s.v[9] - (0.5 * s.v[311])) > 1e-9) {
            s.store_sub_from_scalar_ad(330, s.v[9], A::scale(s.ad_value(311), 0.5));
        } else {
            s.store_scalar(330, 1e-9);
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

        s.v[1010] = if (if self.param_given[121] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1010] != 0.0) {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.v[1011] = if (if self.param_given[122] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1011] != 0.0) {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.v[1012] = if (if self.param_given[123] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1012] != 0.0) {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.v[1013] = if (if self.param_given[124] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1013] != 0.0) {
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

        s.v[1014] = if (if self.param_given[137] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1014] != 0.0) {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.v[1015] = if (if self.param_given[138] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1015] != 0.0) {
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

        s.v[172] = p.p186;

        s.v[173] = p.p187;

        s.v[174] = p.p188;

        s.v[1016] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1016] != 0.0) {
            s.store_add_ad(40, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p201), p.p200), p.p199), A::scale(s.ad_value(316), p.p202)), A::scale(s.ad_value(318), p.p203));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(314), p.p205), p.p204), A::scale(s.ad_value(316), p.p206)), A::scale(s.ad_value(318), p.p207));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(42, p.p208);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(43, p.p209);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(44, p.p210);
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(331, &A::scale({
                if ((1.0 + ((p.p212 * s.v[316]) * (((1.0 + (s.v[313] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p212), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p213)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(332, &A::scale({
                if ((1.0 + ((p.p215 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p215), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p216)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(333, &A::scale({
                if ((1.0 + ((p.p218 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p218), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p216)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p217));
        }

        s.v[1017] = if (s.v[312] > (2.0 * s.v[333])) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_scalar(334, 75000000000.0);
        }

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_sub_ad(335, A::sqrt(A::add(s.ad_value(331), A::scale(s.ad_value(332), 0.5))), A::sqrt(s.ad_value(331)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_add_ad(336, A::sqrt(s.ad_value(331)), A::mul(s.ad_value(334), A::ln(A::offset(A::mul(A::div(A::scale(s.ad_value(333), 2.0), s.ad_value(312)), A::offset(A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0))), 1.0))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_square(336, 336);
        }

        s.v[1018] = if (s.v[312] >= s.v[333]) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (!(s.v[1017] != 0.0))) && (s.v[1018] != 0.0)) {
            s.store_add_ad_rhs(336, 331, A::div(A::mul(s.ad_value(332), s.ad_value(333)), s.ad_value(312)));
        }

        if (((s.v[1016] != 0.0) && (!(s.v[1017] != 0.0))) && (!(s.v[1018] != 0.0))) {
            s.store_add_ad_rhs(336, 331, A::mul(s.ad_value(332), A::sub_from_scalar(2.0, A::div(s.ad_value(312), s.ad_value(333)))));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad_rhs(45, 336, A::sub(A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p219)), A::scale(s.ad_value(315), p.p220)));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(46, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p223), p.p222), p.p221), A::scale(s.ad_value(316), p.p224)), A::scale(s.ad_value(318), p.p225));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(47, p.p226);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(48, p.p227);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(49, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p230), p.p229), p.p228), A::scale(s.ad_value(316), p.p231)), A::scale(s.ad_value(318), p.p232));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(50, &A::scale({
                if (1e-6 > (1.0 + (p.p234 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::offset(A::scale(s.ad_value(314), p.p234), 1.0)
                }
            }, p.p233));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(55, p.p235);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(56, p.p236);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(57, p.p239);
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
        if (s.v[1016] != 0.0) {
            s.store_scalar(58, p.p240);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(51, A::mul(A::offset(A::scale(A::powf(s.ad_value(314), p.p243), p.p242), p.p241), A::offset(A::scale(s.ad_value(316), p.p244), 1.0)), A::offset(A::scale(s.ad_value(318), p.p245), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(52, p.p247);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(53, p.p246);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(54, p.p248);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(62, A::scale(A::powf(s.ad_value(314), p.p250), p.p249), A::offset(A::scale(s.ad_value(316), p.p251), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(63, p.p253);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(64, p.p252);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(59, A::scale(A::powf(s.ad_value(314), p.p255), p.p254), A::offset(A::scale(s.ad_value(316), p.p256), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(60, p.p258);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(61, p.p257);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale_ad(337, A::offset(A::scale(s.ad_value(316), p.p261), 1.0), p.p260);
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(338, &A::scale({
                if ((1.0 + (p.p263 * s.v[316])) > 0.001) {
                    A::offset(A::scale(s.ad_value(316), p.p263), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p262));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(339, A::offset(A::mul(A::div(A::mul(s.ad_value(337), s.ad_value(338)), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::div(A::neg(s.ad_value(312)), s.ad_value(338))))), 1.0), A::mul(A::div_from_scalar((p.p264 * p.p265), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(312)), 1.0 / (p.p265))))));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(339, &{
                if (s.v[339] > 1e-15) {
                    s.ad_value(339)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(340, A::offset(A::scale(s.ad_value(316), p.p266), 1.0), A::mul(A::scale(s.ad_value(316), p.p267), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p268)), 1.0))));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad_lhs(65, A::div(A::scale(s.ad_value(313), p.p259), A::mul(s.ad_value(339), s.ad_value(312))), 340);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(314), p.p270), p.p269), A::scale(s.ad_value(316), p.p271)), A::scale(s.ad_value(318), p.p272));
        }

        if (s.v[1016] != 0.0) {
            s.store_scale_ad(67, A::offset(A::scale(s.ad_value(316), p.p274), 1.0), p.p273);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(68, p.p275);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(69, p.p276);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(70, p.p277);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(71, A::mul(A::offset(A::scale(A::powf(s.ad_value(314), p.p280), p.p279), p.p278), A::offset(A::scale(s.ad_value(316), p.p281), 1.0)), A::offset(A::scale(s.ad_value(318), p.p282), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(72, p.p283);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(73, p.p284);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(74, p.p285);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(75, A::mul(A::scale(A::offset(A::scale(s.ad_value(314), p.p287), 1.0), p.p286), A::offset(A::scale(s.ad_value(316), p.p288), 1.0)), A::offset(A::scale(s.ad_value(318), p.p289), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(76, p.p290);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(77, p.p291);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(78, A::scale(s.ad_value(316), p.p292), A::offset(A::scale(s.ad_value(316), p.p293), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(79, p.p294);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(80, p.p295);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(81, p.p296);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(82, A::mul(A::offset(A::mul(A::div(A::scale(s.ad_value(340), p.p298), s.ad_value(339)), A::powf(s.ad_value(314), p.p299)), p.p297), A::offset(A::scale(s.ad_value(316), p.p300), 1.0)), A::offset(A::scale(s.ad_value(318), p.p301), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(314), p.p303), p.p302), A::scale(s.ad_value(316), p.p304)), A::scale(s.ad_value(318), p.p305));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(84, p.p306);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(85, p.p307);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(86, p.p308);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(87, p.p309, A::offset(A::scale(s.ad_value(314), p.p310), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(88, A::scale(A::powf(s.ad_value(314), p.p312), p.p311), A::offset(A::scale(s.ad_value(316), p.p313), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_powf(341, 314, p.p315);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad(89, A::mul(A::scale(s.ad_value(341), p.p314), A::offset(A::scale(s.ad_value(316), p.p317), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p316), s.ad_value(341)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_powf(341, 314, p.p319);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad(90, A::mul(A::scale(s.ad_value(341), p.p318), A::offset(A::scale(s.ad_value(316), p.p321), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p320), s.ad_value(341)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(91, p.p322);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(92, A::scale(A::offset(A::scale(s.ad_value(314), p.p324), 1.0), p.p323), A::offset(A::scale(s.ad_value(316), p.p325), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(93, p.p326);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(94, p.p327);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(95, A::scale(A::offset(A::scale(s.ad_value(314), p.p329), 1.0), p.p328), A::offset(A::scale(s.ad_value(316), p.p330), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(96, A::scale(A::offset(A::scale(s.ad_value(314), p.p332), 1.0), p.p331), A::offset(A::scale(s.ad_value(316), p.p333), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(97, p.p334);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(98, p.p335);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar(99, p.p336, 318);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(100, (p.p337 * p.p237), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(101, (p.p338 * p.p238), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(102, p.p339);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(103, p.p340);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(104, p.p341);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(105, p.p340);
        }

        s.v[1019] = if (if self.param_given[342] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1019] != 0.0)) {
            s.store_scalar(105, p.p342);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(106, p.p341);
        }

        s.v[1020] = if (if self.param_given[343] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_scalar(106, p.p343);
        }

        if (s.v[1016] != 0.0) {
            s.copy_ad(107, 105);
        }

        s.v[1021] = if (if self.param_given[344] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1021] != 0.0)) {
            s.store_scalar(107, p.p344);
        }

        if (s.v[1016] != 0.0) {
            s.copy_ad(108, 106);
        }

        s.v[1022] = if (if self.param_given[345] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1022] != 0.0)) {
            s.store_scalar(108, p.p345);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(109, p.p346);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(110, (p.p347 * p.p237), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(111, (p.p348 * p.p238), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(112, p.p349);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(113, p.p350);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(114, p.p351);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(115, p.p352);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(116, p.p353);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(117, p.p354);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale_ad(118, A::mul(A::scale(s.ad_value(321), (8.8541878176e-12 * p.p210)), s.ad_value(320)), 1.0 / (p.p209));
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(125, 321, ((8.8541878176e-12 * p.p210) * (p.p237 * 1.0 / (p.p235))));
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(126, 321, ((8.8541878176e-12 * p.p210) * (p.p238 * 1.0 / (p.p236))));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(119, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p357), p.p356), p.p355), A::scale(s.ad_value(316), p.p358)), A::scale(s.ad_value(318), p.p359));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(314), p.p361), p.p360), A::scale(s.ad_value(316), p.p362)), A::scale(s.ad_value(318), p.p363));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(32, p.p297);
        }

        s.v[1023] = if (if self.param_given[364] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1023] != 0.0)) {
            s.store_scalar(32, p.p364);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(33, p.p298);
        }

        s.v[1024] = if (if self.param_given[365] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1024] != 0.0)) {
            s.store_scalar(33, p.p365);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(34, p.p299);
        }

        s.v[1025] = if (if self.param_given[366] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1025] != 0.0)) {
            s.store_scalar(34, p.p366);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(35, p.p300);
        }

        s.v[1026] = if (if self.param_given[367] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1026] != 0.0)) {
            s.store_scalar(35, p.p367);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(36, p.p301);
        }

        s.v[1027] = if (if self.param_given[368] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1027] != 0.0)) {
            s.store_scalar(36, p.p368);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(121, A::mul(A::add(s.ad_value(32), A::mul(A::div(A::mul(s.ad_value(33), s.ad_value(340)), s.ad_value(339)), A::pow(s.ad_value(314), s.ad_value(34)))), A::offset(A::mul(s.ad_value(35), s.ad_value(316)), 1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(318)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(37, p.p309);
        }

        s.v[1028] = if (if self.param_given[369] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1028] != 0.0)) {
            s.store_scalar(37, p.p369);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(38, p.p310);
        }

        s.v[1029] = if (if self.param_given[370] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1029] != 0.0)) {
            s.store_scalar(38, p.p370);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad_rhs(122, 37, A::offset(A::mul(s.ad_value(38), s.ad_value(314)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(123, A::scale(A::powf(s.ad_value(314), p.p372), p.p371), A::offset(A::scale(s.ad_value(316), p.p373), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_powf(341, 314, p.p375);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad(124, A::mul(A::scale(s.ad_value(341), p.p374), A::offset(A::scale(s.ad_value(316), p.p377), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p376), s.ad_value(341)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(127, p.p378);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(128, p.p379);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(129, p.p380);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(130, 325, p.p381);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(131, 322, p.p382);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(132, 322, p.p383);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(133, p.p384);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(134, p.p385);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(135, p.p386);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(136, p.p387);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(137, 326, p.p388);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(138, 326, p.p389);
        }

        if (s.v[1016] != 0.0) {
            s.store_sub_from_scalar_ad(998, 1.0, A::div_from_scalar((2.0 * p.p396), s.ad_value(312)));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(342, &{
                if (s.v[998] > 0.001) {
                    s.ad_value(998)
                } else {
                    A::constant(0.001)
                }
            });
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(343, 1.0, A::powf(s.ad_value(342), p.p397));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(139, p.p390);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad_lhs(140, A::mul(A::mul(A::scale(s.ad_value(65), p.p391), s.ad_value(65)), s.ad_value(316)), 316);
        }

        if (s.v[1016] != 0.0) {
            s.store_scaled_mul(141, 343, 318, p.p392);
        }

        if (s.v[1016] != 0.0) {
            s.store_scaled_mul(142, 343, 318, p.p393);
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
        if (s.v[1016] != 0.0) {
            s.store_scaled_mul(143, 343, 318, p.p394);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(144, p.p395);
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_scaled(344, 313, p.p399, (2.0 * p.p398));
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar(345, 1e-6, 344);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul(346, 314, 345);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(145, p.p400);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(314), p.p402), p.p401), A::scale(s.ad_value(316), p.p403)), A::scale(s.ad_value(318), p.p404));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(147, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p407), p.p406), p.p405), A::scale(s.ad_value(316), p.p408)), A::scale(s.ad_value(318), p.p409));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(148, A::mul(A::scale(A::offset(A::scale(A::powf(s.ad_value(314), p.p412), p.p411), 1.0), p.p410), A::offset(A::scale(s.ad_value(316), p.p413), 1.0)), A::offset(A::scale(s.ad_value(318), p.p414), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(149, A::scale(A::powf(s.ad_value(314), p.p417), p.p416), p.p415);
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(347, A::mul(A::div_from_scalar((p.p418 * p.p419), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(312)), 1.0 / (p.p419))))), 1.0);
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(347, &{
                if (s.v[347] > 1e-15) {
                    s.ad_value(347)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(150, A::div(A::scale(s.ad_value(344), p.p259), A::mul(s.ad_value(347), s.ad_value(312))), A::offset(A::scale(s.ad_value(316), p.p420), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(314), p.p422), p.p421), A::scale(s.ad_value(316), p.p423)), A::scale(s.ad_value(318), p.p424));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(152, A::scale(A::powf(s.ad_value(314), p.p426), p.p425), A::offset(A::scale(s.ad_value(316), p.p427), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(153, p.p428);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(154, p.p429);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(155, A::scale(A::powf(s.ad_value(314), p.p431), p.p430), A::offset(A::scale(s.ad_value(316), p.p432), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(156, p.p434);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(157, p.p433);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(158, p.p435);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(159, 346, p.p436);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(160, 346, p.p437);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(161, 346, p.p438);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(162, p.p439);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(348, A::add(A::offset(A::scale(s.ad_value(314), p.p832), p.p831), A::scale(s.ad_value(316), p.p833)), A::scale(s.ad_value(318), p.p834));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(349, A::add(A::offset(A::scale(s.ad_value(314), p.p836), p.p835), A::scale(s.ad_value(316), p.p837)), A::scale(s.ad_value(318), p.p838));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(163, A::add(A::div(A::scale(A::add(A::scale(s.ad_value(329), (0.3333333333333333 * 1.0 / (s.v[14]))), s.ad_value(330)), p.p443), A::scale(s.ad_value(328), s.v[14])), A::div_from_scalar((p.p441 + p.p442), A::mul(s.ad_value(329), s.ad_value(327)))), A::scale(s.ad_value(1), p.p440));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(164, (if (p.p445 > 0.0) { p.p445 } else { 0.0 }));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(165, (if (p.p446 > 0.0) { p.p446 } else { 0.0 }));
        }

        s.v[1030] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1030] != 0.0)) {
            s.copy_ad(165, 164);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad_lhs(166, A::scale(s.ad_value(1), p.p12), 164);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad_lhs(167, A::scale(s.ad_value(1), p.p13), 165);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(168, 1, p.p448);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(169, 1, p.p447);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(170, 1, p.p449);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(171, 1, p.p450);
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(350, A::div(A::offset(A::div_from_scalar(p.p454, s.ad_value(314)), 1.0), s.ad_value(316)), p.p453);
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(350, &{
                if (s.v[350] > 1e-6) {
                    s.ad_value(350)
                } else {
                    A::constant(1e-6)
                }
            });
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(172, A::div_from_scalar(p.p452, s.ad_value(350)), p.p451);
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(173, A::div(A::scale(A::offset(A::offset(A::div_from_scalar(p.p458, s.ad_value(314)), 1.0), p.p457), p.p456), s.ad_value(316)), p.p455);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(174, p.p459);
        }

        s.v[1031] = if ((((if self.param_given[460] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[461] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[462] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[463] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_add_ad(40, A::add(A::offset(A::scale(s.ad_value(314), p.p461), p.p460), A::scale(s.ad_value(316), p.p462)), A::scale(s.ad_value(318), p.p463));
        }

        s.v[1032] = if ((((if self.param_given[464] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[465] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[466] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[467] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1032] != 0.0)) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(314), p.p465), p.p464), A::scale(s.ad_value(316), p.p466)), A::scale(s.ad_value(318), p.p467));
        }

        s.v[1033] = if ((((if self.param_given[468] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[469] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[470] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[471] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1033] != 0.0)) {
            s.store_add_ad(45, A::add(A::offset(A::scale(s.ad_value(314), p.p469), p.p468), A::scale(s.ad_value(316), p.p470)), A::scale(s.ad_value(318), p.p471));
        }

        s.v[1034] = if ((((if self.param_given[472] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[473] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[474] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[475] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1034] != 0.0)) {
            s.store_add_ad(46, A::add(A::offset(A::scale(s.ad_value(314), p.p473), p.p472), A::scale(s.ad_value(316), p.p474)), A::scale(s.ad_value(318), p.p475));
        }

        s.v[1035] = if ((((if self.param_given[476] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[477] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[478] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[479] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1035] != 0.0)) {
            s.store_add_ad(47, A::add(A::offset(A::scale(s.ad_value(314), p.p477), p.p476), A::scale(s.ad_value(316), p.p478)), A::scale(s.ad_value(318), p.p479));
        }

        s.v[1036] = if ((((if self.param_given[480] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[481] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[482] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[483] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1036] != 0.0)) {
            s.store_add_ad(49, A::add(A::offset(A::scale(s.ad_value(314), p.p481), p.p480), A::scale(s.ad_value(316), p.p482)), A::scale(s.ad_value(318), p.p483));
        }

        s.v[1037] = if ((((if self.param_given[484] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[485] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[486] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[487] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1037] != 0.0)) {
            s.store_add_ad(50, A::add(A::offset(A::scale(s.ad_value(314), p.p485), p.p484), A::scale(s.ad_value(316), p.p486)), A::scale(s.ad_value(318), p.p487));
        }

        s.v[1038] = if ((((if self.param_given[488] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[489] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[490] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[491] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1038] != 0.0)) {
            s.store_add_ad(57, A::add(A::offset(A::scale(s.ad_value(314), p.p489), p.p488), A::scale(s.ad_value(316), p.p490)), A::scale(s.ad_value(318), p.p491));
        }

        s.v[1039] = if ((((if self.param_given[492] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[493] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[494] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[495] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1039] != 0.0)) {
            s.store_add_ad(58, A::add(A::offset(A::scale(s.ad_value(314), p.p493), p.p492), A::scale(s.ad_value(316), p.p494)), A::scale(s.ad_value(318), p.p495));
        }

        s.v[1040] = if ((((if self.param_given[496] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[497] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[498] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[499] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1040] != 0.0)) {
            s.store_add_ad(51, A::add(A::offset(A::scale(s.ad_value(314), p.p497), p.p496), A::scale(s.ad_value(316), p.p498)), A::scale(s.ad_value(318), p.p499));
        }

        s.v[1041] = if ((((if self.param_given[504] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[505] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[506] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[507] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1041] != 0.0)) {
            s.store_add_ad(52, A::add(A::offset(A::scale(s.ad_value(314), p.p505), p.p504), A::scale(s.ad_value(316), p.p506)), A::scale(s.ad_value(318), p.p507));
        }

        s.v[1042] = if ((((if self.param_given[500] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[501] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[502] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[503] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1042] != 0.0)) {
            s.store_add_ad(53, A::add(A::offset(A::scale(s.ad_value(314), p.p501), p.p500), A::scale(s.ad_value(316), p.p502)), A::scale(s.ad_value(318), p.p503));
        }

        s.v[1043] = if ((((if self.param_given[508] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[509] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[510] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[511] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1043] != 0.0)) {
            s.store_add_ad(54, A::add(A::offset(A::scale(s.ad_value(314), p.p509), p.p508), A::scale(s.ad_value(316), p.p510)), A::scale(s.ad_value(318), p.p511));
        }

        s.v[1044] = if ((((if self.param_given[512] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[513] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[514] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[515] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1044] != 0.0)) {
            s.store_mul_ad_rhs(62, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p513), p.p512), A::scale(s.ad_value(316), p.p514)), A::scale(s.ad_value(318), p.p515)));
        }

        s.v[1045] = if ((((if self.param_given[520] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[521] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[522] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[523] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1045] != 0.0)) {
            s.store_add_ad(63, A::add(A::offset(A::scale(s.ad_value(314), p.p521), p.p520), A::scale(s.ad_value(316), p.p522)), A::scale(s.ad_value(318), p.p523));
        }

        s.v[1046] = if ((((if self.param_given[516] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[517] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[518] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[519] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_add_ad(64, A::add(A::offset(A::scale(s.ad_value(314), p.p517), p.p516), A::scale(s.ad_value(316), p.p518)), A::scale(s.ad_value(318), p.p519));
        }

        s.v[1047] = if ((((if self.param_given[524] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[525] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[526] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[527] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1047] != 0.0)) {
            s.store_mul_ad_rhs(59, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p525), p.p524), A::scale(s.ad_value(316), p.p526)), A::scale(s.ad_value(318), p.p527)));
        }

        s.v[1048] = if ((((if self.param_given[532] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[533] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[534] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[535] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1048] != 0.0)) {
            s.store_add_ad(60, A::add(A::offset(A::scale(s.ad_value(314), p.p533), p.p532), A::scale(s.ad_value(316), p.p534)), A::scale(s.ad_value(318), p.p535));
        }

        s.v[1049] = if ((((if self.param_given[528] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[529] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[530] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[531] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1049] != 0.0)) {
            s.store_add_ad(61, A::add(A::offset(A::scale(s.ad_value(314), p.p529), p.p528), A::scale(s.ad_value(316), p.p530)), A::scale(s.ad_value(318), p.p531));
        }

        s.v[1050] = if ((((if self.param_given[536] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[537] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[538] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[539] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1050] != 0.0)) {
            s.store_mul_ad(65, A::div(s.ad_value(313), s.ad_value(312)), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p537), p.p536), A::scale(s.ad_value(316), p.p538)), A::scale(s.ad_value(318), p.p539)));
        }

        s.v[1051] = if ((((if self.param_given[540] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[541] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[542] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[543] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(314), p.p541), p.p540), A::scale(s.ad_value(316), p.p542)), A::scale(s.ad_value(318), p.p543));
        }

        s.v[1052] = if ((((if self.param_given[544] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[545] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[546] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[547] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1052] != 0.0)) {
            s.store_add_ad(67, A::add(A::offset(A::scale(s.ad_value(314), p.p545), p.p544), A::scale(s.ad_value(316), p.p546)), A::scale(s.ad_value(318), p.p547));
        }

        s.v[1053] = if ((((if self.param_given[548] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[549] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[550] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[551] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1053] != 0.0)) {
            s.store_add_ad(69, A::add(A::offset(A::scale(s.ad_value(314), p.p549), p.p548), A::scale(s.ad_value(316), p.p550)), A::scale(s.ad_value(318), p.p551));
        }

        s.v[1054] = if ((((if self.param_given[552] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[553] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[554] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[555] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1054] != 0.0)) {
            s.store_add_ad(71, A::add(A::offset(A::scale(s.ad_value(314), p.p553), p.p552), A::scale(s.ad_value(316), p.p554)), A::scale(s.ad_value(318), p.p555));
        }

        s.v[1055] = if ((((if self.param_given[556] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[557] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[558] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[559] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1055] != 0.0)) {
            s.store_add_ad(73, A::add(A::offset(A::scale(s.ad_value(314), p.p557), p.p556), A::scale(s.ad_value(316), p.p558)), A::scale(s.ad_value(318), p.p559));
        }

        s.v[1056] = if ((((if self.param_given[560] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[561] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[562] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[563] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1056] != 0.0)) {
            s.store_add_ad(75, A::add(A::offset(A::scale(s.ad_value(314), p.p561), p.p560), A::scale(s.ad_value(316), p.p562)), A::scale(s.ad_value(318), p.p563));
        }

        s.v[1057] = if ((((if self.param_given[564] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[565] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[566] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[567] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1057] != 0.0)) {
            s.store_mul_ad_rhs(78, 316, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p565), p.p564), A::scale(s.ad_value(316), p.p566)), A::scale(s.ad_value(318), p.p567)));
        }

        s.v[1058] = if ((((if self.param_given[568] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[569] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[570] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[571] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1058] != 0.0)) {
            s.store_add_ad(79, A::add(A::offset(A::scale(s.ad_value(314), p.p569), p.p568), A::scale(s.ad_value(316), p.p570)), A::scale(s.ad_value(318), p.p571));
        }

        s.v[1059] = if ((((if self.param_given[572] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[573] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[574] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[575] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1059] != 0.0)) {
            s.store_add_ad(80, A::add(A::offset(A::scale(s.ad_value(314), p.p573), p.p572), A::scale(s.ad_value(316), p.p574)), A::scale(s.ad_value(318), p.p575));
        }

        s.v[1060] = if ((((if self.param_given[576] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[577] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[578] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[579] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1060] != 0.0)) {
            s.store_add_ad(81, A::add(A::offset(A::scale(s.ad_value(314), p.p577), p.p576), A::scale(s.ad_value(316), p.p578)), A::scale(s.ad_value(318), p.p579));
        }

        s.v[1061] = if ((((if self.param_given[580] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[581] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[582] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[583] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1061] != 0.0)) {
            s.store_mul_ad_rhs(82, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p581), p.p580), A::scale(s.ad_value(316), p.p582)), A::scale(s.ad_value(318), p.p583)));
        }

        s.v[1062] = if ((((if self.param_given[584] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[585] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[586] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[587] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1062] != 0.0)) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(314), p.p585), p.p584), A::scale(s.ad_value(316), p.p586)), A::scale(s.ad_value(318), p.p587));
        }

        s.v[1063] = if ((((if self.param_given[588] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[589] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[590] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[591] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1063] != 0.0)) {
            s.store_add_ad(84, A::add(A::offset(A::scale(s.ad_value(314), p.p589), p.p588), A::scale(s.ad_value(316), p.p590)), A::scale(s.ad_value(318), p.p591));
        }

        s.v[1064] = if ((((if self.param_given[592] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[593] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[594] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[595] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1064] != 0.0)) {
            s.store_add_ad(85, A::add(A::offset(A::scale(s.ad_value(314), p.p593), p.p592), A::scale(s.ad_value(316), p.p594)), A::scale(s.ad_value(318), p.p595));
        }

        s.v[1065] = if ((((if self.param_given[596] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[597] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[598] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[599] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1065] != 0.0)) {
            s.store_add_ad(87, A::add(A::offset(A::scale(s.ad_value(314), p.p597), p.p596), A::scale(s.ad_value(316), p.p598)), A::scale(s.ad_value(318), p.p599));
        }

        s.v[1066] = if ((((if self.param_given[600] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[601] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[602] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[603] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1066] != 0.0)) {
            s.store_mul_ad_rhs(88, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p601), p.p600), A::scale(s.ad_value(316), p.p602)), A::scale(s.ad_value(318), p.p603)));
        }

        s.v[1067] = if ((((if self.param_given[604] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[605] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[606] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[607] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1067] != 0.0)) {
            s.store_add_ad(89, A::add(A::offset(A::scale(s.ad_value(314), p.p605), p.p604), A::scale(s.ad_value(316), p.p606)), A::scale(s.ad_value(318), p.p607));
        }

        s.v[1068] = if ((((if self.param_given[608] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[609] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[610] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[611] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1068] != 0.0)) {
            s.store_add_ad(90, A::add(A::offset(A::scale(s.ad_value(314), p.p609), p.p608), A::scale(s.ad_value(316), p.p610)), A::scale(s.ad_value(318), p.p611));
        }

        s.v[1069] = if ((((if self.param_given[612] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[613] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[614] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[615] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1069] != 0.0)) {
            s.store_add_ad(92, A::add(A::offset(A::scale(s.ad_value(314), p.p613), p.p612), A::scale(s.ad_value(316), p.p614)), A::scale(s.ad_value(318), p.p615));
        }

        s.v[1070] = if ((((if self.param_given[616] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[617] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[618] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[619] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1070] != 0.0)) {
            s.store_add_ad(94, A::add(A::offset(A::scale(s.ad_value(314), p.p617), p.p616), A::scale(s.ad_value(316), p.p618)), A::scale(s.ad_value(318), p.p619));
        }

        s.v[1071] = if ((((if self.param_given[620] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[621] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[622] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[623] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1071] != 0.0)) {
            s.store_add_ad(95, A::add(A::offset(A::scale(s.ad_value(314), p.p621), p.p620), A::scale(s.ad_value(316), p.p622)), A::scale(s.ad_value(318), p.p623));
        }

        s.v[1072] = if ((((if self.param_given[624] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[625] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[626] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[627] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1072] != 0.0)) {
            s.store_add_ad(96, A::add(A::offset(A::scale(s.ad_value(314), p.p625), p.p624), A::scale(s.ad_value(316), p.p626)), A::scale(s.ad_value(318), p.p627));
        }

        s.v[1073] = if ((((if self.param_given[628] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[629] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[630] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[631] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1073] != 0.0)) {
            s.store_mul_ad_rhs(99, 319, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p629), p.p628), A::scale(s.ad_value(316), p.p630)), A::scale(s.ad_value(318), p.p631)));
        }

        s.v[1074] = if ((((if self.param_given[632] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[633] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[634] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[635] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1074] != 0.0)) {
            s.store_mul_ad_rhs(100, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p633), p.p632), A::scale(s.ad_value(316), p.p634)), A::scale(s.ad_value(318), p.p635)));
        }

        s.v[1075] = if ((((if self.param_given[636] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[637] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[638] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[639] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1075] != 0.0)) {
            s.store_mul_ad_rhs(101, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p637), p.p636), A::scale(s.ad_value(316), p.p638)), A::scale(s.ad_value(318), p.p639)));
        }

        s.v[1076] = if ((((if self.param_given[640] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[641] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[642] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[643] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1076] != 0.0)) {
            s.store_add_ad(102, A::add(A::offset(A::scale(s.ad_value(314), p.p641), p.p640), A::scale(s.ad_value(316), p.p642)), A::scale(s.ad_value(318), p.p643));
        }

        s.v[1077] = if ((((if self.param_given[644] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[645] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[646] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[647] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1077] != 0.0)) {
            s.store_mul_ad_rhs(110, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p645), p.p644), A::scale(s.ad_value(316), p.p646)), A::scale(s.ad_value(318), p.p647)));
        }

        s.v[1078] = if ((((if self.param_given[648] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[649] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[650] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[651] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1078] != 0.0)) {
            s.store_mul_ad_rhs(111, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p649), p.p648), A::scale(s.ad_value(316), p.p650)), A::scale(s.ad_value(318), p.p651)));
        }

        s.v[1079] = if ((((if self.param_given[652] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[653] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[654] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[655] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1079] != 0.0)) {
            s.store_add_ad(114, A::add(A::offset(A::scale(s.ad_value(314), p.p653), p.p652), A::scale(s.ad_value(316), p.p654)), A::scale(s.ad_value(318), p.p655));
        }

        s.v[1080] = if ((((if self.param_given[656] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[657] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[658] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[659] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1080] != 0.0)) {
            s.store_add_ad(115, A::add(A::offset(A::scale(s.ad_value(314), p.p657), p.p656), A::scale(s.ad_value(316), p.p658)), A::scale(s.ad_value(318), p.p659));
        }

        s.v[1081] = if ((((if self.param_given[660] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[661] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[662] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[663] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1081] != 0.0)) {
            s.store_mul_ad(118, A::scale(A::mul(s.ad_value(322), s.ad_value(320)), 1000000.0), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p661), p.p660), A::scale(s.ad_value(316), p.p662)), A::scale(s.ad_value(318), p.p663)));
        }

        s.v[1082] = if ((((if self.param_given[664] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[667] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_add_ad(119, A::add(A::offset(A::scale(s.ad_value(314), p.p665), p.p664), A::scale(s.ad_value(316), p.p666)), A::scale(s.ad_value(318), p.p667));
        }

        s.v[1083] = if ((((if self.param_given[668] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[669] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[670] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[671] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1083] != 0.0)) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(314), p.p669), p.p668), A::scale(s.ad_value(316), p.p670)), A::scale(s.ad_value(318), p.p671));
        }

        s.v[1084] = if ((((((((if self.param_given[672] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[673] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[674] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[675] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[580] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[581] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[582] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[583] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(28, p.p580);
        }

        s.v[1085] = if (if self.param_given[672] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1085] != 0.0)) {
            s.store_scalar(28, p.p672);
        }

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
        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(29, p.p581);
        }

        s.v[1086] = if (if self.param_given[673] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(29, p.p673);
        }

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(30, p.p582);
        }

        s.v[1087] = if (if self.param_given[674] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(30, p.p674);
        }

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(31, p.p583);
        }

        s.v[1088] = if (if self.param_given[675] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1088] != 0.0)) {
            s.store_scalar(31, p.p675);
        }

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_mul_ad_rhs(121, 314, A::add(A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(314))), A::mul(s.ad_value(30), s.ad_value(316))), A::mul(s.ad_value(31), s.ad_value(318))));
        }

        s.v[1089] = if ((((((((if self.param_given[676] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[677] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[678] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[679] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[596] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[597] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[598] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[599] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(28, p.p596);
        }

        s.v[1090] = if (if self.param_given[676] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_scalar(28, p.p676);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(29, p.p597);
        }

        s.v[1091] = if (if self.param_given[677] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1091] != 0.0)) {
            s.store_scalar(29, p.p677);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(30, p.p598);
        }

        s.v[1092] = if (if self.param_given[678] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1092] != 0.0)) {
            s.store_scalar(30, p.p678);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(31, p.p599);
        }

        s.v[1093] = if (if self.param_given[679] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1093] != 0.0)) {
            s.store_scalar(31, p.p679);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_add_ad(122, A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(314))), A::mul(s.ad_value(30), s.ad_value(316))), A::mul(s.ad_value(31), s.ad_value(318)));
        }

        s.v[1094] = if ((((if self.param_given[680] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[681] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[682] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[683] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1094] != 0.0)) {
            s.store_mul_ad_rhs(123, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p681), p.p680), A::scale(s.ad_value(316), p.p682)), A::scale(s.ad_value(318), p.p683)));
        }

        s.v[1095] = if ((((if self.param_given[684] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[685] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[686] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[687] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1095] != 0.0)) {
            s.store_mul_ad_rhs(124, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p685), p.p684), A::scale(s.ad_value(316), p.p686)), A::scale(s.ad_value(318), p.p687)));
        }

        s.v[1096] = if ((((if self.param_given[688] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[689] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[690] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[691] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1096] != 0.0)) {
            s.store_mul_ad_rhs(125, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p689), p.p688), A::scale(s.ad_value(316), p.p690)), A::scale(s.ad_value(318), p.p691)));
        }

        s.v[1097] = if ((((if self.param_given[692] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[693] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[694] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[695] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_mul_ad_rhs(126, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p693), p.p692), A::scale(s.ad_value(316), p.p694)), A::scale(s.ad_value(318), p.p695)));
        }

        s.v[1098] = if ((((if self.param_given[696] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[697] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[698] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[699] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_mul_ad_rhs(130, 325, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p697), p.p696), A::scale(s.ad_value(316), p.p698)), A::scale(s.ad_value(318), p.p699)));
        }

        s.v[1099] = if ((((if self.param_given[700] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[701] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[702] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[703] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1099] != 0.0)) {
            s.store_mul_ad_rhs(131, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p701), p.p700), A::scale(s.ad_value(316), p.p702)), A::scale(s.ad_value(318), p.p703)));
        }

        s.v[1100] = if ((((if self.param_given[704] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[705] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[706] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[707] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_mul_ad_rhs(132, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p705), p.p704), A::scale(s.ad_value(316), p.p706)), A::scale(s.ad_value(318), p.p707)));
        }

        s.v[1101] = if ((((if self.param_given[708] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[709] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[710] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[711] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1101] != 0.0)) {
            s.store_mul_ad_rhs(137, 326, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p709), p.p708), A::scale(s.ad_value(316), p.p710)), A::scale(s.ad_value(318), p.p711)));
        }

        s.v[1102] = if ((((if self.param_given[712] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[713] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[714] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[715] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1102] != 0.0)) {
            s.store_mul_ad_rhs(138, 326, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p713), p.p712), A::scale(s.ad_value(316), p.p714)), A::scale(s.ad_value(318), p.p715)));
        }

        s.v[1103] = if ((((if self.param_given[716] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[717] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[718] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[719] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_mul_ad_rhs(140, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p717), p.p716), A::scale(s.ad_value(316), p.p718)), A::scale(s.ad_value(318), p.p719)));
        }

        s.v[1104] = if ((((if self.param_given[720] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[721] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[722] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[723] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1104] != 0.0)) {
            s.store_mul_ad_rhs(141, 318, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p721), p.p720), A::scale(s.ad_value(316), p.p722)), A::scale(s.ad_value(318), p.p723)));
        }

        s.v[1105] = if ((((if self.param_given[724] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[725] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[726] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[727] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1105] != 0.0)) {
            s.store_mul_ad_rhs(142, 318, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p725), p.p724), A::scale(s.ad_value(316), p.p726)), A::scale(s.ad_value(318), p.p727)));
        }

        s.v[1106] = if ((((if self.param_given[728] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[729] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[730] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[731] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1106] != 0.0)) {
            s.store_mul_ad_rhs(143, 318, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p729), p.p728), A::scale(s.ad_value(316), p.p730)), A::scale(s.ad_value(318), p.p731)));
        }

        s.v[1107] = if ((((if self.param_given[732] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[733] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[734] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[735] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1107] != 0.0)) {
            s.store_add_ad(145, A::add(A::offset(A::scale(s.ad_value(314), p.p733), p.p732), A::scale(s.ad_value(316), p.p734)), A::scale(s.ad_value(318), p.p735));
        }

        s.v[1108] = if ((((if self.param_given[736] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[737] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[738] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[739] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(314), p.p737), p.p736), A::scale(s.ad_value(316), p.p738)), A::scale(s.ad_value(318), p.p739));
        }

        s.v[1109] = if ((((if self.param_given[740] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[741] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[742] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[743] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1109] != 0.0)) {
            s.store_add_ad(147, A::add(A::offset(A::scale(s.ad_value(314), p.p741), p.p740), A::scale(s.ad_value(316), p.p742)), A::scale(s.ad_value(318), p.p743));
        }

        s.v[1110] = if ((((if self.param_given[744] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[745] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[746] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[747] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1110] != 0.0)) {
            s.store_add_ad(148, A::add(A::offset(A::scale(s.ad_value(314), p.p745), p.p744), A::scale(s.ad_value(316), p.p746)), A::scale(s.ad_value(318), p.p747));
        }

        s.v[1111] = if ((((if self.param_given[748] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[749] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[750] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[751] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1111] != 0.0)) {
            s.store_add_ad(149, A::add(A::offset(A::scale(s.ad_value(314), p.p749), p.p748), A::scale(s.ad_value(316), p.p750)), A::scale(s.ad_value(318), p.p751));
        }

        s.v[1112] = if ((((if self.param_given[752] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[753] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[754] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[755] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1112] != 0.0)) {
            s.store_mul_ad(150, A::div(s.ad_value(344), s.ad_value(312)), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p753), p.p752), A::scale(s.ad_value(316), p.p754)), A::scale(s.ad_value(318), p.p755)));
        }

        s.v[1113] = if ((((if self.param_given[756] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[757] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[758] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[759] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1113] != 0.0)) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(314), p.p757), p.p756), A::scale(s.ad_value(316), p.p758)), A::scale(s.ad_value(318), p.p759));
        }

        s.v[1114] = if ((((if self.param_given[760] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[761] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[762] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[763] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1114] != 0.0)) {
            s.store_mul_ad_rhs(152, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p761), p.p760), A::scale(s.ad_value(316), p.p762)), A::scale(s.ad_value(318), p.p763)));
        }

        s.v[1115] = if ((((if self.param_given[764] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[765] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[766] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[767] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1115] != 0.0)) {
            s.store_add_ad(153, A::add(A::offset(A::scale(s.ad_value(314), p.p765), p.p764), A::scale(s.ad_value(316), p.p766)), A::scale(s.ad_value(318), p.p767));
        }

        s.v[1116] = if ((((if self.param_given[768] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[769] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[770] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[771] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1116] != 0.0)) {
            s.store_add_ad(154, A::add(A::offset(A::scale(s.ad_value(314), p.p769), p.p768), A::scale(s.ad_value(316), p.p770)), A::scale(s.ad_value(318), p.p771));
        }

        s.v[1117] = if ((((if self.param_given[772] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[773] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[774] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[775] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1117] != 0.0)) {
            s.store_mul_ad_rhs(155, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p773), p.p772), A::scale(s.ad_value(316), p.p774)), A::scale(s.ad_value(318), p.p775)));
        }

        s.v[1118] = if ((((if self.param_given[780] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[781] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[782] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[783] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1118] != 0.0)) {
            s.store_add_ad(156, A::add(A::offset(A::scale(s.ad_value(314), p.p781), p.p780), A::scale(s.ad_value(316), p.p782)), A::scale(s.ad_value(318), p.p783));
        }

        s.v[1119] = if ((((if self.param_given[776] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[777] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[778] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[779] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1119] != 0.0)) {
            s.store_add_ad(157, A::add(A::offset(A::scale(s.ad_value(314), p.p777), p.p776), A::scale(s.ad_value(316), p.p778)), A::scale(s.ad_value(318), p.p779));
        }

        s.v[1120] = if ((((if self.param_given[784] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[785] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[786] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[787] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1120] != 0.0)) {
            s.store_mul_ad_rhs(159, 346, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p785), p.p784), A::scale(s.ad_value(316), p.p786)), A::scale(s.ad_value(318), p.p787)));
        }

        s.v[1121] = if ((((if self.param_given[788] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[789] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[790] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[791] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1121] != 0.0)) {
            s.store_mul_ad_rhs(160, 346, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p789), p.p788), A::scale(s.ad_value(316), p.p790)), A::scale(s.ad_value(318), p.p791)));
        }

        s.v[1122] = if ((((if self.param_given[792] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[793] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[794] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[795] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1122] != 0.0)) {
            s.store_mul_ad_rhs(161, 346, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p793), p.p792), A::scale(s.ad_value(316), p.p794)), A::scale(s.ad_value(318), p.p795)));
        }

        s.v[1123] = if ((((if self.param_given[796] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[797] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[798] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[799] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1123] != 0.0)) {
            s.store_mul_ad_rhs(172, 318, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p797), p.p796), A::scale(s.ad_value(316), p.p798)), A::scale(s.ad_value(318), p.p799)));
        }

        s.v[1124] = if ((((if self.param_given[800] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[801] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[802] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[803] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1124] != 0.0)) {
            s.store_mul_ad_rhs(173, 319, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p801), p.p800), A::scale(s.ad_value(316), p.p802)), A::scale(s.ad_value(318), p.p803)));
        }

        s.v[1125] = if ((((if self.param_given[804] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[805] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[806] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[807] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1125] != 0.0)) {
            s.store_add_ad(174, A::add(A::offset(A::scale(s.ad_value(314), p.p805), p.p804), A::scale(s.ad_value(316), p.p806)), A::scale(s.ad_value(318), p.p807));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(1005, 0.0);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(1006, 0.0);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(1004, 0.0);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(39, p.p812);
        }

        s.v[1126] = if (if self.param_given[813] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1126] != 0.0)) {
            s.store_scalar(39, p.p813);
        }

        s.v[1127] = if (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0)))) { 1.0 } else { 0.0 };

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (s.v[1] - 0.5);
            let assign9190_cond_e9118: f64 = if (((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) && (s.v[1004] < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1005, 1005, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1004), (s.v[7] + s.v[3])), (s.v[5] + (0.5 * s.v[3])))));
            }
            if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_add_ad_rhs(1006, 1006, A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(1004), (s.v[7] + s.v[3])), (s.v[6] + (0.5 * s.v[3])))));
            }
            if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
                s.store_offset(1004, 1004, 1.0);
            }
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(989, 1005, 2);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(990, 1006, 2);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(991, (1.0 / (p.p808 + (0.5 * s.v[3]))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scalar(992, (1.0 / (p.p809 + (0.5 * s.v[3]))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1002, &{
                if ((s.v[3] + s.v[310]) > 1e-9) {
                    A::offset(s.ad_value(310), s.v[3])
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_ad(1003, &{
                if (((s.v[4] + s.v[311]) + p.p810) > 1e-9) {
                    A::offset(A::add(s.ad_value(4), s.ad_value(311)), p.p810)
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1000, 1.0, A::powf(s.ad_value(1002), p.p818));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1001, 1.0, A::powf(s.ad_value(1003), p.p819));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_scale_ad(993, A::add(A::add(A::offset(A::scale(s.ad_value(1000), p.p815), 1.0), A::scale(s.ad_value(1001), p.p816)), A::mul(A::scale(s.ad_value(1000), p.p817), s.ad_value(1001))), (1.0 + (p.p814 * (s.v[353] - 1.0))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(994, A::scale(A::add(s.ad_value(989), s.ad_value(990)), p.p811), 993);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(995, A::scale(A::add(s.ad_value(991), s.ad_value(992)), p.p811), 993);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1000, 1.0, A::powf(s.ad_value(1002), p.p824));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_from_scalar_ad(1001, 1.0, A::powf(s.ad_value(1003), p.p825));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add_ad(996, A::add(A::offset(A::scale(s.ad_value(1000), p.p821), 1.0), A::scale(s.ad_value(1001), p.p822)), A::mul(A::scale(s.ad_value(1000), p.p823), s.ad_value(1001)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_sub_ad_lhs(998, A::sub(A::add(s.ad_value(989), s.ad_value(990)), s.ad_value(991)), 992);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(999, A::offset(s.ad_value(994), 1.0), A::offset(s.ad_value(995), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(65, 65, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(82, A::mul(A::mul(s.ad_value(82), s.ad_value(999)), A::offset(A::scale(s.ad_value(995), p.p812), 1.0)), A::offset(A::scale(s.ad_value(994), p.p812), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(121, A::mul(A::mul(s.ad_value(121), s.ad_value(999)), A::offset(A::mul(s.ad_value(39), s.ad_value(995)), 1.0)), A::offset(A::mul(s.ad_value(39), s.ad_value(994)), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_mul(150, 150, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad_lhs(999, A::scale(s.ad_value(998), p.p820), 996);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(40, 40, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(145, 145, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_div_ad(999, A::scale(s.ad_value(998), p.p826), A::powf(s.ad_value(996), p.p827));
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(62, 62, 999);
        }

        if ((s.v[1016] != 0.0) && (s.v[1127] != 0.0)) {
            s.store_add(155, 155, 999);
        }

        s.v[1128] = if ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1129] = if (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_offset(998, 4, s.v[8]);
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_scalar(999, (1.0 / p.p828));
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_from_scalar_ad(11, (p.p828 * p.p828), A::scale(s.ad_value(998), s.v[8]));
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(12, A::sub(A::scale(A::exp(A::scale(s.ad_value(999), ((-10.0) * s.v[8]))), ((0.1 * s.v[8]) + (0.01 * p.p828))), A::mul(A::offset(A::scale(s.ad_value(998), 0.1), (0.01 * p.p828)), A::exp(A::mul(A::scale(s.ad_value(998), (-10.0)), s.ad_value(999))))), 4);
        }

        if (((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) && (s.v[1129] != 0.0)) {
            s.store_div_ad_lhs(13, A::sub(A::scale(A::exp(A::scale(s.ad_value(999), ((-20.0) * s.v[8]))), ((0.05 * s.v[8]) + (0.0025 * p.p828))), A::mul(A::offset(A::scale(s.ad_value(998), 0.05), (0.0025 * p.p828)), A::exp(A::mul(A::scale(s.ad_value(998), (-20.0)), s.ad_value(999))))), 4);
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad(998, A::add(s.ad_value(11), A::scale(s.ad_value(12), p.p829)), A::scale(s.ad_value(13), p.p830));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(40, 40, A::mul(s.ad_value(348), s.ad_value(998)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(65, 65, A::offset(A::mul(s.ad_value(349), s.ad_value(998)), 1.0));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_add_ad_rhs(145, 145, A::mul(s.ad_value(348), s.ad_value(998)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1128] != 0.0)) {
            s.store_mul_ad_rhs(150, 150, A::offset(A::mul(s.ad_value(349), s.ad_value(998)), 1.0));
        }

        s.copy_ad(175, 40);

        s.copy_ad(176, 41);

        s.copy_ad(177, 42);

        s.copy_ad(179, 43);

        s.copy_ad(180, 44);

        if (s.v[45] > 1e20) {
            s.store_ad(181, &{
                if (s.v[45] < 1e26) {
                    s.ad_value(45)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(181, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(182, 46);
        } else {
            s.store_scalar(182, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(183, 47);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(184, 48);

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
        s.copy_ad(185, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(186, 50);
        } else {
            s.store_scalar(186, 0.0);
        }

        s.copy_ad(190, 55);

        s.copy_ad(191, 56);

        if (s.v[57] > 1e23) {
            s.store_ad(192, &{
                if (s.v[57] < 1e27) {
                    s.ad_value(57)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(192, 1e23);
        }

        if (s.v[58] > 1e23) {
            s.store_ad(193, &{
                if (s.v[58] < 1e27) {
                    s.ad_value(58)
                } else {
                    A::constant(1e27)
                }
            });
        } else {
            s.store_scalar(193, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(187, 51);
        } else {
            s.store_scalar(187, 0.0);
        }

        if (s.v[53] > 0.0) {
            s.store_ad(189, &{
                if (s.v[53] < 0.5) {
                    s.ad_value(53)
                } else {
                    A::constant(0.5)
                }
            });
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[52] > 0.0) {
            s.store_ad(188, &{
                if (s.v[52] < 1.0) {
                    s.ad_value(52)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(178, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(194, 62);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.store_ad(196, &{
                if (s.v[64] < 1.0) {
                    s.ad_value(64)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(195, 63);
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(197, 59);
        } else {
            s.store_scalar(197, 0.0);
        }

        if (s.v[61] > 0.0) {
            s.store_ad(198, &{
                if (s.v[61] < 1.0) {
                    s.ad_value(61)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(199, 60);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(200, 65);
        } else {
            s.store_scalar(200, 0.0);
        }

        s.copy_ad(201, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(202, 67);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(204, 69);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(206, 71);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(208, 73);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(210, 75);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 76);

        s.copy_ad(212, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(213, 78);
        } else {
            s.store_scalar(213, 0.0);
        }

        s.copy_ad(214, 79);

        if (s.v[80] > (-0.5)) {
            s.store_ad(215, &{
                if (s.v[80] < 1.0) {
                    s.ad_value(80)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(215, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(216, 81);
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(217, 82);
        } else {
            s.store_scalar(217, 0.0);
        }

        s.copy_ad(218, 83);

        if (s.v[84] > (-0.5)) {
            s.store_ad(219, &{
                if (s.v[84] < 1.0) {
                    s.ad_value(84)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(219, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(220, 85);
        } else {
            s.store_scalar(220, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(221, 86);
        } else {
            s.store_scalar(221, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(222, 87);
        } else {
            s.store_scalar(222, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(223, 88);
        } else {
            s.store_scalar(223, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(224, 89);
        } else {
            s.store_scalar(224, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(225, 90);
        } else {
            s.store_scalar(225, 0.0);
        }

        s.copy_ad(226, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(227, 92);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 93);

        s.copy_ad(229, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(230, 95);
        } else {
            s.store_scalar(230, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(231, 96);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[97] > 1e-12) {
            s.copy_ad(232, 97);
        } else {
            s.store_scalar(232, 1e-12);
        }

        s.copy_ad(233, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(234, 99);
        } else {
            s.store_scalar(234, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(235, 100);
        } else {
            s.store_scalar(235, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(236, 101);
        } else {
            s.store_scalar(236, 0.0);
        }

        s.copy_ad(237, 102);

        s.copy_ad(238, 103);

        s.copy_ad(239, 104);

        s.copy_ad(240, 105);

        s.copy_ad(241, 106);

        s.copy_ad(242, 107);

        s.copy_ad(243, 108);

        s.copy_ad(244, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(245, 110);
        } else {
            s.store_scalar(245, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(246, 111);
        } else {
            s.store_scalar(246, 0.0);
        }

        s.copy_ad(247, 112);

        s.copy_ad(248, 113);

        s.copy_ad(249, 114);

        s.copy_ad(250, 115);

        s.copy_ad(251, 116);

        s.copy_ad(252, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(253, 118);
        } else {
            s.store_scalar(253, 0.0);
        }

        s.copy_ad(254, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(255, 120);
        } else {
            s.store_scalar(255, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(256, 121);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(257, 122);
        } else {
            s.store_scalar(257, 2.0);
        }

        s.copy_ad(258, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(259, 124);
        } else {
            s.store_scalar(259, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(260, 125);
        } else {
            s.store_scalar(260, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(261, 126);
        } else {
            s.store_scalar(261, 0.0);
        }

        s.copy_ad(262, 127);

        s.copy_ad(263, 128);

        s.copy_ad(264, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(265, 130);
        } else {
            s.store_scalar(265, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(266, 131);
        } else {
            s.store_scalar(266, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(267, 132);
        } else {
            s.store_scalar(267, 0.0);
        }

        s.copy_ad(268, 133);

        s.copy_ad(269, 134);

        s.copy_ad(270, 135);

        s.copy_ad(271, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(272, 137);
        } else {
            s.store_scalar(272, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(273, 138);
        } else {
            s.store_scalar(273, 0.0);
        }

        s.copy_ad(274, 139);

        if (s.v[140] > 0.0) {
            s.copy_ad(275, 140);
        } else {
            s.store_scalar(275, 0.0);
        }

        if (s.v[141] > 0.0) {
            s.copy_ad(276, 141);
        } else {
            s.store_scalar(276, 0.0);
        }

        if (s.v[142] > 0.0) {
            s.copy_ad(277, 142);
        } else {
            s.store_scalar(277, 0.0);
        }

        if (s.v[143] > 0.0) {
            s.copy_ad(278, 143);
        } else {
            s.store_scalar(278, 0.0);
        }

        s.copy_ad(279, 144);

        s.copy_ad(280, 145);

        s.copy_ad(281, 146);

        s.copy_ad(282, 147);

        if (s.v[148] > 1e20) {
            s.store_ad(283, &{
                if (s.v[148] < 1e26) {
                    s.ad_value(148)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(283, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(284, 149);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(285, 150);
        } else {
            s.store_scalar(285, 0.0);
        }

        s.copy_ad(286, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(287, 152);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[153] > 0.0) {
            s.store_ad(288, &{
                if (s.v[153] < 1.0) {
                    s.ad_value(153)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(288, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(289, 154);
        } else {
            s.store_scalar(289, 0.0);
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
        if (s.v[155] > 0.0) {
            s.copy_ad(290, 155);
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[157] > 0.0) {
            s.store_ad(292, &{
                if (s.v[157] < 1.0) {
                    s.ad_value(157)
                } else {
                    A::constant(1.0)
                }
            });
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(291, 156);
        } else {
            s.store_scalar(291, 0.0);
        }

        s.copy_ad(293, 158);

        if (s.v[159] > 0.0) {
            s.copy_ad(294, 159);
        } else {
            s.store_scalar(294, 0.0);
        }

        if (s.v[160] > 0.0) {
            s.copy_ad(295, 160);
        } else {
            s.store_scalar(295, 0.0);
        }

        if (s.v[161] > 0.0) {
            s.copy_ad(296, 161);
        } else {
            s.store_scalar(296, 0.0);
        }

        s.copy_ad(297, 162);

        if (s.v[163] > 0.0) {
            s.copy_ad(298, 163);
        } else {
            s.store_scalar(298, 0.0);
        }

        s.copy_ad(299, 166);

        s.copy_ad(300, 167);

        s.copy_ad(301, 169);

        s.copy_ad(302, 170);

        s.copy_ad(303, 171);

        s.copy_ad(304, 168);

        if (s.v[172] > 0.0001) {
            s.copy_ad(305, 172);
        } else {
            s.store_scalar(305, 0.0001);
        }

        if (s.v[173] > 0.0) {
            s.copy_ad(306, 173);
        } else {
            s.store_scalar(306, 0.0);
        }

        s.copy_ad(307, 174);

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
            s.copy_ad(191, 190);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(193, 192);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(246, 245);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(248, 247);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(250, 249);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(252, 251);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(236, 235);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(242, 240);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(243, 241);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(261, 260);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(263, 262);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(267, 266);
        }

        if (s.v[1130] != 0.0) {
            s.copy_ad(273, 272);
        }

        s.store_scale(757, 180, 8.8541878176e-12);

        s.store_div(758, 757, 179);

        s.store_square(759, 179);

        s.store_scale(760, 758, 6.241449993689894e18);

        s.store_mul(761, 255, 181);

        if (s.v[761] > 1e20) {
            s.store_ad(761, &{
                if (s.v[761] < 1e26) {
                    s.ad_value(761)
                } else {
                    A::constant(1e26)
                }
            });
        } else {
            s.store_scalar(761, 1e20);
        }

        s.v[762] = 0.0;

        s.v[1131] = if (p.p51 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1131] != 0.0) {
            s.store_scale_ad(762, A::powf(s.ad_value(758), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.v[1132] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1131] != 0.0) && (s.v[1132] != 0.0)) {
            s.store_scale(762, 762, (7.448711 / 5.951993));
        }

        s.store_scale(763, 758, (1e-8 * 1.0 / (s.v[756])));

        s.store_scale(764, 212, 0.5);

        s.v[765] = 0.5;

        s.v[1133] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (s.v[1133] != 0.0) {
            s.store_scale(764, 212, 0.3333333333333333);
        }

        if (s.v[1133] != 0.0) {
            s.store_scalar(765, 0.3333333333333333);
        }

        s.store_offset_ad(997, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(222)), 1.0)), (-1.0));

        s.store_ad(766, &A::div(A::mul(A::offset(s.ad_value(997), (-1.0)), A::offset(s.ad_value(997), (-1.0))), {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_offset_ad(997, A::pow_from_scalar(2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(257)), 1.0)), (-1.0));

        s.store_ad(767, &A::div(A::mul(A::offset(s.ad_value(997), (-1.0)), A::offset(s.ad_value(997), (-1.0))), {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }));

        s.store_div_from_scalar(768, 1.0, 226);

        s.store_div(769, 757, 190);

        s.store_div(770, 757, 191);

        s.store_div_ad_lhs(771, A::sqrt(A::scale(s.ad_value(192), ((2.0 * 1.6021918e-19) * (s.v[756] * s.v[356])))), 769);

        s.store_div_ad_lhs(772, A::sqrt(A::scale(s.ad_value(193), ((2.0 * 1.6021918e-19) * (s.v[756] * s.v[356])))), 770);

        s.store_square(773, 771);

        s.store_square(774, 772);

        s.store_offset_ad(775, A::div(A::ln(A::offset(A::exp(A::scale(s.ad_value(264), (0.005 * s.v[356]))), (-1.0))), s.ad_value(264)), (-((((((0.005 * s.v[356])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(776, A::ln(A::scale(s.ad_value(771), 0.5)), 775);

        s.store_add_ad_lhs(777, A::ln(A::scale(s.ad_value(772), 0.5)), 775);

        s.store_div_from_scalar(809, 1.0, 771);

        s.store_offset_scaled(810, 771, 3.1, 8.5);

        s.store_square(778, 810);

        s.store_scale(811, 810, 0.5);

        s.v[1134] = if (s.v[809] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1134] != 0.0) {
            s.store_scale(779, 809, 64.0);
        }

        s.v[1135] = if (s.v[809] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1134] != 0.0)) && (s.v[1135] != 0.0)) {
            s.store_offset_scaled(779, 809, 22.0, 3.0);
        }

        s.v[1136] = if (s.v[809] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (s.v[1136] != 0.0)) {
            s.store_offset_scaled(779, 809, (-7.2), 15.5);
        }

        if (((!(s.v[1134] != 0.0)) && (!(s.v[1135] != 0.0))) && (!(s.v[1136] != 0.0))) {
            s.copy_ad(779, 771);
        }

        s.store_sub_ad(780, A::add(s.ad_value(811), A::scale(s.ad_value(773), 0.5)), A::mul(s.ad_value(771), A::sqrt(A::add(A::add(s.ad_value(811), A::scale(s.ad_value(773), 0.25)), s.ad_value(779)))));

        s.store_div_from_scalar(809, 1.0, 772);

        s.store_offset_scaled(810, 772, 3.1, 8.5);

        s.store_square(781, 810);

        s.store_scale(811, 810, 0.5);

        s.v[1137] = if (s.v[809] < 0.06) { 1.0 } else { 0.0 };

        if (s.v[1137] != 0.0) {
            s.store_scale(782, 809, 64.0);
        }

        s.v[1138] = if (s.v[809] <= 0.45) { 1.0 } else { 0.0 };

        if ((!(s.v[1137] != 0.0)) && (s.v[1138] != 0.0)) {
            s.store_offset_scaled(782, 809, 22.0, 3.0);
        }

        s.v[1139] = if (s.v[809] <= 1.6) { 1.0 } else { 0.0 };

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (s.v[1139] != 0.0)) {
            s.store_offset_scaled(782, 809, (-7.2), 15.5);
        }

        if (((!(s.v[1137] != 0.0)) && (!(s.v[1138] != 0.0))) && (!(s.v[1139] != 0.0))) {
            s.copy_ad(782, 772);
        }

        s.store_sub_ad(783, A::add(s.ad_value(811), A::scale(s.ad_value(774), 0.5)), A::mul(s.ad_value(772), A::sqrt(A::add(A::add(s.ad_value(811), A::scale(s.ad_value(774), 0.25)), s.ad_value(782)))));

        s.store_div_from_scalar(784, 1.0, 244);

        s.store_scale_ad(785, A::sqrt(A::scale(s.ad_value(244), ((2.0 * 1.6021918e-19) * 9.1093826e-31))), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(786, 785, 179);

        s.store_mul(787, 785, 190);

        s.store_mul(788, 785, 191);

        s.v[789] = 0.0;

        s.v[1140] = if (s.v[239] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1140] != 0.0) {
            s.store_div_ad_lhs(789, A::scale(s.ad_value(238), (-0.495)), 239);
        }

        s.v[790] = 0.0;

        s.v[1141] = if (s.v[241] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1141] != 0.0) {
            s.store_div_ad_lhs(790, A::scale(s.ad_value(240), (-0.495)), 241);
        }

        s.v[1142] = if (s.v[243] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1142] != 0.0) {
            s.store_div_ad_lhs(791, A::scale(s.ad_value(242), (-0.495)), 243);
        }

        s.store_ad(792, &A::pow_from_scalar(s.v[353], s.ad_value(237)));

        s.store_mul(234, 234, 792);

        s.store_mul(235, 235, 792);

        s.store_mul(236, 236, 792);

        s.store_div_ad(793, A::scale(s.ad_value(245), 4e-18), A::square(s.ad_value(190)));

        s.store_div_ad(794, A::scale(s.ad_value(246), 4e-18), A::square(s.ad_value(191)));

        if ((1.0 + (s.v[249] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 249, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(711, 247, 785);

        s.store_scaled_mul(795, 711, 190, 500000000.0);

        if ((1.0 + (s.v[250] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 250, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(712, 248, 785);

        s.store_scaled_mul(796, 712, 191, 500000000.0);

        s.v[797] = 0.0;

        s.v[1143] = if (s.v[270] > 1e-10) { 1.0 } else { 0.0 };

        if (s.v[1143] != 0.0) {
            s.store_div_from_scalar(797, 0.75, 270);
        }

        s.store_square(798, 271);

        s.store_mul_ad_rhs(716, 305, A::pow_from_scalar(s.v[353], s.ad_value(307)));

        s.store_scale(799, 275, (9.1093826e-31 * 1000000000.0));

        s.v[1144] = if (s.v[298] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1144] != 0.0) {
            s.store_div_from_scalar(800, 1.0, 298);
        }

        if (!(s.v[1144] != 0.0)) {
            s.store_scalar(800, 0.0);
        }

        s.v[1145] = if (s.v[299] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1145] != 0.0) {
            s.store_div_from_scalar(801, 1.0, 299);
        }

        if (!(s.v[1145] != 0.0)) {
            s.store_scalar(801, 0.0);
        }

        s.v[1146] = if (s.v[300] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1146] != 0.0) {
            s.store_div_from_scalar(802, 1.0, 300);
        }

        if (!(s.v[1146] != 0.0)) {
            s.store_scalar(802, 0.0);
        }

        s.v[1147] = if (s.v[301] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1147] != 0.0) {
            s.store_div_from_scalar(803, 1.0, 301);
        }

        if (!(s.v[1147] != 0.0)) {
            s.store_scalar(803, 0.0);
        }

        s.v[1148] = if (s.v[302] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1148] != 0.0) {
            s.store_div_from_scalar(804, 1.0, 302);
        }

        if (!(s.v[1148] != 0.0)) {
            s.store_scalar(804, 0.0);
        }

        s.v[1149] = if (s.v[303] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1149] != 0.0) {
            s.store_div_from_scalar(805, 1.0, 303);
        }

        if (!(s.v[1149] != 0.0)) {
            s.store_scalar(805, 0.0);
        }

        s.v[1150] = if (s.v[304] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1150] != 0.0) {
            s.store_div_from_scalar(806, 1.0, 304);
        }

        if (!(s.v[1150] != 0.0)) {
            s.store_scalar(806, 0.0);
        }

        s.store_scale(20, 2, s.v[647]);

        s.store_scale(21, 2, s.v[648]);

        s.store_scale(22, 2, s.v[649]);

        s.store_scale(23, 2, s.v[674]);

        s.store_scale(24, 2, s.v[675]);

        s.store_scale(25, 2, s.v[676]);

        s.v[26] = 0.0;

        s.v[1151] = if (p.p43 == 3.0) { 1.0 } else { 0.0 };

        if (s.v[1151] != 0.0) {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 313);

        s.v[1152] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1152] != 0.0) {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.v[1153] = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

        if (s.v[1153] != 0.0) {
            s.store_scale(20, 2, s.v[650]);
        }

        if (s.v[1153] != 0.0) {
            s.store_sub_ad(21, A::scale(s.ad_value(2), s.v[651]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1153] != 0.0) {
            s.copy_ad(22, 27);
        }

        if (s.v[1153] != 0.0) {
            s.store_scale(23, 2, s.v[677]);
        }

        if (s.v[1153] != 0.0) {
            s.store_sub_ad(24, A::scale(s.ad_value(2), s.v[678]), A::mul(s.ad_value(26), s.ad_value(27)));
        }

        if (s.v[1153] != 0.0) {
            s.copy_ad(25, 27);
        }

        s.v[1154] = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };

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
        if (s.v[1154] != 0.0) {
            s.store_ad(647, &{
                if (s.v[20] > 0.0) {
                    s.ad_value(20)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(648, &{
                if (s.v[21] > 0.0) {
                    s.ad_value(21)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(649, &{
                if (s.v[22] > 0.0) {
                    s.ad_value(22)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(674, &{
                if (s.v[23] > 0.0) {
                    s.ad_value(23)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(675, &{
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1154] != 0.0) {
            s.store_ad(676, &{
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(649, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(674, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(675, 0.0);
        }

        if (!(s.v[1154] != 0.0)) {
            s.store_scalar(676, 0.0);
        }

        s.v[657] = 0.0;

        s.v[684] = 0.0;

        s.v[659] = 0.0;

        s.v[686] = 0.0;

        s.v[658] = 0.0;

        s.v[685] = 0.0;

        s.v[660] = 0.0;

        s.v[687] = 0.0;

        s.v[655] = 0.0;

        s.v[682] = 0.0;

        s.v[656] = 0.0;

        s.v[683] = 0.0;

        s.v[668] = 0.0;

        s.v[695] = 0.0;

        s.v[669] = 1.0;

        s.v[696] = 1.0;

        s.v[670] = 0.0;

        s.v[697] = 0.0;

        s.v[671] = 1.0;

        s.v[698] = 1.0;

        s.v[672] = 0.0;

        s.v[699] = 0.0;

        s.v[673] = 1.0;

        s.v[700] = 1.0;

        s.v[667] = 0.0;

        s.v[694] = 0.0;

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

        s.v[666] = 0.0;

        s.v[693] = 0.0;

        s.v[652] = 1.0;

        s.v[679] = 1.0;

        s.v[653] = 1.0;

        s.v[680] = 1.0;

        s.v[654] = 1.0;

        s.v[681] = 1.0;

        s.v[492] = 0.0;

        s.v[493] = 0.0;

        s.v[481] = 0.0;

        s.v[482] = 0.0;

        s.v[483] = 0.0;

        s.v[484] = 0.0;

        s.v[485] = 0.0;

        s.v[494] = 0.0;

        s.v[495] = 0.0;

        s.v[496] = 0.0;

        s.v[502] = 0.0;

        s.v[491] = 0.0;

        s.v[1155] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[1156] = if ((s.v[388] * s.v[647]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1156] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(647), s.v[388])), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1156] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1157] = if ((s.v[389] * s.v[648]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1157] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(648), s.v[389])), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1157] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        s.v[1158] = if ((s.v[390] * s.v[649]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1158] != 0.0)) {
            s.store_scale_ad(457, A::ln(A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(649), s.v[390])), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1158] != 0.0))) {
            s.store_scalar(457, 100000000.0);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(655, &A::min(A::min(s.ad_value(455), s.ad_value(456)), s.ad_value(457)));
        }

        s.v[1159] = if ((((s.v[655] * s.v[372])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1159] != 0.0)) {
            s.store_exp_ad(656, A::scale(s.ad_value(655), s.v[372]));
        }

        s.v[1160] = if ((s.v[655] * s.v[372]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (!(s.v[1159] != 0.0))) && (s.v[1160] != 0.0)) {
            s.store_div_from_scalar_ad(656, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(655), s.v[372])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(655), s.v[372])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(655), s.v[372])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1155] != 0.0) && (!(s.v[1159] != 0.0))) && (!(s.v[1160] != 0.0))) {
            s.store_scale_ad(656, A::offset(A::mul(A::offset(A::scale(s.ad_value(655), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(655), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(655), s.v[372]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(397, s.v[394]);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(398, s.v[395]);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(399, s.v[396]);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(400, p.p848);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(401, p.p849);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(402, p.p850);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(403, p.p845);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(404, p.p846);
        }

        if (s.v[1155] != 0.0) {
            s.store_scalar(405, p.p847);
        }

        s.v[1161] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scalar(397, (s.v[395] + s.v[396]));
        }

        if ((s.v[1155] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scalar(400, (0.9 * (p.p849).min(p.p850)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1161] != 0.0)) {
            s.store_scalar(403, (p.p846 + p.p847));
        }

        s.v[1162] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scalar(398, (s.v[394] + s.v[396]));
        }

        if ((s.v[1155] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scalar(401, (0.9 * (p.p848).min(p.p850)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1162] != 0.0)) {
            s.store_scalar(404, (p.p845 + p.p847));
        }

        s.v[1163] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_scalar(399, (s.v[394] + s.v[395]));
        }

        if ((s.v[1155] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_scalar(402, (0.9 * (p.p848).min(p.p849)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1163] != 0.0)) {
            s.store_scalar(405, (p.p845 + p.p846));
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(657, &A::min(A::min(s.ad_value(397), s.ad_value(398)), s.ad_value(399)));
        }

        if (s.v[1155] != 0.0) {
            s.store_scale(658, 657, 0.1);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(378, &A::max(A::max(s.ad_value(400), s.ad_value(401)), s.ad_value(402)));
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_rhs(659, 657, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378)))));
        }

        if (s.v[1155] != 0.0) {
            s.store_offset_ad(660, A::min(A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405)), (-0.05));
        }

        s.v[1164] = if ((s.v[564] * s.v[674]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1164] != 0.0)) {
            s.store_scale_ad(455, A::ln(A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(564), s.ad_value(674))), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1164] != 0.0))) {
            s.store_scalar(455, 100000000.0);
        }

        s.v[1165] = if ((s.v[565] * s.v[675]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1165] != 0.0)) {
            s.store_scale_ad(456, A::ln(A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(565), s.ad_value(675))), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1165] != 0.0))) {
            s.store_scalar(456, 100000000.0);
        }

        s.v[1166] = if ((s.v[566] * s.v[676]) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1166] != 0.0)) {
            s.store_scale_ad(457, A::ln(A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(566), s.ad_value(676))), 1.0)), s.v[371]);
        }

        if ((s.v[1155] != 0.0) && (!(s.v[1166] != 0.0))) {
            s.store_scalar(457, 100000000.0);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(682, &A::min(A::min(s.ad_value(455), s.ad_value(456)), s.ad_value(457)));
        }

        s.v[1167] = if ((((s.v[682] * s.v[372])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1167] != 0.0)) {
            s.store_exp_ad(683, A::scale(s.ad_value(682), s.v[372]));
        }

        s.v[1168] = if ((s.v[682] * s.v[372]) < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (!(s.v[1167] != 0.0))) && (s.v[1168] != 0.0)) {
            s.store_div_from_scalar_ad(683, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(682), s.v[372])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(682), s.v[372])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(682), s.v[372])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[1155] != 0.0) && (!(s.v[1167] != 0.0))) && (!(s.v[1168] != 0.0))) {
            s.store_scale_ad(683, A::offset(A::mul(A::offset(A::scale(s.ad_value(682), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(682), s.v[372]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(682), s.v[372]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(397, 570);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(398, 571);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(399, 572);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(400, 512);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(401, 513);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(402, 514);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(403, 509);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(404, 510);
        }

        if (s.v[1155] != 0.0) {
            s.copy_ad(405, 511);
        }

        s.v[1169] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_add(397, 571, 572);
        }

        if ((s.v[1155] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_scale_ad(400, A::min(s.ad_value(513), s.ad_value(514)), 0.9);
        }

        if ((s.v[1155] != 0.0) && (s.v[1169] != 0.0)) {
            s.store_add(403, 510, 511);
        }

        s.v[1170] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_add(398, 570, 572);
        }

        if ((s.v[1155] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_scale_ad(401, A::min(s.ad_value(512), s.ad_value(514)), 0.9);
        }

        if ((s.v[1155] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_add(404, 509, 511);
        }

        s.v[1171] = if (s.v[676] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_add(399, 570, 571);
        }

        if ((s.v[1155] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_scale_ad(402, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
        }

        if ((s.v[1155] != 0.0) && (s.v[1171] != 0.0)) {
            s.store_add(405, 509, 510);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(684, &A::min(A::min(s.ad_value(397), s.ad_value(398)), s.ad_value(399)));
        }

        if (s.v[1155] != 0.0) {
            s.store_scale(685, 684, 0.1);
        }

        if (s.v[1155] != 0.0) {
            s.store_ad(378, &A::max(A::max(s.ad_value(400), s.ad_value(401)), s.ad_value(402)));
        }

        if (s.v[1155] != 0.0) {
            s.store_mul_ad_rhs(686, 684, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378)))));
        }

        if (s.v[1155] != 0.0) {
            s.store_offset_ad(687, A::min(A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405)), (-0.05));
        }

        s.v[1172] = if (s.v[475] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1173, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1174, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1175, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1182, 0.0);
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
        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1184, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1185, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1186, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1187, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1188, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1190, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1191, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1192, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1194, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1195, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1197, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1198, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1199, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1201, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1202, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1203, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1204, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1205, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1206, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1207, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1208, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1209, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1210, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1211, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1212, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1213, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1214, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1216, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1217, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(499, 0.4);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(500, 0.65);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(501, 0.8);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scale_ad(486, A::neg(s.ad_value(499)), p.p945);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scale_ad(487, A::neg(s.ad_value(500)), p.p945);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scale_ad(488, A::neg(s.ad_value(501)), p.p945);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(489, 0.1);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(490, 0.2);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1186, 0.0);
        }

        s.v[1221] = if !(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1222] = if (s.v[486] < s.v[655]) { 1.0 } else { 0.0 };

        s.v[1223] = if (((((-0.5) * (s.v[486] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (s.v[1222] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_exp_ad(1184, A::scale(s.ad_value(486), (s.v[372] * (-0.5))));
        }

        s.v[1224] = if (((-0.5) * (s.v[486] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (s.v[1222] != 0.0)) && (!(s.v[1223] != 0.0))) && (s.v[1224] != 0.0)) {
            let assign14560_ad_e12416: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1184, &assign14560_ad_e12416);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (s.v[1222] != 0.0)) && (!(s.v[1223] != 0.0))) && (!(s.v[1224] != 0.0))) {
            s.store_scale_ad(1184, A::offset(A::mul(A::offset(A::scale(s.ad_value(486), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(486), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(486), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (s.v[1222] != 0.0)) {
            s.store_div_from_scalar(1185, 1.0, 1184);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (s.v[1222] != 0.0)) {
            s.store_square(1182, 1185);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (!(s.v[1222] != 0.0))) {
            s.store_mul_ad_lhs(1182, A::offset(A::scale(A::sub(s.ad_value(486), s.ad_value(655)), s.v[372]), 1.0), 656);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (!(s.v[1222] != 0.0))) {
            s.store_sqrt(1185, 1182);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (!(s.v[1222] != 0.0))) {
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.v[1225] = if (s.v[486] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (s.v[1225] != 0.0)) {
            s.store_scale_ad(1186, A::ln(A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) && (!(s.v[1225] != 0.0))) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1185), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::offset(A::scale(s.ad_value(1185), 3.0), 1.0))))), (s.v[371] * 2.0)), 486);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_sub(1187, 657, 1186);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_scale_ad(1188, A::sub(A::add(s.ad_value(486), s.ad_value(1187)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(1187)), A::sub(s.ad_value(486), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_scale_ad(1189, A::sub(A::add(s.ad_value(486), s.ad_value(660)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(660)), A::sub(s.ad_value(486), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1221] != 0.0)) {
            s.store_scale_ad(1190, A::sub(s.ad_value(486), A::sqrt(A::offset(A::mul(s.ad_value(486), s.ad_value(486)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1226] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1226] != 0.0)) {
            s.store_scalar(1218, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.v[1227] = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (s.v[1227] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1228] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) && (s.v[1228] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) && (!(s.v[1228] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p848)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1229] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) && (s.v[1229] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) && (!(s.v[1229] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) {
            s.store_scale(1198, 1191, s.v[424]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[385]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1227] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.v[1230] = if (p.p862 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (s.v[1230] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[409]), s.ad_value(1194)), s.v[439]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1231] = if (((-p.p848) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (s.v[1231] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (!(s.v[1231] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[436]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[436])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1232] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (s.v[1232] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (!(s.v[1232] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1233] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (s.v[1233] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (!(s.v[1233] != 0.0))) {
            let assign15100_ad_e13312: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign15100_ad_e13312);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1234] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (s.v[1234] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1235] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (s.v[1235] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) && (!(s.v[1235] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) && (!(s.v[1234] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[436]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1230] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p862);
        }

        s.v[1236] = if (p.p868 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (s.v[1236] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1237] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1236] != 0.0))) && (s.v[1237] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1236] != 0.0))) && (!(s.v[1237] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1236] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[427]), s.ad_value(1191)), s.v[412]);
        }

        s.v[1238] = if (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1236] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(442)), s.ad_value(1216)));
        }

        s.v[1239] = if (((-s.v[442]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1236] != 0.0))) && (!(s.v[1238] != 0.0))) && (s.v[1239] != 0.0)) {
            let assign15290_ad_e13639: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign15290_ad_e13639);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1236] != 0.0))) && (!(s.v[1238] != 0.0))) && (!(s.v[1239] != 0.0))) {
            let assign15300_ad_e13689: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign15300_ad_e13689);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1236] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p868);
        }

        s.v[1240] = if (p.p877 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (s.v[1240] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1241] = if (s.v[1190] > ((-s.v[445]) * p.p877)) { 1.0 } else { 0.0 };

        s.v[1242] = if (p.p880 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1240] != 0.0))) && (s.v[1241] != 0.0)) && (s.v[1242] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[449]), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1240] != 0.0))) && (s.v[1241] != 0.0)) && (!(s.v[1242] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[449])), p.p880);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1240] != 0.0))) && (s.v[1241] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) && (!(s.v[1240] != 0.0))) && (!(s.v[1241] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p877)), s.v[452]), s.v[446]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1226] != 0.0))) {
            s.store_mul_ad_lhs(1218, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1243] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1243] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.v[1244] = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1244] != 0.0)) {
            s.store_scalar(1193, 0.0);
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
        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1245] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) && (s.v[1245] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) && (!(s.v[1245] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p849)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1246] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) && (s.v[1246] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) && (!(s.v[1246] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scale(1198, 1191, s.v[425]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[386]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1244] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.v[1247] = if (p.p863 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1247] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[410]), s.ad_value(1194)), s.v[440]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1248] = if (((-p.p849) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (s.v[1248] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (!(s.v[1248] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[437]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[437])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1249] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (s.v[1249] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (!(s.v[1249] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1250] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (s.v[1250] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (!(s.v[1250] != 0.0))) {
            let assign15800_ad_e14455: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign15800_ad_e14455);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1251] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (s.v[1251] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1252] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (s.v[1252] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) && (!(s.v[1252] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) && (!(s.v[1251] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[437]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1247] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p863);
        }

        s.v[1253] = if (p.p869 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1253] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1254] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1253] != 0.0))) && (s.v[1254] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1253] != 0.0))) && (!(s.v[1254] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1253] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[428]), s.ad_value(1191)), s.v[413]);
        }

        s.v[1255] = if (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1253] != 0.0))) && (s.v[1255] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(443)), s.ad_value(1216)));
        }

        s.v[1256] = if (((-s.v[443]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1253] != 0.0))) && (!(s.v[1255] != 0.0))) && (s.v[1256] != 0.0)) {
            let assign15990_ad_e14782: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign15990_ad_e14782);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1253] != 0.0))) && (!(s.v[1255] != 0.0))) && (!(s.v[1256] != 0.0))) {
            let assign16000_ad_e14832: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign16000_ad_e14832);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1253] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p869);
        }

        s.v[1257] = if (p.p878 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (s.v[1257] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1258] = if (s.v[1190] > ((-s.v[445]) * p.p878)) { 1.0 } else { 0.0 };

        s.v[1259] = if (p.p881 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (s.v[1259] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[450]), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) && (!(s.v[1259] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[450])), p.p881);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1257] != 0.0))) && (s.v[1258] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) && (!(s.v[1257] != 0.0))) && (!(s.v[1258] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p878)), s.v[453]), s.v[447]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1243] != 0.0))) {
            s.store_mul_ad_lhs(1219, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1260] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1260] != 0.0)) {
            s.store_scalar(1220, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.v[1261] = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (s.v[1261] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1262] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) && (s.v[1262] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) && (!(s.v[1262] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p850)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1263] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) && (s.v[1263] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) && (!(s.v[1263] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_scale(1198, 1191, s.v[426]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[387]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1261] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.v[1264] = if (p.p864 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (s.v[1264] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[411]), s.ad_value(1194)), s.v[441]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1265] = if (((-p.p850) * s.v[414]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (s.v[1265] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (!(s.v[1265] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[438]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[438])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1266] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (s.v[1266] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (!(s.v[1266] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1267] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (s.v[1267] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (!(s.v[1267] != 0.0))) {
            let assign16500_ad_e15598: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign16500_ad_e15598);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1268] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (s.v[1268] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1269] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (s.v[1269] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) && (!(s.v[1269] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) && (!(s.v[1268] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[438]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1264] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p864);
        }

        s.v[1270] = if (p.p870 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (s.v[1270] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1271] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1270] != 0.0))) && (s.v[1271] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1270] != 0.0))) && (!(s.v[1271] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1270] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[429]), s.ad_value(1191)), s.v[414]);
        }

        s.v[1272] = if (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1270] != 0.0))) && (s.v[1272] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(444)), s.ad_value(1216)));
        }

        s.v[1273] = if (((-s.v[444]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1270] != 0.0))) && (!(s.v[1272] != 0.0))) && (s.v[1273] != 0.0)) {
            let assign16690_ad_e15925: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign16690_ad_e15925);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1270] != 0.0))) && (!(s.v[1272] != 0.0))) && (!(s.v[1273] != 0.0))) {
            let assign16700_ad_e15975: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign16700_ad_e15975);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1270] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(486), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p870);
        }

        s.v[1274] = if (p.p879 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (s.v[1274] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1275] = if (s.v[1190] > ((-s.v[445]) * p.p879)) { 1.0 } else { 0.0 };

        s.v[1276] = if (p.p882 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1275] != 0.0)) && (s.v[1276] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[451]), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1275] != 0.0)) && (!(s.v[1276] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[451])), p.p882);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1274] != 0.0))) && (s.v[1275] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) && (!(s.v[1274] != 0.0))) && (!(s.v[1275] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p879)), s.v[454]), s.v[448]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1260] != 0.0))) {
            s.store_mul_ad_lhs(1220, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_add_ad(476, A::add(A::mul(s.ad_value(647), s.ad_value(1218)), A::mul(s.ad_value(648), s.ad_value(1219))), A::mul(s.ad_value(649), s.ad_value(1220)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1186, 0.0);
        }

        s.v[1277] = if !(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1278] = if (s.v[487] < s.v[655]) { 1.0 } else { 0.0 };

        s.v[1279] = if (((((-0.5) * (s.v[487] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (s.v[1278] != 0.0)) && (s.v[1279] != 0.0)) {
            s.store_exp_ad(1184, A::scale(s.ad_value(487), (s.v[372] * (-0.5))));
        }

        s.v[1280] = if (((-0.5) * (s.v[487] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (s.v[1278] != 0.0)) && (!(s.v[1279] != 0.0))) && (s.v[1280] != 0.0)) {
            let assign16960_ad_e16346: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1184, &assign16960_ad_e16346);
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
        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (s.v[1278] != 0.0)) && (!(s.v[1279] != 0.0))) && (!(s.v[1280] != 0.0))) {
            s.store_scale_ad(1184, A::offset(A::mul(A::offset(A::scale(s.ad_value(487), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(487), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(487), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (s.v[1278] != 0.0)) {
            s.store_div_from_scalar(1185, 1.0, 1184);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (s.v[1278] != 0.0)) {
            s.store_square(1182, 1185);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (!(s.v[1278] != 0.0))) {
            s.store_mul_ad_lhs(1182, A::offset(A::scale(A::sub(s.ad_value(487), s.ad_value(655)), s.v[372]), 1.0), 656);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (!(s.v[1278] != 0.0))) {
            s.store_sqrt(1185, 1182);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (!(s.v[1278] != 0.0))) {
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.v[1281] = if (s.v[487] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_scale_ad(1186, A::ln(A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) && (!(s.v[1281] != 0.0))) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1185), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::offset(A::scale(s.ad_value(1185), 3.0), 1.0))))), (s.v[371] * 2.0)), 487);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) {
            s.store_sub(1187, 657, 1186);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) {
            s.store_scale_ad(1188, A::sub(A::add(s.ad_value(487), s.ad_value(1187)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(1187)), A::sub(s.ad_value(487), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) {
            s.store_scale_ad(1189, A::sub(A::add(s.ad_value(487), s.ad_value(660)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(660)), A::sub(s.ad_value(487), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1277] != 0.0)) {
            s.store_scale_ad(1190, A::sub(s.ad_value(487), A::sqrt(A::offset(A::mul(s.ad_value(487), s.ad_value(487)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1282] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1282] != 0.0)) {
            s.store_scalar(1218, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.v[1283] = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (s.v[1283] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1284] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) && (s.v[1284] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) && (!(s.v[1284] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p848)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1285] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) && (s.v[1285] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) && (!(s.v[1285] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_scale(1198, 1191, s.v[424]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[385]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1283] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.v[1286] = if (p.p862 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (s.v[1286] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[409]), s.ad_value(1194)), s.v[439]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1287] = if (((-p.p848) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (s.v[1287] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1287] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[436]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[436])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1288] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (s.v[1288] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1288] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1289] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (s.v[1289] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1289] != 0.0))) {
            let assign17500_ad_e17242: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign17500_ad_e17242);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1290] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (s.v[1290] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1291] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (s.v[1291] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) && (!(s.v[1291] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) && (!(s.v[1290] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[436]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1286] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p862);
        }

        s.v[1292] = if (p.p868 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (s.v[1292] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1293] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1292] != 0.0))) && (s.v[1293] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1292] != 0.0))) && (!(s.v[1293] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[427]), s.ad_value(1191)), s.v[412]);
        }

        s.v[1294] = if (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1292] != 0.0))) && (s.v[1294] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(442)), s.ad_value(1216)));
        }

        s.v[1295] = if (((-s.v[442]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1292] != 0.0))) && (!(s.v[1294] != 0.0))) && (s.v[1295] != 0.0)) {
            let assign17690_ad_e17569: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign17690_ad_e17569);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1292] != 0.0))) && (!(s.v[1294] != 0.0))) && (!(s.v[1295] != 0.0))) {
            let assign17700_ad_e17619: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign17700_ad_e17619);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1292] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(487), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p868);
        }

        s.v[1296] = if (p.p877 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (s.v[1296] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1297] = if (s.v[1190] > ((-s.v[445]) * p.p877)) { 1.0 } else { 0.0 };

        s.v[1298] = if (p.p880 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1296] != 0.0))) && (s.v[1297] != 0.0)) && (s.v[1298] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[449]), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1296] != 0.0))) && (s.v[1297] != 0.0)) && (!(s.v[1298] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[449])), p.p880);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1296] != 0.0))) && (s.v[1297] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) && (!(s.v[1296] != 0.0))) && (!(s.v[1297] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p877)), s.v[452]), s.v[446]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1282] != 0.0))) {
            s.store_mul_ad_lhs(1218, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1299] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1299] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.v[1300] = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (s.v[1300] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1301] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) && (s.v[1301] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) && (!(s.v[1301] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p849)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1302] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) && (s.v[1302] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) && (!(s.v[1302] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_scale(1198, 1191, s.v[425]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[386]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1300] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.v[1303] = if (p.p863 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (s.v[1303] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[410]), s.ad_value(1194)), s.v[440]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1304] = if (((-p.p849) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (s.v[1304] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1304] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[437]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[437])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1305] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (s.v[1305] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1305] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1306] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (s.v[1306] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1306] != 0.0))) {
            let assign18200_ad_e18385: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign18200_ad_e18385);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1307] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (s.v[1307] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1308] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (s.v[1308] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) && (!(s.v[1308] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) && (!(s.v[1307] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[437]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1303] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p863);
        }

        s.v[1309] = if (p.p869 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (s.v[1309] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1310] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1310] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1310] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[428]), s.ad_value(1191)), s.v[413]);
        }

        s.v[1311] = if (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1309] != 0.0))) && (s.v[1311] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(443)), s.ad_value(1216)));
        }

        s.v[1312] = if (((-s.v[443]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

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
        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1311] != 0.0))) && (s.v[1312] != 0.0)) {
            let assign18390_ad_e18712: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign18390_ad_e18712);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1309] != 0.0))) && (!(s.v[1311] != 0.0))) && (!(s.v[1312] != 0.0))) {
            let assign18400_ad_e18762: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign18400_ad_e18762);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1309] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(487), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p869);
        }

        s.v[1313] = if (p.p878 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (s.v[1313] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1314] = if (s.v[1190] > ((-s.v[445]) * p.p878)) { 1.0 } else { 0.0 };

        s.v[1315] = if (p.p881 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) && (s.v[1315] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[450]), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) && (!(s.v[1315] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[450])), p.p881);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1313] != 0.0))) && (s.v[1314] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) && (!(s.v[1313] != 0.0))) && (!(s.v[1314] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p878)), s.v[453]), s.v[447]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1299] != 0.0))) {
            s.store_mul_ad_lhs(1219, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1316] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1316] != 0.0)) {
            s.store_scalar(1220, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.v[1317] = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (s.v[1317] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1318] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) && (s.v[1318] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) && (!(s.v[1318] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p850)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1319] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) && (s.v[1319] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) && (!(s.v[1319] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_scale(1198, 1191, s.v[426]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[387]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1317] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.v[1320] = if (p.p864 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (s.v[1320] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[411]), s.ad_value(1194)), s.v[441]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1321] = if (((-p.p850) * s.v[414]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (s.v[1321] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (!(s.v[1321] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[438]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[438])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1322] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (s.v[1322] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (!(s.v[1322] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1323] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (s.v[1323] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (!(s.v[1323] != 0.0))) {
            let assign18900_ad_e19528: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign18900_ad_e19528);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1324] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (s.v[1324] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1325] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (s.v[1325] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) && (!(s.v[1325] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) && (!(s.v[1324] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[438]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1320] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p864);
        }

        s.v[1326] = if (p.p870 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (s.v[1326] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1327] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1326] != 0.0))) && (s.v[1327] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1326] != 0.0))) && (!(s.v[1327] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1326] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[429]), s.ad_value(1191)), s.v[414]);
        }

        s.v[1328] = if (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1326] != 0.0))) && (s.v[1328] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(444)), s.ad_value(1216)));
        }

        s.v[1329] = if (((-s.v[444]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1326] != 0.0))) && (!(s.v[1328] != 0.0))) && (s.v[1329] != 0.0)) {
            let assign19090_ad_e19855: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign19090_ad_e19855);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1326] != 0.0))) && (!(s.v[1328] != 0.0))) && (!(s.v[1329] != 0.0))) {
            let assign19100_ad_e19905: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign19100_ad_e19905);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1326] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(487), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p870);
        }

        s.v[1330] = if (p.p879 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (s.v[1330] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1331] = if (s.v[1190] > ((-s.v[445]) * p.p879)) { 1.0 } else { 0.0 };

        s.v[1332] = if (p.p882 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1330] != 0.0))) && (s.v[1331] != 0.0)) && (s.v[1332] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[451]), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1330] != 0.0))) && (s.v[1331] != 0.0)) && (!(s.v[1332] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[451])), p.p882);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1330] != 0.0))) && (s.v[1331] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) && (!(s.v[1330] != 0.0))) && (!(s.v[1331] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p879)), s.v[454]), s.v[448]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1316] != 0.0))) {
            s.store_mul_ad_lhs(1220, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_add_ad(477, A::add(A::mul(s.ad_value(647), s.ad_value(1218)), A::mul(s.ad_value(648), s.ad_value(1219))), A::mul(s.ad_value(649), s.ad_value(1220)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1186, 0.0);
        }

        s.v[1333] = if !(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1334] = if (s.v[488] < s.v[655]) { 1.0 } else { 0.0 };

        s.v[1335] = if (((((-0.5) * (s.v[488] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (s.v[1334] != 0.0)) && (s.v[1335] != 0.0)) {
            s.store_exp_ad(1184, A::scale(s.ad_value(488), (s.v[372] * (-0.5))));
        }

        s.v[1336] = if (((-0.5) * (s.v[488] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (s.v[1334] != 0.0)) && (!(s.v[1335] != 0.0))) && (s.v[1336] != 0.0)) {
            let assign19360_ad_e20276: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1184, &assign19360_ad_e20276);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (s.v[1334] != 0.0)) && (!(s.v[1335] != 0.0))) && (!(s.v[1336] != 0.0))) {
            s.store_scale_ad(1184, A::offset(A::mul(A::offset(A::scale(s.ad_value(488), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(488), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(488), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (s.v[1334] != 0.0)) {
            s.store_div_from_scalar(1185, 1.0, 1184);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (s.v[1334] != 0.0)) {
            s.store_square(1182, 1185);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (!(s.v[1334] != 0.0))) {
            s.store_mul_ad_lhs(1182, A::offset(A::scale(A::sub(s.ad_value(488), s.ad_value(655)), s.v[372]), 1.0), 656);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (!(s.v[1334] != 0.0))) {
            s.store_sqrt(1185, 1182);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (!(s.v[1334] != 0.0))) {
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.v[1337] = if (s.v[488] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (s.v[1337] != 0.0)) {
            s.store_scale_ad(1186, A::ln(A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) && (!(s.v[1337] != 0.0))) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1185), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::offset(A::scale(s.ad_value(1185), 3.0), 1.0))))), (s.v[371] * 2.0)), 488);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) {
            s.store_sub(1187, 657, 1186);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) {
            s.store_scale_ad(1188, A::sub(A::add(s.ad_value(488), s.ad_value(1187)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(1187)), A::sub(s.ad_value(488), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) {
            s.store_scale_ad(1189, A::sub(A::add(s.ad_value(488), s.ad_value(660)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(660)), A::sub(s.ad_value(488), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1333] != 0.0)) {
            s.store_scale_ad(1190, A::sub(s.ad_value(488), A::sqrt(A::offset(A::mul(s.ad_value(488), s.ad_value(488)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1338] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1338] != 0.0)) {
            s.store_scalar(1218, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.v[1339] = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (s.v[1339] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1340] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) && (s.v[1340] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) && (!(s.v[1340] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p848)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1341] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) && (s.v[1341] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) && (!(s.v[1341] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) {
            s.store_scale(1198, 1191, s.v[424]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[385]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1339] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.v[1342] = if (p.p862 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (s.v[1342] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[409]), s.ad_value(1194)), s.v[439]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1343] = if (((-p.p848) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[436]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[436])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1344] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1344] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1344] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1345] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

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
        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1345] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1345] != 0.0))) {
            let assign19900_ad_e21172: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign19900_ad_e21172);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1346] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (s.v[1346] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1347] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (s.v[1347] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) && (!(s.v[1347] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) && (!(s.v[1346] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[436]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1342] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p862);
        }

        s.v[1348] = if (p.p868 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (s.v[1348] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1349] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1348] != 0.0))) && (s.v[1349] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1348] != 0.0))) && (!(s.v[1349] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1348] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[427]), s.ad_value(1191)), s.v[412]);
        }

        s.v[1350] = if (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1348] != 0.0))) && (s.v[1350] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(442)), s.ad_value(1216)));
        }

        s.v[1351] = if (((-s.v[442]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1348] != 0.0))) && (!(s.v[1350] != 0.0))) && (s.v[1351] != 0.0)) {
            let assign20090_ad_e21499: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign20090_ad_e21499);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1348] != 0.0))) && (!(s.v[1350] != 0.0))) && (!(s.v[1351] != 0.0))) {
            let assign20100_ad_e21549: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign20100_ad_e21549);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1348] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(488), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p868);
        }

        s.v[1352] = if (p.p877 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (s.v[1352] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1353] = if (s.v[1190] > ((-s.v[445]) * p.p877)) { 1.0 } else { 0.0 };

        s.v[1354] = if (p.p880 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1352] != 0.0))) && (s.v[1353] != 0.0)) && (s.v[1354] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[449]), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1352] != 0.0))) && (s.v[1353] != 0.0)) && (!(s.v[1354] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[449])), p.p880);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1352] != 0.0))) && (s.v[1353] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) && (!(s.v[1352] != 0.0))) && (!(s.v[1353] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p877)), s.v[452]), s.v[446]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_mul_ad_lhs(1218, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1355] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1355] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.v[1356] = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (s.v[1356] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1357] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) && (s.v[1357] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) && (!(s.v[1357] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p849)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1358] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) && (s.v[1358] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) && (!(s.v[1358] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) {
            s.store_scale(1198, 1191, s.v[425]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[386]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1356] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.v[1359] = if (p.p863 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (s.v[1359] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[410]), s.ad_value(1194)), s.v[440]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1360] = if (((-p.p849) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (s.v[1360] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (!(s.v[1360] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[437]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[437])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1361] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (!(s.v[1361] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1362] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (s.v[1362] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (!(s.v[1362] != 0.0))) {
            let assign20600_ad_e22315: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign20600_ad_e22315);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1363] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (s.v[1363] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1364] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (s.v[1364] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) && (!(s.v[1364] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) && (!(s.v[1363] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[437]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1359] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p863);
        }

        s.v[1365] = if (p.p869 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (s.v[1365] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1366] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1365] != 0.0))) && (s.v[1366] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1365] != 0.0))) && (!(s.v[1366] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[428]), s.ad_value(1191)), s.v[413]);
        }

        s.v[1367] = if (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1365] != 0.0))) && (s.v[1367] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(443)), s.ad_value(1216)));
        }

        s.v[1368] = if (((-s.v[443]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1365] != 0.0))) && (!(s.v[1367] != 0.0))) && (s.v[1368] != 0.0)) {
            let assign20790_ad_e22642: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign20790_ad_e22642);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1365] != 0.0))) && (!(s.v[1367] != 0.0))) && (!(s.v[1368] != 0.0))) {
            let assign20800_ad_e22692: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign20800_ad_e22692);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1365] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(488), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p869);
        }

        s.v[1369] = if (p.p878 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (s.v[1369] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1370] = if (s.v[1190] > ((-s.v[445]) * p.p878)) { 1.0 } else { 0.0 };

        s.v[1371] = if (p.p881 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1369] != 0.0))) && (s.v[1370] != 0.0)) && (s.v[1371] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[450]), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1369] != 0.0))) && (s.v[1370] != 0.0)) && (!(s.v[1371] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[450])), p.p881);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1369] != 0.0))) && (s.v[1370] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) && (!(s.v[1369] != 0.0))) && (!(s.v[1370] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p878)), s.v[453]), s.v[447]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1355] != 0.0))) {
            s.store_mul_ad_lhs(1219, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1372] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1372] != 0.0)) {
            s.store_scalar(1220, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.v[1373] = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (s.v[1373] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1374] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) && (s.v[1374] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) && (!(s.v[1374] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p850)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1375] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) && (s.v[1375] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) && (!(s.v[1375] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_scale(1198, 1191, s.v[426]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[387]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1373] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.v[1376] = if (p.p864 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (s.v[1376] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[411]), s.ad_value(1194)), s.v[441]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1377] = if (((-p.p850) * s.v[414]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (s.v[1377] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (!(s.v[1377] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[438]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[438])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1378] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (s.v[1378] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (!(s.v[1378] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1379] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (s.v[1379] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (!(s.v[1379] != 0.0))) {
            let assign21300_ad_e23458: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign21300_ad_e23458);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
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
        s.v[1380] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (s.v[1380] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1381] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (s.v[1381] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) && (!(s.v[1381] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) && (!(s.v[1380] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[438]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1376] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p864);
        }

        s.v[1382] = if (p.p870 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (s.v[1382] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1383] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1382] != 0.0))) && (s.v[1383] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1382] != 0.0))) && (!(s.v[1383] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1382] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[429]), s.ad_value(1191)), s.v[414]);
        }

        s.v[1384] = if (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1382] != 0.0))) && (s.v[1384] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(444)), s.ad_value(1216)));
        }

        s.v[1385] = if (((-s.v[444]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1382] != 0.0))) && (!(s.v[1384] != 0.0))) && (s.v[1385] != 0.0)) {
            let assign21490_ad_e23785: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign21490_ad_e23785);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1382] != 0.0))) && (!(s.v[1384] != 0.0))) && (!(s.v[1385] != 0.0))) {
            let assign21500_ad_e23835: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign21500_ad_e23835);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1382] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(488), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p870);
        }

        s.v[1386] = if (p.p879 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (s.v[1386] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1387] = if (s.v[1190] > ((-s.v[445]) * p.p879)) { 1.0 } else { 0.0 };

        s.v[1388] = if (p.p882 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1386] != 0.0))) && (s.v[1387] != 0.0)) && (s.v[1388] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[451]), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1386] != 0.0))) && (s.v[1387] != 0.0)) && (!(s.v[1388] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[451])), p.p882);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1386] != 0.0))) && (s.v[1387] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) && (!(s.v[1386] != 0.0))) && (!(s.v[1387] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p879)), s.v[454]), s.v[448]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1372] != 0.0))) {
            s.store_mul_ad_lhs(1220, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_add_ad(478, A::add(A::mul(s.ad_value(647), s.ad_value(1218)), A::mul(s.ad_value(648), s.ad_value(1219))), A::mul(s.ad_value(649), s.ad_value(1220)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1186, 0.0);
        }

        s.v[1389] = if !(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1390] = if (s.v[489] < s.v[655]) { 1.0 } else { 0.0 };

        s.v[1391] = if (((((-0.5) * (s.v[489] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (s.v[1390] != 0.0)) && (s.v[1391] != 0.0)) {
            s.store_exp_ad(1184, A::scale(s.ad_value(489), (s.v[372] * (-0.5))));
        }

        s.v[1392] = if (((-0.5) * (s.v[489] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (s.v[1390] != 0.0)) && (!(s.v[1391] != 0.0))) && (s.v[1392] != 0.0)) {
            let assign21760_ad_e24206: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1184, &assign21760_ad_e24206);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (s.v[1390] != 0.0)) && (!(s.v[1391] != 0.0))) && (!(s.v[1392] != 0.0))) {
            s.store_scale_ad(1184, A::offset(A::mul(A::offset(A::scale(s.ad_value(489), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(489), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(489), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (s.v[1390] != 0.0)) {
            s.store_div_from_scalar(1185, 1.0, 1184);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (s.v[1390] != 0.0)) {
            s.store_square(1182, 1185);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (!(s.v[1390] != 0.0))) {
            s.store_mul_ad_lhs(1182, A::offset(A::scale(A::sub(s.ad_value(489), s.ad_value(655)), s.v[372]), 1.0), 656);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (!(s.v[1390] != 0.0))) {
            s.store_sqrt(1185, 1182);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (!(s.v[1390] != 0.0))) {
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.v[1393] = if (s.v[489] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (s.v[1393] != 0.0)) {
            s.store_scale_ad(1186, A::ln(A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) && (!(s.v[1393] != 0.0))) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1185), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::offset(A::scale(s.ad_value(1185), 3.0), 1.0))))), (s.v[371] * 2.0)), 489);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) {
            s.store_sub(1187, 657, 1186);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) {
            s.store_scale_ad(1188, A::sub(A::add(s.ad_value(489), s.ad_value(1187)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(1187)), A::sub(s.ad_value(489), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) {
            s.store_scale_ad(1189, A::sub(A::add(s.ad_value(489), s.ad_value(660)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(660)), A::sub(s.ad_value(489), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1389] != 0.0)) {
            s.store_scale_ad(1190, A::sub(s.ad_value(489), A::sqrt(A::offset(A::mul(s.ad_value(489), s.ad_value(489)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1394] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1394] != 0.0)) {
            s.store_scalar(1218, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.v[1395] = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1395] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1396] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) && (s.v[1396] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) && (!(s.v[1396] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p848)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1397] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) && (s.v[1397] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) && (!(s.v[1397] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) {
            s.store_scale(1198, 1191, s.v[424]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[385]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1395] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.v[1398] = if (p.p862 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1398] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[409]), s.ad_value(1194)), s.v[439]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1399] = if (((-p.p848) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (s.v[1399] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1399] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[436]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[436])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1400] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (s.v[1400] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1400] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1401] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (s.v[1401] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1401] != 0.0))) {
            let assign22300_ad_e25102: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign22300_ad_e25102);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1402] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (s.v[1402] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1403] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (s.v[1403] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) && (!(s.v[1403] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) && (!(s.v[1402] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[436]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1398] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p862);
        }

        s.v[1404] = if (p.p868 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1404] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1405] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1404] != 0.0))) && (s.v[1405] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1404] != 0.0))) && (!(s.v[1405] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1404] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[427]), s.ad_value(1191)), s.v[412]);
        }

        s.v[1406] = if (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1404] != 0.0))) && (s.v[1406] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(442)), s.ad_value(1216)));
        }

        s.v[1407] = if (((-s.v[442]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1404] != 0.0))) && (!(s.v[1406] != 0.0))) && (s.v[1407] != 0.0)) {
            let assign22490_ad_e25429: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign22490_ad_e25429);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1404] != 0.0))) && (!(s.v[1406] != 0.0))) && (!(s.v[1407] != 0.0))) {
            let assign22500_ad_e25479: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign22500_ad_e25479);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1404] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(489), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p868);
        }

        s.v[1408] = if (p.p877 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (s.v[1408] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1409] = if (s.v[1190] > ((-s.v[445]) * p.p877)) { 1.0 } else { 0.0 };

        s.v[1410] = if (p.p880 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1408] != 0.0))) && (s.v[1409] != 0.0)) && (s.v[1410] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[449]), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449])), A::scale(s.ad_value(1190), s.v[449]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1408] != 0.0))) && (s.v[1409] != 0.0)) && (!(s.v[1410] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[449])), p.p880);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1408] != 0.0))) && (s.v[1409] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) && (!(s.v[1408] != 0.0))) && (!(s.v[1409] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p877)), s.v[452]), s.v[446]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1394] != 0.0))) {
            s.store_mul_ad_lhs(1218, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1411] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1411] != 0.0)) {
            s.store_scalar(1219, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.v[1412] = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (s.v[1412] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1413] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) && (s.v[1413] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) && (!(s.v[1413] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p849)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1414] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) && (s.v[1414] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) && (!(s.v[1414] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) {
            s.store_scale(1198, 1191, s.v[425]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[386]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1412] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.v[1415] = if (p.p863 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (s.v[1415] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[410]), s.ad_value(1194)), s.v[440]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_square(1203, 1202);
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
        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1416] = if (((-p.p849) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (s.v[1416] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (!(s.v[1416] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[437]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[437])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1417] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (s.v[1417] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (!(s.v[1417] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1418] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (s.v[1418] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (!(s.v[1418] != 0.0))) {
            let assign23000_ad_e26245: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign23000_ad_e26245);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1419] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (s.v[1419] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1420] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (s.v[1420] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) && (!(s.v[1420] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) && (!(s.v[1419] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[437]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1415] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p863);
        }

        s.v[1421] = if (p.p869 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (s.v[1421] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1422] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1421] != 0.0))) && (s.v[1422] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1421] != 0.0))) && (!(s.v[1422] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1421] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[428]), s.ad_value(1191)), s.v[413]);
        }

        s.v[1423] = if (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1421] != 0.0))) && (s.v[1423] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(443)), s.ad_value(1216)));
        }

        s.v[1424] = if (((-s.v[443]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1421] != 0.0))) && (!(s.v[1423] != 0.0))) && (s.v[1424] != 0.0)) {
            let assign23190_ad_e26572: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign23190_ad_e26572);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1421] != 0.0))) && (!(s.v[1423] != 0.0))) && (!(s.v[1424] != 0.0))) {
            let assign23200_ad_e26622: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign23200_ad_e26622);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1421] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(489), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p869);
        }

        s.v[1425] = if (p.p878 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (s.v[1425] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1426] = if (s.v[1190] > ((-s.v[445]) * p.p878)) { 1.0 } else { 0.0 };

        s.v[1427] = if (p.p881 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1425] != 0.0))) && (s.v[1426] != 0.0)) && (s.v[1427] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[450]), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450])), A::scale(s.ad_value(1190), s.v[450]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1425] != 0.0))) && (s.v[1426] != 0.0)) && (!(s.v[1427] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[450])), p.p881);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1425] != 0.0))) && (s.v[1426] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) && (!(s.v[1425] != 0.0))) && (!(s.v[1426] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p878)), s.v[453]), s.v[447]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1411] != 0.0))) {
            s.store_mul_ad_lhs(1219, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        s.v[1428] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1428] != 0.0)) {
            s.store_scalar(1220, 0.0);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.v[1429] = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (s.v[1429] != 0.0)) {
            s.store_scalar(1193, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.v[1430] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) && (s.v[1430] != 0.0)) {
            s.store_scalar(1196, 0.0);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) && (!(s.v[1430] != 0.0))) {
            s.store_scale_ad(1196, A::add(A::div(A::mul(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195))), A::sub_from_scalar(1.0, s.ad_value(1195))), s.ad_value(1195)), (1.0 - (2.0 * p.p850)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_add(1197, 1195, 1196);
        }

        s.v[1431] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) && (s.v[1431] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(s.ad_value(1194), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) && (!(s.v[1431] != 0.0))) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_scale(1198, 1191, s.v[426]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_scale_ad(1199, A::mul(A::offset(s.ad_value(1185), (-1.0)), s.ad_value(1198)), s.v[387]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1429] != 0.0))) {
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.v[1432] = if (p.p864 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (s.v[1432] != 0.0)) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_scale_ad(1201, A::div(A::scale(s.ad_value(1198), s.v[411]), s.ad_value(1194)), s.v[441]);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_square(1203, 1202);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_sqrt_ad(1204, A::div(A::square(s.ad_value(1203)), A::offset(A::square(s.ad_value(1203)), 1.0)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_sqrt(1205, 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_mul(1206, 1204, 1205);
        }

        s.v[1433] = if (((-p.p850) * s.v[414]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (s.v[1433] != 0.0)) {
            s.store_div_from_scalar_ad(1207, 1.0, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (!(s.v[1433] != 0.0))) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_div_ad(1208, A::mul(s.ad_value(1197), s.ad_value(1207)), A::add(s.ad_value(1197), s.ad_value(1207)));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_sqrt_ad(1209, A::scale(A::div(s.ad_value(1201), s.ad_value(1205)), 0.375));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_sub_ad_lhs(1210, A::scale(A::mul(s.ad_value(1202), s.ad_value(1205)), 2.0), 1204);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_add_ad(1211, A::sub(A::mul(A::scale(s.ad_value(1202), s.v[438]), s.ad_value(1205)), A::scale(s.ad_value(1204), s.v[438])), A::scale(A::mul(s.ad_value(1201), s.ad_value(1206)), 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_mul_ad_lhs(1212, A::offset(s.ad_value(1210), (-1.0)), 1209);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_square(1173, 1212);
        }

        s.v[1434] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (s.v[1434] != 0.0)) {
            s.store_div_from_scalar_ad(1174, 1.0, A::offset(A::scale(s.ad_value(1212), s.v[373]), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (!(s.v[1434] != 0.0))) {
            s.store_div_from_scalar_ad(1174, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(1212), s.v[373])));
        }

        s.v[1435] = if (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (s.v[1435] != 0.0)) {
            s.store_exp_ad(1191, A::sub(s.ad_value(1211), s.ad_value(1173)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (!(s.v[1435] != 0.0))) {
            let assign23700_ad_e27388: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1191, &assign23700_ad_e27388);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_mul_ad_lhs(1175, A::add(A::add(A::scale(s.ad_value(1174), 0.29214664), A::scale(A::square(s.ad_value(1174)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(1174)), s.ad_value(1174)), s.v[375])), 1191);
        }

        s.v[1436] = if (s.v[1212] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (s.v[1436] != 0.0)) {
            s.copy_ad(1213, 1175);
        }

        s.v[1437] = if (s.v[1211] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (!(s.v[1436] != 0.0))) && (s.v[1437] != 0.0)) {
            s.store_exp(1191, 1211);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (!(s.v[1436] != 0.0))) && (!(s.v[1437] != 0.0))) {
            s.store_div_from_scalar_ad(1191, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1211)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) && (!(s.v[1436] != 0.0))) {
            s.store_sub_ad_lhs(1213, A::scale(s.ad_value(1191), 2.0), 1175);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_scale_ad(1214, A::div(A::scale(s.ad_value(1213), s.v[438]), s.ad_value(1209)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1432] != 0.0))) {
            s.store_scale_ad(1200, A::mul(A::mul(s.ad_value(1199), s.ad_value(1214)), s.ad_value(1208)), p.p864);
        }

        s.v[1438] = if (p.p870 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (s.v[1438] != 0.0)) {
            s.store_scalar(1215, 0.0);
        }

        s.v[1439] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1438] != 0.0))) && (s.v[1439] != 0.0)) {
            s.store_sqrt_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1438] != 0.0))) && (!(s.v[1439] != 0.0))) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]), p.p850);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1438] != 0.0))) {
            s.store_scale_ad(1216, A::div(A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[429]), s.ad_value(1191)), s.v[414]);
        }

        s.v[1440] = if (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1438] != 0.0))) && (s.v[1440] != 0.0)) {
            s.store_exp_ad(1191, A::div(A::neg(s.ad_value(444)), s.ad_value(1216)));
        }

        s.v[1441] = if (((-s.v[444]) / s.v[1216]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1438] != 0.0))) && (!(s.v[1440] != 0.0))) && (s.v[1441] != 0.0)) {
            let assign23890_ad_e27715: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(1216))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(1191, 1e-100, assign23890_ad_e27715);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1438] != 0.0))) && (!(s.v[1440] != 0.0))) && (!(s.v[1441] != 0.0))) {
            let assign23900_ad_e27765: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(1216)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(1191, &assign23900_ad_e27765);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1438] != 0.0))) {
            s.store_scale_ad(1215, A::mul(A::mul(A::mul(s.ad_value(489), s.ad_value(1216)), s.ad_value(1216)), s.ad_value(1191)), p.p870);
        }

        s.v[1442] = if (p.p879 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (s.v[1442] != 0.0)) {
            s.store_scalar(1217, 1.0);
        }

        s.v[1443] = if (s.v[1190] > ((-s.v[445]) * p.p879)) { 1.0 } else { 0.0 };

        s.v[1444] = if (p.p882 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1442] != 0.0))) && (s.v[1443] != 0.0)) && (s.v[1444] != 0.0)) {
            s.store_mul_ad(1191, A::mul(A::mul(A::scale(s.ad_value(1190), s.v[451]), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451])), A::scale(s.ad_value(1190), s.v[451]));
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1442] != 0.0))) && (s.v[1443] != 0.0)) && (!(s.v[1444] != 0.0))) {
            s.store_powf_ad(1191, A::abs(A::scale(s.ad_value(1190), s.v[451])), p.p882);
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1442] != 0.0))) && (s.v[1443] != 0.0)) {
            s.store_div_from_scalar_ad(1217, 1.0, A::sub_from_scalar(1.0, s.ad_value(1191)));
        }

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) && (!(s.v[1442] != 0.0))) && (!(s.v[1443] != 0.0))) {
            s.store_offset_ad(1217, A::scale(A::offset(s.ad_value(1190), (s.v[445] * p.p879)), s.v[454]), s.v[448]);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (!(s.v[1428] != 0.0))) {
            s.store_mul_ad_lhs(1220, A::scale(A::add(A::add(A::add(s.ad_value(1192), s.ad_value(1193)), s.ad_value(1200)), s.ad_value(1215)), p.p29), 1217);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_add_ad(479, A::add(A::mul(s.ad_value(647), s.ad_value(1218)), A::mul(s.ad_value(648), s.ad_value(1219))), A::mul(s.ad_value(649), s.ad_value(1220)));
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1189, 0.0);
        }

        if ((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) {
            s.store_scalar(1186, 0.0);
        }

        s.v[1445] = if !(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)) { 1.0 } else { 0.0 };

        s.v[1446] = if (s.v[490] < s.v[655]) { 1.0 } else { 0.0 };

        s.v[1447] = if (((((-0.5) * (s.v[490] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (s.v[1446] != 0.0)) && (s.v[1447] != 0.0)) {
            s.store_exp_ad(1184, A::scale(s.ad_value(490), (s.v[372] * (-0.5))));
        }

        s.v[1448] = if (((-0.5) * (s.v[490] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (s.v[1446] != 0.0)) && (!(s.v[1447] != 0.0))) && (s.v[1448] != 0.0)) {
            let assign24160_ad_e28136: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(1184, &assign24160_ad_e28136);
        }

        if ((((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (s.v[1446] != 0.0)) && (!(s.v[1447] != 0.0))) && (!(s.v[1448] != 0.0))) {
            s.store_scale_ad(1184, A::offset(A::mul(A::offset(A::scale(s.ad_value(490), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(490), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(490), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_div_from_scalar(1185, 1.0, 1184);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (s.v[1446] != 0.0)) {
            s.store_square(1182, 1185);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (!(s.v[1446] != 0.0))) {
            s.store_mul_ad_lhs(1182, A::offset(A::scale(A::sub(s.ad_value(490), s.ad_value(655)), s.v[372]), 1.0), 656);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (!(s.v[1446] != 0.0))) {
            s.store_sqrt(1185, 1182);
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (!(s.v[1446] != 0.0))) {
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.v[1449] = if (s.v[490] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (s.v[1449] != 0.0)) {
            s.store_scale_ad(1186, A::ln(A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) && (!(s.v[1449] != 0.0))) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(1185), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::offset(A::scale(s.ad_value(1185), 3.0), 1.0))))), (s.v[371] * 2.0)), 490);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_sub(1187, 657, 1186);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_scale_ad(1188, A::sub(A::add(s.ad_value(490), s.ad_value(1187)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(490), s.ad_value(1187)), A::sub(s.ad_value(490), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_scale_ad(1189, A::sub(A::add(s.ad_value(490), s.ad_value(660)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(490), s.ad_value(660)), A::sub(s.ad_value(490), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1445] != 0.0)) {
            s.store_scale_ad(1190, A::sub(s.ad_value(490), A::sqrt(A::offset(A::mul(s.ad_value(490), s.ad_value(490)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1450] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1155] != 0.0) && (s.v[1172] != 0.0)) && (s.v[1450] != 0.0)) {
            s.store_scalar(1218, 0.0);
        }

    }
}
