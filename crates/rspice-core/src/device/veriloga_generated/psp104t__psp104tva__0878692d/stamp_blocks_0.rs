#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[981] = (p.p37 >= 0.0);
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

        if s.b[981] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[981]) {
            s.store_scalar(0, (-1.0));
        }

        s.v[756] = (8.8541878176e-12 * 11.8);

        s.v[351] = (273.15 + p.p38);

        s.v[475] = 0.0;

        s.b[982] = (p.p944 > 0.5);
        s.v[982] = if s.b[982] { 1.0 } else { 0.0 };

        if s.b[982] {
            s.store_scalar(475, 1.0);
        }

        if (!s.b[982]) {
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

        s.b[983] = ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0));
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if s.b[983] {
            s.store_scalar(474, 1.0);
        }

        if (!s.b[983]) {
            s.store_scalar(474, 0.0);
        }

        s.b[984] = (s.v[474] == 1.0);
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        if s.b[984] {
            s.store_scalar(458, (if ((p.p844 * p.p883) > 1e-18) { (p.p844 * p.p883) } else { 1e-18 }));
        }

        if s.b[984] {
            s.store_scalar(459, (if ((p.p847 * p.p884) > 0.05) { (p.p847 * p.p884) } else { 0.05 }));
        }

        if s.b[984] {
            s.store_scalar(460, (if ((if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) < 0.95) { (if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[984] {
            s.store_scalar(461, (p.p853 * p.p886));
            s.store_offset(463, 461, s.v[376]);
            s.store_sub_from_scalar(468, 1.0, 460);
            s.store_div_from_scalar(469, 1.0, 468);
        }

        s.b[985] = (p.p44 == 0.0);
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

        if s.b[985] {
            s.store_scalar(506, p.p842);
            s.store_scalar(507, p.p843);
            s.store_scalar(508, p.p844);
            s.store_scalar(509, p.p845);
            s.store_scalar(510, p.p846);
            s.store_scalar(511, p.p847);
            s.store_scalar(512, p.p848);
            s.store_scalar(513, p.p849);
            s.store_scalar(514, p.p850);
            s.store_scalar(515, p.p851);
            s.store_scalar(516, p.p852);
            s.store_scalar(517, p.p853);
            s.store_scalar(518, p.p854);
            s.store_scalar(519, p.p855);
            s.store_scalar(520, p.p856);
            s.store_scalar(523, p.p857);
            s.store_scalar(524, p.p858);
            s.store_scalar(525, p.p859);
            s.store_scalar(521, p.p860);
            s.store_scalar(522, p.p861);
            s.store_scalar(526, p.p862);
            s.store_scalar(527, p.p863);
            s.store_scalar(528, p.p864);
            s.store_scalar(529, p.p865);
            s.store_scalar(530, p.p866);
            s.store_scalar(531, p.p867);
            s.store_scalar(532, p.p868);
            s.store_scalar(533, p.p869);
            s.store_scalar(534, p.p870);
            s.store_scalar(535, p.p871);
            s.store_scalar(536, p.p872);
            s.store_scalar(537, p.p873);
            s.store_scalar(538, p.p874);
            s.store_scalar(539, p.p875);
            s.store_scalar(540, p.p876);
            s.store_scalar(541, p.p877);
            s.store_scalar(542, p.p878);
            s.store_scalar(543, p.p879);
            s.store_scalar(544, p.p880);
            s.store_scalar(545, p.p881);
            s.store_scalar(546, p.p882);
            s.store_scalar(553, p.p945);
            s.store_scalar(554, p.p946);
            s.store_scalar(637, p.p889);
            s.store_scalar(638, p.p890);
            s.store_scalar(639, p.p891);
            s.store_scalar(640, p.p892);
            s.store_scalar(547, p.p883);
            s.store_scalar(548, p.p884);
            s.store_scalar(549, p.p885);
            s.store_scalar(550, p.p886);
            s.store_scalar(551, p.p887);
            s.store_scalar(552, p.p888);
        }

        if (!s.b[985]) {
            s.store_scalar(506, p.p893);
            s.store_scalar(507, p.p894);
            s.store_scalar(508, p.p895);
            s.store_scalar(509, p.p896);
            s.store_scalar(510, p.p897);
            s.store_scalar(511, p.p898);
            s.store_scalar(512, p.p899);
            s.store_scalar(513, p.p900);
            s.store_scalar(514, p.p901);
            s.store_scalar(515, p.p902);
            s.store_scalar(516, p.p903);
            s.store_scalar(517, p.p904);
            s.store_scalar(518, p.p905);
            s.store_scalar(519, p.p906);
            s.store_scalar(520, p.p907);
            s.store_scalar(523, p.p908);
            s.store_scalar(524, p.p909);
            s.store_scalar(525, p.p910);
            s.store_scalar(521, p.p911);
            s.store_scalar(522, p.p912);
            s.store_scalar(526, p.p913);
            s.store_scalar(527, p.p914);
            s.store_scalar(528, p.p915);
            s.store_scalar(529, p.p916);
            s.store_scalar(530, p.p917);
            s.store_scalar(531, p.p918);
            s.store_scalar(532, p.p919);
            s.store_scalar(533, p.p920);
            s.store_scalar(534, p.p921);
            s.store_scalar(535, p.p922);
            s.store_scalar(536, p.p923);
            s.store_scalar(537, p.p924);
            s.store_scalar(538, p.p925);
            s.store_scalar(539, p.p926);
            s.store_scalar(540, p.p927);
            s.store_scalar(541, p.p928);
            s.store_scalar(542, p.p929);
            s.store_scalar(543, p.p930);
        }

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[985]) {
            s.store_scalar(544, p.p931);
            s.store_scalar(545, p.p932);
            s.store_scalar(546, p.p933);
            s.store_scalar(553, p.p947);
            s.store_scalar(554, p.p948);
            s.store_scalar(637, p.p940);
            s.store_scalar(638, p.p941);
            s.store_scalar(639, p.p942);
            s.store_scalar(640, p.p943);
            s.store_scalar(547, p.p934);
            s.store_scalar(548, p.p935);
            s.store_scalar(549, p.p936);
            s.store_scalar(550, p.p937);
            s.store_scalar(551, p.p938);
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

        s.store_scaled_div(592, 521, 507, s.v[756]);

        s.store_scaled_div(593, 522, 508, s.v[756]);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 593);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar(599, 1.0, 511);

        s.store_div_from_scalar_sub_from_scalar_ad(612, 1.0, 1.0, A::pow_from_scalar(s.v[445], s.ad_value(544)));

        s.store_div_from_scalar_sub_from_scalar_ad(613, 1.0, 1.0, A::pow_from_scalar(s.v[445], s.ad_value(545)));

        s.store_div_from_scalar_sub_from_scalar_ad(614, 1.0, 1.0, A::pow_from_scalar(s.v[445], s.ad_value(546)));

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.store_div_from_scalar(617, 1.0, 543);

        s.store_mul_ad_lhs(618, A::mul3_scaled_output(A::square(s.ad_value(612)), A::pow_from_scalar(s.v[445], A::offset(s.ad_value(544), (-1.0))), s.ad_value(544), -1.0), 615);

        s.store_mul_ad_lhs(619, A::mul3_scaled_output(A::square(s.ad_value(613)), A::pow_from_scalar(s.v[445], A::offset(s.ad_value(545), (-1.0))), s.ad_value(545), -1.0), 616);

        s.store_mul_ad_lhs(620, A::mul3_scaled_output(A::square(s.ad_value(614)), A::pow_from_scalar(s.v[445], A::offset(s.ad_value(546), (-1.0))), s.ad_value(546), -1.0), 617);

        s.b[986] = ((((s.v[547] != 1.0) || (s.v[548] != 1.0)) || (s.v[549] != 1.0)) || (s.v[550] != 1.0));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if s.b[986] {
            s.store_scalar(636, 1.0);
        }

        if (!s.b[986]) {
            s.store_scalar(636, 0.0);
        }

        s.b[987] = (s.v[636] == 1.0);
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if s.b[987] {
            s.store_ad_value(621, {
                if ((s.v[508] * s.v[547]) > 1e-18) {
                    A::mul(s.ad_value(508), s.ad_value(547))
                } else {
                    A::constant(1e-18)
                }
            });
        }

        if s.b[987] {
            s.store_ad_value(622, {
                if ((s.v[511] * s.v[548]) > 0.05) {
                    A::mul(s.ad_value(511), s.ad_value(548))
                } else {
                    A::constant(0.05)
                }
            });
        }

        if s.b[987] {
            s.store_ad_value(623, {
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

        if s.b[987] {
            s.store_mul(624, 517, 550);
            s.store_offset(626, 624, s.v[376]);
            s.store_sub_from_scalar(631, 1.0, 623);
            s.store_div_from_scalar(632, 1.0, 631);
        }

        s.v[867] = 0.0;

        s.v[352] = ((ctx_temp + p.p55) + p.p35);

        s.v[353] = (s.v[352] / s.v[351]);

        s.v[354] = (s.v[352] - s.v[351]);

        s.v[355] = ((s.v[352] * 1.3806505e-23) / 1.6021918e-19);

        s.v[356] = (1.0 / s.v[355]);

        s.v[366] = (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0)));

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

        if (!(s.v[442] > 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (!(s.v[443] > 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (!(s.v[444] > 0.0)) {
            s.store_scalar(444, 0.0);
        }

        s.b[1007] = (s.v[474] == 1.0);
        s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };

        if s.b[1007] {
            s.store_offset(462, 461, s.v[377]);
            s.store_scale_ad(464, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(463), s.v[370], s.ad_value(462), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));
            s.store_ad_value(465, A::sub_scaled_inputs(s.ad_value(459), s.v[367], A::ln(s.ad_value(464)), (2.0 * s.v[371])));
            s.store_ad_value(466, A::add_scaled_inputs(s.ad_value(465), 1.0, A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(465)), s.v[372])), s.v[371]));
            s.store_div_from_scalar(467, 1.0, 466);
            s.store_mul_pow_ad_rhs(470, 458, A::mul(s.ad_value(459), s.ad_value(467)), s.ad_value(460));
            s.store_mul3_lhs(471, 470, 466, 469);
            s.store_scale(472, 470, 2.0);
        }

        s.store_offset(558, 515, s.v[377]);

        s.store_offset(559, 516, s.v[377]);

        s.store_offset(560, 517, s.v[377]);

        s.store_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[370], s.ad_value(558), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[370], s.ad_value(559), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(563, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(557), s.v[370], s.ad_value(560), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_mul3_lhs(564, 518, 561, 561);

        s.store_mul3_lhs(565, 519, 562, 562);

        s.store_mul3_lhs(566, 520, 563, 563);

        s.store_ad_value(567, A::sub_scaled_inputs(s.ad_value(509), s.v[367], A::ln(s.ad_value(561)), (2.0 * s.v[371])));

        s.store_ad_value(568, A::sub_scaled_inputs(s.ad_value(510), s.v[367], A::ln(s.ad_value(562)), (2.0 * s.v[371])));

        s.store_ad_value(569, A::sub_scaled_inputs(s.ad_value(511), s.v[367], A::ln(s.ad_value(563)), (2.0 * s.v[371])));

        s.store_ad_value(570, A::add_scaled_inputs(s.ad_value(567), 1.0, A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(567)), s.v[372])), s.v[371]));

        s.store_ad_value(571, A::add_scaled_inputs(s.ad_value(568), 1.0, A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(568)), s.v[372])), s.v[371]));

        s.store_ad_value(572, A::add_scaled_inputs(s.ad_value(569), 1.0, A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(569)), s.v[372])), s.v[371]));

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(575, 1.0, 572);

        s.store_mul_pow_ad_rhs(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512));

        s.store_mul_pow_ad_rhs(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513));

        s.store_mul_pow_ad_rhs(584, 508, A::mul(s.ad_value(511), s.ad_value(575)), s.ad_value(514));

        s.store_mul3_lhs(585, 582, 570, 579);

        s.store_mul3_lhs(586, 583, 571, 580);

        s.store_mul3_lhs(587, 584, 572, 581);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_scale(590, 584, 2.0);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(602, A::scale(s.ad_value(560), 0.5), s.v[371]);

        s.store_scale(603, 600, s.v[372]);

        s.store_scale(604, 601, s.v[372]);

        s.store_scale(605, 602, s.v[372]);

        s.store_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(608, A::mul3_scaled_output(s.ad_value(531), A::square(s.ad_value(602)), s.ad_value(602), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_ad_rhs(609, 535, A::scale_offset(s.ad_value(538), (s.v[366] - s.v[365]), 1.0));

        s.store_mul_ad_rhs(610, 536, A::scale_offset(s.ad_value(539), (s.v[366] - s.v[365]), 1.0));

        s.store_mul_ad_rhs(611, 537, A::scale_offset(s.ad_value(540), (s.v[366] - s.v[365]), 1.0));

        if (!(s.v[609] > 0.0)) {
            s.store_scalar(609, 0.0);
        }

        if (!(s.v[610] > 0.0)) {
            s.store_scalar(610, 0.0);
        }

        if (!(s.v[611] > 0.0)) {
            s.store_scalar(611, 0.0);
        }

        s.b[1008] = (s.v[636] == 1.0);
        s.v[1008] = if s.b[1008] { 1.0 } else { 0.0 };

        if s.b[1008] {
            s.store_offset(625, 624, s.v[377]);
            s.store_scale_ad(627, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(626), s.v[370], s.ad_value(625), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));
            s.store_ad_value(628, A::sub_scaled_inputs(s.ad_value(622), s.v[367], A::ln(s.ad_value(627)), (2.0 * s.v[371])));
            s.store_ad_value(629, A::add_scaled_inputs(s.ad_value(628), 1.0, A::ln_one_plus_exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(628)), s.v[372])), s.v[371]));
            s.store_div_from_scalar(630, 1.0, 629);
            s.store_mul_pow_ad_rhs(633, 621, A::mul(s.ad_value(622), s.ad_value(630)), s.ad_value(623));
            s.store_mul3_lhs(634, 633, 629, 632);
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

        s.b[1009] = (p.p39 > 0.0);
        s.v[1009] = if s.b[1009] { 1.0 } else { 0.0 };

        if s.b[1009] {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1009] {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
            s.store_div_from_scalar(2, 1.0, 1);
        }

    }

    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.v[4] * s.v[2]) > 1e-9) {
            s.store_scale(4, 2, s.v[4]);
        } else {
            s.store_scalar(4, 1e-9);
        }

        s.v[11] = p.p5;

        s.v[12] = p.p6;

        s.v[13] = p.p7;

        s.v[14] = (if (p.p10 < 1.5) { 1.0 } else { 2.0 });

        s.v[308] = (1e-6 / s.v[3]);

        s.store_div_from_scalar(309, 1e-6, 4);

        s.store_offset_scaled(310, 309, ((p.p191) * ((p.p189 * (1.0 + (p.p190 * s.v[308]))))), (p.p189 * (1.0 + (p.p190 * s.v[308]))));

        s.store_offset_scaled(311, 309, ((p.p195) * ((p.p193 * (1.0 + (p.p194 * s.v[308]))))), (p.p193 * (1.0 + (p.p194 * s.v[308]))));

        if (((s.v[3] + s.v[310]) - (2.0 * p.p192)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[3]) + ((-(2.0 * p.p192)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[4] + s.v[311]) - (2.0 * p.p196)) > 1e-9) {
            s.store_offset_add(313, 4, 311, (-(2.0 * p.p196)));
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
            s.store_offset(320, 310, ((((s.v[3]) + ((-(2.0 * p.p192))))) + (p.p197)));
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[4] + s.v[311]) - (2.0 * p.p196)) + p.p198) > 1e-9) {
            s.store_offset_add(321, 4, 311, (((-(2.0 * p.p196))) + (p.p198)));
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
            s.store_offset_add(324, 4, 311, p.p198);
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

        s.b[1010] = param_given[121];
        s.v[1010] = if s.b[1010] { 1.0 } else { 0.0 };

        if s.b[1010] {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.b[1011] = param_given[122];
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if s.b[1011] {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.b[1012] = param_given[123];
        s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };

        if s.b[1012] {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.b[1013] = param_given[124];
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        if s.b[1013] {
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

        s.b[1014] = param_given[137];
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        if s.b[1014] {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.b[1015] = param_given[138];
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if s.b[1015] {
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

        s.b[1016] = (p.p39 > 0.0);
        s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };

        if s.b[1016] {
            s.store_ad_value(40, A::add_scaled_inputs3_offset(A::powf(s.ad_value(314), p.p201), p.p200, s.ad_value(316), p.p202, s.ad_value(318), p.p203, p.p199));
            s.store_ad_value(41, A::add_scaled_inputs3_offset(s.ad_value(314), p.p205, s.ad_value(316), p.p206, s.ad_value(318), p.p207, p.p204));
            s.store_scalar(42, p.p208);
            s.store_scalar(43, p.p209);
            s.store_scalar(44, p.p210);
        }

        if s.b[1016] {
            s.store_scale_ad(331, {
                if ((1.0 + ((p.p212 * s.v[316]) * (((1.0 + (s.v[313] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p212, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }

        if s.b[1016] {
            s.store_scale_ad(332, {
                if ((1.0 + ((p.p215 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p215, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }

        if s.b[1016] {
            s.store_scale_ad(333, {
                if ((1.0 + ((p.p218 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p218, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p217);
        }

        s.b[1017] = (s.v[312] > (2.0 * s.v[333]));
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1017]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_ad_value(336, A::add_scaled_product(A::sqrt(s.ad_value(331)), 1.0, s.ad_value(334), A::ln(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::offset(A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0))), 1.0)), 1.0));
            s.store_square(336, 336);
        }

        s.b[1018] = (s.v[312] >= s.v[333]);
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        if ((s.b[1016] && (!s.b[1017])) && s.b[1018]) {
            s.store_add_ad_rhs(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));
        }

        if ((s.b[1016] && (!s.b[1017])) && (!s.b[1018])) {
            s.store_add_ad_rhs(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));
        }

        if s.b[1016] {
            s.store_mul_ad_rhs(45, 336, A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p219)), 1.0, s.ad_value(315), p.p220));
            s.store_ad_value(46, A::add_scaled_inputs3_offset(A::powf(s.ad_value(314), p.p223), p.p222, s.ad_value(316), p.p224, s.ad_value(318), p.p225, p.p221));
            s.store_scalar(47, p.p226);
            s.store_scalar(48, p.p227);
            s.store_ad_value(49, A::add_scaled_inputs3_offset(A::powf(s.ad_value(314), p.p230), p.p229, s.ad_value(316), p.p231, s.ad_value(318), p.p232, p.p228));
        }

    }

    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1016] {
            s.store_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p234 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p234, 1.0)
                }
            }, p.p233);
        }

        if s.b[1016] {
            s.store_scalar(55, p.p235);
            s.store_scalar(56, p.p236);
            s.store_scalar(57, p.p239);
            s.store_scalar(58, p.p240);
            s.store_ad_value(51, A::mul3(A::scale_offset(A::powf(s.ad_value(314), p.p243), p.p242, p.p241), A::scale_offset(s.ad_value(316), p.p244, 1.0), A::scale_offset(s.ad_value(318), p.p245, 1.0)));
            s.store_scalar(52, p.p247);
            s.store_scalar(53, p.p246);
            s.store_scalar(54, p.p248);
            s.store_scaled_mul_ad(62, A::powf(s.ad_value(314), p.p250), A::scale_offset(s.ad_value(316), p.p251, 1.0), p.p249);
            s.store_scalar(63, p.p253);
            s.store_scalar(64, p.p252);
            s.store_scaled_mul_ad(59, A::powf(s.ad_value(314), p.p255), A::scale_offset(s.ad_value(316), p.p256, 1.0), p.p254);
            s.store_scalar(60, p.p258);
            s.store_scalar(61, p.p257);
            s.store_offset_scaled(337, 316, ((p.p261) * (p.p260)), p.p260);
        }

        if s.b[1016] {
            s.store_scale_ad(338, {
                if ((1.0 + (p.p263 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p263, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p262);
        }

        if s.b[1016] {
            s.store_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp(A::div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0))), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p264 * p.p265), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p265)))));
        }

        if s.b[1016] {
            s.store_ad_value(339, {
                if (s.v[339] > 1e-15) {
                    s.ad_value(339)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if s.b[1016] {
            s.store_ad_value(340, A::add_scaled_product(A::scale_offset(s.ad_value(316), p.p266, 1.0), 1.0, s.ad_value(316), A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p268), 1.0)), p.p267));
            s.store_mul_ad_lhs(65, A::div_scaled_inputs(s.ad_value(313), p.p259, A::mul(s.ad_value(339), s.ad_value(312)), 1.0), 340);
            s.store_ad_value(66, A::add_scaled_inputs3_offset(s.ad_value(314), p.p270, s.ad_value(316), p.p271, s.ad_value(318), p.p272, p.p269));
            s.store_offset_scaled(67, 316, ((p.p274) * (p.p273)), p.p273);
            s.store_scalar(68, p.p275);
            s.store_scalar(69, p.p276);
            s.store_scalar(70, p.p277);
            s.store_ad_value(71, A::mul3(A::scale_offset(A::powf(s.ad_value(314), p.p280), p.p279, p.p278), A::scale_offset(s.ad_value(316), p.p281, 1.0), A::scale_offset(s.ad_value(318), p.p282, 1.0)));
            s.store_scalar(72, p.p283);
            s.store_scalar(73, p.p284);
            s.store_scalar(74, p.p285);
            s.store_ad_value(75, A::mul3_scaled_output(A::scale_offset(s.ad_value(314), p.p287, 1.0), A::scale_offset(s.ad_value(316), p.p288, 1.0), A::scale_offset(s.ad_value(318), p.p289, 1.0), p.p286));
            s.store_scalar(76, p.p290);
            s.store_scalar(77, p.p291);
            s.store_mul_scaled_ad_rhs(78, 316, p.p292, A::scale_offset(s.ad_value(316), p.p293, 1.0));
            s.store_scalar(79, p.p294);
            s.store_scalar(80, p.p295);
            s.store_scalar(81, p.p296);
            s.store_ad_value(82, A::mul3(A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p298, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p299)), p.p297), A::scale_offset(s.ad_value(316), p.p300, 1.0), A::scale_offset(s.ad_value(318), p.p301, 1.0)));
            s.store_ad_value(83, A::add_scaled_inputs3_offset(s.ad_value(314), p.p303, s.ad_value(316), p.p304, s.ad_value(318), p.p305, p.p302));
            s.store_scalar(84, p.p306);
            s.store_scalar(85, p.p307);
            s.store_scalar(86, p.p308);
            s.store_div_from_scalar_offset_scaled_input(87, p.p309, 314, p.p310, 1.0);
            s.store_scaled_mul_ad(88, A::powf(s.ad_value(314), p.p312), A::scale_offset(s.ad_value(316), p.p313, 1.0), p.p311);
            s.store_powf(341, 314, p.p315);
            s.store_ad_value(89, A::div_scaled_product(s.ad_value(341), A::scale_offset(s.ad_value(316), p.p317, 1.0), p.p314, A::offset(A::mul_scaled_lhs(s.ad_value(314), p.p316, s.ad_value(341)), 1.0), 1.0));
            s.store_powf(341, 314, p.p319);
            s.store_ad_value(90, A::div_scaled_product(s.ad_value(341), A::scale_offset(s.ad_value(316), p.p321, 1.0), p.p318, A::offset(A::mul_scaled_lhs(s.ad_value(314), p.p320, s.ad_value(341)), 1.0), 1.0));
            s.store_scalar(91, p.p322);
            s.store_scaled_mul_ad(92, A::scale_offset(s.ad_value(314), p.p324, 1.0), A::scale_offset(s.ad_value(316), p.p325, 1.0), p.p323);
            s.store_scalar(93, p.p326);
            s.store_scalar(94, p.p327);
            s.store_scaled_mul_ad(95, A::scale_offset(s.ad_value(314), p.p329, 1.0), A::scale_offset(s.ad_value(316), p.p330, 1.0), p.p328);
            s.store_scaled_mul_ad(96, A::scale_offset(s.ad_value(314), p.p332, 1.0), A::scale_offset(s.ad_value(316), p.p333, 1.0), p.p331);
            s.store_scalar(97, p.p334);
            s.store_scalar(98, p.p335);
            s.store_div_from_scalar(99, p.p336, 318);
            s.store_div_from_scalar_scaled_input(100, (p.p337 * p.p237), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(101, (p.p338 * p.p238), 316, 1e-6);
            s.store_scalar(102, p.p339);
            s.store_scalar(103, p.p340);
            s.store_scalar(104, p.p341);
            s.store_scalar(105, p.p340);
        }

        s.b[1019] = param_given[342];
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1019]) {
            s.store_scalar(105, p.p342);
        }

        if s.b[1016] {
            s.store_scalar(106, p.p341);
        }

        s.b[1020] = param_given[343];
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1020]) {
            s.store_scalar(106, p.p343);
        }

        if s.b[1016] {
            s.copy_ad(107, 105);
        }

        s.b[1021] = param_given[344];
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1021]) {
            s.store_scalar(107, p.p344);
        }

        if s.b[1016] {
            s.copy_ad(108, 106);
        }

        s.b[1022] = param_given[345];
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1022]) {
            s.store_scalar(108, p.p345);
        }

        if s.b[1016] {
            s.store_scalar(109, p.p346);
            s.store_div_from_scalar_scaled_input(110, (p.p347 * p.p237), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(111, (p.p348 * p.p238), 316, 1e-6);
            s.store_scalar(112, p.p349);
            s.store_scalar(113, p.p350);
            s.store_scalar(114, p.p351);
            s.store_scalar(115, p.p352);
            s.store_scalar(116, p.p353);
            s.store_scalar(117, p.p354);
            s.store_scaled_mul(118, 321, 320, ((8.8541878176e-12 * p.p210) * 1.0 / (p.p209)));
            s.store_scale(125, 321, ((8.8541878176e-12 * p.p210) * (p.p237 * 1.0 / (p.p235))));
            s.store_scale(126, 321, ((8.8541878176e-12 * p.p210) * (p.p238 * 1.0 / (p.p236))));
            s.store_ad_value(119, A::add_scaled_inputs3_offset(A::powf(s.ad_value(314), p.p357), p.p356, s.ad_value(316), p.p358, s.ad_value(318), p.p359, p.p355));
            s.store_ad_value(120, A::add_scaled_inputs3_offset(s.ad_value(314), p.p361, s.ad_value(316), p.p362, s.ad_value(318), p.p363, p.p360));
            s.store_scalar(32, p.p297);
        }

        s.b[1023] = param_given[364];
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1023]) {
            s.store_scalar(32, p.p364);
        }

        if s.b[1016] {
            s.store_scalar(33, p.p298);
        }

        s.b[1024] = param_given[365];
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1024]) {
            s.store_scalar(33, p.p365);
        }

        if s.b[1016] {
            s.store_scalar(34, p.p299);
        }

        s.b[1025] = param_given[366];
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1025]) {
            s.store_scalar(34, p.p366);
        }

        if s.b[1016] {
            s.store_scalar(35, p.p300);
        }

        s.b[1026] = param_given[367];
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1026]) {
            s.store_scalar(35, p.p367);
        }

        if s.b[1016] {
            s.store_scalar(36, p.p301);
        }

        s.b[1027] = param_given[368];
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1027]) {
            s.store_scalar(36, p.p368);
        }

        if s.b[1016] {
            s.store_ad_value(121, A::mul3(A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(318)), 1.0)));
            s.store_scalar(37, p.p309);
        }

        s.b[1028] = param_given[369];
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1028]) {
            s.store_scalar(37, p.p369);
        }

        if s.b[1016] {
            s.store_scalar(38, p.p310);
        }

        s.b[1029] = param_given[370];
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1029]) {
            s.store_scalar(38, p.p370);
        }

        if s.b[1016] {
            s.store_div_ad_rhs(122, 37, A::offset(A::mul(s.ad_value(38), s.ad_value(314)), 1.0));
            s.store_scaled_mul_ad(123, A::powf(s.ad_value(314), p.p372), A::scale_offset(s.ad_value(316), p.p373, 1.0), p.p371);
            s.store_powf(341, 314, p.p375);
            s.store_ad_value(124, A::div_scaled_product(s.ad_value(341), A::scale_offset(s.ad_value(316), p.p377, 1.0), p.p374, A::offset(A::mul_scaled_lhs(s.ad_value(314), p.p376, s.ad_value(341)), 1.0), 1.0));
            s.store_scalar(127, p.p378);
            s.store_scalar(128, p.p379);
            s.store_scalar(129, p.p380);
            s.store_scale(130, 325, p.p381);
            s.store_scale(131, 322, p.p382);
            s.store_scale(132, 322, p.p383);
            s.store_scalar(133, p.p384);
            s.store_scalar(134, p.p385);
            s.store_scalar(135, p.p386);
            s.store_scalar(136, p.p387);
            s.store_scale(137, 326, p.p388);
            s.store_scale(138, 326, p.p389);
        }

    }

    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1016] {
            s.store_sub_from_scalar_ad(998, 1.0, A::div_from_scalar((2.0 * p.p396), s.ad_value(312)));
        }

        if s.b[1016] {
            s.store_ad_value(342, {
                if (s.v[998] > 0.001) {
                    s.ad_value(998)
                } else {
                    A::constant(0.001)
                }
            });
        }

        if s.b[1016] {
            s.store_div_from_scalar_powf_ad(343, 1.0, s.ad_value(342), p.p397);
            s.store_scalar(139, p.p390);
            s.store_mul_ad_lhs(140, A::mul3_scaled_output(s.ad_value(65), s.ad_value(65), s.ad_value(316), p.p391), 316);
            s.store_scaled_mul(141, 343, 318, p.p392);
            s.store_scaled_mul(142, 343, 318, p.p393);
            s.store_scaled_mul(143, 343, 318, p.p394);
            s.store_scalar(144, p.p395);
            s.store_offset_scaled(344, 313, p.p399, (2.0 * p.p398));
            s.store_div_from_scalar(345, 1e-6, 344);
            s.store_mul(346, 314, 345);
            s.store_scalar(145, p.p400);
            s.store_ad_value(146, A::add_scaled_inputs3_offset(s.ad_value(314), p.p402, s.ad_value(316), p.p403, s.ad_value(318), p.p404, p.p401));
            s.store_ad_value(147, A::add_scaled_inputs3_offset(A::powf(s.ad_value(314), p.p407), p.p406, s.ad_value(316), p.p408, s.ad_value(318), p.p409, p.p405));
            s.store_ad_value(148, A::mul3_scaled_output(A::scale_offset(A::powf(s.ad_value(314), p.p412), p.p411, 1.0), A::scale_offset(s.ad_value(316), p.p413, 1.0), A::scale_offset(s.ad_value(318), p.p414, 1.0), p.p410));
            s.store_offset_scaled_ad(149, A::powf(s.ad_value(314), p.p417), p.p416, p.p415);
            s.store_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p418 * p.p419), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p419)))), 1.0);
        }

        if s.b[1016] {
            s.store_ad_value(347, {
                if (s.v[347] > 1e-15) {
                    s.ad_value(347)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if s.b[1016] {
            s.store_mul_ad(150, A::div_scaled_inputs(s.ad_value(344), p.p259, A::mul(s.ad_value(347), s.ad_value(312)), 1.0), A::scale_offset(s.ad_value(316), p.p420, 1.0));
            s.store_ad_value(151, A::add_scaled_inputs3_offset(s.ad_value(314), p.p422, s.ad_value(316), p.p423, s.ad_value(318), p.p424, p.p421));
            s.store_scaled_mul_ad(152, A::powf(s.ad_value(314), p.p426), A::scale_offset(s.ad_value(316), p.p427, 1.0), p.p425);
            s.store_scalar(153, p.p428);
            s.store_scalar(154, p.p429);
            s.store_scaled_mul_ad(155, A::powf(s.ad_value(314), p.p431), A::scale_offset(s.ad_value(316), p.p432, 1.0), p.p430);
            s.store_scalar(156, p.p434);
            s.store_scalar(157, p.p433);
            s.store_scalar(158, p.p435);
            s.store_scale(159, 346, p.p436);
            s.store_scale(160, 346, p.p437);
            s.store_scale(161, 346, p.p438);
            s.store_scalar(162, p.p439);
            s.store_ad_value(348, A::add_scaled_inputs3_offset(s.ad_value(314), p.p832, s.ad_value(316), p.p833, s.ad_value(318), p.p834, p.p831));
            s.store_ad_value(349, A::add_scaled_inputs3_offset(s.ad_value(314), p.p836, s.ad_value(316), p.p837, s.ad_value(318), p.p838, p.p835));
            s.store_ad_value(163, A::add_scaled_inputs3(A::div_scaled_inputs(A::add_scaled_inputs(s.ad_value(329), ((0.3333333333333333 * 1.0 / (s.v[14])) * p.p443), s.ad_value(330), p.p443), 1.0, s.ad_value(328), s.v[14]), 1.0, A::div_from_scalar((p.p441 + p.p442), A::mul(s.ad_value(329), s.ad_value(327))), 1.0, s.ad_value(1), p.p440));
        }

        if s.b[1016] {
            s.store_scalar(164, (if (p.p445 > 0.0) { p.p445 } else { 0.0 }));
        }

        if s.b[1016] {
            s.store_scalar(165, (if (p.p446 > 0.0) { p.p446 } else { 0.0 }));
        }

        s.b[1030] = (p.p44 == 0.0);
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1030]) {
            s.copy_ad(165, 164);
        }

        if s.b[1016] {
            s.store_scaled_mul(166, 1, 164, p.p12);
            s.store_scaled_mul(167, 1, 165, p.p13);
            s.store_scale(168, 1, p.p448);
            s.store_scale(169, 1, p.p447);
            s.store_scale(170, 1, p.p449);
            s.store_scale(171, 1, p.p450);
            s.store_offset_div_ad(350, A::offset(A::div_from_scalar(p.p454, s.ad_value(314)), 1.0), s.ad_value(316), p.p453);
        }

        if s.b[1016] {
            s.store_ad_value(350, {
                if (s.v[350] > 1e-6) {
                    s.ad_value(350)
                } else {
                    A::constant(1e-6)
                }
            });
        }

        if s.b[1016] {
            s.store_offset_div_from_scalar_ad(172, p.p452, s.ad_value(350), p.p451);
            s.store_offset_div_ad(173, A::scaled_offset(A::div_from_scalar(p.p458, s.ad_value(314)), ((1.0) + (p.p457)), p.p456), s.ad_value(316), p.p455);
            s.store_scalar(174, p.p459);
        }

        s.b[1031] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1031]) {
            s.store_ad_value(40, A::add_scaled_inputs3_offset(s.ad_value(314), p.p461, s.ad_value(316), p.p462, s.ad_value(318), p.p463, p.p460));
        }

        s.b[1032] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1032]) {
            s.store_ad_value(41, A::add_scaled_inputs3_offset(s.ad_value(314), p.p465, s.ad_value(316), p.p466, s.ad_value(318), p.p467, p.p464));
        }

        s.b[1033] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);
        s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1033]) {
            s.store_ad_value(45, A::add_scaled_inputs3_offset(s.ad_value(314), p.p469, s.ad_value(316), p.p470, s.ad_value(318), p.p471, p.p468));
        }

        s.b[1034] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1034]) {
            s.store_ad_value(46, A::add_scaled_inputs3_offset(s.ad_value(314), p.p473, s.ad_value(316), p.p474, s.ad_value(318), p.p475, p.p472));
        }

        s.b[1035] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1035]) {
            s.store_ad_value(47, A::add_scaled_inputs3_offset(s.ad_value(314), p.p477, s.ad_value(316), p.p478, s.ad_value(318), p.p479, p.p476));
        }

        s.b[1036] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1036]) {
            s.store_ad_value(49, A::add_scaled_inputs3_offset(s.ad_value(314), p.p481, s.ad_value(316), p.p482, s.ad_value(318), p.p483, p.p480));
        }

        s.b[1037] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1037]) {
            s.store_ad_value(50, A::add_scaled_inputs3_offset(s.ad_value(314), p.p485, s.ad_value(316), p.p486, s.ad_value(318), p.p487, p.p484));
        }

        s.b[1038] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1038]) {
            s.store_ad_value(57, A::add_scaled_inputs3_offset(s.ad_value(314), p.p489, s.ad_value(316), p.p490, s.ad_value(318), p.p491, p.p488));
        }

        s.b[1039] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1039]) {
            s.store_ad_value(58, A::add_scaled_inputs3_offset(s.ad_value(314), p.p493, s.ad_value(316), p.p494, s.ad_value(318), p.p495, p.p492));
        }

        s.b[1040] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1040]) {
            s.store_ad_value(51, A::add_scaled_inputs3_offset(s.ad_value(314), p.p497, s.ad_value(316), p.p498, s.ad_value(318), p.p499, p.p496));
        }

        s.b[1041] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1041]) {
            s.store_ad_value(52, A::add_scaled_inputs3_offset(s.ad_value(314), p.p505, s.ad_value(316), p.p506, s.ad_value(318), p.p507, p.p504));
        }

        s.b[1042] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1042]) {
            s.store_ad_value(53, A::add_scaled_inputs3_offset(s.ad_value(314), p.p501, s.ad_value(316), p.p502, s.ad_value(318), p.p503, p.p500));
        }

        s.b[1043] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1043]) {
            s.store_ad_value(54, A::add_scaled_inputs3_offset(s.ad_value(314), p.p509, s.ad_value(316), p.p510, s.ad_value(318), p.p511, p.p508));
        }

        s.b[1044] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1044]) {
            s.store_mul_ad_rhs(62, 315, A::add_scaled_inputs3_offset(s.ad_value(314), p.p513, s.ad_value(316), p.p514, s.ad_value(318), p.p515, p.p512));
        }

        s.b[1045] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1045]) {
            s.store_ad_value(63, A::add_scaled_inputs3_offset(s.ad_value(314), p.p521, s.ad_value(316), p.p522, s.ad_value(318), p.p523, p.p520));
        }

        s.b[1046] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1046]) {
            s.store_ad_value(64, A::add_scaled_inputs3_offset(s.ad_value(314), p.p517, s.ad_value(316), p.p518, s.ad_value(318), p.p519, p.p516));
        }

        s.b[1047] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1047]) {
            s.store_mul_ad_rhs(59, 315, A::add_scaled_inputs3_offset(s.ad_value(314), p.p525, s.ad_value(316), p.p526, s.ad_value(318), p.p527, p.p524));
        }

        s.b[1048] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1048]) {
            s.store_ad_value(60, A::add_scaled_inputs3_offset(s.ad_value(314), p.p533, s.ad_value(316), p.p534, s.ad_value(318), p.p535, p.p532));
        }

        s.b[1049] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1049]) {
            s.store_ad_value(61, A::add_scaled_inputs3_offset(s.ad_value(314), p.p529, s.ad_value(316), p.p530, s.ad_value(318), p.p531, p.p528));
        }

        s.b[1050] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1050]) {
            s.store_mul_ad(65, A::div(s.ad_value(313), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p537, s.ad_value(316), p.p538, s.ad_value(318), p.p539, p.p536));
        }

        s.b[1051] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1051]) {
            s.store_ad_value(66, A::add_scaled_inputs3_offset(s.ad_value(314), p.p541, s.ad_value(316), p.p542, s.ad_value(318), p.p543, p.p540));
        }

        s.b[1052] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1052]) {
            s.store_ad_value(67, A::add_scaled_inputs3_offset(s.ad_value(314), p.p545, s.ad_value(316), p.p546, s.ad_value(318), p.p547, p.p544));
        }

        s.b[1053] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1053]) {
            s.store_ad_value(69, A::add_scaled_inputs3_offset(s.ad_value(314), p.p549, s.ad_value(316), p.p550, s.ad_value(318), p.p551, p.p548));
        }

        s.b[1054] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1054]) {
            s.store_ad_value(71, A::add_scaled_inputs3_offset(s.ad_value(314), p.p553, s.ad_value(316), p.p554, s.ad_value(318), p.p555, p.p552));
        }

        s.b[1055] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1055]) {
            s.store_ad_value(73, A::add_scaled_inputs3_offset(s.ad_value(314), p.p557, s.ad_value(316), p.p558, s.ad_value(318), p.p559, p.p556));
        }

        s.b[1056] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1056]) {
            s.store_ad_value(75, A::add_scaled_inputs3_offset(s.ad_value(314), p.p561, s.ad_value(316), p.p562, s.ad_value(318), p.p563, p.p560));
        }

        s.b[1057] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1057]) {
            s.store_mul_ad_rhs(78, 316, A::add_scaled_inputs3_offset(s.ad_value(314), p.p565, s.ad_value(316), p.p566, s.ad_value(318), p.p567, p.p564));
        }

        s.b[1058] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1058]) {
            s.store_ad_value(79, A::add_scaled_inputs3_offset(s.ad_value(314), p.p569, s.ad_value(316), p.p570, s.ad_value(318), p.p571, p.p568));
        }

        s.b[1059] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1059]) {
            s.store_ad_value(80, A::add_scaled_inputs3_offset(s.ad_value(314), p.p573, s.ad_value(316), p.p574, s.ad_value(318), p.p575, p.p572));
        }

        s.b[1060] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1060]) {
            s.store_ad_value(81, A::add_scaled_inputs3_offset(s.ad_value(314), p.p577, s.ad_value(316), p.p578, s.ad_value(318), p.p579, p.p576));
        }

        s.b[1061] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1061]) {
            s.store_mul_ad_rhs(82, 314, A::add_scaled_inputs3_offset(s.ad_value(314), p.p581, s.ad_value(316), p.p582, s.ad_value(318), p.p583, p.p580));
        }

        s.b[1062] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1062]) {
            s.store_ad_value(83, A::add_scaled_inputs3_offset(s.ad_value(314), p.p585, s.ad_value(316), p.p586, s.ad_value(318), p.p587, p.p584));
        }

        s.b[1063] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1063]) {
            s.store_ad_value(84, A::add_scaled_inputs3_offset(s.ad_value(314), p.p589, s.ad_value(316), p.p590, s.ad_value(318), p.p591, p.p588));
        }

        s.b[1064] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1064]) {
            s.store_ad_value(85, A::add_scaled_inputs3_offset(s.ad_value(314), p.p593, s.ad_value(316), p.p594, s.ad_value(318), p.p595, p.p592));
        }

        s.b[1065] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1065]) {
            s.store_ad_value(87, A::add_scaled_inputs3_offset(s.ad_value(314), p.p597, s.ad_value(316), p.p598, s.ad_value(318), p.p599, p.p596));
        }

        s.b[1066] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1066]) {
            s.store_mul_ad_rhs(88, 314, A::add_scaled_inputs3_offset(s.ad_value(314), p.p601, s.ad_value(316), p.p602, s.ad_value(318), p.p603, p.p600));
        }

        s.b[1067] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1067]) {
            s.store_ad_value(89, A::add_scaled_inputs3_offset(s.ad_value(314), p.p605, s.ad_value(316), p.p606, s.ad_value(318), p.p607, p.p604));
        }

        s.b[1068] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1068]) {
            s.store_ad_value(90, A::add_scaled_inputs3_offset(s.ad_value(314), p.p609, s.ad_value(316), p.p610, s.ad_value(318), p.p611, p.p608));
        }

        s.b[1069] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1069]) {
            s.store_ad_value(92, A::add_scaled_inputs3_offset(s.ad_value(314), p.p613, s.ad_value(316), p.p614, s.ad_value(318), p.p615, p.p612));
        }

        s.b[1070] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1070]) {
            s.store_ad_value(94, A::add_scaled_inputs3_offset(s.ad_value(314), p.p617, s.ad_value(316), p.p618, s.ad_value(318), p.p619, p.p616));
        }

        s.b[1071] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1071]) {
            s.store_ad_value(95, A::add_scaled_inputs3_offset(s.ad_value(314), p.p621, s.ad_value(316), p.p622, s.ad_value(318), p.p623, p.p620));
        }

        s.b[1072] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1072]) {
            s.store_ad_value(96, A::add_scaled_inputs3_offset(s.ad_value(314), p.p625, s.ad_value(316), p.p626, s.ad_value(318), p.p627, p.p624));
        }

    }

    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1073] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1073]) {
            s.store_mul_ad_rhs(99, 319, A::add_scaled_inputs3_offset(s.ad_value(314), p.p629, s.ad_value(316), p.p630, s.ad_value(318), p.p631, p.p628));
        }

        s.b[1074] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1074]) {
            s.store_mul_ad_rhs(100, 317, A::add_scaled_inputs3_offset(s.ad_value(314), p.p633, s.ad_value(316), p.p634, s.ad_value(318), p.p635, p.p632));
        }

        s.b[1075] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1075]) {
            s.store_mul_ad_rhs(101, 317, A::add_scaled_inputs3_offset(s.ad_value(314), p.p637, s.ad_value(316), p.p638, s.ad_value(318), p.p639, p.p636));
        }

        s.b[1076] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1076]) {
            s.store_ad_value(102, A::add_scaled_inputs3_offset(s.ad_value(314), p.p641, s.ad_value(316), p.p642, s.ad_value(318), p.p643, p.p640));
        }

        s.b[1077] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1077]) {
            s.store_mul_ad_rhs(110, 317, A::add_scaled_inputs3_offset(s.ad_value(314), p.p645, s.ad_value(316), p.p646, s.ad_value(318), p.p647, p.p644));
        }

        s.b[1078] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1078]) {
            s.store_mul_ad_rhs(111, 317, A::add_scaled_inputs3_offset(s.ad_value(314), p.p649, s.ad_value(316), p.p650, s.ad_value(318), p.p651, p.p648));
        }

        s.b[1079] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1079]) {
            s.store_ad_value(114, A::add_scaled_inputs3_offset(s.ad_value(314), p.p653, s.ad_value(316), p.p654, s.ad_value(318), p.p655, p.p652));
        }

        s.b[1080] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1080]) {
            s.store_ad_value(115, A::add_scaled_inputs3_offset(s.ad_value(314), p.p657, s.ad_value(316), p.p658, s.ad_value(318), p.p659, p.p656));
        }

        s.b[1081] = (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1081]) {
            s.store_mul_ad_affine_product_rhs(118, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p661, s.ad_value(316), p.p662, s.ad_value(318), p.p663, p.p660), 1.0 / (1e-6), 0.0);
        }

        s.b[1082] = (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1082]) {
            s.store_ad_value(119, A::add_scaled_inputs3_offset(s.ad_value(314), p.p665, s.ad_value(316), p.p666, s.ad_value(318), p.p667, p.p664));
        }

        s.b[1083] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1083]) {
            s.store_ad_value(120, A::add_scaled_inputs3_offset(s.ad_value(314), p.p669, s.ad_value(316), p.p670, s.ad_value(318), p.p671, p.p668));
        }

        s.b[1084] = (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(28, p.p580);
        }

        s.b[1085] = param_given[672];
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1085]) {
            s.store_scalar(28, p.p672);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(29, p.p581);
        }

        s.b[1086] = param_given[673];
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1086]) {
            s.store_scalar(29, p.p673);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(30, p.p582);
        }

        s.b[1087] = param_given[674];
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1087]) {
            s.store_scalar(30, p.p674);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(31, p.p583);
        }

        s.b[1088] = param_given[675];
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1088]) {
            s.store_scalar(31, p.p675);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_mul_ad_rhs(121, 314, A::add_scaled_product(A::add_scaled_product(A::add_scaled_product(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(314), 1.0), 1.0, s.ad_value(30), s.ad_value(316), 1.0), 1.0, s.ad_value(31), s.ad_value(318), 1.0));
        }

        s.b[1089] = (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(28, p.p596);
        }

        s.b[1090] = param_given[676];
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1090]) {
            s.store_scalar(28, p.p676);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(29, p.p597);
        }

        s.b[1091] = param_given[677];
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1091]) {
            s.store_scalar(29, p.p677);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(30, p.p598);
        }

        s.b[1092] = param_given[678];
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1092]) {
            s.store_scalar(30, p.p678);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(31, p.p599);
        }

        s.b[1093] = param_given[679];
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1093]) {
            s.store_scalar(31, p.p679);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_ad_value(122, A::add_scaled_product(A::add_scaled_product(A::add_scaled_product(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(314), 1.0), 1.0, s.ad_value(30), s.ad_value(316), 1.0), 1.0, s.ad_value(31), s.ad_value(318), 1.0));
        }

        s.b[1094] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1094]) {
            s.store_mul_ad_rhs(123, 314, A::add_scaled_inputs3_offset(s.ad_value(314), p.p681, s.ad_value(316), p.p682, s.ad_value(318), p.p683, p.p680));
        }

        s.b[1095] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1095]) {
            s.store_mul_ad_rhs(124, 314, A::add_scaled_inputs3_offset(s.ad_value(314), p.p685, s.ad_value(316), p.p686, s.ad_value(318), p.p687, p.p684));
        }

        s.b[1096] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1096]) {
            s.store_mul_ad_rhs(125, 322, A::add_scaled_inputs3_offset(s.ad_value(314), p.p689, s.ad_value(316), p.p690, s.ad_value(318), p.p691, p.p688));
        }

        s.b[1097] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1097]) {
            s.store_mul_ad_rhs(126, 322, A::add_scaled_inputs3_offset(s.ad_value(314), p.p693, s.ad_value(316), p.p694, s.ad_value(318), p.p695, p.p692));
        }

        s.b[1098] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1098]) {
            s.store_mul_ad_rhs(130, 325, A::add_scaled_inputs3_offset(s.ad_value(314), p.p697, s.ad_value(316), p.p698, s.ad_value(318), p.p699, p.p696));
        }

        s.b[1099] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1099]) {
            s.store_mul_ad_rhs(131, 322, A::add_scaled_inputs3_offset(s.ad_value(314), p.p701, s.ad_value(316), p.p702, s.ad_value(318), p.p703, p.p700));
        }

        s.b[1100] = (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1100]) {
            s.store_mul_ad_rhs(132, 322, A::add_scaled_inputs3_offset(s.ad_value(314), p.p705, s.ad_value(316), p.p706, s.ad_value(318), p.p707, p.p704));
        }

        s.b[1101] = (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1101]) {
            s.store_mul_ad_rhs(137, 326, A::add_scaled_inputs3_offset(s.ad_value(314), p.p709, s.ad_value(316), p.p710, s.ad_value(318), p.p711, p.p708));
        }

        s.b[1102] = (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1102]) {
            s.store_mul_ad_rhs(138, 326, A::add_scaled_inputs3_offset(s.ad_value(314), p.p713, s.ad_value(316), p.p714, s.ad_value(318), p.p715, p.p712));
        }

        s.b[1103] = (((param_given[716] || param_given[717]) || param_given[718]) || param_given[719]);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1103]) {
            s.store_mul_ad_rhs(140, 315, A::add_scaled_inputs3_offset(s.ad_value(314), p.p717, s.ad_value(316), p.p718, s.ad_value(318), p.p719, p.p716));
        }

        s.b[1104] = (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1104]) {
            s.store_mul_ad_rhs(141, 318, A::add_scaled_inputs3_offset(s.ad_value(314), p.p721, s.ad_value(316), p.p722, s.ad_value(318), p.p723, p.p720));
        }

        s.b[1105] = (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1105]) {
            s.store_mul_ad_rhs(142, 318, A::add_scaled_inputs3_offset(s.ad_value(314), p.p725, s.ad_value(316), p.p726, s.ad_value(318), p.p727, p.p724));
        }

        s.b[1106] = (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1106]) {
            s.store_mul_ad_rhs(143, 318, A::add_scaled_inputs3_offset(s.ad_value(314), p.p729, s.ad_value(316), p.p730, s.ad_value(318), p.p731, p.p728));
        }

        s.b[1107] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1107]) {
            s.store_ad_value(145, A::add_scaled_inputs3_offset(s.ad_value(314), p.p733, s.ad_value(316), p.p734, s.ad_value(318), p.p735, p.p732));
        }

        s.b[1108] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1108]) {
            s.store_ad_value(146, A::add_scaled_inputs3_offset(s.ad_value(314), p.p737, s.ad_value(316), p.p738, s.ad_value(318), p.p739, p.p736));
        }

        s.b[1109] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1109]) {
            s.store_ad_value(147, A::add_scaled_inputs3_offset(s.ad_value(314), p.p741, s.ad_value(316), p.p742, s.ad_value(318), p.p743, p.p740));
        }

        s.b[1110] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1110]) {
            s.store_ad_value(148, A::add_scaled_inputs3_offset(s.ad_value(314), p.p745, s.ad_value(316), p.p746, s.ad_value(318), p.p747, p.p744));
        }

        s.b[1111] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1111]) {
            s.store_ad_value(149, A::add_scaled_inputs3_offset(s.ad_value(314), p.p749, s.ad_value(316), p.p750, s.ad_value(318), p.p751, p.p748));
        }

        s.b[1112] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1112]) {
            s.store_mul_ad(150, A::div(s.ad_value(344), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p753, s.ad_value(316), p.p754, s.ad_value(318), p.p755, p.p752));
        }

        s.b[1113] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1113]) {
            s.store_ad_value(151, A::add_scaled_inputs3_offset(s.ad_value(314), p.p757, s.ad_value(316), p.p758, s.ad_value(318), p.p759, p.p756));
        }

        s.b[1114] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1114]) {
            s.store_mul_ad_rhs(152, 315, A::add_scaled_inputs3_offset(s.ad_value(314), p.p761, s.ad_value(316), p.p762, s.ad_value(318), p.p763, p.p760));
        }

        s.b[1115] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1115]) {
            s.store_ad_value(153, A::add_scaled_inputs3_offset(s.ad_value(314), p.p765, s.ad_value(316), p.p766, s.ad_value(318), p.p767, p.p764));
        }

        s.b[1116] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1116]) {
            s.store_ad_value(154, A::add_scaled_inputs3_offset(s.ad_value(314), p.p769, s.ad_value(316), p.p770, s.ad_value(318), p.p771, p.p768));
        }

        s.b[1117] = (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1117]) {
            s.store_mul_ad_rhs(155, 315, A::add_scaled_inputs3_offset(s.ad_value(314), p.p773, s.ad_value(316), p.p774, s.ad_value(318), p.p775, p.p772));
        }

        s.b[1118] = (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1118]) {
            s.store_ad_value(156, A::add_scaled_inputs3_offset(s.ad_value(314), p.p781, s.ad_value(316), p.p782, s.ad_value(318), p.p783, p.p780));
        }

        s.b[1119] = (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1119]) {
            s.store_ad_value(157, A::add_scaled_inputs3_offset(s.ad_value(314), p.p777, s.ad_value(316), p.p778, s.ad_value(318), p.p779, p.p776));
        }

        s.b[1120] = (((param_given[784] || param_given[785]) || param_given[786]) || param_given[787]);
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1120]) {
            s.store_mul_ad_rhs(159, 346, A::add_scaled_inputs3_offset(s.ad_value(314), p.p785, s.ad_value(316), p.p786, s.ad_value(318), p.p787, p.p784));
        }

        s.b[1121] = (((param_given[788] || param_given[789]) || param_given[790]) || param_given[791]);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1121]) {
            s.store_mul_ad_rhs(160, 346, A::add_scaled_inputs3_offset(s.ad_value(314), p.p789, s.ad_value(316), p.p790, s.ad_value(318), p.p791, p.p788));
        }

        s.b[1122] = (((param_given[792] || param_given[793]) || param_given[794]) || param_given[795]);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1122]) {
            s.store_mul_ad_rhs(161, 346, A::add_scaled_inputs3_offset(s.ad_value(314), p.p793, s.ad_value(316), p.p794, s.ad_value(318), p.p795, p.p792));
        }

        s.b[1123] = (((param_given[796] || param_given[797]) || param_given[798]) || param_given[799]);
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1123]) {
            s.store_mul_ad_rhs(172, 318, A::add_scaled_inputs3_offset(s.ad_value(314), p.p797, s.ad_value(316), p.p798, s.ad_value(318), p.p799, p.p796));
        }

        s.b[1124] = (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1124]) {
            s.store_mul_ad_rhs(173, 319, A::add_scaled_inputs3_offset(s.ad_value(314), p.p801, s.ad_value(316), p.p802, s.ad_value(318), p.p803, p.p800));
        }

        s.b[1125] = (((param_given[804] || param_given[805]) || param_given[806]) || param_given[807]);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1125]) {
            s.store_ad_value(174, A::add_scaled_inputs3_offset(s.ad_value(314), p.p805, s.ad_value(316), p.p806, s.ad_value(318), p.p807, p.p804));
        }

        if s.b[1016] {
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(39, p.p812);
        }

        s.b[1126] = param_given[813];
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1126]) {
            s.store_scalar(39, p.p813);
        }

        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (s.v[1] - 0.5);
            let assign9190_cond_e9118: f64 = if ((s.b[1016] && s.b[1127]) && (s.v[1004] < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1016] && s.b[1127]) {
                s.store_add_ad_rhs(1005, 1005, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));
                s.store_add_ad_rhs(1006, 1006, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));
                s.store_offset(1004, 1004, 1.0);
            }
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_mul(989, 1005, 2);
            s.store_mul(990, 1006, 2);
            s.store_scalar(991, (1.0 / (p.p808 + (0.5 * s.v[3]))));
            s.store_scalar(992, (1.0 / (p.p809 + (0.5 * s.v[3]))));
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_ad_value(1002, {
                if ((s.v[3] + s.v[310]) > 1e-9) {
                    A::offset(s.ad_value(310), s.v[3])
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_ad_value(1003, {
                if (((s.v[4] + s.v[311]) + p.p810) > 1e-9) {
                    A::offset(A::add(s.ad_value(4), s.ad_value(311)), p.p810)
                } else {
                    A::constant(1e-9)
                }
            });
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p818);
            s.store_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p819);
            s.store_ad_value(993, A::add_scaled_product(A::add_scaled_inputs(A::scale_offset(s.ad_value(1000), p.p815, 1.0), 1.0, s.ad_value(1001), p.p816), (1.0 + (p.p814 * (s.v[353] - 1.0))), s.ad_value(1000), s.ad_value(1001), (p.p817 * (1.0 + (p.p814 * (s.v[353] - 1.0))))));
            s.store_div_ad_lhs(994, A::add_scaled_inputs(s.ad_value(989), p.p811, s.ad_value(990), p.p811), 993);
            s.store_div_ad_lhs(995, A::add_scaled_inputs(s.ad_value(991), p.p811, s.ad_value(992), p.p811), 993);
            s.store_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p824);
            s.store_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p825);
        }

    }

    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1016] && s.b[1127]) {
            s.store_ad_value(996, A::add_scaled_product(A::add_scaled_inputs(A::scale_offset(s.ad_value(1000), p.p821, 1.0), 1.0, s.ad_value(1001), p.p822), 1.0, s.ad_value(1000), s.ad_value(1001), p.p823));
            s.store_sub_ad_lhs(998, A::add_scaled_inputs3(s.ad_value(989), 1.0, s.ad_value(990), 1.0, s.ad_value(991), -1.0), 992);
            s.store_div_ad(999, A::offset(s.ad_value(994), 1.0), A::offset(s.ad_value(995), 1.0));
            s.store_mul(65, 65, 999);
            s.store_ad_value(82, A::div_scaled_product3(s.ad_value(82), s.ad_value(999), A::scale_offset(s.ad_value(995), p.p812, 1.0), 1.0, A::scale_offset(s.ad_value(994), p.p812, 1.0), 1.0));
            s.store_ad_value(121, A::div_scaled_product3(s.ad_value(121), s.ad_value(999), A::offset(A::mul(s.ad_value(39), s.ad_value(995)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(994)), 1.0), 1.0));
            s.store_mul(150, 150, 999);
            s.store_scaled_div(999, 998, 996, p.p820);
            s.store_add(40, 40, 999);
            s.store_add(145, 145, 999);
            s.store_ad_value(999, A::div_scaled_inputs(s.ad_value(998), p.p826, A::powf(s.ad_value(996), p.p827), 1.0));
            s.store_add(62, 62, 999);
            s.store_add(155, 155, 999);
        }

        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1128]) && s.b[1129]) {
            s.store_offset(998, 4, s.v[8]);
            s.store_scalar(999, (1.0 / p.p828));
            s.store_div_from_scalar_scaled_input(11, (p.p828 * p.p828), 998, s.v[8]);
            s.store_div_ad_lhs(12, A::add_scaled_product(A::exp_scaled_input(s.ad_value(999), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p828)), A::scale_offset(s.ad_value(998), 0.1, (0.01 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-10.0), s.ad_value(999))), (-1.0)), 4);
            s.store_div_ad_lhs(13, A::add_scaled_product(A::exp_scaled_input(s.ad_value(999), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p828)), A::scale_offset(s.ad_value(998), 0.05, (0.0025 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-20.0), s.ad_value(999))), (-1.0)), 4);
        }

        if (s.b[1016] && s.b[1128]) {
            s.store_ad_value(998, A::add_scaled_inputs3(s.ad_value(11), 1.0, s.ad_value(12), p.p829, s.ad_value(13), p.p830));
            s.store_ad_value(40, A::add_scaled_product(s.ad_value(40), 1.0, s.ad_value(348), s.ad_value(998), 1.0));
            s.store_mul3_affine_rhs(65, 65, 349, 998, 1.0, 1.0);
            s.store_ad_value(145, A::add_scaled_product(s.ad_value(145), 1.0, s.ad_value(348), s.ad_value(998), 1.0));
            s.store_mul3_affine_rhs(150, 150, 349, 998, 1.0, 1.0);
        }

        s.copy_ad(175, 40);

        s.copy_ad(176, 41);

        s.copy_ad(177, 42);

        s.copy_ad(179, 43);

        s.copy_ad(180, 44);

        if (s.v[45] > 1e20) {
            s.store_ad_value(181, {
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

        s.copy_ad(185, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(186, 50);
        } else {
            s.store_scalar(186, 0.0);
        }

        s.copy_ad(190, 55);

        s.copy_ad(191, 56);

        if (s.v[57] > 1e23) {
            s.store_ad_value(192, {
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
            s.store_ad_value(193, {
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
            s.store_ad_value(189, {
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
            s.store_ad_value(188, {
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
            s.store_ad_value(196, {
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
            s.store_ad_value(198, {
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
            s.store_ad_value(215, {
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
            s.store_ad_value(219, {
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

    }

    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
            s.store_ad_value(283, {
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
            s.store_ad_value(288, {
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

        if (s.v[155] > 0.0) {
            s.copy_ad(290, 155);
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[157] > 0.0) {
            s.store_ad_value(292, {
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

        s.b[1130] = (p.p44 == 0.0);
        s.v[1130] = if s.b[1130] { 1.0 } else { 0.0 };

        if s.b[1130] {
            s.copy_ad(191, 190);
            s.copy_ad(193, 192);
            s.copy_ad(246, 245);
            s.copy_ad(248, 247);
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(236, 235);
            s.copy_ad(242, 240);
            s.copy_ad(243, 241);
            s.copy_ad(261, 260);
            s.copy_ad(263, 262);
            s.copy_ad(267, 266);
            s.copy_ad(273, 272);
        }

        s.store_scale(757, 180, 8.8541878176e-12);

        s.store_div(758, 757, 179);

        s.store_square(759, 179);

        s.store_scale(760, 758, 6.241449993689894e18);

        s.store_mul(761, 255, 181);

        if (s.v[761] > 1e20) {
            s.store_ad_value(761, {
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

        s.b[1131] = (p.p51 > 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if s.b[1131] {
            s.store_scale_ad(762, A::powf(s.ad_value(758), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.b[1132] = (s.v[0] == (-1.0));
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1132]) {
            s.store_scale(762, 762, (7.448711 / 5.951993));
        }

        s.store_scale(763, 758, (1e-8 * 1.0 / (s.v[756])));

        s.store_scale(764, 212, 0.5);

        s.v[765] = 0.5;

        s.b[1133] = (s.v[0] == (-1.0));
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if s.b[1133] {
            s.store_scale(764, 212, 0.3333333333333333);
            s.store_scalar(765, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(222)), 1.0), (-1.0));

        s.store_ad_value(766, A::div_scaled_product(A::offset(s.ad_value(997), (-1.0)), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0));

        s.store_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(257)), 1.0), (-1.0));

        s.store_ad_value(767, A::div_scaled_product(A::offset(s.ad_value(997), (-1.0)), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0));

        s.store_div_from_scalar(768, 1.0, 226);

        s.store_div(769, 757, 190);

        s.store_div(770, 757, 191);

        s.store_div_ad_lhs(771, A::sqrt_scaled_input(s.ad_value(192), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 769);

        s.store_div_ad_lhs(772, A::sqrt_scaled_input(s.ad_value(193), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 770);

        s.store_square(773, 771);

        s.store_square(774, 772);

        s.store_offset_div_ad(775, A::ln(A::offset(A::exp_scaled_input(s.ad_value(264), (0.005 * s.v[356])), (-1.0))), s.ad_value(264), (-((((((0.005 * s.v[356])) as f64).exp() - 1.0)) as f64).ln()));

        s.store_add_ad_lhs(776, A::ln_scaled_input(s.ad_value(771), 0.5), 775);

        s.store_add_ad_lhs(777, A::ln_scaled_input(s.ad_value(772), 0.5), 775);

        s.store_div_from_scalar(809, 1.0, 771);

        s.store_offset_scaled(810, 771, 3.1, 8.5);

        s.store_square(778, 810);

        s.store_scale(811, 810, 0.5);

        s.b[1134] = (s.v[809] < 0.06);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if s.b[1134] {
            s.store_scale(779, 809, 64.0);
        }

        s.b[1135] = (s.v[809] <= 0.45);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if ((!s.b[1134]) && s.b[1135]) {
            s.store_offset_scaled(779, 809, 22.0, 3.0);
        }

        s.b[1136] = (s.v[809] <= 1.6);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {
            s.store_offset_scaled(779, 809, (-7.2), 15.5);
        }

        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {
            s.copy_ad(779, 771);
        }

        s.store_ad_value(780, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(811), 1.0, s.ad_value(773), 0.5), 1.0, s.ad_value(771), A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), (-1.0)));

        s.store_div_from_scalar(809, 1.0, 772);

        s.store_offset_scaled(810, 772, 3.1, 8.5);

        s.store_square(781, 810);

        s.store_scale(811, 810, 0.5);

        s.b[1137] = (s.v[809] < 0.06);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if s.b[1137] {
            s.store_scale(782, 809, 64.0);
        }

        s.b[1138] = (s.v[809] <= 0.45);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if ((!s.b[1137]) && s.b[1138]) {
            s.store_offset_scaled(782, 809, 22.0, 3.0);
        }

        s.b[1139] = (s.v[809] <= 1.6);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {
            s.store_offset_scaled(782, 809, (-7.2), 15.5);
        }

        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {
            s.copy_ad(782, 772);
        }

        s.store_ad_value(783, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(811), 1.0, s.ad_value(774), 0.5), 1.0, s.ad_value(772), A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), (-1.0)));

        s.store_div_from_scalar(784, 1.0, 244);

        s.store_scaled_sqrt_scaled_input(785, 244, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(786, 785, 179);

        s.store_mul(787, 785, 190);

        s.store_mul(788, 785, 191);

        s.v[789] = 0.0;

        s.b[1140] = (s.v[239] < 0.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_scaled_div(789, 238, 239, (-0.495));
        }

        s.v[790] = 0.0;

        s.b[1141] = (s.v[241] < 0.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_scaled_div(790, 240, 241, (-0.495));
        }

        s.b[1142] = (s.v[243] < 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_scaled_div(791, 242, 243, (-0.495));
        }

        s.store_pow_from_scalar_ad(792, s.v[353], s.ad_value(237));

        s.store_mul(234, 234, 792);

        s.store_mul(235, 235, 792);

        s.store_mul(236, 236, 792);

        s.store_ad_value(793, A::div_scaled_inputs(s.ad_value(245), 4e-18, A::square(s.ad_value(190)), 1.0));

        s.store_ad_value(794, A::div_scaled_inputs(s.ad_value(246), 4e-18, A::square(s.ad_value(191)), 1.0));

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

    }

    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scaled_mul(796, 712, 191, 500000000.0);

        s.v[797] = 0.0;

        s.b[1143] = (s.v[270] > 1e-10);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_div_from_scalar(797, 0.75, 270);
        }

        s.store_square(798, 271);

        s.store_mul_ad_rhs(716, 305, A::pow_from_scalar(s.v[353], s.ad_value(307)));

        s.store_scale(799, 275, (9.1093826e-31 * 1000000000.0));

        s.b[1144] = (s.v[298] > 0.0);
        s.v[1144] = if s.b[1144] { 1.0 } else { 0.0 };

        if s.b[1144] {
            s.store_div_from_scalar(800, 1.0, 298);
        }

        if (!s.b[1144]) {
            s.store_scalar(800, 0.0);
        }

        s.b[1145] = (s.v[299] > 0.0);
        s.v[1145] = if s.b[1145] { 1.0 } else { 0.0 };

        if s.b[1145] {
            s.store_div_from_scalar(801, 1.0, 299);
        }

        if (!s.b[1145]) {
            s.store_scalar(801, 0.0);
        }

        s.b[1146] = (s.v[300] > 0.0);
        s.v[1146] = if s.b[1146] { 1.0 } else { 0.0 };

        if s.b[1146] {
            s.store_div_from_scalar(802, 1.0, 300);
        }

        if (!s.b[1146]) {
            s.store_scalar(802, 0.0);
        }

        s.b[1147] = (s.v[301] > 0.0);
        s.v[1147] = if s.b[1147] { 1.0 } else { 0.0 };

        if s.b[1147] {
            s.store_div_from_scalar(803, 1.0, 301);
        }

        if (!s.b[1147]) {
            s.store_scalar(803, 0.0);
        }

        s.b[1148] = (s.v[302] > 0.0);
        s.v[1148] = if s.b[1148] { 1.0 } else { 0.0 };

        if s.b[1148] {
            s.store_div_from_scalar(804, 1.0, 302);
        }

        if (!s.b[1148]) {
            s.store_scalar(804, 0.0);
        }

        s.b[1149] = (s.v[303] > 0.0);
        s.v[1149] = if s.b[1149] { 1.0 } else { 0.0 };

        if s.b[1149] {
            s.store_div_from_scalar(805, 1.0, 303);
        }

        if (!s.b[1149]) {
            s.store_scalar(805, 0.0);
        }

        s.b[1150] = (s.v[304] > 0.0);
        s.v[1150] = if s.b[1150] { 1.0 } else { 0.0 };

        if s.b[1150] {
            s.store_div_from_scalar(806, 1.0, 304);
        }

        if (!s.b[1150]) {
            s.store_scalar(806, 0.0);
        }

        s.store_scale(20, 2, s.v[647]);

        s.store_scale(21, 2, s.v[648]);

        s.store_scale(22, 2, s.v[649]);

        s.store_scale(23, 2, s.v[674]);

        s.store_scale(24, 2, s.v[675]);

        s.store_scale(25, 2, s.v[676]);

        s.v[26] = 0.0;

        s.b[1151] = (p.p43 == 3.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if s.b[1151] {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 313);

        s.b[1152] = (p.p39 == 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.b[1153] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_scale(20, 2, s.v[650]);
            s.store_ad_value(21, A::add_scaled_product(s.ad_value(2), s.v[651], s.ad_value(26), s.ad_value(27), (-1.0)));
            s.copy_ad(22, 27);
            s.store_scale(23, 2, s.v[677]);
            s.store_ad_value(24, A::add_scaled_product(s.ad_value(2), s.v[678], s.ad_value(26), s.ad_value(27), (-1.0)));
            s.copy_ad(25, 27);
        }

        s.b[1154] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            s.store_ad_value(647, {
                if (s.v[20] > 0.0) {
                    s.ad_value(20)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1154] {
            s.store_ad_value(648, {
                if (s.v[21] > 0.0) {
                    s.ad_value(21)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1154] {
            s.store_ad_value(649, {
                if (s.v[22] > 0.0) {
                    s.ad_value(22)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1154] {
            s.store_ad_value(674, {
                if (s.v[23] > 0.0) {
                    s.ad_value(23)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1154] {
            s.store_ad_value(675, {
                if (s.v[24] > 0.0) {
                    s.ad_value(24)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[1154] {
            s.store_ad_value(676, {
                if (s.v[25] > 0.0) {
                    s.ad_value(25)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!s.b[1154]) {
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(649, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
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

        s.b[1155] = (p.p43 > 0.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        s.b[1156] = ((s.v[388] * s.v[647]) > 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1156]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1156])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1157] = ((s.v[389] * s.v[648]) > 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1157]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1157])) {
            s.store_scalar(456, 100000000.0);
        }

        s.b[1158] = ((s.v[390] * s.v[649]) > 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1158]) {
            s.store_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(649), s.v[390])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1158])) {
            s.store_scalar(457, 100000000.0);
        }

        if s.b[1155] {
            s.store_min3(655, 455, 456, 457);
        }

        s.b[1159] = ((((s.v[655] * s.v[372])) as f64).abs() < 230.25850929940458);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1159]) {
            s.store_exp_scaled_input(656, 655, s.v[372]);
        }

        s.b[1160] = ((s.v[655] * s.v[372]) < 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if ((s.b[1155] && (!s.b[1159])) && s.b[1160]) {
            s.store_div_from_scalar_offset_ad(656, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(655), s.v[372]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(655), s.v[372]), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[1155] && (!s.b[1159])) && (!s.b[1160])) {
            s.store_scaled_offset_ad(656, A::mul(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::offset(A::mul_scaled_output(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(655), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if s.b[1155] {
            s.store_scalar(397, s.v[394]);
            s.store_scalar(398, s.v[395]);
            s.store_scalar(399, s.v[396]);
            s.store_scalar(400, p.p848);
            s.store_scalar(401, p.p849);
            s.store_scalar(402, p.p850);
            s.store_scalar(403, p.p845);
            s.store_scalar(404, p.p846);
            s.store_scalar(405, p.p847);
        }

        s.b[1161] = (s.v[647] == 0.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1161]) {
            s.store_scalar(397, (s.v[395] + s.v[396]));
            s.store_scalar(400, (0.9 * (p.p849).min(p.p850)));
            s.store_scalar(403, (p.p846 + p.p847));
        }

        s.b[1162] = (s.v[648] == 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1162]) {
            s.store_scalar(398, (s.v[394] + s.v[396]));
            s.store_scalar(401, (0.9 * (p.p848).min(p.p850)));
            s.store_scalar(404, (p.p845 + p.p847));
        }

        s.b[1163] = (s.v[649] == 0.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1163]) {
            s.store_scalar(399, (s.v[394] + s.v[395]));
            s.store_scalar(402, (0.9 * (p.p848).min(p.p849)));
            s.store_scalar(405, (p.p845 + p.p846));
        }

        if s.b[1155] {
            s.store_min3(657, 397, 398, 399);
            s.store_scale(658, 657, 0.1);
            s.store_max3(378, 400, 401, 402);
            s.store_mul_sub_from_scalar_ad_rhs(659, 657, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))));
            s.store_offset_min_ad(660, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));
        }

        s.b[1164] = ((s.v[564] * s.v[674]) > 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1155] && s.b[1164]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1164])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1165] = ((s.v[565] * s.v[675]) > 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1165]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1165])) {
            s.store_scalar(456, 100000000.0);
        }

        s.b[1166] = ((s.v[566] * s.v[676]) > 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1166]) {
            s.store_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(566), s.ad_value(676))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1166])) {
            s.store_scalar(457, 100000000.0);
        }

        if s.b[1155] {
            s.store_min3(682, 455, 456, 457);
        }

        s.b[1167] = ((((s.v[682] * s.v[372])) as f64).abs() < 230.25850929940458);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1167]) {
            s.store_exp_scaled_input(683, 682, s.v[372]);
        }

        s.b[1168] = ((s.v[682] * s.v[372]) < 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if ((s.b[1155] && (!s.b[1167])) && s.b[1168]) {
            s.store_div_from_scalar_offset_ad(683, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(682), s.v[372]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(682), s.v[372]), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((s.b[1155] && (!s.b[1167])) && (!s.b[1168])) {
            s.store_scaled_offset_ad(683, A::mul(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::offset(A::mul_scaled_output(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(682), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if s.b[1155] {
            s.copy_ad(397, 570);
            s.copy_ad(398, 571);
            s.copy_ad(399, 572);
            s.copy_ad(400, 512);
            s.copy_ad(401, 513);
            s.copy_ad(402, 514);
            s.copy_ad(403, 509);
            s.copy_ad(404, 510);
            s.copy_ad(405, 511);
        }

        s.b[1169] = (s.v[674] == 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1169]) {
            s.store_add(397, 571, 572);
            s.store_scale_ad(400, A::min(s.ad_value(513), s.ad_value(514)), 0.9);
            s.store_add(403, 510, 511);
        }

        s.b[1170] = (s.v[675] == 0.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1170]) {
            s.store_add(398, 570, 572);
            s.store_scale_ad(401, A::min(s.ad_value(512), s.ad_value(514)), 0.9);
            s.store_add(404, 509, 511);
        }

        s.b[1171] = (s.v[676] == 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1171]) {
            s.store_add(399, 570, 571);
            s.store_scale_ad(402, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
            s.store_add(405, 509, 510);
        }

        if s.b[1155] {
            s.store_min3(684, 397, 398, 399);
            s.store_scale(685, 684, 0.1);
            s.store_max3(378, 400, 401, 402);
            s.store_mul_sub_from_scalar_ad_rhs(686, 684, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))));
            s.store_offset_min_ad(687, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));
        }

        s.b[1172] = (s.v[475] == 1.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1172]) {
            s.store_scalar(1173, 0.0);
            s.store_scalar(1174, 0.0);
            s.store_scalar(1175, 0.0);
            s.store_scalar(1182, 0.0);
            s.store_scalar(1184, 0.0);
            s.store_scalar(1185, 0.0);
            s.store_scalar(1186, 0.0);
            s.store_scalar(1187, 0.0);
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
            s.store_scalar(499, 0.4);
            s.store_scalar(500, 0.65);
            s.store_scalar(501, 0.8);
            s.store_scale(486, 499, (-p.p945));
            s.store_scale(487, 500, (-p.p945));
            s.store_scale(488, 501, (-p.p945));
            s.store_scalar(489, 0.1);
            s.store_scalar(490, 0.2);
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1221] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        s.b[1222] = (s.v[486] < s.v[655]);
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        s.b[1223] = (((((-0.5) * (s.v[486] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) && s.b[1223]) {
            s.store_exp_scaled_input(1184, 486, (s.v[372] * (-0.5)));
        }

        s.b[1224] = (((-0.5) * (s.v[486] * s.v[372])) < 0.0);
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) && (!s.b[1223])) && s.b[1224]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) && (!s.b[1223])) && (!s.b[1224])) {
            s.store_scaled_offset_ad(1184, A::mul(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1222]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && (!s.b[1222])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(486), s.v[372], s.ad_value(655), s.v[372]), 1.0, 656);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1221]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1225] = (s.v[486] > 0.0);
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && s.b[1225]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1221]) && (!s.b[1225])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 486);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1221]) {
            s.store_sub(1187, 657, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(486), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(1187)), A::sub(s.ad_value(486), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(486), 0.5, s.ad_value(660), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(660)), A::sub(s.ad_value(486), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 486, A::sqrt(A::offset(A::mul(s.ad_value(486), s.ad_value(486)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1226] = (s.v[647] == 0.0);
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1226]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1226])) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.b[1227] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1227]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1228] = (p.p848 == 0.5);
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && s.b[1228]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && (!s.b[1228])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1229] = (p.p848 == 0.5);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && s.b[1229]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) && (!s.b[1229])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1227])) {
            s.store_scale(1198, 1191, s.v[424]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[385]);
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.b[1230] = (p.p862 == 0.0);
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1230]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[409] * s.v[439]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1231] = (((-p.p848) * s.v[412]) == (-1.0));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1231]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1231])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[436]), s.ad_value(1202), s.ad_value(1205), s.v[436]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1232] = (s.v[1212] > 0.0);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1232]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1232])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1233] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1233]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1233])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1234] = (s.v[1212] > 0.0);
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && s.b[1234]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1235] = (s.v[1211] > (-230.25850929940458));
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1234])) && s.b[1235]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1234])) && (!s.b[1235])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) && (!s.b[1234])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1230])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[436] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);
        }

        s.b[1236] = (p.p868 == 0.0);
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1236]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1237] = (p.p848 == 0.5);
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && s.b[1237]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && (!s.b[1237])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p845, s.ad_value(1189)), (s.v[427] * s.v[412]), s.ad_value(1191), 1.0));
        }

        s.b[1238] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && s.b[1238]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1239] = (((-s.v[442]) / s.v[1216]) < 0.0);
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && (!s.b[1238])) && s.b[1239]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) && (!s.b[1238])) && (!s.b[1239])) {
            let assign15300_ad_e13688: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign15300_ad_e13688, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1236])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868);
        }

        s.b[1240] = (p.p877 > 1000.0);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1226])) && s.b[1240]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1241] = (s.v[1190] > ((-s.v[445]) * p.p877));
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        s.b[1242] = (p.p880 == 4.0);
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && s.b[1241]) && s.b[1242]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && s.b[1241]) && (!s.b[1242])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && s.b[1241]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1226])) && (!s.b[1240])) && (!s.b[1241])) {
            s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1226])) {
            s.store_mul_scale_ad_lhs(1218, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        s.b[1243] = (s.v[648] == 0.0);
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1243]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1243])) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.b[1244] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1244]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1245] = (p.p849 == 0.5);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && s.b[1245]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && (!s.b[1245])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1246] = (p.p849 == 0.5);
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && s.b[1246]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) && (!s.b[1246])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1244])) {
            s.store_scale(1198, 1191, s.v[425]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[386]);
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.b[1247] = (p.p863 == 0.0);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1247]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[410] * s.v[440]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1248] = (((-p.p849) * s.v[413]) == (-1.0));
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1248]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1248])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[437]), s.ad_value(1202), s.ad_value(1205), s.v[437]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1249] = (s.v[1212] > 0.0);
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1249]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1249])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1250] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1250]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1250])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1251] = (s.v[1212] > 0.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && s.b[1251]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1252] = (s.v[1211] > (-230.25850929940458));
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1251])) && s.b[1252]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1251])) && (!s.b[1252])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) && (!s.b[1251])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1247])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[437] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);
        }

        s.b[1253] = (p.p869 == 0.0);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1253]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1254] = (p.p849 == 0.5);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && s.b[1254]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && (!s.b[1254])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p846, s.ad_value(1189)), (s.v[428] * s.v[413]), s.ad_value(1191), 1.0));
        }

        s.b[1255] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && s.b[1255]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1256] = (((-s.v[443]) / s.v[1216]) < 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && (!s.b[1255])) && s.b[1256]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) && (!s.b[1255])) && (!s.b[1256])) {
            let assign16000_ad_e14831: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign16000_ad_e14831, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1253])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869);
        }

        s.b[1257] = (p.p878 > 1000.0);
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1243])) && s.b[1257]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1258] = (s.v[1190] > ((-s.v[445]) * p.p878));
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        s.b[1259] = (p.p881 == 4.0);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && s.b[1258]) && (!s.b[1259])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && s.b[1258]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1243])) && (!s.b[1257])) && (!s.b[1258])) {
            s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1243])) {
            s.store_mul_scale_ad_lhs(1219, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        s.b[1260] = (s.v[649] == 0.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1260]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1260])) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.b[1261] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1261]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1262] = (p.p850 == 0.5);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && s.b[1262]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1263] = (p.p850 == 0.5);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && s.b[1263]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) && (!s.b[1263])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1261])) {
            s.store_scale(1198, 1191, s.v[426]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[387]);
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.b[1264] = (p.p864 == 0.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1264]) {
            s.store_scalar(1200, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[411] * s.v[441]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1265] = (((-p.p850) * s.v[414]) == (-1.0));
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1265]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1265])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[438]), s.ad_value(1202), s.ad_value(1205), s.v[438]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1266] = (s.v[1212] > 0.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1266]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1266])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1267] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1267]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1267])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1268] = (s.v[1212] > 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && s.b[1268]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1269] = (s.v[1211] > (-230.25850929940458));
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1268])) && s.b[1269]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1268])) && (!s.b[1269])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) && (!s.b[1268])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1264])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[438] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);
        }

        s.b[1270] = (p.p870 == 0.0);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1270]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1271] = (p.p850 == 0.5);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && s.b[1271]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && (!s.b[1271])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p847, s.ad_value(1189)), (s.v[429] * s.v[414]), s.ad_value(1191), 1.0));
        }

        s.b[1272] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && s.b[1272]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1273] = (((-s.v[444]) / s.v[1216]) < 0.0);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && (!s.b[1272])) && s.b[1273]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) && (!s.b[1272])) && (!s.b[1273])) {
            let assign16700_ad_e15974: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign16700_ad_e15974, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1270])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870);
        }

        s.b[1274] = (p.p879 > 1000.0);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1260])) && s.b[1274]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1275] = (s.v[1190] > ((-s.v[445]) * p.p879));
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        s.b[1276] = (p.p882 == 4.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && s.b[1275]) && s.b[1276]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && s.b[1275]) && (!s.b[1276])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && s.b[1275]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1260])) && (!s.b[1274])) && (!s.b[1275])) {
            s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1260])) {
            s.store_mul_scale_ad_lhs(1220, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(476, A::add_scaled_product(A::add_scaled_products(s.ad_value(647), s.ad_value(1218), 1.0, s.ad_value(648), s.ad_value(1219), 1.0), 1.0, s.ad_value(649), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1277] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        s.b[1278] = (s.v[487] < s.v[655]);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        s.b[1279] = (((((-0.5) * (s.v[487] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) && s.b[1279]) {
            s.store_exp_scaled_input(1184, 487, (s.v[372] * (-0.5)));
        }

        s.b[1280] = (((-0.5) * (s.v[487] * s.v[372])) < 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) && (!s.b[1279])) && s.b[1280]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) && (!s.b[1279])) && (!s.b[1280])) {
            s.store_scaled_offset_ad(1184, A::mul(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1278]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && (!s.b[1278])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(487), s.v[372], s.ad_value(655), s.v[372]), 1.0, 656);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1277]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1281] = (s.v[487] > 0.0);
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && s.b[1281]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1277]) && (!s.b[1281])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 487);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1277]) {
            s.store_sub(1187, 657, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(487), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(1187)), A::sub(s.ad_value(487), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(487), 0.5, s.ad_value(660), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(660)), A::sub(s.ad_value(487), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 487, A::sqrt(A::offset(A::mul(s.ad_value(487), s.ad_value(487)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1282] = (s.v[647] == 0.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1282]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1282])) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.b[1283] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1283]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1284] = (p.p848 == 0.5);
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && s.b[1284]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && (!s.b[1284])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1285] = (p.p848 == 0.5);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && s.b[1285]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) && (!s.b[1285])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1283])) {
            s.store_scale(1198, 1191, s.v[424]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[385]);
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.b[1286] = (p.p862 == 0.0);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1286]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[409] * s.v[439]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1287] = (((-p.p848) * s.v[412]) == (-1.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1287]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[436]), s.ad_value(1202), s.ad_value(1205), s.v[436]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1288] = (s.v[1212] > 0.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1288]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1288])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1289] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1289]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1289])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1290] = (s.v[1212] > 0.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && s.b[1290]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1291] = (s.v[1211] > (-230.25850929940458));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1290])) && s.b[1291]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1290])) && (!s.b[1291])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) && (!s.b[1290])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1286])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[436] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);
        }

        s.b[1292] = (p.p868 == 0.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1292]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1293] = (p.p848 == 0.5);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && s.b[1293]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && (!s.b[1293])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p845, s.ad_value(1189)), (s.v[427] * s.v[412]), s.ad_value(1191), 1.0));
        }

        s.b[1294] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && s.b[1294]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1295] = (((-s.v[442]) / s.v[1216]) < 0.0);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && (!s.b[1294])) && s.b[1295]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) && (!s.b[1294])) && (!s.b[1295])) {
            let assign17700_ad_e17618: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign17700_ad_e17618, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1292])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868);
        }

    }

    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1296] = (p.p877 > 1000.0);
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1282])) && s.b[1296]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1297] = (s.v[1190] > ((-s.v[445]) * p.p877));
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        s.b[1298] = (p.p880 == 4.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && s.b[1297]) && s.b[1298]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && s.b[1297]) && (!s.b[1298])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && s.b[1297]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1282])) && (!s.b[1296])) && (!s.b[1297])) {
            s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1282])) {
            s.store_mul_scale_ad_lhs(1218, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        s.b[1299] = (s.v[648] == 0.0);
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1299]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1299])) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.b[1300] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1300]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1301] = (p.p849 == 0.5);
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && s.b[1301]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && (!s.b[1301])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1302] = (p.p849 == 0.5);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && s.b[1302]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) && (!s.b[1302])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1300])) {
            s.store_scale(1198, 1191, s.v[425]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[386]);
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.b[1303] = (p.p863 == 0.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1303]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[410] * s.v[440]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1304] = (((-p.p849) * s.v[413]) == (-1.0));
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1304]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1304])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[437]), s.ad_value(1202), s.ad_value(1205), s.v[437]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1305] = (s.v[1212] > 0.0);
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1305]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1305])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1306] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1306]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1306])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1307] = (s.v[1212] > 0.0);
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && s.b[1307]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1308] = (s.v[1211] > (-230.25850929940458));
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1307])) && s.b[1308]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1308])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) && (!s.b[1307])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1303])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[437] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);
        }

        s.b[1309] = (p.p869 == 0.0);
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1309]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1310] = (p.p849 == 0.5);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && s.b[1310]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && (!s.b[1310])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p846, s.ad_value(1189)), (s.v[428] * s.v[413]), s.ad_value(1191), 1.0));
        }

        s.b[1311] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && s.b[1311]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1312] = (((-s.v[443]) / s.v[1216]) < 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && (!s.b[1311])) && s.b[1312]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) && (!s.b[1311])) && (!s.b[1312])) {
            let assign18400_ad_e18761: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign18400_ad_e18761, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1309])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869);
        }

        s.b[1313] = (p.p878 > 1000.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1299])) && s.b[1313]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1314] = (s.v[1190] > ((-s.v[445]) * p.p878));
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        s.b[1315] = (p.p881 == 4.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && s.b[1314]) && s.b[1315]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && s.b[1314]) && (!s.b[1315])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && s.b[1314]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1299])) && (!s.b[1313])) && (!s.b[1314])) {
            s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1299])) {
            s.store_mul_scale_ad_lhs(1219, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        s.b[1316] = (s.v[649] == 0.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1316]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1316])) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.b[1317] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1318] = (p.p850 == 0.5);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && s.b[1318]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && (!s.b[1318])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1319] = (p.p850 == 0.5);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && s.b[1319]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && (!s.b[1319])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) {
            s.store_scale(1198, 1191, s.v[426]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[387]);
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.b[1320] = (p.p864 == 0.0);
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1320]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[411] * s.v[441]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1321] = (((-p.p850) * s.v[414]) == (-1.0));
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1321]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1321])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[438]), s.ad_value(1202), s.ad_value(1205), s.v[438]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1322] = (s.v[1212] > 0.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1322]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1322])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1323] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1323]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1323])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1324] = (s.v[1212] > 0.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1324]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1325] = (s.v[1211] > (-230.25850929940458));
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1324])) && s.b[1325]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1325])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1324])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[438] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);
        }

        s.b[1326] = (p.p870 == 0.0);
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1326]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1327] = (p.p850 == 0.5);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && s.b[1327]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && (!s.b[1327])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p847, s.ad_value(1189)), (s.v[429] * s.v[414]), s.ad_value(1191), 1.0));
        }

        s.b[1328] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && s.b[1328]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1329] = (((-s.v[444]) / s.v[1216]) < 0.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && (!s.b[1328])) && s.b[1329]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && (!s.b[1328])) && (!s.b[1329])) {
            let assign19100_ad_e19904: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign19100_ad_e19904, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870);
        }

        s.b[1330] = (p.p879 > 1000.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1330]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1331] = (s.v[1190] > ((-s.v[445]) * p.p879));
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        s.b[1332] = (p.p882 == 4.0);
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && s.b[1331]) && s.b[1332]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && s.b[1331]) && (!s.b[1332])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && s.b[1331]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && (!s.b[1331])) {
            s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1316])) {
            s.store_mul_scale_ad_lhs(1220, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(477, A::add_scaled_product(A::add_scaled_products(s.ad_value(647), s.ad_value(1218), 1.0, s.ad_value(648), s.ad_value(1219), 1.0), 1.0, s.ad_value(649), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1333] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        s.b[1334] = (s.v[488] < s.v[655]);
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        s.b[1335] = (((((-0.5) * (s.v[488] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) && s.b[1335]) {
            s.store_exp_scaled_input(1184, 488, (s.v[372] * (-0.5)));
        }

        s.b[1336] = (((-0.5) * (s.v[488] * s.v[372])) < 0.0);
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) && (!s.b[1335])) && s.b[1336]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) && (!s.b[1335])) && (!s.b[1336])) {
            s.store_scaled_offset_ad(1184, A::mul(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(488), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && (!s.b[1334])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(488), s.v[372], s.ad_value(655), s.v[372]), 1.0, 656);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1333]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1337] = (s.v[488] > 0.0);
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1337]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && (!s.b[1337])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 488);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1333]) {
            s.store_sub(1187, 657, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(488), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(1187)), A::sub(s.ad_value(488), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(488), 0.5, s.ad_value(660), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(660)), A::sub(s.ad_value(488), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 488, A::sqrt(A::offset(A::mul(s.ad_value(488), s.ad_value(488)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1338] = (s.v[647] == 0.0);
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1338]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1338])) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.b[1339] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1339]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1340] = (p.p848 == 0.5);
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && s.b[1340]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && (!s.b[1340])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1341] = (p.p848 == 0.5);
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && s.b[1341]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && (!s.b[1341])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) {
            s.store_scale(1198, 1191, s.v[424]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[385]);
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.b[1342] = (p.p862 == 0.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1342]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[409] * s.v[439]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1343] = (((-p.p848) * s.v[412]) == (-1.0));
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1343]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1343])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[436]), s.ad_value(1202), s.ad_value(1205), s.v[436]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1344] = (s.v[1212] > 0.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1344]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1344])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1345] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1345]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1345])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1346] = (s.v[1212] > 0.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1346]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1347] = (s.v[1211] > (-230.25850929940458));
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1346])) && s.b[1347]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1346])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[436] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);
        }

        s.b[1348] = (p.p868 == 0.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1348]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1349] = (p.p848 == 0.5);
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && s.b[1349]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && (!s.b[1349])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p845, s.ad_value(1189)), (s.v[427] * s.v[412]), s.ad_value(1191), 1.0));
        }

        s.b[1350] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && s.b[1350]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1351] = (((-s.v[442]) / s.v[1216]) < 0.0);
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && (!s.b[1350])) && s.b[1351]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && (!s.b[1350])) && (!s.b[1351])) {
            let assign20100_ad_e21548: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign20100_ad_e21548, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868);
        }

        s.b[1352] = (p.p877 > 1000.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1352]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1353] = (s.v[1190] > ((-s.v[445]) * p.p877));
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        s.b[1354] = (p.p880 == 4.0);
        s.v[1354] = if s.b[1354] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && s.b[1353]) && s.b[1354]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && s.b[1353]) && (!s.b[1354])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && s.b[1353]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && (!s.b[1353])) {
            s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1338])) {
            s.store_mul_scale_ad_lhs(1218, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        s.b[1355] = (s.v[648] == 0.0);
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1355]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1355])) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.b[1356] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1356]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1357] = (p.p849 == 0.5);
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && s.b[1357]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && (!s.b[1357])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1358] = (p.p849 == 0.5);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && s.b[1358]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && (!s.b[1358])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) {
            s.store_scale(1198, 1191, s.v[425]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[386]);
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.b[1359] = (p.p863 == 0.0);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1359]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[410] * s.v[440]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1360] = (((-p.p849) * s.v[413]) == (-1.0));
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1360]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1360])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[437]), s.ad_value(1202), s.ad_value(1205), s.v[437]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1361] = (s.v[1212] > 0.0);
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1361]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1361])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1362] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1362] = if s.b[1362] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1362]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1362])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1363] = (s.v[1212] > 0.0);
        s.v[1363] = if s.b[1363] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1363]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1364] = (s.v[1211] > (-230.25850929940458));
        s.v[1364] = if s.b[1364] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1363])) && s.b[1364]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1364])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1363])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[437] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);
        }

        s.b[1365] = (p.p869 == 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1365]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1366] = (p.p849 == 0.5);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && s.b[1366]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && (!s.b[1366])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p846, s.ad_value(1189)), (s.v[428] * s.v[413]), s.ad_value(1191), 1.0));
        }

        s.b[1367] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && s.b[1367]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1368] = (((-s.v[443]) / s.v[1216]) < 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && (!s.b[1367])) && s.b[1368]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && (!s.b[1367])) && (!s.b[1368])) {
            let assign20800_ad_e22691: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign20800_ad_e22691, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869);
        }

        s.b[1369] = (p.p878 > 1000.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1369]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1370] = (s.v[1190] > ((-s.v[445]) * p.p878));
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        s.b[1371] = (p.p881 == 4.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && s.b[1370]) && s.b[1371]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && s.b[1370]) && (!s.b[1371])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && s.b[1370]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && (!s.b[1370])) {
            s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1355])) {
            s.store_mul_scale_ad_lhs(1219, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        s.b[1372] = (s.v[649] == 0.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1372]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1372])) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.b[1373] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1373]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1374] = (p.p850 == 0.5);
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && s.b[1374]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && (!s.b[1374])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1375] = (p.p850 == 0.5);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && s.b[1375]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && (!s.b[1375])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) {
            s.store_scale(1198, 1191, s.v[426]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[387]);
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.b[1376] = (p.p864 == 0.0);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1376]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[411] * s.v[441]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1377] = (((-p.p850) * s.v[414]) == (-1.0));
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1377]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1377])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[438]), s.ad_value(1202), s.ad_value(1205), s.v[438]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1378] = (s.v[1212] > 0.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1378]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1378])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1379] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1379]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1379])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1380] = (s.v[1212] > 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1380]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1381] = (s.v[1211] > (-230.25850929940458));
        s.v[1381] = if s.b[1381] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1380])) && s.b[1381]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1381])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1380])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[438] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);
        }

        s.b[1382] = (p.p870 == 0.0);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1382]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1383] = (p.p850 == 0.5);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && s.b[1383]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && (!s.b[1383])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p847, s.ad_value(1189)), (s.v[429] * s.v[414]), s.ad_value(1191), 1.0));
        }

        s.b[1384] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && s.b[1384]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1385] = (((-s.v[444]) / s.v[1216]) < 0.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && (!s.b[1384])) && s.b[1385]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && (!s.b[1384])) && (!s.b[1385])) {
            let assign21500_ad_e23834: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign21500_ad_e23834, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870);
        }

        s.b[1386] = (p.p879 > 1000.0);
        s.v[1386] = if s.b[1386] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1386]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1387] = (s.v[1190] > ((-s.v[445]) * p.p879));
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        s.b[1388] = (p.p882 == 4.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && s.b[1387]) && s.b[1388]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && s.b[1387]) && (!s.b[1388])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && s.b[1387]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && (!s.b[1387])) {
            s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1372])) {
            s.store_mul_scale_ad_lhs(1220, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(478, A::add_scaled_product(A::add_scaled_products(s.ad_value(647), s.ad_value(1218), 1.0, s.ad_value(648), s.ad_value(1219), 1.0), 1.0, s.ad_value(649), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1389] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        s.b[1390] = (s.v[489] < s.v[655]);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        s.b[1391] = (((((-0.5) * (s.v[489] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) && s.b[1391]) {
            s.store_exp_scaled_input(1184, 489, (s.v[372] * (-0.5)));
        }

        s.b[1392] = (((-0.5) * (s.v[489] * s.v[372])) < 0.0);
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) && (!s.b[1391])) && s.b[1392]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) && (!s.b[1391])) && (!s.b[1392])) {
            s.store_scaled_offset_ad(1184, A::mul(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(489), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && (!s.b[1390])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(489), s.v[372], s.ad_value(655), s.v[372]), 1.0, 656);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1389]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1393] = (s.v[489] > 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1393]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(1184), 1.0), A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && (!s.b[1393])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul(A::offset(s.ad_value(1185), 1.0), A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 489);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1389]) {
            s.store_sub(1187, 657, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(489), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(1187)), A::sub(s.ad_value(489), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(489), 0.5, s.ad_value(660), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(660)), A::sub(s.ad_value(489), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 489, A::sqrt(A::offset(A::mul(s.ad_value(489), s.ad_value(489)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1394] = (s.v[647] == 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1394]) {
            s.store_scalar(1218, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1155] && s.b[1172]) && (!s.b[1394])) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.b[1395] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1395]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1396] = (p.p848 == 0.5);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && s.b[1396]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && (!s.b[1396])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1397] = (p.p848 == 0.5);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && s.b[1397]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && (!s.b[1397])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) {
            s.store_scale(1198, 1191, s.v[424]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[385]);
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.b[1398] = (p.p862 == 0.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1398]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[409] * s.v[439]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1399] = (((-p.p848) * s.v[412]) == (-1.0));
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1399]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1399])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[436]), s.ad_value(1202), s.ad_value(1205), s.v[436]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1400] = (s.v[1212] > 0.0);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1400]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1400])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1401] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1401]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1401])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1402] = (s.v[1212] > 0.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1402]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1403] = (s.v[1211] > (-230.25850929940458));
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1402])) && s.b[1403]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1403])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1402])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[436] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);
        }

        s.b[1404] = (p.p868 == 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1404]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1405] = (p.p848 == 0.5);
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && s.b[1405]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && (!s.b[1405])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p845, s.ad_value(1189)), (s.v[427] * s.v[412]), s.ad_value(1191), 1.0));
        }

        s.b[1406] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && s.b[1406]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1407] = (((-s.v[442]) / s.v[1216]) < 0.0);
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && (!s.b[1406])) && s.b[1407]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && (!s.b[1406])) && (!s.b[1407])) {
            let assign22500_ad_e25478: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign22500_ad_e25478, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868);
        }

        s.b[1408] = (p.p877 > 1000.0);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1408]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1409] = (s.v[1190] > ((-s.v[445]) * p.p877));
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        s.b[1410] = (p.p880 == 4.0);
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && s.b[1409]) && s.b[1410]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && s.b[1409]) && (!s.b[1410])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && s.b[1409]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && (!s.b[1409])) {
            s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1394])) {
            s.store_mul_scale_ad_lhs(1218, A::add(A::add_scaled_inputs3(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0), s.ad_value(1215)), p.p29, 1217);
        }

        s.b[1411] = (s.v[648] == 0.0);
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1411]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1411])) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.b[1412] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1412]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1413] = (p.p849 == 0.5);
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && s.b[1413]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && (!s.b[1413])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1414] = (p.p849 == 0.5);
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && s.b[1414]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && (!s.b[1414])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) {
            s.store_scale(1198, 1191, s.v[425]);
            s.store_mul_scaled_ad_lhs(1199, A::offset(s.ad_value(1185), (-1.0)), 1198, s.v[386]);
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.b[1415] = (p.p863 == 0.0);
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1415]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[410] * s.v[440]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product(s.ad_value(1203), s.ad_value(1203), 1.0, A::offset(A::square(s.ad_value(1203)), 1.0), 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1416] = (((-p.p849) * s.v[413]) == (-1.0));
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1416]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1416])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_product(A::add_scaled_product(s.ad_value(1204), (-s.v[437]), s.ad_value(1202), s.ad_value(1205), s.v[437]), 1.0, s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1417] = (s.v[1212] > 0.0);
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1417]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1417])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1418] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1418]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1418])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_product(A::add_scaled_inputs(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374]), 1.0, A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1419] = (s.v[1212] > 0.0);
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1419]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1420] = (s.v[1211] > (-230.25850929940458));
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1419])) && s.b[1420]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1420])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1419])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[437] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);
        }

        s.b[1421] = (p.p869 == 0.0);
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1421]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1422] = (p.p849 == 0.5);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && s.b[1422]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && (!s.b[1422])) {
            s.store_powf_ad(1191, A::scale(A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) {
            s.store_ad_value(1216, A::div_scaled_inputs(A::sub_from_scalar(p.p846, s.ad_value(1189)), (s.v[428] * s.v[413]), s.ad_value(1191), 1.0));
        }

        s.b[1423] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && s.b[1423]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1424] = (((-s.v[443]) / s.v[1216]) < 0.0);
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && (!s.b[1423])) && s.b[1424]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && (!s.b[1423])) && (!s.b[1424])) {
            let assign23200_ad_e26621: A = A::offset(A::mul(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::offset(A::mul_scaled_output(A::offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458)), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign23200_ad_e26621, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869);
        }

        s.b[1425] = (p.p878 > 1000.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1425]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1426] = (s.v[1190] > ((-s.v[445]) * p.p878));
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        s.b[1427] = (p.p881 == 4.0);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && s.b[1426]) && s.b[1427]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && s.b[1426]) && (!s.b[1427])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);
        }

    }
}
