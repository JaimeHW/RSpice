#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[985] {s.store_scalar(638, p.p890);s.store_scalar(639, p.p891);s.store_scalar(640, p.p892);s.store_scalar(547, p.p883);s.store_scalar(548, p.p884);s.store_scalar(549, p.p885);s.store_scalar(550, p.p886);s.store_scalar(551, p.p887);s.store_scalar(552, p.p888);}
        if (!s.b[985]) {s.store_scalar(506, p.p893);s.store_scalar(507, p.p894);s.store_scalar(508, p.p895);s.store_scalar(509, p.p896);s.store_scalar(510, p.p897);s.store_scalar(511, p.p898);s.store_scalar(512, p.p899);s.store_scalar(513, p.p900);s.store_scalar(514, p.p901);s.store_scalar(515, p.p902);s.store_scalar(516, p.p903);s.store_scalar(517, p.p904);s.store_scalar(518, p.p905);s.store_scalar(519, p.p906);s.store_scalar(520, p.p907);s.store_scalar(523, p.p908);s.store_scalar(524, p.p909);s.store_scalar(525, p.p910);s.store_scalar(521, p.p911);s.store_scalar(522, p.p912);s.store_scalar(526, p.p913);s.store_scalar(527, p.p914);s.store_scalar(528, p.p915);s.store_scalar(529, p.p916);s.store_scalar(530, p.p917);s.store_scalar(531, p.p918);s.store_scalar(532, p.p919);s.store_scalar(533, p.p920);s.store_scalar(534, p.p921);s.store_scalar(535, p.p922);s.store_scalar(536, p.p923);s.store_scalar(537, p.p924);s.store_scalar(538, p.p925);s.store_scalar(539, p.p926);s.store_scalar(540, p.p927);s.store_scalar(541, p.p928);s.store_scalar(542, p.p929);s.store_scalar(543, p.p930);s.store_scalar(544, p.p931);s.store_scalar(545, p.p932);s.store_scalar(546, p.p933);s.store_scalar(554, p.p948);s.store_scalar(637, p.p940);s.store_scalar(638, p.p941);s.store_scalar(639, p.p942);s.store_scalar(640, p.p943);s.store_scalar(547, p.p934);s.store_scalar(548, p.p935);s.store_scalar(549, p.p936);s.store_scalar(550, p.p937);s.store_scalar(551, p.p938);s.store_scalar(552, p.p939);}
        s.store_primal_offset(555, 515, s.v[376]);s.store_primal_offset(556, 516, s.v[376]);s.store_primal_offset(557, 517, s.v[376]);s.store_primal_sub_from_scalar(576, 1.0, 512);s.store_primal_sub_from_scalar(577, 1.0, 513);s.store_primal_sub_from_scalar(578, 1.0, 514);s.store_primal_div_from_scalar(579, 1.0, 576);s.store_primal_div_from_scalar(580, 1.0, 577);s.store_primal_div_from_scalar(581, 1.0, 578);s.store_primal_div_from_scalar(591, s.v[756], 506);s.store_primal_div_scaled_inputs_indices(592, 521, s.v[756], 507, 1.0);s.store_primal_div_scaled_inputs_indices(593, 522, s.v[756], 508, 1.0);s.store_primal_div_from_scalar(594, 1.0, 591);s.store_primal_div_from_scalar(595, 1.0, 592);s.store_primal_div_from_scalar(596, 1.0, 593);s.store_primal_div_from_scalar(597, 1.0, 509);s.store_primal_div_from_scalar(598, 1.0, 510);s.store_primal_div_from_scalar(599, 1.0, 511);s.store_primal_div_from_scalar(615, 1.0, 541);s.store_primal_div_from_scalar(616, 1.0, 542);s.store_div_from_scalar(617, 1.0, 543);s.b[986] = ((((s.v[547] != 1.0) || (s.v[548] != 1.0)) || (s.v[549] != 1.0)) || (s.v[550] != 1.0));s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });
        if s.b[986] {s.store_scalar(636, 1.0);}
        if (!s.b[986]) {s.store_scalar(636, 0.0);}
        s.b[987] = (s.v[636] == 1.0);s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });
        if s.b[987] {
            if ((s.v[508] * s.v[547]) > 1e-18) {
                s.store_primal_mul(621, 508, 547);
            } else {
                s.store_scalar(621, 1e-18);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[987] {
            if ((s.v[511] * s.v[548]) > 0.05) {
                s.store_primal_mul(622, 511, 548);
            } else {
                s.store_scalar(622, 0.05);
            }
        }
        if s.b[987] {
            if ((if ((s.v[514] * s.v[549]) > 0.05) { (s.v[514] * s.v[549]) } else { 0.05 }) < 0.95) {
                if ((s.v[514] * s.v[549]) > 0.05) {
                    s.store_primal_mul(623, 514, 549);
                } else {
                    s.store_scalar(623, 0.05);
                }
            } else {
                s.store_scalar(623, 0.95);
            }
        }
        if s.b[987] {s.store_primal_mul(624, 517, 550);s.store_primal_offset(626, 624, s.v[376]);s.store_primal_sub_from_scalar(631, 1.0, 623);s.store_primal_div_from_scalar(632, 1.0, 631);}
        s.store_scalar(352, ((ctx_temp + p.p55) + p.p35));s.store_scalar(353, (s.v[352] / s.v[351]));s.store_scalar(354, (s.v[352] - s.v[351]));s.store_scalar(355, ((s.v[352] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(356, (1.0 / s.v[355]));s.store_scalar(366, (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0))));s.store_scalar(367, (s.v[366] / s.v[365]));s.store_scalar(371, (s.v[368] * s.v[366]));s.store_scalar(372, (1.0 / s.v[371]));s.store_scalar(377, ((-((0.000702 * s.v[366]) * s.v[366])) / (1108.0 + s.v[366])));s.store_scalar(382, (p.p851 + s.v[377]));s.store_scalar(383, (p.p852 + s.v[377]));s.store_scalar(384, (p.p853 + s.v[377]));s.store_scalar(385, (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[370]) - (s.v[382] * s.v[372])))) as f64).exp()));s.store_scalar(386, (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[370]) - (s.v[383] * s.v[372])))) as f64).exp()));s.store_scalar(387, (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[381] * s.v[370]) - (s.v[384] * s.v[372])))) as f64).exp()));s.store_scalar(388, ((p.p854 * s.v[385]) * s.v[385]));s.store_scalar(389, ((p.p855 * s.v[386]) * s.v[386]));s.store_scalar(390, ((p.p856 * s.v[387]) * s.v[387]));s.store_scalar(391, ((p.p845 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[385]) as f64).ln())));s.store_scalar(392, ((p.p846 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[386]) as f64).ln())));s.store_scalar(393, ((p.p847 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[387]) as f64).ln())));s.store_scalar(394, (s.v[391] + (s.v[371] * (((1.0 + ((((0.05 - s.v[391]) * s.v[372])) as f64).exp())) as f64).ln())));s.store_scalar(395, (s.v[392] + (s.v[371] * (((1.0 + ((((0.05 - s.v[392]) * s.v[372])) as f64).exp())) as f64).ln())));s.store_scalar(396, (s.v[393] + (s.v[371] * (((1.0 + ((((0.05 - s.v[393]) * s.v[372])) as f64).exp())) as f64).ln())));s.store_scalar(406, (1.0 / s.v[394]));s.store_scalar(407, (1.0 / s.v[395]));s.store_scalar(408, (1.0 / s.v[396]));s.store_scalar(415, (p.p842 * (((p.p845 * s.v[406])) as f64).powf(p.p848)));s.store_scalar(416, (p.p843 * (((p.p846 * s.v[407])) as f64).powf(p.p849)));s.store_scalar(417, (p.p844 * (((p.p847 * s.v[408])) as f64).powf(p.p850)));s.store_scalar(418, ((s.v[415] * s.v[394]) * s.v[412]));s.store_scalar(419, ((s.v[416] * s.v[395]) * s.v[413]));s.store_scalar(420, ((s.v[417] * s.v[396]) * s.v[414]));s.store_scalar(421, (2.0 * s.v[415]));s.store_scalar(422, (2.0 * s.v[416]));s.store_scalar(423, (2.0 * s.v[417]));s.store_scalar(433, ((0.5 * s.v[382])).max(s.v[371]));s.store_scalar(434, ((0.5 * s.v[383])).max(s.v[371]));s.store_scalar(435, ((0.5 * s.v[384])).max(s.v[371]));s.store_scalar(436, (s.v[433] * s.v[372]));s.store_scalar(437, (s.v[434] * s.v[372]));s.store_scalar(438, (s.v[435] * s.v[372]));s.store_scalar(439, (((((((32.0 * p.p865) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(440, (((((((32.0 * p.p866) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(441, (((((((32.0 * p.p867) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[435] * s.v[435]) * s.v[435]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(442, (p.p871 * (1.0 + (p.p874 * (s.v[366] - s.v[365])))));s.store_scalar(443, (p.p872 * (1.0 + (p.p875 * (s.v[366] - s.v[365])))));s.store_scalar(444, (p.p873 * (1.0 + (p.p876 * (s.v[366] - s.v[365])))));
        if (!(s.v[442] > 0.0)) {s.store_scalar(442, 0.0);}
        if (!(s.v[443] > 0.0)) {s.store_scalar(443, 0.0);}
        if (!(s.v[444] > 0.0)) {s.store_scalar(444, 0.0);}
        s.b[1007] = (s.v[474] == 1.0);s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });
        if s.b[1007] {s.store_primal_offset(462, 461, s.v[377]);s.store_primal_scale_ad(464, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(463), s.v[370], s.ad_value(462), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(465, 459, s.v[367], 464, (2.0 * s.v[371]));s.store_primal_add_scaled_inputs_mixed_ia(466, 465, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(465), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);s.store_primal_div_from_scalar(467, 1.0, 466);s.store_primal_mul_pow_mixed_iai(470, 458, A::mul(s.ad_value(459), s.ad_value(467)), 460);s.store_primal_mul3_lhs(471, 470, 466, 469);s.store_primal_scale(472, 470, 2.0);}
        s.store_primal_offset(558, 515, s.v[377]);s.store_primal_offset(559, 516, s.v[377]);s.store_primal_offset(560, 517, s.v[377]);s.store_primal_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[370], s.ad_value(558), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
    ) {
        s.store_primal_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[370], s.ad_value(559), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));s.store_primal_scale_ad(563, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(557), s.v[370], s.ad_value(560), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));s.store_primal_mul3_lhs(564, 518, 561, 561);s.store_primal_mul3_lhs(565, 519, 562, 562);s.store_primal_mul3_lhs(566, 520, 563, 563);s.store_primal_sub_scaled_inputs_ln_rhs(567, 509, s.v[367], 561, (2.0 * s.v[371]));s.store_primal_sub_scaled_inputs_ln_rhs(568, 510, s.v[367], 562, (2.0 * s.v[371]));s.store_primal_sub_scaled_inputs_ln_rhs(569, 511, s.v[367], 563, (2.0 * s.v[371]));s.store_primal_add_scaled_inputs_mixed_ia(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);s.store_primal_add_scaled_inputs_mixed_ia(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);s.store_primal_add_scaled_inputs_mixed_ia(572, 569, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(569), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);s.store_primal_div_from_scalar(573, 1.0, 570);s.store_primal_div_from_scalar(574, 1.0, 571);s.store_primal_div_from_scalar(575, 1.0, 572);s.store_primal_mul_pow_mixed_iai(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), 512);s.store_primal_mul_pow_mixed_iai(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), 513);s.store_primal_mul_pow_mixed_iai(584, 508, A::mul(s.ad_value(511), s.ad_value(575)), 514);s.store_primal_mul3_lhs(585, 582, 570, 579);s.store_primal_mul3_lhs(586, 583, 571, 580);s.store_primal_mul3_lhs(587, 584, 572, 581);s.store_primal_scale(588, 582, 2.0);s.store_primal_scale(589, 583, 2.0);s.store_primal_scale(590, 584, 2.0);s.store_primal_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[371]);s.store_primal_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[371]);s.store_primal_max_with_scalar_ad(602, A::scale(s.ad_value(560), 0.5), s.v[371]);s.store_primal_scale(603, 600, s.v[372]);s.store_primal_scale(604, 601, s.v[372]);s.store_primal_scale(605, 602, s.v[372]);s.store_primal_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(608, A::mul3_scaled_output(s.ad_value(531), A::square(s.ad_value(602)), s.ad_value(602), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_mul_scale_offset_rhs(609, 535, 538, (s.v[366] - s.v[365]), 1.0);s.store_primal_mul_scale_offset_rhs(610, 536, 539, (s.v[366] - s.v[365]), 1.0);s.store_mul_scale_offset_rhs(611, 537, 540, (s.v[366] - s.v[365]), 1.0);
        if (!(s.v[609] > 0.0)) {s.store_scalar(609, 0.0);}
        if (!(s.v[610] > 0.0)) {s.store_scalar(610, 0.0);}
        if (!(s.v[611] > 0.0)) {s.store_scalar(611, 0.0);}
        s.b[1008] = (s.v[636] == 1.0);s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });
        if s.b[1008] {s.store_primal_offset(625, 624, s.v[377]);s.store_primal_scale_ad(627, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(626), s.v[370], s.ad_value(625), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(628, 622, s.v[367], 627, (2.0 * s.v[371]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1008] {s.store_primal_add_scaled_inputs_mixed_ia(629, 628, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(628), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);s.store_primal_div_from_scalar(630, 1.0, 629);s.store_primal_mul_pow_mixed_iai(633, 621, A::mul(s.ad_value(622), s.ad_value(630)), 623);s.store_primal_mul3_lhs(634, 633, 629, 632);s.store_primal_scale(635, 633, 2.0);}
        s.store_scalar(1, 1.0);s.store_scalar(2, 1.0);s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);s.store_scalar(3, p.p0);s.store_scalar(4, p.p1);s.store_scalar(5, p.p2);s.store_scalar(6, p.p3);s.store_scalar(7, p.p4);s.store_scalar(8, p.p8);s.store_scalar(647, p.p19);s.store_scalar(648, p.p20);s.store_scalar(649, p.p21);s.store_scalar(674, p.p22);s.store_scalar(675, p.p23);s.store_scalar(676, p.p24);s.store_scalar(650, p.p25);s.store_scalar(651, p.p26);s.store_scalar(677, p.p27);s.store_scalar(678, p.p28);s.store_scalar(10, p.p14);s.b[1009] = (p.p39 > 0.0);s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });
        if s.b[1009] {s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));}
        if s.b[1009] {s.store_primal_floor_ad(1, A::offset(s.ad_value(1), 0.5));s.store_primal_div_from_scalar(2, 1.0, 1);}
        if ((s.v[4] * s.v[2]) > 1e-9) {
            s.store_primal_scale(4, 2, s.v[4]);
        } else {
            s.store_scalar(4, 1e-9);
        }
        s.store_scalar(11, p.p5);s.store_scalar(12, p.p6);s.store_scalar(13, p.p7);s.store_scalar(308, (1e-6 / s.v[3]));s.store_primal_div_from_scalar(309, 1e-6, 4);s.store_primal_offset_scaled(310, 309, ((p.p191) * ((p.p189 * (1.0 + (p.p190 * s.v[308]))))), (p.p189 * (1.0 + (p.p190 * s.v[308]))));s.store_primal_offset_scaled(311, 309, ((p.p195) * ((p.p193 * (1.0 + (p.p194 * s.v[308]))))), (p.p193 * (1.0 + (p.p194 * s.v[308]))));
        if (((s.v[3] + s.v[310]) - (2.0 * p.p192)) > 1e-9) {
            s.store_primal_offset(312, 310, ((s.v[3]) + ((-(2.0 * p.p192)))));
        } else {
            s.store_scalar(312, 1e-9);
        }
        if (((s.v[4] + s.v[311]) - (2.0 * p.p196)) > 1e-9) {
            s.store_primal_offset_add(313, 4, 311, (-(2.0 * p.p196)));
        } else {
            s.store_scalar(313, 1e-9);
        }
        s.store_primal_div_from_scalar(314, 1e-6, 312);s.store_primal_square(315, 314);s.store_primal_div_from_scalar(316, 1e-6, 313);s.store_primal_div_from_scalar(317, 1.0, 316);s.store_primal_mul(318, 314, 316);s.store_primal_div_from_scalar(319, 1.0, 318);
        if ((((s.v[3] + s.v[310]) - (2.0 * p.p192)) + p.p197) > 1e-9) {
            s.store_primal_offset(320, 310, ((((s.v[3]) + ((-(2.0 * p.p192))))) + (p.p197)));
        } else {
            s.store_scalar(320, 1e-9);
        }
        if ((((s.v[4] + s.v[311]) - (2.0 * p.p196)) + p.p198) > 1e-9) {
            s.store_primal_offset_add(321, 4, 311, (((-(2.0 * p.p196))) + (p.p198)));
        } else {
            s.store_scalar(321, 1e-9);
        }
        s.store_primal_scale(322, 321, 1000000.0);
        if (((s.v[3] + s.v[310]) + p.p197) > 1e-9) {
            s.store_primal_offset(323, 310, ((s.v[3]) + (p.p197)));
        } else {
            s.store_scalar(323, 1e-9);
        }
        if (((s.v[4] + s.v[311]) + p.p198) > 1e-9) {
            s.store_primal_offset_add(324, 4, 311, p.p198);
        } else {
            s.store_scalar(324, 1e-9);
        }
        s.store_primal_scale(325, 323, 1000000.0);s.store_primal_scale(326, 324, 1000000.0);s.store_scalar(40, p.p56);s.store_scalar(41, p.p57);s.store_scalar(42, p.p58);s.store_scalar(43, p.p59);s.store_scalar(44, p.p60);s.store_scalar(45, p.p61);s.store_scalar(46, p.p62);s.store_scalar(47, p.p63);s.store_scalar(48, p.p64);s.store_scalar(49, p.p65);s.store_scalar(50, p.p66);s.store_scalar(55, p.p67);s.store_scalar(56, p.p68);s.store_scalar(57, p.p69);s.store_scalar(58, p.p70);s.store_scalar(51, p.p71);s.store_scalar(52, p.p73);s.store_scalar(53, p.p72);s.store_scalar(54, p.p74);s.store_scalar(59, p.p78);s.store_scalar(60, p.p80);s.store_scalar(61, p.p79);s.store_scalar(62, p.p75);s.store_scalar(63, p.p77);s.store_scalar(64, p.p76);s.store_scalar(65, p.p81);s.store_scalar(66, p.p82);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(67, p.p83);s.store_scalar(68, p.p84);s.store_scalar(69, p.p85);s.store_scalar(70, p.p86);s.store_scalar(71, p.p87);s.store_scalar(72, p.p88);s.store_scalar(73, p.p89);s.store_scalar(74, p.p90);s.store_scalar(75, p.p91);s.store_scalar(76, p.p92);s.store_scalar(77, p.p93);s.store_scalar(78, p.p94);s.store_scalar(79, p.p95);s.store_scalar(80, p.p96);s.store_scalar(81, p.p97);s.store_scalar(82, p.p98);s.store_scalar(83, p.p99);s.store_scalar(84, p.p100);s.store_scalar(85, p.p101);s.store_scalar(86, p.p102);s.store_scalar(87, p.p103);s.store_scalar(88, p.p104);s.store_scalar(89, p.p105);s.store_scalar(90, p.p106);s.store_scalar(91, p.p107);s.store_scalar(92, p.p108);s.store_scalar(93, p.p109);s.store_scalar(94, p.p110);s.store_scalar(95, p.p111);s.store_scalar(96, p.p112);s.store_scalar(97, p.p113);s.store_scalar(98, p.p114);s.store_scalar(99, p.p115);s.store_scalar(100, p.p116);s.store_scalar(101, p.p117);s.store_scalar(102, p.p118);s.store_scalar(103, p.p119);s.store_scalar(104, p.p120);s.store_scalar(105, p.p119);s.b[1010] = param_given[121];s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });
        if s.b[1010] {s.store_scalar(105, p.p121);}
        s.store_scalar(106, p.p120);s.b[1011] = param_given[122];s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
        if s.b[1011] {s.store_scalar(106, p.p122);}
        s.copy_ad(107, 105);s.b[1012] = param_given[123];s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });
        if s.b[1012] {s.store_scalar(107, p.p123);}
        s.copy_ad(108, 106);s.b[1013] = param_given[124];s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });
        if s.b[1013] {s.store_scalar(108, p.p124);}
        s.store_scalar(109, p.p125);s.store_scalar(110, p.p126);s.store_scalar(111, p.p127);s.store_scalar(112, p.p128);s.store_scalar(113, p.p129);s.store_scalar(114, p.p130);s.store_scalar(115, p.p131);s.store_scalar(116, p.p132);s.store_scalar(117, p.p133);s.store_scalar(118, p.p134);s.store_scalar(119, p.p135);s.store_scalar(120, p.p136);s.store_scalar(121, p.p98);s.b[1014] = param_given[137];s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
        if s.b[1014] {s.store_scalar(121, p.p137);}
        s.store_scalar(122, p.p103);s.b[1015] = param_given[138];s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });
        if s.b[1015] {s.store_scalar(122, p.p138);}
        s.store_scalar(123, p.p139);s.store_scalar(124, p.p140);s.store_scalar(125, p.p141);s.store_scalar(126, p.p142);s.store_scalar(127, p.p143);s.store_scalar(128, p.p144);s.store_scalar(129, p.p145);s.store_scalar(130, p.p146);s.store_scalar(131, p.p147);s.store_scalar(132, p.p148);s.store_scalar(133, p.p149);s.store_scalar(134, p.p150);s.store_scalar(135, p.p151);s.store_scalar(136, p.p152);s.store_scalar(137, p.p153);s.store_scalar(138, p.p154);s.store_scalar(139, p.p155);s.store_scalar(145, p.p161);s.store_scalar(146, p.p162);s.store_scalar(147, p.p163);s.store_scalar(148, p.p164);s.store_scalar(149, p.p165);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(150, p.p166);s.store_scalar(151, p.p167);s.store_scalar(152, p.p168);s.store_scalar(153, p.p169);s.store_scalar(154, p.p170);s.store_scalar(155, p.p171);s.store_scalar(156, p.p173);s.store_scalar(157, p.p172);s.store_scalar(173, p.p187);s.b[1016] = (p.p39 > 0.0);s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
        if s.b[1016] {s.store_primal_add_scaled_inputs3_offset_mixed_aii(40, A::powf(s.ad_value(314), p.p201), p.p200, 316, p.p202, 318, p.p203, p.p199);s.store_primal_add_scaled_inputs3_offset_indices(41, 314, p.p205, 316, p.p206, 318, p.p207, p.p204);s.store_scalar(42, p.p208);s.store_scalar(43, p.p209);s.store_scalar(44, p.p210);}
        if s.b[1016] {
            s.store_primal_scale_ad(331, {
                if ((1.0 + ((p.p212 * s.v[316]) * (((1.0 + (s.v[313] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p212, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }
        if s.b[1016] {
            s.store_primal_scale_ad(332, {
                if ((1.0 + ((p.p215 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p215, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }
        if s.b[1016] {
            s.store_primal_scale_ad(333, {
                if ((1.0 + ((p.p218 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p218, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p217);
        }
        s.b[1017] = (s.v[312] > (2.0 * s.v[333]));s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1017]) {s.store_scalar(334, 75000000000.0);s.store_primal_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));s.store_primal_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);s.store_primal_square(336, 336);}
        s.b[1018] = (s.v[312] >= s.v[333]);s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
        if ((s.b[1016] && (!s.b[1017])) && s.b[1018]) {s.store_primal_add_mixed_ia(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));}
        if ((s.b[1016] && (!s.b[1017])) && (!s.b[1018])) {s.store_primal_add_mixed_ia(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));}
        if s.b[1016] {s.store_primal_mul_sub_scaled_inputs_rhs_mixed_ai(45, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p219)), 1.0, 315, p.p220);s.store_primal_add_scaled_inputs3_offset_mixed_aii(46, A::powf(s.ad_value(314), p.p223), p.p222, 316, p.p224, 318, p.p225, p.p221);s.store_scalar(47, p.p226);s.store_scalar(48, p.p227);s.store_primal_add_scaled_inputs3_offset_mixed_aii(49, A::powf(s.ad_value(314), p.p230), p.p229, 316, p.p231, 318, p.p232, p.p228);}
        if s.b[1016] {
            s.store_primal_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p234 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p234, 1.0)
                }
            }, p.p233);
        }
        if s.b[1016] {s.store_scalar(55, p.p235);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1016] {s.store_scalar(56, p.p236);s.store_scalar(57, p.p239);s.store_scalar(58, p.p240);s.store_primal_mul3_ad(51, A::scale_offset(A::powf(s.ad_value(314), p.p243), p.p242, p.p241), A::scale_offset(s.ad_value(316), p.p244, 1.0), A::scale_offset(s.ad_value(318), p.p245, 1.0));s.store_scalar(52, p.p247);s.store_scalar(53, p.p246);s.store_scalar(54, p.p248);s.store_primal_mul_powf_scale_offset_lhs(62, 314, 316, p.p250, (p.p251) * (p.p249), (1.0) * (p.p249));s.store_scalar(63, p.p253);s.store_scalar(64, p.p252);s.store_primal_mul_powf_scale_offset_lhs(59, 314, 316, p.p255, (p.p256) * (p.p254), (1.0) * (p.p254));s.store_scalar(60, p.p258);s.store_scalar(61, p.p257);s.store_primal_offset_scaled(337, 316, ((p.p261) * (p.p260)), p.p260);}
        if s.b[1016] {
            s.store_primal_scale_ad(338, {
                if ((1.0 + (p.p263 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p263, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p262);
        }
        if s.b[1016] {s.store_primal_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p264 * p.p265), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p265)))));}
        if s.b[1016] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }
        if s.b[1016] {s.store_primal_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p266, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p268), 1.0)), p.p267);s.store_primal_mul_div_scaled_inputs_mixed_iia(65, 340, 313, p.p259, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(66, 314, p.p270, 316, p.p271, 318, p.p272, p.p269);s.store_primal_offset_scaled(67, 316, ((p.p274) * (p.p273)), p.p273);s.store_scalar(68, p.p275);s.store_scalar(69, p.p276);s.store_scalar(70, p.p277);s.store_primal_mul3_ad(71, A::scale_offset(A::powf(s.ad_value(314), p.p280), p.p279, p.p278), A::scale_offset(s.ad_value(316), p.p281, 1.0), A::scale_offset(s.ad_value(318), p.p282, 1.0));s.store_scalar(72, p.p283);s.store_scalar(73, p.p284);s.store_scalar(74, p.p285);s.store_primal_mul3_ad_scaled_output(75, A::scale_offset(s.ad_value(314), p.p287, 1.0), A::scale_offset(s.ad_value(316), p.p288, 1.0), A::scale_offset(s.ad_value(318), p.p289, 1.0), p.p286);s.store_scalar(76, p.p290);s.store_scalar(77, p.p291);s.store_primal_mul_scale_offset_rhs(78, 316, 316, ((p.p293) * (p.p292)), p.p292);s.store_scalar(79, p.p294);s.store_scalar(80, p.p295);s.store_scalar(81, p.p296);s.store_primal_mul3_ad(82, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p298, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p299)), p.p297), A::scale_offset(s.ad_value(316), p.p300, 1.0), A::scale_offset(s.ad_value(318), p.p301, 1.0));s.store_primal_add_scaled_inputs3_offset_indices(83, 314, p.p303, 316, p.p304, 318, p.p305, p.p302);s.store_scalar(84, p.p306);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1016] {s.store_scalar(85, p.p307);s.store_scalar(86, p.p308);s.store_primal_div_from_scalar_offset_scaled_input(87, p.p309, 314, p.p310, 1.0);s.store_primal_mul_powf_scale_offset_lhs(88, 314, 316, p.p312, (p.p313) * (p.p311), (1.0) * (p.p311));s.store_primal_powf(341, 314, p.p315);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(89, 341, A::scale_offset(s.ad_value(316), p.p317, 1.0), p.p314, A::mul_scaled_lhs(s.ad_value(314), p.p316, s.ad_value(341)), 1.0, 1.0);s.store_primal_powf(341, 314, p.p319);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(90, 341, A::scale_offset(s.ad_value(316), p.p321, 1.0), p.p318, A::mul_scaled_lhs(s.ad_value(314), p.p320, s.ad_value(341)), 1.0, 1.0);s.store_scalar(91, p.p322);s.store_primal_scaled_mul_scale_offset_inputs(92, 314, p.p324, 1.0, 316, p.p325, 1.0, p.p323);s.store_scalar(93, p.p326);s.store_scalar(94, p.p327);s.store_primal_scaled_mul_scale_offset_inputs(95, 314, p.p329, 1.0, 316, p.p330, 1.0, p.p328);s.store_primal_scaled_mul_scale_offset_inputs(96, 314, p.p332, 1.0, 316, p.p333, 1.0, p.p331);s.store_scalar(97, p.p334);s.store_scalar(98, p.p335);s.store_primal_div_from_scalar(99, p.p336, 318);s.store_primal_div_from_scalar_scaled_input(100, (p.p337 * p.p237), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(101, (p.p338 * p.p238), 316, 1e-6);s.store_scalar(102, p.p339);s.store_scalar(103, p.p340);s.store_scalar(104, p.p341);s.store_scalar(105, p.p340);}
        s.b[1019] = param_given[342];s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1019]) {s.store_scalar(105, p.p342);}
        if s.b[1016] {s.store_scalar(106, p.p341);}
        s.b[1020] = param_given[343];s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1020]) {s.store_scalar(106, p.p343);}
        if s.b[1016] {s.copy_ad(107, 105);}
        s.b[1021] = param_given[344];s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1021]) {s.store_scalar(107, p.p344);}
        if s.b[1016] {s.copy_ad(108, 106);}
        s.b[1022] = param_given[345];s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1022]) {s.store_scalar(108, p.p345);}
        if s.b[1016] {s.store_scalar(109, p.p346);s.store_primal_div_from_scalar_scaled_input(110, (p.p347 * p.p237), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(111, (p.p348 * p.p238), 316, 1e-6);s.store_scalar(112, p.p349);s.store_scalar(113, p.p350);s.store_scalar(114, p.p351);s.store_scalar(115, p.p352);s.store_scalar(116, p.p353);s.store_scalar(117, p.p354);s.store_primal_scaled_mul(118, 321, 320, ((8.8541878176e-12 * p.p210) * 1.0 / (p.p209)));s.store_primal_scale(125, 321, ((8.8541878176e-12 * p.p210) * (p.p237 * 1.0 / (p.p235))));s.store_primal_scale(126, 321, ((8.8541878176e-12 * p.p210) * (p.p238 * 1.0 / (p.p236))));s.store_primal_add_scaled_inputs3_offset_mixed_aii(119, A::powf(s.ad_value(314), p.p357), p.p356, 316, p.p358, 318, p.p359, p.p355);s.store_primal_add_scaled_inputs3_offset_indices(120, 314, p.p361, 316, p.p362, 318, p.p363, p.p360);s.store_scalar(32, p.p297);}
        s.b[1023] = param_given[364];s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1023]) {s.store_scalar(32, p.p364);}
        if s.b[1016] {s.store_scalar(33, p.p298);}
        s.b[1024] = param_given[365];s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1024]) {s.store_scalar(33, p.p365);}
        if s.b[1016] {s.store_scalar(34, p.p299);}
        s.b[1025] = param_given[366];s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1025]) {s.store_scalar(34, p.p366);}
        if s.b[1016] {s.store_scalar(35, p.p300);}
        s.b[1026] = param_given[367];s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1026]) {s.store_scalar(35, p.p367);}
        if s.b[1016] {s.store_scalar(36, p.p301);}
        s.b[1027] = param_given[368];s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1027]) {s.store_scalar(36, p.p368);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1016] {s.store_primal_mul3_ad(121, A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(318)), 1.0));s.store_scalar(37, p.p309);}
        s.b[1028] = param_given[369];s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1028]) {s.store_scalar(37, p.p369);}
        if s.b[1016] {s.store_scalar(38, p.p310);}
        s.b[1029] = param_given[370];s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1029]) {s.store_scalar(38, p.p370);}
        if s.b[1016] {s.store_primal_div_scaled_value_offset_denominator(122, s.ad_value(37), 1.0, A::mul(s.ad_value(38), s.ad_value(314)), 1.0, 1.0);s.store_primal_mul_powf_scale_offset_lhs(123, 314, 316, p.p372, (p.p373) * (p.p371), (1.0) * (p.p371));s.store_primal_powf(341, 314, p.p375);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(124, 341, A::scale_offset(s.ad_value(316), p.p377, 1.0), p.p374, A::mul_scaled_lhs(s.ad_value(314), p.p376, s.ad_value(341)), 1.0, 1.0);s.store_scalar(127, p.p378);s.store_scalar(128, p.p379);s.store_scalar(129, p.p380);s.store_primal_scale(130, 325, p.p381);s.store_primal_scale(131, 322, p.p382);s.store_primal_scale(132, 322, p.p383);s.store_scalar(133, p.p384);s.store_scalar(134, p.p385);s.store_scalar(135, p.p386);s.store_scalar(136, p.p387);s.store_primal_scale(137, 326, p.p388);s.store_primal_scale(138, 326, p.p389);s.store_primal_sub_from_scalar_ad(998, 1.0, A::div_from_scalar((2.0 * p.p396), s.ad_value(312)));s.store_scalar(139, p.p390);s.store_primal_offset_scaled(344, 313, p.p399, (2.0 * p.p398));s.store_scalar(145, p.p400);s.store_primal_add_scaled_inputs3_offset_indices(146, 314, p.p402, 316, p.p403, 318, p.p404, p.p401);s.store_primal_add_scaled_inputs3_offset_mixed_aii(147, A::powf(s.ad_value(314), p.p407), p.p406, 316, p.p408, 318, p.p409, p.p405);s.store_primal_mul3_ad_scaled_output(148, A::scale_offset(A::powf(s.ad_value(314), p.p412), p.p411, 1.0), A::scale_offset(s.ad_value(316), p.p413, 1.0), A::scale_offset(s.ad_value(318), p.p414, 1.0), p.p410);s.store_primal_offset_scaled_ad(149, A::powf(s.ad_value(314), p.p417), p.p416, p.p415);s.store_primal_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p418 * p.p419), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p419)))), 1.0);}
        if s.b[1016] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }
        if s.b[1016] {s.store_primal_mul_div_scaled_inputs_mixed_aia(150, A::scale_offset(s.ad_value(316), p.p420, 1.0), 344, p.p259, A::mul(s.ad_value(347), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(151, 314, p.p422, 316, p.p423, 318, p.p424, p.p421);s.store_primal_mul_powf_scale_offset_lhs(152, 314, 316, p.p426, (p.p427) * (p.p425), (1.0) * (p.p425));s.store_scalar(153, p.p428);s.store_scalar(154, p.p429);s.store_primal_mul_powf_scale_offset_lhs(155, 314, 316, p.p431, (p.p432) * (p.p430), (1.0) * (p.p430));s.store_scalar(156, p.p434);s.store_scalar(157, p.p433);s.store_primal_add_scaled_inputs3_offset_indices(348, 314, p.p832, 316, p.p833, 318, p.p834, p.p831);s.store_primal_add_scaled_inputs3_offset_indices(349, 314, p.p836, 316, p.p837, 318, p.p838, p.p835);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1016] {s.store_primal_offset_div_scaled_offset_numerator_mixed_ai(173, A::div_from_scalar(p.p458, s.ad_value(314)), p.p456, (((1.0) + (p.p457)) * p.p456), 316, 1.0, p.p455);}
        s.b[1031] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1031]) {s.store_primal_add_scaled_inputs3_offset_indices(40, 314, p.p461, 316, p.p462, 318, p.p463, p.p460);}
        s.b[1032] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1032]) {s.store_primal_add_scaled_inputs3_offset_indices(41, 314, p.p465, 316, p.p466, 318, p.p467, p.p464);}
        s.b[1033] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1033]) {s.store_primal_add_scaled_inputs3_offset_indices(45, 314, p.p469, 316, p.p470, 318, p.p471, p.p468);}
        s.b[1034] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1034]) {s.store_primal_add_scaled_inputs3_offset_indices(46, 314, p.p473, 316, p.p474, 318, p.p475, p.p472);}
        s.b[1035] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1035]) {s.store_primal_add_scaled_inputs3_offset_indices(47, 314, p.p477, 316, p.p478, 318, p.p479, p.p476);}
        s.b[1036] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1036]) {s.store_primal_add_scaled_inputs3_offset_indices(49, 314, p.p481, 316, p.p482, 318, p.p483, p.p480);}
        s.b[1037] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1037]) {s.store_primal_add_scaled_inputs3_offset_indices(50, 314, p.p485, 316, p.p486, 318, p.p487, p.p484);}
        s.b[1038] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1038]) {s.store_primal_add_scaled_inputs3_offset_indices(57, 314, p.p489, 316, p.p490, 318, p.p491, p.p488);}
        s.b[1039] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1039]) {s.store_primal_add_scaled_inputs3_offset_indices(58, 314, p.p493, 316, p.p494, 318, p.p495, p.p492);}
        s.b[1040] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1040]) {s.store_primal_add_scaled_inputs3_offset_indices(51, 314, p.p497, 316, p.p498, 318, p.p499, p.p496);}
        s.b[1041] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1041]) {s.store_primal_add_scaled_inputs3_offset_indices(52, 314, p.p505, 316, p.p506, 318, p.p507, p.p504);}
        s.b[1042] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1042]) {s.store_primal_add_scaled_inputs3_offset_indices(53, 314, p.p501, 316, p.p502, 318, p.p503, p.p500);}
        s.b[1043] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1043]) {s.store_primal_add_scaled_inputs3_offset_indices(54, 314, p.p509, 316, p.p510, 318, p.p511, p.p508);}
        s.b[1044] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1044]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(62, 315, 314, p.p513, 316, p.p514, 318, p.p515, p.p512);}
        s.b[1045] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1045]) {s.store_primal_add_scaled_inputs3_offset_indices(63, 314, p.p521, 316, p.p522, 318, p.p523, p.p520);}
        s.b[1046] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1046]) {s.store_primal_add_scaled_inputs3_offset_indices(64, 314, p.p517, 316, p.p518, 318, p.p519, p.p516);}
        s.b[1047] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1047]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(59, 315, 314, p.p525, 316, p.p526, 318, p.p527, p.p524);}
        s.b[1048] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1048]) {s.store_primal_add_scaled_inputs3_offset_indices(60, 314, p.p533, 316, p.p534, 318, p.p535, p.p532);}
        s.b[1049] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1049]) {s.store_primal_add_scaled_inputs3_offset_indices(61, 314, p.p529, 316, p.p530, 318, p.p531, p.p528);}
        s.b[1050] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1050]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(65, A::add_scaled_inputs3_offset(s.ad_value(314), p.p537, s.ad_value(316), p.p538, s.ad_value(318), p.p539, p.p536), 313, 1.0, 312, 1.0);}
        s.b[1051] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1051]) {s.store_primal_add_scaled_inputs3_offset_indices(66, 314, p.p541, 316, p.p542, 318, p.p543, p.p540);}
        s.b[1052] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1052]) {s.store_primal_add_scaled_inputs3_offset_indices(67, 314, p.p545, 316, p.p546, 318, p.p547, p.p544);}
        s.b[1053] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1053]) {s.store_primal_add_scaled_inputs3_offset_indices(69, 314, p.p549, 316, p.p550, 318, p.p551, p.p548);}
        s.b[1054] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1054]) {s.store_primal_add_scaled_inputs3_offset_indices(71, 314, p.p553, 316, p.p554, 318, p.p555, p.p552);}
        s.b[1055] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1055]) {s.store_primal_add_scaled_inputs3_offset_indices(73, 314, p.p557, 316, p.p558, 318, p.p559, p.p556);}
        s.b[1056] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1056]) {s.store_primal_add_scaled_inputs3_offset_indices(75, 314, p.p561, 316, p.p562, 318, p.p563, p.p560);}
        s.b[1057] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1057]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(78, 316, 314, p.p565, 316, p.p566, 318, p.p567, p.p564);}
        s.b[1058] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1058]) {s.store_primal_add_scaled_inputs3_offset_indices(79, 314, p.p569, 316, p.p570, 318, p.p571, p.p568);}
        s.b[1059] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1016] && s.b[1059]) {s.store_primal_add_scaled_inputs3_offset_indices(80, 314, p.p573, 316, p.p574, 318, p.p575, p.p572);}
        s.b[1060] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1060]) {s.store_primal_add_scaled_inputs3_offset_indices(81, 314, p.p577, 316, p.p578, 318, p.p579, p.p576);}
        s.b[1061] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1061]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(82, 314, 314, p.p581, 316, p.p582, 318, p.p583, p.p580);}
        s.b[1062] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1062]) {s.store_primal_add_scaled_inputs3_offset_indices(83, 314, p.p585, 316, p.p586, 318, p.p587, p.p584);}
        s.b[1063] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1063]) {s.store_primal_add_scaled_inputs3_offset_indices(84, 314, p.p589, 316, p.p590, 318, p.p591, p.p588);}
        s.b[1064] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1064]) {s.store_primal_add_scaled_inputs3_offset_indices(85, 314, p.p593, 316, p.p594, 318, p.p595, p.p592);}
        s.b[1065] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1065]) {s.store_primal_add_scaled_inputs3_offset_indices(87, 314, p.p597, 316, p.p598, 318, p.p599, p.p596);}
        s.b[1066] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1066]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(88, 314, 314, p.p601, 316, p.p602, 318, p.p603, p.p600);}
        s.b[1067] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1067]) {s.store_primal_add_scaled_inputs3_offset_indices(89, 314, p.p605, 316, p.p606, 318, p.p607, p.p604);}
        s.b[1068] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1068]) {s.store_primal_add_scaled_inputs3_offset_indices(90, 314, p.p609, 316, p.p610, 318, p.p611, p.p608);}
        s.b[1069] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1069]) {s.store_primal_add_scaled_inputs3_offset_indices(92, 314, p.p613, 316, p.p614, 318, p.p615, p.p612);}
        s.b[1070] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1070]) {s.store_primal_add_scaled_inputs3_offset_indices(94, 314, p.p617, 316, p.p618, 318, p.p619, p.p616);}
        s.b[1071] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1071]) {s.store_primal_add_scaled_inputs3_offset_indices(95, 314, p.p621, 316, p.p622, 318, p.p623, p.p620);}
        s.b[1072] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1072]) {s.store_primal_add_scaled_inputs3_offset_indices(96, 314, p.p625, 316, p.p626, 318, p.p627, p.p624);}
        s.b[1073] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1073]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(99, 319, 314, p.p629, 316, p.p630, 318, p.p631, p.p628);}
        s.b[1074] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1074]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(100, 317, 314, p.p633, 316, p.p634, 318, p.p635, p.p632);}
        s.b[1075] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1075]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(101, 317, 314, p.p637, 316, p.p638, 318, p.p639, p.p636);}
        s.b[1076] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1076]) {s.store_primal_add_scaled_inputs3_offset_indices(102, 314, p.p641, 316, p.p642, 318, p.p643, p.p640);}
        s.b[1077] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1077]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(110, 317, 314, p.p645, 316, p.p646, 318, p.p647, p.p644);}
        s.b[1078] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1078]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(111, 317, 314, p.p649, 316, p.p650, 318, p.p651, p.p648);}
        s.b[1079] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1079]) {s.store_primal_add_scaled_inputs3_offset_indices(114, 314, p.p653, 316, p.p654, 318, p.p655, p.p652);}
        s.b[1080] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1080]) {s.store_primal_add_scaled_inputs3_offset_indices(115, 314, p.p657, 316, p.p658, 318, p.p659, p.p656);}
        s.b[1081] = (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1081]) {s.store_primal_mul_ad_affine_product_rhs(118, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p661, s.ad_value(316), p.p662, s.ad_value(318), p.p663, p.p660), 1.0 / (1e-6), 0.0);}
        s.b[1082] = (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1082]) {s.store_primal_add_scaled_inputs3_offset_indices(119, 314, p.p665, 316, p.p666, 318, p.p667, p.p664);}
        s.b[1083] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1083]) {s.store_primal_add_scaled_inputs3_offset_indices(120, 314, p.p669, 316, p.p670, 318, p.p671, p.p668);}
        s.b[1084] = (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1084]) {s.store_scalar(28, p.p580);}
        s.b[1085] = param_given[672];s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1084]) && s.b[1085]) {s.store_scalar(28, p.p672);}
        if (s.b[1016] && s.b[1084]) {s.store_scalar(29, p.p581);}
        s.b[1086] = param_given[673];s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1084]) && s.b[1086]) {s.store_scalar(29, p.p673);}
        if (s.b[1016] && s.b[1084]) {s.store_scalar(30, p.p582);}
        s.b[1087] = param_given[674];s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1084]) && s.b[1087]) {s.store_scalar(30, p.p674);}
        if (s.b[1016] && s.b[1084]) {s.store_scalar(31, p.p583);}
        s.b[1088] = param_given[675];s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1084]) && s.b[1088]) {s.store_scalar(31, p.p675);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1016] && s.b[1084]) {s.store_primal_mul_mixed_ia(121, 314, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(314), 1.0, s.ad_value(30), s.ad_value(316), 1.0, s.ad_value(31), s.ad_value(318), 1.0));}
        s.b[1089] = (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]);s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1089]) {s.store_scalar(28, p.p596);}
        s.b[1090] = param_given[676];s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1089]) && s.b[1090]) {s.store_scalar(28, p.p676);}
        if (s.b[1016] && s.b[1089]) {s.store_scalar(29, p.p597);}
        s.b[1091] = param_given[677];s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1089]) && s.b[1091]) {s.store_scalar(29, p.p677);}
        if (s.b[1016] && s.b[1089]) {s.store_scalar(30, p.p598);}
        s.b[1092] = param_given[678];s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1089]) && s.b[1092]) {s.store_scalar(30, p.p678);}
        if (s.b[1016] && s.b[1089]) {s.store_scalar(31, p.p599);}
        s.b[1093] = param_given[679];s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1089]) && s.b[1093]) {s.store_scalar(31, p.p679);}
        if (s.b[1016] && s.b[1089]) {s.store_primal_add_scaled_value_products3_indices(122, 28, 1.0, 29, 314, 1.0, 30, 316, 1.0, 31, 318, 1.0);}
        s.b[1094] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1094]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(123, 314, 314, p.p681, 316, p.p682, 318, p.p683, p.p680);}
        s.b[1095] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1095]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(124, 314, 314, p.p685, 316, p.p686, 318, p.p687, p.p684);}
        s.b[1096] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1096]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(125, 322, 314, p.p689, 316, p.p690, 318, p.p691, p.p688);}
        s.b[1097] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1097]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(126, 322, 314, p.p693, 316, p.p694, 318, p.p695, p.p692);}
        s.b[1098] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1098]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(130, 325, 314, p.p697, 316, p.p698, 318, p.p699, p.p696);}
        s.b[1099] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1099]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(131, 322, 314, p.p701, 316, p.p702, 318, p.p703, p.p700);}
        s.b[1100] = (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1100]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(132, 322, 314, p.p705, 316, p.p706, 318, p.p707, p.p704);}
        s.b[1101] = (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1101]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(137, 326, 314, p.p709, 316, p.p710, 318, p.p711, p.p708);}
        s.b[1102] = (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1102]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(138, 326, 314, p.p713, 316, p.p714, 318, p.p715, p.p712);}
        s.b[1107] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1107]) {s.store_primal_add_scaled_inputs3_offset_indices(145, 314, p.p733, 316, p.p734, 318, p.p735, p.p732);}
        s.b[1108] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1108]) {s.store_primal_add_scaled_inputs3_offset_indices(146, 314, p.p737, 316, p.p738, 318, p.p739, p.p736);}
        s.b[1109] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1109]) {s.store_primal_add_scaled_inputs3_offset_indices(147, 314, p.p741, 316, p.p742, 318, p.p743, p.p740);}
        s.b[1110] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1110]) {s.store_primal_add_scaled_inputs3_offset_indices(148, 314, p.p745, 316, p.p746, 318, p.p747, p.p744);}
        s.b[1111] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1111]) {s.store_primal_add_scaled_inputs3_offset_indices(149, 314, p.p749, 316, p.p750, 318, p.p751, p.p748);}
        s.b[1112] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1112]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(150, A::add_scaled_inputs3_offset(s.ad_value(314), p.p753, s.ad_value(316), p.p754, s.ad_value(318), p.p755, p.p752), 344, 1.0, 312, 1.0);}
        s.b[1113] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1113]) {s.store_primal_add_scaled_inputs3_offset_indices(151, 314, p.p757, 316, p.p758, 318, p.p759, p.p756);}
        s.b[1114] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1114]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(152, 315, 314, p.p761, 316, p.p762, 318, p.p763, p.p760);}
        s.b[1115] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1115]) {s.store_primal_add_scaled_inputs3_offset_indices(153, 314, p.p765, 316, p.p766, 318, p.p767, p.p764);}
        s.b[1116] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1116]) {s.store_primal_add_scaled_inputs3_offset_indices(154, 314, p.p769, 316, p.p770, 318, p.p771, p.p768);}
        s.b[1117] = (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1117]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(155, 315, 314, p.p773, 316, p.p774, 318, p.p775, p.p772);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1118] = (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]);s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1118]) {s.store_primal_add_scaled_inputs3_offset_indices(156, 314, p.p781, 316, p.p782, 318, p.p783, p.p780);}
        s.b[1119] = (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]);s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1119]) {s.store_primal_add_scaled_inputs3_offset_indices(157, 314, p.p777, 316, p.p778, 318, p.p779, p.p776);}
        s.b[1124] = (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1124]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(173, 319, 314, p.p801, 316, p.p802, 318, p.p803, p.p800);}
        if s.b[1016] {s.store_scalar(1005, 0.0);s.store_scalar(1006, 0.0);s.store_scalar(1004, 0.0);s.store_scalar(39, p.p812);}
        s.b[1126] = param_given[813];s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
        if (s.b[1016] && s.b[1126]) {s.store_scalar(39, p.p813);}
        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });let mut t2: usize = 0;
        while {
            let t0: f64 = (s.v[1] - 0.5);let t1: f64 = if ((s.b[1016] && s.b[1127]) && (s.v[1004] < t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;assert!(t2 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1016] && s.b[1127]) {s.store_primal_add_mixed_ia(1005, 1005, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));s.store_primal_add_mixed_ia(1006, 1006, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));s.store_primal_offset(1004, 1004, 1.0);}
        }
        if (s.b[1016] && s.b[1127]) {s.store_primal_mul(989, 1005, 2);s.store_primal_mul(990, 1006, 2);s.store_scalar(991, (1.0 / (p.p808 + (0.5 * s.v[3]))));s.store_scalar(992, (1.0 / (p.p809 + (0.5 * s.v[3]))));}
        if (s.b[1016] && s.b[1127]) {
            if ((s.v[3] + s.v[310]) > 1e-9) {
                s.store_primal_offset(1002, 310, s.v[3]);
            } else {
                s.store_scalar(1002, 1e-9);
            }
        }
        if (s.b[1016] && s.b[1127]) {
            if (((s.v[4] + s.v[311]) + p.p810) > 1e-9) {
                s.store_primal_offset_add(1003, 4, 311, p.p810);
            } else {
                s.store_scalar(1003, 1e-9);
            }
        }
        if (s.b[1016] && s.b[1127]) {s.store_primal_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p818);s.store_primal_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p819);s.store_primal_add_scaled_inputs_product_mixed_aiii(993, A::scale_offset(s.ad_value(1000), p.p815, 1.0), (1.0 + (p.p814 * (s.v[353] - 1.0))), 1001, (p.p816 * (1.0 + (p.p814 * (s.v[353] - 1.0)))), 1000, 1001, (p.p817 * (1.0 + (p.p814 * (s.v[353] - 1.0)))));s.store_primal_div_scaled_inputs2_indices(994, 989, p.p811, 990, p.p811, 993, 1.0);s.store_primal_div_scaled_inputs2_indices(995, 991, p.p811, 992, p.p811, 993, 1.0);s.store_primal_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p824);s.store_primal_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p825);s.store_primal_add_scaled_inputs_product_mixed_aiii(996, A::scale_offset(s.ad_value(1000), p.p821, 1.0), 1.0, 1001, p.p822, 1000, 1001, p.p823);s.store_primal_add_scaled_inputs4_indices(998, 989, 1.0, 990, 1.0, 991, -1.0, 992, -1.0);s.store_primal_div_scaled_offset_numerator_mixed_ia(999, 994, 1.0, 1.0, A::offset(s.ad_value(995), 1.0), 1.0);s.store_primal_mul(65, 65, 999);s.store_primal_div_scaled_product3_mixed_iiaa(82, 82, 999, A::scale_offset(s.ad_value(995), p.p812, 1.0), 1.0, A::scale_offset(s.ad_value(994), p.p812, 1.0), 1.0);s.store_primal_div_scaled_product3_mixed_iiaa(121, 121, 999, A::offset(A::mul(s.ad_value(39), s.ad_value(995)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(994)), 1.0), 1.0);s.store_primal_mul(150, 150, 999);s.store_primal_div_scaled_inputs_indices(999, 998, p.p820, 996, 1.0);s.store_primal_add(40, 40, 999);s.store_primal_add(145, 145, 999);s.store_primal_div_scaled_inputs_mixed_ia(999, 998, p.p826, A::powf(s.ad_value(996), p.p827), 1.0);s.store_primal_add(62, 62, 999);s.store_primal_add(155, 155, 999);}
        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if ((s.b[1016] && s.b[1128]) && s.b[1129]) {s.store_primal_offset(998, 4, s.v[8]);s.store_scalar(999, (1.0 / p.p828));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1016] && s.b[1128]) && s.b[1129]) {s.store_primal_div_from_scalar_scaled_input(11, (p.p828 * p.p828), 998, s.v[8]);s.store_primal_div_scaled_add_product_mixed_aaai(12, A::exp_scaled_input(s.ad_value(999), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p828)), A::scale_offset(s.ad_value(998), 0.1, (0.01 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-10.0), s.ad_value(999))), (-1.0), 4, 1.0);s.store_primal_div_scaled_add_product_mixed_aaai(13, A::exp_scaled_input(s.ad_value(999), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p828)), A::scale_offset(s.ad_value(998), 0.05, (0.0025 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-20.0), s.ad_value(999))), (-1.0), 4, 1.0);}
        if (s.b[1016] && s.b[1128]) {s.store_primal_add_scaled_inputs3_indices(998, 11, 1.0, 12, p.p829, 13, p.p830);s.store_primal_add_scaled_product_indices(40, 40, 1.0, 348, 998, 1.0);s.store_primal_mul_scale_offset_mixed_ia(65, 65, A::mul(s.ad_value(349), s.ad_value(998)), 1.0, 1.0);s.store_primal_add_scaled_product_indices(145, 145, 1.0, 348, 998, 1.0);s.store_primal_mul_scale_offset_mixed_ia(150, 150, A::mul(s.ad_value(349), s.ad_value(998)), 1.0, 1.0);}
        s.copy_ad(175, 40);s.copy_ad(176, 41);s.copy_ad(177, 42);s.copy_ad(179, 43);s.copy_ad(180, 44);
        if (s.v[45] > 1e20) {
            if (s.v[45] < 1e26) {
                s.copy_ad(181, 45);
            } else {
                s.store_scalar(181, 1e26);
            }
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
        s.copy_ad(184, 48);s.copy_ad(185, 49);
        if (s.v[50] > 0.0) {
            s.copy_ad(186, 50);
        } else {
            s.store_scalar(186, 0.0);
        }
        s.copy_ad(190, 55);s.copy_ad(191, 56);
        if (s.v[57] > 1e23) {
            if (s.v[57] < 1e27) {
                s.copy_ad(192, 57);
            } else {
                s.store_scalar(192, 1e27);
            }
        } else {
            s.store_scalar(192, 1e23);
        }
        if (s.v[58] > 1e23) {
            if (s.v[58] < 1e27) {
                s.copy_ad(193, 58);
            } else {
                s.store_scalar(193, 1e27);
            }
        } else {
            s.store_scalar(193, 1e23);
        }
        if (s.v[51] > 0.0) {
            s.copy_ad(187, 51);
        } else {
            s.store_scalar(187, 0.0);
        }
        if (s.v[53] > 0.0) {
            if (s.v[53] < 0.5) {
                s.copy_ad(189, 53);
            } else {
                s.store_scalar(189, 0.5);
            }
        } else {
            s.store_scalar(189, 0.0);
        }
        if (s.v[52] > 0.0) {
            if (s.v[52] < 1.0) {
                s.copy_ad(188, 52);
            } else {
                s.store_scalar(188, 1.0);
            }
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
            if (s.v[64] < 1.0) {
                s.copy_ad(196, 64);
            } else {
                s.store_scalar(196, 1.0);
            }
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
            if (s.v[61] < 1.0) {
                s.copy_ad(198, 61);
            } else {
                s.store_scalar(198, 1.0);
            }
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
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
        s.copy_ad(211, 76);s.copy_ad(212, 77);
        if (s.v[78] > 0.0) {
            s.copy_ad(213, 78);
        } else {
            s.store_scalar(213, 0.0);
        }
        s.copy_ad(214, 79);
        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(215, 80);
            } else {
                s.store_scalar(215, 1.0);
            }
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
            if (s.v[84] < 1.0) {
                s.copy_ad(219, 84);
            } else {
                s.store_scalar(219, 1.0);
            }
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
        s.copy_ad(228, 93);s.copy_ad(229, 94);
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
        s.copy_ad(237, 102);s.copy_ad(238, 103);s.copy_ad(239, 104);s.copy_ad(240, 105);s.copy_ad(241, 106);s.copy_ad(242, 107);s.copy_ad(243, 108);s.copy_ad(244, 109);
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
        s.copy_ad(247, 112);s.copy_ad(248, 113);s.copy_ad(249, 114);s.copy_ad(250, 115);s.copy_ad(251, 116);s.copy_ad(252, 117);
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
        s.copy_ad(262, 127);s.copy_ad(263, 128);s.copy_ad(264, 129);
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.copy_ad(268, 133);s.copy_ad(269, 134);s.copy_ad(270, 135);s.copy_ad(271, 136);
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
        s.copy_ad(274, 139);s.copy_ad(280, 145);s.copy_ad(281, 146);s.copy_ad(282, 147);
        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(283, 148);
            } else {
                s.store_scalar(283, 1e26);
            }
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
            if (s.v[153] < 1.0) {
                s.copy_ad(288, 153);
            } else {
                s.store_scalar(288, 1.0);
            }
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
            if (s.v[157] < 1.0) {
                s.copy_ad(292, 157);
            } else {
                s.store_scalar(292, 1.0);
            }
        } else {
            s.store_scalar(292, 0.0);
        }
        if (s.v[156] > 0.0) {
            s.copy_ad(291, 156);
        } else {
            s.store_scalar(291, 0.0);
        }
        if (s.v[173] > 0.0) {
            s.copy_ad(306, 173);
        } else {
            s.store_scalar(306, 0.0);
        }
        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_primal_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }
        s.store_scalar(16, p.p16);s.store_scalar(17, p.p15);s.store_scalar(18, p.p18);s.store_scalar(19, p.p17);s.b[1130] = (p.p44 == 0.0);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if s.b[1130] {s.copy_ad(191, 190);s.copy_ad(193, 192);s.copy_ad(246, 245);s.copy_ad(248, 247);s.copy_ad(250, 249);s.copy_ad(252, 251);s.copy_ad(236, 235);s.copy_ad(242, 240);s.copy_ad(243, 241);s.copy_ad(261, 260);s.copy_ad(263, 262);s.copy_ad(267, 266);s.copy_ad(273, 272);}
        s.store_primal_scale(757, 180, 8.8541878176e-12);s.store_primal_div(758, 757, 179);s.store_primal_square(759, 179);s.store_primal_scale(760, 758, 6.241449993689894e18);s.store_primal_mul(761, 255, 181);
        if (s.v[761] > 1e20) {
            if (s.v[761] < 1e26) {
            } else {
                s.store_scalar(761, 1e26);
            }
        } else {
            s.store_scalar(761, 1e20);
        }
        s.store_scalar(762, 0.0);s.b[1131] = (p.p51 > 0.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if s.b[1131] {s.store_primal_scale_ad(762, A::powf(s.ad_value(758), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));}
        s.b[1132] = (s.v[0] == (-1.0));s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1132]) {s.store_primal_scale(762, 762, (7.448711 / 5.951993));}
        s.store_primal_scale(763, 758, (1e-8 * 1.0 / (s.v[756])));s.store_primal_scale(764, 212, 0.5);s.store_scalar(765, 0.5);s.b[1133] = (s.v[0] == (-1.0));s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if s.b[1133] {s.store_primal_scale(764, 212, 0.3333333333333333);s.store_scalar(765, 0.3333333333333333);}
        s.store_primal_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(222)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(766, 997, (-1.0), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);
    }
}
